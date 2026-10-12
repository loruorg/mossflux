# Deploy

The image is `ghcr.io/zhom/mossflux:latest`. It holds only the `mossflux`
binary (distroless, non-root, user 65532). Give it a SMILES file; it builds the
index in memory at start and serves REST and MCP on 8080 and gRPC on 50051.

## Data

Any SMILES file works: `.smi` (`SMILES name`), TSV with a `smiles` or
`canonical_smiles` header, or `ID<TAB>SMILES`. Gzip is read directly. ChEMBL 37
(2.9M molecules, 279 MiB, CC BY-SA 3.0):

```sh
mkdir -p data && curl -fL -o data/chembl_37_chemreps.txt.gz \
  https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/releases/chembl_37/chembl_37_chemreps.txt.gz
```

## Docker

```sh
docker run -d --name mossflux -p 8080:8080 -p 50051:50051 \
  -v "$PWD/data:/data:ro" ghcr.io/zhom/mossflux:latest \
  serve --db chembl=/data/chembl_37_chemreps.txt.gz

curl localhost:8080/ready      # 503 while loading, then 200
```

### Docker Compose

[`deploy/compose.yaml`](../deploy/compose.yaml) downloads ChEMBL into a volume
on first start, then runs the server:

```sh
docker compose -f deploy/compose.yaml up -d
```

### Build the image

```sh
docker build -t mossflux .                          # x86-64-v3 (AVX2) or arm64
docker build --build-arg TARGET_CPU=native -t mossflux .   # this CPU only
```

## Kubernetes

[`deploy/k8s/mossflux.yaml`](../deploy/k8s/mossflux.yaml) has a PVC, an init
container that downloads ChEMBL once, a Deployment with probes, and a Service:

```sh
kubectl apply -f deploy/k8s/mossflux.yaml
kubectl port-forward svc/mossflux 8080:8080
```

- Readiness uses `/ready`, so no traffic arrives before the index is built.
- More replicas need a `ReadWriteMany` volume, or bake the data into an image.
- gRPC through an Ingress needs HTTP/2 to the backend (the Service port sets
  `appProtocol: kubernetes.io/h2c`).

## Configuration

Every flag has an environment variable.

| Env | Flag | Default | |
| --- | --- | --- | --- |
| `MOSSFLUX_DB` | `--db` | required | `NAME=PATH`, comma-separated for several |
| `MOSSFLUX_LISTEN` | `--listen` | `0.0.0.0:8080` | REST and MCP |
| `MOSSFLUX_GRPC` | `--grpc` | `0.0.0.0:50051` | `off` to disable |
| `MOSSFLUX_CORS_ORIGINS` | `--cors-origin` | any | Comma-separated |
| `MOSSFLUX_THREADS` | `--threads` | all cores | Build and search threads |
| `MOSSFLUX_MAX_MOLS` | `--max-mols` | all | Rows read per database |
| `MOSSFLUX_SMILES_COL`, `MOSSFLUX_ID_COL` | | auto | 1-based columns |
| `RUST_LOG` | | `info` | Log filter (stderr) |

Request limits: [api.md](api.md#limits).

## Sizing

Measured on ChEMBL 37 (2,896,304 molecules indexed, 1,515 skipped), Apple M4
Max, all cores, machine under load:

| | |
| --- | --- |
| Build at start | 33 to 41 s |
| Index memory (`memory_bytes`) | 5.1 GB |
| Process RSS after build | 7.2 GB |

Plan for about 2.5 KB of RAM per molecule. The index is not saved to disk; each
start rebuilds it.

## Production notes

- Put it behind a TLS proxy. The server speaks plain HTTP and h2c gRPC.
- Set `MOSSFLUX_CORS_ORIGINS` to your site's origin.
- `MOSSFLUX_MAX_CONCURRENT` bounds CPU use; extra requests wait.
- Subgraph matching is NP-complete; keep `MOSSFLUX_MAX_TIMEOUT` low on a public
  endpoint.
- SIGTERM stops the server gracefully.
