//! Golden-set semantic tests.
//!
//! `differential_fuzz` proves `pipeline == brute oracle`, but both sides share
//! mossflux's parser and perception, so it cannot show that the interpretation
//! matches standard SMARTS/SMILES (e.g. `[H]` as an element, not an H count, or
//! `[Sn]` as tin).
//!
//! Each `(query, target_smiles, expected_match)` is hand-computed from the
//! SMARTS/SMILES specification. Each runs the full pipeline
//! (`SubstructIndex::search`), so a regression in parse, perceive, screen, census
//! or verify trips the assertion.

use mossflux_substruct::smarts::parse as parse_smarts;
use mossflux_substruct::SubstructIndex;

#[test]
fn deep_negation_does_not_overflow_the_stack() {
    // A long run of `!` must not overflow the stack: negation parsing is iterative.
    let atom = format!("[{}C]", "!".repeat(200_000));
    let bond = format!("C{}=C", "!".repeat(200_000));
    // Must return normally (Ok or a clean Err) — never abort.
    let _ = parse_smarts(&atom);
    let _ = parse_smarts(&bond);

    // Double negation is its own inverse for a boolean predicate, so even/odd runs
    // collapse: `[!!C]` ≡ `[C]`, `[!!!C]` ≡ `[!C]`.
    assert_eq!(
        parse_smarts("[!!C]").unwrap().atoms,
        parse_smarts("C").unwrap().atoms,
        "double negation must collapse to the bare predicate"
    );
    assert_eq!(
        parse_smarts("[!!!C]").unwrap().atoms,
        parse_smarts("[!C]").unwrap().atoms,
        "triple negation must collapse to a single Not"
    );
}

/// `true` iff `query` matches the single molecule `smiles` through the full pipeline.
fn matches(query: &str, smiles: &str) -> bool {
    let (idx, skipped) =
        SubstructIndex::from_smiles_lossy(std::slice::from_ref(&smiles.to_string()));
    assert_eq!(skipped, 0, "target SMILES must parse: {smiles:?}");
    let hits = idx
        .search(query)
        .unwrap_or_else(|e| panic!("query must parse: {query:?}: {e}"));
    hits == vec![0]
}

#[test]
fn golden_hydrogen_element_vs_h_count() {
    // Standard SMARTS: bare `[H]` is the hydrogen ELEMENT (≡ `[#1]`), matching an
    // explicit hydrogen ATOM node — NOT "any atom with one attached H". Implicit
    // hydrogens are counted (h_count), never materialised as atom nodes, so `[H]`
    // matches only molecules that carry an explicit `[H]`/`[2H]` atom.
    let cases: &[(&str, &str, bool)] = &[
        // [H] = hydrogen atom. Molecular hydrogen and explicit-H targets DO match;
        // targets whose hydrogens are all implicit do NOT.
        ("[H]", "[H][H]", true),  // dihydrogen: two explicit H atoms
        ("[H]", "[H]O[H]", true), // water written with explicit H
        ("[H]", "O", false),      // water with implicit H: no explicit H atom node
        ("[H]", "CCO", false),    // ethanol, all H implicit
        ("[H]", "[2H]C([2H])([2H])[2H]", true), // perdeuteromethane: explicit (isotopic) H
        // `[#1]` is the SAME thing and must agree with `[H]` everywhere.
        ("[#1]", "[H][H]", true),
        ("[#1]", "CCO", false),
        // A genuine total-H-count primitive still works AFTER an element symbol:
        ("[CH3]", "CCO", true),  // ethanol's methyl carbon has 3 H
        ("[OH]", "CCO", true),   // ethanol's hydroxyl O has 1 H
        ("[OH]", "CC=O", false), // acetaldehyde carbonyl O has 0 H
    ];
    for &(q, smi, want) in cases {
        assert_eq!(matches(q, smi), want, "query {q:?} on {smi:?}");
    }
}

#[test]
fn golden_total_h_is_spelling_independent() {
    // `H<n>` is TOTAL attached hydrogen, counting hydrogens written as bonded `[H]`
    // nodes the same as implicit/bracket ones — so chemically identical molecules give
    // the same answer regardless of spelling. Methane has four H however drawn;
    // `[CH4]` matches all spellings, `[CH3]` matches none.
    let methane = ["C", "[CH4]", "[H]C", "C([H])([H])([H])[H]"];
    for spelling in methane {
        assert!(
            matches("[CH4]", spelling),
            "[CH4] must match methane spelled {spelling:?}"
        );
        assert!(
            !matches("[CH3]", spelling),
            "[CH3] must NOT match methane spelled {spelling:?}"
        );
    }
    // The acid census-exact SKIP must stay consistent: `C(=O)[OH]` matches an acid whose
    // hydroxyl H is explicit (`O[H]`) as well as implicit.
    assert!(matches("C(=O)[OH]", "CC(=O)O"), "acid, implicit OH");
    assert!(matches("C(=O)[OH]", "CC(=O)O[H]"), "acid, explicit O[H]");
    // A bare `[H]` (hydrogen element) still matches only an explicit H node.
    assert!(matches("[H]", "[H]C"));
    assert!(!matches("[H]", "C"));
}

#[test]
fn golden_multiletter_elements() {
    // A multi-letter element symbol must denote that element — `[Sn]` is tin, not
    // "sulfur AND aromatic-nitrogen". Single-letter primitives that share the first
    // letter still win when no valid element follows: `[A]` aliphatic-wildcard,
    // `[R]` in-ring, `[D3]` degree-3.
    let cases: &[(&str, &str, bool)] = &[
        ("[Sn]", "C[Sn](C)(C)C", true), // tetramethyltin contains tin
        ("[Sn]", "CCS", false),         // ethanethiol: sulfur, NOT tin
        ("[As]", "[AsH3]", true),       // arsine contains arsenic
        ("[As]", "CCS", false),         // sulfur is not arsenic
        ("[Co]", "[Co]", true),         // a cobalt atom
        ("[Co]", "CCO", false),         // carbon/oxygen, no cobalt
        ("[Fe]", "[Fe]", true),
        ("[Cl]", "CCl", true), // chloroethane
        ("[Br]", "CBr", true),
        // Single-letter primitives must be unaffected by the element-precedence rule.
        ("[A]", "C", true),         // aliphatic carbon is an aliphatic atom
        ("[A]", "c1ccccc1", false), // benzene atoms are aromatic, not aliphatic
        ("[R]", "C1CCCCC1", true),  // cyclohexane atoms are in a ring
        ("[R]", "CC", false),       // ethane: acyclic
        ("[D3]", "CC(C)C", true),   // isobutane central carbon has degree 3
        ("[D3]", "CC", false),      // ethane carbons have degree 1
    ];
    for &(q, smi, want) in cases {
        assert_eq!(matches(q, smi), want, "query {q:?} on {smi:?}");
    }
}

#[test]
fn golden_connectivity_implicit_h_ring_bond() {
    let cases: &[(&str, &str, bool)] = &[
        // X = total connectivity = heavy connections + attached H.
        ("[CX4]", "CC", true),  // ethane C: 1 C + 3 H = 4
        ("[CX4]", "C", true),   // methane C: 4 H = 4
        ("[CX3]", "C=O", true), // formaldehyde C: =O(1) + 2 H = 3
        ("[CX4]", "C=O", false),
        ("[OX2]", "CCO", true),  // ethanol O: 1 C + 1 H = 2
        ("[OX1]", "CC=O", true), // carbonyl O: =C(1) + 0 H = 1
        // X must NOT double-count a bonded [H]: methane spelled [H]C is still X4.
        ("[CX4]", "[H]C", true),
        ("[CX4]", "C([H])([H])([H])[H]", true),
        // h = implicit-H. Aromatic benzene CH has h1.
        ("[ch1]", "c1ccccc1", true),
        ("[ch0]", "c1ccccc1", false),
        ("[nh1]", "c1cc[nH]c1", true), // pyrrole NH has h1
        ("[nh0]", "c1ccncc1", true),   // pyridine N has h0
        // x = ring-bond count.
        ("[cx2]", "c1ccccc1", true),      // benzene C: 2 ring bonds
        ("[x3]", "c1ccc2ccccc2c1", true), // naphthalene fusion C: 3 ring bonds
        ("[x0]", "CCO", true),            // any acyclic atom
        ("[x2]", "CCO", false),           // no ring bonds at all
        ("[R;x2]", "C1CCC1CC", true),     // a ring atom with exactly 2 ring bonds
    ];
    for &(q, smi, want) in cases {
        assert_eq!(matches(q, smi), want, "query {q:?} on {smi:?}");
    }
}

#[test]
fn golden_valence_and_ring_count() {
    let cases: &[(&str, &str, bool)] = &[
        // v = total bond-order valence. The aromatic rule is the landmine: a benzene CH
        // is v4 and a pyridine N is v3, computed from Kekulé-equivalent orders — and it
        // must hold for BOTH the aromatic and Kekulé spellings.
        ("[cv4]", "c1ccccc1", true),
        ("[cv4]", "C1=CC=CC=C1", true), // Kekulé benzene — same answer
        ("[nv3]", "c1ccncc1", true),    // pyridine N
        ("[nv3]", "C1=CC=NC=C1", true),
        ("[nv3]", "c1cc[nH]c1", true), // pyrrole N (lone-pair donor) is also v3
        ("[nv3]", "C1=CC=CN1", true),
        ("[Cv4]", "CC", true),          // sp3 carbon
        ("[Cv4]", "C=O", true),         // formaldehyde C: =O(2)+2H = 4
        ("[Nv3]", "CCN", true),         // amine N: 1 bond + 2 H
        ("[Sv6]", "CS(=O)(=O)C", true), // sulfone S: 2 single + 2 double = 6
        ("[Cv4]", "C#N", true),         // nitrile C: #N(3) + 1 H = 4
        ("[Nv3]", "C#N", true),         // nitrile N: #C(3) = 3
        // R<n> = SSSR ring-membership count. R0 = acyclic, R2 = fusion/spiro.
        ("[R1]", "c1ccccc1", true),
        ("[R2]", "c1ccccc1", false),
        ("[R2]", "c1ccc2ccccc2c1", true), // naphthalene fusion carbon, in 2 rings
        ("[R0]", "CCO", true),
        ("[R0]", "c1ccccc1", false),
        ("[R2]", "C1CCC11CCC1", true), // spiro centre, in 2 rings of the same size
        ("[R0]", "c1ccc2ccccc2c1", false), // every naphthalene atom is cyclic
        // An aromatic carbonyl carbon (pyridone/pyranone/tropone) is v4,
        // NEVER the impossible v5 (the exocyclic =O is already in the bond sum).
        ("[c;v4]=[#8]", "O=c1cccc[nH]1", true), // 2-pyridone carbonyl C
        ("[c;v5]", "O=c1cccc[nH]1", false),
        ("[c;v5]", "O=c1cccco1", false), // pyranone
        ("[c;v5]", "O=c1cccc1", false),  // tropone-ish / no carbon ever v5
        // Charged aromatic N (pyridinium / N-oxide) is aromatic and the N is v4,
        // SPELLING-INDEPENDENTLY (aromatic-input == Kekulé).
        ("[#7;v4]", "c1cc[n+](C)cc1", true),
        ("[#7;v4]", "C[N+]1=CC=CC=C1", true),
        ("[cv4]", "c1cc[n+]([O-])cc1", true), // pyridine N-oxide ring carbons
        ("[cv4]", "[O-][n+]1ccccc1", true),   // alt spelling — same answer
    ];
    for &(q, smi, want) in cases {
        assert_eq!(matches(q, smi), want, "query {q:?} on {smi:?}");
    }
}

#[test]
fn golden_stereo_directional_atommap_ignored() {
    // Substructure search is stereo-agnostic: chirality `@`/`@@`, directional
    // bonds `/`\\, and atom-map `:n` are parsed and IGNORED.
    let cases: &[(&str, &str, bool)] = &[
        // [C@H] = "a carbon with one H", matching EITHER handedness and the achiral form:
        ("[C@H]", "C[C@H](N)O", true),
        ("[C@H]", "C[C@@H](N)O", true),
        ("[C@H]", "CC(N)O", true),
        ("[C@@H]", "C[C@H](N)O", true), // @@ query also matches @ target
        ("[C@H]", "CC(=O)O", false),    // the carbonyl C has 0 H
        // Directional bonds carry E/Z geometry, ignored -> match the plain double bond:
        ("F/C=C/F", "FC=CF", true),
        ("F/C=C\\F", "FC=CF", true), // cis query, trans target — geometry ignored
        ("C/C=C/C", "CC=CC", true),
        // Atom-map labels are dropped (no constraint):
        ("[C:1]", "CCO", true),
        ("[N:2]", "CCO", false),
        ("[C:1](=O)[N:2]", "CC(=O)N", true), // mapped amide query still matches an amide
    ];
    for &(q, smi, want) in cases {
        assert_eq!(matches(q, smi), want, "query {q:?} on {smi:?}");
    }
}

#[test]
fn golden_charge_and_aromaticity() {
    // A small spread of other spec-pinned semantics, hand-derived.
    let cases: &[(&str, &str, bool)] = &[
        ("[O-]", "CC(=O)[O-]", true),   // acetate carboxylate oxygen
        ("[O-]", "CCO", false),         // neutral hydroxyl
        ("[N+]", "C[N+](C)(C)C", true), // quaternary ammonium
        ("c", "c1ccccc1", true),        // aromatic carbon in benzene
        ("c", "C1CCCCC1", false),       // cyclohexane: aliphatic
        ("[c]", "c1ccncc1", true),      // pyridine has aromatic carbons
        ("[n]", "c1ccncc1", true),      // ...and an aromatic nitrogen
        ("[n]", "c1ccccc1", false),     // benzene has none
    ];
    for &(q, smi, want) in cases {
        assert_eq!(matches(q, smi), want, "query {q:?} on {smi:?}");
    }
}

#[test]
fn golden_recursive_smarts_semantics() {
    // Recursive `$(env)` matches an atom iff `env` embeds with its root atom (index 0)
    // mapped to that atom. The brute oracle shares mossflux's rooted sub-matcher, so only
    // these hand-derived truths pin the *meaning* (not just oracle self-agreement).
    let cases: &[(&str, &str, bool)] = &[
        // `$([OH])` ≡ "a hydroxyl oxygen" (O with one H).
        ("[$([OH])]", "CCO", true),     // ethanol O–H
        ("[$([OH])]", "CC=O", false),   // acetaldehyde: carbonyl O has no H
        ("[$([OH])]", "CC(=O)O", true), // acetic acid: the acid O–H
        // `$(C=O)` ≡ "a carbonyl carbon" (root is the C of C=O).
        ("[$(C=O)]", "CC=O", true),      // acetaldehyde carbonyl C
        ("[$(C=O)]", "CCO", false),      // ethanol: no C=O
        ("[$(C=O)]", "c1ccccc1", false), // benzene: no carbonyl
        // Outer constraint AND recursive: `[O;$(OC)]` = an O bonded to a carbon.
        ("[O;$(OC)]", "CCO", true), // ethanol O is bonded to C
        ("[O;$(OC)]", "O", false),  // bare water O: no carbon neighbour
        // Negated recursive: `[C;!$(C=O)]` = an aliphatic C that is NOT a carbonyl C.
        ("[C;!$(C=O)]", "CC=O", true), // the methyl C qualifies (carbonyl C excluded)
        ("[C;!$(C=O)]", "CO", true),   // methanol C: C–O single bond, not C=O
        ("[C;!$(C=O)]", "O=C=O", false), // CO2: its only C is a carbonyl C ⇒ excluded
        // Bonded environment: `$(C[OH])` = a carbon bonded to a hydroxyl oxygen.
        ("[$(C[OH])]", "CCO", true),
        ("[$(C[OH])]", "CC=O", false),
        // Ring-rooted recursive: `$(c1ccccc1)` = a benzene-ring carbon.
        ("[$(c1ccccc1)]", "c1ccccc1", true),
        ("[$(c1ccccc1)]", "Cc1ccccc1", true), // toluene ring carbons
        ("[$(c1ccccc1)]", "C1CCCCC1", false), // cyclohexane: aliphatic
        ("[$(c1ccccc1)]", "c1ccncc1", false), // pyridine: ring is not all-carbon
        // TRUE nesting: `$(C[$([OH])])` = a carbon bonded to (an atom that is OH).
        ("[$(C[$([OH])])]", "CCO", true),
        ("[$(C[$([OH])])]", "CC=O", false),
        // Independence/overlap: a recursive env match is independent of the outer
        // match and of other envs (it may reuse the same atoms). `[$(CN)][$(NC)]`
        // matches methylamine C–N: env for the C (root C, uses N) and env for the N
        // (root N, uses C) overlap each other AND the outer binding — only legal if
        // env matches are independent. Standard recursive-SMARTS semantics ⇒ true.
        ("[$(CN)][$(NC)]", "CN", true),
        ("[$(CN)][$(NC)]", "CC", false), // ethane: no nitrogen at all
    ];
    for &(q, smi, want) in cases {
        assert_eq!(matches(q, smi), want, "query {q:?} on {smi:?}");
    }
}

#[test]
fn golden_negated_structural_does_not_break_element_screening() {
    // `[C;!R]` = aliphatic carbon NOT in a ring. The negated structural branch must
    // not strip the carbon label from the screen/census, or the query would screen
    // against nothing.
    let cases: &[(&str, &str, bool)] = &[
        ("[C;!R]", "CC", true),        // ethane: both carbons acyclic
        ("[C;!R]", "C1CC1", false),    // cyclopropane: every carbon is in a ring
        ("[C;!R]", "C1CC1C", true),    // methylcyclopropane: the methyl C is acyclic
        ("[C;!R]", "c1ccccc1", false), // benzene: aromatic, no aliphatic C at all
        ("[C;!R]", "c1ccccc1C", true), // toluene: the methyl C is aliphatic & acyclic
        // `[c;!a]` is unsatisfiable (aromatic carbon that is not aromatic) ⇒ matches
        // nothing; the screen may still emit the (6,true) bit, but VF2 rejects all.
        ("[c;!a]", "c1ccccc1", false),
        ("[c;!a]", "CC", false),
    ];
    for &(q, smi, want) in cases {
        assert_eq!(matches(q, smi), want, "query {q:?} on {smi:?}");
    }
}
