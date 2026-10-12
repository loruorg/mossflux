# Security

## Reporting

Report a vulnerability through the GitHub security advisory form ("Security"
tab, "Report a vulnerability"), not in a public issue. Include the input (a
SMILES, SMARTS or FPS file is usually enough) and what you saw.

Two kinds of problem count:

- **Availability:** an input that panics, aborts, overflows the stack, or uses
  time or memory far out of proportion to its size, through any public entry
  point, within the limits below.
- **Integrity:** an input for which a search returns a different row set than
  the brute-force oracle (substructure) or an exact f64 Tanimoto scan
  (similarity).

## Guarantees

- All four library crates are `#![forbid(unsafe_code)]`. The engine crates
  depend only on `rayon` and `flate2`; the server adds `tokio`, `axum`,
  `tonic` and `serde`. CI runs `cargo audit`.
- Malformed or oversized input to the parsers, `SubstructIndex` and the FPS
  loader is a returned error with a byte position or line number. Limits are in
  [docs/design.md](docs/design.md#limits-and-deadlines) and pinned by
  `crates/mossflux-substruct/tests/hostile_input.rs`.
- Search time is not bounded by default: subgraph matching is NP-complete. A
  service that takes queries from others must set a deadline
  (`SubstructIndex::search_with_limits` or `mossflux substruct --timeout`).
  `mossflux serve` always sets one and caps it (`--max-timeout`).
  Past it the search returns `SearchError::Timeout` and no rows.
- Caller errors with no answer (a query of the wrong width, a row id out of
  range) panic; each is listed under `# Panics` in the rustdoc. Validate them
  at your request boundary.
