# API

`mossflux serve` exposes one engine over three transports:

| Transport | Default address | Use |
| --- | --- | --- |
| REST (JSON) | `:8080` | Web apps, scripts, `curl` |
| MCP | `:8080/mcp` (HTTP), or `mossflux mcp` (stdio) | AI agents |
| gRPC | `:50051` | Services; schema in [`proto/mossflux.proto`](../crates/mossflux-server/proto/mossflux.proto) |

All three return the same answers. Counts are always exact; `limit` caps only
the hit list.

## Search semantics

- **Similarity**: Tanimoto over a 2048-bit ECFP4-style fingerprint computed by
  mossflux from the SMILES. The hit set equals a brute-force scan. These bits
  are mossflux's own and do not match RDKit or other toolkits.
- **Substructure**: SMARTS subgraph match (a SMILES is a valid SMARTS). The hit
  set equals a brute-force match over every molecule. Hits are in database row
  order.
- **Deadline**: every substructure search has one (default 10 s, maximum 60 s).
  Past it the answer is an error, never a partial list.

## REST

Requests take a JSON body (`POST`) or the same fields as query parameters
(`GET`). Errors are `{"error": "..."}` with status 400 (bad input), 404
(unknown database or id), 408 (deadline) or 503 (still loading).

### `GET /v1/databases`

```json
{"databases":[{"name":"chembl","molecules":2896304,"skipped":1515,
  "fingerprint":{"kind":"ecfp4","bits":2048},"build_ms":33254,
  "memory_bytes":5077756205,"ready":true}]}
```

### `POST /v1/similarity`

| Field | Default | |
| --- | --- | --- |
| `smiles` | required | Query molecule |
| `threshold` | 0.7 | Every molecule with Tanimoto ≥ threshold, in `(0, 1]` |
| `k` | | The `k` nearest molecules instead |
| `limit` | 100 | Hits returned |
| `db` | first | Database name |

```sh
curl -s localhost:8080/v1/similarity \
  -H 'content-type: application/json' \
  -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.5,"limit":3}'
```

```json
{"hits":[{"id":"CHEMBL25","smiles":"CC(=O)Oc1ccccc1C(=O)O","score":1.0}, ...],
 "count":14,"took_us":1699,"scanned":139065,"total":199946}
```

`scanned` is the number of rows whose score was computed; BitBound skipped the
rest. `took_us` is server time, fingerprinting included.

### `POST /v1/substructure`

| Field | Default | |
| --- | --- | --- |
| `smarts` | required | Pattern |
| `limit` | 100 | Hits returned |
| `offset` | 0 | Hits skipped, for paging |
| `timeout_ms` | 10000 | Deadline, capped by the server |
| `db` | first | Database name |

```sh
curl -s 'localhost:8080/v1/substructure?smarts=S(=O)(=O)N&limit=2'
```

```json
{"hits":[{"id":"CHEMBL501671","smiles":"..."}, ...],
 "count":15303,"took_us":138,"screened":15303,"total":199946}
```

`screened` is the number of candidates left by the screen and census for the
exact matcher. When it equals `count`, the census answered the query exactly.

### `POST /v1/molecule`

Formula, weight and counts of one SMILES.

```json
{"formula":"C9H8O4","weight":180.159,"atoms":13,"heavy_atoms":13,"bonds":13,
 "rings":1,"aromatic_atoms":6,"fingerprint_bits_set":29}
```

### Other

| Path | |
| --- | --- |
| `GET /v1/databases/{db}/molecules/{id}` | One stored molecule |
| `GET /health` | 200 when the process is up |
| `GET /ready` | 200 when every database is loaded, else 503 |

## MCP

Tools: `list_databases`, `similarity_search`, `substructure_search`,
`molecule_properties`, `get_molecule`. Arguments are the REST fields.

Over HTTP, point the client at `http://HOST:8080/mcp` (streamable HTTP, JSON
responses). Claude Code:

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

Over stdio, the client starts the process:

```json
{"mcpServers": {"mossflux": {
  "command": "mossflux",
  "args": ["mcp", "--db", "chembl=/data/chembl_37_chemreps.txt.gz"]}}}
```

## gRPC

Service `mossflux.v1.Search`: `ListDatabases`, `Similarity`, `Substructure`,
`Molecule`. Zero values mean "default" (`k = 0` is a threshold search,
`threshold = 0` is 0.7). Status codes: `INVALID_ARGUMENT`, `NOT_FOUND`,
`DEADLINE_EXCEEDED`, `UNAVAILABLE`.

```sh
grpcurl -plaintext -import-path crates/mossflux-server/proto -proto mossflux.proto \
  -d '{"smarts":"c1ccncc1","limit":1}' localhost:50051 mossflux.v1.Search/Substructure
```

## Limits

| Flag | Env | Default |
| --- | --- | --- |
| `--max-hits` | `MOSSFLUX_MAX_HITS` | 10,000 per request |
| `--timeout` | `MOSSFLUX_TIMEOUT` | 10 s |
| `--max-timeout` | `MOSSFLUX_MAX_TIMEOUT` | 60 s |
| `--max-concurrent` | `MOSSFLUX_MAX_CONCURRENT` | 4 searches at once; more wait |

Queries are limited to 4,096 bytes and request bodies to 64 KiB. The parser
limits for SMILES and SMARTS are in [design.md](design.md#limits-and-deadlines).
