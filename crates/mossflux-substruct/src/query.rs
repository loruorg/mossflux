//! The query (SMARTS) graph: [`QueryGraph`], plus the atom-predicate tree
//! [`Pred`] and bond-predicate tree [`BondPred`].
//!
//! A query has the topology of a [`MolGraph`](crate::graph::MolGraph), but each
//! atom and bond carries a predicate tree instead of concrete properties. The
//! SMARTS logical operators (`!`, `&`, `,`, `;`) lower onto [`Pred::And`],
//! [`Pred::Or`] and [`Pred::Not`]. Evaluation is a recursive walk over small trees.

use crate::graph::{bond_order, Bond};

/// An atom predicate: a boolean test over a target atom's perceived properties.
///
/// Leaves are SMARTS atom primitives. The SMARTS parser handles operator
/// precedence; this type defines evaluation only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pred {
    /// `*` — matches any atom.
    Any,
    /// An aliphatic element, e.g. `C`: this atomic number and not aromatic.
    Element(u8),
    /// An aromatic element, e.g. `c`: this atomic number and aromatic.
    AromaticElement(u8),
    /// `#n` — matches any atom with atomic number `n`, regardless of aromaticity.
    AtomicNumber(u8),
    /// `a` — matches any aromatic atom.
    Aromatic,
    /// `A` — matches any aliphatic (non-aromatic) atom.
    Aliphatic,
    /// `R` — matches any atom that is in at least one ring.
    InRing,
    /// `r<n>` — matches an atom in an SSSR ring of size exactly `n`.
    ///
    /// Reads the perceived ring-size data (see
    /// [`MolGraph::in_ring_of_size`](crate::graph::MolGraph::in_ring_of_size)); on an
    /// unperceived graph it is `false`. The screen never relies on this predicate.
    RingSize(u8),
    /// `D<n>` — matches an atom of explicit degree exactly `n`.
    Degree(u8),
    /// `H<n>` — matches an atom whose total attached-H count is exactly `n`.
    TotalH(u8),
    /// `h<n>` — implicit plus bracket H count; see
    /// [`crate::graph::AtomView::av_h_count`].
    ImplicitH(u8),
    /// `X<n>` — total connectivity: `degree + H`.
    TotalConnections(u8),
    /// `x<n>` — number of the atom's ring bonds.
    RingBondCount(u8),
    /// `v<n>` — total bond-order valence.
    Valence(u8),
    /// `R<n>` — SSSR ring-membership count (`R0` = acyclic).
    RingCount(u8),
    /// `+`/`-`/`+n`/`-n` — matches an atom of formal charge exactly `c`.
    Charge(i8),
    /// Logical AND of two predicates (SMARTS `&` and `;`).
    And(Box<Pred>, Box<Pred>),
    /// Logical OR of two predicates (SMARTS `,`).
    Or(Box<Pred>, Box<Pred>),
    /// Logical NOT (SMARTS `!`).
    Not(Box<Pred>),
    /// `$(...)` — a recursive SMARTS environment.
    ///
    /// Holds on a target atom iff the sub-pattern embeds (subgraph monomorphism)
    /// with its atom 0 mapped to that atom. The embedding is independent of the
    /// outer match and may reuse its atoms. Evaluated by [`rooted_match_exists`];
    /// nesting works because the sub-pattern's atoms are themselves [`Pred`]s.
    Recursive(Box<QueryGraph>),
}

impl Pred {
    /// Convenience constructor for [`Pred::And`] that boxes its operands.
    pub fn and(lhs: Pred, rhs: Pred) -> Pred {
        Pred::And(Box::new(lhs), Box::new(rhs))
    }

    /// Convenience constructor for [`Pred::Or`] that boxes its operands.
    pub fn or(lhs: Pred, rhs: Pred) -> Pred {
        Pred::Or(Box::new(lhs), Box::new(rhs))
    }

    /// Convenience constructor for [`Pred::Not`] that boxes its operand.
    ///
    /// A constructor, not `std::ops::Not`, so the lint is allowed.
    #[allow(clippy::should_implement_trait)]
    pub fn not(inner: Pred) -> Pred {
        Pred::Not(Box::new(inner))
    }

    /// Evaluates this predicate against atom `i` of `mol`.
    ///
    /// Bound on [`GraphView`](crate::graph::GraphView), not `AtomView`, because the
    /// [`Pred::Recursive`] arm walks the target graph.
    pub fn eval<V: crate::graph::GraphView>(&self, mol: &V, i: u32) -> bool {
        match self {
            Pred::Any => true,
            Pred::Element(n) => mol.av_element(i) == *n && !mol.av_aromatic(i),
            Pred::AromaticElement(n) => mol.av_element(i) == *n && mol.av_aromatic(i),
            Pred::AtomicNumber(n) => mol.av_element(i) == *n,
            Pred::Aromatic => mol.av_aromatic(i),
            Pred::Aliphatic => !mol.av_aromatic(i),
            Pred::InRing => mol.av_in_ring(i),
            Pred::RingSize(n) => mol.av_ring_size(i, *n),
            Pred::Degree(d) => mol.av_degree(i) == *d,
            // Implicit/bracket H plus bonded `[H]` nodes, so `H<n>` does not depend
            // on how H is written. Summed in u32: the u8 sum can pass 255.
            Pred::TotalH(h) => mol.av_h_count(i) as u32 + mol.av_explicit_h(i) as u32 == *h as u32,
            Pred::ImplicitH(h) => mol.av_h_count(i) == *h,
            // `av_degree` counts bonded `[H]` nodes and `av_h_count` excludes them,
            // so each connection is counted once.
            Pred::TotalConnections(x) => {
                mol.av_degree(i) as u32 + mol.av_h_count(i) as u32 == *x as u32
            }
            Pred::RingBondCount(x) => mol.av_ring_bond_count(i) == *x,
            Pred::Valence(v) => mol.av_valence(i) == *v,
            Pred::RingCount(r) => mol.av_ring_count(i) == *r,
            Pred::Charge(c) => mol.av_charge(i) == *c,
            Pred::And(l, r) => l.eval(mol, i) && r.eval(mol, i),
            Pred::Or(l, r) => l.eval(mol, i) || r.eval(mol, i),
            Pred::Not(inner) => !inner.eval(mol, i),
            Pred::Recursive(env) => rooted_match_exists(env, mol, i),
        }
    }
}

/// Sentinel for an unassigned env atom.
const NONE: u32 = u32::MAX;

/// Largest recursive environment matched with a stack buffer. The parser caps only
/// `$(...)` nesting depth, not body size, so larger envs use a heap buffer; the
/// result is never truncated.
const REC_STACK_ATOMS: usize = 32;

/// Returns `true` iff `env` embeds into `mol` as a subgraph monomorphism with its
/// atom 0 mapped to `host`: the SMARTS `$(env)` primitive (see [`Pred::Recursive`]).
///
/// Injectivity is env-local: the env may overlap atoms of a surrounding match, but
/// its own atoms map to distinct target atoms.
///
/// The brute oracle (`MolGraph`) and the production matcher (`MatchMol`) share this
/// code through [`GraphView`](crate::graph::GraphView), so they agree on every
/// recursive query.
pub fn rooted_match_exists<V: crate::graph::GraphView>(
    env: &QueryGraph,
    mol: &V,
    host: u32,
) -> bool {
    let k = env.atom_count();
    // The parser never produces an empty env.
    if k == 0 {
        return true;
    }
    // The root is `host`.
    if !env.atoms[0].eval(mol, host) {
        return false;
    }
    // Steps until the next deadline check (see `crate::deadline`).
    let mut tick = crate::deadline::CHECK_EVERY;
    if k <= REC_STACK_ATOMS {
        let mut assign = [NONE; REC_STACK_ATOMS];
        assign[0] = host;
        rec_extend(env, mol, &mut assign[..k], 1, &mut tick)
    } else {
        let mut assign = vec![NONE; k];
        assign[0] = host;
        rec_extend(env, mol, &mut assign, 1, &mut tick)
    }
}

/// Places env atoms `pos, pos+1, …` in index order; `true` on the first complete
/// embedding. `assign[0..pos]` are bound. Candidates for `pos` are the target
/// neighbours of a placed env neighbour's image, or every target atom when `pos`
/// starts a disconnected component.
fn rec_extend<V: crate::graph::GraphView>(
    env: &QueryGraph,
    mol: &V,
    assign: &mut [u32],
    pos: usize,
    tick: &mut u32,
) -> bool {
    let k = assign.len();
    if pos == k {
        return true;
    }
    // Parsed SMARTS is in DFS order, so a non-root env atom almost always has a
    // placed neighbour.
    match placed_env_neighbor(env, pos) {
        Some(anchor_q) => {
            let anchor_img = assign[anchor_q];
            for nk in 0..mol.gv_degree(anchor_img) {
                let (t, _o, _f) = mol.gv_neighbor(anchor_img, nk);
                if rec_try(env, mol, assign, pos, t, tick) {
                    return true;
                }
            }
            false
        }
        // No placed neighbour: try every target atom.
        None => {
            for t in 0..mol.gv_atom_count() {
                if rec_try(env, mol, assign, pos, t, tick) {
                    return true;
                }
            }
            false
        }
    }
}

/// The smallest-index env atom `< pos` that is bonded to env atom `pos`, or `None`
/// if `pos` has no already-placed neighbour (a fresh component root).
fn placed_env_neighbor(env: &QueryGraph, pos: usize) -> Option<usize> {
    let q = pos as u32;
    let mut best: Option<usize> = None;
    for &bi in &env.incident[pos] {
        let b = &env.bonds[bi as usize];
        let other = if b.a == q { b.b } else { b.a } as usize;
        if other < pos {
            best = Some(best.map_or(other, |c| c.min(other)));
        }
    }
    best
}

/// Tries to bind env atom `pos` to target atom `t` (injectivity, atom predicate,
/// bonds to placed atoms), then recurses; backtracks on failure.
///
/// Once the thread's deadline has passed (see [`crate::deadline`]) every call
/// returns `false`; the verify stage then reports a timeout instead of trusting it.
fn rec_try<V: crate::graph::GraphView>(
    env: &QueryGraph,
    mol: &V,
    assign: &mut [u32],
    pos: usize,
    t: u32,
    tick: &mut u32,
) -> bool {
    *tick -= 1;
    if *tick == 0 {
        *tick = crate::deadline::CHECK_EVERY;
        if crate::deadline::check_now() {
            *tick = 1; // stay on the check path, so every later call fails at once
            return false;
        }
    }
    let q = pos as u32;
    // Env-local injectivity only; the outer match's atoms may be reused.
    if assign[..pos].contains(&t) {
        return false;
    }
    if !env.atoms[pos].eval(mol, t) {
        return false;
    }
    // Each env bond is checked once, at its later-placed endpoint.
    for &bi in &env.incident[pos] {
        let b = &env.bonds[bi as usize];
        let other = if b.a == q { b.b } else { b.a } as usize;
        if other < pos {
            match mol.gv_bond(t, assign[other]) {
                Some((order, flags)) => {
                    let bond = Bond {
                        a: 0,
                        b: 0,
                        order,
                        flags,
                    };
                    if !b.pred.eval(&bond) {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }
    assign[pos] = t;
    if rec_extend(env, mol, assign, pos + 1, tick) {
        return true;
    }
    assign[pos] = NONE;
    false
}

/// A bond predicate: a boolean test over a target bond.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BondPred {
    /// `~` — matches any bond.
    Any,
    /// A specific bond order (one of [`bond_order`]).
    Order(u8),
    /// `:` — matches an aromatic bond.
    Aromatic,
    /// `@` — matches a ring bond.
    Ring,
    /// The SMARTS default bond (no symbol written): single or aromatic.
    SingleOrAromatic,
    /// Logical AND of two bond predicates.
    And(Box<BondPred>, Box<BondPred>),
    /// Logical OR of two bond predicates.
    Or(Box<BondPred>, Box<BondPred>),
    /// Logical NOT of a bond predicate.
    Not(Box<BondPred>),
}

impl BondPred {
    /// Convenience constructor for [`BondPred::And`].
    pub fn and(lhs: BondPred, rhs: BondPred) -> BondPred {
        BondPred::And(Box::new(lhs), Box::new(rhs))
    }

    /// Convenience constructor for [`BondPred::Or`].
    pub fn or(lhs: BondPred, rhs: BondPred) -> BondPred {
        BondPred::Or(Box::new(lhs), Box::new(rhs))
    }

    /// Convenience constructor for [`BondPred::Not`].
    ///
    /// A constructor, not `std::ops::Not`, so the lint is allowed.
    #[allow(clippy::should_implement_trait)]
    pub fn not(inner: BondPred) -> BondPred {
        BondPred::Not(Box::new(inner))
    }

    /// Evaluates this bond predicate against a target `bond`.
    pub fn eval(&self, bond: &Bond) -> bool {
        match self {
            BondPred::Any => true,
            BondPred::Order(o) => bond.order == *o,
            BondPred::Aromatic => bond.is_aromatic(),
            BondPred::Ring => bond.in_ring(),
            BondPred::SingleOrAromatic => bond.order == bond_order::SINGLE || bond.is_aromatic(),
            BondPred::And(l, r) => l.eval(bond) && r.eval(bond),
            BondPred::Or(l, r) => l.eval(bond) || r.eval(bond),
            BondPred::Not(inner) => !inner.eval(bond),
        }
    }
}

/// One query bond: an unordered atom-index pair plus the bond predicate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryBond {
    /// First endpoint (query-atom index).
    pub a: u32,
    /// Second endpoint (query-atom index).
    pub b: u32,
    /// The predicate a target bond must satisfy to match this query bond.
    pub pred: BondPred,
}

/// A parsed SMARTS query as a graph of predicate-bearing atoms and bonds.
///
/// Queries are small, so adjacency is a per-atom `Vec` of incident-bond indices
/// rather than a flat CSR. `Eq` is derived so a graph can nest in
/// [`Pred::Recursive`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QueryGraph {
    /// Per-query-atom predicate, indexed by query-atom id.
    pub atoms: Vec<Pred>,
    /// All query bonds.
    pub bonds: Vec<QueryBond>,
    /// `incident[i]` lists indices (into [`Self::bonds`]) of bonds touching atom `i`.
    pub incident: Vec<Vec<u32>>,
}

impl QueryGraph {
    /// Number of query atoms.
    #[inline]
    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    /// Number of query bonds.
    #[inline]
    pub fn bond_count(&self) -> usize {
        self.bonds.len()
    }
}

/// Incremental builder for a [`QueryGraph`].
#[derive(Clone, Debug, Default)]
pub struct QueryGraphBuilder {
    atoms: Vec<Pred>,
    bonds: Vec<QueryBond>,
    incident: Vec<Vec<u32>>,
}

impl QueryGraphBuilder {
    /// A fresh, empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a query atom carrying `pred` and returns its index.
    pub fn add_atom(&mut self, pred: Pred) -> u32 {
        let idx = self.atoms.len() as u32;
        self.atoms.push(pred);
        self.incident.push(Vec::new());
        idx
    }

    /// Appends a query bond and returns its index.
    ///
    /// # Panics
    /// Panics if `a == b`, an endpoint is out of range, or `a` and `b` are already
    /// bonded. The matchers assume at most one bond per atom pair; with two, VF2 and
    /// the brute-force oracle can disagree.
    pub fn add_bond(&mut self, a: u32, b: u32, pred: BondPred) -> u32 {
        let n = self.atoms.len() as u32;
        assert!(
            a < n && b < n,
            "query bond endpoint out of range: ({a}, {b}) with {n} atoms"
        );
        assert!(a != b, "self-bond on query atom {a} is not allowed");
        assert!(
            !self.has_bond(a, b),
            "query atoms {a} and {b} are already bonded"
        );
        let idx = self.bonds.len() as u32;
        self.bonds.push(QueryBond { a, b, pred });
        self.incident[a as usize].push(idx);
        self.incident[b as usize].push(idx);
        idx
    }

    /// Number of atoms added so far.
    #[inline]
    pub fn atom_count(&self) -> usize {
        self.atoms.len()
    }

    /// True if query atoms `a` and `b` are already bonded.
    ///
    /// # Panics
    /// Panics if `a` is out of range.
    #[must_use]
    pub fn has_bond(&self, a: u32, b: u32) -> bool {
        self.incident[a as usize].iter().any(|&i| {
            let bd = &self.bonds[i as usize];
            (bd.a == a && bd.b == b) || (bd.a == b && bd.b == a)
        })
    }

    /// Read-only view of the query bonds added so far.
    #[inline]
    pub fn bonds_slice(&self) -> &[QueryBond] {
        &self.bonds
    }

    /// Finalises into a [`QueryGraph`].
    pub fn finish(self) -> QueryGraph {
        QueryGraph {
            atoms: self.atoms,
            bonds: self.bonds,
            incident: self.incident,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{atom_flags, bond_flags, MolGraph, MolGraphBuilder};

    /// Atom 0: aromatic ring carbon, charge 0, 1 H. Atom 1: aliphatic oxygen,
    /// charge -1, 1 H.
    fn sample_mol() -> MolGraph {
        let mut b = MolGraphBuilder::new();
        b.add_atom(6, 0, 1, atom_flags::AROMATIC | atom_flags::IN_RING); // 0
        b.add_atom(8, -1, 1, 0); // 1
        b.add_bond(0, 1, bond_order::SINGLE, 0);
        b.finish()
    }

    #[test]
    fn element_predicate_respects_aromaticity() {
        let mol = sample_mol();
        // Aliphatic carbon `C` must NOT match the aromatic carbon.
        assert!(!Pred::Element(6).eval(&mol, 0));
        // Aromatic carbon `c` matches it.
        assert!(Pred::AromaticElement(6).eval(&mol, 0));
        // `#6` matches regardless of aromaticity.
        assert!(Pred::AtomicNumber(6).eval(&mol, 0));
        // Aliphatic oxygen: `O` matches, `o` does not.
        assert!(Pred::Element(8).eval(&mol, 1));
        assert!(!Pred::AromaticElement(8).eval(&mol, 1));
    }

    #[test]
    fn structural_predicates() {
        let mol = sample_mol();
        assert!(Pred::Any.eval(&mol, 0));
        assert!(Pred::Aromatic.eval(&mol, 0));
        assert!(!Pred::Aromatic.eval(&mol, 1));
        assert!(Pred::Aliphatic.eval(&mol, 1));
        assert!(Pred::InRing.eval(&mol, 0));
        assert!(!Pred::InRing.eval(&mol, 1));
        // Degree was cached as 1 for each atom (one bond).
        assert!(Pred::Degree(1).eval(&mol, 0));
        assert!(Pred::TotalH(1).eval(&mol, 0));
        assert!(Pred::Charge(0).eval(&mol, 0));
        assert!(Pred::Charge(-1).eval(&mol, 1));
        // RingSize is conservatively false until perception supplies it.
        assert!(!Pred::RingSize(6).eval(&mol, 0));
    }

    #[test]
    fn logical_operators_compose_with_correct_semantics() {
        let mol = sample_mol();
        // `[c;R]` — aromatic carbon AND in a ring: true at atom 0.
        let p = Pred::and(Pred::AromaticElement(6), Pred::InRing);
        assert!(p.eval(&mol, 0));
        // `[C,O]` on the oxygen: aliphatic C OR aliphatic O -> true via O.
        let p = Pred::or(Pred::Element(6), Pred::Element(8));
        assert!(p.eval(&mol, 1));
        // `[!a]` on atom 1: not aromatic -> true.
        assert!(Pred::not(Pred::Aromatic).eval(&mol, 1));
        // `[!a]` on atom 0: aromatic -> false.
        assert!(!Pred::not(Pred::Aromatic).eval(&mol, 0));
    }

    #[test]
    fn bond_predicates() {
        let mut b = MolGraphBuilder::new();
        b.add_atom(6, 0, 0, atom_flags::AROMATIC);
        b.add_atom(6, 0, 0, atom_flags::AROMATIC);
        b.add_atom(6, 0, 2, 0);
        b.add_bond(
            0,
            1,
            bond_order::AROMATIC,
            bond_flags::AROMATIC | bond_flags::IN_RING,
        );
        b.add_bond(1, 2, bond_order::DOUBLE, 0);
        let g = b.finish();
        let arom = &g.bonds[0];
        let dbl = &g.bonds[1];

        assert!(BondPred::Any.eval(arom));
        assert!(BondPred::Aromatic.eval(arom));
        assert!(!BondPred::Aromatic.eval(dbl));
        assert!(BondPred::Ring.eval(arom));
        assert!(!BondPred::Ring.eval(dbl));
        assert!(BondPred::Order(bond_order::DOUBLE).eval(dbl));
        // Default bond (single-or-aromatic): true for aromatic, false for double.
        assert!(BondPred::SingleOrAromatic.eval(arom));
        assert!(!BondPred::SingleOrAromatic.eval(dbl));
        // `[!@]` style: not a ring bond -> the double bond qualifies.
        assert!(BondPred::not(BondPred::Ring).eval(dbl));
    }

    #[test]
    fn query_builder_tracks_incidence() {
        let mut qb = QueryGraphBuilder::new();
        let a = qb.add_atom(Pred::AromaticElement(6));
        let b = qb.add_atom(Pred::AromaticElement(6));
        let c = qb.add_atom(Pred::Any);
        qb.add_bond(a, b, BondPred::Aromatic);
        qb.add_bond(b, c, BondPred::Any);
        let q = qb.finish();

        assert_eq!(q.atom_count(), 3);
        assert_eq!(q.bond_count(), 2);
        // Middle atom `b` is incident to both bonds.
        assert_eq!(q.incident[b as usize].len(), 2);
        // Terminal atoms touch one bond each.
        assert_eq!(q.incident[a as usize], vec![0]);
        assert_eq!(q.incident[c as usize], vec![1]);
    }

    #[test]
    #[should_panic(expected = "self-bond")]
    fn query_rejects_self_bonds() {
        let mut qb = QueryGraphBuilder::new();
        let a = qb.add_atom(Pred::Any);
        qb.add_bond(a, a, BondPred::Any);
    }
}
