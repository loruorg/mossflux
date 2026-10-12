# Changelog

## 0.1.0 — initial release

- Exact Tanimoto similarity search (threshold and kNN) with BitBound pruning,
  an inverted bitmap index and incremental append in memory.
- Exact SMARTS substructure search: SMILES and SMARTS parsers (recursive
  `$(...)`), ring and aromaticity perception, superset screen, census skip
  layer and VF2, tested against a brute-force oracle and RDKit.
- Input limits and search deadlines for untrusted input; no `unsafe` code.
- `mossflux` CLI: `search`, `substruct`, `info`, `tanimoto`, `bench`,
  `substruct-bench`.
