# mossflux

Exact molecular search in Rust. Two engines: **similarity search** (Tanimoto
over binary fingerprints, threshold and kNN) and **substructure search**
(SMARTS). Both return exactly what a brute-force scan returns. The SMILES and
SMARTS parsers, ring and aromaticity perception, screen and subgraph matcher
are all written here, with no external chemistry library and no `unsafe` code.

Docs: [mossflux.com/docs](https://mossflux.com/docs/). Live demo on ChEMBL 37:
[mossflux.com](https://mossflux.com/#demo).

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

## Quick start

Serve a SMILES file (REST and MCP on 8080, gRPC on 50051):

```sh
docker run -p 8080:8080 -p 50051:50051 -v "$PWD/data:/data:ro" \
  loruorg/mossflux serve --db chembl=/data/chembl_37_chemreps.txt.gz

curl -sG localhost:8080/v1/substructure --data-urlencode 'smarts=c1ccc2[nH]ccc2c1' -d limit=5
```

Or install the CLI:

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
mossflux substruct --smiles my.smi --smarts 'C(=O)[OH]' --format tsv > hits.tsv
```

More: [Get started](https://mossflux.com/docs/), [CLI](https://mossflux.com/docs/cli),
[API](https://mossflux.com/docs/api), [Deploy](https://mossflux.com/docs/deploy).

## Results

ChEMBL 37 (2.9M compounds), exact on every query. Conditions:
[Benchmarks](https://mossflux.com/docs/benchmarks).

- **Similarity vs FPSim2: 2.64x to 2.77x faster**, 256-bit Morgan r=2,
  AMD EPYC 9645 VM, 1 thread against 1 worker, search only, median of 11.
- **Similarity vs Arthor, per thread: 0.93 to 1.45 B molecules/s** on an
  AVX2 `znver3` build, synthetic 50M 256-bit rows, against Arthor's published
  0.9 to 1.2. Arthor's 96-thread aggregate is not claimed.
- **Substructure: 5.2 to 160 M molecules/s/thread** over a 17-query battery,
  2M ChEMBL, 8 threads, EPYC 9645 VM, median of 3.

## Contributing

Build, test and dataset notes: [Development](https://mossflux.com/docs/development).
Exactness rules: [Correctness](https://mossflux.com/docs/correctness). The docs
are the Markdown files in [`docs/`](docs/); the site build is in
[`scripts/site/`](scripts/site/). Vulnerabilities: [SECURITY.md](SECURITY.md).

## Licence

`MIT OR Apache-2.0`, at your option. See [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).
