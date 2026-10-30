---
description: What is a self hosted molecular search tool? It runs Tanimoto and SMARTS queries on your own SMILES file. Learn to install mossflux, size RAM, connect agents.
faq:
  - answer: "A self-hosted molecular search tool is a program you run on your own machine or cluster that searches your compound files by similarity or substructure, so structures and queries stay on infrastructure you control. mossflux is one: a single Rust binary serving REST, gRPC and MCP over a SMILES file."
    question: What is a self-hosted molecular search tool?
  - answer: You can run mossflux without Docker by installing it from source with cargo install --git. That build needs Rust 1.95 or later. mossflux publishes no prebuilt binaries to download, so the source build is the route for anyone running it outside Docker.
    question: Can I run mossflux without Docker?
  - answer: You need about 2.5 KB of RAM per molecule in your compound library. The index lives in memory and is rebuilt at each start, so multiply that figure by the molecule count of your SMILES file to get the total the server will use.
    question: How much RAM do I need for my compound library?
  - answer: "mossflux fingerprints are not the same as RDKit's, because they are mossflux's own bits and don't match RDKit's. Chirality is parsed and then ignored, and salts are not stripped. Because of that, canonicalise your structures before loading them into the tool."
    question: "Are mossflux fingerprints the same as RDKit's, and does it handle stereochemistry or salts?"
  - answer: "A substructure search that times out returns an error to the caller and never a partial list of matches. The server's default deadline is 10 s and the maximum is 60 s, so any query that runs past its deadline fails with an error."
    question: What happens when a substructure search times out?
image: /blog/self-hosted-molecular-search-tool/featured-4adae0b4b058.webp
image_alt: "Illustration for Self hosted molecular search tool: Setup guide"
publishDate: "2026-10-30"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/self-hosted-molecular-search-tool#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q364\",\"@type\":\"Organization\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\",\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]}],\"datePublished\":\"2026-10-30\",\"description\":\"What is a self hosted molecular search tool? It runs Tanimoto and SMARTS queries on your own SMILES file. Learn to install mossflux, size RAM, connect agents.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A self-hosted molecular search tool is a program you run on your own machine or cluster that searches your compound files by similarity or substructure, so structures and queries stay on infrastructure you control. mossflux is one: a single Rust binary serving REST, gRPC and MCP over a SMILES file.\"},\"name\":\"What is a self-hosted molecular search tool?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You can run mossflux without Docker by installing it from source with cargo install --git. That build needs Rust 1.95 or later. mossflux publishes no prebuilt binaries to download, so the source build is the route for anyone running it outside Docker.\"},\"name\":\"Can I run mossflux without Docker?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You need about 2.5 KB of RAM per molecule in your compound library. The index lives in memory and is rebuilt at each start, so multiply that figure by the molecule count of your SMILES file to get the total the server will use.\"},\"name\":\"How much RAM do I need for my compound library?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux fingerprints are not the same as RDKit's, because they are mossflux's own bits and don't match RDKit's. Chirality is parsed and then ignored, and salts are not stripped. Because of that, canonicalise your structures before loading them into the tool.\"},\"name\":\"Are mossflux fingerprints the same as RDKit's, and does it handle stereochemistry or salts?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A substructure search that times out returns an error to the caller and never a partial list of matches. The server's default deadline is 10 s and the maximum is 60 s, so any query that runs past its deadline fails with an error.\"},\"name\":\"What happens when a substructure search times out?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q749568\",\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\",\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]}],\"name\":\"Self hosted molecular search tool: Setup guide\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/kXlW-ZxcE2zwpsdPCedbZVRmNJzRvOTh.webp\"},\"url\":\"https://mossflux.com/blog/self-hosted-molecular-search-tool\"},{\"@id\":\"https://mossflux.com/blog/self-hosted-molecular-search-tool#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q364\"},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com\"},{\"@type\":\"CreativeWork\",\"name\":\"loruorg/mossflux repository README\",\"url\":\"https://github.com/loruorg/mossflux\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"GitHub - zilliztech/MolSearch: An opensource molecule analyze software\",\"url\":\"https://github.com/zilliztech/MolSearch\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux changelog\",\"url\":\"https://mossflux.com/docs/changelog\"}],\"dateModified\":\"2026-10-30\",\"datePublished\":\"2026-10-30\",\"description\":\"What is a self hosted molecular search tool? It runs Tanimoto and SMARTS queries on your own SMILES file. Learn to install mossflux, size RAM, connect agents.\",\"headline\":\"Self hosted molecular search tool: Setup guide\",\"image\":\"https://api.birchseek.com/m/kXlW-ZxcE2zwpsdPCedbZVRmNJzRvOTh.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/self-hosted-molecular-search-tool#webpage\"},\"keywords\":[\"self hosted molecular search tool\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/self-hosted-molecular-search-tool#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q749568\"},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\"},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2512},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"statistical Markov model\",\"name\":\"Hidden Markov model\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hidden_Markov_model\",\"https://www.wikidata.org/wiki/Q176769\",\"https://www.google.com/search?kgmid=/m/0p8wy\"]},{\"@type\":\"Thing\",\"description\":\"specialized artificial neural networks that are designed for tasks whose inputs are graphs\",\"name\":\"Graph neural network\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Graph_neural_network\",\"https://www.wikidata.org/wiki/Q107750691\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"chemical compound\",\"name\":\"Histidine\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Histidine\",\"https://www.wikidata.org/wiki/Q485277\",\"https://www.google.com/search?kgmid=/m/0h1tz\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - molecular search
  - cheminformatics
  - tanimoto similarity
  - smarts substructure
  - self-hosting
  - mcp
title: "Self hosted molecular search tool: Setup guide"
---

**A self hosted molecular search tool is a program you run on your own hardware that answers Tanimoto similarity and SMARTS substructure queries over your own SMILES file. mossflux is a [Rust engine](https://mossflux.com/docs/design) for this job, and every result matches a brute-force scan. Install it from source with `cargo install --git` or pull the `loruorg/mossflux` [Docker image](https://mossflux.com/docs/deploy).**

Cheminformatics engineers, platform engineers and agent builders often hold an in-house compound collection, or want ChEMBL behind an internal API, and the structures can't go to a third-party service. A general vector database or a command-line script doesn't give both similarity and substructure search. That's the gap a self hosted molecular search tool fills.

After reading, you'll be able to define the two search types, stand up a self hosted molecular search tool, size its memory, connect an agent and judge any tool against a checklist.

> **Key Takeaways**
> - ChEMBL 37 (2,896,304 molecules) builds in 33 to 41 s into a 5.1 GB index. Nothing is persisted, so every start rebuilds it.
> - The fingerprints are mossflux's own and don't match RDKit bit for bit.
> - A substructure search past its deadline returns an error, never a partial list (default 10 s, maximum 60 s).
> - A request returns at most 10,000 hits; 4 searches run concurrently by default.

## What are the two kinds of molecular search?

Molecular search comes in two kinds: similarity search ranks molecules by the Tanimoto score between fingerprints, and substructure search finds molecules that contain a query pattern written in SMARTS. Similarity search returns either every hit at or above a threshold or the k nearest, and a SMILES is also a valid SMARTS.

![Isometric illustration of a server setup performing distinct global similarity and exact substructure molecular searches.](/blog/self-hosted-molecular-search-tool/image-1-bf083be14c83.webp)

### Similarity search

A fingerprint is a fixed-length string of bits, where each bit records whether the molecule has some small structural feature. The Tanimoto score of two fingerprints is the number of bits they share divided by the number of bits set in either one.

Identical bit patterns score 1.0, and fingerprints with nothing in common score 0. Two questions can be asked of that score:

- Threshold search returns every molecule with a score of at least t, so the result size depends on the data.
- kNN search returns the k highest-scoring molecules, so the result size is fixed and the cut-off score depends on the data.

A score of 1.0 means the two fingerprints have equal bits. Different structures can set the same bits. So a 1.0 doesn't prove the two molecules are the same compound.

### Substructure search

Substructure search asks of each molecule whether its graph of atoms and bonds contains the query pattern. SMARTS extends SMILES with wildcards and logical operators, so a plain structure works as a query.

Be careful with that shortcut. SMARTS applies its own defaults to atoms and bonds, so `c` (aromatic carbon) and `C` (aliphatic carbon) match different atoms. The Daylight SMARTS manual lists the defaults.

### One file, two questions

A SMILES file of your own compounds can answer both questions. Aspirin, `CC(=O)Oc1ccccc1C(=O)O`, as a similarity query returns the molecules whose fingerprints score closest to it.

The sulfonamide `S(=O)(=O)N` as a SMARTS query returns every molecule that contains that group, however different the rest of the structure is.

The first answer is a ranked list of scores. The second is a set of matches with no score. Either way, a self hosted molecular search tool should return exactly what a brute-force scan returns.

## Why self-host a molecular search tool?

A self hosted molecular search tool keeps your structures and your queries on hardware you control, and it removes the quotas of a shared service.

1. In-house collections stay in-house. A process you run reads your SMILES file, so proprietary structures stay on infrastructure you control.
2. The [public demo](https://mossflux.com) at api.mossflux.com searches ChEMBL 37, which is released under CC BY-SA 3.0. A query sent there leaves your network, so private structures belong on your own server.
3. Teams that search make-on-demand spaces reveal their target chemotypes in every query. Running the search yourself keeps those queries in your own logs.
4. The same engine sits behind REST, gRPC and MCP. A script, a service and an AI agent client all query one index, and the three transports cannot disagree. The [API reference](https://mossflux.com/docs/api) lists the endpoints and fields.
6. The [loruorg/mossflux README](https://github.com/loruorg/mossflux) gives the licence as `MIT OR Apache-2.0`. mossflux is free, with no paid plan or hosted service.

Self-hosting has a cost: you run the process and you size its RAM for your database. The [deploy docs](https://mossflux.com/docs/deploy) cover sizing.

## How do you set up a self-hosted molecular search tool?

You set up a self hosted molecular search tool by installing mossflux, loading a SMILES file, waiting for readiness, then posting a similarity query and a SMARTS query to its REST API.

1. Install the binary. From source you need [Rust 1.95 or later](https://mossflux.com/docs/api):

   ```sh
   RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
   ```

 `target-cpu=native` turns on hardware popcount, so the binary runs only on CPUs like the build machine. Build on the machine that will serve. There are no prebuilt CLI binaries and no crates.io release.

 Or run `docker pull loruorg/mossflux` (distroless, non-root). The [published Docker image](https://mossflux.com/docs/deploy) is x86-64. On arm64 you build from the Dockerfile.

2. Prepare a SMILES file. The `.smi`, `.smiles`, `.ism` and `.can` extensions, TSV and ChEMBL chemreps files all load, gzip included. Columns are detected automatically or set with flags. Canonicalise the structures before loading, since a self hosted molecular search tool only sees what you give it.

3. Start the server with the invocation given in the deploy documentation, copying its flags exactly. The name you give your database there is the `db` value in every query.

4. Wait for readiness. `GET /health` answers once the process is up, and `GET /ready` returns 503 until every database is loaded, then 200.

   ```sh
   curl -i "$MOSSFLUX_URL/ready"
   ```

 Set `MOSSFLUX_URL` to the server's REST address. Don't send queries before you see 200.

5. Run a similarity query with aspirin. Send either `threshold` or `k`, plus `limit` and `db`:

   ```sh
   curl -s "$MOSSFLUX_URL/v1/similarity" -H "Content-Type: application/json" \
     -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.7,"limit":10,"db":"chembl"}'
   ```

 The response is JSON listing hits with exact Tanimoto scores. The API reference gives the field names.

6. Run a substructure query for sulfonamides. The fields are `smarts`, `limit`, `offset`, `timeout_ms` and `db`:

   ```sh
   curl -s "$MOSSFLUX_URL/v1/substructure" -H "Content-Type: application/json" \
     -d '{"smarts":"S(=O)(=O)N","limit":10,"offset":0,"timeout_ms":5000,"db":"chembl"}'
   ```

 Raise `offset` to page through hits.

Queries can be up to 4,096 bytes and request bodies up to 64 KiB.

The [CLI](https://mossflux.com/docs/cli) offers the same searches without a server: `search` runs similarity over an FPS file and `substruct` runs SMARTS over a SMILES file.

### Worked example: ChEMBL 37

Download the ChEMBL chemreps SMILES file for ChEMBL 37 and load it as the `chembl` database. Poll `/ready` until it returns 200, then post the two bodies from steps 5 and 6.

The first returns molecules ranked by Tanimoto score against aspirin. The second returns molecules matching `S(=O)(=O)N`.

To check the setup on your own file, run the same two queries and compare them with a brute-force scan. [Run it on your library](https://mossflux.com/#get).

## How do you connect an AI agent to molecular search over MCP?

An agent connects to mossflux by adding its MCP endpoint to the agent's client, and because one engine answers REST, gRPC and MCP, the agent gets the same rows as any other client. The [API docs](https://mossflux.com/docs/api) describe a server that "answers the same searches over REST, MCP and gRPC."

1. Add the server to your agent. For the public demo, Claude Code needs one command:

   ```sh
   claude mcp add --transport http mossflux https://api.mossflux.com/mcp
   ```

 A self-hosted server exposes `/mcp` over streamable HTTP. For stdio, run `mossflux mcp`.

2. Check the five tools. They are `list_databases`, `similarity_search`, `substructure_search`, `molecule_properties` and `get_molecule`.
3. Run a call sequence. The agent calls `list_databases` to learn the database names. It then calls `similarity_search` with aspirin, `CC(=O)Oc1ccccc1C(=O)O`, and finally `get_molecule` on one id from the hits.
4. Expect the same limits. The per-request caps and the substructure deadline apply over MCP too.

gRPC clients call the service `mossflux.v1.Search`, with the methods ListDatabases, Similarity, Substructure and Molecule.

## How much RAM, build time and speed does a self hosted molecular search tool need?

Size a mossflux host at about 2.5 KB of RAM per molecule, expect an index build of under a minute at ChEMBL scale, and read each speed figure with its test conditions.

The [mossflux deploy documentation](https://mossflux.com/docs/deploy) says the index is built in memory at start and is not saved to disk. For ChEMBL 37 (2,896,304 molecules) the build takes 33 to 41 s and the index is 5.1 GB.

Every restart of a self hosted molecular search tool like mossflux repeats the build. That's why readiness probes belong on `/ready`.

[Per request](https://mossflux.com/docs/api), the server returns at most 10,000 hits and runs 4 concurrent searches by default. Further searches wait.

The [benchmarks page](https://mossflux.com/docs/benchmarks) holds the full tables. The FPSim2 comparison ran on an AMD EPYC 9645 VM with 256-bit Morgan r=2 fingerprints over ChEMBL 37 (2.9M molecules), one mossflux thread against one FPSim2 worker, search only, median of 11.

The Arthor row uses an AVX2 `znver3` build on synthetic 50M rows of 256 bits. The 8-core row ran EPYC native on ChEMBL at 256 bits with t = 0.85.

The substructure row used a 17-query battery over 2M ChEMBL rows, 8 threads, on the EPYC 9645 VM, median of 3.

| Test | Result |
|---|---|
| Similarity vs FPSim2, 256 bits | 2.64x to 2.77x faster |
| Per thread, Arthor's ISA class | 0.93 to 1.45 billion molecules a second (Arthor published 0.9 to 1.2 per thread) |
| 8 cores, t = 0.85 | 21.4 billion molecules a second effective |
| Substructure, per thread | 5.2 to 160 million molecules a second |

Two things are not claimed: an aggregate win over Arthor's 96-thread figure, and any billion-scale substructure rate.

The timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on it.

## What does exact mean, and what doesn't it cover?

Exact, in the [mossflux docs](https://mossflux.com/docs/), means the result set is identical to a brute-force scan of every molecule: no false negatives and no false positives. mossflux keeps this by pruning only on a proven bound, refusing a request that exceeds a limit instead of changing the answer, and returning an error, never a partial list, when a search times out.

The [design of the two engines](https://mossflux.com/docs/design) keeps each shortcut safe. BitBound pruning uses a rational bound rounded down, so rounding can't drop a true hit. The inverted bitmap returns the same rows as the linear scan, byte for byte.

Substructure search runs a pipeline. The path screen is a superset filter that never drops a match.

The [API documentation](https://mossflux.com/docs/api) sets the server default at 10 s and the maximum at 60 s.

The [correctness page](https://mossflux.com/docs/correctness) lists the invariants the tests check. Both engines are tested against a brute-force oracle (`brute.rs`), and substructure search is tested against RDKit, where 114 of 114 queries agree.

mossflux has these limits:

- There is no persisted index, write-ahead log, delete, compaction or sharding. Append in memory is supported.
- Row ids are `u32`.
- Chirality is parsed and ignored.
- Input is SMILES and FPS files.
- There's no Python package, and there are no bindings.

No approximate mode exists. There's no setting that trades hits for speed.

## How do you choose a self-hosted molecular search tool?

Choose a self-hosted molecular search tool by answering seven questions from its documentation and by counting the services its deployment needs.

| Criterion | mossflux | Milvus-based MolSearch demo |
| --- | --- | --- |
| Start-up | One process | Milvus, then a webserver container |
| Components named | One binary or image | Milvus, PostgreSQL, RDKit |
| Deploy files | Compose and Kubernetes manifests | Docker commands |

Put these questions to any self hosted molecular search tool:

1. Licence: which one applies?
2. Resources: how much memory does the index need, and does it survive a restart?
3. Setup: how many moving parts are there? mossflux is one process, and its [deploy documentation](https://mossflux.com/docs/deploy) covers Docker, Docker Compose and Kubernetes.
4. Transports: which protocols are offered, and do they share one engine?
5. Correctness: does it return the same rows as a brute-force scan, or an approximation?
6. Inputs: which files load? mossflux takes SMILES files and chemfp FPS files.
7. Maturity: how old is the release? mossflux's current one is 0.1.0, the initial release, and the [changelog](https://mossflux.com/docs/changelog) lists what it contains.

The [zilliztech/MolSearch README](https://github.com/zilliztech/MolSearch) lists Milvus, PostgreSQL and RDKit as requirements. Its deploy steps run Milvus in Docker first, then the molsearch webserver container.

It's a stack that suits a team already running Milvus.

A vector database ranks vectors by distance, so it serves similarity over vectors. SMARTS substructure search is a separate matching problem: it tests a pattern against molecular graphs, and a stack built for vectors needs something extra to answer it.

Best for:

- mossflux: exact Tanimoto and SMARTS queries from one process, with REST, gRPC and MCP clients reading the same results.
- Milvus-based MolSearch demo: reading a worked example of molecular similarity on top of a vector database.

## Frequently asked questions

### What is a self-hosted molecular search tool?

A self-hosted molecular search tool is a program you run on your own machine or cluster that searches your compound files by similarity or substructure, so structures and queries stay on infrastructure you control. mossflux is one: a single Rust binary [serving REST, gRPC and MCP](https://mossflux.com/docs/api) over a SMILES file.

### Can I run mossflux without Docker?

mossflux publishes no prebuilt binaries, so the source build is the route outside Docker.

### How much RAM do I need for my compound library?

[Plan for about 2.5 KB of RAM](https://mossflux.com/docs/deploy) per molecule. The index lives in memory and is rebuilt at each start, so multiply that figure by the molecule count of your SMILES file.

### Are mossflux fingerprints the same as RDKit's?

The fingerprints are mossflux's own bits and don't match RDKit's.

### What happens when a substructure search times out?

The request returns an error, never a partial list. The server's default deadline is 10 s and the maximum is 60 s.

## What should you do next?

**Today:** install from source or pull the Docker image, load a SMILES file, and check that `/ready` returns 200. Then run aspirin (`CC(=O)Oc1ccccc1C(=O)O`) as a similarity query and `S(=O)(=O)N` as a SMARTS query.

**This week:** canonicalise the library before ingestion and size the host's RAM from your molecule count. Set a timeout for substructure queries you don't control, then connect REST or MCP clients.

**Before production:** read the [correctness](https://mossflux.com/docs/correctness) and [benchmarks](https://mossflux.com/docs/benchmarks) pages and the [security notes](https://mossflux.com/docs/security), and check the limits list against your requirements.

[Run it on your library](https://mossflux.com/#get).

## Sources

[Design · mossflux docs](https://mossflux.com/docs/design) · [mossflux docs](https://mossflux.com/docs/) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [API · mossflux docs](https://mossflux.com/docs/api) · [mossflux · exact similarity search at billions of molecules…](https://mossflux.com) · [loruorg/mossflux repository README](https://github.com/loruorg/mossflux) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness). Every other source is linked where the article cites it.
