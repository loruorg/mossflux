#!/usr/bin/env python3
"""RDKit similarity-search baseline on the SAME fingerprints mossflux searches.

Loads a chemfp FPS file into RDKit ExplicitBitVect objects, then times the
canonical naive Python similarity search:

    sims = DataStructs.BulkTanimotoSimilarity(query, fps)
    hits = [(i, s) for i, s in enumerate(sims) if s >= threshold]

This is the path mossflux must beat "astronomically". Fingerprint loading is
EXCLUDED from the timing (search-only); a warm-up query primes caches.

Requires RDKit:  ./.venv/bin/pip install rdkit
Usage:
    ./.venv/bin/python scripts/bench_rdkit.py data/chembl_37.fps.gz \
        --threshold 0.7 --queries 100 --limit 0
"""
from __future__ import annotations

import argparse
import gzip
import sys
import time

from rdkit import DataStructs


def open_maybe_gzip(path: str):
    if path.endswith(".gz"):
        return gzip.open(path, "rt")
    return open(path, "r")


# byte value -> list of set bit offsets within the byte (LSB = bit 0)
BYTE_BITS = [[j for j in range(8) if (v >> j) & 1] for v in range(256)]


def load_fps(path: str, limit: int):
    nbits = 0
    fps = []
    with open_maybe_gzip(path) as fin:
        for line in fin:
            if not line or line[0] == "#":
                if line.startswith("#num_bits="):
                    nbits = int(line[len("#num_bits="):].strip())
                continue
            tab = line.find("\t")
            if tab < 0:
                continue
            hexfp = line[:tab]
            raw = bytes.fromhex(hexfp)
            if nbits == 0:
                nbits = len(raw) * 8
            onbits = []
            for i, byte in enumerate(raw):
                if byte:
                    base = i * 8
                    onbits.extend(base + j for j in BYTE_BITS[byte])
            bv = DataStructs.ExplicitBitVect(nbits)
            bv.SetBitsFromList(onbits)
            fps.append(bv)
            if limit and len(fps) >= limit:
                break
            if len(fps) % 250_000 == 0:
                print(f"  loaded {len(fps):,}", file=sys.stderr)
    return fps, nbits


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("fps")
    ap.add_argument("--threshold", type=float, default=0.7)
    ap.add_argument("--queries", type=int, default=100)
    ap.add_argument("--repeat", type=int, default=1)
    ap.add_argument("--limit", type=int, default=0, help="cap fingerprints loaded (0 = all)")
    ap.add_argument("--naive", action="store_true",
                    help="per-pair Python loop (literal 'python slowness') instead of BulkTanimoto")
    args = ap.parse_args()

    t0 = time.perf_counter()
    fps, nbits = load_fps(args.fps, args.limit)
    load_s = time.perf_counter() - t0
    n = len(fps)
    print(f"loaded {n:,} fingerprints × {nbits} bits in {load_s:.1f}s", file=sys.stderr)
    if n == 0:
        return 1

    nq = min(args.queries, n)
    step = max(1, n // nq)
    queries = [fps[i * step] for i in range(nq)]
    thr = args.threshold

    sim = DataStructs.TanimotoSimilarity
    bulk = DataStructs.BulkTanimotoSimilarity

    def search(q):
        if args.naive:
            # Per-pair Python loop — the literal "python slowness" path.
            return sum(1 for fp in fps if sim(q, fp) >= thr)
        sims = bulk(q, fps)
        return sum(1 for s in sims if s >= thr)

    # Warm-up (also primes caches).
    warm = sum(search(q) for q in queries)

    # Timed search.
    t1 = time.perf_counter()
    total_hits = 0
    for _ in range(args.repeat):
        for q in queries:
            total_hits += search(q)
    elapsed = time.perf_counter() - t1

    runs = nq * args.repeat
    ms = elapsed * 1e3 / runs
    label = "per-pair Python loop" if args.naive else "BulkTanimotoSimilarity"
    print(f"=== RDKit {label} (1 thread) ===")
    print(f"  dataset          : {n:,} fingerprints")
    print(f"  threshold        : {thr}")
    print(f"  mean latency     : {ms:.4f} ms/query")
    print(f"  throughput       : {runs / elapsed:.1f} queries/s")
    print(f"  per molecule     : {elapsed * 1e9 / (runs * n):.2f} ns/mol")
    print(f"  hits (warm-up)   : {warm:,}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
