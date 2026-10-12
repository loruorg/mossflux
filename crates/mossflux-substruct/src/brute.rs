//! The brute-force subgraph-monomorphism matcher: the exactness oracle.
//!
//! Decides whether a [`QueryGraph`] embeds into a [`MolGraph`] as a non-induced
//! subgraph. It is written to be obviously correct, not fast: every optimised
//! matcher and every screen is tested against it. Do not add pruning or clever
//! ordering here.
//!
//! # Semantics: subgraph monomorphism
//!
//! A match is an injective map `f: atoms(Q) -> atoms(T)` such that
//!
//! 1. for every query atom `q`, its [`Pred`](crate::query::Pred) holds on `f(q)`;
//! 2. for every query bond `(q, r)`, a target bond joins `f(q)` and `f(r)` and
//!    satisfies the query bond's [`BondPred`];
//! 3. distinct query atoms map to distinct target atoms.
//!
//! Query non-edges are not constrained: the target may add bonds the query does
//! not mention (so `c1ccccc1` matches inside naphthalene despite the fusion bond).
//! Only bonds the query demands are checked.
//!
//! # Algorithm
//!
//! Recursive backtracking over query atoms in index order, trying every target
//! atom for each. A candidate is skipped if it is already used, fails the atom
//! predicate, or lacks a satisfying bond to an already-assigned neighbour. Each
//! query bond is thus checked exactly once, when its second endpoint is placed.
//! Returns on the first complete embedding. The empty query matches every target.
//!
//! Worst case is `O(|T|^|Q|)`; this is for tests and verification, not hot paths.

use crate::graph::{Bond, MolGraph};
use crate::query::{BondPred, QueryGraph};

/// Sentinel for an unassigned query atom. No real molecule has `u32::MAX` atoms.
const NONE: u32 = u32::MAX;

/// Returns `true` iff `query` embeds into `mol` as a subgraph monomorphism
/// (see the [module docs](self)).
///
/// Returns on the first complete embedding. The empty query matches any target.
/// No screening is done; use this for verification and tests.
pub fn matches(query: &QueryGraph, mol: &MolGraph) -> bool {
    let nq = query.atom_count();
    if nq == 0 {
        return true;
    }
    // More query atoms than target atoms cannot inject.
    if nq > mol.atom_count() {
        return false;
    }

    // assign[q] = target atom bound to query atom q, or NONE.
    let mut assign = vec![NONE; nq];
    // used[t] = true iff target atom t is already an image.
    let mut used = vec![false; mol.atom_count()];

    extend(query, mol, 0, &mut assign, &mut used)
}

/// Assigns query atoms `q, q+1, …` to target atoms; `true` once all are bound.
///
/// `assign` and `used` are restored on backtrack, so they are unchanged after a
/// `false` return.
fn extend(
    query: &QueryGraph,
    mol: &MolGraph,
    q: usize,
    assign: &mut [u32],
    used: &mut [bool],
) -> bool {
    // Every bond was checked when its second endpoint was placed.
    if q == query.atom_count() {
        return true;
    }

    for t in 0..mol.atom_count() as u32 {
        // Injectivity.
        if used[t as usize] {
            continue;
        }
        // Atom predicate.
        if !query.atoms[q].eval(mol, t) {
            continue;
        }
        // Bonds to already-placed neighbours.
        if !bonds_consistent(query, mol, q, t, assign) {
            continue;
        }

        assign[q] = t;
        used[t as usize] = true;
        if extend(query, mol, q + 1, assign, used) {
            return true;
        }
        assign[q] = NONE;
        used[t as usize] = false;
    }

    false
}

/// Checks every query bond from `q` to an already-assigned atom: a target bond
/// must join `t` and that atom's image and satisfy the query bond's predicate.
fn bonds_consistent(query: &QueryGraph, mol: &MolGraph, q: usize, t: u32, assign: &[u32]) -> bool {
    for &bond_idx in &query.incident[q] {
        let qbond = &query.bonds[bond_idx as usize];
        let other_q = if qbond.a == q as u32 {
            qbond.b
        } else {
            qbond.a
        };
        let mapped = assign[other_q as usize];
        if mapped == NONE {
            // Checked when the far endpoint is placed.
            continue;
        }
        // `mapped != t` by injectivity, so there is no self-bond case.
        if !target_bond_ok(mol, t, mapped, &qbond.pred) {
            return false;
        }
    }
    true
}

/// `true` iff a target bond joins `a` and `b` and satisfies `pred`.
fn target_bond_ok(mol: &MolGraph, a: u32, b: u32, pred: &BondPred) -> bool {
    for nbr in mol.neighbors(a) {
        if nbr.atom == b {
            let bond: &Bond = &mol.bonds[nbr.bond as usize];
            return pred.eval(bond);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{BondPred, Pred, QueryGraphBuilder};
    use crate::smarts::parse as parse_smarts;
    use crate::smiles::parse as parse_smiles;

    /// Parses a SMILES target and a SMARTS query, then runs the oracle.
    fn hits(smarts: &str, smiles: &str) -> bool {
        let mol = parse_smiles(smiles).expect("target SMILES should parse");
        let q = parse_smarts(smarts).expect("query SMARTS should parse");
        matches(&q, &mol)
    }

    // --- Positive matches ---

    /// The methyl is extra structure the query does not constrain.
    #[test]
    fn benzene_matches_toluene() {
        assert!(hits("c1ccccc1", "Cc1ccccc1"));
    }

    /// The fusion bond between two matched carbons is an extra edge a
    /// non-induced query does not forbid.
    #[test]
    fn benzene_matches_naphthalene() {
        assert!(hits("c1ccccc1", "c1ccc2ccccc2c1"));
    }

    #[test]
    fn hydroxyl_matches_ethanol() {
        assert!(hits("[OH]", "CCO"));
    }

    #[test]
    fn aromatic_n_matches_pyridine() {
        assert!(hits("[n]", "c1ccncc1"));
    }

    #[test]
    fn aromatic_atom_matches_benzene() {
        assert!(hits("a", "c1ccccc1"));
    }

    #[test]
    fn c_o_path_matches_ethanol() {
        assert!(hits("CO", "CCO"));
    }

    // --- Non-matches ---

    /// Cyclohexane carbons are aliphatic, so `c` fails on every atom.
    #[test]
    fn benzene_does_not_match_cyclohexane() {
        assert!(!hits("c1ccccc1", "C1CCCCC1"));
    }

    #[test]
    fn aromatic_atom_does_not_match_aliphatic() {
        assert!(!hits("a", "CCC"));
    }

    #[test]
    fn benzene_does_not_match_ethanol() {
        assert!(!hits("c1ccccc1", "CCO"));
    }

    #[test]
    fn oxygen_does_not_match_methane() {
        assert!(!hits("O", "C"));
    }

    // --- Semantics and edge cases ---

    /// Built directly because the SMARTS parser rejects an empty string.
    #[test]
    fn empty_query_matches_anything() {
        let q = QueryGraphBuilder::new().finish();
        let mol = parse_smiles("CCO").unwrap();
        assert!(matches(&q, &mol));
    }

    #[test]
    fn query_bigger_than_target_fails() {
        let mol = parse_smiles("CC").unwrap();
        let q = parse_smarts("CCC").unwrap();
        assert!(!matches(&q, &mol));
    }

    /// Two query atoms must not collapse onto one target atom.
    #[test]
    fn injectivity_forbids_atom_reuse() {
        let mol = parse_smiles("C").unwrap();
        let q = parse_smarts("CC").unwrap();
        assert!(!matches(&q, &mol));
    }

    #[test]
    fn bond_predicate_is_enforced() {
        assert!(!hits("C=C", "CC"));
        assert!(hits("C=C", "C=C"));
    }

    /// A disconnected query (`[#6].[#8]`) matches a target where those atoms are
    /// bonded: the query does not constrain the missing edge.
    #[test]
    fn disconnected_query_ignores_extra_target_bonds() {
        let mut qb = QueryGraphBuilder::new();
        qb.add_atom(Pred::AtomicNumber(6));
        qb.add_atom(Pred::AtomicNumber(8));
        let q = qb.finish();
        let mol = parse_smiles("CO").unwrap();
        assert!(matches(&q, &mol));
        // Both elements must still be present.
        let methane = parse_smiles("C").unwrap();
        assert!(!matches(&q, &methane));
    }

    #[test]
    fn any_bond_matches_either_order() {
        assert!(hits("C~C", "CC"));
        assert!(hits("C~C", "C=C"));
    }

    /// An aromatic-bond query edge does not match a single target bond.
    #[test]
    fn aromatic_bond_query_rejects_single_bond() {
        let mut qb = QueryGraphBuilder::new();
        let a = qb.add_atom(Pred::Any);
        let b = qb.add_atom(Pred::Any);
        qb.add_bond(a, b, BondPred::Aromatic);
        let q = qb.finish();
        let mol = parse_smiles("CC").unwrap();
        assert!(!matches(&q, &mol));
    }
}
