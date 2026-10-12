# Benchmarks

Every number has its conditions. A number without them is not a result.
All runs are exact: each fast path is checked against a brute-force or f64
reference on every query of the run, and the run fails on any difference.

Timings below were taken before the 2026-10-03 hardening pass (f64 scores,
exact-fraction kNN heap, VF2 deadline counter). The exactness gates pass on the
new code; the timings were not re-measured on it.

## Machines

| Tag | Machine | ISA |
| --- | --- | --- |
| M4 | Apple M4 Max, 16 cores | arm64, NEON |
| EPYC | AMD EPYC 9645 (Zen 5), 12-vCPU cloud VM, runs capped at 8 vCPU, `nice -19` | x86-64, AVX-512 with `vpopcntq` |
| Arthor (reference, not available) | 2x AMD EPYC 7443 (Zen 3), 48 cores / 96 threads, 1 TB DDR4-3200 | x86-64, AVX2, no `vpopcntq` |

Builds use `-C target-cpu=native` unless marked `znver3`
(`RUSTFLAGS="-C target-cpu=znver3"`: AVX2, no AVX-512, Arthor's ISA class).

Dataset: ChEMBL 37, 2,897,819 compounds. Query sets: 1000 seeded,
molecule-uniform random rows (seed `0xB1054ED`), zero-popcount queries dropped.

## Similarity

### Against RDKit and FPSim2 (M4, 2048-bit)

M4, ChEMBL 37 2.9M, RDKit Morgan r=2, 2048-bit, t = 0.7, search only, warm.
RDKit full-dataset values are extrapolated from 1M (Bulk) and 100K (naive)
subsets. FPSim2 loads `chembl_37.h5`, the same fingerprints.

| Engine | ns / molecule | ms / query |
| --- | ---: | ---: |
| RDKit per-pair Python loop, 1 thread | 1613 | ~4700 |
| RDKit `BulkTanimotoSimilarity`, 1 thread | 183 | ~530 |
| FPSim2, 1 worker | 12.44 | 36.1 |
| FPSim2, `n_workers=16` | - | 14.1 |
| mossflux full scan (raw kernel), 1 thread | 7.04 | - |
| mossflux BitBound, 1 thread | 3.66 | 10.6 |
| mossflux BitBound, 16 threads | 0.87 | 2.5 |

FPSim2 `n_workers` splits one query over threads; the mossflux 16-thread figure
is across-query throughput. Other M4 workloads, same conditions: t = 0.9 is
4.4 ms (1 thread) and 1.4 ms (16 threads); kNN k = 1000 is 32.3 ms and 15.5 ms.

A re-run on 2026-08-29 under load average 8 to 57 did not reproduce these
latencies (1.1x to 3.1x slower, with a 2.4x spread on identical runs), while
all load-independent counts did reproduce. Re-measure on an idle machine before
quoting the M4 latencies.

### Linear scan and inverted bitmap (EPYC, 2048-bit)

EPYC, AVX-512 native, ChEMBL 37 2.9M, 2048-bit, search only, median of 11
trials, bitmap equal to linear on all 1000 queries.

| Case | FPSim2 | mossflux linear | mossflux bitmap |
| --- | ---: | ---: | ---: |
| t = 0.7, 1 thread | 22.12 ms | 14.29 ms | 2.37 ms |
| t = 0.7, 4 threads | 9.61 ms | 4.82 ms | 0.64 ms |
| t = 0.9, 1 thread | - | 5.00 ms | 0.76 ms |
| t = 0.9, 4 threads | - | 1.61 ms | 0.21 ms |

The FPSim2 column is from a separate run on the same box (mossflux linear was
13.67 ms and 4.48 ms in that run). Raw kernel: 7.58 ns per 2048-bit Tanimoto,
33.8 GB/s against a 34.3 GB/s STREAM-Triad ceiling per core, so the linear scan
is bandwidth-bound and AVX-512 does not help it.

### FPSim2 at 256 bits (EPYC)

EPYC, `nice -19 taskset -c 0-7`, native 256-bit Morgan r=2 for both tools,
ChEMBL 37 2.9M, 1 mossflux thread against 1 FPSim2 worker, median of 11 trials,
search only (FPSim2 query fingerprints built before the timer). mossflux gated
on all 1000 queries against f64 truth.

| t | mossflux | FPSim2 | ratio |
| --- | ---: | ---: | ---: |
| 0.70 | 1.91 ms | 5.04 ms | 2.64x |
| 0.80 | 1.31 ms | 3.63 ms | 2.77x |
| 0.85 | 1.03 ms | 2.75 ms | 2.68x |
| 0.90 | 0.66 ms | 1.79 ms | 2.71x |

The two tools sample different, distribution-matched query sets. No cross-tool
hit-set equality gate was run.

### Bytes read per molecule (EPYC, 256-bit)

EPYC, ChEMBL 37 2.9M, ECFP folded 2048 to 256 bits (`--fold 256 --inverted`),
1000 queries, exact on every query. Bytes are counted in code (`words_read()`),
so they do not depend on the machine. Arthor scores every molecule, so its
value is 1.0x.

| t | band only | with early abandon | 1 thread | 8 threads |
| --- | ---: | ---: | ---: | ---: |
| 0.70 | 0.684x | 0.339x | 2.02 ms | 0.293 ms |
| 0.80 | 0.479x | 0.169x | 1.45 ms | 0.202 ms |
| 0.85 | 0.359x | 0.100x | 1.11 ms | 0.154 ms |
| 0.90 | 0.240x | 0.048x | 0.75 ms | 0.103 ms |

An M4 re-run over the same rows gave equal or lower values. Fewer bytes give
less time only when the scan is bandwidth-bound (billion scale or on disk).

BitBound band at 2048 bits, ChEMBL 37 (mean popcount 52.5): 63.29% scanned
(1.6x prune) at t = 0.7, 21.40% (4.7x) at t = 0.9.

### Per-thread rate on Arthor's ISA

Synthetic 50M rows, 256-bit, density 0.18, 1 thread, exact, N-invariant from
10M to 100M. Billions of molecules scored per second per thread.

| t | native (AVX-512) | znver3 (AVX2) | Arthor, published |
| --- | ---: | ---: | ---: |
| 0.5 (band ~100%) | 1.01 | 0.93 | 0.9 - 1.2 |
| 0.7 | 1.03 | 0.99 | 0.9 - 1.2 |
| 0.85 | 2.04 | 1.45 | 0.9 - 1.2 |

At 60M rows, t = 0.5, znver3 falls to 0.8 as the working set leaves cache.

### Multi-core scaling

Effective B/s = N / per-query time, exact on every query.

| Machine, data | Case | 1c → 4c → 8c (B/s) | 8c efficiency |
| --- | --- | --- | ---: |
| EPYC native, ChEMBL 256-bit 2.9M | t = 0.70 | 1.4 → 5.6 → 11.2 | 99% |
| EPYC native, ChEMBL 256-bit 2.9M | t = 0.85 | 2.6 → 9.9 → 21.4 | ~100% |
| EPYC znver3, ChEMBL 256-bit 2.9M | t = 0.70 | 1.3 → 5.2 → 9.3 | 87% |
| EPYC znver3, ChEMBL 256-bit 2.9M | t = 0.85 | 2.7 → 10.1 → 18.7 | 86% |
| EPYC native, synthetic 60M | t = 0.5 | 1.0 → 3.5 → 6.4 | 82% |
| EPYC znver3, synthetic 60M | t = 0.5 | 0.8 → 2.9 → 4.9 | 81% |
| M4, synthetic 256-bit 2M | t = 0.5 | 1.30 → 5.09 → 9.53 | 92% |
| M4, synthetic 256-bit 2M | t = 0.85 | 1.96 → 7.44 → 14.18 | 91% |
| M4, synthetic 256-bit 20M | t = 0.5 | 1.22 → 4.65 → 8.50 | 87% |
| M4, synthetic 256-bit 20M | t = 0.85 | 1.80 → 6.98 → 12.57 | 87% |

## Substructure

### Build and memory

M4, 8 threads, first 2,000,000 rows of `chembl_37_chemreps.txt.gz`
(observed 2026-08-29): 1,999,013 molecules indexed (987 rows do not parse),
build 22.757 s (87,841 molecules/s), index 2535.6 MiB = 1,330 B/molecule, of
which the screen is 256.1 B/molecule. Bytes per molecule grow with the slice:
the first 200,000 rows give 1,264 B/molecule.

### Query battery (EPYC)

EPYC, 8 threads, 2,000,000 ChEMBL 37 rows, median of 3 trials. Per-thread rate
= 2,000,000 / total seconds / 8. Each run checks the pipeline against the
brute-force oracle on all probes.

| Query | SMARTS | Candidates | Verify ms | Total ms | M mol/s/thread |
| --- | --- | ---: | ---: | ---: | ---: |
| sulfonamide | `S(=O)(=O)N` | 196,687 | 20.03 | 21.49 | 160 * |
| fluorine | `F` | 463,703 | 0.05 | 3.12 | 80.1 |
| any ring bond | `[R]~[R]` | 1,981,934 | 0.29 | 3.54 | 70.5 |
| nitrile | `C#N` | 122,935 | 13.74 | 17.85 | 54.5 * |
| ester | `C(=O)OC` | 237,834 | 0.04 | 6.15 | 40.6 |
| carbonyl | `C=O` | 1,397,499 | 0.23 | 6.48 | 38.6 |
| amide | `C(=O)N` | 1,104,927 | 0.18 | 6.84 | 36.5 |
| chlorine | `Cl` | 378,951 | 0.04 | 7.13 | 35.0 |
| aromatic N | `[#7;R]` | 1,630,222 | 0.23 | 7.37 | 33.9 |
| carboxylic acid | `C(=O)[OH]` | 201,964 | 0.04 | 8.29 | 30.1 |
| branched C | `[CD3]` | 1,724,568 | 0.29 | 9.72 | 25.7 |
| quaternary C | `[CD4]` | 522,377 | 0.10 | 11.16 | 22.4 |
| any halogen | `[F,Cl,Br,I]` | 827,030 | 0.16 | 11.62 | 21.5 |
| phenol | `c1ccc(O)cc1` | 756,625 | 16.08 | 18.83 | 13.3 |
| naphthalene | `c1ccc2ccccc2c1` | 54,368 | 16.13 | 25.49 | 9.8 |
| benzene | `c1ccccc1` | 1,705,452 | 31.82 | 38.85 | 6.4 |
| pyridine | `c1ccncc1` | 587,846 | 46.21 | 48.38 | 5.2 |

\* Re-measured after the `SULFONAMIDE_S` and `C_TRP_N` census classes landed;
the other columns of these two rows are from before. Range: 5.2 to 160 M
molecules/s/thread. On the same box, the build took 37.6 s at 1,120
B/molecule, before later census classes grew memory to the 1,330 above.

Candidate counts have since moved slightly with screen and census changes
(pyridine +2.37%, nitrile -6.92%, others under 0.5%); answers did not.
The current battery has 21 probes: 14 are answered by the census without VF2.

## Comparison with Arthor

Arthor figures are vendor-published and not reproduced here.

Similarity. Arthor's 30 to 50 B/s is an aggregate over 96 SMT threads on
2x EPYC 7443, Enamine REAL 2023 (4.29B), 256-bit ECFP4, 19 queries. Its
per-thread rate is 0.9 to 1.2 B/s.

- May claim: per thread, on Arthor's ISA (znver3), mossflux scores 0.93 to
  0.99 B/s in the score-everything regime and 1.45 where the band prunes.
- May not claim: any aggregate win. The 96-thread, two-socket box and the
  Enamine dataset are not available, and aggregates across different machines
  are not comparable. Caveats: znver3 code runs on Zen 5 silicon, the synthetic
  density is not Enamine's, and Enamine's tighter popcounts shrink the band.
- Arthor's 409 GB/s is theoretical; no measured bandwidth is published.

Substructure. No public Arthor source gives a per-thread rate at billion scale
with named hardware. The one like-for-like point is benzene, full exact scan,
single thread: Arthor about 4.0 M/s (Intel i7-6900K, ChEMBL22 1.68M, 2017);
mossflux 6.4 to 7.4 M/s/thread (EPYC VM, ChEMBL 37 2M). Different CPU and era.
No billion-scale substructure claim is made.

Sources:

- Sayle and Mayfield, Sheffield 2023 (similarity rates, hardware, dataset):
  https://nextmovesoftware.com/talks/Sayle_FngerprntsProcessingJustTheBitsYouNeedAndNoneOfThe1sYouDont_Sheffield_20230621.pdf
- Mayfield, DOCK 2023 (same hardware, ~10 B/s linear):
  https://nextmovesoftware.com/talks/Mayfield_ManagingAndSearchingOneTrillionCompounds_DOCK_230812.pdf
- "Searching ChEMBL in the browser", 2017 (substructure, single thread):
  https://nextmovesoftware.com/blog/2017/01/05/searching-chembl-in-the-browser/
- Sayle, ICCS 2018 (indole, about 2.3 M/s on one CPU):
  https://www.nextmovesoftware.com/talks/Sayle_EvolutionVsRevolution_ICCS_201805.pdf
- ZINC20, Irwin et al. 2020 (aggregate, 8-machine cluster):
  https://pmc.ncbi.nlm.nih.gov/articles/PMC8284596/

## Open method gaps

- No frozen query set shared by mossflux, FPSim2 and RDKit.
- No cross-tool hit-set equality gate.
- No paired-bootstrap confidence intervals.

## Reproduce

Use an idle machine: a load average near 32 inflated every timing about 1.5x
while hit counts stayed identical. Name the ISA with every timing.

```sh
cargo build --release --all

# Similarity, 2048-bit, with brute-force gate
./target/release/mossflux bench --fps data/chembl_37.fps.gz \
  --threshold 0.7 --queries 1000 --repeat 11 --brute
./target/release/mossflux bench --fps data/chembl_37.fps.gz \
  --knn 1000 --queries 1000 --repeat 11

# Inverted bitmap, gated against the linear scan
./target/release/mossflux bench --fps data/chembl_37.fps.gz \
  --threshold 0.7 --queries 1000 --repeat 11 --threads 1 --inverted

# 256-bit fold, bytes per molecule
./target/release/mossflux bench --fps data/chembl_37.fps.gz --fold 256 \
  --threshold 0.85 --queries 1000 --repeat 11 --threads 1 --inverted

# FPSim2 at 256 bits, native and znver3 builds (needs a Python venv)
bash scripts/reproduce_arthor_compare.sh

# Substructure battery (startup exactness gate)
./target/release/mossflux substruct-bench \
  --smiles data/chembl_37_chemreps.txt.gz --limit 2000000 --threads 8 --repeat 3
```

RDKit and FPSim2 rows: `scripts/bench_rdkit.py` and `scripts/bench_fpsim2.py`.
EPYC substructure run: `scripts/bench_host.sh`.
