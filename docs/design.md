# Design

mossflux has two exact search engines in one Rust workspace. No external
chemistry library is used: the SMILES and SMARTS parsers, perception, screen
and matcher are all in this repository.

| Crate | Holds |
| --- | --- |
| `mossflux-core` | Fingerprint packing, popcount, Tanimoto, BitBound bounds. `DEFAULT_NBITS = 2048`. |
| `mossflux-index` | Similarity: arena, FPS loader, threshold and kNN search, inverted bitmap, segments. |
| `mossflux-substruct` | Substructure: SMILES, SMARTS, perception, screen, census, VF2, brute oracle. |
| `mossflux-server` | REST, gRPC and MCP over one engine; SMILES databases. |
| `mossflux-cli` | The `mossflux` binary and benchmark harnesses. |

## Similarity engine

- **Packed `u64` fingerprints, autovectorised popcount.** The compiler emits
  native popcount (CNT, POPCNT, `vpopcntq`) with no `unsafe` code.
- **Immutable, popcount-sorted arena.** Rows with similar popcount are
  contiguous, so the BitBound band is one contiguous range.
- **BitBound pruning with a rational threshold.** The bound is `num/den` with
  the numerator rounded down, not a float, so rounding can never drop a true
  hit. At `t <= 0` there is no pruning and every row is returned.
- **Exact scores.** `Hit::score` is the f64 that the threshold compares. kNN
  ranks by exact fraction (cross-multiplication); ties go to lower popcount,
  then earlier input row.
- **Inverted bitmap.** Columns are bit-transposed; per-row counts are
  bit-sliced into planes and a query ripple-adds its set-bit columns 64 rows at
  a time, restricted to the band. Sequential, no scatter writes. Plane sweep is
  cache-blocked to stay in L2.
- **Early abandon, off by default.** Rarest-column-first partial sums. Exact,
  but the extra control flow costs more than it saves in RAM. Opt in with
  `bench --abandon on`. There is no query planner.
- **Segmented append.** `SegmentedIndex` holds independently popcount-sorted
  segments. A row's score depends only on its bits, so the union of segment
  hits equals one arena over the same rows. Appends need no rebuild.
- **Row store semantics.** Duplicate fingerprints under different ids are
  separate hits. `score == 1.0` means equal bits, not the same compound.
  Canonicalisation (salts, tautomers, stereo) is done before ingestion.

## Substructure engine

Pipeline: parse → perceive → screen → census → verify.

- **SMILES and SMARTS parsers.** SMARTS includes recursive `$(...)` (depth 10),
  `v<n>`, `X<n>`, `D`, `H`. Atom maps and chirality parse and are ignored.
  Every malformed input is an error with a byte position; out-of-range values
  are refused, not clamped.
- **Perception.** Rings and aromaticity with the project's own Huckel model.
  Linear-time for bridges and simple cycle blocks; incremental GF(2) cycle
  basis.
- **Path screen.** 1024-bit fingerprint of paths up to 7 atoms plus ring-closure
  fragments. A superset filter: it may pass non-matches, never drop a match.
  For OR, wildcards and recursion it degrades to "verify everything". Path
  enumeration has a budget per atom; past it the target sets every bit, so the
  screen stays a superset.
- **Census.** 32 per-molecule count classes (elements, bond types, degrees,
  ring N, and augmented atoms such as `C_DBL_O`, `AMIDE_C`, `ACID_C`, `ESTER_C`,
  `C_TRP_N`, `SULFONAMIDE_S`, `RING_RING_BOND`, `N_DEG3`).
  - Exact classes (count ≥ 1 if and only if a match exists) skip VF2.
  - Forward-sound classes (`PHENOL_C`) accept on the count and still verify the
    rest.
  - `FUSED_BENZENE_PAIR` counts 6-cycles by direct enumeration, not SSSR, so it
    may reject.
  - Bare elements are exact only for C, N, O, S (both aromaticities) and
    aliphatic F, Cl, Br, I. Others share a class and go to VF2.
- **Aromatic ring skip.** Pure aromatic ring queries are answered from the
  molecule's precomputed aromatic ring set.
- **VF2 verify.** Over a compact per-molecule blob (`MatchMol`: atoms, CSR
  adjacency, bond data) to cut cache misses. Disconnected queries match each
  component alone first.
- **Brute oracle.** `brute.rs` is an exhaustive monomorphism search, kept slow
  and obvious. Every fast path is tested against it.

## Limits and deadlines

A limit refuses input; it never changes an answer.

| Input | Limit |
| --- | --- |
| SMILES | 65,534 atoms, 1,024 ring closures, charge ±15, 2^26 perception steps |
| SMARTS | 2,048 atoms including `$(...)` bodies, 512 terms per expression, nesting 10, counts ≤ 254 |
| FPS | 65,472 bits, 1 MiB per line, `u32::MAX` records, UTF-8 ids, no count encodings |

Subgraph matching is NP-complete. `search_with_limits` takes a deadline; past
it the search returns `SearchError::Timeout` and no rows, never a partial list.

## Build and code choices

- **Release profile:** `lto = "fat"`, `codegen-units = 1` so the hot loops
  inline and vectorise. `overflow-checks = true`: the loops are memory-bound,
  so the check is free and a wrapped counter becomes a panic.
- **`target-cpu=native`** in `.cargo/config.toml` for native popcount. Set
  `RUSTFLAGS` for a portable binary.
- **No `unsafe`.** `#![forbid(unsafe_code)]` in all three library crates.
- **Dependencies:** `rayon`, `flate2`, `clap` (binary only).
- **Exact only.** No approximate search by default; it would drop true hits.

## Server

- One `Engine` holds the databases; REST (axum), gRPC (tonic) and MCP
  (JSON-RPC over HTTP and stdio) are thin layers over it, so they cannot
  disagree.
- A database is a SMILES file. Each row is parsed once; the graph feeds both
  the substructure index and a 2048-bit ECFP4-style fingerprint
  (`mossflux_substruct::fingerprint`). The bits are mossflux's own, with a
  fixed 64-bit hash, so they are the same on every platform.
- The build runs in chunks of 65,536 rows, so only one chunk of full graphs is
  in memory.
- Searches run on blocking threads behind a semaphore. Every substructure
  search has a deadline; past it the reply is an error, never a partial list.
- The HTTP port answers `/ready` 503 until every database is loaded, for
  orchestrator probes.

## Compared with Arthor

| Feature | Arthor | mossflux |
| --- | --- | --- |
| Substructure search, SMARTS | yes | yes, exact, recursive `$()` |
| Similarity search, Tanimoto | yes | yes, threshold and kNN, exact |
| Query by SMILES | yes | yes (own fingerprint) |
| Multiple databases | yes | yes (`--db NAME=PATH`, repeat) |
| REST API | yes | yes |
| Web UI | yes | demo site only |
| Prebuilt on-disk index | yes | no; built at start |
| Billion-scale sharding | yes | no |

mossflux also has gRPC and MCP endpoints, exact hit counts with paging, and a
deadline on every substructure search.

## Rejected ideas

Similarity:

- Explicit AVX-512 CSA / `vpternlogq` kernel: LLVM already emits it; 1.1x to
  1.4x at best, needs `unsafe`.
- Harley-Seal CSA tree, MLP unrolling: help AVX-512, regress AVX2 and NEON.
- CSR posting-list inverted index: random scatter writes made it 2.6x to 7.7x
  slower than the linear scan.
- Multi-resolution folded screens, per-bin sub-bitmaps: weak at low thresholds.
- DivideSkip or WAND on the bitmap: survivor verify is DRAM-bound at 10M+ rows.
- LSH: breaks exactness; the bitmap is already faster at t ≥ 0.6.
- GPU for single queries: dispatch latency exceeds query time.
- Single-query on-disk scan: IOPS-bound; only a batched tier could work.
- Heavy compression of segments: fingerprints barely compress.
- mmap for a cold tier: page faults are hidden blocking I/O.
- Size-only heuristic to enable early abandon: de-vectorised the scan and
  halved low-threshold throughput.
- Aggregate-B/s targets against Arthor (5.2x or 17x FPSim2): not comparable
  across machines, and over the in-RAM roofline (~4x).

Substructure:

- VF2++ cutting rules, query automorphism, match ordering, prefetch, hot/cold
  splits: they cut ALU work, but about 94% of verify time is cache misses.
- Twin and colour-refinement symmetry pruning: drops true matches.
- Per-molecule adjacency bitsets: neighbours are already hot; +240 B/molecule.
- Large subgraph-feature or scaffold indexes: redundant with the census or
  inexact.
- SSSR ring filters in the reject direction: SSSR is not invariant under
  non-induced monomorphism, so they drop matches.
- Automatic census class generation: matched propane to cyclopropane.
- `cD3` filter column: prunes nothing (hit rate 0.670 against 0.672).
- `P` or `OTHER` as census-exact: thousands of false positives at ChEMBL scale.
- Cross-molecule streaming arena: candidates too sparse to prefetch.
- Co-locating scalar fields, neighbour look-ahead: more memory traffic,
  slower.

## Not built yet

- On-disk `.seg` segment format. Everything runs in memory.
- Persisted index; `mossflux substruct` and `mossflux serve` rebuild at start.
- Write-ahead log, deletes, compaction.
- Sharding, NUMA pinning, `u64` row ids (`Hit::index` is `u32`).
- Batched NVMe tier.
- Cost-based query planner.
- Register-strip blocking of the plane sweep; batch SIMD across queries.
- `[cD3]` census-exact skip; O(n) answer for bare `*`.
- Stable library API.
