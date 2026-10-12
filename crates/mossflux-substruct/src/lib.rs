//! Exact substructure (SMARTS subgraph-monomorphism) search.
//!
//! The stack has no external chemistry dependency:
//!
//! * [`smiles`] and [`smarts`] parse input into a [`MolGraph`] or a [`QueryGraph`].
//! * [`perception`] assigns rings, aromaticity and implicit hydrogens.
//! * [`screen`] and [`census`] reduce a query to features every match must have.
//! * [`vf2`] is the production matcher; [`brute`] is the exhaustive reference oracle.
//! * [`index::SubstructIndex`] ties these together: screen, then verify each candidate.
//!
//! # Correctness model
//!
//! * The fast matcher returns the same verdict as the brute-force oracle for every
//!   (target, query) pair.
//! * Screens are superset filters: they may admit non-matches, which the verifier
//!   rejects, but never drop a true match. A query feature the screen cannot bound
//!   (OR, wildcards, negation, recursion) contributes no requirement.
//! * Aromaticity follows the Hückel model documented in [`perception`].

#![forbid(unsafe_code)]

pub mod brute;
pub mod census;
pub mod compact;
mod deadline;
pub mod fingerprint;
pub mod graph;
pub mod index;
pub mod perception;
pub mod props;
pub mod query;
pub mod screen;
pub mod smarts;
pub mod smiles;
pub mod vf2;

pub use brute::matches as brute_matches;
pub use census::{query_min_census, target_census, Census, CensusColumns, NUM_CLASSES};
pub use graph::{atom_flags, bond_flags, bond_order, Bond, MolGraph, MolGraphBuilder, Neighbor};
pub use index::{BuildError, BuildErrorKind, Screened, SearchError, SearchLimits, SubstructIndex};
pub use query::{BondPred, Pred, QueryBond, QueryGraph, QueryGraphBuilder};
pub use screen::{
    query_required_mask, target_fingerprint, Fingerprint, ScreenIndex, FP_BITS, MAX_PATH_ATOMS,
};
pub use smarts::{parse as parse_smarts, SmartsError};
pub use smiles::{parse as parse_smiles, SmilesError};
pub use vf2::{matches as vf2_matches, Matcher, PlannedQuery};

#[cfg(test)]
mod tests {
    use super::*;

    /// A query atom predicate evaluates against a target atom through the public API.
    #[test]
    fn target_and_query_interoperate() {
        // Target: aromatic carbon (atom 0) single-bonded to aliphatic oxygen.
        let mut mb = MolGraphBuilder::new();
        mb.add_atom(6, 0, 1, atom_flags::AROMATIC | atom_flags::IN_RING);
        mb.add_atom(8, 0, 1, 0);
        mb.add_bond(0, 1, bond_order::SINGLE, 0);
        let mol = mb.finish();

        // Query: a single aromatic-carbon atom `[c]`.
        let mut qb = QueryGraphBuilder::new();
        let q_atom = qb.add_atom(Pred::AromaticElement(6));
        let query = qb.finish();

        assert_eq!(query.atom_count(), 1);
        // The query's aromatic-carbon predicate matches the target's atom 0…
        assert!(query.atoms[q_atom as usize].eval(&mol, 0));
        // …and not the aliphatic oxygen.
        assert!(!query.atoms[q_atom as usize].eval(&mol, 1));
    }
}
