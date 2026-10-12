# mossflux

Exact molecular search in Rust. Two engines: **similarity search** (Tanimoto
over binary fingerprints, threshold and kNN) and **substructure search**
(SMARTS). Both return exactly what a brute-force scan returns. The SMILES and
SMARTS parsers, ring and aromaticity perception, screen and subgraph matcher
are all written here, with no external chemistry library and no `unsafe` code.

## Features

- Threshold and kNN similarity search with exact f64 scores.
- BitBound pruning over a popcount-sorted arena; an inverted bitmap index.
- Incremental append in memory with no rebuild and no change to any answer.
- Fingerprints of any width up to 65,472 bits; folding to smaller widths.
- SMARTS search with recursive `$(...)`, a superset screen, an exact census
  layer and a VF2 matcher, all tested against a brute-force oracle.
- Safe on untrusted input: size limits are clean errors; searches can run
  under a deadline that returns an error, never a partial list.
- Loads chemfp FPS (plain or gzip) and SMILES files (`.smi`, TSV, ChEMBL
  chemreps).
- A server with REST, gRPC and MCP endpoints over SMILES databases, with its
  own ECFP4-style fingerprints, so a similarity query is just a SMILES.

Live demo on ChEMBL 37: [mossflux.com](https://mossflux.com/).

## Quick start

```sh
cargo build --release --all
./target/release/mossflux --help
```

The default build targets the host CPU. For a portable binary:

```sh
RUSTFLAGS="-C target-cpu=x86-64-v3" cargo build --release --all
```

Examples (real output, ChEMBL 37):

```
$ ./target/release/mossflux tanimoto 0f0f 0ff0
0.333333

$ ./target/release/mossflux info --fps data/chembl_37.fps.gz
loaded 2,897,819 fingerprints × 2048 bits in 4.40s (659071 mol/s)
fingerprints : 2,897,819
width        : 2048 bits (32 u64 lanes)
memory        : 707.5 MiB (fingerprints)
popcount      : min 1, median 51, mean 52.5, max 249

$ ./target/release/mossflux search --fps data/chembl_37.fps.gz \
    --query-id CHEMBL153534 --threshold 0.9 --limit 5
loaded 2,897,819 fingerprints × 2048 bits in 4.42s (656239 mol/s)
1 hits in 13.012 ms (showing 1)
  1.0000  CHEMBL153534
```

Substructure search over a SMILES file. Give one run all its queries; the
index is built in memory per run:

```sh
./target/release/mossflux substruct --smiles data/chembl_37_chemreps.txt.gz \
    --smarts 'c1ccc2ccccc2c1' --smarts '[NX3;!$(NC=O)]' --count --timeout 30
./target/release/mossflux substruct --smiles my.smi --smarts 'C(=O)[OH]' \
    --format tsv > hits.tsv
```

Subcommands: `search`, `substruct`, `info`, `tanimoto`, `bench`,
`substruct-bench`, `serve` and `mcp`.
Get the data with `./scripts/fetch_chembl.sh`; see
[docs/development.md](docs/development.md).

## Run as a service

```sh
docker run -p 8080:8080 -p 50051:50051 -v "$PWD/data:/data:ro" \
  ghcr.io/zhom/mossflux:latest serve --db chembl=/data/chembl_37_chemreps.txt.gz

curl -s 'localhost:8080/v1/substructure?smarts=c1ccc2[nH]ccc2c1&limit=5'
curl -s localhost:8080/v1/similarity -H 'content-type: application/json' \
  -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.6}'
```

REST and MCP (`/mcp`) on 8080, gRPC on 50051. Docker Compose and Kubernetes:
[docs/deploy.md](docs/deploy.md). Endpoints: [docs/api.md](docs/api.md).

## Results

ChEMBL 37 (2.9M compounds), exact on every query. Details and conditions:
[docs/benchmarks.md](docs/benchmarks.md).

- **Similarity vs FPSim2: 2.64x to 2.77x faster**, 256-bit Morgan r=2,
  AMD EPYC 9645 VM, 1 thread against 1 worker, search only, median of 11.
- **Similarity vs Arthor, per thread: 0.93 to 1.45 B molecules/s** on an
  AVX2 `znver3` build, synthetic 50M 256-bit rows, against Arthor's published
  0.9 to 1.2. Arthor's 96-thread aggregate is not claimed.
- **Substructure: 5.2 to 160 M molecules/s/thread** over a 17-query battery,
  2M ChEMBL, 8 threads, EPYC 9645 VM, median of 3.

## Documentation

| Document | Content |
| --- | --- |
| [docs/design.md](docs/design.md) | Architecture, design decisions, rejected ideas. |
| [docs/benchmarks.md](docs/benchmarks.md) | All results with conditions; Arthor comparison; how to reproduce. |
| [docs/correctness.md](docs/correctness.md) | Exactness invariants and test gates. |
| [docs/development.md](docs/development.md) | Build, test, datasets, traps. |
| [docs/api.md](docs/api.md) | Service API. |
| [docs/deploy.md](docs/deploy.md) | Docker and deployment. |
| [SECURITY.md](SECURITY.md) | Guarantees on untrusted input; reporting. |
| [CHANGELOG.md](CHANGELOG.md) | Releases. |

## Licence

`MIT OR Apache-2.0`, at your option. See [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).
