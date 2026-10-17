# CLI

The `mossflux` binary searches files from the shell and runs the server. Results go to stdout, diagnostics to stderr.

| Subcommand | Does |
| --- | --- |
| `search` | Similarity search of an FPS file for one query |
| `substruct` | SMARTS search of a SMILES file |
| `serve` | REST, gRPC and MCP server ([Deploy](deploy.md)) |
| `mcp` | MCP server over stdio ([API](api.md#mcp)) |
| `info` | Statistics of an FPS file |
| `tanimoto` | Similarity of two hex fingerprints |
| `bench` | Similarity throughput ([Benchmarks](benchmarks.md#reproduce)) |
| `substruct-bench` | Substructure build time, memory and latency |

Exit code 0 is success (also on a closed pipe), 1 is an error, 2 is a usage
error. `mossflux <subcommand> --help` shows every flag.

## Similarity: `search`

```sh
mossflux search --fps data/chembl_37.fps.gz --query-id CHEMBL153534 --threshold 0.9 --limit 5
```

```text
loaded 2,897,819 fingerprints × 2048 bits in 4.42s (656239 mol/s)
1 hits in 13.012 ms (showing 1)
  1.0000  CHEMBL153534
```

| Flag | Default | Description |
| --- | --- | --- |
| `-f`, `--fps` | required | FPS file, plain or gzip |
| `--query-id` | | Query by id: the first record with that id |
| `--query-index` | | Query by record number: the 0-based position in the file |
| `--query` | | Query by hex fingerprint, at the file's width |
| `-t`, `--threshold` | 0.7 | Return every row with Tanimoto ≥ t |
| `-k`, `--knn` | | Return the k nearest rows instead. Ties go to the lower popcount, then the earlier record |
| `-l`, `--limit` | 20 | Hits printed. The count is always exact |
| `--format` | `text` | `text` or `tsv` |

Give one query flag. The file is loaded into memory on each run.

## Substructure: `substruct`

```sh
mossflux substruct --smiles data/chembl_37_chemreps.txt.gz \
  --smarts 'c1ccc2ccccc2c1' --smarts '[NX3;!$(NC=O)]' --count --timeout 30
mossflux substruct --smiles my.smi --smarts 'C(=O)[OH]' --format tsv > hits.tsv
```

Each run builds the index in memory, so give one run all its queries.

| Flag | Default | Description |
| --- | --- | --- |
| `-s`, `--smiles` | required | SMILES file, plain or gzip |
| `-q`, `--smarts` | | Query. Repeat the flag for more queries |
| `--smarts-file` | | One query per line. Blank lines and lines that start with `#` are skipped |
| `-l`, `--limit` | 20 | Hits printed per query. The count is always exact |
| `--count` | | Print only the count of each query |
| `--timeout` | none | Seconds per query. Past it: an error and no hits |
| `--strict` | | Fail on a row that does not parse. Default: skip the row and report it |
| `--max-mols` | all | Rows read from the file |
| `--threads` | 0 (all cores) | Threads for the build and the verify stage |
| `--smiles-col`, `--id-col` | auto | 1-based columns. `--id-col 0` uses the line number as the id |
| `--format` | `text` | `text` or `tsv` |

Subgraph matching is NP-complete: a short query can take exponential time. Set
`--timeout` for queries that you do not control.

## Input files

A SMILES file is split on tabs, or on spaces if a line has no tab. Without
`--smiles-col`, the columns are found in this order:

1. A `.smi`, `.smiles`, `.ism` or `.can` file (also `.gz`): SMILES, then name.
2. A first line with a `smiles` or `canonical_smiles` column is a header. The
   first other column is the id. ChEMBL chemreps files use this layout.
3. One column is a bare SMILES. Two or more columns are `ID SMILES`.

A row with no id uses its line number.

An FPS file is the [chemfp FPS format](https://chemfp.com/fps_format/): an
optional `#num_bits=` header, then `<hex>\t<id>` per line. Widths up to 65,472
bits are accepted.

## Other subcommands

`info` loads an FPS file and prints its statistics:

```sh
mossflux info --fps data/chembl_37.fps.gz
```

```text
loaded 2,897,819 fingerprints × 2048 bits in 4.40s (659071 mol/s)
fingerprints : 2,897,819
width        : 2048 bits (32 u64 lanes)
memory        : 707.5 MiB (fingerprints)
popcount      : min 1, median 51, mean 52.5, max 249
```

`tanimoto` compares two hex fingerprints:

```sh
mossflux tanimoto 0f0f 0ff0
```

```text
0.333333
```

`serve` and `mcp` take the flags in [Deploy](deploy.md#configuration).
`bench` and `substruct-bench` are in [Benchmarks](benchmarks.md#reproduce).
