//! The in-RAM bit **screen**: a superset filter that rules out targets that
//! cannot contain a query, so the verifier ([`vf2`](crate::vf2) /
//! [`brute`](crate::brute)) runs on fewer molecules.
//!
//! # Soundness
//!
//! Each target is reduced to a fixed-width *fingerprint*: one bit per labelled
//! path or ring fragment. A query is reduced to a set of **required** bits that
//! every matching molecule must set. A target is kept iff its fingerprint
//! contains all required bits.
//!
//! The screen must never drop a true match. A query bit is required only when its
//! fragment is *forced*: every atom has one determined `(element, aromatic)` label
//! and every bond (for bond-aware tokens) one determined order, with no `OR`,
//! wildcard or negation that could let a match avoid it. Unforced fragments emit
//! no bit; a query with no forced fragment gets an empty mask and keeps every
//! target. Bit collisions only add bits to a target, so they cost selectivity,
//! not soundness.
//!
//! # Fragments
//!
//! Targets enumerate every simple path of up to [`MAX_PATH_ATOMS`] atoms
//! (including single atoms) and every ring closed by such a path. Atoms are
//! tokenised as `(atomic number, aromatic)`, bonds as their order. A path and its
//! reverse (and a ring under rotation and reflection) hash identically, so a
//! forced query fragment produces the same token string as its image in any
//! matching target.

use crate::graph::{bond_order, MolGraph};
use crate::query::{BondPred, Pred, QueryGraph};

/// Fingerprint width in bits. A power of two, so folding a hash is a mask.
pub const FP_BITS: usize = 1024;

/// Number of `u64` words backing one [`FP_BITS`]-wide fingerprint.
pub const FP_WORDS: usize = FP_BITS / 64;

/// Longest path, in atoms, enumerated on both the target and the query side.
pub const MAX_PATH_ATOMS: usize = 7;

/// Path-enumeration steps allowed per atom; see [`path_step_budget`].
pub const PATH_STEPS_PER_ATOM: usize = 4_096;

/// The smallest and largest path-enumeration budget of one molecule or query.
pub const PATH_STEPS_RANGE: (usize, usize) = (1 << 17, 1 << 22);

/// The most path-enumeration steps one fingerprint or query mask of `atoms` atoms
/// may take: [`PATH_STEPS_PER_ATOM`] per atom, clamped to [`PATH_STEPS_RANGE`].
///
/// Path enumeration costs about `atoms × degree^6`, which is unbounded in practice
/// on dense hostile graphs. Past the budget both sides stop in the direction that
/// keeps the screen a superset: a target fingerprint becomes **all bits set** (the
/// row passes every screen), and a query mask keeps only the bits found so far
/// (fewer required bits admit more rows).
pub fn path_step_budget(atoms: usize) -> usize {
    atoms
        .saturating_mul(PATH_STEPS_PER_ATOM)
        .clamp(PATH_STEPS_RANGE.0, PATH_STEPS_RANGE.1)
}

/// A fixed-width fingerprint: [`FP_BITS`] bits packed into [`FP_WORDS`] words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fingerprint {
    words: [u64; FP_WORDS],
}

impl Default for Fingerprint {
    fn default() -> Self {
        Self::new()
    }
}

impl Fingerprint {
    /// An all-zero fingerprint (no fragments recorded).
    #[inline]
    pub fn new() -> Self {
        Fingerprint {
            words: [0; FP_WORDS],
        }
    }

    /// Sets the bit at `index` (taken modulo [`FP_BITS`], so any hash is valid).
    #[inline]
    pub fn set(&mut self, index: usize) {
        let bit = index & (FP_BITS - 1);
        self.words[bit >> 6] |= 1u64 << (bit & 63);
    }

    /// True if the bit at `index` (mod [`FP_BITS`]) is set.
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        let bit = index & (FP_BITS - 1);
        self.words[bit >> 6] & (1u64 << (bit & 63)) != 0
    }

    /// ORs every bit of `other` into `self`. The union of two required masks is
    /// still a required mask.
    #[inline]
    pub fn union_with(&mut self, other: &Fingerprint) {
        for (s, o) in self.words.iter_mut().zip(other.words.iter()) {
            *s |= *o;
        }
    }

    /// True iff every bit set in `mask` is also set in `self` (`self ⊇ mask`).
    #[inline]
    pub fn contains(&self, mask: &Fingerprint) -> bool {
        // Branchless accumulate (no early return) so the loop autovectorises.
        let mut acc = 0u64;
        for (m, s) in mask.words.iter().zip(self.words.iter()) {
            acc |= m & !s;
        }
        acc == 0
    }

    /// Number of set bits.
    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.words.iter().map(|w| w.count_ones()).sum()
    }

    /// The backing words, read as one row by the column transpose.
    #[inline]
    pub(crate) fn words(&self) -> &[u64] {
        &self.words
    }
}

/// Hashes a canonical fragment token slice to a bit index in `[0, FP_BITS)`.
///
/// FNV-1a. Target and query sides must both use this same function.
fn hash_tokens(tokens: &[u8]) -> usize {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325; // FNV offset basis
    for &b in tokens {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3); // FNV prime
    }
    (h as usize) & (FP_BITS - 1)
}

/// Canonical atom token: `[atomic number, aromatic as 0/1]`.
#[inline]
fn atom_token(element: u8, aromatic: bool) -> [u8; 2] {
    [element, aromatic as u8]
}

/// Reusable token buffers and the step budget for one path enumeration, so no
/// allocation happens per path.
struct PathTokenScratch {
    fwd: Vec<u8>,
    rev: Vec<u8>,
    a_fwd: Vec<u8>,
    a_rev: Vec<u8>,
    c_buf: Vec<u8>,
    c_best: Vec<u8>,
    /// Path steps left before the budget ([`path_step_budget`]) is spent.
    steps_left: usize,
    /// Set when a step was refused because `steps_left` was 0.
    exhausted: bool,
}

impl PathTokenScratch {
    fn new(budget: usize) -> Self {
        PathTokenScratch {
            fwd: Vec::new(),
            rev: Vec::new(),
            a_fwd: Vec::new(),
            a_rev: Vec::new(),
            c_buf: Vec::new(),
            c_best: Vec::new(),
            steps_left: budget,
            exhausted: false,
        }
    }

    /// Takes one enumeration step; `false` once the budget is spent.
    #[inline]
    fn step(&mut self) -> bool {
        if self.steps_left == 0 {
            self.exhausted = true;
            return false;
        }
        self.steps_left -= 1;
        true
    }

    /// Bond-aware **path** token (tag `0xA0`): atoms interleaved with bond orders.
    /// Returns the smaller of the forward and reverse serialisations.
    ///
    /// `bonds[i]` joins `atoms[i]` and `atoms[i + 1]`.
    fn canonicalise(&mut self, atoms: &[[u8; 2]], bonds: &[u8]) -> &[u8] {
        debug_assert_eq!(bonds.len() + 1, atoms.len().max(1));

        // Forward serialisation: a0, b0, a1, b1, …, a_{n-1}.
        self.fwd.clear();
        self.fwd.push(0xA0); // fragment-kind tag
        for (i, a) in atoms.iter().enumerate() {
            self.fwd.extend_from_slice(a);
            if i < bonds.len() {
                self.fwd.push(bonds[i]);
            }
        }

        // Reverse serialisation: a_{n-1}, b_{n-2}, …, a0.
        self.rev.clear();
        self.rev.push(0xA0);
        for (i, a) in atoms.iter().rev().enumerate() {
            self.rev.extend_from_slice(a);
            if i < bonds.len() {
                // Reversed bond between reversed atoms i and i+1 is bonds[n-2-i].
                self.rev.push(bonds[bonds.len() - 1 - i]);
            }
        }

        if self.fwd <= self.rev {
            &self.fwd
        } else {
            &self.rev
        }
    }

    /// Bond-agnostic **atom-only path** token (tag `0xC0`), reversal-symmetric.
    ///
    /// Sound because every query edge maps to a target edge whatever its bond
    /// predicate, so a path of forced-label query atoms forces a same-label target
    /// path. This lets queries with unforced bonds (such as aromatic rings, whose
    /// default bond is single-or-aromatic) still emit bits.
    fn atomonly_path(&mut self, atoms: &[[u8; 2]]) -> &[u8] {
        self.a_fwd.clear();
        self.a_fwd.push(0xC0);
        for a in atoms {
            self.a_fwd.extend_from_slice(a);
        }
        self.a_rev.clear();
        self.a_rev.push(0xC0);
        for a in atoms.iter().rev() {
            self.a_rev.extend_from_slice(a);
        }
        if self.a_fwd <= self.a_rev {
            &self.a_fwd
        } else {
            &self.a_rev
        }
    }

    /// Bond-aware **cycle** token (tag `0xB0`) for a ring of `k` atoms where
    /// `atoms[i]` is bonded to `atoms[(i+1)%k]` by `bonds[i]`. A ring has no start
    /// and no direction, so the canonical form is the smallest serialisation over
    /// all rotations and both directions.
    fn cycle(&mut self, atoms: &[[u8; 2]], bonds: &[u8]) -> &[u8] {
        let k = atoms.len();
        debug_assert_eq!(bonds.len(), k);
        self.c_best.clear();
        for dir in [1i64, -1] {
            for s in 0..k {
                self.c_buf.clear();
                self.c_buf.push(0xB0);
                for i in 0..k {
                    let idx = (s as i64 + dir * i as i64).rem_euclid(k as i64) as usize;
                    self.c_buf.extend_from_slice(&atoms[idx]);
                    let b = if dir == 1 {
                        bonds[idx]
                    } else {
                        bonds[(idx + k - 1) % k]
                    };
                    self.c_buf.push(b);
                }
                if self.c_best.is_empty() || self.c_buf < self.c_best {
                    self.c_best.clear();
                    self.c_best.extend_from_slice(&self.c_buf);
                }
            }
        }
        &self.c_best
    }

    /// Bond-agnostic **atom-only cycle** token (tag `0xD0`), invariant under
    /// rotation and reflection. The ring analogue of
    /// [`atomonly_path`](Self::atomonly_path).
    fn atomonly_cycle(&mut self, atoms: &[[u8; 2]]) -> &[u8] {
        let k = atoms.len();
        self.c_best.clear();
        for dir in [1i64, -1] {
            for s in 0..k {
                self.c_buf.clear();
                self.c_buf.push(0xD0);
                for i in 0..k {
                    let idx = (s as i64 + dir * i as i64).rem_euclid(k as i64) as usize;
                    self.c_buf.extend_from_slice(&atoms[idx]);
                }
                if self.c_best.is_empty() || self.c_buf < self.c_best {
                    self.c_best.clear();
                    self.c_best.extend_from_slice(&self.c_buf);
                }
            }
        }
        &self.c_best
    }
}

/// Builds the fingerprint of a perceived target molecule.
///
/// Every simple path of up to [`MAX_PATH_ATOMS`] atoms (each prefix, including the
/// single start atom) and every ring it closes sets its bits, so a forced query
/// fragment of any length up to the bound finds its bit here.
pub fn target_fingerprint(mol: &MolGraph) -> Fingerprint {
    let mut fp = Fingerprint::new();
    let n = mol.atom_count();
    if n == 0 {
        return fp;
    }

    // `on_path` keeps paths simple.
    let mut path_atoms: Vec<[u8; 2]> = Vec::with_capacity(MAX_PATH_ATOMS);
    let mut path_bonds: Vec<u8> = Vec::with_capacity(MAX_PATH_ATOMS);
    let mut on_path = vec![false; n];
    let mut tokens = PathTokenScratch::new(path_step_budget(n));

    for start in 0..n as u32 {
        on_path[start as usize] = true;
        path_atoms.push(atom_token(
            mol.element[start as usize],
            mol.is_aromatic(start),
        ));
        dfs_paths(
            mol,
            start,
            start,
            &mut path_atoms,
            &mut path_bonds,
            &mut on_path,
            &mut tokens,
            &mut fp,
        );
        path_atoms.pop();
        on_path[start as usize] = false;
        if tokens.exhausted {
            // Unknown paths remain, so any bit may be one of them: claim them all.
            return Fingerprint {
                words: [u64::MAX; FP_WORDS],
            };
        }
    }

    fp
}

/// Depth-first path enumeration: records the current path's bits, the ring bits
/// if `last` bonds back to `start`, then extends to each unvisited neighbour.
#[allow(clippy::too_many_arguments)]
fn dfs_paths(
    mol: &MolGraph,
    start: u32,
    last: u32,
    path_atoms: &mut Vec<[u8; 2]>,
    path_bonds: &mut Vec<u8>,
    on_path: &mut [bool],
    tokens: &mut PathTokenScratch,
    fp: &mut Fingerprint,
) {
    if !tokens.step() {
        return;
    }
    fp.set(hash_tokens(tokens.canonicalise(path_atoms, path_bonds)));
    fp.set(hash_tokens(tokens.atomonly_path(path_atoms)));

    // Ring closure: a path of 3 or more atoms plus a bond back to `start`.
    if path_atoms.len() >= 3 {
        if let Some(close) = mol.neighbors(last).iter().find(|nb| nb.atom == start) {
            path_bonds.push(mol.bonds[close.bond as usize].order);
            fp.set(hash_tokens(tokens.cycle(path_atoms, path_bonds)));
            path_bonds.pop();
            fp.set(hash_tokens(tokens.atomonly_cycle(path_atoms)));
        }
    }

    if path_atoms.len() >= MAX_PATH_ATOMS {
        return;
    }

    for nb in mol.neighbors(last) {
        if on_path[nb.atom as usize] {
            continue;
        }
        let bond = &mol.bonds[nb.bond as usize];
        on_path[nb.atom as usize] = true;
        path_atoms.push(atom_token(
            mol.element[nb.atom as usize],
            mol.is_aromatic(nb.atom),
        ));
        path_bonds.push(bond.order);
        dfs_paths(
            mol, start, nb.atom, path_atoms, path_bonds, on_path, tokens, fp,
        );
        path_bonds.pop();
        path_atoms.pop();
        on_path[nb.atom as usize] = false;
    }
}

// ===========================================================================
// Query side: derive the REQUIRED bits a matching molecule must set.
// ===========================================================================

/// The `(element, aromatic)` label a query atom predicate *forces*, if any.
///
/// `Some(label)` means every atom that satisfies the predicate has exactly this
/// label. Returning `None` is always safe (fewer required bits); `Some` is allowed
/// only when the predicate provably admits no other label. Shared with
/// [`census`](crate::census) so the proof has one definition.
pub(crate) fn forced_atom_label(pred: &Pred) -> Option<(u8, bool)> {
    match pred {
        Pred::Element(n) => Some((*n, false)),
        Pred::AromaticElement(n) => Some((*n, true)),
        // AND: one branch forces a label and the other does not contradict it.
        Pred::And(l, r) => match (forced_atom_label(l), forced_atom_label(r)) {
            (Some(a), Some(b)) => {
                if a == b {
                    Some(a)
                } else {
                    // Contradictory (e.g. `C;c`): matches nothing. Not forced.
                    None
                }
            }
            // The other branch may still contradict the aromaticity (`C;a`).
            (Some(a), None) => label_compatible(a, r).then_some(a),
            (None, Some(b)) => label_compatible(b, l).then_some(b),
            (None, None) => None,
        },
        // The host atom is the env root, so it forces whatever the root forces.
        Pred::Recursive(env) => env.atoms.first().and_then(forced_atom_label),
        // `*`, `#n` (aromaticity unknown), `a`/`A` (element unknown), structural
        // primitives, OR and NOT do not pin one label.
        _ => None,
    }
}

/// True if `pred` does not contradict an atom labelled `(element, aromatic)`.
///
/// Only called when another AND branch already forces `label`, so every atom
/// matching the conjunction has `label` whatever this returns. A `false` only
/// suppresses a bit; a `true` on a contradiction makes the query unsatisfiable,
/// which drops no true match.
fn label_compatible(label: (u8, bool), pred: &Pred) -> bool {
    let (elem, aromatic) = label;
    match pred {
        // Structural facts say nothing about element or aromaticity.
        Pred::InRing
        | Pred::RingSize(_)
        | Pred::Degree(_)
        | Pred::TotalH(_)
        | Pred::ImplicitH(_)
        | Pred::TotalConnections(_)
        | Pred::RingBondCount(_)
        | Pred::Valence(_)
        | Pred::RingCount(_)
        | Pred::Charge(_)
        | Pred::Any => true,
        Pred::Aromatic => aromatic,
        Pred::Aliphatic => !aromatic,
        Pred::Element(n) => *n == elem && !aromatic,
        Pred::AromaticElement(n) => *n == elem && aromatic,
        Pred::AtomicNumber(n) => *n == elem,
        Pred::And(l, r) => label_compatible(label, l) && label_compatible(label, r),
        // The label stays possible if either branch allows it.
        Pred::Or(l, r) => label_compatible(label, l) || label_compatible(label, r),
        // The host is the env root, so check the root predicate.
        Pred::Recursive(env) => env
            .atoms
            .first()
            .is_none_or(|root| label_compatible(label, root)),
        // Sound for the reason in the doc comment: the other branch fixes the
        // label, so `[C;!R]` keeps its carbon bit. At worst (`!#6`) the
        // conjunction is unsatisfiable and the extra bit drops no match.
        Pred::Not(_) => true,
    }
}

/// The bond order a query bond predicate *forces*, if any: `Some(order)` means
/// every bond satisfying the predicate has exactly that order. Shared with
/// [`census`](crate::census).
pub(crate) fn forced_bond_order(pred: &BondPred) -> Option<u8> {
    match pred {
        BondPred::Order(o) => Some(*o),
        BondPred::Aromatic => Some(bond_order::AROMATIC),
        // AND: `=&@` forces DOUBLE; `-&=` is contradictory, so not forced.
        BondPred::And(l, r) => match (forced_bond_order(l), forced_bond_order(r)) {
            (Some(a), Some(b)) => (a == b).then_some(a),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        },
        // `~`, default single-or-aromatic, `@`, OR, NOT.
        _ => None,
    }
}

/// Builds the query's **required-bit mask**: the fingerprint bits that every
/// molecule matching `query` is guaranteed to set.
///
/// Enumerates the query's forced paths and rings (bond-aware and atom-only) with
/// the same canonical scheme as [`target_fingerprint`], and folds in the masks of
/// unconditionally forced `$(env)` environments. A path stops at the first atom
/// or bond that is not forced.
///
/// # Soundness
///
/// Let target `T` match via injective embedding `f`. For a forced query path
/// `q0–…–qk`, each `f(qi)` satisfies `qi`'s predicate, so it has exactly the
/// forced label, and each target bond `f(qi)–f(qi+1)` exists and has exactly the
/// forced order. So `f` maps the path to a simple target path with the same token
/// string, whose bit `T`'s fingerprint sets. Every required bit is in `T`, so `T`
/// is kept.
///
/// A query with no forced atom gets an empty mask, which keeps every target.
pub fn query_required_mask(query: &QueryGraph) -> Fingerprint {
    let mut mask = Fingerprint::new();
    let nq = query.atom_count();
    if nq == 0 {
        return mask;
    }

    let labels: Vec<Option<(u8, bool)>> = query.atoms.iter().map(forced_atom_label).collect();
    let bond_orders: Vec<Option<u8>> = query
        .bonds
        .iter()
        .map(|b| forced_bond_order(&b.pred))
        .collect();

    let mut path_atoms: Vec<[u8; 2]> = Vec::with_capacity(MAX_PATH_ATOMS);
    let mut path_bonds: Vec<u8> = Vec::with_capacity(MAX_PATH_ATOMS);
    let mut on_path = vec![false; nq];
    let mut tokens = PathTokenScratch::new(path_step_budget(nq));

    for start in 0..nq as u32 {
        let Some((e, a)) = labels[start as usize] else {
            continue; // unforced atom: no forced path can begin here
        };
        on_path[start as usize] = true;
        path_atoms.push(atom_token(e, a));
        dfs_required(
            query,
            start,
            start,
            &labels,
            &bond_orders,
            &mut path_atoms,
            &mut path_bonds,
            &mut on_path,
            &mut tokens,
            &mut mask,
        );
        path_atoms.pop();
        on_path[start as usize] = false;
    }

    // Second pass: atom-only fragments, which extend through any query edge.
    for start in 0..nq as u32 {
        let Some((e, a)) = labels[start as usize] else {
            continue;
        };
        on_path[start as usize] = true;
        path_atoms.push(atom_token(e, a));
        dfs_required_atomonly(
            query,
            start,
            start,
            &labels,
            &mut path_atoms,
            &mut on_path,
            &mut tokens,
            &mut mask,
        );
        path_atoms.pop();
        on_path[start as usize] = false;
    }

    // Third pass: an unconditionally forced `$(env)` means every match contains an
    // embedding of `env`, so `env`'s required bits are required here too. Only
    // presence bits are used, so the env embedding may overlap the main match.
    let mut envs: Vec<&QueryGraph> = Vec::new();
    for pred in &query.atoms {
        collect_forced_recursive_envs(pred, &mut envs);
    }
    for env in envs {
        let env_mask = query_required_mask(env);
        mask.union_with(&env_mask);
    }

    mask
}

/// Collects every `$(env)` that `pred` forces unconditionally, i.e. reached only
/// through AND. `Or` (another branch may match) and `Not` (the env is absent) are
/// skipped.
fn collect_forced_recursive_envs<'a>(pred: &'a Pred, out: &mut Vec<&'a QueryGraph>) {
    match pred {
        Pred::Recursive(env) => out.push(env),
        Pred::And(l, r) => {
            collect_forced_recursive_envs(l, out);
            collect_forced_recursive_envs(r, out);
        }
        // Or / Not / leaves force no env unconditionally.
        _ => {}
    }
}

/// DFS over the query collecting atom-only path and ring bits (`0xC0` / `0xD0`).
/// Like [`dfs_required`], but extends through any bond whose neighbour atom has a
/// forced label.
#[allow(clippy::too_many_arguments)]
fn dfs_required_atomonly(
    query: &QueryGraph,
    start: u32,
    last: u32,
    labels: &[Option<(u8, bool)>],
    path_atoms: &mut Vec<[u8; 2]>,
    on_path: &mut [bool],
    tokens: &mut PathTokenScratch,
    mask: &mut Fingerprint,
) {
    if !tokens.step() {
        return; // keep the bits found so far: a smaller mask is still sound
    }
    mask.set(hash_tokens(tokens.atomonly_path(path_atoms)));

    if path_atoms.len() >= 3
        && query.incident[last as usize].iter().any(|&bi| {
            let qb = &query.bonds[bi as usize];
            let other = if qb.a == last { qb.b } else { qb.a };
            other == start
        })
    {
        mask.set(hash_tokens(tokens.atomonly_cycle(path_atoms)));
    }

    if path_atoms.len() >= MAX_PATH_ATOMS {
        return;
    }

    for &bi in &query.incident[last as usize] {
        let qb = &query.bonds[bi as usize];
        let other = if qb.a == last { qb.b } else { qb.a };
        if on_path[other as usize] {
            continue;
        }
        let Some((e, a)) = labels[other as usize] else {
            continue; // unforced neighbour atom: cannot extend a forced-atom path
        };
        on_path[other as usize] = true;
        path_atoms.push(atom_token(e, a));
        dfs_required_atomonly(
            query, start, other, labels, path_atoms, on_path, tokens, mask,
        );
        path_atoms.pop();
        on_path[other as usize] = false;
    }
}

/// DFS over the query collecting bond-aware forced path and ring bits. The query
/// counterpart of [`dfs_paths`].
#[allow(clippy::too_many_arguments)]
fn dfs_required(
    query: &QueryGraph,
    start: u32,
    last: u32,
    labels: &[Option<(u8, bool)>],
    bond_orders: &[Option<u8>],
    path_atoms: &mut Vec<[u8; 2]>,
    path_bonds: &mut Vec<u8>,
    on_path: &mut [bool],
    tokens: &mut PathTokenScratch,
    mask: &mut Fingerprint,
) {
    if !tokens.step() {
        return; // keep the bits found so far: a smaller mask is still sound
    }
    // Only forced atoms and bonds are ever pushed, so this path is forced.
    let bit = hash_tokens(tokens.canonicalise(path_atoms, path_bonds));
    mask.set(bit);

    // Forced ring closure: a forced-order bond from `last` back to `start`.
    if path_atoms.len() >= 3 {
        let closing = query.incident[last as usize].iter().find_map(|&bi| {
            let qb = &query.bonds[bi as usize];
            let other = if qb.a == last { qb.b } else { qb.a };
            (other == start).then(|| bond_orders[bi as usize]).flatten()
        });
        if let Some(order) = closing {
            path_bonds.push(order);
            mask.set(hash_tokens(tokens.cycle(path_atoms, path_bonds)));
            path_bonds.pop();
        }
    }

    if path_atoms.len() >= MAX_PATH_ATOMS {
        return;
    }

    for &bi in &query.incident[last as usize] {
        let Some(order) = bond_orders[bi as usize] else {
            continue; // unforced bond: cannot extend a forced path through it
        };
        let qb = &query.bonds[bi as usize];
        let other = if qb.a == last { qb.b } else { qb.a };
        if on_path[other as usize] {
            continue; // keep paths simple
        }
        let Some((e, a)) = labels[other as usize] else {
            continue; // unforced neighbour atom
        };

        on_path[other as usize] = true;
        path_atoms.push(atom_token(e, a));
        path_bonds.push(order);
        dfs_required(
            query,
            start,
            other,
            labels,
            bond_orders,
            path_atoms,
            path_bonds,
            on_path,
            tokens,
            mask,
        );
        path_bonds.pop();
        path_atoms.pop();
        on_path[other as usize] = false;
    }
}

// ===========================================================================
// In-RAM screen index: a bit-transposed column store + AND-containment.
// ===========================================================================
//
// Fingerprints are stored column-major in a `ColumnBitmap`: one dense bitmap of
// rows per fingerprint bit, so a query touches only its set columns. The screen
// builds with `ColumnBitmap::from_rows` in insertion order (not the popcount-sorted
// `Arena` build), so row ids stay aligned with the molecule store.

use mossflux_index::ColumnBitmap;

/// An in-RAM screen over target fingerprints, stored bit-transposed. Row ids are
/// dense `u32`s in insertion order, aligned with the owning index's molecules.
///
/// [`screen_contains`](Self::screen_contains) returns a superset of the true
/// matches (proof on [`query_required_mask`]).
#[derive(Clone, Debug, Default)]
pub struct ScreenIndex {
    /// Row-major fingerprints in insertion order; the source of the transpose.
    rows: Vec<Fingerprint>,
    /// The transpose of `rows`. `None` means stale.
    cols: Option<ScreenColumns>,
}

/// Column-major fingerprints: column `j` has bit `p` of word `w` set iff row
/// `w*64 + p` has fingerprint bit `j`.
#[derive(Clone, Debug)]
struct ScreenColumns {
    inner: ColumnBitmap,
}

impl ScreenColumns {
    /// Transposes `fps` in order, so row ids equal `fps` indices.
    fn build(fps: &[Fingerprint]) -> ScreenColumns {
        ScreenColumns {
            inner: ColumnBitmap::from_rows(fps.len(), FP_BITS, |i| fps[i].words()),
        }
    }

    /// Words per column = `ceil(n / 64)`.
    #[inline]
    fn wpc(&self) -> usize {
        self.inner.wpc()
    }

    /// Column `j`'s dense row bitmap: `wpc` words.
    #[inline]
    fn column(&self, j: usize) -> &[u64] {
        self.inner.column(j)
    }

    /// Number of rows that set bit `j`.
    #[inline]
    fn col_popcount(&self, j: usize) -> u32 {
        self.inner.col_popcount(j)
    }

    /// Approximate heap size in bytes.
    #[inline]
    fn mem_bytes(&self) -> usize {
        self.inner.mem_bytes()
    }
}

impl ScreenIndex {
    /// An empty screen.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of indexed rows (targets).
    #[inline]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// True if no rows are indexed.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Appends a fingerprint and returns its row id. Marks the transpose stale.
    ///
    /// # Panics
    /// Panics past `u32::MAX` rows, where the row id would wrap and collide.
    pub fn push(&mut self, fp: Fingerprint) -> u32 {
        assert!(
            self.rows.len() < u32::MAX as usize,
            "screen caps at 2^32 rows; shard beyond that"
        );
        let id = self.rows.len() as u32;
        self.rows.push(fp);
        self.cols = None;
        id
    }

    /// Computes and appends the fingerprint of `mol`, returning its row id.
    pub fn push_mol(&mut self, mol: &MolGraph) -> u32 {
        self.push(target_fingerprint(mol))
    }

    /// The fingerprint of `row`.
    ///
    /// # Panics
    /// Panics if `row >= self.len()`.
    #[inline]
    #[must_use]
    pub fn fingerprint(&self, row: u32) -> &Fingerprint {
        &self.rows[row as usize]
    }

    /// Builds a screen from precomputed fingerprints; `fps[i]` becomes row `i`.
    /// Same as pushing each in order. The transpose is left stale; call
    /// [`prepare`](Self::prepare) to build it.
    pub fn from_fingerprints(fps: Vec<Fingerprint>) -> Self {
        ScreenIndex {
            rows: fps,
            cols: None,
        }
    }

    /// Builds the transpose if stale. Performance only: `screen_contains` returns
    /// the same rows either way.
    pub fn prepare(&mut self) {
        if self.cols.is_none() {
            self.cols = Some(ScreenColumns::build(&self.rows));
        }
    }

    /// Returns, in ascending order, the row ids whose fingerprint contains every
    /// bit in `mask`. An empty mask returns all rows.
    ///
    /// Intersects the column bitmaps of the mask's set bits, rarest column first,
    /// and stops once the survivor set is empty. Without a built transpose (see
    /// [`prepare`](Self::prepare)) it falls back to a row-major scan with the same
    /// result.
    pub fn screen_contains(&self, mask: &Fingerprint) -> Vec<u32> {
        let n = self.rows.len();
        if n == 0 {
            return Vec::new();
        }

        let mut qcols: Vec<usize> = Vec::new();
        for (wi, &word) in mask.words.iter().enumerate() {
            let mut bits = word;
            while bits != 0 {
                qcols.push(wi * 64 + bits.trailing_zeros() as usize);
                bits &= bits - 1;
            }
        }
        if qcols.is_empty() {
            return (0..n as u32).collect();
        }

        let Some(cols) = self.cols.as_ref() else {
            let mut out = Vec::new();
            for (i, row) in self.rows.iter().enumerate() {
                if row.contains(mask) {
                    out.push(i as u32);
                }
            }
            return out;
        };
        let wpc = cols.wpc();

        qcols.sort_by_key(|&j| cols.col_popcount(j));
        let mut acc: Vec<u64> = cols.column(qcols[0]).to_vec();

        // Live word span `[lo, hi)`: words outside it are zero. AND only clears
        // bits, so the span only shrinks; empty means no candidate.
        let (mut lo, mut hi) = trim_zero_span(&acc, 0, wpc);

        for &j in &qcols[1..] {
            if lo >= hi {
                break;
            }
            let col = cols.column(j);
            for (a, c) in acc[lo..hi].iter_mut().zip(col[lo..hi].iter()) {
                *a &= *c;
            }
            (lo, hi) = trim_zero_span(&acc, lo, hi);
        }

        // Spare high bits of the last word are always 0; `r < n` is a guard.
        let mut out = Vec::new();
        for (off, &word) in acc[lo..hi].iter().enumerate() {
            let w = lo + off;
            let mut bits = word;
            while bits != 0 {
                let r = w * 64 + bits.trailing_zeros() as usize;
                bits &= bits - 1;
                if r < n {
                    out.push(r as u32);
                }
            }
        }
        out
    }

    /// [`screen_contains`](Self::screen_contains) with `query`'s required mask.
    pub fn screen_query(&self, query: &QueryGraph) -> Vec<u32> {
        let mask = query_required_mask(query);
        self.screen_contains(&mask)
    }

    /// Approximate heap size in bytes of the rows and the transpose (if built),
    /// counted by capacity.
    pub fn mem_bytes(&self) -> usize {
        use core::mem::size_of;
        let mut bytes = self.rows.capacity() * size_of::<Fingerprint>();
        if let Some(cols) = self.cols.as_ref() {
            bytes += cols.mem_bytes();
        }
        bytes
    }
}

/// Shrinks `[lo, hi)` to the tightest span that covers every non-zero word of
/// `acc`. Returns an empty span when every word is 0.
#[inline]
fn trim_zero_span(acc: &[u64], mut lo: usize, mut hi: usize) -> (usize, usize) {
    while lo < hi && acc[lo] == 0 {
        lo += 1;
    }
    while hi > lo && acc[hi - 1] == 0 {
        hi -= 1;
    }
    (lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brute;
    use crate::smarts::parse as parse_smarts;
    use crate::smiles::parse as parse_smiles;

    // ---- Fingerprint bit-vector mechanics -----------------------------------

    #[test]
    fn fingerprint_set_get_contains_count() {
        let mut a = Fingerprint::new();
        assert_eq!(a.count_ones(), 0);
        a.set(3);
        a.set(70);
        a.set(FP_BITS + 5); // wraps to bit 5
        assert!(a.get(3));
        assert!(a.get(70));
        assert!(a.get(5));
        assert_eq!(a.count_ones(), 3);

        // Containment: a ⊇ subset.
        let mut sub = Fingerprint::new();
        sub.set(3);
        sub.set(70);
        assert!(a.contains(&sub));
        // Add a bit a does not have ⇒ no longer contained.
        sub.set(9);
        assert!(!a.contains(&sub));
        // The empty mask is contained in everything (the no-screen case).
        assert!(a.contains(&Fingerprint::new()));
        assert!(Fingerprint::new().contains(&Fingerprint::new()));
    }

    // ---- Target fingerprint sanity ------------------------------------------

    #[test]
    fn target_fingerprint_is_nonempty_and_path_is_reverse_symmetric() {
        let mol = parse_smiles("CCO").unwrap();
        let fp = target_fingerprint(&mol);
        // At least: 2 atom kinds + several paths ⇒ multiple bits.
        assert!(
            fp.count_ones() >= 3,
            "ethanol fp too sparse: {}",
            fp.count_ones()
        );

        // The canonical path token of C-C-O equals that of O-C-C (reverse).
        let co = parse_smiles("CCO").unwrap();
        let oc = parse_smiles("OCC").unwrap();
        // OCC and CCO are the *same* molecule written two ways ⇒ identical fp.
        assert_eq!(target_fingerprint(&co), target_fingerprint(&oc));
    }

    // ---- Forced-label derivation (the soundness-critical helpers) ------------

    #[test]
    fn forced_atom_labels_are_conservative() {
        use crate::query::Pred;
        // Bare elements force their label.
        assert_eq!(forced_atom_label(&Pred::Element(6)), Some((6, false)));
        assert_eq!(
            forced_atom_label(&Pred::AromaticElement(6)),
            Some((6, true))
        );
        // `#6` does NOT force aromaticity ⇒ no single label.
        assert_eq!(forced_atom_label(&Pred::AtomicNumber(6)), None);
        // Wildcards / structural-only / OR / NOT are unforced.
        assert_eq!(forced_atom_label(&Pred::Any), None);
        assert_eq!(forced_atom_label(&Pred::Aromatic), None);
        assert_eq!(forced_atom_label(&Pred::InRing), None);
        assert_eq!(
            forced_atom_label(&Pred::or(Pred::Element(6), Pred::Element(7))),
            None
        );
        assert_eq!(forced_atom_label(&Pred::not(Pred::Element(6))), None);

        // `[C;R]` — aliphatic carbon AND in-ring ⇒ still forces (6, false).
        assert_eq!(
            forced_atom_label(&Pred::and(Pred::Element(6), Pred::InRing)),
            Some((6, false))
        );
        // `[C;a]` — aliphatic carbon AND aromatic ⇒ contradiction ⇒ unforced.
        assert_eq!(
            forced_atom_label(&Pred::and(Pred::Element(6), Pred::Aromatic)),
            None
        );
        // `[c;R]` — aromatic carbon AND ring ⇒ forces (6, true).
        assert_eq!(
            forced_atom_label(&Pred::and(Pred::AromaticElement(6), Pred::InRing)),
            Some((6, true))
        );
        // `[C;!R]` — the label survives a negated structural branch: every match
        // is still an aliphatic carbon.
        assert_eq!(
            forced_atom_label(&Pred::and(Pred::Element(6), Pred::not(Pred::InRing))),
            Some((6, false))
        );
        // `[c;!a]` is unsatisfiable, so emitting the (6, true) bit drops no match.
        assert_eq!(
            forced_atom_label(&Pred::and(
                Pred::AromaticElement(6),
                Pred::not(Pred::Aromatic)
            )),
            Some((6, true))
        );
    }

    #[test]
    fn recursive_env_mask_inherits_env_required_bits() {
        // `[$(C=O)]`'s host atom forces only "carbon"; folding in the env's mask
        // makes it screen at least as tightly as plain `C=O`.
        let rec = query_required_mask(&parse_smarts("[$(C=O)]").unwrap());
        let plain = query_required_mask(&parse_smarts("C=O").unwrap());
        assert!(plain.count_ones() > 0, "C=O must force some bits");
        assert!(
            rec.contains(&plain),
            "[$(C=O)] mask must contain the C=O mask (env folding)"
        );
        // A negated recursive must not fold env bits: a match need not contain C=O.
        let neg = query_required_mask(&parse_smarts("[!$(C=O)]").unwrap());
        assert!(
            !neg.contains(&plain) || plain.count_ones() == 0,
            "negated recursive must not require the env's bits"
        );
    }

    #[test]
    fn forced_bond_orders_are_conservative() {
        use crate::query::BondPred;
        assert_eq!(
            forced_bond_order(&BondPred::Order(bond_order::DOUBLE)),
            Some(2)
        );
        assert_eq!(
            forced_bond_order(&BondPred::Aromatic),
            Some(bond_order::AROMATIC)
        );
        // `~`, default single-or-aromatic, ring, OR, NOT ⇒ unforced.
        assert_eq!(forced_bond_order(&BondPred::Any), None);
        assert_eq!(forced_bond_order(&BondPred::SingleOrAromatic), None);
        assert_eq!(forced_bond_order(&BondPred::Ring), None);
        assert_eq!(
            forced_bond_order(&BondPred::or(
                BondPred::Order(bond_order::SINGLE),
                BondPred::Order(bond_order::DOUBLE)
            )),
            None
        );
    }

    // ---- Query mask: forced queries emit bits, ambiguous ones degrade --------

    #[test]
    fn ambiguous_query_degrades_to_empty_mask() {
        // These queries force no atom label, so the mask is empty.
        for q in ["*", "a", "A", "[#6]", "[C,N]", "[!C]", "[R]"] {
            let query = parse_smarts(q).unwrap();
            let mask = query_required_mask(&query);
            assert_eq!(
                mask.count_ones(),
                0,
                "query {q:?} should degrade to no-screen"
            );
        }
    }

    #[test]
    fn unforced_bond_still_keeps_forced_atom_bits() {
        // `C~C` has forced atoms but an unforced bond: it gets the atom-only path
        // bit but not the bond-aware one. Masks form the strict chain
        // C ⊂ C~C ⊂ C-C.
        let c = query_required_mask(&parse_smarts("C").unwrap());
        let cc = query_required_mask(&parse_smarts("C~C").unwrap());
        let cc_single = query_required_mask(&parse_smarts("C-C").unwrap());

        assert!(cc.contains(&c), "C~C must require every bit C does");
        assert!(
            cc.count_ones() > c.count_ones(),
            "C~C adds the bond-agnostic two-atom (atom-only) path bit"
        );
        assert!(
            cc_single.contains(&cc),
            "C-C must require everything C~C does"
        );
        assert!(
            cc_single.count_ones() > cc.count_ones(),
            "forcing the bond adds the bond-aware two-atom path bit on top"
        );
    }

    #[test]
    fn aromatic_ring_query_screens_by_ring_topology() {
        // A heteroaromatic ring query has unforced bonds, but its c5n1 six-ring
        // connectivity is forced, so the atom-only ring bit is required.
        let pyridine_q = query_required_mask(&parse_smarts("c1ccncc1").unwrap());
        assert!(
            pyridine_q.count_ones() >= 3,
            "a ring query should emit ring + path bits, not just element presence"
        );
        // Pyridine must screen in itself.
        let pyridine_t = target_fingerprint(&parse_smiles("c1ccncc1").unwrap());
        assert!(
            pyridine_t.contains(&pyridine_q),
            "pyridine must screen-in itself"
        );
        // Benzene + pyrrole has aromatic C and N but no c5n1 six-ring, so it is
        // screened out.
        let benzene_plus_pyrrole =
            target_fingerprint(&parse_smiles("c1ccccc1.c1cc[nH]c1").unwrap());
        assert!(
            !benzene_plus_pyrrole.contains(&pyridine_q),
            "a benzene+pyrrole molecule lacks the pyridine six-ring; must be screened out"
        );
    }

    #[test]
    fn forced_query_emits_bits_present_in_a_true_match() {
        // A fully forced query emits bits, and a matching target has all of them.
        for (q, t) in [
            ("CCO", "CCO"),
            ("CCO", "OCCC"), // CCO path present inside a longer chain
            ("c1ccccc1", "Cc1ccccc1"),
            ("C=O", "CC(=O)O"),
            ("CCl", "ClCC"),
        ] {
            let query = parse_smarts(q).unwrap();
            let mol = parse_smiles(t).unwrap();
            // Sanity: this really is a true match.
            assert!(brute::matches(&query, &mol), "{q} should match {t}");
            let mask = query_required_mask(&query);
            assert!(mask.count_ones() > 0, "forced query {q} should emit bits");
            let fp = target_fingerprint(&mol);
            assert!(
                fp.contains(&mask),
                "required bits of {q} missing from {t} fingerprint (false negative!)"
            );
        }
    }

    // ---- ScreenIndex containment --------------------------------------------

    #[test]
    fn screen_index_keeps_supersets_and_drops_others() {
        let smiles = ["CCO", "c1ccccc1", "CC(=O)O", "CCN", "C1CCCCC1"];
        let mut idx = ScreenIndex::new();
        for s in smiles {
            idx.push_mol(&parse_smiles(s).unwrap());
        }
        idx.prepare(); // exercise the bit-transposed column-AND path
        assert_eq!(idx.len(), smiles.len());

        // A benzene query keeps benzene and drops rows with no aromatic carbon.
        let q = parse_smarts("c1ccccc1").unwrap();
        let cands = idx.screen_query(&q);
        assert!(cands.contains(&1), "benzene row must survive its own query");
        assert!(
            !cands.contains(&0),
            "ethanol must be screened out for benzene"
        );
        assert!(
            !cands.contains(&4),
            "cyclohexane must be screened out for benzene"
        );

        // An empty mask keeps every row.
        let any = parse_smarts("*").unwrap();
        let all = idx.screen_query(&any);
        assert_eq!(all.len(), idx.len(), "wildcard query screens nothing out");
    }

    // ---- Column store equals row-major containment --------------------------

    /// The column-AND screen and the row-major fallback both return exactly the
    /// rows a per-row `Fingerprint::contains` scan returns, sorted ascending.
    #[test]
    fn column_screen_equals_row_major_contains_oracle() {
        let targets = target_corpus();
        let queries = query_corpus();
        let mols: Vec<_> = targets.iter().map(|s| parse_smiles(s).unwrap()).collect();

        let mut unprepared = ScreenIndex::new();
        for m in &mols {
            unprepared.push_mol(m);
        }
        let mut prepared = unprepared.clone();
        prepared.prepare();

        for qs in &queries {
            let query = parse_smarts(qs).unwrap();
            let mask = query_required_mask(&query);

            let oracle: Vec<u32> = (0..mols.len() as u32)
                .filter(|&r| unprepared.fingerprint(r).contains(&mask))
                .collect();

            let col = prepared.screen_contains(&mask);
            let fallback = unprepared.screen_contains(&mask);

            assert_eq!(
                col, oracle,
                "column-AND screen != contains oracle for {qs:?}"
            );
            assert_eq!(
                fallback, oracle,
                "row-major fallback != contains oracle for {qs:?}"
            );
            assert!(
                col.windows(2).all(|w| w[0] < w[1]),
                "column screen result not sorted ascending for {qs:?}"
            );
        }
    }

    /// The column screen is correct when `n` is not a multiple of 64 and when the
    /// survivor set empties mid-scan.
    #[test]
    fn column_screen_handles_partial_last_word_and_empty_survivors() {
        // 70 rows: the last word holds rows 64..69 and 58 spare bits.
        let mut idx = ScreenIndex::new();
        for i in 0..70u32 {
            let s = if i % 2 == 0 { "c1ccccc1" } else { "CCO" };
            idx.push_mol(&parse_smiles(s).unwrap());
        }
        idx.prepare();

        let benzene = query_required_mask(&parse_smarts("c1ccccc1").unwrap());
        let got = idx.screen_contains(&benzene);
        assert!(
            got.iter().all(|&r| r < 70 && r % 2 == 0),
            "spurious / out-of-range rows: {got:?}"
        );
        let oracle: Vec<u32> = (0..70u32)
            .filter(|&r| idx.fingerprint(r).contains(&benzene))
            .collect();
        assert_eq!(got, oracle);

        // No row has nitrogen, so the survivor set empties.
        let nitro = query_required_mask(&parse_smarts("N").unwrap());
        assert!(
            nitro.count_ones() > 0,
            "N should force the nitrogen-presence bit"
        );
        assert!(
            idx.screen_contains(&nitro).is_empty(),
            "no row has nitrogen"
        );
    }

    // ---- Superset (no false negative) and pruning power ----------------------

    /// Targets spanning chains, rings, aromatics, heteroatoms, halogens, charges.
    fn target_corpus() -> Vec<&'static str> {
        vec![
            "C",
            "CC",
            "CCC",
            "CCCC",
            "CCO",
            "OCCO",
            "CC(C)C",
            "CCN",
            "CCOCC",
            "CC(=O)O",
            "CC(=O)N",
            "CC(C)C(=O)O",
            "NCCCN",
            "ClCCBr",
            "FC(F)(F)F",
            "C1CCCCC1",
            "C1CCCC1",
            "C1CCNCC1",
            "O1CCCC1",
            "c1ccccc1",
            "Cc1ccccc1",
            "c1ccncc1",
            "c1ccc(O)cc1",
            "c1cc[nH]c1",
            "c1ccoc1",
            "c1ccsc1",
            "Nc1ccccc1",
            "c1ccc2ccccc2c1",
            "Oc1ccccc1O",
            "CCl",
            "CBr",
            "CF",
            "CCCCCCCC",
            "OCC(O)CO",
            "c1ccc(N)cc1",
        ]
    }

    /// Forced queries plus some ambiguous ones that must degrade.
    fn query_corpus() -> Vec<&'static str> {
        vec![
            // forced single atoms & paths
            "C",
            "O",
            "N",
            "CC",
            "CO",
            "CN",
            "CCO",
            "CCC",
            "CCN",
            "C=O",
            "C#N",
            "ClC",
            "BrC",
            "FC",
            "CCl",
            "OCC",
            "CC(=O)O",
            // forced aromatics
            "c",
            "cc",
            "ccc",
            "cccc",
            "c1ccccc1",
            "co",
            "cn",
            "cs",
            "Oc",
            "Nc",
            "c1ccncc1",
            // forced with structural ANDs that keep a concrete label
            "[C;R]",
            "[c;R]",
            "[c;R][c;R]",
            // ambiguous
            "*",
            "a",
            "A",
            "[#6]",
            "[C,N]",
            "[!C]",
            "C~C",
            "[R]",
            "[r6]",
            "[#6][#8]",
            "a~a",
            "C-,=C",
        ]
    }

    /// For every (target, query) pair, a match by the brute-force oracle must be
    /// kept by the screen. Across the sweep the screen must also prune many true
    /// non-matches.
    #[test]
    fn screen_is_a_sound_superset_with_pruning_power() {
        let targets = target_corpus();
        let queries = query_corpus();

        let mols: Vec<_> = targets
            .iter()
            .map(|s| parse_smiles(s).unwrap_or_else(|e| panic!("target {s:?}: {e}")))
            .collect();
        let mut idx = ScreenIndex::new();
        for m in &mols {
            idx.push_mol(m);
        }
        idx.prepare();

        let mut pairs = 0usize;
        let mut true_matches = 0usize;
        let mut pruned_true_nonmatches = 0usize;

        for qs in &queries {
            let query = parse_smarts(qs).unwrap_or_else(|e| panic!("query {qs:?}: {e}"));
            let candidates = idx.screen_query(&query);
            let kept: std::collections::HashSet<u32> = candidates.into_iter().collect();

            for (row, mol) in mols.iter().enumerate() {
                let row = row as u32;
                let is_match = brute::matches(&query, mol);
                let is_kept = kept.contains(&row);
                pairs += 1;

                if is_match {
                    true_matches += 1;
                    assert!(
                        is_kept,
                        "FALSE NEGATIVE: query {qs:?} matches target {:?} but the \
                         screen dropped it",
                        targets[row as usize]
                    );
                } else if !is_kept {
                    pruned_true_nonmatches += 1;
                }
            }
        }

        assert!(pairs > 1000, "gate should compare many pairs, got {pairs}");
        assert!(true_matches > 50, "corpus should yield many true matches");
        assert!(
            pruned_true_nonmatches > 100,
            "screen has no power: it pruned only {pruned_true_nonmatches} true \
             non-matches"
        );
    }
}
