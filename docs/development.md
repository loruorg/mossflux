# Development

## Commands

```sh
cargo build --release --all
cargo test --release --all
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all
cargo +1.95 check --all --all-targets   # MSRV; use a separate CARGO_TARGET_DIR
cargo audit
```

CI (`.github/workflows/ci.yml`) runs all of these, with tests on Linux and
macOS. All must pass before a change is done. Tests at reduced sizes run by
default; see [correctness.md](correctness.md) to arm the full gates.

## CLI

`mossflux` writes results to stdout and diagnostics to stderr. Exit 0 on
success (also on a closed pipe), 1 on error, 2 on usage error.

| Subcommand | Required | Does |
| --- | --- | --- |
| `search` | `--fps` and one of `--query-index` (file record number), `--query-id`, `--query` (hex, file width) | One similarity query. `--threshold` or `--knn`; `--format tsv`. |
| `substruct` | `--smiles` and `--smarts` (repeatable) or `--smarts-file` | Exact SMARTS search; index built per run. `--count`, `--strict`, `--timeout`, `--format tsv`. |
| `info` | `--fps` | Arena statistics. |
| `tanimoto` | two hex strings | Similarity of two fingerprints. |
| `bench` | `--fps` or `--synth-n` | Similarity throughput; `--brute`, `--inverted`, `--fold`, `--abandon`. |
| `substruct-bench` | none | Substructure build, memory and latency over a SMARTS battery. |

## Datasets

`data/` is gitignored. Never commit it.

| File | Size | Content | Source |
| --- | ---: | --- | --- |
| `chembl_37_chemreps.txt.gz` | 279 MiB | ChEMBL 37 SMILES, TSV | `scripts/fetch_chembl.sh` |
| `chembl_37.fps.gz` | 168 MiB | 2048-bit RDKit Morgan r=2, FPS | EBI ChEMBL FTP, same directory |
| `chembl_37.h5` | 314 MiB | FPSim2 index | EBI ChEMBL FTP, same directory |

ChEMBL 37 has 2,897,819 compounds (2,897,820 lines with the header).
`chembl_37.fps.gz` was not made by `scripts/gen_fingerprints.py`; do not
regenerate it. The 256-bit comparison builds its own `data/chembl_37_256.fps`
and `.h5` with `scripts/reproduce_arthor_compare.sh`.

```sh
./scripts/fetch_chembl.sh
python scripts/gen_fingerprints.py data/chembl_37_chemreps.txt.gz \
  data/chembl_37_256.fps --nbits 256
```

## Traps

1. **rustfmt.** The tree is clean. If `cargo fmt` gives a diff much larger than
   your change, stop and find why.
2. **`MOSSFLUX_FUZZ_CHEMBL` path.** Tests run in the crate directory: use an
   absolute path to a decompressed file (`.gz` is refused).
3. **Fuzz cost.** Grows about quadratically. N=200 takes seconds; the default
   3000 did not finish in 10 minutes.
4. **`target-cpu=native`.** Binaries are not portable. For a znver3 build, set
   `RUSTFLAGS` and a separate `CARGO_TARGET_DIR`, or you overwrite the native
   binary.
5. **Release profile.** `overflow-checks`, fat LTO and `codegen-units = 1` are
   deliberate. Do not change them for a benchmark.
6. **No planner.** `bench --abandon auto` means off. Early abandon is opt-in
   with `--abandon on`.
7. **Query sampling.** `bench --queries` is seeded random sampling (seed
   `0xB1054ED`). A different seed moves every number.
8. **Two similarity paths.** If you touch the linear or bitmap path, run
   `bench --inverted`.
9. **SMARTS scope.** Atom maps and chirality are parsed and ignored.
   Unsupported constructs (for example aromatic `Se`, `Te`, `As`) are
   `SmartsError::Unsupported` with a position.
10. **Idle machine.** Load inflates timings about 1.5x. Check `uptime` first.
11. **Time verify with `screen_query` + `verify_screened`.**
    `verify_candidates` re-runs the census and measures a cost `search` does not
    pay.
12. **Perception changes.** Prove them neutral with the full-corpus hash check
    in [correctness.md](correctness.md); unit tests cannot see all molecules.
13. **`--query-index`** is a file record number, not an arena row.
14. **New census-exact classes.** Test with `verify_candidates` over every row:
    the screen can hide false positives on small corpora.
