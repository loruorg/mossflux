#!/usr/bin/env bash
# Reproduce the mossflux-vs-Arthor-class + mossflux-vs-FPSim2 comparison end to end.
#
# Every headline number in docs/benchmarks.md is produced by the commands below. The
# only inputs are the public ChEMBL chemreps file and a Python venv with RDKit +
# FPSim2. Run from the repo root. Heavy steps are marked; a beefy box is assumed.
#
# Prereqs:
#   - data/chembl_37_chemreps.txt.gz   (EBI ChEMBL-37 chemical representations)
#   - .venv with: pip install rdkit FPSim2
#   - rustc/cargo (target-cpu=native build via .cargo or RUSTFLAGS)
set -euo pipefail
cd "$(dirname "$0")/.."
PY=.venv/bin/python
CHEMREPS=data/chembl_37_chemreps.txt.gz
NBITS=256

# 1) Native 256-bit Morgan FPS for mossflux (the search input). [~5-10 min]
$PY scripts/gen_fingerprints.py "$CHEMREPS" data/chembl_37_256.fps --nbits $NBITS --radius 2

# 2) 256-bit FPSim2 .h5 from the SAME Morgan params. [~5-10 min, multiprocess]
$PY scripts/build_fpsim2_256.py "$CHEMREPS" data/chembl_37_256.h5 $NBITS

# 3) Fingerprint-identity gate: mossflux-FPS bytes == fresh RDKit Morgan-256 on a
#    deterministic sample; FPSim2 .h5 is built from the same generator call, so it
#    is identical by construction (it is NOT directly hit-set-compared here).
$PY scripts/fp_identity_gate.py "$CHEMREPS" data/chembl_37_256.fps data/chembl_37_256.h5 $NBITS 200

# 4) mossflux native (AVX-512) — search-only, exact-gated (headline f64 gate + bitmap gate).
cargo build --release
for t in 0.7 0.8 0.85 0.9; do
  nice -n 19 taskset -c 0-7 target/release/mossflux bench --fps data/chembl_37_256.fps \
    --threshold "$t" --queries 1000 --repeat 11 --threads 1 --inverted
done

# 5) mossflux on ARTHOR-CLASS ISA (Zen3 AVX2, NO AVX-512 vpternlogq/vpopcntq). This is
#    the apples-to-apples build for the per-thread-vs-Arthor claim. Separate target dir.
CARGO_TARGET_DIR=target-znver3 RUSTFLAGS="-C target-cpu=znver3" cargo build --release
for t in 0.5 0.7 0.85; do
  nice -n 19 taskset -c 0-7 target-znver3/release/mossflux bench --fps data/chembl_37_256.fps \
    --threshold "$t" --queries 1000 --repeat 11 --threads 1 --inverted
done

# 6) FPSim2 SEARCH-ONLY (query fps pre-generated) + the from-SMILES path for the
#    fp-gen overhead, same uniform-stride 1000-query set.
for t in 0.7 0.8 0.85 0.9; do
  nice -n 19 taskset -c 0-7 $PY scripts/bench_fpsim2.py data/chembl_37_256.h5 "$CHEMREPS" \
    --threshold "$t" --queries 1000 --repeat 5 --workers 1 --include-fpgen
done

# 7) At-scale per-thread B/s + N-invariance (synthetic, distinct rows). Effective
#    B/s = N / per-query-ms. Compare to Arthor's published 0.9-1.2 B/s/thread.
for B in target/release/mossflux target-znver3/release/mossflux; do
  for N in 10000000 25000000 60000000 100000000; do
    nice -n 19 taskset -c 0-7 $B bench --synth-n "$N" --synth-nbits $NBITS \
      --synth-density 0.18 --threshold 0.5 --queries 60 --repeat 3 --threads 1 --inverted
  done
done

echo "Done. Effective B/s/thread = N / (single-thread median ms). See docs/benchmarks.md."
