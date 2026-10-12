//! Spelling-independence gate.
//!
//! The exactness gates feed the matcher and the brute oracle the same perceived
//! graph, so they prove `matcher == oracle` but nothing about perception. Here,
//! for each group of SMILES that denote the same molecule (Kekulé vs aromatic,
//! etc.), every query must return an identical result. The query string is
//! shared across spellings, so a disagreement can only come from perception.

use mossflux_substruct::index::SubstructIndex;

/// Each inner slice is a set of SMILES denoting the SAME molecule.
const EQUIVALENT: &[&[&str]] = &[
    &["c1ccccc1", "C1=CC=CC=C1"],               // benzene
    &["Cc1ccccc1", "CC1=CC=CC=C1"],             // toluene
    &["Oc1ccccc1", "OC1=CC=CC=C1"],             // phenol
    &["Nc1ccccc1", "NC1=CC=CC=C1"],             // aniline
    &["c1ccncc1", "C1=CC=NC=C1"],               // pyridine
    &["c1cc[nH]c1", "C1=CC=CN1"],               // pyrrole
    &["c1ccoc1", "C1=CC=CO1"],                  // furan
    &["c1ccsc1", "C1=CC=CS1"],                  // thiophene
    &["O=c1cccc[nH]1", "O=C1C=CC=CN1"],         // 2-pyridone
    &["O=Cc1ccccc1", "O=CC1=CC=CC=C1"],         // benzaldehyde
    &["c1ccc2ccccc2c1", "C1=CC=C2C=CC=CC2=C1"], // naphthalene
];
// NOTE: explicit bonded `[H]` spellings (e.g. `[H]C` vs `C`) are NOT graph-equivalent —
// an explicit `[H]` is a real atom+bond node, so they legitimately differ for atom/bond
// queries like `*~*` or `[D1]`. They ARE equivalent for the TOTAL-H primitive (`[CHn]`),
// which is asserted directly in `tests/golden_smarts.rs::golden_total_h_is_spelling_independent`.

const QUERIES: &[&str] = &[
    "*",
    "a",
    "A",
    "c",
    "C",
    "n",
    "N",
    "o",
    "O",
    "s",
    "[#6]",
    "[#7]",
    "[#8]",
    "[nH]",
    "[cH]",
    "[OH]",
    "[NH2]",
    "cc",
    "CC",
    "c:c",
    "C=C",
    "C-C",
    "c1ccccc1",
    "[R]",
    "[r6]",
    "[r5]",
    "[!R]",
    "*~*",
    "[c;H1]",
    "[N,O]",
    "[c,n]",
    // H-count primitives — must be spelling-independent across implicit/bracket/bonded H:
    "[CH4]",
    "[CH3]",
    "[CH1]",
    "[CH0]",
    "[OH1]",
    "[OH0]",
    "C(=O)[OH]",
    "[H]",
    "[#1]",
];

#[test]
fn equivalent_smiles_answer_every_query_identically() {
    let mut compared = 0usize;
    let mut saw_hit = 0usize;
    let mut saw_miss = 0usize;
    for group in EQUIVALENT {
        assert!(
            group.len() >= 2,
            "a group needs at least two spellings to compare"
        );
        // One single-molecule index per spelling (search => [] or [0]).
        let idx: Vec<SubstructIndex> = group
            .iter()
            .map(|s| {
                SubstructIndex::from_smiles(&[*s]).unwrap_or_else(|e| panic!("parse `{s}`: {e:?}"))
            })
            .collect();
        for &q in QUERIES {
            // Every query must search successfully on every spelling; a failure
            // is a defect, never a skipped comparison.
            let results: Vec<bool> = idx
                .iter()
                .zip(group.iter())
                .map(|(ix, s)| {
                    let hits = ix
                        .search(q)
                        .unwrap_or_else(|e| panic!("query `{q}` failed to search on `{s}`: {e:?}"));
                    assert!(
                        hits.len() <= 1,
                        "a one-molecule index cannot return {} rows",
                        hits.len()
                    );
                    !hits.is_empty()
                })
                .collect();
            for (k, r) in results.iter().enumerate() {
                assert_eq!(
                    *r, results[0],
                    "SPELLING-DEPENDENT ANSWER: query `{q}` differs between `{}` ({}) and `{}` ({})",
                    group[0], results[0], group[k], r
                );
            }
            if results[0] {
                saw_hit += 1;
            } else {
                saw_miss += 1;
            }
            compared += 1;
        }
    }
    // ANTI-VACUITY: every (group, query) pair really ran, and the battery really
    // separates matches from non-matches — so the equality above is comparing
    // meaningful answers, not `false == false` everywhere.
    assert_eq!(
        compared,
        EQUIVALENT.len() * QUERIES.len(),
        "not every (molecule group, query) pair was compared"
    );
    assert!(saw_hit > 50, "battery produced only {saw_hit} matches");
    assert!(
        saw_miss > 50,
        "battery produced only {saw_miss} non-matches"
    );
}
