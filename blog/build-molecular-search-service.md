---
description: How do you build a molecular search service? Run mossflux serve on a SMILES file, wait for /ready, then query over REST, gRPC or MCP. Get exact hits. Try it.
faq:
  - answer: To build a molecular search service, install mossflux with cargo install, start mossflux serve with --db NAME=PATH pointing at a SMILES file, wait for GET /ready to return 200, then send POST /v1/similarity or POST /v1/substructure. One server answers REST and MCP on port 8080 and gRPC on 50051.
    question: How do I build a molecular search service?
  - answer: "mossflux does not do approximate search; it searches exactly and has no approximate mode, so there's no LSH or ANN. There is also no persisted on-disk index yet, so every start rebuilds the index in memory, as the design documentation describes."
    question: Does mossflux do approximate search?
  - answer: "mossflux can't read SDF files, and it doesn't strip salts or handle tautomers. It reads SMILES files and chemfp FPS files, and it doesn't accept SDF, InChI or 3D input. Chirality is parsed and ignored, so canonicalise your structures before ingestion."
    question: Can mossflux read SDF files or handle salts and tautomers?
  - answer: "mossflux is free and open source under MIT OR Apache-2.0 at release 0.1.0, and it has no Python package. There's also no hosted service and no prebuilt binaries, so the CLI installs from source with cargo install --git, as the CLI documentation covers."
    question: Is mossflux free, and does it have Python bindings?
publishDate: "2026-10-31"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/build-molecular-search-service#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\",\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]}],\"datePublished\":\"2026-10-31\",\"description\":\"How do you build a molecular search service? Run mossflux serve on a SMILES file, wait for /ready, then query over REST, gRPC or MCP. Get exact hits. Try it.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"To build a molecular search service, install mossflux with cargo install, start mossflux serve with --db NAME=PATH pointing at a SMILES file, wait for GET /ready to return 200, then send POST /v1/similarity or POST /v1/substructure. One server answers REST and MCP on port 8080 and gRPC on 50051.\"},\"name\":\"How do I build a molecular search service?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux does not do approximate search; it searches exactly and has no approximate mode, so there's no LSH or ANN. There is also no persisted on-disk index yet, so every start rebuilds the index in memory, as the design documentation describes.\"},\"name\":\"Does mossflux do approximate search?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux can't read SDF files, and it doesn't strip salts or handle tautomers. It reads SMILES files and chemfp FPS files, and it doesn't accept SDF, InChI or 3D input. Chirality is parsed and ignored, so canonicalise your structures before ingestion.\"},\"name\":\"Can mossflux read SDF files or handle salts and tautomers?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux is free and open source under MIT OR Apache-2.0 at release 0.1.0, and it has no Python package. There's also no hosted service and no prebuilt binaries, so the CLI installs from source with cargo install --git, as the CLI documentation covers.\"},\"name\":\"Is mossflux free, and does it have Python bindings?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q165194\",\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q749568\",\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q608751\",\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\",\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q6120337\",\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q26356541\",\"@type\":\"Thing\",\"description\":\"cross-platform binary RPC protocol\",\"name\":\"GRPC\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GRPC\",\"https://www.wikidata.org/wiki/Q26356541\",\"https://www.google.com/search?kgmid=/g/11cp5mklv8\"]}],\"name\":\"Build molecular search service: SMILES to API\",\"url\":\"https://mossflux.com/blog/build-molecular-search-service\"},{\"@id\":\"https://mossflux.com/blog/build-molecular-search-service#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Security · mossflux docs\",\"url\":\"https://mossflux.com/docs/security\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Model Context Protocol specification\",\"url\":\"https://modelcontextprotocol.io/specification/2025-03-26\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"}],\"dateModified\":\"2026-10-31\",\"datePublished\":\"2026-10-31\",\"description\":\"How do you build a molecular search service? Run mossflux serve on a SMILES file, wait for /ready, then query over REST, gRPC or MCP. Get exact hits. Try it.\",\"headline\":\"Build molecular search service: SMILES to API\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/build-molecular-search-service#webpage\"},\"keywords\":[\"build molecular search service\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/build-molecular-search-service#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q165194\"},{\"@id\":\"https://www.wikidata.org/wiki/Q749568\"},{\"@id\":\"https://www.wikidata.org/wiki/Q608751\"},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\"},{\"@id\":\"https://www.wikidata.org/wiki/Q6120337\"},{\"@id\":\"https://www.wikidata.org/wiki/Q26356541\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2171},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"statistical Markov model\",\"name\":\"Hidden Markov model\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hidden_Markov_model\",\"https://www.wikidata.org/wiki/Q176769\",\"https://www.google.com/search?kgmid=/m/0p8wy\"]},{\"@type\":\"Thing\",\"description\":\"specialized artificial neural networks that are designed for tasks whose inputs are graphs\",\"name\":\"Graph neural network\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Graph_neural_network\",\"https://www.wikidata.org/wiki/Q107750691\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"chemical compound\",\"name\":\"Histidine\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Histidine\",\"https://www.wikidata.org/wiki/Q485277\",\"https://www.google.com/search?kgmid=/m/0h1tz\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - molecular search
  - similarity search
  - substructure search
  - smiles
  - mcp
  - cheminformatics
title: "Build molecular search service: SMILES to API"
---

**To build molecular search service endpoints, load a SMILES file into `mossflux serve`, wait for `/ready` to return 200, then send a similarity or substructure query. Every result matches a brute-force scan, so the service misses no true hit and returns no false hit, as the [mossflux docs](https://mossflux.com/docs/) state. One server answers the same searches over REST, gRPC and MCP, so the three agree, according to the [API documentation](https://mossflux.com/docs/api).**

This job falls to cheminformatics engineers, platform engineers and AI-agent builders who need every true hit from a compound library behind an API. The library might be [ChEMBL 37 with 2,896,304 molecules](https://mossflux.com/docs/deploy) or an in-house SMILES file.

The [public demo](https://mossflux.com/) at https://api.mossflux.com searches ChEMBL 37 (2.9 million molecules) on one small 4-thread server over REST and MCP. You can run the same server on your own file, query it, size and deploy it, and know where it stops.

> **Key Takeaways**
> - The index is [built in memory at every start](https://mossflux.com/docs/deploy) and isn't saved to disk; plan about 2.5 KB of RAM per molecule.
> - Each request returns at most 10,000 hits, and [4 searches run concurrently](https://mossflux.com/docs/api) by default; extra searches wait.
> - Fingerprints are mossflux's own 2048-bit ECFP4-style bits, so they don't match RDKit bit for bit.
> - Agents get five MCP tools: `list_databases`, `similarity_search`, `substructure_search`, `molecule_properties`, `get_molecule`.

## What do similarity search and substructure search compute?

Similarity search returns every molecule whose Tanimoto score to the query fingerprint is at or above a threshold, or the k highest scores. Substructure search returns the molecules that contain a SMARTS query as a subgraph.

A fingerprint is a row of bits, and each bit records whether the molecule has some small feature. The server computes its own 2048-bit ECFP4-style fingerprints (the idea behind RDKit's Morgan fingerprints), so a similarity query is just a SMILES. The [CLI](https://mossflux.com/docs/cli) also loads chemfp FPS files.

Tanimoto is `T = shared set bits / bits set in either fingerprint`. If the query sets 40 bits, a candidate sets 50 and 30 are shared, then 60 bits are set in either, so T = 30 / 60 = 0.5.

A threshold search returns every molecule with T >= t. A kNN search returns the k nearest, with ties going to the lower popcount, then the earlier row. A score of `1.0` means equal bits, not the same compound.

For substructure search, the query is a SMARTS pattern, and a molecule matches when the pattern maps onto some part of its atom-and-bond graph. A SMILES is a valid SMARTS, so `CC(=O)O` works as a query. The Daylight theory manual and the OpenSMILES specification define the two notations.

Atom maps and chirality parse and are ignored.

Against aspirin, `CC(=O)Oc1ccccc1C(=O)O`, one pattern matches and one doesn't:

| SMARTS | Result | Reason |
|---|---|---|
| `c1ccccc1C(=O)O` | match | the benzene ring carries a carboxylic acid group |
| `S(=O)(=O)N` | no match | aspirin has no sulfur atom |

## How do you build molecular search service endpoints from a SMILES file?

To build a molecular search service from a SMILES file, install the mossflux CLI, start `mossflux serve` on the file, wait for `/ready`, and send JSON queries over REST. The [CLI documentation](https://mossflux.com/docs/cli) describes the binary this way: "The mossflux binary searches files from the shell and runs the server."

1. Install from source.

   ```sh
   RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
   ```

 The native build runs only on CPUs like the build machine, so build on the host that will serve.

2. Start the server. A database is a SMILES file; repeat `--db NAME=PATH` to serve more than one.

   ```sh
   mossflux serve --db chembl=chembl.smi
   ```

3. Check readiness. Run `curl -i localhost:8080/ready` and wait for a 200 before you send queries.

4. Send a similarity query. The fields are `smiles`, `threshold`, `k`, `limit` and `db`.

   ```sh
   curl -s localhost:8080/v1/similarity -H 'content-type: application/json' -d @sim.json
   ```

   ```json
   {"smiles": "CC(=O)Oc1ccccc1C(=O)O", "threshold": 0.7, "limit": 10, "db": "chembl"}
   ```

 The response lists molecules at or above the threshold with their Tanimoto scores. Use `k` instead of `threshold` to get the nearest k.

5. Send a substructure query to `POST /v1/substructure`. The fields are `smarts`, `limit`, `offset`, `timeout_ms` and `db`.

   ```json
   {"smarts": "S(=O)(=O)N", "limit": 10, "offset": 0, "timeout_ms": 5000, "db": "chembl"}
   ```

 Set `timeout_ms` for any query you don't control, and page through long result sets with `offset`.

6. Look up hits. `GET /v1/databases` lists the loaded databases. `GET /v1/databases/{db}/molecules/{id}` fetches one molecule by id. `POST /v1/molecule` takes a SMILES and returns its properties.

With aspirin as the query and a library in `chembl.smi`, step 4 returns aspirin itself at a score of 1.0 if its bits are present, plus every close analogue above 0.7. Step 5 returns the sulfonamides.

For one-off searches without a server, the CLI also has `search` and `substruct`. Counts are always exact, and `--limit` caps only the printed hits. To run these requests against a hosted copy first, [try the demo](https://mossflux.com/#demo).

## What does exact mean, and why does a slow query return an error?

In mossflux, exact means a search returns the same rows as a brute-force scan, with no false negatives and no false positives. A slow query returns an error because substructure matching runs under a deadline, and pruning rejects a molecule only on a proven bound.

A brute-force scan compares every molecule with the query and keeps the ones that qualify, and each speed-up must return those same rows. The [correctness documentation](https://mossflux.com/docs/correctness) describes what "exact" means in mossflux and the tests that prove it.

For similarity search, each speed-up keeps that guarantee:

- BitBound pruning works over a popcount-sorted arena, where fingerprints are ordered by their number of set bits. A molecule whose popcount is too far from the query's can't reach the threshold, so the scan skips it.

- The bound is a rational number rounded down, so rounding can't drop a true hit.
- The inverted bitmap index returns the same rows as the linear scan, byte for byte.
- Appending molecules to the segmented index in memory changes no answer.

The matcher is why deadlines exist. The [CLI documentation](https://mossflux.com/docs/cli) states: "Subgraph matching is NP-complete: a short query can take exponential time."

The server default deadline is 10 s and the maximum is 60 s, according to the [API documentation](https://mossflux.com/docs/api). Past it you get an error, never a partial list. A limit refuses a request and never changes an answer.

The evidence is a brute-force oracle, `brute.rs`, which every fast path is checked against. Substructure search is also compared with RDKit, and in that differential 114 of 114 queries agree.

## What inputs, limits and non-goals apply when you build a molecular search service?

The service accepts SMILES files and chemfp FPS files, enforces fixed size, hit and time limits, and leaves several jobs to you. Per the [mossflux security documentation](https://mossflux.com/docs/security), malformed or oversized input is an error with a byte position or line number.

The [API documentation](https://mossflux.com/docs/api) lists the accepted inputs and the limits per endpoint. 

Column detection is automatic or set with flags.

The hard limits are:

- SMILES up to 65,534 atoms, SMARTS up to 2,048 atoms.
- Hits: 10,000 per request. Concurrency: 4 searches at once by default. Extra searches wait.
- Deadline: set per substructure request with `timeout_ms`, within the server maximum.

The jobs mossflux leaves out:

- Approximate search: mossflux searches exactly only.
- Deletes and sharding: neither exists, and row ids are `u32`.
- Chemistry clean-up: you canonicalise salts, tautomers and stereo before ingestion. Chirality is parsed and ignored.
- Other formats: there's no SDF, InChI or 3D input.
- Toolkit fingerprints: the server's 2048-bit ECFP4-style bits are its own and are not bit-compatible with RDKit.

## How do you size and deploy a self-hosted molecular search service?

To size and deploy a self-hosted molecular search service, work out how much RAM the in-memory index needs and keep traffic away until the build finishes. The [mossflux design documentation](https://mossflux.com/docs/design) states that the index is built in memory at start and is not saved to disk.

1. Size the host. Multiply the per-molecule planning figure by your own molecule count, and leave headroom above it. Measure memory on a test load before you size production.
2. Pull the image. The [deploy documentation](https://mossflux.com/docs/deploy) says to use `loruorg/mossflux` from Docker Hub. It's x86-64 only, distroless and runs as a non-root user. On arm64, build the image from the Dockerfile instead.
3. Choose a manifest. The Docker Compose and Kubernetes manifests live in `deploy/`.
4. Point the Kubernetes readiness probe at `/ready`. `GET /ready` returns 503 until every database is loaded, then 200, so a pod that's rebuilding its index receives no traffic.
5. Check the start-up. On ChEMBL 37, `/ready` should answer 503 through the build, then 200. If you serve several databases with `--db NAME=PATH`, the probe stays at 503 until the last one has loaded.

Because nothing is written to disk, every restart pays the full build time again, whether it comes from a rollout, a node move or a crash.

## How do REST, gRPC and MCP return the same answer?

REST, gRPC and MCP return the same answer because one engine inside `mossflux serve` handles all three, so an HTTP client, a gRPC client and an AI agent get the same hits for the same query. The [mossflux API documentation](https://mossflux.com/docs/api) says the server "answers the same searches over REST, MCP and gRPC."

Each transport reaches that engine differently:

- REST sends JSON over HTTP on port 8080, for example `POST /v1/similarity`.
- gRPC listens on port 50051 as service `mossflux.v1.Search`, with the calls ListDatabases, Similarity, Substructure and Molecule.
- MCP shares port 8080 at `/mcp` as streamable HTTP, or runs over stdio with `mossflux mcp`.

The [Model Context Protocol specification](https://modelcontextprotocol.io/specification/2025-03-26) describes MCP as an open protocol that connects LLM applications to external data sources and tools. A tool is a named operation the model can call. mossflux exposes five: `list_databases`, `similarity_search`, `substructure_search`, `molecule_properties` and `get_molecule`.

To register the server in Claude Code, run:

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

That command points at the [public demo](https://mossflux.com/), which searches ChEMBL 37. To search your own file, swap in the `/mcp` address of your own server.

Search is exact and uses no machine learning, so an agent that calls `substructure_search` receives the complete hit list.

## How fast is exact molecular search, and under what conditions?

Against FPSim2, exact similarity search in mossflux was 2.64x to 2.77x faster on an AMD EPYC 9645 VM, with 256-bit Morgan r=2 fingerprints over ChEMBL 37 (2.9M molecules), one mossflux thread against one FPSim2 worker, search only, median of 11.

The [mossflux benchmark tables](https://mossflux.com/docs/benchmarks) list the conditions for every run.

Per thread, on Arthor's ISA class (an AVX2 `znver3` build), mossflux reached 0.93 to 1.45 billion molecules a second on synthetic 50M 256-bit rows, against Arthor's published 0.9 to 1.2 per thread.

On 8 cores, EPYC native, ChEMBL 256-bit rows at t = 0.85, the effective rate was 21.4 billion molecules a second.

Substructure search ran at 5.2 to 160 million molecules a second per thread over a 17-query battery on 2M ChEMBL rows, 8 threads, EPYC 9645 VM, median of 3.

| Test | mossflux | Reference |
| --- | --- | --- |
| Similarity, 1 thread, 256-bit | 2.64x to 2.77x faster | FPSim2, 1 worker |
| Similarity per thread, AVX2 `znver3` | 0.93 to 1.45 billion/s | Arthor, 0.9 to 1.2 billion/s |
| Similarity, 8 cores, t = 0.85 | 21.4 billion/s effective | None |
| Substructure per thread, 8 threads | 5.2 to 160 million/s | None |

No aggregate win over Arthor's 96-thread figure is claimed, and no billion-scale substructure rate is claimed.

The timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on it.

The similarity benchmarks use 256-bit fingerprints, while the server computes 2048-bit fingerprints, so a request to `mossflux serve` isn't covered by these rates.

## Frequently asked questions

### Does mossflux save its index to disk?

There's also no persisted on-disk index yet: every start rebuilds the index in memory, as the [design documentation](https://mossflux.com/docs/design) describes.

### Can mossflux read SDF files or handle salts and tautomers?

It doesn't strip salts or handle tautomers, and chirality is parsed and ignored, so canonicalise your structures before ingestion.

## Build molecular search service APIs on your own library

Start today: query the demo at https://api.mossflux.com, or install the CLI and run `mossflux substruct` on a SMILES file.

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

This week, serve your own database with `mossflux serve`, point your orchestrator's readiness probe at `/ready`, and set `timeout_ms` on substructure queries you don't control.

Later, register the MCP endpoint with an agent.

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

To build a molecular search service that returns every true hit, [run it on your library](https://mossflux.com/#get).

## Sources

[mossflux docs](https://mossflux.com/docs/) · [API · mossflux docs](https://mossflux.com/docs/api) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [mossflux · exact similarity search at billions of molecules…](https://mossflux.com/) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [Design · mossflux docs](https://mossflux.com/docs/design) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks). Every other source is linked where the article cites it.
