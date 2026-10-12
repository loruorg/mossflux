# AGENTS.md

Contract for a coding agent in this repository.

mossflux is a Rust workspace for exact molecular search: Tanimoto similarity
over fingerprints, and SMARTS substructure search. No external chemistry
library. Exactness comes before speed.

## Read

- [docs/development.md](docs/development.md): commands, CLI, datasets, traps.
- [docs/correctness.md](docs/correctness.md): the 13 invariants and the gates.
- [docs/design.md](docs/design.md): architecture, decisions, rejected ideas.
  Do not propose a rejected idea again without new evidence.
- [docs/benchmarks.md](docs/benchmarks.md): measured results.

## Which crate

| Change | Crate |
| --- | --- |
| Tanimoto, bit packing, folding, popcount bounds | `mossflux-core` |
| Similarity search, kNN, FPS, arena, inverted bitmap, segments | `mossflux-index` |
| SMILES, SMARTS, perception, screen, census, VF2 | `mossflux-substruct` |
| CLI flags, harnesses, output | `mossflux-cli` |

## Before you finish

```sh
cargo build --release --all
cargo test --release --all
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
```

All must pass. If `cargo fmt` gives a diff much larger than your change, stop.

## Hard rules

1. **Exact.** Both engines return exactly what brute force returns. A prune
   rejects only on a proven bound. The bitmap equals the linear scan. An append
   never changes an answer. The screen is a superset. A census skip needs a
   proof in both directions. SSSR ring filters never reject. A limit refuses;
   it never changes an answer. Full list: docs/correctness.md.
2. **Do not optimise `brute.rs`.** It is the oracle.
3. **Never weaken a test, assertion or gate** to make a change pass. That
   includes smaller corpora, deleted probes, equality loosened to subset, or an
   assertion moved behind a feature flag.
4. **No `unsafe`.** `#![forbid(unsafe_code)]` stays in the three library
   crates. Code that needs `unsafe` needs a human decision.
5. **Never quote a number you did not observe.** Every performance number
   states machine, ISA, dataset, width, threads, trials. Check `uptime` first;
   load inflates timings about 1.5x.
6. **`data/` is never committed.** Do not regenerate `chembl_37.fps.gz` or
   `chembl_37.h5`; they come from the EBI FTP.
7. **Census changes** need the armed `differential_fuzz` and `scale_exactness`
   gates. Perception or screen changes need the full-corpus hash check.
8. **Release profile** (`lto = "fat"`, `codegen-units = 1`,
   `overflow-checks = true`) stays as is.

If a speed-up needs a rule relaxed, stop and ask.
