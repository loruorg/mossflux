//! Scale exactness, screen pruning, memory linearity and a memory projection.
//!
//! Runs `search == brute-force over all rows` on a synthetic corpus of thousands
//! to ~100k molecules, so scale-only bugs (a transpose that mis-addresses a row
//! past 64k, a wrapping `u32` row id, a parallel-verify race) show up as a
//! whole-system disagreement. Also checks that the screen is a superset of the
//! hits, that selective queries prune to candidates ≪ n, that spelling
//! independence holds at scale, and that memory grows linearly with `n`.
//!
//! # Sizing
//!
//! The brute oracle costs `O(n · |battery| · brute_per_mol)`, so the default size
//! is modest. Set `MOSSFLUX_STAGE5_N` for a heavier release-mode run:
//!
//! ```text
//! MOSSFLUX_STAGE5_N=100000 cargo test -p mossflux-substruct --release \
//!     --test scale_exactness -- --nocapture
//! ```
//!
//! Run with `--nocapture` to see the timing and memory-projection prints.

use mossflux_substruct::index::SubstructIndex;
use mossflux_substruct::{brute, parse_smarts, parse_smiles};
use std::collections::HashSet;
use std::time::Instant;

/// Default corpus size: large enough that the transpose spans many column words
/// and low-selectivity queries take the parallel-verify path. Override with
/// `MOSSFLUX_STAGE5_N`.
const DEFAULT_N: usize = 20_000;

/// Resolve the corpus size from `MOSSFLUX_STAGE5_N`, falling back to [`DEFAULT_N`].
fn corpus_size() -> usize {
    std::env::var("MOSSFLUX_STAGE5_N")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&n: &usize| n > 0)
        .unwrap_or(DEFAULT_N)
}

/// The motif alphabet: a spread of chains, rings, aromatics, heteroaromatics,
/// carbonyls, halogens and amines, so the generated corpus exercises every
/// fragment class the screen pins a bit on.
const MOTIFS: &[&str] = &[
    "c1ccccc1",
    "C1CCCCC1",
    "CC(=O)O",
    "CCO",
    "CCN",
    "c1ccncc1",
    "Cc1ccccc1",
    "CC(C)C(=O)O",
    "OCC(O)CO",
    "FC(F)(F)F",
    "ClCCBr",
    "Nc1ccccc1",
    "c1ccc(O)cc1",
    "C1CCNCC1",
    "CC(=O)N",
    "CC#N",
    "c1ccoc1",
    "c1ccsc1",
    "C1CCOC1",
    "BrCCBr",
];

/// Generate `n` valid SMILES deterministically: a carbon-chain prefix of cycling
/// length glued onto a cycling motif, giving a diverse fingerprint population.
fn corpus(n: usize) -> Vec<String> {
    (0..n)
        .map(|i| {
            let base = MOTIFS[i % MOTIFS.len()];
            let chain = "C".repeat(1 + (i % 6));
            format!("{chain}{base}")
        })
        .collect()
}

/// The exactness battery: forced single atoms / paths / rings (the screen has
/// teeth), bond primitives, ring+property combos, logical bracket expressions, and
/// a disconnected fragment (the degrade-to-no-screen path). Each yields real hits
/// and real misses against the generated corpus.
const BATTERY: &[&str] = &[
    // single-atom predicates (low selectivity → big candidate sets → parallel verify)
    "C",
    "c",
    "N",
    "O",
    "F",
    "Cl",
    "Br",
    "[#6]",
    "[#7]",
    "[#8]",
    "a",
    "[R]",
    "[r6]",
    // two-atom paths with various bonds
    "CC",
    "CO",
    "CN",
    "C=O",
    "C#N",
    "cc",
    "co",
    "cn",
    "c:c",
    "[#6][#8]",
    "CCl",
    // branched / longer paths / rings (more selective → screen should prune hard)
    "CCC",
    "CC(=O)O",
    "FC(F)(F)F",
    "c1ccccc1",
    "c1ccncc1",
    "C1CCCCC1",
    "c1ccc(O)cc1",
    "Nc1ccccc1",
    "[#6]1[#6][#6][#6][#6][#6]1",
    // ring + property / bond logic / negation
    "[c;R]1[c;R][c;R][c;R][c;R][c;R]1",
    "[R]~[R]",
    "[#7;R]",
    "[!c]~[!c]",
    // disconnected fragment → empty mask → screen-everything degradation
    "C.O",
];

/// Selective queries the screen must prune hard: each names a fragment carried by
/// only a fraction of the motif alphabet.
const SELECTIVE_PROBES: &[&str] = &[
    "FC(F)(F)F", // trifluoromethyl — only the FC(F)(F)F motif
    "c1ccncc1",  // pyridine ring
    "c1ccoc1",   // furan ring
    "c1ccsc1",   // thiophene ring
    "BrCCBr",    // dibromoethane
    "C#N",       // nitrile
    "Nc1ccccc1", // aniline
];

// ---------------------------------------------------------------------------
// Exactness at scale, screen pruning, and timing.
// ---------------------------------------------------------------------------

/// Scale exactness gate: for the whole battery, `search` returns exactly the set
/// the brute-force oracle accepts over every molecule. Also asserts the screen is a
/// sound superset, that selective queries prune to candidates ≪ n, and prints
/// build/search timing.
#[test]
fn scale_search_equals_brute_force_and_screen_prunes() {
    let n = corpus_size();
    let smiles = corpus(n);

    // ---- Build (timed) ----
    let t_build = Instant::now();
    let idx = SubstructIndex::from_smiles(&smiles).unwrap();
    let build = t_build.elapsed();
    assert_eq!(idx.len(), n, "every input row must be indexed");

    // Memory split for the print: the screen (per-row fingerprints + transposed
    // column store) and the molecule graphs (the in-RAM CT store).
    let mem = idx.mem_bytes();
    let screen_bytes = idx.screen().mem_bytes();
    let graph_bytes: usize = (0..n as u32).map(|r| idx.mol(r).mem_bytes()).sum();
    let bytes_per_mol = mem as f64 / n as f64;

    // ---- The exactness sweep (timed) over the whole battery ----
    let mut total_pairs = 0usize;
    let mut total_hits = 0usize;
    let mut pruned_examples = 0usize;
    let t_sweep = Instant::now();

    for &q in BATTERY {
        // The shipped pipeline answer.
        let got = idx
            .search(q)
            .unwrap_or_else(|e| panic!("battery query {q:?} should parse: {e}"));
        let got_set: HashSet<u32> = got.iter().copied().collect();
        assert_eq!(got.len(), got_set.len(), "query {q:?}: duplicate rows");
        assert!(
            got.windows(2).all(|w| w[0] < w[1]),
            "query {q:?}: result not strictly ascending"
        );

        // The screen candidate set must be a superset of the verified hits.
        // `screen_candidates` is the same code as `search`'s first stage.
        let (_mask_bits, cands) = idx.screen_candidates(q).unwrap();
        let cand_set: HashSet<u32> = cands.iter().copied().collect();
        for &h in &got {
            assert!(
                cand_set.contains(&h),
                "query {q:?}: verified hit {h} not in screen candidates (screen \
                 dropped a real match — superset property VIOLATED)"
            );
        }
        // Candidates never exceed the corpus. Even with an empty bit-screen mask,
        // the census stage may still prune (e.g. `[#7;R]` forces a census class but
        // no path bit), so candidates can be < n at mask_bits == 0.
        {
            assert!(cands.len() <= n, "query {q:?}: candidates exceed corpus");
            if cands.len() < n {
                pruned_examples += 1;
            }
        }

        // The oracle answer: brute-force monomorphism over EVERY molecule.
        let parsed = parse_smarts(q).unwrap();
        let mut want: Vec<u32> = Vec::new();
        for row in 0..n as u32 {
            // Oracle re-parses the SMILES (the index stores the compact MatchMol).
            let mol = parse_smiles(&smiles[row as usize]).unwrap();
            if brute::matches(&parsed, &mol) {
                want.push(row);
            }
        }
        let want_set: HashSet<u32> = want.iter().copied().collect();

        // EXACT equality at scale, with a diagnostic naming the disagreement.
        assert_eq!(
            got_set,
            want_set,
            "query {q:?}: search != brute-force at scale (n={n}).\n  \
             screen+vf2 extra: {:?}\n  brute-only (MISSED!): {:?}",
            got_set.difference(&want_set).take(8).collect::<Vec<_>>(),
            want_set.difference(&got_set).take(8).collect::<Vec<_>>(),
        );

        total_pairs += n;
        total_hits += want.len();
    }
    let sweep = t_sweep.elapsed();

    // The gate must be a substantial sweep with both outcomes present, or a
    // degenerate corpus could pass it vacuously.
    assert!(
        total_pairs > 100_000,
        "gate should compare many (mol, query) pairs, got {total_pairs}"
    );
    assert!(total_hits > 0, "corpus should yield real matches");
    assert!(
        total_hits < total_pairs,
        "corpus should also yield non-matches, not all-hits"
    );
    assert!(
        pruned_examples > 0,
        "no battery query pruned below n — the screen showed no power at scale"
    );

    // ---- Selective queries leave candidates ≪ n ----
    // Each probe names a fragment in a minority of motifs; require fewer than n/2
    // candidates (headroom for chain-prefix path collisions).
    for &q in SELECTIVE_PROBES {
        let (mask_bits, cands) = idx.screen_candidates(q).unwrap();
        assert!(
            mask_bits > 0,
            "selective probe {q:?}: expected a non-empty required mask"
        );
        assert!(
            cands.len() * 2 < n,
            "selective probe {q:?}: screen left {} / {n} candidates (≥ n/2) — not \
             pruning",
            cands.len()
        );
        // And the screen never drops a match: the verified hits are all candidates.
        let cand_set: HashSet<u32> = cands.iter().copied().collect();
        for h in idx.search(q).unwrap() {
            assert!(
                cand_set.contains(&h),
                "probe {q:?}: hit {h} not screened in"
            );
        }
    }

    // ---- Timing and memory print ----
    let mib = 1024.0 * 1024.0;
    let screen_bpm = screen_bytes as f64 / n as f64;
    let graph_bpm = graph_bytes as f64 / n as f64;
    println!(
        "\n[stage5 scale] n = {n} molecules\n  \
         build      : {build:?}  ({:.1} mol/ms)\n  \
         battery    : {} queries x {n} mols = {total_pairs} pairs in {sweep:?} \
         ({total_hits} oracle hits)\n  \
         search avg : {:.3} ms/query (screen+verify over {n} mols)\n  \
         memory     : {:.1} MiB total  ({bytes_per_mol:.1} bytes/molecule)\n    \
         - screen (fps + transposed columns) : {:.1} MiB  ({screen_bpm:.1} B/mol)\n    \
         - graphs (in-RAM CT store)          : {:.1} MiB  ({graph_bpm:.1} B/mol)",
        n as f64 / build.as_secs_f64().max(1e-9) / 1000.0,
        BATTERY.len(),
        sweep.as_secs_f64() * 1e3 / BATTERY.len() as f64,
        mem as f64 / mib,
        screen_bytes as f64 / mib,
        graph_bytes as f64 / mib,
    );

    // Project the measured per-molecule footprint to 1e9 molecules, split between
    // screen bits and the CT store. It counts reserved bytes, so it does not depend
    // on debug vs release.
    let gib = 1024.0 * 1024.0 * 1024.0;
    let billion = 1_000_000_000.0_f64;
    println!(
        "  PROJECTION : {bytes_per_mol:.1} bytes/mol -> {:.1} GiB for 1e9 molecules \
         in RAM\n    \
         = {:.1} GiB screen bits + {:.1} GiB CT store (graphs)",
        bytes_per_mol * billion / gib,
        screen_bpm * billion / gib,
        graph_bpm * billion / gib,
    );
}

// ---------------------------------------------------------------------------
// Memory grows linearly, no quadratic blow-up.
// ---------------------------------------------------------------------------

/// Memory linearity gate: build the same corpus at a base size and 4×, and assert
/// bytes-per-molecule stays roughly constant. A hidden O(n²) structure (e.g. a
/// per-pair table) would make it grow.
#[test]
fn memory_is_roughly_linear_in_corpus_size() {
    // No brute sweep here, so small sizes suffice; both span several column words.
    let base = 2_000usize;
    let big = base * 4; // 8_000

    let idx_small = SubstructIndex::from_smiles(&corpus(base)).unwrap();
    let idx_big = SubstructIndex::from_smiles(&corpus(big)).unwrap();

    let bpm_small = idx_small.mem_bytes() as f64 / base as f64;
    let bpm_big = idx_big.mem_bytes() as f64 / big as f64;

    println!(
        "\n[stage5 linearity] bytes/molecule: n={base} -> {bpm_small:.1}, \
         n={big} -> {bpm_big:.1}  (ratio {:.3})",
        bpm_big / bpm_small
    );

    // Graphs, per-row fingerprints and the `FP_BITS · ceil(n/64)` column store are
    // all linear in `n`. A quadratic structure would grow bytes/molecule ~4× here;
    // require ±20%.
    let ratio = bpm_big / bpm_small;
    assert!(
        (0.8..=1.2).contains(&ratio),
        "bytes/molecule changed by {ratio:.3}x across a 4x scale-up — memory is not \
         linear (suspect a quadratic structure)"
    );

    // Total memory must grow, but stay well under the 16× quadratic bound.
    let total_small = idx_small.mem_bytes() as f64;
    let total_big = idx_big.mem_bytes() as f64;
    assert!(
        total_big > total_small,
        "more molecules must use more memory"
    );
    assert!(
        total_big < total_small * 8.0,
        "total memory grew {:.1}x for a 4x scale-up — super-linear",
        total_big / total_small
    );
}

// ---------------------------------------------------------------------------
// Spelling independence at scale.
// ---------------------------------------------------------------------------

/// Spelling independence inside a large index. Two corpora differ only at one
/// row, which holds the same molecule in Kekulé or aromatic spelling; every query
/// must agree on whether that row matches. The filler makes the property hold
/// through the full at-scale screen, not just a one-row index.
#[test]
fn spelling_independence_holds_at_scale() {
    // Equivalent (Kekulé vs aromatic) spellings of the same molecule.
    const EQUIVALENT: &[(&str, &str)] = &[
        ("c1ccccc1", "C1=CC=CC=C1"),               // benzene
        ("Cc1ccccc1", "CC1=CC=CC=C1"),             // toluene
        ("Oc1ccccc1", "OC1=CC=CC=C1"),             // phenol
        ("c1ccncc1", "C1=CC=NC=C1"),               // pyridine
        ("c1cc[nH]c1", "C1=CC=CN1"),               // pyrrole
        ("c1ccc2ccccc2c1", "C1=CC=C2C=CC=CC2=C1"), // naphthalene
    ];
    // A query battery with aromatic, aliphatic, ring and bracket primitives — the
    // discriminating shapes for perception.
    const QUERIES: &[&str] = &[
        "a", "A", "c", "C", "n", "N", "o", "O", "[#6]", "[#7]", "[nH]", "cc", "CC", "c:c", "C=C",
        "c1ccccc1", "[R]", "[r6]", "[r5]", "[!R]", "[c;H1]", "[c,n]",
    ];

    let filler_n = 4_000usize; // bulk so the at-scale screen is fully exercised
    let probe_row = filler_n as u32 / 2; // place the probe molecule mid-corpus
    let mut compared = 0usize;
    let mut probe_hits = 0usize;
    let mut probe_misses = 0usize;

    for &(kekule, aromatic) in EQUIVALENT {
        // Build two big corpora that are byte-identical except the probe row, which
        // holds the same molecule under the two spellings. Surrounding filler is a
        // simple, distinct molecule so it can never alias the probe's answer.
        let build_with = |probe: &str| {
            let mut smiles: Vec<String> = corpus(filler_n);
            smiles[probe_row as usize] = probe.to_string();
            SubstructIndex::from_smiles(&smiles).unwrap()
        };
        let idx_k = build_with(kekule);
        let idx_a = build_with(aromatic);

        for &q in QUERIES {
            // Compare membership of `probe_row` under each spelling. The search
            // must succeed, so a query that stops parsing cannot pass as `Err == Err`.
            let hit = |ix: &SubstructIndex, spelling: &str| -> bool {
                ix.search(q)
                    .unwrap_or_else(|e| panic!("query {q:?} failed on `{spelling}`: {e:?}"))
                    .binary_search(&probe_row)
                    .is_ok()
            };
            let k_hit = hit(&idx_k, kekule);
            let a_hit = hit(&idx_a, aromatic);
            assert_eq!(
                k_hit, a_hit,
                "SPELLING-DEPENDENT AT SCALE: query {q:?} on row {probe_row} differs \
                 between Kekulé `{kekule}` ({k_hit}) and aromatic `{aromatic}` \
                 ({a_hit})"
            );
            if k_hit {
                probe_hits += 1;
            } else {
                probe_misses += 1;
            }
            compared += 1;
        }
    }
    // ANTI-VACUITY: every pair ran, and the battery separates matches from
    // non-matches at the probe row rather than answering `false` everywhere.
    assert_eq!(
        compared,
        EQUIVALENT.len() * QUERIES.len(),
        "not every (spelling pair, query) was compared"
    );
    assert!(
        probe_hits > 20,
        "probe row matched only {probe_hits} queries"
    );
    assert!(
        probe_misses > 10,
        "probe row missed only {probe_misses} queries"
    );
}
