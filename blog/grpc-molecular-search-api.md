---
description: A gRPC molecular search API serves Tanimoto and SMARTS searches as typed calls. Learn how to run mossflux on port 50051 and query it. Start with one database.
faq:
  - answer: A timed-out gRPC molecular search returns an error and never a partial list. The server default deadline is 10 s and the maximum is 60 s. The client sees a gRPC error status, so it can retry with a narrower query instead of trusting a truncated list.
    question: Does a gRPC molecular search return partial results when it times out?
  - answer: Neither a Python package nor a crates.io release exists for the client. Generate stubs from the .proto file with your own gRPC toolchain, and install the mossflux CLI with cargo install --git from the project repository. Prebuilt CLI binaries are not published either, so the build runs from source.
    question: Is there a Python package or a crates.io release for the client?
  - answer: The molecular search index needs about 2.5 KB of RAM per molecule. ChEMBL 37 (2,896,304 molecules) gives a 5.1 GB index that builds in 33 to 41 s. The index lives in memory and is not saved to disk, so every start rebuilds it.
    question: How much memory does the molecular search index need?
  - answer: "gRPC and REST return the same search results, because one engine serves all three transports: REST, MCP and gRPC. REST and MCP answer on port 8080 and gRPC on port 50051. The transports share the engine, so they can't disagree on a hit."
    question: Do gRPC and REST return the same search results?
image: /blog/grpc-molecular-search-api/featured-080edc0ff927.webp
image_alt: "Illustration for gRPC molecular search API: Setup and calls"
publishDate: "2026-11-23"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/grpc-molecular-search-api#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q26356541\",\"@type\":\"Thing\",\"description\":\"cross-platform binary RPC protocol\",\"name\":\"GRPC\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GRPC\",\"https://www.wikidata.org/wiki/Q26356541\",\"https://www.google.com/search?kgmid=/g/11cp5mklv8\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q165194\",\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]}],\"datePublished\":\"2026-11-23\",\"description\":\"A gRPC molecular search API serves Tanimoto and SMARTS searches as typed calls. Learn how to run mossflux on port 50051 and query it. Start with one database.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A timed-out gRPC molecular search returns an error and never a partial list. The server default deadline is 10 s and the maximum is 60 s. The client sees a gRPC error status, so it can retry with a narrower query instead of trusting a truncated list.\"},\"name\":\"Does a gRPC molecular search return partial results when it times out?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Neither a Python package nor a crates.io release exists for the client. Generate stubs from the .proto file with your own gRPC toolchain, and install the mossflux CLI with cargo install --git from the project repository. Prebuilt CLI binaries are not published either, so the build runs from source.\"},\"name\":\"Is there a Python package or a crates.io release for the client?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The molecular search index needs about 2.5 KB of RAM per molecule. ChEMBL 37 (2,896,304 molecules) gives a 5.1 GB index that builds in 33 to 41 s. The index lives in memory and is not saved to disk, so every start rebuilds it.\"},\"name\":\"How much memory does the molecular search index need?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"gRPC and REST return the same search results, because one engine serves all three transports: REST, MCP and gRPC. REST and MCP answer on port 8080 and gRPC on port 50051. The transports share the engine, so they can't disagree on a hit.\"},\"name\":\"Do gRPC and REST return the same search results?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q1645574\",\"@type\":\"Thing\",\"description\":\"data serialization format\",\"name\":\"Protocol Buffers\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protocol_Buffers\",\"https://www.wikidata.org/wiki/Q1645574\",\"https://www.google.com/search?kgmid=/m/04dzxdz\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q28865\",\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q580427\",\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q8777\",\"@type\":\"Thing\",\"description\":\"application-layer protocol for transferring web resources\",\"name\":\"HTTP\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HTTP\",\"https://www.wikidata.org/wiki/Q8777\",\"https://www.google.com/search?kgmid=/m/03hgt\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q364\",\"@type\":\"Organization\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q2984686\",\"@type\":\"Thing\",\"description\":\"package list of Python modules\",\"name\":\"Python Package Index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_Package_Index\",\"https://www.wikidata.org/wiki/Q2984686\",\"https://www.google.com/search?kgmid=/m/07kdx48\"]}],\"name\":\"gRPC molecular search API: Setup and calls\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/MDCIawH5CGbHJs2DQFdOpgqnUYNkvb1k.webp\"},\"url\":\"https://mossflux.com/blog/grpc-molecular-search-api\"},{\"@id\":\"https://mossflux.com/blog/grpc-molecular-search-api#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q26356541\"},{\"@id\":\"https://www.wikidata.org/wiki/Q165194\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"loruorg/mossflux on GitHub\",\"url\":\"https://github.com/loruorg/mossflux\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"fullstorydev/grpcurl\",\"url\":\"https://github.com/fullstorydev/grpcurl\"},{\"@type\":\"CreativeWork\",\"name\":\"Documentation - gRPC\",\"url\":\"https://grpc.io/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Status codes and their use in gRPC | gRPC\",\"url\":\"https://grpc.io/docs/guides/status-codes/\"},{\"@type\":\"CreativeWork\",\"name\":\"Security · mossflux docs\",\"url\":\"https://mossflux.com/docs/security\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"}],\"dateModified\":\"2026-11-23\",\"datePublished\":\"2026-11-23\",\"description\":\"A gRPC molecular search API serves Tanimoto and SMARTS searches as typed calls. Learn how to run mossflux on port 50051 and query it. Start with one database.\",\"headline\":\"gRPC molecular search API: Setup and calls\",\"image\":\"https://api.birchseek.com/m/MDCIawH5CGbHJs2DQFdOpgqnUYNkvb1k.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/grpc-molecular-search-api#webpage\"},\"keywords\":[\"grpc molecular search api\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/grpc-molecular-search-api#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q1645574\"},{\"@id\":\"https://www.wikidata.org/wiki/Q28865\"},{\"@id\":\"https://www.wikidata.org/wiki/Q580427\"},{\"@id\":\"https://www.wikidata.org/wiki/Q8777\"},{\"@id\":\"https://www.wikidata.org/wiki/Q364\"},{\"@id\":\"https://www.wikidata.org/wiki/Q2984686\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2326},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"contextual queries\",\"name\":\"Semantic search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Semantic_search\",\"https://www.wikidata.org/wiki/Q1891170\",\"https://www.google.com/search?kgmid=/m/02r4_2d\"]},{\"@type\":\"Thing\",\"description\":\"database designed to store vector embeddings and perform similarity search in high-dimensional spaces\",\"name\":\"Vector database\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Vector_database\",\"https://www.wikidata.org/wiki/Q118396004\",\"https://www.google.com/search?kgmid=/g/11kph5bck7\"]},{\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - grpc
  - molecular search
  - mossflux
  - tanimoto similarity
  - substructure search
  - cheminformatics
title: "gRPC molecular search API: Setup and calls"
---

**A grpc molecular search api puts chemical structure search behind a typed service contract: the [mossflux API docs](https://mossflux.com/docs/api) say `mossflux serve` opens gRPC on port 50051 with the `mossflux.v1.Search` service and four methods (`ListDatabases`, `Similarity`, `Substructure`, `Molecule`). According to the [mossflux docs](https://mossflux.com/docs/), it searches molecules by Tanimoto similarity and SMARTS substructure, and every result matches a brute-force scan. [mossflux is open source](https://github.com/loruorg/mossflux) and free, with no hosted service.**

This guide is for engineers who want a gRPC molecular search API inside their own services.

By the end you can start the server, get the contract, make a `Similarity` call with aspirin and a `Substructure` call with a sulfonamide, and handle the errors, deadlines and limits that come back.

> **Key Takeaways**
> - A search past its deadline (server default 10 s, maximum 60 s) returns an error, never a partial list.
> - Each request is capped at 10,000 hits, and 4 searches run concurrently by default; more wait.
> - The in-memory index needs about 2.5 KB of RAM per molecule, and `/ready` returns 503 until every database is loaded.

## What is a gRPC molecular search API, and what does mossflux expose?

A gRPC molecular search API answers chemical structure queries through typed remote calls, and the [mossflux API documentation](https://mossflux.com/docs/api) says `mossflux serve` exposes one as a gRPC service named `mossflux.v1.Search` on port 50051. Its four methods put four questions to a compound library:

![Illustration for What is a gRPC molecular search API, and what does mossflux expose?](/blog/grpc-molecular-search-api/image-1-f4640dd326f2.webp)

- `ListDatabases` asks which libraries are loaded. It returns their names, which you pass to the other methods.
- `Similarity` asks which molecules look like this one. It takes a query SMILES and returns molecules ranked by Tanimoto score, either above a threshold or the k nearest.
- `Substructure` asks which molecules contain this pattern. It takes a SMARTS, such as `S(=O)(=O)N` for a sulfonamide, and returns the matches.
- `Molecule` asks for the record of one molecule: its properties.

Tanimoto similarity is the number of bits two fingerprints share divided by the number of bits set in either.

You add one or more databases with `--db NAME=PATH`. The [CLI docs](https://mossflux.com/docs/cli) put it this way: a database is a served SMILES file, and the server computes its own 2048-bit ECFP4-style fingerprints. A similarity query is therefore just a SMILES. These bits are mossflux's own and aren't RDKit-compatible.

The gRPC section of the API documentation and the `.proto` file define the exact request and response fields. The same documentation says `mossflux serve` answers the same searches over REST, MCP and gRPC.

One process serves REST (JSON) and MCP on port 8080 and gRPC on port 50051, all over one engine. The three transports can't disagree, because there's only one implementation behind them.

The REST equivalents are the routes `databases`, `similarity`, `substructure` and `molecule`. On your own machine, a gRPC client reaches `localhost:50051`, and a browser or `curl` reaches port 8080 of the same process.

## How do you run a gRPC molecular search server?

A gRPC molecular search API server is one `mossflux serve` process that loads SMILES files into memory and answers on port 50051. Setup takes six steps.

1. Install the CLI from source. You need Rust 1.95 or later. The [CLI reference](https://mossflux.com/docs/cli) lists every `serve` flag.

2. Prepare one SMILES file per database. The server fingerprints each file itself when it loads, so you don't ship fingerprints.

3. Start the server with one or more databases, repeating the flag for each: `mossflux serve --db chembl=PATH --db inhouse=PATH`. Each name is what clients pass to select a database.

4. Use Docker if you prefer. The image is `loruorg/mossflux` on Docker Hub: x86-64, distroless, non-root. On arm64, build the image from the Dockerfile. Compose and Kubernetes manifests are in `deploy/`. Copy the `docker run` command and port mapping from the [deploy page](https://mossflux.com/docs/deploy).

5. Wait for readiness before connecting. Poll it first, for example with `curl -f http://localhost:8080/ready`, because gRPC calls made during loading have no database to search.

6. Size the memory before you start. The index is built in memory at every start and is not saved to disk, so each restart repeats the build. The deploy page gives these figures for ChEMBL 37 (2,896,304 molecules): build in 33 to 41 s, index 5.1 GB, and about 2.5 KB of RAM per molecule. Use the per-molecule figure for your own files, and add the sizes of all databases, since the server holds every one at once.

The server is up when `/ready` answers 200 and `ListDatabases` returns the names you passed to `--db`.

[Try the demo](https://mossflux.com/#demo) to run a search before you load your own files.

## How do you get the `.proto` contract for a gRPC molecular search API?

You get the mossflux `.proto` contract from the gRPC section of the API docs and from the repository, then generate the client with your own gRPC toolchain. Every stub derives from that file.

1. Open the published contract. Read the gRPC section of the [mossflux API documentation](https://mossflux.com/docs/api), which describes how `mossflux serve` answers searches over REST, MCP and gRPC. The source is in the [mossflux repository](https://github.com/loruorg/mossflux). Take the `.proto` file from those pages as they present it, and don't guess a path inside the repository.

2. Save the file in your project unchanged. It defines the `mossflux.v1.Search` service and its messages, so any edit puts your client out of step with the server.

3. Generate stubs for your language. The [gRPC documentation](https://grpc.io/docs/) site tells you to "Select a language or platform, then choose its Quick start." Run the code generator named in that quick start against the saved `.proto` file. You should get a client class with one method per RPC: `ListDatabases`, `Similarity`, `Substructure` and `Molecule`.

4. Point the client at your own server, the host where you run `mossflux serve`. Call `ListDatabases` first. It should return the database names you passed with `--db`.

5. Test from the shell if you like. The [grpcurl README](https://github.com/fullstorydev/grpcurl) describes the tool as "basically curl for gRPC servers." Give it the `.proto` file and a method name, and add transport flags only as your own server's setup requires.

mossflux ships the contract and the server, and nothing else on the client side. It has no Python bindings, no crates.io release and no prebuilt CLI binaries, so don't search PyPI or crates.io for a client package. The CLI installs from source with `cargo install --git https://github.com/loruorg/mossflux mossflux-cli`.

## How do you run a similarity and substructure search over gRPC?

You run both searches by calling `mossflux.v1.Search` on port 50051 with a request message that holds a query string and a database name. Both calls assume your server runs on `localhost:50051` without TLS, and that you add your copy of the contract to each `grpcurl` command with `-proto` and `-import-path`.

1. Send the similarity request, with aspirin as the query SMILES. The [API docs](https://mossflux.com/docs/api) list `smiles`, `threshold`, `k`, `limit` and `db` for similarity. A threshold returns every molecule at or above it, and `k` returns the k nearest. Match each name against the message in your `.proto` before you send, because `grpcurl` rejects a field the message doesn't define.

```sh
grpcurl -plaintext \
  -d '{"smiles": "CC(=O)Oc1ccccc1C(=O)O", "threshold": 0.7, "limit": 100, "db": "chembl"}' \
  localhost:50051 mossflux.v1.Search/Similarity
```

2. Read the similarity response. It's a list of hits, each holding a molecule identifier from the database and its exact Tanimoto score; the `.proto` gives the field names. Canonicalisation (salts, tautomers, stereo) is done before ingestion, so two different structures can both score 1.0.

3. Send the substructure request, with the sulfonamide SMARTS as the query. A plain SMILES is also a valid SMARTS, so `c1ccncc1` works as a query too.

```sh
grpcurl -plaintext \
  -d '{"smarts": "S(=O)(=O)N", "limit": 100, "db": "chembl"}' \
  localhost:50051 mossflux.v1.Search/Substructure
```

4. Check the deadline fields before you rely on them. The REST substructure request has `offset` and `timeout_ms`. Read the Substructure message in your `.proto` to see whether the gRPC request carries them, and leave them out of the call if it doesn't.

5. Read the substructure response. It lists the matching molecules, each with its database identifier. A substructure hit has no similarity score, because the match is exact: the hits are the rows a brute-force scan finds.

Each call returns a hit list or an error status that names the problem. Both calls take a database name, and it must be one you started with `--db NAME=PATH`; `ListDatabases` shows which names the server has loaded.

## What happens on timeouts and limits in a gRPC molecular search API?

A mossflux search that runs out of time, asks for too much or sends oversized input gets an error, and no answer is ever shortened or altered.

The [API documentation](https://mossflux.com/docs/api) sets the deadline: the server default is 10 s and the maximum is 60 s.

A deadline bounds the cost of a query you don't control.

Past the deadline the answer is an error, never a partial list. Every list a client receives is complete, so it matches a brute-force scan.

The [gRPC guide to status codes](https://grpc.io/docs/guides/status-codes/) explains what the client sees: gRPC reports a failed call through a status code, and the call returns no result message. Handle the status in your client's error path, as you would for any other failed call.

Each request is limited to 10,000 hits. A limit refuses, and it never changes an answer.

The server runs 4 searches at once by default. Further requests wait their turn.

The [security page](https://mossflux.com/docs/security) sets the input limits. SMILES can have up to 65,534 atoms and SMARTS up to 2,048 atoms.

Malformed or oversized input is refused with an error that carries a byte position or a line number. That position lets you fix a bad SMILES or SMARTS string without guessing which character failed.

The [correctness page](https://mossflux.com/docs/correctness) describes how the exact results are tested against a brute-force oracle.

## What does gRPC change about molecular search, and what does it not?

gRPC changes how a client talks to mossflux, and nothing about what the engine searches or how fast the search runs.

| Aspect | gRPC changes | gRPC leaves unchanged |
| --- | --- | --- |
| Client contract | Typed stubs from the `.proto` file | REST and MCP still served |
| Wire format | Protobuf messages | JSON over REST |
| Answers | Nothing | Same rows as REST and MCP |
| Index | Nothing | In memory, rebuilt on every start |
| Chemistry | Nothing | mossflux's own fingerprints, SMILES and SMARTS input |

The [API docs](https://mossflux.com/docs/api) state that `mossflux serve` answers the same searches over REST, MCP and gRPC. All three transports share one engine, so the answers match.

The published timings measure the search engine, not gRPC overhead, and mossflux makes no claim that gRPC is faster than REST.

The similarity timing is 2.64x to 2.77x faster than FPSim2 at 256 bits. The conditions: AMD EPYC 9645 VM, a 256-bit Morgan r=2 query, ChEMBL 37 (2.9M molecules), 1 mossflux thread against 1 FPSim2 worker, search only, median of 11 trials.

The substructure range is 5.2 to 160 million molecules a second per thread. The conditions: a 17-query battery, 2M ChEMBL rows, 8 threads, the same EPYC 9645 VM, median of 3 trials. No billion-scale substructure claim is made.

These timings predate the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on it. The [benchmarks page](https://mossflux.com/docs/benchmarks) holds the full tables with the machine, data and method of each run.

Scope limits stay as they were under REST:

- The index lives in memory and is rebuilt on every start. There's no on-disk index, no deletes and no sharding, and row ids are `u32`.
- Fingerprints are mossflux's own and aren't RDKit-compatible.
- Chirality is parsed and ignored. There's no salt stripping or tautomer handling.
- The Docker image is x86-64 only.

Pick gRPC for services that generate typed clients from the `.proto` contract. REST suits `curl` and JSON clients, and MCP suits agents that call chemistry search as a tool.

## Frequently asked questions

### Does a gRPC molecular search return partial results when it times out?

A timed-out search returns an error, never a partial list. The server default deadline is 10 s and the maximum is 60 s. The client sees a gRPC error status, so it can retry with a narrower query.

### Is there a Python package or a crates.io release for the client?

Neither exists. Prebuilt CLI binaries aren't published either, so the build runs from source.

### How much memory does the molecular search index need?

Plan for about 2.5 KB of RAM per molecule. ChEMBL 37 (2,896,304 molecules) gives a 5.1 GB index, built in 33 to 41 s. The index lives in memory and isn't saved to disk, so every start rebuilds it.

### Do gRPC and REST return the same search results?

They return the same results, because one engine serves all three transports: REST, MCP and gRPC. REST and MCP answer on port 8080 and gRPC on port 50051.

## What should you do next with your gRPC molecular search API?

Three steps take a gRPC molecular search API from a fresh install to a service you can size, ordered by effort.

1. Today, install the CLI or pull the Docker image, start `mossflux serve` with one `--db NAME=PATH`, and wait for `/ready` to return 200.
2. This week, generate stubs from the `.proto`, call `Similarity` and `Substructure` with your own SMILES and SMARTS, and set a deadline on every query you don't control.
3. Then read the [benchmarks](https://mossflux.com/docs/benchmarks) and [security limits](https://mossflux.com/docs/security) pages for the conditions behind every number.

The [mossflux docs home](https://mossflux.com/docs/) holds the full reference for all three transports.

[Run it on your library](https://mossflux.com/#get). Start with one database and one query; if the results don't fit your workflow, stop the process and nothing else on the machine has changed.

## Sources

[API · mossflux docs](https://mossflux.com/docs/api) · [mossflux docs](https://mossflux.com/docs/) · [loruorg/mossflux on GitHub](https://github.com/loruorg/mossflux) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [Documentation - gRPC](https://grpc.io/docs/) · [fullstorydev/grpcurl](https://github.com/fullstorydev/grpcurl) · [Status codes and their use in gRPC | gRPC](https://grpc.io/docs/guides/status-codes/) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks). Every other source is linked where the article cites it.
