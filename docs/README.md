# Get started

mossflux finds molecules by Tanimoto similarity and by SMARTS substructure. Every answer is exactly what a brute-force scan returns.

## Try the demo

The demo server holds ChEMBL 37 (2,896,304 molecules). Search it in the
browser at [mossflux.com](https://mossflux.com/#demo), or send it a request:

```sh
curl -s https://api.mossflux.com/v1/similarity -H 'content-type: application/json' \
  -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.7,"limit":2}'
```

```json
{"hits":[{"id":"CHEMBL25","smiles":"CC(=O)Oc1ccccc1C(=O)O","score":1.0},
  {"id":"CHEMBL350343","smiles":"CC(=O)Oc1ccccc1C(=O)Oc1ccccc1C(=O)O","score":0.8529411764705882}],
 "count":4,"took_us":14007,"scanned":433325,"total":2896304}
```

`count` is exact; `limit` caps only the hit list.

## Serve your own library

Any SMILES file works, gzip included. This example uses ChEMBL 37 (279 MiB,
CC BY-SA 3.0):

```sh
mkdir -p data && curl -fL -o data/chembl_37_chemreps.txt.gz \
  https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/releases/chembl_37/chembl_37_chemreps.txt.gz

docker run -p 8080:8080 -p 50051:50051 -v "$PWD/data:/data:ro" \
  loruorg/mossflux serve --db chembl=/data/chembl_37_chemreps.txt.gz
```

The server builds the index in memory at start. `GET /ready` returns 503 until
the index is ready, then 200. Plan for about 2.5 KB of RAM per molecule.

```sh
curl -s localhost:8080/v1/similarity -H 'content-type: application/json' \
  -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.6}'
curl -sG localhost:8080/v1/substructure \
  --data-urlencode 'smarts=c1ccc2[nH]ccc2c1' -d limit=5
```

REST and MCP use port 8080, gRPC uses 50051. The image is for x86-64; on arm64,
[build the image](deploy.md#build-the-image). Endpoints are in [API](api.md);
Compose, Kubernetes and settings are in [Deploy](deploy.md).

## Install the CLI

Needs Rust 1.95 or later:

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

`target-cpu=native` turns on hardware popcount. The binary then runs only on
CPUs like the build machine.

Substructure search over a SMILES file:

```sh
mossflux substruct --smiles data/chembl_37_chemreps.txt.gz --smarts 'C(=O)[OH]' --count
```

Similarity search over a [chemfp FPS](https://chemfp.com/fps_format/) file.
ChEMBL publishes `chembl_37.fps.gz` (2048-bit Morgan, radius 2) in the same
directory as the SMILES file:

```sh
mossflux search --fps data/chembl_37.fps.gz --query-id CHEMBL25 --threshold 0.7
```

All subcommands and flags are in [CLI](cli.md).
