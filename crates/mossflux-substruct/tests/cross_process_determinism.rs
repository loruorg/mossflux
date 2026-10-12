//! Cross-process determinism.
//!
//! Rust seeds `RandomState` once per process, so an in-process determinism check
//! cannot see a `HashMap`/`HashSet` iteration-order dependence. Perception does
//! use a hash map (`perception.rs` starts the perimeter walk at an arbitrary
//! `adj.keys().next()`); the answer must not depend on that choice.
//!
//! This test re-execs its own binary several times. Each child gets a fresh
//! `RandomState` seed, so a hash-order dependence makes the digests differ. The
//! parent then checks its own digest against the children's.
//!
//! The corpus deliberately leans on fused/bridged/spiro polycyclics and charged
//! heteroaromatics, which is where ring perception, the perimeter walk and
//! aromaticity assignment do the most work.

use std::process::Command;

use mossflux_substruct::index::SubstructIndex;

/// Set in a re-exec'd child; makes the test print its digest and return.
const CHILD_ENV: &str = "MOSSFLUX_DETERMINISM_CHILD";
const MARKER: &str = "MOSSFLUX_DIGEST=";
const TEST_NAME: &str = "search_and_perception_are_stable_across_processes";

/// Ring-heavy, perception-hostile molecules.
const CORPUS: &[&str] = &[
    // fused / bridged / spiro — the perimeter walk's real work
    "c1ccc2ccccc2c1",
    "c1ccc2c(c1)cccc2",
    "c1ccc2c(c1)[nH]c1ccccc12",
    "c1ccc2ncccc2c1",
    "c1ccc2c(c1)nccn2",
    "C1CC2CCC1CC2",
    "C1CCC2(CC1)CCCC2",
    "C1CC2CCC3CCC1C23",
    "Clc1ccc2nccnc2c1",
    "c1ccc2[nH]ccc2c1",
    // charged / N-oxide aromatics (the aromaticity edge cases)
    "c1cc[n+]([O-])cc1",
    "[O-][n+]1ccccc1",
    "c1cc[n+](C)cc1",
    "[nH+]1ccccc1",
    "[O-]c1ccccc1",
    // Kekule vs aromatic pairs, so perception must reconcile both spellings
    "C1=CC=CC=C1",
    "c1ccccc1",
    "C1=CC=C2C=CC=CC2=C1",
    "O=c1cccc[nH]1",
    "O=C1C=CC=CN1",
    // heteroaromatics and macro/small rings
    "c1ccoc1",
    "c1ccsc1",
    "c1cc[nH]c1",
    "c1cnc[nH]1",
    "C1CCCCCCCCC1",
    "C1CC1",
    "C1CCNCC1",
    // real, feature-dense drugs
    "CC(=O)Oc1ccccc1C(=O)O",
    "Cn1cnc2c1c(=O)n(C)c(=O)n2C",
    "CC(C)Cc1ccc(cc1)C(C)C(=O)O",
    "c1ccc(cc1)S(=O)(=O)Nc1ccccc1",
    "CS(=O)(=O)N",
    "OP(=O)(O)O",
    "C[N+](C)(C)CC([O-])=O",
];

const QUERIES: &[&str] = &[
    "c",
    "a",
    "A",
    "C",
    "n",
    "o",
    "s",
    "[R]",
    "[R1]",
    "[R2]",
    "[r5]",
    "[r6]",
    "[!R]",
    "cc",
    "CC",
    "c:c",
    "C=C",
    "[R]~[R]",
    "[R2]~[R2]",
    "c1ccccc1",
    "c1ccncc1",
    "C1CCCCC1",
    "c1ccc2ccccc2c1",
    "[n+]",
    "[O-]",
    "[nH]",
    "[cH]",
    "[x3]",
    "[x2]",
    "C(=O)N",
    "C(=O)O",
    "S(=O)(=O)N",
    "[$(c1ccccc1)]",
    "[$(C=O)]",
    "[!$(C=O)]",
];

/// FNV-1a over the whole (corpus, query) answer surface, plus the index's memory
/// footprint. Any drift — a different hit set, a different row, a different build
/// size — changes the digest.
fn digest() -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut h = OFFSET;
    let mut mix = |bytes: &[u8]| {
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(PRIME);
        }
    };

    let smiles: Vec<String> = CORPUS.iter().map(|s| s.to_string()).collect();
    let idx = SubstructIndex::from_smiles(&smiles).expect("determinism corpus must build");
    assert_eq!(idx.len(), CORPUS.len());
    mix(&(idx.mem_bytes() as u64).to_le_bytes());

    let mut total_hits = 0usize;
    for q in QUERIES {
        mix(q.as_bytes());
        let hits = idx
            .search(q)
            .unwrap_or_else(|e| panic!("determinism query {q:?} must parse: {e:?}"));
        mix(&(hits.len() as u64).to_le_bytes());
        for row in &hits {
            mix(&row.to_le_bytes());
        }
        total_hits += hits.len();
    }
    // Anti-vacuity: a digest over an all-empty answer surface would be stable and
    // meaningless.
    assert!(
        total_hits > 200,
        "digest covered only {total_hits} hits — not enough answer surface"
    );
    assert!(
        total_hits < QUERIES.len() * CORPUS.len(),
        "every query matched every molecule; the digest separates nothing"
    );
    h
}

#[test]
fn search_and_perception_are_stable_across_processes() {
    if std::env::var(CHILD_ENV).is_ok() {
        // Child mode: print the digest for the parent to collect, then stop
        // (without re-spawning, which would recurse forever).
        println!("{MARKER}{:016x}", digest());
        return;
    }

    let exe = std::env::current_exe().expect("test binary path");
    let mut child_digests: Vec<String> = Vec::new();
    for run in 0..3 {
        let out = Command::new(&exe)
            .args(["--exact", TEST_NAME, "--nocapture", "--test-threads=1"])
            .env(CHILD_ENV, "1")
            .output()
            .unwrap_or_else(|e| panic!("re-exec {exe:?} failed: {e}"));
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(
            out.status.success(),
            "child run {run} failed ({:?})\n--- stdout ---\n{stdout}\n--- stderr ---\n{stderr}",
            out.status
        );
        // `--nocapture` interleaves the print with the harness's own progress
        // line, so scan the whole stdout for the marker rather than a line prefix.
        let at = stdout.find(MARKER).unwrap_or_else(|| {
            panic!("child run {run} printed no digest\n--- stdout ---\n{stdout}")
        }) + MARKER.len();
        let d: String = stdout[at..]
            .chars()
            .take_while(|c| c.is_ascii_hexdigit())
            .collect();
        assert_eq!(
            d.len(),
            16,
            "child run {run} printed a malformed digest {d:?}"
        );
        child_digests.push(d);
    }

    assert_eq!(child_digests.len(), 3, "all three children must report");
    // Three processes, three RandomState seeds, one answer. A hash-iteration-order
    // dependence in parsing, perception, screening or matching shows up here.
    assert!(
        child_digests.windows(2).all(|w| w[0] == w[1]),
        "NONDETERMINISM ACROSS PROCESSES: child digests {child_digests:?} differ. \
         The answer depends on something that is not fixed by the input — a hash \
         iteration order, an address, or a clock."
    );
    // And this process agrees with them.
    assert_eq!(
        format!("{:016x}", digest()),
        child_digests[0],
        "the parent process disagrees with its own children"
    );
}
