//! Hostile-input regressions: every string here is one that a stranger can send
//! to a search service. Each must give a clean error or the exact answer, never
//! a panic, an abort, or a silently different molecule. Expected answers are
//! stated from first principles, not read back from the code.

use mossflux_substruct::smarts::{MAX_COUNT, MAX_QUERY_ATOMS};
use mossflux_substruct::smiles::{MAX_ATOMS, MAX_CHARGE, MAX_RINGS};
use mossflux_substruct::{
    brute_matches, parse_smarts, parse_smiles, BondPred, Pred, QueryGraphBuilder, SmartsError,
    SmilesError, SubstructIndex,
};

// ---------------------------------------------------------------------------
// SMILES grammar: malformed input is an error, never a different molecule
// ---------------------------------------------------------------------------

#[test]
fn smiles_ring_label_zero_is_a_real_label() {
    // OpenSMILES ring bonds are the digits 0-9 and %00-%99. An unclosed label 0
    // is an error, not a chain.
    assert!(matches!(
        parse_smiles("C0CC"),
        Err(SmilesError::UnclosedRing { label: 0, .. })
    ));
    assert!(matches!(
        parse_smiles("C%00CC"),
        Err(SmilesError::UnclosedRing { label: 0, .. })
    ));
    let ring = parse_smiles("C0CC0").expect("label 0 closes a ring");
    assert_eq!((ring.atom_count(), ring.bond_count()), (3, 3));
}

#[test]
fn smiles_refuses_malformed_bonds_and_branches() {
    for (smiles, why) in [
        ("C.1CC1", "a ring digit after '.'"),
        ("C.(C)C", "a branch after '.'"),
        ("CC=", "a trailing bond"),
        ("CC#", "a trailing bond"),
        ("CC.", "a trailing '.'"),
        (".C", "a leading '.'"),
        ("C(C=)C", "a bond with no atom at the end of a branch"),
        ("C(=)C", "a bond with no atom at the end of a branch"),
    ] {
        assert!(
            matches!(parse_smiles(smiles), Err(SmilesError::DanglingBond { .. })),
            "{smiles:?} ({why}) must be DanglingBond, got {:?}",
            parse_smiles(smiles).map(|m| m.bond_count())
        );
    }
    for smiles in ["C-=C", "C=-C", "C=.C", "C..C"] {
        assert!(
            matches!(
                parse_smiles(smiles),
                Err(SmilesError::UnexpectedChar { .. })
            ),
            "two bond symbols in a row ({smiles:?}) must be refused"
        );
    }
    assert!(matches!(
        parse_smiles("C()C"),
        Err(SmilesError::EmptyBranch { pos: 1 })
    ));
}

#[test]
fn smiles_bracket_fields_are_ordered_and_unique() {
    for smiles in [
        "[CH2H3]", "[C+-]", "[C+2-3]", "[C:]", "[C::1]", "[C@@@]", "[C+H]", "[CH3@]",
    ] {
        assert!(
            matches!(
                parse_smiles(smiles),
                Err(SmilesError::UnexpectedChar { .. })
            ),
            "{smiles:?} repeats or misorders a bracket field"
        );
    }
    // Every standard form still parses.
    for smiles in [
        "[13CH3:1]",
        "[C@@H](F)(Cl)Br",
        "[NH4+]",
        "[O-]",
        "[Fe+2]",
        "[Fe++]",
        "[2H]C",
        "F/C=C/F",
        "[nH]1cccc1",
        "[C@H]1CCCN1",
    ] {
        assert!(parse_smiles(smiles).is_ok(), "{smiles:?} must still parse");
    }
}

#[test]
fn smiles_out_of_range_values_are_refused_not_clamped() {
    let too_many_signs = format!("[C{}]", "+".repeat(200));
    assert!(matches!(
        parse_smiles(&too_many_signs),
        Err(SmilesError::ValueOutOfRange { what: "charge", .. })
    ));
    let over = format!("[C+{}]", MAX_CHARGE + 1);
    assert!(matches!(
        parse_smiles(&over),
        Err(SmilesError::ValueOutOfRange { what: "charge", .. })
    ));
    let at_limit = parse_smiles(&format!("[C-{MAX_CHARGE}]")).unwrap();
    assert_eq!(at_limit.charge[0], -(MAX_CHARGE as i8));
    assert!(matches!(
        parse_smiles("[9999C]"),
        Err(SmilesError::ValueOutOfRange {
            what: "isotope",
            ..
        })
    ));
    assert!(parse_smiles("[999C]").is_ok());
}

#[test]
fn smiles_size_limits_are_errors() {
    let atoms = "C".repeat(MAX_ATOMS + 1);
    assert!(matches!(
        parse_smiles(&atoms),
        Err(SmilesError::TooLarge { what: "atoms", .. })
    ));
    // Connected cyclopropanes: one ring closure each.
    let rings = |k: usize| "C1CC1".repeat(k);
    assert!(parse_smiles(&rings(MAX_RINGS)).is_ok());
    assert!(matches!(
        parse_smiles(&rings(MAX_RINGS + 1)),
        Err(SmilesError::TooLarge {
            what: "ring closures",
            ..
        })
    ));
}

// ---------------------------------------------------------------------------
// SMARTS grammar and ranges
// ---------------------------------------------------------------------------

#[test]
fn smarts_refuses_malformed_topology() {
    for smarts in ["C0CC", "C%00CC"] {
        assert!(matches!(
            parse_smarts(smarts),
            Err(SmartsError::UnclosedRing { label: 0, .. })
        ));
    }
    for smarts in ["C.1CC1", "C.(C)C", "CC=", "CC~", "CC.", ".C", "C(C=)C"] {
        assert!(
            matches!(parse_smarts(smarts), Err(SmartsError::DanglingBond { .. })),
            "{smarts:?} must be DanglingBond"
        );
    }
    for smarts in ["C=.C", "C.=C", "C..C"] {
        assert!(
            matches!(
                parse_smarts(smarts),
                Err(SmartsError::UnexpectedChar { .. })
            ),
            "{smarts:?} must be refused"
        );
    }
    assert!(matches!(
        parse_smarts("C()C"),
        Err(SmartsError::EmptyBranch { pos: 1 })
    ));
    // The two halves of a ring closure must agree, as in SMILES.
    assert!(matches!(
        parse_smarts("C=1CCC#1"),
        Err(SmartsError::RingBondMismatch { label: 1, .. })
    ));
    assert!(matches!(
        parse_smarts("C-1CC=1"),
        Err(SmartsError::RingBondMismatch { label: 1, .. })
    ));
    assert!(parse_smarts("C=1CCC=1").is_ok());
    assert!(parse_smarts("C0CC0").is_ok());
}

#[test]
fn smarts_count_and_charge_ranges() {
    for prim in ["#", "D", "H", "h", "X", "x", "v", "R", "r"] {
        let over = format!("[C;{prim}{}]", MAX_COUNT + 1);
        assert!(
            matches!(
                parse_smarts(&over),
                Err(SmartsError::ValueOutOfRange { max, .. }) if max == MAX_COUNT
            ),
            "{over:?} must be out of range"
        );
        assert!(
            parse_smarts(&format!("[C;{prim}{MAX_COUNT}]")).is_ok(),
            "{prim}{MAX_COUNT} is in range"
        );
    }
    // Values that overflow the integer parse must be errors, not panics.
    for smarts in [
        "[C+9999999999]",
        "[C-2147483648]",
        "[+99999999999999999999]",
        "[C+16]",
    ] {
        assert!(
            matches!(
                parse_smarts(smarts),
                Err(SmartsError::ValueOutOfRange { .. })
            ),
            "{smarts:?} must be out of range"
        );
    }
    let many = format!("[C{}]", "-".repeat(500));
    assert!(matches!(
        parse_smarts(&many),
        Err(SmartsError::ValueOutOfRange { .. })
    ));
    assert!(parse_smarts("[C+15]").is_ok());
}

#[test]
fn smarts_atom_budget_counts_recursive_bodies() {
    let chain = |n: usize| "C".repeat(n);
    assert!(parse_smarts(&chain(MAX_QUERY_ATOMS)).is_ok());
    assert!(matches!(
        parse_smarts(&chain(MAX_QUERY_ATOMS + 1)),
        Err(SmartsError::TooLarge { .. })
    ));
    // The host atom plus its environment: MAX - 1 env atoms fit, MAX do not.
    let env = |n: usize| format!("[$({})]", chain(n));
    assert!(parse_smarts(&env(MAX_QUERY_ATOMS - 1)).is_ok());
    assert!(matches!(
        parse_smarts(&env(MAX_QUERY_ATOMS)),
        Err(SmartsError::TooLarge { .. })
    ));
}

#[test]
#[should_panic(expected = "already bonded")]
fn query_builder_refuses_a_parallel_bond() {
    // Two bonds on one pair would make VF2 and the oracle disagree.
    let mut qb = QueryGraphBuilder::new();
    let a = qb.add_atom(Pred::Element(6));
    let b = qb.add_atom(Pred::Element(6));
    qb.add_bond(a, b, BondPred::Any);
    qb.add_bond(b, a, BondPred::Any);
}

// ---------------------------------------------------------------------------
// Counts past 255: no overflow panic, and the exact answer
// ---------------------------------------------------------------------------

/// `[OH99]` with 157 bonded `[H]`: total H is 99 + 157 = 256, past a u8. The
/// census (build time) and both matchers (search time) must not overflow.
#[test]
fn hydrogen_and_connectivity_sums_past_255_are_exact() {
    let total_h = format!("C[OH99]{}", "([H])".repeat(157));
    let total_x = format!("[CH99]{}", "(C)".repeat(157));
    let corpus = [total_h.as_str(), total_x.as_str(), "CO", "CC"];
    let idx = SubstructIndex::from_smiles(&corpus).expect("builds without overflow");
    let mols: Vec<_> = corpus.iter().map(|s| parse_smiles(s).unwrap()).collect();
    // By hand. Row 0: a CH3 (X4) on an O with 256 H (X 257). Row 1: a C with 99
    // H and 157 CH3 neighbours (X 256). Row 2: CH3-OH. Row 3: two CH3. A u8 sum
    // wraps 256 to 0, so `[OH0]` and `[CX0]` are the cases a wrap would match.
    for (q, want) in [
        ("[OH0]", vec![]),
        ("[OH2]", vec![]),
        ("[OH1]", vec![2]),
        ("[!H0;O]", vec![0, 2]),
        ("[$([OH1])]", vec![2]),
        ("[CX0]", vec![]),
        ("[CX4]", vec![0, 1, 2, 3]),
        ("[X3]", vec![]),
        ("[CH3]", vec![0, 1, 2, 3]),
    ] {
        let parsed = parse_smarts(q).unwrap();
        let brute: Vec<u32> = (0..mols.len() as u32)
            .filter(|&i| brute_matches(&parsed, &mols[i as usize]))
            .collect();
        assert_eq!(brute, want, "oracle on {q:?}");
        assert_eq!(idx.search(q).unwrap(), want, "search on {q:?}");
    }
}

/// A ring of 300 atoms is not a ring of 44 (300 mod 256), and a ring of 256 is
/// not "no ring" (256 mod 256 = 0). Ring sizes saturate at 255.
#[test]
fn ring_sizes_past_255_do_not_wrap() {
    let ring = |n: usize| format!("C1{}C1", "C".repeat(n - 2));
    let corpus = [ring(300), ring(256), ring(6)];
    let idx = SubstructIndex::from_smiles(&corpus).unwrap();
    for (q, want) in [
        ("[r44]", vec![]),
        ("[r0]", vec![]),
        ("[r6]", vec![2]),
        ("[r254]", vec![]),
        ("[R]", vec![0, 1, 2]),
        ("[R1]", vec![0, 1, 2]),
    ] {
        assert_eq!(idx.search(q).unwrap(), want, "{q:?}");
    }
}

// ---------------------------------------------------------------------------
// Stack depth at the query-size limit
// ---------------------------------------------------------------------------

/// Both matchers recurse once per query atom. At the atom limit they must still
/// fit a 2 MiB stack (the rayon worker and test-thread default): 300 rows put the
/// verify stage on the parallel path, so this runs on rayon workers.
#[test]
fn largest_query_fits_a_worker_stack() {
    let chain = "C".repeat(MAX_QUERY_ATOMS);
    let corpus = vec![chain.clone(); 300];
    let idx = SubstructIndex::from_smiles(&corpus).unwrap();
    assert_eq!(idx.search(&chain).unwrap().len(), 300);
    let env = format!("[$({})]", "C".repeat(MAX_QUERY_ATOMS - 1));
    assert_eq!(idx.search(&env).unwrap().len(), 300);
    let mol = parse_smiles(&chain).unwrap();
    assert!(brute_matches(&parse_smarts(&chain).unwrap(), &mol));
}

// ---------------------------------------------------------------------------
// Exponential queries: exact early rejects, and a deadline for the rest
// ---------------------------------------------------------------------------

/// A disconnected query fails at once when one component cannot embed alone.
/// Without that pre-check, `C.C.C.C.C.C.C.[C+]` against a 30-carbon chain would
/// try every way of placing seven carbons before finding no `[C+]`.
#[test]
fn disconnected_query_with_an_impossible_component_is_fast() {
    let chain = "C".repeat(30);
    let idx = SubstructIndex::from_smiles(&[chain.as_str()]).unwrap();
    for q in [
        "C.C.C.C.C.C.C.[C+]",
        "*.*.*.*.*.*.*.[!#6]",
        "[C+].C.C.C.C.C.C.C",
    ] {
        let t = std::time::Instant::now();
        assert!(idx.search(q).unwrap().is_empty(), "{q:?}");
        assert!(
            t.elapsed() < std::time::Duration::from_secs(5),
            "{q:?} took {:?}",
            t.elapsed()
        );
    }
    // The pre-check is only a necessary condition: a query whose components
    // all embed alone still needs the joint search, and still matches.
    assert_eq!(idx.search("C.C.C.C").unwrap(), vec![0]);
}

/// Components that each embed alone but not together are still exponential
/// (subgraph matching is NP-complete). The deadline turns that into an error,
/// never a partial or empty "answer".
#[test]
fn deadline_stops_an_exponential_search_with_an_error() {
    use mossflux_substruct::{SearchError, SearchLimits};
    use std::time::{Duration, Instant};
    // 29 carbons and one oxygen; the query needs two non-carbons, so each
    // `[!#6]` embeds alone but the pair never does, after placing the `*`s.
    let target = format!("{}O", "C".repeat(29));
    let corpus = vec![target; 300]; // >= 256 rows: the parallel verify path
    let idx = SubstructIndex::from_smiles(&corpus).unwrap();
    let q = "*.*.*.*.*.*.*.*.[!#6].[!#6]";
    let t = Instant::now();
    let r = idx.search_with_limits(q, SearchLimits::timeout(Duration::from_millis(200)));
    assert_eq!(r, Err(SearchError::Timeout));
    assert!(
        t.elapsed() < Duration::from_secs(5),
        "stopped {:?} after start",
        t.elapsed()
    );
    // The serial path (fewer than 256 candidates) stops too.
    let small = SubstructIndex::from_smiles(&corpus[..3]).unwrap();
    let r = small.search_with_limits(q, SearchLimits::timeout(Duration::from_millis(200)));
    assert_eq!(r, Err(SearchError::Timeout));
    // A recursive environment with the same shape stops as well.
    let rq = "[$(*.*.*.*.*.*.*.*.[!#6].[!#6])]";
    let r = small.search_with_limits(rq, SearchLimits::timeout(Duration::from_millis(200)));
    assert_eq!(r, Err(SearchError::Timeout));
    // A normal query under a generous deadline gives the exact answer.
    assert_eq!(
        idx.search_with_limits("CO", SearchLimits::timeout(Duration::from_secs(60)))
            .unwrap()
            .len(),
        300
    );
}

/// `screen_query` + `verify_screened` is exactly `search` (the split a benchmark
/// times), and a `Screened` from another index is refused.
#[test]
fn screen_query_then_verify_screened_equals_search() {
    let corpus = ["CCO", "c1ccccc1O", "CC(=O)N", "c1ccpcc1", "CP(C)C", "ClCCl"];
    let idx = SubstructIndex::from_smiles(&corpus).unwrap();
    for q in [
        "O", "P", "Cl", "c1ccccc1", "C(=O)N", "[$(C=O)]", "C.Cl", "[!#6]",
    ] {
        let s = idx.screen_query(q).unwrap();
        assert_eq!(idx.verify_screened(&s), idx.search(q).unwrap(), "{q:?}");
    }
    let other = SubstructIndex::from_smiles(&["C"]).unwrap();
    let s = other.screen_query("C").unwrap();
    let r = std::panic::catch_unwind(|| idx.verify_screened(&s));
    assert!(r.is_err(), "a Screened from another index must be refused");
}
