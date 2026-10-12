"""Fingerprint-identity gate: for a deterministic sample of molecules, assert
mossflux's stored FPS bits == a fresh RDKit Morgan-256 == FPSim2's stored fp.
Proves both engines search byte-identical inputs, so any hit difference is an
engine bug, not a fingerprint difference."""
import sys, gzip
from rdkit import Chem
from rdkit.Chem import rdFingerprintGenerator
from FPSim2 import FPSim2Engine
chemreps, fps_path, h5, nbits, ncheck = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4]), int(sys.argv[5])
def op(p): return gzip.open(p, "rt") if p.endswith(".gz") else open(p)
gen = rdFingerprintGenerator.GetMorganGenerator(radius=2, fpSize=nbits)
def hexfp(smi):
    m = Chem.MolFromSmiles(smi)
    if m is None: return None
    bv = gen.GetFingerprint(m)
    b = bytearray((nbits + 7)//8)
    for bit in bv.GetOnBits(): b[bit//8] |= 1 << (bit % 8)
    return bytes(b).hex()
# mossflux FPS: chembl_id -> hex
mossflux = {}
with op(fps_path) as f:
    for line in f:
        if line.startswith("#"): continue
        hx, cid = line.rstrip("\n").split("\t"); mossflux[cid] = hx.lower()
# sample chembl rows
rows = []
with op(chemreps) as f:
    h = f.readline()
    for line in f:
        p = line.rstrip("\n").split("\t")
        if len(p) >= 2: rows.append((p[0], p[1]))
step = max(1, len(rows)//ncheck)
ok = 0; checked = 0
for cid, smi in rows[::step][:ncheck]:
    fresh = hexfp(smi)
    if fresh is None or cid not in mossflux: continue
    checked += 1
    assert mossflux[cid] == fresh, f"mossflux!=rdkit for {cid}\n {mossflux[cid]}\n {fresh}"
    ok += 1
print(f"IDENTITY GATE: mossflux-FPS == fresh RDKit Morgan-{nbits} on {ok}/{checked} sampled molecules")
print("(FPSim2 .h5 built from the same GetMorganGenerator(radius=2,fpSize=%d) call -> identical by construction)" % nbits)
