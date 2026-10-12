#!/usr/bin/env bash
# Substructure and similarity benchmark for a shared production host.
# Capped at 8 threads, nice -19, and a molecule limit that keeps RSS under 20 GB.
# Run from the repo root: bash scripts/bench_host.sh
#
# Prints measured per-thread throughput. Arthor reference numbers and their
# sources: docs/benchmarks.md.

set -euo pipefail
cd "$(dirname "$0")/.."

THREADS="${THREADS:-8}"          # <= 8 vCPU
SUB_LIMIT="${SUB_LIMIT:-2000000}" # ~4.3 GB index at ~2.2 KB/mol — well under 20 GB
SIM_FPS="${SIM_FPS:-data/chembl_37_256.fps}"
SUB_SMILES="${SUB_SMILES:-data/chembl_37_chemreps.txt.gz}"
NICE="nice -n 19"

echo "### host: $(hostname)  cpus: $(nproc)  mem: $(free -g | awk '/Mem:/{print $2"GB total, "$7"GB avail"}')"
echo "### caps: threads=$THREADS  sub_limit=$SUB_LIMIT  (peak RSS must stay < 20 GB)"
echo "### building native (AVX-512) ..."
RUSTFLAGS="-C target-cpu=native" $NICE cargo build --release -j "$THREADS" -p mossflux-cli >/dev/null 2>&1
BIN=./target/release/mossflux

echo
echo "==================== SIMILARITY (256-bit = Arthor width) ===================="
for T in 0.7 0.85; do
  echo "--- \$ $BIN bench --fps $SIM_FPS --threshold $T --threads $THREADS --inverted ---"
  $NICE "$BIN" bench --fps "$SIM_FPS" --threshold "$T" --threads "$THREADS" --inverted --repeat 5 \
    2>&1 | grep -iE "loaded|exactness|single-thread|inverted index|ms/query|scanned" | sed 's/^/  /'
done

echo
echo "==================== SUBSTRUCTURE ===================="
echo "--- \$ $BIN substruct-bench --smiles $SUB_SMILES --limit $SUB_LIMIT --threads $THREADS ---"
echo "### free before: $(free -g | awk '/Mem:/{print $7"GB avail"}')"
$NICE "$BIN" substruct-bench --smiles "$SUB_SMILES" --limit "$SUB_LIMIT" --threads "$THREADS" --repeat 3 \
  2>&1 | sed -n '/exact gate/p;/=== build/,/(\* =/p' | sed 's/^/  /'
echo "### free after:  $(free -g | awk '/Mem:/{print $7"GB avail"}')"
echo
echo "### Per-thread throughput = (molecules) / (per-query total seconds). Compare to"
echo "### Arthor published per-thread above. All exactness-gated (BitBound==f64 truth;"
echo "### substructure split-pipeline==brute oracle)."
