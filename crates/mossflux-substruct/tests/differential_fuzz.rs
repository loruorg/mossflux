//! Differential fuzz: the full screen→census→verify pipeline vs the brute-force
//! oracle, over a broad query battery on a diverse corpus.
//!
//! Covers halogen ORs, degree/ring/charge primitives, negation, disconnected
//! fragments, recursive SMARTS, and self-queries (every corpus molecule parsed as
//! its own SMARTS must match at least itself, so a dropped real match surfaces as
//! a self-miss).
//!
//! The assertion everywhere is exact set equality:
//!
//! ```text
//! idx.search(q)  ==  { i : brute::matches(parse_smarts(q), mols[i]) }
//! ```
//!
//! A missing brute match is a false negative; an extra id is a false positive.
//! Either fails the test.
//!
//! Deterministic: generated queries use a fixed LCG. Set
//! `MOSSFLUX_FUZZ_CHEMBL=<path-to-CID-SMILES-or-TSV>` (decompressed) to also sample
//! real molecules from a data file; unset, the test is self-contained.

use mossflux_substruct::index::SubstructIndex;
use mossflux_substruct::{brute, parse_smarts, parse_smiles};
use std::collections::BTreeSet;

/// The corpus the tests run on: the embedded [`CORPUS`] always, plus — when
/// `MOSSFLUX_FUZZ_CHEMBL=<file>` is set — a deterministic sample of real molecules
/// from that one-per-line / `id<TAB>smiles` file (decompressed; `.gz` panics). The
/// sample is deterministic. Unparseable rows are skipped. Cap via
/// `MOSSFLUX_FUZZ_CHEMBL_N`.
fn full_corpus() -> Vec<String> {
    let mut corpus: Vec<String> = CORPUS.iter().map(|s| s.to_string()).collect();
    let Ok(path) = std::env::var("MOSSFLUX_FUZZ_CHEMBL") else {
        return corpus;
    };
    let cap: usize = std::env::var("MOSSFLUX_FUZZ_CHEMBL_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let text: String = if path.ends_with(".gz") {
        // No gzip decoder in this test; require a decompressed file.
        panic!("MOSSFLUX_FUZZ_CHEMBL: pass a DECOMPRESSED file (got .gz {path:?})");
    } else {
        String::from_utf8_lossy(&bytes).into_owned()
    };
    let mut added = 0usize;
    for (lineno, line) in text.lines().enumerate() {
        if added >= cap {
            break;
        }
        let line = line.trim();
        if line.is_empty() || lineno == 0 && line.to_ascii_lowercase().contains("smiles") {
            continue;
        }
        // SMILES is the 2nd whitespace/tab field if present, else the whole line.
        let smiles = line
            .split([' ', '\t'])
            .nth(1)
            .filter(|s| !s.is_empty())
            .unwrap_or(line);
        if parse_smiles(smiles).is_ok() {
            corpus.push(smiles.to_string());
            added += 1;
        }
    }
    eprintln!(
        "full_corpus: {} embedded + {} sampled from {path:?}",
        CORPUS.len(),
        added
    );
    corpus
}

/// A diverse, hand-picked corpus: chains, branch points, every halogen, the common
/// heteroaromatics and fused systems, carbonyl/sulfonyl/nitrile/nitro groups,
/// charged species, and Kekulé/aromatic spelling pairs of several rings. Chosen so
/// the query battery below has real hits *and* real misses to separate.
const CORPUS: &[&str] = &[
    // --- simple chains / branch points (degree primitives) ---
    "C",
    "CC",
    "CCC",
    "CCCC",
    "CCCCCC",
    "CC(C)C",
    "CC(C)(C)C",
    "CC(C)(C)CC",
    "CCO",
    "CCN",
    "CCS",
    "OCCO",
    "NCCN",
    "CC(O)CC(O)C",
    "C(C)(C)(C)C",
    // --- halogens (single + multi + aromatic-bound) ---
    "CF",
    "CCl",
    "CBr",
    "CI",
    "FC(F)(F)F",
    "ClC(Cl)Cl",
    "ClCCBr",
    "FCCF",
    "Fc1ccccc1",
    "Clc1ccccc1",
    "Brc1ccccc1",
    "Fc1ccc(Cl)cc1",
    "FC(F)(F)c1ccccc1",
    // --- benzene / substituted, Kekulé vs aromatic spellings ---
    "c1ccccc1",
    "C1=CC=CC=C1",
    "Cc1ccccc1",
    "CC1=CC=CC=C1",
    "Oc1ccccc1",
    "OC1=CC=CC=C1",
    "Nc1ccccc1",
    "c1ccc(cc1)O",
    "Cc1ccc(C)cc1",
    // --- fused / polycyclic ---
    "c1ccc2ccccc2c1",
    "C1=CC=C2C=CC=CC2=C1",
    "c1ccc2c(c1)cccc2",
    "c1ccc2[nH]ccc2c1",
    "c1ccc2ncccc2c1",
    "c1ccc2c(c1)nccn2",
    // --- heteroaromatics ---
    "c1ccncc1",
    "C1=CC=NC=C1",
    "c1ccnc(c1)C",
    "c1cncnc1",
    "c1ccoc1",
    "c1ccsc1",
    "c1cc[nH]c1",
    "c1c[nH]cn1",
    "c1cnc[nH]1",
    "c1ccc(nc1)N",
    "c1cnccn1",
    // --- carbonyls / acids / amides / esters / nitriles ---
    "CC=O",
    "CC(=O)C",
    "CC(=O)O",
    "CC(=O)N",
    "CC(=O)OC",
    "CC(=O)Cl",
    "O=C=O",
    "CC#N",
    "N#CC#N",
    "O=C(O)c1ccccc1",
    "CC(=O)Nc1ccccc1",
    "COC(=O)c1ccccc1",
    // --- acyl decoys the census-exact skip (amide/acid/ester) must not confuse:
    //     they exercise the !aromatic + element guards, which have no VF2 fallback. ---
    "NC(=O)N",
    "O=C(N)N", // urea (two N on the acyl C)
    "NC(=O)OC",
    "COC(=O)N",      // carbamate (N + O-alkyl)
    "CC(=O)OC(=O)C", // acetic anhydride (two acyl O)
    "CC(=O)SC",      // thioester (=O but S, not O, alkyl)
    "COC(=O)OC",     // dimethyl carbonate (two O-alkyl)
    "NC(=S)N",
    "CC(=S)N",
    "CC(=S)O", // thio-decoys (=S, not =O)
    // --- sulfur / phosphorus / nitro ---
    "CS(=O)(=O)N",
    "CS(=O)(=O)O",
    "CSC",
    "CSSC",
    "OP(=O)(O)O",
    "C[N+](=O)[O-]",
    "O=S(=O)(c1ccccc1)N",
    "Cn1cnc2c1c(=O)n(C)c(=O)n2C",
    // sulfonamide POSITIVES + DECOYS for the S(=O)(=O)N census-exact SKIP (no VF2):
    "NS(=O)(=O)N",
    "OS(=O)(=O)N",
    "NS(=O)(=O)c1ccccc1", // sulfamide, sulfamic acid, sulfanilamide
    "CS(=O)N",
    "CS(=O)(=O)Cl",
    "CS(=O)(=O)C",
    "O=S(=O)(O)O", // sulfinamide / -Cl / sulfone / sulfuric acid
    // --- charged / zwitterion ---
    "CC(=O)[O-]",
    "C[N+](C)(C)C",
    "[NH4+]",
    "[O-]c1ccccc1",
    "C[N+](C)(C)CC([O-])=O",
    "[Na+].[Cl-]",
    "c1cc[n+](C)cc1",
    // --- real drugs (rich, fused, multi-feature) ---
    "CC(=O)Oc1ccccc1C(=O)O",        // aspirin
    "CC(C)Cc1ccc(cc1)C(C)C(=O)O",   // ibuprofen
    "CN1C=NC2=C1C(=O)N(C(=O)N2C)C", // caffeine (Kekulé)
    "Cn1cnc2c1c(=O)n(c(=O)n2C)C",   // caffeine (aromatic)
    "c1ccc(cc1)S(=O)(=O)Nc1ccccc1", // a sulfonanilide
    "OC(=O)CCCCCCCCC",              // a fatty acid
    "c1ccc2c(c1)[nH]c1ccccc12",     // carbazole
    "Clc1ccc2nccnc2c1",             // a chloroquinoxaline
    // --- isotopes / explicit-H brackets (parser stress) ---
    "[13CH4]",
    "[2H]C([2H])([2H])[2H]",
    "C[15NH2]",
    // --- rare elements that share a census class with other labels: aromatic
    // and aliphatic P, and the OTHER class (B, Si, metals, salts) ---
    "c1ccpcc1",
    "CP(C)C",
    "CB(C)C",
    "C[Si](C)(C)C",
    "[Na+].[Cl-]",
    "[Zn+2].[O-]C=O",
    "[Fe]",
];

/// The fixed adversarial query battery (in addition to per-molecule self-queries).
const QUERIES: &[&str] = &[
    // elements, aromatic vs aliphatic
    "C",
    "c",
    "N",
    "n",
    "O",
    "o",
    "S",
    "s",
    "[#6]",
    "[#7]",
    "[#8]",
    "[#16]",
    // bare elements whose census class is NOT exact (shared with other labels):
    // the census shortcut must not return every row with any such atom
    "P",
    "p",
    "B",
    "[Si]",
    "[Zn]",
    "[Fe]",
    "[Na+]",
    "[#15]",
    // halogens — the ANY_HALOGEN census class
    "F",
    "Cl",
    "Br",
    "I",
    "[F,Cl,Br,I]",
    "[Cl,Br,I]",
    "[F,Cl]",
    "[F,Cl,Br,I]~[F,Cl,Br,I]",
    // mixed OR that MUST NOT force the halogen union (the conservative path)
    "[F,Cl,#7]",
    "[F,Cl,N]",
    "[F,O]",
    // degree — the C_DEG4 census class
    "[CD1]",
    "[CD2]",
    "[CD3]",
    "[CD4]",
    "[D1]",
    "[D4]",
    "[CD4,N]",
    "[C;D3]",
    "[#6D4]",
    // rings / ring sizes
    "[R]",
    "[r5]",
    "[r6]",
    "[#6;R]",
    "[#7;R]",
    "[c;r6]",
    "[n;r5]",
    "[R]~[R]",
    // X connectivity / h implicit-H / x ring-bond-count
    "[CX4]",
    "[CX3]",
    "[OX1]",
    "[OX2]",
    "[NX3]",
    "[cX3]",
    "[#6X4]",
    "[ch1]",
    "[ch0]",
    "[nh1]",
    "[Oh1]",
    "[x0]",
    "[x2]",
    "[x3]",
    "[cx2]",
    "[R;x2]",
    "[CX4;!R]",
    "[c;x2]",
    "[#6;X3]",
    // v valence / R<n> ring-count
    "[cv4]",
    "[nv3]",
    "[Cv4]",
    "[Nv3]",
    "[Ov2]",
    "[Sv6]",
    "[Pv5]",
    "[R0]",
    "[R1]",
    "[R2]",
    "[#6;R2]",
    "[c;R1]",
    "[R2]~[R2]",
    "[N;R0]",
    // stereo / directional / atom-map are parsed-and-ignored
    "[C@H]",
    "[C@@H]",
    "F/C=C/F",
    "[C:1]",
    "[N:2]",
    // bonds
    "C=O",
    "C#N",
    "C=C",
    "C-Cl",
    "C~N",
    "cc",
    "c:c",
    // negation
    "[!C]",
    "[!#6]",
    "[!c]",
    "[!R]",
    "[!#6;!#1]",
    // multi-atom motifs
    "c1ccccc1",
    "c1ccncc1",
    "C(=O)N",
    "C(=O)O",
    "C(=O)[OH]",
    "S(=O)(=O)N",
    "C(=O)OC",
    "COC=O",
    "CC(=O)OC", // ester chain (census-exact) + its 5-atom superset
    "c1ccc2ccccc2c1",
    "Nc1ccccc1",
    "Oc1ccccc1",
    "c1ccc(cc1)C(=O)O",
    "c1ccc(O)cc1",
    "c1ccc(O)nc1",
    "Oc1ccc2ccccc2c1", // phenol early-accept + near-misses
    "[#7]c1ccccc1",
    "O=C[OH]",
    "[CX3]=O",
    // charges
    "[N+]",
    "[O-]",
    "[n+]",
    "[NX4+]",
    "[O-]C=O",
    // disconnected fragments (component injectivity)
    "C.C",
    "C.N",
    "[F,Cl,Br,I].c1ccccc1",
    "c1ccccc1.c1ccccc1",
    "N.N.N",
    // AND/OR composites
    "[C,N]",
    "[c,n]",
    "[#7,#8]",
    "[#7,#8;R]",
    "[C;R]",
    "[c;R]",
    "[F,Cl;R]",
    // recursive SMARTS $(...): rooted envs, outer-AND, negated, OR-ed, ring-rooted,
    // bonded, multi-atom outer, and a nested $ inside $. The pipeline must equal the
    // oracle on every one (both share the rooted sub-matcher; the screen/census must
    // not over-prune them).
    "[$(C=O)]",
    "[$([OH])]",
    "[$(c1ccccc1)]",
    "[C$(C=O)]",
    "[O;$(OC)]",
    "[!$(C=O)]",
    "[N;!$(C=O)]",
    "[$(C=O),$([OH])]",
    "[$(C[OH])]",
    "[$(*=O)]",
    "[$(C=O)]~[$([OH])]",
    "O=[$(C=O)]",
    "[$(C[$([OH])])]",
    "[$([#7]);!$([#7]=O)]",
    "[$(cc)]",
    "[$(Cc)]",
    "c1cc[$(c)]cc1",
];

/// Build an index over a corpus and assert the pipeline equals the brute oracle for
/// every query in `queries`, returning the number of (query, corpus) pairs checked.
fn assert_pipeline_equals_oracle(corpus: &[String], queries: &[String]) -> usize {
    let (idx, skipped) = SubstructIndex::from_smiles_lossy(corpus);
    assert_eq!(skipped, 0, "the curated corpus must all parse");

    // Perceive each molecule once for the oracle side.
    let mols: Vec<_> = corpus
        .iter()
        .map(|s| parse_smiles(s).unwrap_or_else(|e| panic!("corpus parse {s:?}: {e}")))
        .collect();

    assert!(
        !corpus.is_empty(),
        "empty corpus: nothing could be compared"
    );
    assert!(
        !queries.is_empty(),
        "empty query battery: nothing could be compared"
    );

    let mut checks = 0usize;
    let mut matched = 0usize;
    let mut skipped_queries: Vec<&String> = Vec::new();
    for q in queries {
        // A query the SMARTS parser rejects is counted, and the count is asserted
        // below, so a parser regression cannot turn this gate into a no-op.
        let parsed = match parse_smarts(q) {
            Ok(p) => p,
            Err(_) => {
                skipped_queries.push(q);
                continue;
            }
        };
        // Oracle: the exact set of molecules the brute matcher accepts.
        let oracle: BTreeSet<u32> = mols
            .iter()
            .enumerate()
            .filter(|(_, m)| brute::matches(&parsed, m))
            .map(|(i, _)| i as u32)
            .collect();
        // Pipeline: screen → census → verify.
        let got: BTreeSet<u32> = idx
            .search(q)
            .unwrap_or_else(|e| panic!("search {q:?}: {e:?}"))
            .into_iter()
            .collect();

        if got != oracle {
            let show = |ids: BTreeSet<u32>| -> String {
                ids.into_iter()
                    .take(6)
                    .map(|i| format!("{i}:{:?}", corpus[i as usize]))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            let missing: BTreeSet<u32> = oracle.difference(&got).copied().collect();
            let extra: BTreeSet<u32> = got.difference(&oracle).copied().collect();
            panic!(
                "DIVERGENCE for query {q:?}\n  FALSE NEGATIVES (brute matched, pipeline dropped — CRITICAL): {}\n  false positives (pipeline kept, brute rejected): {}",
                show(missing),
                show(extra),
            );
        }
        checks += corpus.len();
        matched += oracle.len();
    }

    // ANTI-VACUITY. Every one of these can fail for a real reason:
    //  - a SMARTS parser regression that starts rejecting battery queries,
    //  - a corpus/query list accidentally emptied,
    //  - an oracle that stopped matching anything (so `got == oracle` compares
    //    empty against empty for every query and proves nothing).
    assert!(
        skipped_queries.is_empty(),
        "{} of {} queries no longer parse as SMARTS — the gate silently skipped \
         them: {:?}",
        skipped_queries.len(),
        queries.len(),
        skipped_queries.iter().take(8).collect::<Vec<_>>()
    );
    assert_eq!(
        checks,
        queries.len() * corpus.len(),
        "every (query, molecule) pair must have been compared"
    );
    assert!(
        matched > 0,
        "the oracle matched nothing at all — comparison was vacuous"
    );
    assert!(
        matched < checks,
        "the oracle matched EVERYTHING ({matched}/{checks}) — no negatives to separate"
    );
    checks
}

/// Every molecule parsed as its own SMARTS must match at least itself: the
/// sharpest false-negative probe for the screen/census. The full pipeline must
/// also equal brute for each self-query.
fn self_queries(corpus: &[String]) -> Vec<String> {
    corpus
        .iter()
        // Keep only SMILES that re-parse as SMARTS (the supported overlap).
        .filter(|s| parse_smarts(s).is_ok())
        .cloned()
        .collect()
}

#[test]
fn pipeline_equals_brute_oracle_on_fixed_battery() {
    let corpus = full_corpus();
    let queries: Vec<String> = QUERIES.iter().map(|s| s.to_string()).collect();
    let n = assert_pipeline_equals_oracle(&corpus, &queries);
    // The count is ASSERTED, not merely printed: it is the only evidence that the
    // battery actually ran against the whole corpus.
    assert_eq!(
        n,
        QUERIES.len() * corpus.len(),
        "fixed battery compared {n} pairs, expected {} queries x {} molecules",
        QUERIES.len(),
        corpus.len()
    );
    assert!(n > 10_000, "fixed battery is unexpectedly small: {n} pairs");
    eprintln!("fixed battery: {n} (query,molecule) pairs agree with the oracle");
}

#[test]
fn pipeline_equals_brute_oracle_on_self_queries() {
    let corpus = full_corpus();
    let sq = self_queries(&corpus);
    assert!(
        sq.len() >= CORPUS.len() / 2,
        "most molecules should self-parse as SMARTS"
    );
    let n = assert_pipeline_equals_oracle(&corpus, &sq);
    assert_eq!(
        n,
        sq.len() * corpus.len(),
        "self-query sweep did not cover every pair"
    );
    assert!(
        n > 10_000,
        "self-query sweep is unexpectedly small: {n} pairs"
    );
    eprintln!("self-queries: {n} pairs agree; every molecule found its own structure");
}

/// A small deterministic LCG so generated queries reproduce exactly on failure.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 16
    }
    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[(self.next() as usize) % xs.len()]
    }
}

#[test]
fn pipeline_equals_brute_oracle_on_generated_composites() {
    // Mechanically compose primitives into AND/OR/disconnected queries and confirm
    // each still equals the oracle — stresses the forced-label / census / screen
    // interaction on shapes the fixed list does not enumerate.
    let atoms = [
        "C", "c", "N", "n", "O", "F", "Cl", "[R]", "[r6]", "[CD3]", "[CD4]", "[#7]", "[!#6]",
    ];
    let mut rng = Lcg(0x5EED_1234_ABCD);
    let mut queries: Vec<String> = Vec::new();
    for _ in 0..120 {
        let a = rng.pick(&atoms);
        let b = rng.pick(&atoms);
        match rng.next() % 4 {
            0 => queries.push(format!("[{},{}]", strip(a), strip(b))), // OR
            1 => queries.push(format!("[{};{}]", strip(a), strip(b))), // AND
            2 => queries.push(format!("{}.{}", a, b)),                 // disconnected
            _ => queries.push(format!("{}~{}", a, b)),                 // any-bond pair
        }
    }
    let corpus = full_corpus();
    assert_eq!(queries.len(), 120, "the generator must emit 120 composites");
    let n = assert_pipeline_equals_oracle(&corpus, &queries);
    assert_eq!(
        n,
        queries.len() * corpus.len(),
        "composite sweep did not cover every pair"
    );
    eprintln!("generated composites: {n} pairs agree with the oracle");
}

/// Turn an atom token into a bracket-expression primitive (drop surrounding `[]`).
fn strip(tok: &str) -> String {
    tok.trim_start_matches('[')
        .trim_end_matches(']')
        .to_string()
}
