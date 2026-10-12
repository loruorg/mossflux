#!/usr/bin/env bash
# Fetch the ChEMBL 37 SMILES dataset into ./data/.
# 292,540,141 bytes compressed; 2,897,820 lines = 1 header + 2,897,819 compounds.
set -euo pipefail

DEST="$(cd "$(dirname "$0")/.." && pwd)/data"
mkdir -p "$DEST"

URL="https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/releases/chembl_37_chemreps.txt.gz"
OUT="$DEST/chembl_37_chemreps.txt.gz"

echo "Downloading $URL"
curl -L --fail --retry 3 -o "$OUT" "$URL"
echo "Saved $OUT"
echo "Columns (tab-separated): chembl_id  canonical_smiles  standard_inchi  standard_inchi_key"
echo "Decompress with: gunzip -k \"$OUT\""
