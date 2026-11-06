---
description: What is a Rust SMARTS engine? Software that finds every molecule matching a SMARTS pattern and returns the same rows as a brute-force scan. Learn to run it.
faq:
  - answer: mossflux is a free Rust engine for SMARTS substructure search and Tanimoto similarity, released as 0.1.0 under MIT OR Apache-2.0, with no paid plan, hosted service or account. It installs with cargo install --git, needs Rust 1.95 or later, and has no crates.io package or prebuilt binaries.
    question: Is there a free Rust SMARTS engine?
  - answer: mossflux does not handle stereochemistry, salts or tautomers at all. It parses chirality and then ignores it, and it has no salt stripping and no tautomer handling. Canonicalise salts, tautomers and stereo in your own pipeline before ingesting any molecules into mossflux.
    question: Does mossflux handle stereochemistry, salts or tautomers?
  - answer: "A SMARTS search that runs past its deadline returns an error, never a partial list of matches. The server default is 10 s and the maximum is 60 s. For queries you don't control, set the timeout_ms field on the REST request."
    question: What happens if a SMARTS search takes too long?
  - answer: An AI agent can run SMARTS searches over MCP by calling the substructure_search tool over streamable HTTP at /mcp, or over stdio with mossflux mcp. REST and gRPC run on the same engine, so the transports return the same answers. Fields and limits are in the API docs.
    question: Can an AI agent run SMARTS searches over MCP?
image: /blog/rust-smarts-engine/featured-0e812c085de2.webp
image_alt: "Illustration for Rust SMARTS engine: Exact substructure search"
publishDate: "2026-11-06"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/rust-smarts-engine#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q364\",\"@type\":\"Organization\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]}],\"datePublished\":\"2026-11-06\",\"description\":\"What is a Rust SMARTS engine? Software that finds every molecule matching a SMARTS pattern and returns the same rows as a brute-force scan. Learn to run it.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux is a free Rust engine for SMARTS substructure search and Tanimoto similarity, released as 0.1.0 under MIT OR Apache-2.0, with no paid plan, hosted service or account. It installs with cargo install --git, needs Rust 1.95 or later, and has no crates.io package or prebuilt binaries.\"},\"name\":\"Is there a free Rust SMARTS engine?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux does not handle stereochemistry, salts or tautomers at all. It parses chirality and then ignores it, and it has no salt stripping and no tautomer handling. Canonicalise salts, tautomers and stereo in your own pipeline before ingesting any molecules into mossflux.\"},\"name\":\"Does mossflux handle stereochemistry, salts or tautomers?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A SMARTS search that runs past its deadline returns an error, never a partial list of matches. The server default is 10 s and the maximum is 60 s. For queries you don't control, set the timeout_ms field on the REST request.\"},\"name\":\"What happens if a SMARTS search takes too long?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"An AI agent can run SMARTS searches over MCP by calling the substructure_search tool over streamable HTTP at /mcp, or over stdio with mossflux mcp. REST and gRPC run on the same engine, so the transports return the same answers. Fields and limits are in the API docs.\"},\"name\":\"Can an AI agent run SMARTS searches over MCP?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q334661\",\"@type\":\"Thing\",\"description\":\"permissive free software license\",\"name\":\"MIT License\",\"sameAs\":[\"https://en.wikipedia.org/wiki/MIT_License\",\"https://www.wikidata.org/wiki/Q334661\",\"https://www.google.com/search?kgmid=/m/04vpm\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q189053\",\"@type\":\"Thing\",\"description\":\"type of computer interface based on entering text commands and viewing text output\",\"name\":\"Command-line interface\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Command-line_interface\",\"https://www.wikidata.org/wiki/Q189053\",\"https://www.google.com/search?kgmid=/m/01yzs\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q2063\",\"@type\":\"Thing\",\"description\":\"text-based open standard designed for human-readable data interchange\",\"name\":\"JSON\",\"sameAs\":[\"https://en.wikipedia.org/wiki/JSON\",\"https://www.wikidata.org/wiki/Q2063\",\"https://www.google.com/search?kgmid=/m/05cntt\"]}],\"name\":\"Rust SMARTS engine: Exact substructure search\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/f3GvcfBN_vh38HI2w_yAIoEPMBpdX3BX.webp\"},\"url\":\"https://mossflux.com/blog/rust-smarts-engine\"},{\"@id\":\"https://mossflux.com/blog/rust-smarts-engine#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q364\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Getting Started with the RDKit in Python\",\"url\":\"https://www.rdkit.org/docs/GettingStartedInPython.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Daylight Theory Manual: SMARTS\",\"url\":\"https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Subgraph isomorphism problem - Wikipedia\",\"url\":\"https://en.wikipedia.org/wiki/Subgraph_isomorphism_problem\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Model Context Protocol specification: Transports\",\"url\":\"https://modelcontextprotocol.io/specification/2025-06-18/basic/transports\"},{\"@type\":\"CreativeWork\",\"name\":\"Model Context Protocol specification: Tools\",\"url\":\"https://modelcontextprotocol.io/specification/2025-06-18/server/tools\"},{\"@type\":\"CreativeWork\",\"name\":\"loruorg/mossflux on GitHub\",\"url\":\"https://github.com/loruorg/mossflux\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/#get\"}],\"dateModified\":\"2026-11-06\",\"datePublished\":\"2026-11-06\",\"description\":\"What is a Rust SMARTS engine? Software that finds every molecule matching a SMARTS pattern and returns the same rows as a brute-force scan. Learn to run it.\",\"headline\":\"Rust SMARTS engine: Exact substructure search\",\"image\":\"https://api.birchseek.com/m/f3GvcfBN_vh38HI2w_yAIoEPMBpdX3BX.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/rust-smarts-engine#webpage\"},\"keywords\":[\"rust smarts engine\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/rust-smarts-engine#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q334661\"},{\"@id\":\"https://www.wikidata.org/wiki/Q189053\"},{\"@id\":\"https://www.wikidata.org/wiki/Q2063\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2133},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"hardware direct memory access over computer networks\",\"name\":\"Remote direct memory access\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Remote_direct_memory_access\",\"https://www.wikidata.org/wiki/Q2561570\",\"https://www.google.com/search?kgmid=/m/04599j\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"type of computer evaluation tool\",\"name\":\"YCSB\",\"sameAs\":[\"https://en.wikipedia.org/wiki/YCSB\",\"https://www.wikidata.org/wiki/Q18754155\",\"https://www.google.com/search?kgmid=/m/012c5nmm\"]},{\"@type\":\"Thing\",\"description\":\"data processing technique to eliminate duplicate copies of repeating data\",\"name\":\"Data deduplication\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Data_deduplication\",\"https://www.wikidata.org/wiki/Q1182260\",\"https://www.google.com/search?kgmid=/m/043n6w3\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - smarts
  - substructure search
  - rust
  - cheminformatics
  - smiles
  - mcp
title: "Rust SMARTS engine: Exact substructure search"
---

**A Rust SMARTS engine is a Rust implementation of SMARTS substructure matching, with similarity search alongside, that returns exactly what [a brute-force scan returns](https://mossflux.com/docs/). mossflux is one such engine, written in Rust: [its `mossflux` CLI](https://mossflux.com/docs/cli) searches files from the shell, and [`mossflux serve`](https://mossflux.com/docs/api) answers the same searches over REST, MCP and gRPC.**

You've got a compound library and a substructure to find in it, and a Rust SMARTS engine has to find every match. Say you search for sulfonamides with `S(=O)(=O)N`. If the engine skips one matching row, the answer's wrong. Nothing in the output tells you.

A Rust SMARTS engine you can read, build and call gives you code to audit and a brute-force scan to compare against. You'll be able to install mossflux, run a SMARTS query on a SMILES file, call it from a service or agent, and know where it stops.

> **Key Takeaways**
> - A search past its deadline [returns an error](https://mossflux.com/docs/api), never a partial list (server default 10 s, maximum 60 s).
> - 114 of 114 queries [agree with RDKit](https://mossflux.com/docs/correctness).

## What is a Rust SMARTS engine?

A Rust SMARTS engine is a program written in Rust that finds every molecule in a collection containing a substructure described by a SMARTS pattern.

![Illustration for What is a Rust SMARTS engine?](/blog/rust-smarts-engine/image-1-bac605193325.webp)

A SMILES string describes one molecule, such as aspirin: `CC(=O)Oc1ccccc1C(=O)O`.

SMARTS describes a pattern, so it can match many molecules.

A SMARTS adds primitives that SMILES lacks. `D<n>` counts explicit connections, `X<n>` counts total connections, `H<n>` counts total hydrogens and `v<n>` gives valence. Recursive SMARTS, written `$(...)`, lets an atom be defined by a whole sub-pattern.

So `S(=O)(=O)N` finds sulfonamides, and `[$(C=O);D3]` finds only carbonyl carbons with exactly three explicit connections.

Substructure search is the task of testing whether such a pattern occurs inside a molecule's graph. RDKit documents it in its [Getting Started guide](https://www.rdkit.org/docs/GettingStartedInPython.html) under "Substructure Searching".

[mossflux's design page](https://mossflux.com/docs/design) says it has two exact search engines in one Rust workspace, and that no external chemistry library is used. The SMILES and SMARTS parsers, perception, screen and matcher are all in the repository, and the same page describes how the pieces fit.

## How do you install and query a Rust SMARTS engine?

mossflux, a Rust SMARTS engine, installs from source with `cargo install` and answers a first query through the `mossflux substruct` subcommand.

1. Check your Rust version. Run `rustc --version` first, because an older toolchain fails at compile time.

2. Install from the repository. Run this in a shell:

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

`target-cpu=native` turns on hardware popcount. Success is a `mossflux` binary on your `PATH`.

3. Prepare a SMILES file. The CLI reads `.smi`, `.smiles`, `.ism` and `.can` files, TSV, ChEMBL chemreps and gzip. It detects the columns automatically, or you set them with flags.

4. Run `mossflux substruct` with your file and a SMARTS. Copy the argument order and flags from the CLI reference. Output is text or TSV.

5. Read the count, then the hits. The count is always exact. `--limit` caps only the hits printed, so a limited run still reports the full count.

6. Fix input errors by position.

### A worked example you can check by eye

Put aspirin, `CC(=O)Oc1ccccc1C(=O)O`, in a one-line file. Query it with the sulfonamide SMARTS `S(=O)(=O)N`.

Aspirin has no sulfur atom, so a correct Rust SMARTS engine reports no match. Treat that as a negative control: if your build reports a hit, something is wrong.

Now add a second line with sulfanilamide, `Nc1ccc(cc1)S(N)(=O)=O`. It has a sulfur double-bonded to two oxygens and single-bonded to a nitrogen, so that line is the hit and aspirin still isn't. Both results can be checked by reading the structures, without a reference tool.

It's a quick check. When the control and the hit both behave, point the same command at your own library. The [install section on mossflux.com](https://mossflux.com/#get) gives the same `cargo install --git` route.

## How does a Rust SMARTS engine stay exact?

A SMARTS engine stays exact by letting each fast stage discard only what it can prove cannot match. [mossflux's correctness page](https://mossflux.com/docs/correctness) defines exact this way: a substructure search returns the same rows as a brute-force scan of the whole database, with no false negatives and no false positives.

Five stages run in order, and the [mossflux design notes](https://mossflux.com/docs/design) show each one written in the project.

1. Parse reads SMILES and SMARTS, with a byte position on any error.
2. Perceive finds rings and aromaticity with the project's own Hückel model.
3. The path screen is a superset filter: it keeps every molecule that could match and never drops a true hit.
4. Census counts atoms by class, and where a class is exact, the count answers the query without the matcher.
5. VF2 verify runs the subgraph matcher on the survivors.

It decides whether one graph sits inside another, so every candidate that reaches it gets a definite yes or no.

The same rule governs every shortcut. A prune rejects only on a proven bound. A limit refuses, and it never changes an answer.

Take the sulfonamide `S(=O)(=O)N` against aspirin, `CC(=O)Oc1ccccc1C(=O)O`. Parse reads one sulfur, two double-bonded oxygens and a nitrogen. The query has no ring atoms, so perception adds no ring constraint.

Aspirin contains no sulfur, so the path screen and census can discard it: a molecule without sulfur can't contain this pattern. A molecule with sulfur, oxygen and nitrogen in the right paths survives to VF2, which accepts it only if the atoms and bonds line up.

The project checks this against a brute-force oracle, `brute.rs`, and against RDKit, where 114 of 114 queries agree. The correctness page lists the tests behind each check.

## What happens when a SMARTS query is too hard?

A SMARTS query is too hard when the matcher can't finish before its deadline, and mossflux then returns an error instead of an answer.

Hard queries come from the problem itself. The [Wikipedia article on the subgraph isomorphism problem](https://en.wikipedia.org/wiki/Subgraph_isomorphism_problem) says it generalises both maximum clique and the Hamiltonian cycle test, "and is therefore NP-complete".

So every substructure search in this Rust SMARTS engine runs under a deadline. The server default is 10 s and the maximum is 60 s, according to the [mossflux API reference](https://mossflux.com/docs/api), which also lists the request fields.

For queries you don't control, set it yourself: `--timeout` on the CLI, `timeout_ms` on `POST /v1/substructure`.

Past the deadline, the result is an error, never a partial list. A partial list could miss true hits, and mossflux's rule is that every answer has the same rows as a brute-force scan.

This Rust SMARTS engine also caps query and request sizes to guard against untrusted input.

- SMARTS up to 2,048 atoms
- queries up to 4,096 bytes
- request bodies up to 64 KiB
- at most 10,000 hits per request

When a search times out, tighten the query: name elements, ring membership or neighbour counts so the matcher has fewer ways to try. Retrying with a bigger number only waits longer on the same search.

## How fast is a Rust SMARTS engine for substructure search?

mossflux's substructure search runs at 5.2 to 160 million molecules a second per thread, under one fixed set of conditions.

The [mossflux benchmarks documentation](https://mossflux.com/docs/benchmarks) publishes the full tables and method. The run used a 17-query battery over 2M ChEMBL rows.

| Condition | Value |
|---|---|
| Queries | 17-query battery |
| Data | 2M ChEMBL rows |
| Threads | 8 |
| Machine | AMD EPYC 9645 VM |
| Trials | Median of 3 |
| Result | 5.2 to 160 million molecules a second per thread |

The range spans the whole battery. The rate depends on the query, so a Rust SMARTS engine's speed isn't one number. If you plan capacity, time your own queries on your own data.

No billion-scale substructure claim is made. This range is the only substructure speed mossflux states.

The timings were taken before the 2026-10-03 hardening pass. They were not re-measured on the new code, although the exactness gates pass on it.

## How do agents and services call a Rust SMARTS engine?

Agents and services call mossflux through `mossflux serve`, and [the mossflux API reference](https://mossflux.com/docs/api) states that `mossflux serve` answers the same searches over REST, MCP and gRPC. The same sulfonamide search, `S(=O)(=O)N`, runs on each path.

1. Start the server. A database is a served SMILES file. Pass several with repeated `--db NAME=PATH` flags.

   ```sh
   mossflux serve --db chembl=chembl.smi
   ```

2. Call REST. Send `POST /v1/substructure` with `smarts`, `limit`, `offset`, `timeout_ms` and `db`. The response fields are listed in the same API reference.

   ```json
   {"smarts": "S(=O)(=O)N", "limit": 100, "offset": 0, "timeout_ms": 5000, "db": "chembl"}
   ```

3. Call the MCP tool. The tool is `substructure_search`, beside `list_databases`, `similarity_search`, `molecule_properties` and `get_molecule`. The [Model Context Protocol specification](https://modelcontextprotocol.io/specification/2025-06-18/server/tools) says MCP allows servers to expose tools that can be invoked by language models.

4. Pick an MCP transport. The [MCP transports specification](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports) defines stdio, communicating over standard in and out, and streamable HTTP. mossflux serves streamable HTTP at `/mcp` and stdio through `mossflux mcp`. For Claude Code:

   ```sh
   claude mcp add --transport http mossflux https://api.mossflux.com/mcp
   ```

5. Call gRPC. The service is `mossflux.v1.Search`. Its `Substructure` method runs the search, and `ListDatabases`, `Similarity` and `Molecule` are the other methods.

6. Rely on one engine. REST, MCP and gRPC all call the same engine inside the server, so the same query gives the same answer on every transport.

7. Plan for the limits. Each request returns at most 10,000 hits, and 4 searches run at once by default (more wait).

The Docker image is `loruorg/mossflux` (x86-64; on arm64, build the image from the Dockerfile). The [deploy page](https://mossflux.com/docs/deploy) covers it, with the Docker Compose and Kubernetes manifests.

## What does mossflux cost, and what does it not do?

mossflux costs nothing: it is open source under `MIT OR Apache-2.0` at [github.com/loruorg/mossflux](https://github.com/loruorg/mossflux), and [the mossflux docs](https://mossflux.com/docs/) list no paid plan, hosted service or account.

The current release is 0.1.0, the initial release.

The index lives in memory. It is built at start, not saved to disk, and has no deletes or sharding. Row ids are `u32`.

On ChEMBL 37 (2,896,304 molecules) the build takes 33 to 41 s and the index takes 5.1 GB, about 2.5 KB of RAM per molecule.

mossflux does not provide:

1. Stereo-aware matching: [chirality is parsed and ignored](https://mossflux.com/docs/design).
2. Salt stripping: salts stay in the molecule you load.
3. Tautomer handling: two tautomers of one compound are two different molecules.
4. Approximate search: there is no LSH and no ANN, only exact search.

Canonicalisation is your job before ingestion, since this Rust SMARTS engine doesn't do it.

On correctness against RDKit, [the mossflux correctness docs](https://mossflux.com/docs/correctness) report that 114 of 114 queries agree in the RDKit differential. That result covers substructure search on those 114 queries only.

## Frequently asked questions

### Is there a free Rust SMARTS engine?

Yes: mossflux is a Rust engine for SMARTS substructure search and Tanimoto similarity, released as 0.1.0 under `MIT OR Apache-2.0`, with no paid plan, hosted service or account.

### Does mossflux handle stereochemistry, salts or tautomers?

mossflux handles none of the three: it parses chirality and ignores it, and it has no salt stripping or tautomer handling. Canonicalise salts, tautomers and stereo in your own pipeline before ingestion.

### What happens if a SMARTS search takes too long?

The search returns an error once it passes its deadline, never a partial list. The server default is 10 s and the maximum is 60 s. For queries you don't control, set `timeout_ms` on the REST request.

### Can an AI agent run SMARTS searches over MCP?

An agent can call the `substructure_search` tool over streamable HTTP at `/mcp`, or over stdio with `mossflux mcp`. REST and gRPC run on the same engine, so the transports return the same answers. Fields and limits are in the [API docs](https://mossflux.com/docs/api).

### Does it give the same answers as RDKit?

In the RDKit differential, 114 of 114 queries agree. The tests are described in the [correctness docs](https://mossflux.com/docs/correctness).

## Next steps: query your own library

[Run it on your library](https://mossflux.com/#get) today: install from source with `cargo install --git` (Rust 1.95 or later), then run `mossflux substruct` on a SMILES file with the aspirin control and a pattern such as `S(=O)(=O)N`.

Before loading production data, canonicalise salts, tautomers and stereo in your own pipeline. Then set `--timeout` or `timeout_ms` on every query that comes from users or agents.

Next, serve the database with `mossflux serve` and call it over REST, gRPC or MCP, using the request fields in the [API documentation](https://mossflux.com/docs/api). The [mossflux docs](https://mossflux.com/docs/) cover each step, and a brute-force scan of your own file is the check for any result.

## Sources

[mossflux docs](https://mossflux.com/docs/) · [Design · mossflux docs](https://mossflux.com/docs/design) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [API · mossflux docs](https://mossflux.com/docs/api) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [Daylight Theory Manual: SMARTS](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy). Every other source is linked where the article cites it.
