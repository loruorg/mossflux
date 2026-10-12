"""Build a 256-bit Morgan FPSim2 .h5 from chemreps, identical params to mossflux's FPS
(RDKit GetMorganGenerator radius=2 fpSize=256). Sequential int mol-id = line position
among >=2-col rows (gaps where rdkit rejects, same rows mossflux rejects)."""
import sys, gzip
from FPSim2.io import create_db_file
src, out, nbits = sys.argv[1], sys.argv[2], int(sys.argv[3])
def op(p): return gzip.open(p, "rt") if p.endswith(".gz") else open(p)
def mols():
    f = op(src); h = f.readline()
    if not h.lower().startswith("chembl_id"):  # header absent -> first line is data
        p = h.rstrip("\n").split("\t")
        if len(p) >= 2: yield [p[1], 0]
    i = 0
    for line in f:
        p = line.rstrip("\n").split("\t")
        if len(p) < 2:
            i += 1; continue
        yield [p[1], i]; i += 1
create_db_file(mols(), out, mol_format="smiles",
               fp_type="Morgan", fp_params={"radius": 2, "fpSize": nbits})
print("built", out)
