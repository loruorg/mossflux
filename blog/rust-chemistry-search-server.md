---
description: What is a Rust chemistry search server? It answers molecular queries over a network, and mossflux returns exact Tanimoto and SMARTS hits. Learn to install it.
faq:
  - answer: mossflux is an open-source Rust engine and server for exact Tanimoto similarity search and SMARTS substructure search. Start it with mossflux serve --db NAME=PATH and query POST /v1/similarity or POST /v1/substructure. Every result matches a brute-force scan, and a timed-out search returns an error with no partial list.
    question: What is the rust chemistry search server?
  - answer: You should plan for about 2.5 KB of RAM per molecule, because the server builds the index in memory when it starts. ChEMBL 37 has 2,896,304 molecules and gave a 5.1 GB index, so size the machine before you start the server.
    question: How much RAM does it need for ChEMBL?
  - answer: "mossflux does not use RDKit fingerprints and computes its own 2048-bit ECFP4-style fingerprints, which don't match RDKit's bit for bit. Don't compare raw bit vectors across the two toolkits. A similarity query is just a SMILES, and the server fingerprints it for you."
    question: Does it use RDKit fingerprints?
  - answer: Yes, an AI agent can call mossflux through MCP, either over the /mcp path or over stdio with the command mossflux mcp. The tools are list_databases, similarity_search, substructure_search, molecule_properties and get_molecule, and they run on the same engine as the REST endpoints.
    question: Can an AI agent call it?
  - answer: "A substructure search that takes too long returns an error and no partial list of matches. The server's default deadline is 10 s and the maximum is 60 s. Subgraph matching is NP-complete, so a deadline keeps one slow query from holding the server."
    question: What happens when a substructure search takes too long?
image: /blog/rust-chemistry-search-server/featured-0ca2f3cff953.webp
image_alt: "Illustration for Rust chemistry search server: Exact answers"
publishDate: "2026-11-22"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/rust-chemistry-search-server#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"datePublished\":\"2026-11-22\",\"description\":\"What is a Rust chemistry search server? It answers molecular queries over a network, and mossflux returns exact Tanimoto and SMARTS hits. Learn to install it.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux is an open-source Rust engine and server for exact Tanimoto similarity search and SMARTS substructure search. Start it with mossflux serve --db NAME=PATH and query POST /v1/similarity or POST /v1/substructure. Every result matches a brute-force scan, and a timed-out search returns an error with no partial list.\"},\"name\":\"What is the rust chemistry search server?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You should plan for about 2.5 KB of RAM per molecule, because the server builds the index in memory when it starts. ChEMBL 37 has 2,896,304 molecules and gave a 5.1 GB index, so size the machine before you start the server.\"},\"name\":\"How much RAM does it need for ChEMBL?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux does not use RDKit fingerprints and computes its own 2048-bit ECFP4-style fingerprints, which don't match RDKit's bit for bit. Don't compare raw bit vectors across the two toolkits. A similarity query is just a SMILES, and the server fingerprints it for you.\"},\"name\":\"Does it use RDKit fingerprints?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Yes, an AI agent can call mossflux through MCP, either over the /mcp path or over stdio with the command mossflux mcp. The tools are list_databases, similarity_search, substructure_search, molecule_properties and get_molecule, and they run on the same engine as the REST endpoints.\"},\"name\":\"Can an AI agent call it?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A substructure search that takes too long returns an error and no partial list of matches. The server's default deadline is 10 s and the maximum is 60 s. Subgraph matching is NP-complete, so a deadline keeps one slow query from holding the server.\"},\"name\":\"What happens when a substructure search takes too long?\"}],\"name\":\"Rust chemistry search server: Exact answers\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/1QI9uaWDt-I_KQUBcHeVPV132pl6G5gU.webp\"},\"url\":\"https://mossflux.com/blog/rust-chemistry-search-server\"},{\"@id\":\"https://mossflux.com/blog/rust-chemistry-search-server#article\",\"@type\":\"BlogPosting\",\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/\"},{\"@type\":\"CreativeWork\",\"name\":\"Daylight Theory Manual: SMARTS\",\"url\":\"https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Security · mossflux docs\",\"url\":\"https://mossflux.com/docs/security\"},{\"@type\":\"CreativeWork\",\"name\":\"Changelog · mossflux docs\",\"url\":\"https://mossflux.com/docs/changelog\"}],\"dateModified\":\"2026-11-22\",\"datePublished\":\"2026-11-22\",\"description\":\"What is a Rust chemistry search server? It answers molecular queries over a network, and mossflux returns exact Tanimoto and SMARTS hits. Learn to install it.\",\"headline\":\"Rust chemistry search server: Exact answers\",\"image\":\"https://api.birchseek.com/m/1QI9uaWDt-I_KQUBcHeVPV132pl6G5gU.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/rust-chemistry-search-server#webpage\"},\"keywords\":[\"rust chemistry search server\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/rust-chemistry-search-server#webpage\"},\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2200},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"contextual queries\",\"name\":\"Semantic search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Semantic_search\",\"https://www.wikidata.org/wiki/Q1891170\",\"https://www.google.com/search?kgmid=/m/02r4_2d\"]},{\"@type\":\"Thing\",\"description\":\"database designed to store vector embeddings and perform similarity search in high-dimensional spaces\",\"name\":\"Vector database\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Vector_database\",\"https://www.wikidata.org/wiki/Q118396004\",\"https://www.google.com/search?kgmid=/g/11kph5bck7\"]},{\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - rust
  - cheminformatics
  - tanimoto similarity
  - substructure search
  - mcp
  - chembl
title: "Rust chemistry search server: Exact answers"
---

**A rust chemistry search server answers molecular queries over a network, and [mossflux](https://mossflux.com) is one: a Rust engine and server (`mossflux serve`) for exact Tanimoto similarity search and SMARTS substructure search. Every answer is exactly what a brute-force scan returns, as the [mossflux docs](https://mossflux.com/docs/) state, and one server answers the same searches over REST, MCP and gRPC. It uses no external chemistry library.**

You search ChEMBL, an in-house library or a make-on-demand space, and you want molecular search behind an API that returns every true hit. Or you build AI agents that need a chemistry tool over MCP. A Rust chemistry search server that's written for exact answers suits both jobs.

After reading, you can install the server, start it on your own SMILES file, send it similarity and substructure queries, estimate the RAM it needs and judge whether its answers meet your standard for correctness.

> **Key Takeaways**
> - A limit refuses; it never changes an answer, and a substructure search past its deadline returns an error, never a partial list.
> - The index lives in memory: about 2.5 KB of RAM per molecule. ChEMBL 37 (2,896,304 molecules) needs a 5.1 GB index.
> - The server's ECFP4-style fingerprints are mossflux's own and don't match RDKit bit for bit.
> - Every speed figure carries its machine, dataset, width, threads and trials, and no aggregate win over Arthor is claimed.

## What is a Rust chemistry search server, and what does exact mean?

In a Rust chemistry search server such as mossflux, exact means the server returns the same rows as a brute-force scan: no false negatives and no false positives. The [correctness page](https://mossflux.com/docs/correctness) covers the tests behind that rule.

![Illustration for What is a Rust chemistry search server, and what does exact mean?](/blog/rust-chemistry-search-server/image-1-4722c9e2ceab.webp)

`mossflux serve` runs three kinds of search. A threshold search returns every molecule whose Tanimoto score is at least t, and a kNN search returns the k nearest.

A substructure search takes a SMARTS pattern such as the sulfonamide `S(=O)(=O)N`. [The Daylight theory manual](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) describes SMARTS as a language for specifying substructural patterns in molecules.

Three rules keep the answers exact:

- A prune rejects a molecule only on a proven bound.
- A limit refuses a request and never changes an answer.
- A search that passes its deadline returns an error, never a partial list, as the [API documentation](https://mossflux.com/docs/api) states.

Substructure search runs in stages: parse, perceive, path screen, census, then VF2 verify. The [design page](https://mossflux.com/docs/design) describes the path screen as a superset filter that never drops a match. There's no approximate search, no LSH and no ANN, so this Rust chemistry search server never trades recall for speed.

The parsers, perception and matcher live in the project, with no external chemistry library and no `unsafe` code.

You can check this behaviour on [the public demo](https://mossflux.com/) at https://api.mossflux.com, which searches ChEMBL 37 (2.9 million molecules) on one small 4-thread server.

## How do you install and run a Rust chemistry search server?

You install a Rust chemistry search server by building mossflux from source with `cargo install --git`, then starting it with `mossflux serve --db NAME=PATH`.

1. Install the CLI from any directory:

   ```sh
   RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
   ```

 The `target-cpu=native` flag turns on hardware popcount. The cost is portability: the binary runs only on CPUs like the build machine, so build on the host that will serve.
2. Pick your databases. A database is a SMILES file.
3. Start the server, repeating `--db` for each database:

   ```sh
   mossflux serve --db chembl=chembl_37_chemreps.txt.gz --db inhouse=library.smi
   ```

 One process serves REST and MCP on port 8080 and gRPC on port 50051. The index is built in memory at start and isn't saved to disk, so every restart rebuilds it.
4. Check readiness by requesting `/health` and `/ready` on port 8080:

   ```sh
   curl -i http://localhost:8080/health
   curl -i http://localhost:8080/ready
   ```

 Point load balancers and Kubernetes probes at `/ready`, not `/health`, so no traffic arrives before the index exists.

The Docker image `loruorg/mossflux` is on Docker Hub for x86-64, distroless and non-root. On arm64, build the image from the Dockerfile.

Docker Compose and Kubernetes manifests sit in `deploy/`, and the [deploy documentation](https://mossflux.com/docs/deploy) covers configuration and sizing.

Once `/ready` answers 200, the server accepts REST queries on port 8080. To [run it on your library](https://mossflux.com/#get), start with a plain SMILES file.

## How do you call the REST API for similarity and substructure search?

You call the mossflux REST API by sending JSON with `curl` to `POST /v1/similarity` or `POST /v1/substructure` on port 8080. The [API documentation](https://mossflux.com/docs/api) lists the endpoints, their fields and the limits.

Both requests assume a server started with `mossflux serve --db chembl=chembl.smi`.

1. Send a similarity request with the query SMILES in `smiles` and the database name in `db`. The `threshold` and `limit` values are example choices:

```sh
curl -s http://localhost:8080/v1/similarity \
  -H 'content-type: application/json' \
  -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.7,"limit":50,"db":"chembl"}'
```

 This asks for aspirin's neighbours at Tanimoto 0.7 or higher, at most 50 of them. Send `k` to ask for the k nearest molecules instead.

 Each hit carries an exact `f64` score. `score == 1.0` means equal bits, not the same compound. Canonicalisation (salts, tautomers, stereo) is done before ingestion, so two different structures can share a fingerprint.

2. Expect a fixed order for kNN. kNN ranks by exact fraction. Equal scores go to the lower popcount, then to the earlier row, so the same query always returns the same order.

3. Send a substructure request with `smarts`, then use `limit`, `offset` and `timeout_ms` to page and bound the search:

```sh
curl -s http://localhost:8080/v1/substructure \
  -H 'content-type: application/json' \
  -d '{"smarts":"S(=O)(=O)N","limit":100,"offset":0,"timeout_ms":5000,"db":"chembl"}'
```

 This finds sulfonamides, 100 hits per page. Raise `offset` to 100 for the next page. A SMILES is a valid SMARTS, and atom maps and chirality parse and are ignored.

4. Set `timeout_ms` for any pattern you don't control, because such a pattern can run long.

Two other endpoints are useful when you're checking results. `POST /v1/molecule` returns formula, weight and counts for a SMILES, and `GET /v1/databases/{db}/molecules/{id}` fetches a stored molecule by id.

## How do REST, gRPC and MCP share one search engine?

REST, gRPC and MCP share one search engine because `mossflux serve` sends all three transports into the same code, so a REST answer, a gRPC answer and an MCP answer are the same rows. The [API documentation](https://mossflux.com/docs/api) describes it this way: "mossflux serve answers the same searches over REST, MCP and gRPC."

Each transport takes requests in its own format:

- REST: JSON over HTTP on port 8080, for example `POST /v1/similarity`.
- gRPC: port 50051, service `mossflux.v1.Search`, with four calls: `ListDatabases`, `Similarity`, `Substructure` and `Molecule`.
- MCP: `/mcp` on port 8080 (streamable HTTP), or stdio with `mossflux mcp`.

MCP is the Model Context Protocol, which lets an AI agent call a tool. mossflux exposes five: `list_databases`, `similarity_search`, `substructure_search`, `molecule_properties` and `get_molecule`.

To give Claude Code these tools, point it at the public demo endpoint, which searches ChEMBL 37:

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

For your own data, run `mossflux serve --db NAME=PATH` and use your server's `/mcp` URL in the same command.

[The mossflux docs](https://mossflux.com/docs/) state that exact search has no machine-learning component. The engine compares fingerprints and molecular graphs, and every answer is what a brute-force scan returns.

An agent calling `similarity_search` gets the same hits as a `curl` call to `/v1/similarity` with the same SMILES, threshold and database.

## What are the limits of a Rust chemistry search server?

A Rust chemistry search server such as mossflux has four operating limits (memory, startup, request size and concurrency), and each one refuses or delays a request without changing an answer.

Memory comes first, because it's what you'll size a Rust chemistry search server around. The [deploy documentation](https://mossflux.com/docs/deploy) gives the ChEMBL 37 case: 2,896,304 molecules, a build of 33 to 41 s, an index of 5.1 GB and about 2.5 KB of RAM per molecule.

If a library holds 10 million molecules, 2.5 KB each comes to roughly 25 GB.

Startup follows from memory. Every start rebuilds the index, and `/ready` holds traffic until it's done. 

Request size is bounded. SMILES may have up to 65,534 atoms and SMARTS up to 2,048 atoms.

A request returns at most 10,000 hits. The [security page](https://mossflux.com/docs/security) says malformed or oversized input returns an error with a byte position or line number.

Concurrency is capped: by default 4 searches run at once, and more wait their turn.

The default deadline is 10 s and the maximum is 60 s. You set it per request with `timeout_ms`:

```sh
curl -s localhost:8080/v1/substructure \
  -H 'Content-Type: application/json' \
  -d '{"smarts":"S(=O)(=O)N","timeout_ms":2000,"db":"chembl"}'
```

Past the deadline the answer is an error, never a partial list. A response that carries hits carries every hit the brute-force scan finds.

## How fast is this Rust chemistry search server, and under what conditions?

The FPSim2 comparison ran on an AMD EPYC 9645 VM. It used 256-bit Morgan r=2 fingerprints over ChEMBL 37 (2.9M molecules), one mossflux thread against one FPSim2 worker, search only, median of 11.

The per-thread rate comes from an AVX2 `znver3` build over 50M synthetic 256-bit rows. The 8-core run used EPYC native, ChEMBL 256-bit fingerprints and t = 0.85.

| Measurement | Result | Data |
| --- | --- | --- |
| Similarity vs FPSim2, 256-bit | 2.64x to 2.77x faster | ChEMBL 37, 2.9M |
| Similarity per thread, AVX2 | 0.93 to 1.45 billion molecules/s | Synthetic, 50M rows |
| Similarity, 8 cores, t = 0.85 | 21.4 billion molecules/s effective | ChEMBL, 256-bit |
| Substructure per thread | 5.2 to 160 million molecules/s | ChEMBL, 2M rows |

On the same ISA class, Arthor's published rate is 0.9 to 1.2 billion molecules a second per thread. The mossflux range is 0.93 to 1.45 billion.

The 256-bit figures are benchmark configurations.

The benchmarks page states two things it does not claim:

- No aggregate win over Arthor's 96-thread figure.
- No billion-scale substructure rate.

The timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on it.

## What doesn't this Rust chemistry search server do?

The mossflux server leaves out eight things, from RDKit-compatible fingerprints to a hosted service. Release 0.1.0 is the initial release, as the [changelog](https://mossflux.com/docs/changelog) records.

1. Its fingerprints are not RDKit's. The server computes its own 2048-bit ECFP4-style bits, according to the [API documentation](https://mossflux.com/docs/api), so compare scores from mossflux with scores from mossflux.
2. Stereochemistry, salts and tautomers are not handled.
3. The index is not saved to disk. Every start rebuilds it in memory, so plan for the build time at each restart.
4. To remove a molecule, edit the SMILES file and restart.
5. Search is exact only, with no approximate mode.
6. Queries are SMILES or SMARTS.
7. Install with `cargo install --git`, as the [CLI documentation](https://mossflux.com/docs/cli) shows.
8. The code is free and open source under `MIT OR Apache-2.0`, as the [mossflux home page](https://mossflux.com) states.

## Frequently asked questions

### What is the rust chemistry search server?

mossflux is an open-source Rust engine and server for exact Tanimoto similarity search and SMARTS substructure search. Start it with `mossflux serve --db NAME=PATH` and query `POST /v1/similarity` or `POST /v1/substructure`. Every result matches a brute-force scan, and a timed-out search returns an error, not a partial list.

### How much RAM does it need for ChEMBL?

Plan for about 2.5 KB of RAM per molecule, because the server builds the index in memory at start. ChEMBL 37 has 2,896,304 molecules and gave a 5.1 GB index, so size the machine before you start the server.

### Does it use RDKit fingerprints?

mossflux computes its own 2048-bit ECFP4-style fingerprints, which don't match RDKit bit for bit. Don't compare raw bit vectors across the two toolkits. A similarity query is just a SMILES, and the server fingerprints it for you.

### Can an AI agent call it?

An AI agent can call it through MCP, over `/mcp` or over stdio with `mossflux mcp`. The tools are `list_databases`, `similarity_search`, `substructure_search`, `molecule_properties` and `get_molecule`, and they run on the same engine as REST.

### What happens when a substructure search takes too long?

The search returns an error, never a partial list. The server's default deadline is 10 s and the maximum is 60 s. Subgraph matching is NP-complete, so a deadline keeps one slow query from holding the server.

## Next steps: run mossflux on your own library

Run mossflux on your own library in stages: clean and serve the data first, then size and bound it, then wire it in and check it against a brute-force scan.

First, run `cargo install --git` and `mossflux serve --db NAME=PATH`, and wait for `/ready` to return 200.

This week, size the host at about 2.5 KB of RAM per molecule. The [deploy documentation](https://mossflux.com/docs/deploy) covers sizing and configuration.

Later, wire your application to REST, gRPC or MCP using the [API documentation](https://mossflux.com/docs/api). Run a brute-force scan over your own data and compare its rows with mossflux's answers for the same queries; they should match exactly.

[Run it on your library](https://mossflux.com/#get) and keep it if your brute-force check agrees.

## Sources

[mossflux docs](https://mossflux.com/docs/) · [Design · mossflux docs](https://mossflux.com/docs/design) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [API · mossflux docs](https://mossflux.com/docs/api) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks) · [mossflux · exact similarity search at billions of molecules…](https://mossflux.com). Every other source is linked where the article cites it.
