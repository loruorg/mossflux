---
description: How do you deploy exact similarity search? Plan about 2.5 KB RAM per molecule and gate traffic on /ready. Start with this similarity search deployment guide.
faq:
  - answer: A similarity search deployment loads fingerprints into an in-memory index and serves threshold and kNN queries over an API. With mossflux, a single binary listens on ports 8080 and 50051, and the /ready endpoint returns 503 until every database is loaded.
    question: What is a similarity search deployment?
  - answer: Exact search is not automatically slower in mossflux, which has no approximate mode and therefore no faster setting inside it to compare against. Its BitBound prune skips a molecule only when a proven bound shows it cannot reach the threshold, so pruning never discards a molecule that could qualify.
    question: Does exact search mean it is slower than approximate search?
  - answer: A Tanimoto score of 1.0 does not guarantee the same compound. It means the two fingerprints have equal bits, and equal bits do not prove identity. Two different compounds can therefore share a score of 1.0. Canonicalise salts, tautomers and stereo before ingestion.
    question: Is a Tanimoto score of 1.0 the same compound?
  - answer: The index does not survive a restart, because it is rebuilt in memory at every start. ChEMBL 37 takes 33 to 41 s and a 5.1 GB index, about 2.5 KB of RAM per molecule, so budget both time and memory.
    question: Does the index survive a restart?
  - answer: The server can run on arm64 if you build it yourself on that architecture. Build the image locally from the Dockerfile, or install from source with cargo. The Docker image is x86-64 only, so arm64 hosts need one of these two routes.
    question: Can I run the server on arm64?
image: /blog/similarity-search-deployment-guide/featured-c80d5c0280bb.webp
image_alt: "Illustration for Similarity search deployment guide: Sizing RAM"
publishDate: "2026-11-21"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/similarity-search-deployment-guide#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q17126276\",\"@type\":\"Thing\",\"description\":\"searching for similar items in a data set\",\"name\":\"Similarity search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Similarity_search\",\"https://www.wikidata.org/wiki/Q17126276\",\"https://www.google.com/search?kgmid=/m/0y6lbnf\"]}],\"datePublished\":\"2026-11-21\",\"description\":\"How do you deploy exact similarity search? Plan about 2.5 KB RAM per molecule and gate traffic on /ready. Start with this similarity search deployment guide.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A similarity search deployment loads fingerprints into an in-memory index and serves threshold and kNN queries over an API. With mossflux, a single binary listens on ports 8080 and 50051, and the /ready endpoint returns 503 until every database is loaded.\"},\"name\":\"What is a similarity search deployment?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Exact search is not automatically slower in mossflux, which has no approximate mode and therefore no faster setting inside it to compare against. Its BitBound prune skips a molecule only when a proven bound shows it cannot reach the threshold, so pruning never discards a molecule that could qualify.\"},\"name\":\"Does exact search mean it is slower than approximate search?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A Tanimoto score of 1.0 does not guarantee the same compound. It means the two fingerprints have equal bits, and equal bits do not prove identity. Two different compounds can therefore share a score of 1.0. Canonicalise salts, tautomers and stereo before ingestion.\"},\"name\":\"Is a Tanimoto score of 1.0 the same compound?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The index does not survive a restart, because it is rebuilt in memory at every start. ChEMBL 37 takes 33 to 41 s and a 5.1 GB index, about 2.5 KB of RAM per molecule, so budget both time and memory.\"},\"name\":\"Does the index survive a restart?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The server can run on arm64 if you build it yourself on that architecture. Build the image locally from the Dockerfile, or install from source with cargo. The Docker image is x86-64 only, so arm64 hosts need one of these two routes.\"},\"name\":\"Can I run the server on arm64?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q608751\",\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q28865\",\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q19900\",\"@type\":\"Organization\",\"description\":\"American multinational computer corporation\",\"name\":\"Oracle Corporation\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Oracle_Corporation\",\"https://www.wikidata.org/wiki/Q19900\",\"https://www.google.com/search?kgmid=/m/05njw\"]}],\"name\":\"Similarity search deployment guide: Sizing RAM\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/vJJW4qL0J67XnwGlHcCMHcLtHc9hdUWH.webp\"},\"url\":\"https://mossflux.com/blog/similarity-search-deployment-guide\"},{\"@id\":\"https://mossflux.com/blog/similarity-search-deployment-guide#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q17126276\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux benchmarks\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Configure Liveness, Readiness and Startup Probes | Kubernetes\",\"url\":\"https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/\"},{\"@type\":\"CreativeWork\",\"name\":\"Security · mossflux docs\",\"url\":\"https://mossflux.com/docs/security\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/#get\"}],\"dateModified\":\"2026-11-21\",\"datePublished\":\"2026-11-21\",\"description\":\"How do you deploy exact similarity search? Plan about 2.5 KB RAM per molecule and gate traffic on /ready. Start with this similarity search deployment guide.\",\"headline\":\"Similarity search deployment guide: Sizing RAM\",\"image\":\"https://api.birchseek.com/m/vJJW4qL0J67XnwGlHcCMHcLtHc9hdUWH.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/similarity-search-deployment-guide#webpage\"},\"keywords\":[\"similarity search deployment guide\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/similarity-search-deployment-guide#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q608751\"},{\"@id\":\"https://www.wikidata.org/wiki/Q28865\"},{\"@id\":\"https://www.wikidata.org/wiki/Q19900\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2513},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"protocol for applications to provide context to LLMs\",\"name\":\"Model Context Protocol\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Model_Context_Protocol\",\"https://www.wikidata.org/wiki/Q133436854\",\"https://www.google.com/search?kgmid=/g/11x5hnm0vb\"]},{\"@type\":\"Thing\",\"description\":\"number of nonzero symbols in a string\",\"name\":\"Hamming weight\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hamming_weight\",\"https://www.wikidata.org/wiki/Q5645805\",\"https://www.google.com/search?kgmid=/m/048nw2\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - similarity search
  - tanimoto
  - chemical fingerprints
  - kubernetes
  - in-memory index
title: "Similarity search deployment guide: Sizing RAM"
---

**A similarity search deployment guide comes down to one sequence: mossflux [builds an in-memory index](https://mossflux.com/docs/deploy) from your fingerprints at every start (it isn't saved to disk), then [serves exact Tanimoto threshold and kNN queries](https://mossflux.com/docs/api) over REST, gRPC and MCP from one binary. [Results match a brute-force scan](https://mossflux.com/docs/) row for row, and `/ready` returns 503 until every database is loaded.**

You have a compound library, ChEMBL or an in-house collection, and a search API to put in front of it. Missing a true hit is the failure you can't accept, and a speed figure without its machine, fingerprint width and threshold tells you nothing.

With the figures in this similarity search deployment guide, you can size RAM for each database, gate traffic on readiness and set request limits before the first deploy. You can also send a query to the public demo at `https://api.mossflux.com` before you install anything.

> **Key Takeaways**
> - [ChEMBL 37 (2,896,304 molecules)](https://mossflux.com/docs/deploy) rebuilds at every start in 33 to 41 s: a 5.1 GB index, about 2.5 KB RAM per molecule.
> - [A timed-out substructure search](https://mossflux.com/docs/api) returns an error, never a partial list.
> - REST, gRPC and MCP share one engine, so they cannot disagree.
> - [`score == 1.0`](https://mossflux.com/docs/design) means equal bits, not the same compound.

## What does exact similarity search guarantee?

Exact similarity search guarantees the same rows as a brute-force scan of the whole database: no false negatives and no false positives. Pruning such as BitBound may skip a molecule only when a proven bound shows it cannot reach the threshold, so speed never changes the answer.

![Illustration for What does exact similarity search guarantee?](/blog/similarity-search-deployment-guide/image-1-43f82668f674.webp)

The same page defines what "exact" means and lists the tests that check it against a brute-force oracle.

Two fingerprints with very different bit counts (popcounts) can't reach a high Tanimoto score. The engine therefore sorts the arena by popcount and skips ranges that cannot qualify.

The [design page](https://mossflux.com/docs/design) describes BitBound pruning over a popcount-sorted arena. The bound is a rational number rounded down, so rounding can't drop a true hit.

Segmented append adds molecules in memory and changes no answer.

Substructure search follows the same rule. Its path screen is a superset filter that never drops a match, and the RDKit differential agrees on 114 of 114 queries.

A hit list from a screening workflow can therefore be audited. Run any query, run a brute-force scan over the same database, and the two lists must match row for row. A missing row is a defect to report.

mossflux has no approximate mode: no LSH and no ANN. No setting trades answers for speed, so no accuracy percentage applies.

## How does Tanimoto similarity work on binary fingerprints?

Tanimoto similarity on bit vectors is the number of bits set in both fingerprints divided by the number set in either. It equals the Jaccard index of the two sets of on-bits and ranges from 0 to 1.

Threshold search returns every molecule at or above a cutoff, and kNN returns the k nearest.

Suppose two fingerprints have 40 and 50 bits on, with 30 shared. The union holds 40 + 50 - 30 = 60 bits, so the Tanimoto score is 30 / 60 = 0.5.

The two search modes answer different questions. A threshold query with t = 0.7 can return zero hits or thousands. A kNN query with k = 10 always returns ten rows if the database holds that many.

[mossflux ranks kNN](https://mossflux.com/docs/design) by the exact fraction, and ties go to the lower popcount, then the earlier row. Scores are exact f64 values.

Folding shrinks a wide fingerprint to fewer bits, for example 2048 to 256. Unrelated features can then land on the same bit, so folded scores differ from full-width scores.

mossflux accepts fingerprints up to 65,472 bits wide, and [`mossflux search`](https://mossflux.com/docs/cli) reads them from a chemfp FPS file.

The server needs no fingerprints from you. [It computes its own 2048-bit ECFP4-style fingerprints](https://mossflux.com/docs/api), so a query is a SMILES. These bits are mossflux's own and don't match RDKit or other toolkits bit for bit.

This request sends aspirin, `CC(=O)Oc1ccccc1C(=O)O`, as a threshold query:

```json
POST /v1/similarity
{"smiles": "CC(=O)Oc1ccccc1C(=O)O", "threshold": 0.7, "limit": 20, "db": "chembl"}
```

The response lists matching rows with their exact scores. For kNN, send `k` in place of `threshold`. The `db` value is the name you gave with `--db NAME=PATH`.

`score == 1.0` means equal bits, not the same compound. Two different structures can set identical bits.

## How fast is exact similarity search, and under what conditions?

The [full benchmark tables and run conditions](https://mossflux.com/docs/benchmarks) list the machine, data and method of each run.

| Run | mossflux | Compared with |
| --- | --- | --- |
| 256-bit, 1 thread each, ChEMBL 37 | 2.64x to 2.77x faster | FPSim2, 1 worker |
| Per thread, AVX2 `znver3`, 50M synthetic rows | 0.93 to 1.45 billion/s | Arthor, 0.9 to 1.2 billion/s per thread |
| 8 cores, ChEMBL 256-bit, t = 0.85 | 21.4 billion/s effective | None |
| Substructure, 17 queries, 2M rows | 5.2 to 160 million/s per thread | None |

Against FPSim2 at 256 bits, mossflux was 2.64x to 2.77x faster on an AMD EPYC 9645 VM with 256-bit Morgan r=2 fingerprints and ChEMBL 37 (2.9M molecules), using one mossflux thread against one FPSim2 worker, search only, median of 11.

On Arthor's ISA class (an AVX2 `znver3` build), mossflux ran 0.93 to 1.45 billion molecules a second per thread on synthetic 50M rows of 256 bits. Arthor's published figure is 0.9 to 1.2 billion molecules a second per thread. Aggregates across different machines aren't comparable.

With 8 cores on a native EPYC build, ChEMBL at 256 bits and a threshold of t = 0.85, the effective rate was 21.4 billion molecules a second.

Substructure search ran at 5.2 to 160 million molecules a second per thread over a 17-query battery on 2M ChEMBL rows, with 8 threads on an EPYC 9645 VM, median of 3. The range is wide because query difficulty varies.

The timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on the new code.

mossflux fits exact threshold, kNN and substructure search behind one API, where every answer must match a brute-force scan. For a team already running FPSim2, the measured gap covers search only, with one worker each. For a deployment already built on Arthor, the measured figures are per thread on one ISA class.

## Similarity search deployment guide: how much memory do you need?

mossflux needs about 2.5 KB of RAM per molecule, and it builds that index in memory every time the server starts. The [deploy documentation](https://mossflux.com/docs/deploy) works this through on ChEMBL 37.

The documented case is ChEMBL 37, 2,896,304 molecules: the build takes 33 to 41 s, the index is 5.1 GB, and the cost is about 2.5 KB of RAM per molecule. Nothing is saved to disk, so a restart, a rollout or a crash pays the build time again.

1. Count the rows in every database. Apply the per-molecule figure to each database and add the results.
2. Budget the start-up time. Use 33 to 41 s as the ChEMBL 37 reference. The index isn't a file that reloads instantly after a restart: nothing's persisted, so nothing reloads.
3. Gate traffic on `/ready`. The [Kubernetes documentation on probes](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/) says that an application "might need to load large data or configuration files during startup" and that in such cases you don't want to kill it, but you don't want to send it requests either.

```yaml
readinessProbe:
  httpGet:
    path: /ready
    port: 8080
```

4. Check the outcome. Right after a start, `/ready` answers 503. Once the last database finishes loading, it answers 200 and the pod joins the Service.
5. Plan growth. Appended rows aren't persisted either, so the next start rebuilds from the database files alone.

A service with two databases has two separate memory bills and one readiness gate. The pod stays out of rotation until both are loaded.

## What are the limits of an in-memory similarity search?

The limits of mossflux's in-memory similarity search are that every start rebuilds the index, rows cannot be deleted, the fingerprints are its own, and input must be canonical SMILES. The [deploy documentation](https://mossflux.com/docs/deploy) covers the image, Compose and Kubernetes paths within these limits.

1. The index lives in memory. There is no persisted or on-disk index, so every start rebuilds it. Budget RAM and restart time for a full build each time.
2. [The design notes](https://mossflux.com/docs/design) list no deletes, no compaction, no write-ahead log and no sharding, and row ids are u32. To drop a molecule, edit the SMILES file and restart.
3. [The fingerprints are mossflux's own](https://mossflux.com/docs/api) and are not RDKit bit-compatible. Don't compare bits or scores across the two toolkits.
4. Salts, tautomers and stereo are not normalised, and chirality in SMARTS parses and is ignored. Canonicalise your SMILES before ingestion, or two forms of one compound become two rows.
5. Convert structures to SMILES upstream.
6. [The CLI page](https://mossflux.com/docs/cli) confirms there is no Python package, no crates.io release and no prebuilt CLI binaries. Install the CLI with `cargo install --git`. The Docker image is x86-64 only, so on arm64 build it from the Dockerfile.

## Similarity search deployment guide for Docker, Compose and Kubernetes

mossflux deploys four ways: the Docker Hub image, Docker Compose, Kubernetes manifests, or a build from source. Each path ends in the same server, so pick the one that matches how you already run services. The exact Docker, Compose and Kubernetes commands and flags are on the [deploy page](https://mossflux.com/docs/deploy).

1. Pull the container image with `docker pull loruorg/mossflux`. The image on Docker Hub is x86-64, distroless and non-root.
2. For Docker Compose, take the Compose file from the `deploy/` directory of the repository and start it as the deploy page describes. Point it at your SMILES file before the first start.
3. For Kubernetes, take the manifests from `deploy/`.
4. To install from source, follow the [CLI reference](https://mossflux.com/docs/cli). With Rust 1.95 or later, run:

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

Build on the same machine type that will serve.

All four paths run `mossflux serve --db NAME=PATH`, once per database. The database is a SMILES file.

The `mossflux` binary has other subcommands: `search` (similarity over a chemfp FPS file, plain or gzip), `substruct` (SMARTS over a SMILES file), `mcp`, `info`, `tanimoto`, `bench` and `substruct-bench`. The CLI reference lists their flags.

The deployment worked when `curl -i http://localhost:8080/ready` returns 503 while the index builds and then 200. On ChEMBL 37 the build takes 33 to 41 s, so allow for that before the first probe.

To check the whole path on your own compounds, [run it on your library](https://mossflux.com/#get) with the install options on the get page.

## Similarity search deployment guide for production limits

A similarity search API runs safely in production when its limits are published, each limit refuses a request instead of changing an answer, and untrusted input is rejected with a position. The [mossflux API documentation](https://mossflux.com/docs/api) lists the numbers below.

1. Wire the liveness check to `GET /health` and the readiness check to `GET /ready`, so a load balancer holds traffic back while the in-memory index builds. Success is a 200 from `/ready` before the first search arrives.
2. Cap result size and concurrency. One request returns at most 10,000 hits, and the server runs 4 concurrent searches by default while more wait. Page through larger result sets with `offset` rather than asking for more. A cap refuses the request instead of quietly trimming the answer.
3. Publish the input limits to callers. A SMILES may have up to 65,534 atoms and a SMARTS up to 2,048. Malformed or oversized input is rejected with a byte position or line number, and the [security page](https://mossflux.com/docs/security) states what mossflux guarantees on untrusted input.
4. Set a deadline on substructure search. The server default is 10 s and the maximum is 60 s, set per request with `timeout_ms`. Past the deadline the answer is an error, never a partial list, so treat a timeout as "no answer" and not as "no more hits".

A sulfonamide search shows the fields. If your database is named `chembl` (use a name that `GET /v1/databases` lists), the request is:

```sh
curl -X POST https://api.mossflux.com/v1/substructure \
  -H 'Content-Type: application/json' \
  -d '{"smarts": "S(=O)(=O)N", "limit": 100, "offset": 0, "timeout_ms": 5000, "db": "chembl"}'
```

`smarts` is the query, `limit` and `offset` page the hits, and `db` picks the database. Here `limit` asks for 100 hits per page, and `timeout_ms` of 5000 sets a 5 s deadline, below the 10 s default.

The CLI behaves the same way: `--limit` caps only the printed hits, and the counts stay exact.

## One engine behind three transports

[`mossflux serve`](https://mossflux.com/docs/api) answers REST, gRPC and MCP from one engine, so the transports can't disagree. gRPC exposes the service `mossflux.v1.Search` with `ListDatabases`, `Similarity`, `Substructure` and `Molecule`. MCP exposes `list_databases`, `similarity_search`, `substructure_search`, `molecule_properties` and `get_molecule`.

Register the hosted demo as a tool for Claude Code with:

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

For a local stdio server, run `mossflux mcp` instead.

## Frequently asked questions

### What is a similarity search deployment?

A similarity search deployment loads fingerprints into an in-memory index and serves threshold and kNN queries over an API. With mossflux, [one binary listens on ports 8080 and 50051](https://mossflux.com/docs/deploy), and `/ready` returns 503 until every database is loaded.

### Does exact search mean it is slower than approximate search?

Exact search doesn't drop true hits to gain speed; it saves time by skipping only molecules that can't qualify.

[Its BitBound prune](https://mossflux.com/docs/correctness) skips a molecule only when a proven bound shows it cannot reach the threshold.

### Is a Tanimoto score of 1.0 the same compound?

No. A score of 1.0 means equal bits, and two different structures can set identical bits.

### Does the index survive a restart?

The index is rebuilt in memory at every start. ChEMBL 37 takes 33 to 41 s and a 5.1 GB index, about 2.5 KB of RAM per molecule, so budget both time and memory.

### Can I run the server on arm64?

The Docker image is x86-64 only, so on arm64 build it from the Dockerfile.

## Where should you start with your own similarity search deployment?

A similarity search deployment can start with one command and grow in three steps.

Today, install mossflux and run `mossflux search` on a small FPS file, or run `mossflux serve` on a SMILES file.

This week, size RAM at about 2.5 KB per molecule (the [deploy documentation](https://mossflux.com/docs/deploy) has the working), then point the readiness probe at `GET /ready`, which returns 503 until the index is ready.

This month, set limits and timeouts for callers you don't control, and register the MCP tool if an agent needs it.

[Run it on your library](https://mossflux.com/#get). It's free and open source under MIT OR Apache-2.0, with no account, hosted service or paid plan.

## Sources

[Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [API · mossflux docs](https://mossflux.com/docs/api) · [mossflux docs](https://mossflux.com/docs/) · [Design · mossflux docs](https://mossflux.com/docs/design) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [mossflux benchmarks](https://mossflux.com/docs/benchmarks) · [Configure Liveness, Readiness and Startup Probes |…](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/). Every other source is linked where the article cites it.
