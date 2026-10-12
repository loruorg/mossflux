//! The VF2++-style subgraph-monomorphism matcher used in production.
//!
//! It answers the same question as [`crate::brute`]: does a [`QueryGraph`] embed
//! into a [`MolGraph`](crate::graph::MolGraph) as a non-induced subgraph
//! monomorphism (injective, every query bond mapped to a satisfying target bond,
//! query non-edges unconstrained)? [`Matcher::matches`] must return exactly what
//! [`crate::brute::matches`] returns on every pair.
//!
//! # Search
//!
//! 1. **Match order.** [`PlannedQuery`] orders the query atoms once,
//!    most-constrained first, so that every atom after a component's root is
//!    bonded to an earlier atom (its *anchor*).
//! 2. **Anchored candidates.** Candidates for a non-root atom are the target
//!    neighbours of its anchor's image whose bond satisfies the anchor bond.
//! 3. **Feasibility.** Before binding `q -> t`: `t` is unused, `deg(t) >= deg(q)`,
//!    `q`'s atom predicate holds on `t`, and every other query bond from `q` to a
//!    bound atom (ring closures) has a satisfying target bond.
//!
//! A [`Matcher`] is reused across targets. Its "target atom used" array is
//! epoch-stamped, so one counter bump clears it and a search does not allocate.
//!
//! # Exactness
//!
//! * The order changes only when atoms are bound, not which maps are legal.
//! * No candidate is skipped: a root tries every target atom, and any valid map
//!   places a non-root `q` on a neighbour of its anchor's image through a bond
//!   satisfying the anchor predicate, which is exactly the enumerated set.
//! * No false accept: each query bond is checked once, at its later-bound
//!   endpoint (the anchor bond by candidate enumeration, the rest as back-bonds).
//! * Degree pruning is a necessary condition: each neighbour of `q` needs a
//!   distinct neighbour of `t`.

use std::time::Instant;

use crate::compact::MatchMol;
use crate::deadline::CHECK_EVERY;
use crate::query::{BondPred, Pred, QueryGraph};

/// Sentinel "no atom" value.
const NONE: u32 = u32::MAX;

/// A query atom predicate compiled once per query for the hot path.
///
/// A predicate that is only ANDs of simple leaves becomes a flat
/// [`Conjunction`](Self::Conjunction); anything with OR, NOT or `$()` keeps the
/// [`Tree`](Self::Tree). Each [`AtomCheck`] equals the matching [`Pred`] leaf's
/// `eval`, so both forms return the same boolean as the original predicate.
#[derive(Clone, Debug)]
enum CompiledAtomPred {
    /// All checks must pass. An empty list (`*`) is `true`.
    Conjunction(Vec<AtomCheck>),
    /// The original predicate, for OR / NOT / recursion.
    Tree(Box<Pred>),
}

/// One leaf check of a [`CompiledAtomPred::Conjunction`], equal to the matching
/// [`Pred`] leaf's `eval`.
#[derive(Clone, Copy, Debug)]
enum AtomCheck {
    /// `C` / `c`: atomic number and aromaticity both match.
    ElementArom { number: u8, aromatic: bool },
    /// `[#n]` — atomic number `== n`, aromaticity unconstrained.
    AtomicNumber(u8),
    /// `a` — aromatic.
    Aromatic,
    /// `A` — aliphatic (not aromatic).
    Aliphatic,
    /// `R` — in at least one ring.
    InRing,
    /// `r<n>` — in an SSSR ring of size exactly `n`.
    RingSize(u8),
    /// `D<n>` — explicit degree exactly `n`.
    Degree(u8),
    /// `H<n>` — total attached-H count exactly `n`.
    TotalH(u8),
    /// `h<n>` — implicit-H count exactly `n`.
    ImplicitH(u8),
    /// `X<n>` — total connectivity (degree + attached H) exactly `n`.
    TotalConnections(u8),
    /// `x<n>` — ring-bond count exactly `n`.
    RingBondCount(u8),
    /// `v<n>` — total bond-order valence exactly `n`.
    Valence(u8),
    /// `R<n>` — SSSR ring-membership count exactly `n`.
    RingCount(u8),
    /// `+`/`-`/`+n` — formal charge exactly `c`.
    Charge(i8),
}

impl AtomCheck {
    /// Evaluates this leaf against atom `i` of `mol`.
    #[inline(always)]
    fn eval(&self, mol: &MatchMol, i: u32) -> bool {
        match *self {
            AtomCheck::ElementArom { number, aromatic } => {
                mol.element(i) == number && mol.is_aromatic(i) == aromatic
            }
            AtomCheck::AtomicNumber(n) => mol.element(i) == n,
            AtomCheck::Aromatic => mol.is_aromatic(i),
            AtomCheck::Aliphatic => !mol.is_aromatic(i),
            AtomCheck::InRing => mol.in_ring(i),
            AtomCheck::RingSize(n) => mol.in_ring_of_size(i, n),
            AtomCheck::Degree(d) => mol.degree(i) == d,
            // Summed in u32, as in `Pred::eval`: the u8 sum can pass 255.
            AtomCheck::TotalH(h) => mol.h_count(i) as u32 + mol.explicit_h(i) as u32 == h as u32,
            AtomCheck::ImplicitH(h) => mol.h_count(i) == h,
            AtomCheck::TotalConnections(x) => {
                mol.degree(i) as u32 + mol.h_count(i) as u32 == x as u32
            }
            AtomCheck::RingBondCount(x) => mol.ring_bond_count(i) == x,
            AtomCheck::Valence(v) => mol.valence(i) == v,
            AtomCheck::RingCount(r) => mol.ring_count(i) == r,
            AtomCheck::Charge(c) => mol.charge(i) == c,
        }
    }

    /// Evaluation order within a conjunction; smaller runs first. Element tests
    /// are cheapest and most selective. Order affects only speed.
    #[inline]
    fn rank(&self) -> u8 {
        match self {
            AtomCheck::ElementArom { .. } => 0,
            AtomCheck::AtomicNumber(_) => 1,
            AtomCheck::Charge(_) => 2,
            AtomCheck::Degree(_) => 3,
            AtomCheck::TotalH(_) | AtomCheck::ImplicitH(_) | AtomCheck::TotalConnections(_) => 4,
            AtomCheck::Aromatic | AtomCheck::Aliphatic => 5,
            AtomCheck::Valence(_) => 4,
            AtomCheck::InRing => 6,
            AtomCheck::RingCount(_) => 6,
            AtomCheck::RingSize(_) | AtomCheck::RingBondCount(_) => 7,
        }
    }
}

impl CompiledAtomPred {
    /// Compiles `pred` into a conjunction when possible, else keeps the tree.
    fn compile(pred: &Pred) -> Self {
        let mut checks = Vec::new();
        if collect_conjunction(pred, &mut checks) {
            checks.sort_by_key(|c| c.rank());
            CompiledAtomPred::Conjunction(checks)
        } else {
            CompiledAtomPred::Tree(Box::new(pred.clone()))
        }
    }

    /// Evaluates against atom `i` of `mol`; equal to [`Pred::eval`].
    #[inline]
    fn eval(&self, mol: &MatchMol, i: u32) -> bool {
        match self {
            CompiledAtomPred::Conjunction(checks) => {
                for c in checks {
                    if !c.eval(mol, i) {
                        return false;
                    }
                }
                true
            }
            CompiledAtomPred::Tree(p) => p.eval(mol, i),
        }
    }
}

/// Flattens `pred` into [`AtomCheck`]s appended to `out`. Returns `true` iff
/// every node is an AND or a simple leaf; on `false`, `out` holds a partial
/// result that the caller must discard.
fn collect_conjunction(pred: &Pred, out: &mut Vec<AtomCheck>) -> bool {
    match pred {
        Pred::Any => true,
        Pred::Element(n) => {
            out.push(AtomCheck::ElementArom {
                number: *n,
                aromatic: false,
            });
            true
        }
        Pred::AromaticElement(n) => {
            out.push(AtomCheck::ElementArom {
                number: *n,
                aromatic: true,
            });
            true
        }
        Pred::AtomicNumber(n) => {
            out.push(AtomCheck::AtomicNumber(*n));
            true
        }
        Pred::Aromatic => {
            out.push(AtomCheck::Aromatic);
            true
        }
        Pred::Aliphatic => {
            out.push(AtomCheck::Aliphatic);
            true
        }
        Pred::InRing => {
            out.push(AtomCheck::InRing);
            true
        }
        Pred::RingSize(n) => {
            out.push(AtomCheck::RingSize(*n));
            true
        }
        Pred::Degree(d) => {
            out.push(AtomCheck::Degree(*d));
            true
        }
        Pred::TotalH(h) => {
            out.push(AtomCheck::TotalH(*h));
            true
        }
        Pred::ImplicitH(h) => {
            out.push(AtomCheck::ImplicitH(*h));
            true
        }
        Pred::TotalConnections(x) => {
            out.push(AtomCheck::TotalConnections(*x));
            true
        }
        Pred::RingBondCount(x) => {
            out.push(AtomCheck::RingBondCount(*x));
            true
        }
        Pred::Valence(v) => {
            out.push(AtomCheck::Valence(*v));
            true
        }
        Pred::RingCount(r) => {
            out.push(AtomCheck::RingCount(*r));
            true
        }
        Pred::Charge(c) => {
            out.push(AtomCheck::Charge(*c));
            true
        }
        // Both sides must flatten.
        Pred::And(l, r) => {
            let lok = collect_conjunction(l, out);
            let rok = collect_conjunction(r, out);
            lok && rok
        }
        Pred::Or(_, _) | Pred::Not(_) | Pred::Recursive(_) => false,
    }
}

/// One query atom in match order, precomputed by [`PlannedQuery::new`].
#[derive(Clone, Debug)]
struct PlanEntry {
    /// Query degree, for the pruning `deg(t) >= deg(q)`.
    degree: u8,
    /// The compiled atom predicate.
    atom_pred: CompiledAtomPred,
    /// An earlier bonded query atom. `None` only for a component root, which is
    /// tried against every target atom.
    anchor: Option<Anchor>,
    /// Query bonds to earlier atoms, **excluding the anchor bond** (candidate
    /// enumeration already checks it). Together they check every query bond once,
    /// at its later-bound endpoint.
    back_bonds: Vec<BackBond>,
}

/// The earlier neighbour used to enumerate candidates for a non-root atom.
#[derive(Clone, Debug)]
struct Anchor {
    /// Match-order position (not query atom id) of the anchor.
    order_pos: u32,
    /// Predicate of the bond to the anchor; filters the anchor image's neighbours.
    bond_pred: CompiledBondPred,
}

/// A query bond from the current atom to an earlier atom.
#[derive(Clone, Debug)]
struct BackBond {
    /// Match-order position of the other endpoint.
    other_order_pos: u32,
    /// Predicate on the target bond between `t` and the other endpoint's image.
    bond_pred: CompiledBondPred,
}

/// A query bond predicate compiled for the hot path: leaves are one integer or
/// flag test; AND / OR / NOT keep the [`BondPred`] tree. Equal to
/// [`BondPred::eval`].
#[derive(Clone, Debug)]
enum CompiledBondPred {
    /// `~` — any bond.
    Any,
    /// A specific bond order (`bond.order == o`).
    Order(u8),
    /// `:` — aromatic bond.
    Aromatic,
    /// `@` — ring bond.
    Ring,
    /// Default adjacency: single **or** aromatic.
    SingleOrAromatic,
    /// The original predicate, for AND / OR / NOT.
    Tree(Box<BondPred>),
}

impl CompiledBondPred {
    /// Compiles a [`BondPred`].
    fn compile(pred: &BondPred) -> Self {
        match pred {
            BondPred::Any => CompiledBondPred::Any,
            BondPred::Order(o) => CompiledBondPred::Order(*o),
            BondPred::Aromatic => CompiledBondPred::Aromatic,
            BondPred::Ring => CompiledBondPred::Ring,
            BondPred::SingleOrAromatic => CompiledBondPred::SingleOrAromatic,
            BondPred::And(..) | BondPred::Or(..) | BondPred::Not(..) => {
                CompiledBondPred::Tree(Box::new(pred.clone()))
            }
        }
    }

    /// Evaluates against a target bond's order and flags; equal to
    /// [`BondPred::eval`].
    #[inline(always)]
    fn eval_of(&self, order: u8, flags: u8) -> bool {
        use crate::graph::{bond_flags, bond_order};
        match self {
            CompiledBondPred::Any => true,
            CompiledBondPred::Order(o) => order == *o,
            CompiledBondPred::Aromatic => flags & bond_flags::AROMATIC != 0,
            CompiledBondPred::Ring => flags & bond_flags::IN_RING != 0,
            CompiledBondPred::SingleOrAromatic => {
                order == bond_order::SINGLE || (flags & bond_flags::AROMATIC != 0)
            }
            // `BondPred::eval` reads only order and flags; endpoints are unused.
            CompiledBondPred::Tree(p) => p.eval(&crate::graph::Bond {
                a: 0,
                b: 0,
                order,
                flags,
            }),
        }
    }
}

/// A query compiled into a search plan, built once and reused across targets.
#[derive(Clone, Debug)]
pub struct PlannedQuery {
    /// Atoms in match order.
    plan: Vec<PlanEntry>,
    /// For a query that is one ring of bare aromatic atoms with default bonds:
    /// its [`crate::graph::canonical_ring_token`]. A target aromatic ring with
    /// this token is a proven match; a miss falls through to the full search.
    ring_skip: Option<Box<[u8]>>,
    /// [`crate::graph::token_hash`] of `ring_skip` (0 if none).
    ring_skip_hash: u64,
    /// A census class whose count ≥ 1 proves a match, so verification may accept
    /// without search. Never used to reject; see
    /// [`crate::census::census_early_accept_class`].
    early_accept_class: Option<usize>,
    /// Match-order position where each connected component starts. A component's
    /// atoms are contiguous.
    component_starts: Vec<usize>,
}

impl PlannedQuery {
    /// Compiles `query` into a search plan.
    ///
    /// Each component starts at its most selective atom (see `root_rarity_rank`,
    /// then highest degree). It then grows by the unordered atom with the most
    /// bonds to ordered atoms, then highest degree, then lowest index, so every
    /// non-root atom has an anchor. Order affects only speed.
    ///
    /// An empty query gives an empty plan, which matches every target.
    pub fn new(query: &QueryGraph) -> Self {
        let n = query.atom_count();
        let mut plan: Vec<PlanEntry> = Vec::with_capacity(n);

        // Match-order position of each query atom, or NONE while unordered.
        let mut order_pos_of = vec![NONE; n];

        let degree: Vec<u32> = (0..n).map(|i| query.incident[i].len() as u32).collect();

        // Bonds from each unordered atom to the ordered set.
        let mut back_count = vec![0u32; n];

        // One iteration per connected component.
        while plan.len() < n {
            // Root: rarest forced element, then highest degree, then lowest index.
            let mut root = NONE;
            let mut root_rank = u32::MAX;
            let mut root_deg = 0u32;
            for q in 0..n {
                if order_pos_of[q] != NONE {
                    continue;
                }
                let rank = root_rarity_rank(&query.atoms[q]);
                if root == NONE || rank < root_rank || (rank == root_rank && degree[q] > root_deg) {
                    root = q as u32;
                    root_rank = rank;
                    root_deg = degree[q];
                }
            }
            debug_assert!(
                root != NONE,
                "an unordered atom must exist while plan is short"
            );

            place(
                query,
                root,
                None,
                &mut plan,
                &mut order_pos_of,
                &mut back_count,
            );

            // Grow the component until no unordered atom is adjacent to it.
            loop {
                let mut best = NONE;
                let mut best_back = 0u32;
                let mut best_deg = 0u32;
                for q in 0..n {
                    if order_pos_of[q] != NONE {
                        continue;
                    }
                    let bc = back_count[q];
                    if bc == 0 {
                        continue; // not adjacent to the ordered set
                    }
                    // Strict `>` keeps the lowest index on ties.
                    if best == NONE || bc > best_back || (bc == best_back && degree[q] > best_deg) {
                        best = q as u32;
                        best_back = bc;
                        best_deg = degree[q];
                    }
                }
                if best == NONE {
                    break; // component done
                }
                let anchor = pick_anchor(query, best, &order_pos_of);
                place(
                    query,
                    best,
                    Some(anchor),
                    &mut plan,
                    &mut order_pos_of,
                    &mut back_count,
                );
            }
        }

        let ring_skip = detect_aromatic_ring_skip(query);
        let ring_skip_hash = ring_skip
            .as_ref()
            .map(|t| crate::graph::token_hash(t))
            .unwrap_or(0);
        let component_starts = plan
            .iter()
            .enumerate()
            .filter(|(_, e)| e.anchor.is_none())
            .map(|(i, _)| i)
            .collect();
        PlannedQuery {
            plan,
            ring_skip,
            ring_skip_hash,
            early_accept_class: crate::census::census_early_accept_class(query),
            component_starts,
        }
    }

    /// The census class, if any, whose presence proves a match.
    #[inline]
    pub(crate) fn early_accept_class(&self) -> Option<usize> {
        self.early_accept_class
    }

    /// Number of query atoms.
    #[inline]
    fn len(&self) -> usize {
        self.plan.len()
    }
}

/// Appends query atom `q` to the match order with its anchor and back-bonds, and
/// updates `back_count` of its unordered neighbours.
fn place(
    query: &QueryGraph,
    q: u32,
    anchor: Option<Anchor>,
    plan: &mut Vec<PlanEntry>,
    order_pos_of: &mut [u32],
    back_count: &mut [u32],
) {
    let pos = plan.len() as u32;
    order_pos_of[q as usize] = pos;

    // Back-bonds: incident bonds to ordered atoms, except the anchor bond, which
    // candidate enumeration in `search` already checks. With no parallel edges,
    // skipping the anchor atom skips exactly that one bond.
    let anchor_pos = anchor.as_ref().map(|a| a.order_pos);
    let mut back_bonds = Vec::new();
    for &bond_idx in &query.incident[q as usize] {
        let b = &query.bonds[bond_idx as usize];
        let other = if b.a == q { b.b } else { b.a };
        let other_pos = order_pos_of[other as usize];
        if other_pos != NONE && Some(other_pos) != anchor_pos {
            back_bonds.push(BackBond {
                other_order_pos: other_pos,
                bond_pred: CompiledBondPred::compile(&b.pred),
            });
        }
    }

    plan.push(PlanEntry {
        degree: query.incident[q as usize].len().min(u8::MAX as usize) as u8,
        atom_pred: CompiledAtomPred::compile(&query.atoms[q as usize]),
        anchor,
        back_bonds,
    });

    for &bond_idx in &query.incident[q as usize] {
        let b = &query.bonds[bond_idx as usize];
        let other = if b.a == q { b.b } else { b.a };
        if order_pos_of[other as usize] == NONE {
            back_count[other as usize] += 1;
        }
    }
}

/// Root selectivity rank on a drug-like corpus: lower means rarer, so fewer root
/// candidates. Keyed on the forced label; an unforced atom ranks last. Affects
/// only speed.
fn root_rarity_rank(pred: &Pred) -> u32 {
    match crate::screen::forced_atom_label(pred) {
        // Halogens.
        Some((9 | 17 | 35 | 53, _)) => 0,
        // Carbon is the most common element.
        Some((6, true)) => 4,
        Some((6, false)) => 5,
        // Other heteroatoms.
        Some((_, true)) => 1,
        Some((_, false)) => 2,
        None => 6,
    }
}

/// True iff `tok` (with hash `h`) is in the parallel `(hashes, tokens)` arrays.
/// The hash is only a prefilter; the byte compare decides, so a collision cannot
/// cause a false accept.
fn ring_token_present(hashes: &[u64], tokens: &[Box<[u8]>], h: u64, tok: &[u8]) -> bool {
    hashes
        .iter()
        .zip(tokens.iter())
        .any(|(&rh, r)| rh == h && r.as_ref() == tok)
}

/// Returns the canonical ring token if `query` is one simple cycle of bare
/// `Pred::AromaticElement` atoms joined by default single-or-aromatic bonds.
///
/// A target aromatic ring with the same token is then a valid embedding: each
/// atom predicate holds on the same-element aromatic ring atom, and each aromatic
/// ring bond satisfies single-or-aromatic. A miss falls through to the full
/// search, so a ring missing from the target's ring set costs only time. Any extra
/// atom constraint (`[c;R]`, `[cH]`) is not in the token, so such queries are
/// excluded.
fn detect_aromatic_ring_skip(query: &QueryGraph) -> Option<Box<[u8]>> {
    let n = query.atom_count();
    // A single cycle has as many bonds as atoms.
    if n < 3 || query.bonds.len() != n {
        return None;
    }
    for q in 0..n {
        if query.incident[q].len() != 2 {
            return None; // every ring atom has degree exactly 2
        }
        if !matches!(query.atoms[q], Pred::AromaticElement(_)) {
            return None; // must be a bare aromatic element, no extra constraints
        }
    }
    for b in &query.bonds {
        if !matches!(b.pred, BondPred::SingleOrAromatic) {
            return None;
        }
    }
    // Walk from atom 0 to get the cyclic order and confirm one cycle covers all
    // atoms (not several disjoint cycles).
    let mut order: Vec<u32> = Vec::with_capacity(n);
    let mut cur = 0u32;
    let mut prev = NONE;
    loop {
        order.push(cur);
        let mut next = NONE;
        for &bi in &query.incident[cur as usize] {
            let b = &query.bonds[bi as usize];
            let other = if b.a == cur { b.b } else { b.a };
            if other != prev {
                next = other;
                break;
            }
        }
        if next == NONE {
            return None;
        }
        if next == 0 {
            break; // closed the cycle back to the start
        }
        if order.len() >= n {
            return None; // not a simple cycle
        }
        prev = cur;
        cur = next;
    }
    if order.len() != n {
        return None; // a sub-cycle that did not cover every atom
    }
    let labels: Vec<(u8, bool)> = order
        .iter()
        .map(|&q| match query.atoms[q as usize] {
            Pred::AromaticElement(e) => (e, true),
            _ => unreachable!("filtered to AromaticElement above"),
        })
        .collect();
    Some(crate::graph::canonical_ring_token(&labels))
}

/// Picks the anchor for `q`: its ordered neighbour with the smallest match-order
/// position.
///
/// # Panics
/// Panics if `q` has no ordered neighbour. The caller only passes atoms with
/// `back_count > 0`.
fn pick_anchor(query: &QueryGraph, q: u32, order_pos_of: &[u32]) -> Anchor {
    // (order position, bond index)
    let mut best: Option<(u32, u32)> = None;
    for &bond_idx in &query.incident[q as usize] {
        let b = &query.bonds[bond_idx as usize];
        let other = if b.a == q { b.b } else { b.a };
        let other_pos = order_pos_of[other as usize];
        if other_pos == NONE {
            continue;
        }
        match best {
            Some((cur_pos, _)) if cur_pos <= other_pos => {}
            _ => best = Some((other_pos, bond_idx)),
        }
    }
    let (order_pos, bond) = best.expect("pick_anchor called on an atom with no ordered neighbour");
    Anchor {
        order_pos,
        bond_pred: CompiledBondPred::compile(&query.bonds[bond as usize].pred),
    }
}

/// Reusable search state. Reuse one across a corpus so searches do not allocate.
///
/// A matcher may carry a deadline ([`set_deadline`](Self::set_deadline)). Once it
/// passes, [`matches`](Self::matches) returns `false` without finishing, and
/// [`timed_out`](Self::timed_out) turns `true`: a caller that set a deadline must
/// check `timed_out` before it trusts a `false`.
#[derive(Clone, Debug, Default)]
pub struct Matcher {
    /// `used_epoch[t] == epoch` iff target atom `t` is mapped. Grow-only.
    used_epoch: Vec<u32>,
    /// The current epoch; bumping it clears `used_epoch`.
    epoch: u32,
    /// `assign[pos]` is the target atom bound at match-order position `pos`.
    /// Grow-only.
    assign: Vec<u32>,
    /// One past the last match-order position the current search binds: the plan
    /// length, or the end of one component during the per-component pre-check.
    end: usize,
    /// When to give up, if ever.
    deadline: Option<Instant>,
    /// Steps since the matcher was made; the clock is read every `CHECK_EVERY`.
    steps: u32,
    /// Set once the deadline has passed (here, or in a recursive environment on
    /// this thread).
    timed_out: bool,
}

impl Matcher {
    /// A matcher with empty scratch.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets (or with `None`, clears) the deadline and clears [`Self::timed_out`].
    pub fn set_deadline(&mut self, deadline: Option<Instant>) {
        self.deadline = deadline;
        self.timed_out = false;
    }

    /// True once the deadline passed during a search. Every `false` that
    /// [`matches`](Self::matches) returned since then may be wrong.
    pub fn timed_out(&self) -> bool {
        self.timed_out
    }

    /// The slow half of the step counter: reads the clock (and the thread's
    /// recursive-environment flag). After a timeout it keeps every later step on
    /// this path, so the search unwinds without a branch on the hot path.
    #[cold]
    fn deadline_passed(&mut self) -> bool {
        if !self.timed_out {
            let passed =
                self.deadline.is_some_and(|d| Instant::now() >= d) || crate::deadline::expired();
            if !passed {
                return false;
            }
            self.timed_out = true;
        }
        self.steps = CHECK_EVERY - 1;
        true
    }

    /// Starts a new logical search: bumps the epoch, which clears `used_epoch`.
    fn fresh_epoch(&mut self) {
        // On overflow, reset the stamps so a stale one cannot alias the new epoch.
        self.epoch = match self.epoch.checked_add(1) {
            Some(e) => e,
            None => {
                for slot in &mut self.used_epoch {
                    *slot = 0;
                }
                1
            }
        };
    }

    /// Returns `true` iff `planned` embeds into `mol` as a subgraph monomorphism
    /// (the [module definition](self)), reusing this matcher's scratch.
    ///
    /// Stops at the first embedding. An empty query returns `true`. Must equal
    /// [`crate::brute::matches`].
    pub fn matches(&mut self, planned: &PlannedQuery, mol: &MatchMol) -> bool {
        if self.timed_out {
            return false;
        }
        let k = planned.len();
        if k == 0 {
            return true;
        }
        // No injection into fewer atoms.
        if k > mol.atom_count() {
            return false;
        }

        // Ring-skip: a hit is a proven match (see `detect_aromatic_ring_skip`); a
        // miss falls through to the full search.
        if let Some(tok) = &planned.ring_skip {
            if ring_token_present(
                &mol.arom_ring_hashes,
                &mol.arom_rings,
                planned.ring_skip_hash,
                tok,
            ) {
                return true;
            }
        }

        let n = mol.atom_count();
        if self.used_epoch.len() < n {
            self.used_epoch.resize(n, 0);
        }
        // No reset needed: `assign[pos]` is always written before it is read,
        // since back-bonds and anchors read only earlier positions.
        if self.assign.len() < k {
            self.assign.resize(k, NONE);
        }

        // Each component must embed alone (a necessary condition). Checking that
        // first avoids an exponential search over the other components when one
        // cannot embed (e.g. `C.C.C.C.C.[C+]` on a neutral chain).
        let starts = &planned.component_starts;
        if starts.len() > 1 {
            for (c, &start) in starts.iter().enumerate() {
                self.end = starts.get(c + 1).copied().unwrap_or(k);
                self.fresh_epoch();
                if !self.search(planned, mol, start) {
                    return false;
                }
            }
        }
        self.end = k;
        self.fresh_epoch();
        self.search(planned, mol, 0)
    }

    /// Backtracking from match-order position `pos` up to `self.end`. Returns
    /// `true` on the first complete embedding.
    fn search(&mut self, planned: &PlannedQuery, mol: &MatchMol, pos: usize) -> bool {
        if pos == self.end {
            return true;
        }
        let entry = &planned.plan[pos];

        match &entry.anchor {
            // Component root: try every target atom.
            None => {
                for t in 0..mol.atom_count() as u32 {
                    if self.try_assign(planned, mol, pos, t) {
                        return true;
                    }
                }
                false
            }
            // Anchored: neighbours of the anchor's image through a satisfying bond.
            Some(anchor) => {
                let anchor_target = self.assign[anchor.order_pos as usize];
                debug_assert!(
                    anchor_target != NONE,
                    "anchor must be placed before its dependent atom"
                );
                let anchor_pred = &anchor.bond_pred;
                for nbr in mol.neighbors(anchor_target) {
                    // The only check of the anchor bond: back-bonds exclude it.
                    if !anchor_pred.eval_of(nbr.order, nbr.flags) {
                        continue;
                    }
                    if self.try_assign(planned, mol, pos, nbr.atom) {
                        return true;
                    }
                }
                false
            }
        }
    }

    /// Tries to bind position `pos` to target atom `t`: checks feasibility, then
    /// recurses and backtracks. Returns `true` iff an embedding extends `pos -> t`.
    #[inline]
    fn try_assign(&mut self, planned: &PlannedQuery, mol: &MatchMol, pos: usize, t: u32) -> bool {
        // The clock is read only every `CHECK_EVERY` steps.
        self.steps = self.steps.wrapping_add(1);
        if self.steps & (CHECK_EVERY - 1) == 0 && self.deadline_passed() {
            return false;
        }
        let entry = &planned.plan[pos];

        // Injectivity.
        if self.used_epoch[t as usize] == self.epoch {
            return false;
        }
        // Degree pruning: each query neighbour needs a distinct target neighbour.
        if (mol.degree(t) as u32) < entry.degree as u32 {
            return false;
        }
        if !entry.atom_pred.eval(mol, t) {
            return false;
        }
        // Non-anchor bonds to bound atoms (ring closures).
        if !self.back_bonds_ok(mol, entry, t) {
            return false;
        }

        self.assign[pos] = t;
        self.used_epoch[t as usize] = self.epoch;
        if self.search(planned, mol, pos + 1) {
            return true;
        }
        // Backtrack. A stale `assign[pos]` is never read.
        self.used_epoch[t as usize] = 0;
        false
    }

    /// True iff every back-bond of `entry` has a satisfying target bond between
    /// `t` and the other endpoint's image.
    #[inline]
    fn back_bonds_ok(&self, mol: &MatchMol, entry: &PlanEntry, t: u32) -> bool {
        for bb in &entry.back_bonds {
            let other_target = self.assign[bb.other_order_pos as usize];
            // Back-bonds reference only earlier, bound positions.
            debug_assert!(other_target != NONE);
            let pred = &bb.bond_pred;
            if !target_bond_ok(mol, t, other_target, pred) {
                return false;
            }
        }
        true
    }
}

/// True iff target atoms `a` and `b` are bonded and the bond satisfies `pred`.
#[inline]
fn target_bond_ok(mol: &MatchMol, a: u32, b: u32, pred: &CompiledBondPred) -> bool {
    for nbr in mol.neighbors(a) {
        if nbr.atom == b {
            return pred.eval_of(nbr.order, nbr.flags);
        }
    }
    false
}

/// Plans `query` and matches it against `mol` once. For many targets, reuse a
/// [`PlannedQuery`] and a [`Matcher`] instead.
pub fn matches(query: &QueryGraph, mol: &crate::graph::MolGraph) -> bool {
    let planned = PlannedQuery::new(query);
    let compact = MatchMol::from_molgraph(mol);
    Matcher::new().matches(&planned, &compact)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brute;
    use crate::compact::MatchMol;
    use crate::query::{BondPred, Pred, QueryGraphBuilder};
    use crate::smarts::parse as parse_smarts;
    use crate::smiles::parse as parse_smiles;

    #[test]
    fn ring_token_prefilter_byte_compare_is_the_decider() {
        // A hash collision must not cause a ring-skip accept. Two tokens share one
        // hash here.
        let tok_a: Box<[u8]> = vec![1, 2, 3].into_boxed_slice();
        let tok_b: Box<[u8]> = vec![9, 9, 9].into_boxed_slice();
        let tokens = vec![tok_a.clone(), tok_b.clone()];
        let collide = 0x1234_5678_9abc_def0u64;
        let hashes = vec![collide, collide];

        assert!(ring_token_present(&hashes, &tokens, collide, &tok_b));
        assert!(ring_token_present(&hashes, &tokens, collide, &tok_a));
        // Absent token with the colliding hash: the byte compare rejects it.
        let absent: Box<[u8]> = vec![7, 7, 7].into_boxed_slice();
        assert!(!ring_token_present(&hashes, &tokens, collide, &absent));
        // The hash must match too.
        assert!(!ring_token_present(&hashes, &tokens, collide ^ 1, &tok_a));
        // A real token and its `token_hash`.
        let t = crate::graph::canonical_ring_token(&[(6, true); 6]);
        let h = crate::graph::token_hash(&t);
        assert!(ring_token_present(&[h], std::slice::from_ref(&t), h, &t));
    }

    #[test]
    fn ring_skip_equals_oracle_on_adversarial_rings() {
        // The ring-skip path must equal the brute-force oracle, including aromatic
        // vs aliphatic rings, hetero-ring identity, Kekulé input, fused systems that
        // contain the ring, and 5-rings against 6-ring queries.
        let ring_queries = [
            "c1ccccc1",    // benzene
            "c1ccncc1",    // pyridine (c5 n1)
            "c1ccncn1",    // pyrimidine (c4 n2)
            "c1cnccn1",    // pyrazine
            "c1cc[nH]c1",  // pyrrole (5-ring)
            "c1ccoc1",     // furan
            "C1=CC=CC=C1", // Kekulé benzene as a query
        ];
        let molecules = [
            "c1ccccc1",
            "Cc1ccccc1",
            "Oc1ccccc1",
            "C1CCCCC1",
            "c1ccncc1",
            "c1ccncn1",
            "c1cnccn1",
            "c1cc[nH]c1",
            "c1ccoc1",
            "c1ccc2ccccc2c1",
            "c1ccc2ncccc2c1",
            "C1=CC=CC=C1",
            "Nc1ccccc1",
            "FC(F)(F)c1ccccc1",
            "CCO",
            "c1ccc(nc1)N",
            "c1ccc2[nH]ccc2c1",
        ];
        for qs in ring_queries {
            let q = parse_smarts(qs).unwrap();
            let planned = PlannedQuery::new(&q);
            for ms in molecules {
                let mol = parse_smiles(ms).unwrap();
                let mut m = Matcher::new();
                assert_eq!(
                    m.matches(&planned, &MatchMol::from_molgraph(&mol)),
                    brute::matches(&q, &mol),
                    "ring-skip matcher != oracle for query {qs:?} on molecule {ms:?}"
                );
            }
        }
    }

    /// Parses both strings and runs the matcher.
    fn hits(smarts: &str, smiles: &str) -> bool {
        let mol = parse_smiles(smiles).expect("target SMILES parses");
        let q = parse_smarts(smarts).expect("query SMARTS parses");
        matches(&q, &mol)
    }

    // --- Spot checks ---

    #[test]
    fn benzene_matches_toluene_and_naphthalene() {
        assert!(hits("c1ccccc1", "Cc1ccccc1"));
        assert!(hits("c1ccccc1", "c1ccc2ccccc2c1"));
    }

    #[test]
    fn known_nonmatches() {
        assert!(!hits("c1ccccc1", "C1CCCCC1"));
        assert!(!hits("c1ccccc1", "CCO"));
        assert!(!hits("a", "CCC"));
        assert!(!hits("O", "C"));
    }

    #[test]
    fn bond_and_injectivity_semantics() {
        assert!(!hits("C=C", "CC"));
        assert!(hits("C=C", "C=C"));
        assert!(hits("C~C", "CC"));
        // Injectivity: C-C cannot collapse onto methane's single atom.
        assert!(!hits("CC", "C"));
        // Query bigger than target.
        assert!(!hits("CCC", "CC"));
    }

    #[test]
    fn empty_query_matches_anything() {
        let q = QueryGraphBuilder::new().finish();
        let mol = parse_smiles("CCO").unwrap();
        assert!(matches(&q, &mol));
    }

    #[test]
    fn disconnected_query_plans_two_roots_and_matches() {
        // [#6].[#8]: two components, so two roots.
        let mut qb = QueryGraphBuilder::new();
        qb.add_atom(Pred::AtomicNumber(6));
        qb.add_atom(Pred::AtomicNumber(8));
        let q = qb.finish();
        let planned = PlannedQuery::new(&q);
        assert!(planned.plan.iter().all(|e| e.anchor.is_none()));
        let mol = parse_smiles("CO").unwrap();
        assert!(Matcher::new().matches(&planned, &MatchMol::from_molgraph(&mol)));
        let methane = parse_smiles("C").unwrap();
        assert!(!Matcher::new().matches(&planned, &MatchMol::from_molgraph(&methane)));
    }

    #[test]
    fn matcher_is_reusable_without_alloc_between_targets() {
        // One matcher over several targets: the epoch reset must leak no state.
        let q = parse_smarts("c1ccccc1").unwrap();
        let planned = PlannedQuery::new(&q);
        let mut m = Matcher::new();
        let targets = ["Cc1ccccc1", "C1CCCCC1", "c1ccc2ccccc2c1", "CCO", "c1ccncc1"];
        let expected = [true, false, true, false, false];
        for (s, &want) in targets.iter().zip(&expected) {
            let mol = parse_smiles(s).unwrap();
            assert_eq!(
                m.matches(&planned, &MatchMol::from_molgraph(&mol)),
                want,
                "target {s}"
            );
        }
        // A second pass through the same matcher gives the same results.
        for (s, &want) in targets.iter().zip(&expected) {
            let mol = parse_smiles(s).unwrap();
            assert_eq!(
                m.matches(&planned, &MatchMol::from_molgraph(&mol)),
                want,
                "rerun target {s}"
            );
        }
    }

    #[test]
    fn plan_keeps_connected_frontier() {
        // In a connected query, every entry after the first has an anchor.
        for s in [
            "c1ccccc1",
            "CCO",
            "C1CCC(N)CC1",
            "c1ccc2ccccc2c1",
            "CC(C)C(=O)O",
        ] {
            let q = parse_smarts(s).unwrap();
            let planned = PlannedQuery::new(&q);
            for (i, e) in planned.plan.iter().enumerate() {
                if i == 0 {
                    assert!(e.anchor.is_none(), "{s}: first atom is a root");
                } else {
                    assert!(e.anchor.is_some(), "{s}: atom #{i} must be anchored");
                }
            }
        }
    }

    // --- Exactness gate: vf2 == brute over a corpus ---

    /// Chains, rings, fused and bridged systems, heteroaromatics, charges,
    /// repeated elements and symmetric skeletons.
    fn target_corpus() -> Vec<&'static str> {
        vec![
            // chains / branches
            "C",
            "CC",
            "CCC",
            "CCCC",
            "CCO",
            "OCCO",
            "CC(C)C",
            "CC(C)(C)C",
            "CCN",
            "CC(=O)O",
            "CC(=O)N",
            "CCOCC",
            "CC(C)C(=O)O",
            "NCCCN",
            "ClCCBr",
            // simple rings
            "C1CCCCC1",
            "C1CCCC1",
            "C1CCC1",
            "C1CCCCCC1",
            "C1CCNCC1",
            "O1CCCC1",
            // aromatics + heteroaromatics
            "c1ccccc1",
            "Cc1ccccc1",
            "c1ccncc1",
            "c1ccc(O)cc1",
            "c1cc[nH]c1",
            "c1ccoc1",
            "c1ccsc1",
            "Oc1ccccc1O",
            "Nc1ccccc1",
            "c1ccc(N)cc1",
            // fused / bridged / spiro
            "c1ccc2ccccc2c1",
            "c1ccc2[nH]ccc2c1",
            "C1CC2CCC1CC2",
            "C1CCC2(CC1)CCCC2",
            "C1=CC=CC=C1",
            "C1=CC=CC1",
            // charges / brackets
            "[NH4+]",
            "CC[O-]",
            "[O-]C(=O)C",
            "c1cc[n+]([O-])cc1",
            // repeated elements / symmetry
            "FC(F)(F)F",
            "ClC(Cl)Cl",
            "OCC(O)CO",
            "C1CC1C1CC1",
        ]
    }

    /// Atoms, paths, rings, logical expressions, bond primitives, disconnected
    /// fragments and recursive SMARTS, with both hits and misses on the corpus.
    fn query_corpus() -> Vec<&'static str> {
        vec![
            // single-atom predicates
            "C",
            "c",
            "N",
            "n",
            "O",
            "o",
            "S",
            "F",
            "[#6]",
            "[#7]",
            "[#8]",
            "a",
            "A",
            "[R]",
            "[r6]",
            "[r5]",
            "[D3]",
            "[D2]",
            "[H1]",
            "[H2]",
            "[O-]",
            "[N+]",
            "[c]",
            "[n]",
            "[!C]",
            "[C,N]",
            "[#7,#8]",
            "[c;R]",
            // two-atom paths with various bonds
            "CC",
            "CO",
            "CN",
            "C=C",
            "C=O",
            "C#N",
            "C~C",
            "cc",
            "co",
            "cn",
            "c:c",
            "C-C",
            "[#6][#8]",
            "[#6]=[#8]",
            "[c][c]",
            "a~a",
            // branched / longer paths
            "CCC",
            "CCO",
            "CC(C)C",
            "CC=O",
            "CC(=O)O",
            "OCC",
            "NCC",
            "C(C)(C)C",
            "ccc",
            "cccc",
            "c1ccccc1",
            "c1ccncc1",
            "C1CCCCC1",
            "C1CCC1",
            "c1ccc(O)cc1",
            "[#6]1[#6][#6][#6][#6][#6]1",
            // ring + property combos
            "[c;R]1[c;R][c;R][c;R][c;R][c;R]1",
            "[R]~[R]",
            "[#7;R]",
            "c[OH]",
            "[OH]",
            "[NH2]",
            "C[F,Cl,Br]",
            // disconnected fragments
            "C.C",
            "C.O",
            "[#6].[#7]",
            "c1ccccc1.O",
            // bond logic + negation
            "C-,=C",
            "C!@C",
            "[!a]",
            "[!c]~[!c]",
            // recursive SMARTS: plain, combined, negated, OR-ed, ring-rooted
            "[$(C=O)]",
            "[$([OH])]",
            "[$(c1ccccc1)]",
            "[C$(C=O)]",
            "[!$(C=O)]",
            "[$(*=O)]",
            "[$(CO),$(CN)]",
            "[O$(*=O)]",
            "[$(C=O)]~[$([OH])]",
            "[$(cc)]",
            "[$([#7]);!$([#7]=O)]",
            "[$(C[OH])]",
            "[$(*N)]",
        ]
    }

    /// For every (target, query) pair, the matcher returns exactly what the
    /// brute-force oracle returns on the same parsed graphs.
    #[test]
    fn vf2_equals_brute_over_corpus() {
        let targets = target_corpus();
        let queries = query_corpus();

        // One matcher for the whole sweep exercises the epoch reset.
        let mut matcher = Matcher::new();
        let mut pairs = 0usize;
        let mut hits_count = 0usize;

        for qs in &queries {
            let q = match parse_smarts(qs) {
                Ok(q) => q,
                Err(e) => panic!("query {qs:?} should parse for the gate: {e}"),
            };
            let planned = PlannedQuery::new(&q);

            for ts in &targets {
                let mol = parse_smiles(ts)
                    .unwrap_or_else(|e| panic!("target {ts:?} should parse for the gate: {e}"));

                let fast = matcher.matches(&planned, &MatchMol::from_molgraph(&mol));
                let slow = brute::matches(&q, &mol);
                assert_eq!(
                    fast, slow,
                    "VF2 disagrees with oracle on query {qs:?} vs target {ts:?}: \
                     vf2={fast} brute={slow}"
                );
                pairs += 1;
                if slow {
                    hits_count += 1;
                }
            }
        }

        // The sweep must be large and contain both outcomes.
        assert!(
            pairs > 1000,
            "the gate should compare many pairs, got {pairs}"
        );
        assert!(hits_count > 50, "the corpus should yield many true matches");
        assert!(
            hits_count < pairs,
            "the corpus should also yield non-matches, not all-hits"
        );
    }

    // --- Targeted cases (also in the sweep; named here for clear failures) ---

    #[test]
    fn symmetry_benzene_into_benzene() {
        let q = parse_smarts("c1ccccc1").unwrap();
        let mol = parse_smiles("c1ccccc1").unwrap();
        assert_eq!(matches(&q, &mol), brute::matches(&q, &mol));
        assert!(matches(&q, &mol));
    }

    #[test]
    fn repeated_elements_perfluoro() {
        // Four identical F neighbours: a carbon with four F's must embed.
        for (qs, want) in [("C(F)(F)(F)F", true), ("FC(F)(F)F", true)] {
            let q = parse_smarts(qs).unwrap();
            let mol = parse_smiles("FC(F)(F)F").unwrap();
            assert_eq!(matches(&q, &mol), brute::matches(&q, &mol), "{qs}");
            assert_eq!(matches(&q, &mol), want, "{qs}");
        }
    }

    #[test]
    fn ring_query_into_fused_system() {
        // A 6-ring query into fused, spiro and bridged systems; agree with the
        // oracle.
        let q = parse_smarts("C1CCCCC1").unwrap();
        for ts in ["c1ccc2ccccc2c1", "C1CCC2(CC1)CCCC2", "C1CC2CCC1CC2"] {
            let mol = parse_smiles(ts).unwrap();
            assert_eq!(matches(&q, &mol), brute::matches(&q, &mol), "{ts}");
        }
    }

    #[test]
    fn directly_built_query_with_ring_closure_predicate() {
        // A hand-built triangle exercises the ring-closure back-bond: it embeds in
        // cyclopropane but not propane.
        let mut qb = QueryGraphBuilder::new();
        let a = qb.add_atom(Pred::AtomicNumber(6));
        let b = qb.add_atom(Pred::AtomicNumber(6));
        let c = qb.add_atom(Pred::AtomicNumber(6));
        qb.add_bond(a, b, BondPred::Any);
        qb.add_bond(b, c, BondPred::Any);
        qb.add_bond(c, a, BondPred::Any); // the closing bond
        let q = qb.finish();

        let tri = parse_smiles("C1CC1").unwrap();
        let chain = parse_smiles("CCC").unwrap();
        assert_eq!(matches(&q, &tri), brute::matches(&q, &tri));
        assert!(matches(&q, &tri), "triangle embeds into cyclopropane");
        assert_eq!(matches(&q, &chain), brute::matches(&q, &chain));
        assert!(!matches(&q, &chain), "triangle needs the closing bond");
    }
}
