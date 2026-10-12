#!/usr/bin/env python3
"""FPSim2 head-to-head baseline (the leading C-backed Tanimoto search library).

Loads an FPSim2 HDF5 index fully into memory and times threshold search,
SEARCH-ONLY and warm. FPSim2 uses the same BitBound + popcount approach as mossflux,
so this is the closest apples-to-apples comparison to a production specialist tool.

SEARCH-ONLY is enforced honestly: the query fingerprints are pre-generated as
RDKit ExplicitBitVects (using the SAME Morgan params stored in the .h5) BEFORE the
timer, and `similarity()` is called with the precomputed ExplicitBitVect — so the
timed region excludes RDKit parse + Morgan generation. For transparency the script
ALSO reports the from-SMILES latency (fp-gen INSIDE the timer); the gap is the
per-query fp-gen cost. Compare mossflux against the SEARCH-ONLY number.

Requires:  ./.venv/bin/pip install FPSim2
Usage:
    ./.venv/bin/python scripts/bench_fpsim2.py data/chembl_37.h5 \
        data/chembl_37_chemreps.txt.gz --threshold 0.7 --queries 1000 --workers 1
"""
from __future__ import annotations

import argparse
import gzip
import sys
import time


def open_maybe_gzip(path: str):
    if path.endswith(".gz"):
        return gzip.open(path, "rt")
    return open(path, "r")


def read_smiles(path: str, n: int):
    # Uniform stride sample across the WHOLE corpus (not first-N) so the query
    # popcount distribution matches mossflux's molecule-uniform sampling -> fair
    # per-query throughput bridge. Deterministic (no RNG): evenly-spaced indices.
    allsmi = []
    with open_maybe_gzip(path) as fin:
        header = fin.readline()
        if not header.lower().startswith("chembl_id"):
            parts = header.rstrip("\n").split("\t")
            if len(parts) >= 2:
                allsmi.append(parts[1])
        for line in fin:
            parts = line.rstrip("\n").split("\t")
            if len(parts) >= 2:
                allsmi.append(parts[1])
    if n >= len(allsmi):
        return allsmi
    step = len(allsmi) / n
    return [allsmi[int(i * step)] for i in range(n)]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("h5", help="FPSim2 HDF5 index (e.g. data/chembl_37.h5)")
    ap.add_argument("chemreps", help="ChEMBL chemreps file for query SMILES")
    ap.add_argument("--threshold", type=float, default=0.7)
    ap.add_argument("--queries", type=int, default=100)
    ap.add_argument("--repeat", type=int, default=1)
    ap.add_argument("--workers", type=int, default=1)
    ap.add_argument(
        "--include-fpgen",
        action="store_true",
        help="ALSO time the from-SMILES path (fp-gen inside the timer) for comparison",
    )
    args = ap.parse_args()

    from FPSim2 import FPSim2Engine
    from rdkit import Chem
    from rdkit.Chem import rdFingerprintGenerator

    t0 = time.perf_counter()
    fpe = FPSim2Engine(args.h5)  # in_memory_fps=True by default
    load_s = time.perf_counter() - t0
    try:
        n = fpe.fps.shape[0]
    except Exception:
        n = -1
    # Match the DB's exact Morgan params so query fps are identical to the index.
    params = dict(getattr(fpe, "fp_params", {}) or {})
    radius = int(params.get("radius", 2))
    fp_size = int(params.get("fpSize", 256))
    gen = rdFingerprintGenerator.GetMorganGenerator(radius=radius, fpSize=fp_size)
    print(
        f"loaded FPSim2 index ({n:,} fps, Morgan r={radius} fpSize={fp_size}) "
        f"in {load_s:.1f}s",
        file=sys.stderr,
    )

    smis = read_smiles(args.chemreps, args.queries)
    # Pre-generate query ExplicitBitVects OUTSIDE the timer (search-only). Keep
    # only SMILES RDKit+FPSim2 can handle, so timing isn't polluted by errors.
    good_bv = []
    good_smi = []
    for s in smis:
        try:
            mol = Chem.MolFromSmiles(s)
            if mol is None:
                continue
            bv = gen.GetFingerprint(mol)
            _ = fpe.similarity(bv, args.threshold, n_workers=args.workers)  # warm + validate
            good_bv.append(bv)
            good_smi.append(s)
        except Exception:
            pass
    if not good_bv:
        print("no usable query SMILES", file=sys.stderr)
        return 1
    print(f"using {len(good_bv)} query molecules", file=sys.stderr)

    # --- SEARCH-ONLY: time similarity() over PRECOMPUTED query fingerprints. ---
    t1 = time.perf_counter()
    total_hits = 0
    for _ in range(args.repeat):
        for bv in good_bv:
            res = fpe.similarity(bv, args.threshold, n_workers=args.workers)
            total_hits += len(res)
    elapsed = time.perf_counter() - t1
    runs = len(good_bv) * args.repeat
    ms = elapsed * 1e3 / runs

    print(f"=== FPSim2 similarity (threshold {args.threshold}, n_workers={args.workers}) ===")
    print(f"  dataset          : {n:,} fingerprints")
    print(f"  mean latency     : {ms:.4f} ms/query   [SEARCH-ONLY, query fp pre-generated]")
    print(f"  throughput       : {runs / elapsed:.1f} queries/s")
    if n > 0:
        print(f"  per molecule     : {elapsed * 1e9 / (runs * n):.2f} ns/mol")
    print(f"  hits             : {total_hits // runs:,} avg/query")

    # --- Optional: from-SMILES path (fp-gen INSIDE the timer) to quantify fp-gen. ---
    if args.include_fpgen:
        t2 = time.perf_counter()
        for _ in range(args.repeat):
            for s in good_smi:
                _ = fpe.similarity(s, args.threshold, n_workers=args.workers)
        elapsed2 = time.perf_counter() - t2
        ms2 = elapsed2 * 1e3 / runs
        print(f"  mean latency     : {ms2:.4f} ms/query   [INCLUDES per-query fp-gen]")
        print(f"  fp-gen overhead  : {ms2 - ms:.4f} ms/query")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
