---
description: How do you run a local ChEMBL search setup? Download the chemreps file, install mossflux, and run one serve command for exact hits. Start with the steps.
faq:
  - answer: Download the ChEMBL chemreps SMILES file from the EMBL-EBI FTP site, install mossflux with cargo install --git, and run mossflux serve --db chembl=PATH. The server builds an in-memory index at start, then answers similarity and SMARTS substructure queries over REST, gRPC and MCP.
    question: How do I set up a local ChEMBL search?
  - answer: A local search returns every hit, because each result matches a brute-force scan. A substructure search that passes its deadline returns an error, never a partial list, and the --limit flag caps only the printed hits, so the hit counts stay exact.
    question: Does a local search return every hit, or a partial list?
  - answer: "The fingerprints are not compatible with RDKit, because they are mossflux's own 2048-bit ECFP4-style bits that don't match RDKit or other toolkits. Send a SMILES and let the server compute its fingerprint, rather than comparing scores against RDKit output or loading RDKit bits."
    question: Are the fingerprints compatible with RDKit?
  - answer: "You need to canonicalise the SMILES file before ingestion, because mossflux doesn't strip salts or handle tautomers. It also parses chirality but ignores it, so any cleanup has to happen before the server loads the SMILES. The API reference lists the request fields."
    question: Do I need to remove salts and tautomers first?
  - answer: The index does not survive a restart, because it lives in memory and is rebuilt at every start. On ChEMBL 37 (2,896,304 molecules) the index build took 33 to 41 s, and the GET /ready endpoint returns 503 until the build finishes.
    question: Does the index survive a restart?
image: /blog/local-chembl-search-setup/featured-acd200d77d29.webp
image_alt: "Illustration for Local ChEMBL search setup: A step-by-step guide"
publishDate: "2026-11-02"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/local-chembl-search-setup#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"datePublished\":\"2026-11-02\",\"description\":\"How do you run a local ChEMBL search setup? Download the chemreps file, install mossflux, and run one serve command for exact hits. Start with the steps.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Download the ChEMBL chemreps SMILES file from the EMBL-EBI FTP site, install mossflux with cargo install --git, and run mossflux serve --db chembl=PATH. The server builds an in-memory index at start, then answers similarity and SMARTS substructure queries over REST, gRPC and MCP.\"},\"name\":\"How do I set up a local ChEMBL search?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A local search returns every hit, because each result matches a brute-force scan. A substructure search that passes its deadline returns an error, never a partial list, and the --limit flag caps only the printed hits, so the hit counts stay exact.\"},\"name\":\"Does a local search return every hit, or a partial list?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The fingerprints are not compatible with RDKit, because they are mossflux's own 2048-bit ECFP4-style bits that don't match RDKit or other toolkits. Send a SMILES and let the server compute its fingerprint, rather than comparing scores against RDKit output or loading RDKit bits.\"},\"name\":\"Are the fingerprints compatible with RDKit?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You need to canonicalise the SMILES file before ingestion, because mossflux doesn't strip salts or handle tautomers. It also parses chirality but ignores it, so any cleanup has to happen before the server loads the SMILES. The API reference lists the request fields.\"},\"name\":\"Do I need to remove salts and tautomers first?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The index does not survive a restart, because it lives in memory and is rebuilt at every start. On ChEMBL 37 (2,896,304 molecules) the index build took 33 to 41 s, and the GET /ready endpoint returns 503 until the build finishes.\"},\"name\":\"Does the index survive a restart?\"}],\"name\":\"Local ChEMBL search setup: A step-by-step guide\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/AeWDw18qC2xTCAGdJlMmL6E6A8DLNElO.webp\"},\"url\":\"https://mossflux.com/blog/local-chembl-search-setup\"},{\"@id\":\"https://mossflux.com/blog/local-chembl-search-setup#article\",\"@type\":\"BlogPosting\",\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"ChEMBL - ChEMBL\",\"url\":\"https://www.ebi.ac.uk/chembl/\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Security · mossflux docs\",\"url\":\"https://mossflux.com/docs/security\"},{\"@type\":\"CreativeWork\",\"name\":\"Transports - Model Context Protocol specification\",\"url\":\"https://modelcontextprotocol.io/specification/2025-06-18/basic/transports\"}],\"dateModified\":\"2026-11-02\",\"datePublished\":\"2026-11-02\",\"description\":\"How do you run a local ChEMBL search setup? Download the chemreps file, install mossflux, and run one serve command for exact hits. Start with the steps.\",\"headline\":\"Local ChEMBL search setup: A step-by-step guide\",\"image\":\"https://api.birchseek.com/m/AeWDw18qC2xTCAGdJlMmL6E6A8DLNElO.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/local-chembl-search-setup#webpage\"},\"keywords\":[\"local chembl search setup\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/local-chembl-search-setup#webpage\"},\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2294},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"statistical Markov model\",\"name\":\"Hidden Markov model\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hidden_Markov_model\",\"https://www.wikidata.org/wiki/Q176769\",\"https://www.google.com/search?kgmid=/m/0p8wy\"]},{\"@type\":\"Thing\",\"description\":\"specialized artificial neural networks that are designed for tasks whose inputs are graphs\",\"name\":\"Graph neural network\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Graph_neural_network\",\"https://www.wikidata.org/wiki/Q107750691\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"chemical compound\",\"name\":\"Histidine\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Histidine\",\"https://www.wikidata.org/wiki/Q485277\",\"https://www.google.com/search?kgmid=/m/0h1tz\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - chembl
  - similarity search
  - substructure search
  - smarts
  - mcp
  - cheminformatics
title: "Local ChEMBL search setup: A step-by-step guide"
---

**A local ChEMBL search setup takes three steps: download the chemreps SMILES file for the current release from the ChEMBL FTP release directory at EMBL-EBI, install mossflux with `cargo install --git`, and [run `mossflux serve --db chembl=PATH`](https://mossflux.com/docs/cli). The server [answers the same searches over REST, MCP and gRPC](https://mossflux.com/docs/api), and every similarity or substructure hit [equals what a brute-force scan returns](https://mossflux.com/docs/).**

If you reach ChEMBL only through a Python client or a hosted endpoint, someone else decides the data version and the timeouts. A local engine moves those decisions to you: you choose the release, keep the file and can check every hit against a brute-force scan.

After setup you can load ChEMBL 37, run a similarity query and a SMARTS substructure query, state the limits of each plainly, and attach an AI agent to the same engine.

> **Key Takeaways**
> - A substructure search that passes its deadline (server default 10 s, maximum 60 s) returns an error, never a partial list.
> - `--limit` caps only the printed hits; the counts stay exact.
> - The in-memory index is [rebuilt at every start](https://mossflux.com/docs/deploy): ChEMBL 37 (2,896,304 molecules) needs 5.1 GB, about 2.5 KB of RAM per molecule.
> - The fingerprints are mossflux's own bits, not RDKit-compatible.

## Local ChEMBL search setup vs hosted services: which do you need?

Hosted EBI services suit you when you want current data and don't want any setup. A local ChEMBL search setup suits you when you need a fixed release, offline use or your own concurrency limit.

| Criterion | Hosted EBI services | Local engine |
| --- | --- | --- |
| Data version | Whatever EBI currently serves | Pinned to a release you name |
| Network needed | Yes | No, once the file is downloaded |
| Concurrency | Set by the host | Set by you |
| Index location | EMBL-EBI servers | Your own memory |
| Setup | Install a client | Download a file, build an index |

EMBL-EBI runs ChEMBL and maintains the data. [The ChEMBL home page](https://www.ebi.ac.uk/chembl/) states that the current release is "Provided under a Creative Commons Attribution-ShareAlike 3.0 Unported license", which applies to a local copy too.

A local ChEMBL search setup adds control. Pin the data to ChEMBL 37 and a query run next year sees the same molecules. The index sits in your own memory, so a search crosses no network and works offline.

You also decide how many searches run at once, so a batch job doesn't compete with other users.

The public demo shows the API shape before you install anything. The [mossflux API documentation](https://mossflux.com/docs/api) says the demo at `https://api.mossflux.com` (REST and MCP) searches ChEMBL 37 on one small 4-thread server.

Send it a similarity query for aspirin, `CC(=O)Oc1ccccc1C(=O)O`, or a SMARTS query for sulfonamides, `S(=O)(=O)N`, then run the same calls against your own machine. The demo's a shared server, so its response times don't tell you anything about your hardware.

## How do you do a local ChEMBL search setup from the chemreps file?

A local ChEMBL search setup from the chemreps file needs one gzip file, one binary built from source and one `mossflux serve` command. The [deploy documentation](https://mossflux.com/docs/deploy) reports that on mossflux's ChEMBL 37 load (2,896,304 molecules), the index build took 33 to 41 s.

1. Download the chemreps file. Take the SMILES file for a named release from the ChEMBL FTP release directory at EMBL-EBI. Don't unzip it: the [CLI documentation](https://mossflux.com/docs/cli) says the loader reads gzip and detects the columns itself.

   ```sh
   curl -O https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/releases/chembl_37/chembl_37_chemreps.txt.gz
   ```

2. Prepare the structures before loading. If you want parent compounds or one tautomer per molecule, produce a cleaned SMILES file first, and keep the ChEMBL attribution if you share results.

3. Install from source. You need Rust 1.95 or later.

   ```sh
   RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
   ```

4. Start the server. `--db` takes `NAME=PATH`.

   ```sh
   mossflux serve --db chembl=chembl_37_chemreps.txt.gz
   ```

 The [API documentation](https://mossflux.com/docs/api) says REST and MCP listen on port 8080, gRPC on 50051.

5. Wait for `/ready`. `curl -i localhost:8080/ready` returns 503 until every database is loaded, then 200. Don't send queries while it returns 503; `/health` is the liveness check.

6. Run a similarity query. Send the aspirin SMILES, and the server computes the fingerprint itself.

   ```sh
   curl -s localhost:8080/v1/similarity -H 'content-type: application/json' \
     -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.7,"limit":10,"db":"chembl"}'
   ```

7. Run a substructure query. Send SMARTS, here the sulfonamide pattern.

   ```sh
   curl -s localhost:8080/v1/substructure -H 'content-type: application/json' \
     -d '{"smarts":"S(=O)(=O)N","limit":10,"timeout_ms":10000,"db":"chembl"}'
   ```

Your local ChEMBL search setup works when `/ready` returns 200 and both queries return hits. You can [try the demo](https://mossflux.com/#demo) first to see the same queries run against ChEMBL 37.

## What is structure search: similarity or substructure?

In a local ChEMBL search setup, structure search means one of two exact queries. Similarity search takes a SMILES and returns every molecule with a Tanimoto score at or above a threshold, or the k nearest. Substructure search takes a SMARTS and returns every molecule containing that pattern, as [mossflux's documentation](https://mossflux.com/docs/) describes.

Both return exactly what a brute-force scan returns: no false negatives and no false positives. A prune rejects a molecule only on a proven bound. A limit refuses, and it never changes an answer.

Similarity takes a SMILES such as aspirin, `CC(=O)Oc1ccccc1C(=O)O`, plus either a threshold (Tanimoto >= t) or k.

Substructure takes a SMARTS, and a SMILES is a valid SMARTS, so `c1ccncc1` works as a query, as does the sulfonamide pattern `S(=O)(=O)N`. Because mossflux parses chirality and ignores it, a stereo-specific query matches both mirror images.

The [design documentation](https://mossflux.com/docs/design) sets out a fixed pipeline for each query: parse, perceive rings and aromaticity, path screen, census, then VF2 verify. The path screen is a superset filter that never drops a match, so it only saves work. The pipeline is tested against a brute-force oracle.

Two limits apply per request. A request returns at most 10,000 hits. A substructure search has a deadline, 10 s by default and 60 s at most, because subgraph matching is NP-complete.

## How do you run a first similarity and substructure query on ChEMBL?

A first query on ChEMBL takes one `curl` call for similarity, and one CLI command plus one `curl` call for substructure, against a server started with `--db chembl=PATH` on port 8080.

1. Wait for the index. Call `GET /ready`. A query sent while it's still returning 503 fails for that reason alone.

2. Run the similarity query with aspirin. The request body holds only a SMILES, because the server computes the fingerprint:

```sh
curl -s -X POST http://localhost:8080/v1/similarity \
  -H 'Content-Type: application/json' \
  -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.7,"limit":10,"db":"chembl"}'
```

 The fields are `smiles`, `threshold`, `k`, `limit` and `db`. Set `threshold` for every molecule at or above a Tanimoto score, or set `k` for the nearest k. Each hit carries an exact score.

 Don't send this SMILES to the CLI `search` subcommand, which runs over an FPS file, not a SMILES file.

3. Run the substructure query from the CLI. `substruct` takes SMARTS over the chemreps SMILES file:

```sh
mossflux substruct chembl_37_chemreps.txt.gz 'S(=O)(=O)N' --limit 10
```

 `--limit 10` caps only the printed hits, and the count stays exact.

4. Run the same SMARTS over REST.

```sh
curl -s -X POST http://localhost:8080/v1/substructure \
  -H 'Content-Type: application/json' \
  -d '{"smarts":"S(=O)(=O)N","limit":10,"offset":0,"timeout_ms":10000,"db":"chembl"}'
```

 The fields are `smarts`, `limit`, `offset`, `timeout_ms` and `db`. Use `offset` with `limit` to page through hits.

5. Check the deadline behaviour. Lower `timeout_ms` (or pass `--timeout` on the CLI) until a query overruns it, and the reply is an error that carries no hits.

 Treat any error as "no answer". It's never "fewer hits".

The `substruct` command and the `/v1/substructure` call run the same search over the same molecules. The gRPC service is `mossflux.v1.Search`, and an MCP agent calls the same engine.

Run the aspirin and sulfonamide queries back to back. The aspirin query returns molecules ranked by exact score. The sulfonamide query returns every molecule containing `S(=O)(=O)N`, with the count a brute-force scan would give.

## What are the memory, time and request limits of a local ChEMBL search setup?

A local ChEMBL search with mossflux runs inside fixed limits on memory, build time, hits, concurrency, deadline and input size.

1. Memory: the [deploy documentation](https://mossflux.com/docs/deploy) says that on ChEMBL 37 (2,896,304 molecules) the index is 5.1 GB, about 2.5 KB of RAM per molecule. Size the machine from that per-molecule rate for the release you load.
2. Build time. The same load builds in 33 to 41 s. Point your health check at `/ready`.
3. Hits and concurrency: the [API documentation](https://mossflux.com/docs/api) says one request returns at most 10,000 hits. The server runs 4 concurrent searches by default, and further searches wait.
4. Deadline: the server default is 10 s and the maximum is 60 s. A search that passes its deadline returns an error and never a partial list. Set `timeout_ms` on `/v1/substructure` for queries you don't control.
5. Input size: the [security documentation](https://mossflux.com/docs/security) says SMILES may have up to 65,534 atoms and SMARTS up to 2,048 atoms. Malformed input returns an error with a byte position or line number.
6. Index shape: the [design documentation](https://mossflux.com/docs/design) says row ids are u32. There are no deletes and no sharding.
7. Fingerprints: a similarity query is a SMILES, and the server computes 2048-bit ECFP4-style fingerprints of its own. These bits are mossflux's own and are not RDKit-compatible, so scores from the two toolkits will differ.
8. Data preparation: mossflux does no salt stripping and no tautomer handling, and it parses chirality and ignores it. It reads no SDF and no InChI.

For deployment, the `deploy/` directory holds Docker Compose and Kubernetes manifests, and the `loruorg/mossflux` Docker image is x86-64 only.

## How do you connect an AI agent to ChEMBL search over MCP?

You connect an AI agent to ChEMBL search over MCP by pointing an MCP client at the `/mcp` endpoint of `mossflux serve`, or by launching `mossflux mcp` over stdio.

1. Pick a transport. The [Model Context Protocol specification](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports) defines two standard transports: stdio, "communication over standard in and standard out", and Streamable HTTP.
2. Add the server to your client. For the public ChEMBL 37 demo in Claude Code, run:

   ```sh
   claude mcp add --transport http mossflux https://api.mossflux.com/mcp
   ```

 For your own server, use the same form with your URL, such as `http://localhost:8080/mcp`, as your client's documentation describes. Start the client after the index has finished building.
3. Check the tool list. The agent should see five tools: `list_databases`, `similarity_search`, `substructure_search`, `molecule_properties` and `get_molecule`. Call `list_databases` first to confirm the database name, `chembl` if you started the server with `--db chembl=PATH`.
4. Call a tool. The client sends a standard MCP `tools/call` request. For aspirin, the arguments name the SMILES, a threshold, a hit limit and the database:

   ```json
   {
     "method": "tools/call",
     "params": {
       "name": "similarity_search",
       "arguments": {
         "smiles": "CC(=O)Oc1ccccc1C(=O)O",
         "threshold": 0.7,
         "limit": 5,
         "db": "chembl"
       }
     }
   }
   ```

 The result holds the hits with their exact Tanimoto scores, and the counts cover every molecule at or above the threshold. The [API reference](https://mossflux.com/docs/api) lists each response field.

In a local ChEMBL search setup, the agent, a `curl` script and a gRPC service get the same hits because the three transports share one engine. The API page says "mossflux serve answers the same searches over REST, MCP and gRPC."

To check, run the aspirin query over REST and compare the hit lists. A substructure call that passes its deadline returns an error over MCP too.

## Frequently asked questions

### How do I set up a local ChEMBL search?

Download the ChEMBL chemreps SMILES file from the EMBL-EBI FTP site, install mossflux with `cargo install --git`, and run `mossflux serve --db chembl=PATH`. The [mossflux API reference](https://mossflux.com/docs/api) says the server builds an in-memory index at start, then answers similarity and SMARTS substructure queries over REST, gRPC and MCP.

### Does a local search return every hit, or a partial list?

It returns every hit, because each result [matches a brute-force scan](https://mossflux.com/docs/). A substructure search that passes its deadline returns an error, never a partial list, and `--limit` caps only the printed hits, so the counts stay exact.

### Are the fingerprints compatible with RDKit?

No. The fingerprints are mossflux's own 2048-bit ECFP4-style bits and don't match RDKit or other toolkits. Send a SMILES and let the server compute its fingerprint, rather than comparing scores against RDKit output or loading RDKit bits.

### Do I need to remove salts and tautomers first?

Yes. mossflux doesn't strip salts or handle tautomers, so prepare a cleaned SMILES file before loading it.

### Does the index survive a restart?

No. The [deploy documentation](https://mossflux.com/docs/deploy) says the index lives in memory and is rebuilt at every start. On ChEMBL 37 (2,896,304 molecules) the build took 33 to 41 s, and `GET /ready` returns 503 until it finishes.

## Next steps for your local ChEMBL search setup

A local ChEMBL search setup can be running in one sitting, and you're free to add agents and services later.

- Today: download the chemreps SMILES file for a named ChEMBL release from the EMBL-EBI FTP site. Install the CLI with `cargo install --git https://github.com/loruorg/mossflux mossflux-cli`, then start `mossflux serve --db chembl=PATH`.
- This week: run the aspirin similarity query (`CC(=O)Oc1ccccc1C(=O)O`) and the sulfonamide substructure query (`S(=O)(=O)N`). Then check your hardware: [plan for about 2.5 KB of RAM per molecule](https://mossflux.com/docs/deploy), multiply your row count by that figure and compare it with free memory.
- Later: point an MCP client at `/mcp`, or put the REST service in front of your scripts. [One engine answers REST, MCP and gRPC](https://mossflux.com/docs/api), so a script, a service and an agent get the same hits.

[Run it on your own library](https://mossflux.com/#get).

## Sources

[CLI · mossflux docs](https://mossflux.com/docs/cli) · [API · mossflux docs](https://mossflux.com/docs/api) · [mossflux docs](https://mossflux.com/docs/) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [ChEMBL - ChEMBL](https://www.ebi.ac.uk/chembl/) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [Design · mossflux docs](https://mossflux.com/docs/design) · [Security · mossflux docs](https://mossflux.com/docs/security). Every other source is linked where the article cites it.
