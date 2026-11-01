---
description: A substructure search hard deadline bounds NP-complete subgraph matching. Past it, mossflux returns an error, never a partial list. Learn how to set one.
faq:
  - answer: A substructure search hard deadline is a fixed time limit on one query. Subgraph matching is NP-complete, so the limit stops runaway searches. In mossflux, a search that passes the deadline returns an error instead of a partial list. The API docs give the default and maximum.
    question: What is a substructure search hard deadline?
  - answer: In mossflux, a timed-out substructure search returns an error and no hits. A partial list looks like a finished answer, so a caller would read the missing molecules as non-matches. Retry the query with a narrower SMARTS or a longer timeout.
    question: Does a timed-out substructure search return partial results?
  - answer: In mossflux, a result limit is a different control from the substructure search hard deadline. It caps the hits returned or printed and never changes the count, while the deadline bounds the amount of work a single query may do.
    question: Is a result limit the same as a timeout?
  - answer: A short SMARTS can be slow because query length does not set the cost. Substructure search is subgraph matching, and a short query can still force exponentially many atom mappings on one molecule. The path screen is a superset filter that never drops a match, so most queries reach the exponential VF2 verify step with few candidates.
    question: Why can a short SMARTS be slow?
  - answer: mossflux never approximates a search to meet a deadline. It is exact only, with no machine learning and no approximation. Every answer is what a brute-force scan returns, and a search that runs out of time returns an error instead of fewer hits.
    question: Does mossflux ever approximate a search to meet a deadline?
image: /blog/substructure-search-hard-deadline/featured-c7a53939f4ba.webp
image_alt: "Illustration for Substructure search hard deadline: How to set"
publishDate: "2026-11-01"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/substructure-search-hard-deadline#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\",\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q334640\",\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]}],\"datePublished\":\"2026-11-01\",\"description\":\"A substructure search hard deadline bounds NP-complete subgraph matching. Past it, mossflux returns an error, never a partial list. Learn how to set one.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A substructure search hard deadline is a fixed time limit on one query. Subgraph matching is NP-complete, so the limit stops runaway searches. In mossflux, a search that passes the deadline returns an error instead of a partial list. The API docs give the default and maximum.\"},\"name\":\"What is a substructure search hard deadline?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"In mossflux, a timed-out substructure search returns an error and no hits. A partial list looks like a finished answer, so a caller would read the missing molecules as non-matches. Retry the query with a narrower SMARTS or a longer timeout.\"},\"name\":\"Does a timed-out substructure search return partial results?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"In mossflux, a result limit is a different control from the substructure search hard deadline. It caps the hits returned or printed and never changes the count, while the deadline bounds the amount of work a single query may do.\"},\"name\":\"Is a result limit the same as a timeout?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A short SMARTS can be slow because query length does not set the cost. Substructure search is subgraph matching, and a short query can still force exponentially many atom mappings on one molecule. The path screen is a superset filter that never drops a match, so most queries reach the exponential VF2 verify step with few candidates.\"},\"name\":\"Why can a short SMARTS be slow?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux never approximates a search to meet a deadline. It is exact only, with no machine learning and no approximation. Every answer is what a brute-force scan returns, and a search that runs out of time returns an error instead of fewer hits.\"},\"name\":\"Does mossflux ever approximate a search to meet a deadline?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\",\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\",\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q6120337\",\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q131476\",\"@type\":\"Thing\",\"description\":\"study of graphs, which are mathematical structures used to model pairwise relations between objects\",\"name\":\"Graph theory\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Graph_theory\",\"https://www.wikidata.org/wiki/Q131476\",\"https://www.google.com/search?kgmid=/m/0382k\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q52\",\"@type\":\"Thing\",\"description\":\"free multilingual online encyclopedia\",\"name\":\"Wikipedia\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Wikipedia\",\"https://www.wikidata.org/wiki/Q52\",\"https://www.google.com/search?kgmid=/m/0d07ph\"]}],\"name\":\"Substructure search hard deadline: How to set\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/NkXjcVA0aJZvz9CqQyjtLP1imebFFoXO.webp\"},\"url\":\"https://mossflux.com/blog/substructure-search-hard-deadline\"},{\"@id\":\"https://mossflux.com/blog/substructure-search-hard-deadline#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\"},{\"@id\":\"https://www.wikidata.org/wiki/Q334640\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Substructure search - Wikipedia\",\"url\":\"https://en.wikipedia.org/wiki/Substructure_search\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Substructure Searching with Indexes — Open Babel\",\"url\":\"https://openbabel.org/docs/Cheminf101/search.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Daylight Theory Manual: SMARTS\",\"url\":\"https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"loruorg/mossflux\",\"url\":\"https://mossflux.com/#get\"}],\"dateModified\":\"2026-11-01\",\"datePublished\":\"2026-11-01\",\"description\":\"A substructure search hard deadline bounds NP-complete subgraph matching. Past it, mossflux returns an error, never a partial list. Learn how to set one.\",\"headline\":\"Substructure search hard deadline: How to set\",\"image\":\"https://api.birchseek.com/m/NkXjcVA0aJZvz9CqQyjtLP1imebFFoXO.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/substructure-search-hard-deadline#webpage\"},\"keywords\":[\"substructure search hard deadline\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/substructure-search-hard-deadline#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\"},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\"},{\"@id\":\"https://www.wikidata.org/wiki/Q6120337\"},{\"@id\":\"https://www.wikidata.org/wiki/Q131476\"},{\"@id\":\"https://www.wikidata.org/wiki/Q52\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2170},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"statistical Markov model\",\"name\":\"Hidden Markov model\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hidden_Markov_model\",\"https://www.wikidata.org/wiki/Q176769\",\"https://www.google.com/search?kgmid=/m/0p8wy\"]},{\"@type\":\"Thing\",\"description\":\"specialized artificial neural networks that are designed for tasks whose inputs are graphs\",\"name\":\"Graph neural network\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Graph_neural_network\",\"https://www.wikidata.org/wiki/Q107750691\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"chemical compound\",\"name\":\"Histidine\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Histidine\",\"https://www.wikidata.org/wiki/Q485277\",\"https://www.google.com/search?kgmid=/m/0h1tz\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - substructure search
  - smarts
  - cheminformatics
  - timeouts
  - subgraph isomorphism
  - mossflux
title: "Substructure search hard deadline: How to set"
---

**A substructure search hard deadline is required because the search is [subgraph isomorphism](https://en.wikipedia.org/wiki/Substructure_search) (a query graph inside a target graph), which is NP-complete, so a short query can take exponential time. Past the deadline, mossflux returns an error, never a partial list, as the [mossflux CLI docs](https://mossflux.com/docs/cli) state, because a truncated list looks like a complete answer.**

The problem belongs to cheminformatics engineers, platform engineers who serve SMARTS search over an API, and builders of AI agents with a chemistry tool. One caller sends a query that never finishes, and it occupies the service.

After reading, you can choose a deadline, read a timeout correctly, and keep result limits, deadlines and input-size limits apart. The examples use mossflux, documented in the [mossflux docs](https://mossflux.com/docs/).

> **Key Takeaways**
> - A result limit caps hits, a deadline bounds work, and size limits reject oversized queries.
> - The server deadline defaults to 10 s, with a 60 s maximum.
> - After a deadline error, an agent can retry with a narrower SMARTS.

## Why does substructure search need a hard deadline?

Substructure search needs a hard deadline because subgraph matching is NP-complete. A short query graph can force a matcher to try exponentially many atom mappings against one target molecule, and a deadline bounds that work.

![An illustration of a geometric machine analyzing a molecular structure with data labels and gears.](/blog/substructure-search-hard-deadline/image-1-349f82672f0c.webp)

[Wikipedia](https://en.wikipedia.org/wiki/Substructure_search) defines substructure search as "an application of graph theory, specifically subgraph matching in which the query is a hydrogen-depleted molecular graph." The matcher must find the query graph inside each target graph.

Query length alone doesn't predict cost. The sulfonamide pattern `S(=O)(=O)N` has four atoms, but NP-completeness is a worst-case statement about the problem, not about the length of the query.

If you accept SMARTS from callers you don't control, a short string is no safer than a long one. Each query needs a bound on its work: a substructure search hard deadline.

[Open Babel's indexing page](https://openbabel.org/docs/Cheminf101/search.html) describes indexing as "pre-computing the answers to portions of expected questions before they’re asked, so that when the question comes, it can be answered quickly." In substructure search, the screen fills that role.

mossflux runs a path screen first, then ends with VF2 verify, the exact matcher.

## What happens at the substructure search hard deadline?

When a substructure search reaches its deadline, mossflux returns an error and no hits, because a partial hit list looks exactly like a finished answer.

If a matcher stopped partway through a database, every row it had returned would still be a true match, so the list would pass any spot check. The caller couldn't tell a molecule that truly fails to match from one the matcher never reached.

A script that reads absence as "no match here" would then draw a wrong conclusion without any warning. The only safe answer is an error.

The caller learns that the search didn't finish and holds no list to mistake for a complete one.

The error rule for a substructure search hard deadline follows from exactness. The [correctness docs](https://mossflux.com/docs/correctness) describe what "exact" means in mossflux and the tests that prove it.

The [mossflux API docs](https://mossflux.com/docs/api) say "mossflux serve answers the same searches over REST, MCP and gRPC," and one engine sits behind all three, so the deadline error cannot differ between them. An AI agent never receives a truncated list on any transport.

On that error, the agent can narrow the SMARTS and retry. Moving from `S(=O)(=O)N` to `c1ccccc1S(=O)(=O)N` adds an aromatic ring and gives the matcher a more specific pattern. It's a new, complete search.

## Result limit, deadline or input-size limit: what does each control?

A result limit, a deadline and an input-size limit each bound a different thing, and none can stand in for another.

| Control | What it bounds | What it never changes | What the caller sees |
| --- | --- | --- | --- |
| Result limit, CLI `--limit` | Hits printed | The count | Exact count, fewer rows |
| Result limit, REST `limit` and `offset` | Hits returned per request | Which molecules match | One page of hits |
| Deadline, `timeout_ms` or `--timeout` | Time spent matching | The answer: whole or an error | An error, no hits |
| Input-size limits | Query and request size | Answers to valid queries | An error with a position |
| Concurrency cap | Searches running at once | Any answer | Waiting requests |

The result limit controls output only. The [mossflux API docs](https://mossflux.com/docs/api) say counts are always exact and `--limit` caps only the printed hits. With `--limit 20`, the CLI prints at most 20 rows and still reports the true number of matches.

On `POST /v1/substructure`, `limit` and `offset` page through the hits. Each request is capped at 10,000 hits.

The deadline controls work, and it's the substructure search hard deadline that each caller sets. `timeout_ms` on `POST /v1/substructure` and `--timeout` on the CLI set it, and the server default is 10 s with a maximum of 60 s.

Input-size limits act before matching starts. The [mossflux security docs](https://mossflux.com/docs/security) set them: SMARTS may have up to 2,048 atoms, a query may be up to 4,096 bytes, and a request body may be up to 64 KiB. SMILES may have up to 65,534 atoms.

The API docs list each field and limit.

The server also runs 4 concurrent searches by default, and more requests wait. That cap protects the machine and changes no result.

Use the result limit to keep output readable while the count stays exact. Put a substructure search hard deadline on any query you don't control, such as public API or agent traffic.

Input-size limits refuse malformed or oversized queries before they cost any matching time. The concurrency cap keeps one shared server responsive under parallel callers.

## How does screen-then-verify keep queries away from the deadline?

Screen-then-verify keeps queries away from the deadline because four cheap stages shrink the candidate set before the one exponential stage, VF2 verify, runs. The [mossflux design docs](https://mossflux.com/docs/design) list the pipeline in order:

1. Parse the SMARTS or SMILES.
2. Perceive rings and aromaticity, using the project's own Huckel model.
3. Path screen, a superset filter that never drops a match.
4. Census over 32 count classes. Exact classes answer without the matcher.
5. VF2 verify on the survivors.

A superset filter can't create a false negative. It rejects a molecule only when the molecule lacks something every match must have, so each rejected molecule is a proven non-match.

The filter may let non-matches through, and verification removes those.

Parsing, perception, screening and counting cost work that grows with the size of the input. VF2 verify, the subgraph matcher, is the stage where one short query can blow up, and it sees only the molecules that passed the earlier stages.

The query language is [Daylight SMARTS](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html). mossflux supports recursive `$(...)` to depth 10, plus the primitives `v<n>`, `X<n>`, `D` and `H`.

The [deploy docs](https://mossflux.com/docs/deploy) say the demo loads ChEMBL 37 as 2,896,304 molecules. Against that set, a molecule without a sulfur atom can't contain `S(=O)(=O)N`, so the earlier stages can discard it without running VF2.

The queries that hit a substructure search hard deadline are the ones where many molecules survive and the matcher still has a large search to do.

The [correctness docs](https://mossflux.com/docs/correctness) describe two tests of exactness. One is a brute-force oracle (`brute.rs`). The other is a differential against RDKit, where 114 of 114 queries agree.

The [benchmarks page](https://mossflux.com/docs/benchmarks) reports substructure throughput of 5.2 to 160 million molecules a second per thread over a 17-query battery, 2M ChEMBL rows, 8 threads, an EPYC 9645 VM, median of 3.

Those timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on it. No billion-scale substructure claim is made.

## How do you run a SMARTS substructure search with a deadline?

You run a SMARTS substructure search with a deadline by setting a timeout on the CLI, in the REST body or through the MCP tool, and reading a timeout error as "answer unknown". The target here is aspirin `CC(=O)Oc1ccccc1C(=O)O` and the query is the sulfonamide `S(=O)(=O)N`.

1. Install the CLI from source with Rust 1.95 or later.

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

There are no prebuilt binaries and no Python bindings.

2. Search a SMILES file with `--timeout` and `--limit`. The [mossflux CLI docs](https://mossflux.com/docs/cli) give the argument order and the unit of `--timeout`.

```sh
echo 'CC(=O)Oc1ccccc1C(=O)O' > aspirin.smi
mossflux substruct aspirin.smi 'S(=O)(=O)N' --limit 10 --timeout 10
```

`--limit` caps only the printed hits, never the count.

3. Call the REST endpoint. List database names first with `GET /v1/databases`.

```sh
curl -s https://api.mossflux.com/v1/databases
curl -s https://api.mossflux.com/v1/substructure \
  -H 'Content-Type: application/json' \
  -d '{"smarts":"S(=O)(=O)N","limit":10,"offset":0,"timeout_ms":10000,"db":"your_database"}'
```

If a search passes `timeout_ms`, retry with a narrower SMARTS.

4. Call the MCP tool. Register the server in Claude Code, then ask the agent to run `substructure_search` with the same SMARTS.

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

Aspirin has no sulfonamide, so `S(=O)(=O)N` against `aspirin.smi` returns zero hits. That zero is exact: the search finished and found nothing.

A deadline error means something different: the answer's unknown, and a caller mustn't read it as zero.

Three limits apply to every search. mossflux does no stereochemistry-aware matching, no salt stripping and no tautomer handling, so canonicalise your SMILES before ingestion.

To run the REST and MCP steps without installing anything, [try the demo](https://mossflux.com/#demo).

## How long should a substructure search hard deadline be?

The substructure search deadline should follow who controls the query: long for your own scripts, short for every caller you don't control.

The [mossflux API docs](https://mossflux.com/docs/api) list `timeout_ms` as a field on `POST /v1/substructure`, with a server default of 10 s and a maximum of 60 s.

1. Give your own scripts and batch jobs up to the maximum. You wrote the SMARTS and can wait, so set `timeout_ms` to 60000, or leave the 10 s default if your queries are simple. A long deadline holds one of the 4 default concurrent search slots for longer while further searches wait, so don't run a large batch beside a shared service.
2. Give untrusted callers a deadline below the 10 s default. For any API, MCP or agent caller, set `timeout_ms` to a small fraction of the default, for example 2000:

   ```sh
   curl -s https://api.mossflux.com/v1/substructure \
     -H 'content-type: application/json' \
     -d '{"smarts":"S(=O)(=O)N","limit":100,"timeout_ms":2000}'
   ```

 On the command line, the [CLI docs](https://mossflux.com/docs/cli) say to set `--timeout` for queries you do not control.
3. Bound the input as well. The API docs list 4,096 bytes per query, so a deadline never has to stop an oversized pattern.
4. Treat a deadline error as unknown, not empty. The search didn't finish, so zero hits and "no match" are different answers. Narrow the SMARTS by adding atoms or constraints such as `X3` or `D2`, then retry, and don't raise the deadline first for a caller you can't trust.

## Frequently asked questions

### What is a substructure search hard deadline?

A substructure search hard deadline is a fixed time limit on one query. Subgraph matching is NP-complete, so the limit stops runaway searches. The [API docs](https://mossflux.com/docs/api) give the default and maximum.

### Does a timed-out substructure search return partial results?

A timed-out search returns an error and no hits. A partial list looks like a finished answer, so a caller would read the missing molecules as non-matches. Retry with a narrower SMARTS or a longer timeout.

### Is a result limit the same as a timeout?

A result limit is a different control. It caps the hits returned or printed and never changes the count, while the deadline bounds the work a query may do.

### Why can a short SMARTS be slow?

Query length doesn't set the cost. Substructure search is subgraph matching, and a short query can still force exponentially many atom mappings on one molecule.

### Does mossflux ever approximate a search to meet a deadline?

mossflux is exact only, with no machine learning and no approximation. Every answer is what a brute-force scan returns, and a search that runs out of time returns an error instead of fewer hits.

## Next steps: set a deadline on every substructure search

Setting a substructure search hard deadline takes three steps, in this order.

**Today:** Open the [mossflux docs](https://mossflux.com/docs/), read the API and CLI pages, and set `timeout_ms` (REST) or `--timeout` (CLI) on every query you don't control.

**This week:** Handle the result limit, the deadline and the input-size check as three separate settings in your service. Make every client read a deadline error as "unknown", never "empty", and let agents retry with a narrower SMARTS.

**Later:** Read the [correctness invariants](https://mossflux.com/docs/correctness) and check your own queries against them.

Run it on your own library: [install mossflux](https://mossflux.com/#get) and point `mossflux substruct` at your SMILES file with `S(=O)(=O)N`. It's free and open source under MIT OR Apache-2.0, so a trial costs you nothing.

## Sources

[Substructure search - Wikipedia](https://en.wikipedia.org/wiki/Substructure_search) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [API · mossflux docs](https://mossflux.com/docs/api) · [Substructure Searching with Indexes — Open Babel](https://openbabel.org/docs/Cheminf101/search.html) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [Daylight Theory Manual: SMARTS](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks). Every other source is linked where the article cites it.
