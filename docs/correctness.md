# Correctness

Both engines must return exactly what a brute-force scan returns. If a speed-up
needs a rule below relaxed, the speed-up is rejected.

## Invariants

Similarity:

1. `search(q, t)` returns exactly the rows whose f64 Tanimoto on the stored bits
   is `>= t`. No false negatives, no false positives.
2. A prune rejects only on a proven upper bound, never on an estimate.
3. The inverted bitmap equals the linear scan, byte for byte
   (`bitmap_matches_linear`).
4. An append never changes an answer: `SegmentedIndex` equals one arena over
   the same rows, for threshold and kNN.
5. Edges: `t <= 0` returns all rows (score 0 included); `t > 1` and `t = NaN`
   return empty without panic. `score == 1.0` means equal bits.

Substructure:

6. `idx.search(q) == { i : brute::matches(parse_smarts(q), mols[i]) }`. The
   oracle in `brute.rs` is ground truth. Do not optimise it.
7. The screen is a superset filter. If a query breaks its guarantee (OR,
   wildcards, recursion), it degrades to "verify everything".
8. A census class may skip VF2 only with a proof in both directions. A
   one-direction class (`PHENOL_C`) may accept, and still verifies the rest.
9. SSSR-derived ring filters may never reject.

Process:

10. Never weaken a test, assertion or gate to make a change pass.
11. `#![forbid(unsafe_code)]` stays in all three library crates.
12. Every performance number states machine, ISA, dataset, width, threads,
    warm or cold, search-only or end to end.
13. A limit may refuse; it may never change an answer. A timed-out search
    returns `SearchError::Timeout` and no rows. The screen's path budget
    degrades only toward a superset.

## Gates

| Gate | Proves |
| --- | --- |
| Unit tests in `crates/*/src` | `search == brute` on a curated corpus; census class proofs. |
| `mossflux-index/tests/*` | Linear, bitmap, segmented and brute agree; threshold extremes; adversarial inputs. |
| `golden_smarts.rs` | Known-answer parses and matches. |
| `spelling_independence.rs` | Kekule and aromatic spellings give the same answer. |
| `differential_fuzz.rs` | Full pipeline against the oracle over an adversarial battery, plus self-queries. |
| `scale_exactness.rs` | `search == brute` on a generated corpus; every hit was a screen candidate. |
| `hostile_input.rs` | Every input limit and hostile-input fix. |
| `cross_process_determinism.rs` | Re-execs itself to catch hash-order dependence. |
| `rdkit_reference.rs` | The only gate outside this crate: agreement with RDKit. |
| `bench` | f64 truth on every query; `--inverted` also gates bitmap against linear. |
| `substruct-bench` | Refuses to print timings unless the pipeline equals the oracle on its probes. |

The oracle uses the same parser and perception as the pipeline, so it cannot
see a shared parse or perception error. `golden_smarts.rs` and
`rdkit_reference.rs` cover that.

### Arming the full gates

A plain `cargo test --release --all` runs reduced sizes and skips RDKit. All
variables are read in `crates/mossflux-substruct/tests/`.

| Variable | Default | Effect |
| --- | --- | --- |
| `MOSSFLUX_FUZZ_CHEMBL` | unset (109 embedded molecules) | Adds real molecules from a decompressed file, absolute path. |
| `MOSSFLUX_FUZZ_CHEMBL_N` | 3000 when the above is set | Sample size. Cost grows about quadratically: start at 200. |
| `MOSSFLUX_STAGE5_N` | 20,000 | Corpus size for `scale_exactness.rs`. |
| `MOSSFLUX_RDKIT_PYTHON` | unset (reports not run) | Python with `rdkit`. When set, a broken interpreter fails. |

`MOSSFLUX_DETERMINISM_CHILD` and `MOSSFLUX_DIGEST` are internal; do not set them.

```sh
gzcat data/chembl_37_chemreps.txt.gz | head -20001 > /tmp/chembl_sample.tsv
MOSSFLUX_FUZZ_CHEMBL=/tmp/chembl_sample.tsv MOSSFLUX_FUZZ_CHEMBL_N=200 \
  cargo test --release -p mossflux-substruct --test differential_fuzz -- --nocapture
MOSSFLUX_STAGE5_N=100000 \
  cargo test --release -p mossflux-substruct --test scale_exactness -- --nocapture
MOSSFLUX_RDKIT_PYTHON=$PWD/.venv/bin/python \
  cargo test --release -p mossflux-substruct --test rdkit_reference -- --nocapture
```

Observed: RDKit differential 114/114 queries agree over 100 molecules (2,816
hits). Any census change needs the fuzz and scale gates both armed.

### Full-corpus checks

A perception or screen change must leave every real molecule unchanged. Hash
the perceived `MolGraph`, `target_fingerprint`, `target_census` and SMARTS
parse for all 2,897,819 ChEMBL 37 rows on the old and new trees and compare.
The 2026-10-03 changes passed this, and 37 SMARTS queries equalled the oracle
over all 2,896,304 parseable rows.

## Open items

- `scripts/gen_fingerprints.py` does not strip salts, does not expose
  `useChirality`, and does not record its Morgan parameters in the FPS header.
- No cross-tool hit-set equality gate between mossflux and FPSim2.
