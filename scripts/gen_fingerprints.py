#!/usr/bin/env python3
"""Generate ECFP4 (Morgan radius 2, 2048-bit) fingerprints from ChEMBL SMILES.

Emits chemfp-compatible FPS format so the exact same fingerprints can be indexed
by `mossflux` and cross-checked against chemfp / RDKit during benchmarking.

The hex byte order matches `mossflux`'s `parse_hex_fp`: bit `b` is set iff byte
`b // 8` has bit `b % 8` set.

Requires RDKit:
    pip install rdkit

Usage:
    python scripts/gen_fingerprints.py data/chembl_37_chemreps.txt.gz data/chembl_37.fps [--limit N]
"""
from __future__ import annotations

import argparse
import gzip
import io
import sys

NBITS = 2048
RADIUS = 2


def open_maybe_gzip(path: str) -> io.TextIOBase:
    if path.endswith(".gz"):
        return io.TextIOWrapper(gzip.open(path, "rb"), encoding="utf-8")
    return open(path, "r", encoding="utf-8")


def fp_to_hex(bitvect, nbits: int) -> str:
    """RDKit ExplicitBitVect -> chemfp/mossflux byte order, lowercase hex."""
    arr = bytearray((nbits + 7) // 8)
    for bit in bitvect.GetOnBits():
        arr[bit >> 3] |= 1 << (bit & 7)
    return arr.hex()


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("input", help="ChEMBL chemreps file (.txt or .txt.gz)")
    ap.add_argument("output", help="output .fps file")
    ap.add_argument("--limit", type=int, default=None, help="only process the first N molecules")
    ap.add_argument("--nbits", type=int, default=NBITS)
    ap.add_argument("--radius", type=int, default=RADIUS)
    args = ap.parse_args()

    try:
        from rdkit import Chem
        from rdkit.Chem import rdFingerprintGenerator
    except ImportError:
        sys.exit("RDKit is required: pip install rdkit")

    gen = rdFingerprintGenerator.GetMorganGenerator(radius=args.radius, fpSize=args.nbits)

    n_ok = 0
    n_fail = 0
    with open_maybe_gzip(args.input) as fin, open(args.output, "w", encoding="utf-8") as fout:
        fout.write("#FPS1\n")
        fout.write(f"#num_bits={args.nbits}\n")
        fout.write(f"#type=RDKit-Morgan/1 radius={args.radius} fpSize={args.nbits}\n")
        fout.write("#software=RDKit gen_fingerprints.py\n")

        header = fin.readline()  # skip column header line
        if not header.lower().startswith("chembl_id"):
            # Not the expected header; rewind by processing this line too.
            fin = _prepend_line(fin, header)

        for line in fin:
            parts = line.rstrip("\n").split("\t")
            if len(parts) < 2:
                continue
            chembl_id, smiles = parts[0], parts[1]
            mol = Chem.MolFromSmiles(smiles)
            if mol is None:
                n_fail += 1
                continue
            fp = gen.GetFingerprint(mol)
            fout.write(f"{fp_to_hex(fp, args.nbits)}\t{chembl_id}\n")
            n_ok += 1
            if n_ok % 100_000 == 0:
                print(f"  {n_ok:,} fingerprints written", file=sys.stderr)
            if args.limit is not None and n_ok >= args.limit:
                break

    print(f"done: {n_ok:,} fingerprints, {n_fail:,} failed parses -> {args.output}", file=sys.stderr)
    return 0


def _prepend_line(fin, line):
    """Return an iterator that yields `line` first, then the rest of `fin`."""
    import itertools

    return itertools.chain([line], fin)


if __name__ == "__main__":
    raise SystemExit(main())
