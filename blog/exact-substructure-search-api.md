---
description: How does an exact substructure search API work? It matches a brute-force scan row for row and errors past its deadline. Learn the POST call and its limits.
faq:
  - answer: An exact substructure search API is a service that takes a SMARTS query and returns every molecule containing that substructure, matching a brute-force scan row for row. mossflux exposes this as POST /v1/substructure over REST, with the same engine behind gRPC and the MCP substructure_search tool.
    question: What is an exact substructure search API?
  - answer: The mossflux substructure search API does not return partial results on timeout. When a query runs out of time it returns an error and never a partial list of molecules. The default deadline is 10 s and the maximum deadline is 60 s.
    question: Does a substructure search API return partial results on timeout?
  - answer: The mossflux exact substructure search API handles none of stereochemistry, salts or tautomers. Chirality is parsed and ignored, so it plays no part in matching. There is no salt stripping and no tautomer handling, so canonicalise your SMILES before ingestion.
    question: Does mossflux handle stereochemistry, salts or tautomers?
  - answer: "The index needs about 2.5 KB of RAM per molecule, so size your server memory around that figure. The server builds the index in memory at start and doesn't save it to disk, which means the index lives only in RAM."
    question: How much memory does the index need?
  - answer: Yes, mossflux runs a public demo at api.mossflux.com that you can try over REST and MCP. The demo runs over ChEMBL 37. The API docs at mossflux.com/docs/api list the endpoints, fields and limits you need to send queries to the demo.
    question: Is there a public API I can try?
image: /blog/exact-substructure-search-api/featured-dfbeb0bf3697.webp
image_alt: A computational chemist interacts with screens displaying molecular structures.
publishDate: "2026-11-08"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/exact-substructure-search-api#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q165194\",\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\",\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]}],\"datePublished\":\"2026-11-08\",\"description\":\"How does an exact substructure search API work? It matches a brute-force scan row for row and errors past its deadline. Learn the POST call and its limits.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"An exact substructure search API is a service that takes a SMARTS query and returns every molecule containing that substructure, matching a brute-force scan row for row. mossflux exposes this as POST /v1/substructure over REST, with the same engine behind gRPC and the MCP substructure_search tool.\"},\"name\":\"What is an exact substructure search API?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The mossflux substructure search API does not return partial results on timeout. When a query runs out of time it returns an error and never a partial list of molecules. The default deadline is 10 s and the maximum deadline is 60 s.\"},\"name\":\"Does a substructure search API return partial results on timeout?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The mossflux exact substructure search API handles none of stereochemistry, salts or tautomers. Chirality is parsed and ignored, so it plays no part in matching. There is no salt stripping and no tautomer handling, so canonicalise your SMILES before ingestion.\"},\"name\":\"Does mossflux handle stereochemistry, salts or tautomers?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The index needs about 2.5 KB of RAM per molecule, so size your server memory around that figure. The server builds the index in memory at start and doesn't save it to disk, which means the index lives only in RAM.\"},\"name\":\"How much memory does the index need?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Yes, mossflux runs a public demo at api.mossflux.com that you can try over REST and MCP. The demo runs over ChEMBL 37. The API docs at mossflux.com/docs/api list the endpoints, fields and limits you need to send queries to the demo.\"},\"name\":\"Is there a public API I can try?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q7544680\",\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\",\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q210390\",\"@type\":\"Thing\",\"description\":\"phenomenon providing chemical stability in resonating hybrids of cyclic organic compounds\",\"name\":\"Aromaticity\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Aromaticity\",\"https://www.wikidata.org/wiki/Q210390\",\"https://www.google.com/search?kgmid=/m/01jsm7\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q364\",\"@type\":\"Organization\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q2063\",\"@type\":\"Thing\",\"description\":\"text-based open standard designed for human-readable data interchange\",\"name\":\"JSON\",\"sameAs\":[\"https://en.wikipedia.org/wiki/JSON\",\"https://www.wikidata.org/wiki/Q2063\",\"https://www.google.com/search?kgmid=/m/05cntt\"]}],\"name\":\"Exact substructure search API: How to call it\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/lyX-eK6rbesxhZagIbZZXnmXvCumcN96.webp\"},\"url\":\"https://mossflux.com/blog/exact-substructure-search-api\"},{\"@id\":\"https://mossflux.com/blog/exact-substructure-search-api#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q165194\"},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Subgraph isomorphism problem\",\"url\":\"https://en.wikipedia.org/wiki/Subgraph_isomorphism_problem\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Daylight Theory: SMARTS - A Language for Describing Molecular Patterns\",\"url\":\"https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html\"},{\"@type\":\"CreativeWork\",\"name\":\"rdkit.Chem.rdchem module · RDKit documentation\",\"url\":\"https://www.rdkit.org/docs/source/rdkit.Chem.rdchem.html\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/#get\"}],\"dateModified\":\"2026-11-08\",\"datePublished\":\"2026-11-08\",\"description\":\"How does an exact substructure search API work? It matches a brute-force scan row for row and errors past its deadline. Learn the POST call and its limits.\",\"headline\":\"Exact substructure search API: How to call it\",\"image\":\"https://api.birchseek.com/m/lyX-eK6rbesxhZagIbZZXnmXvCumcN96.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/exact-substructure-search-api#webpage\"},\"keywords\":[\"exact substructure search api\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/exact-substructure-search-api#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"},{\"@id\":\"https://www.wikidata.org/wiki/Q7544680\"},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\"},{\"@id\":\"https://www.wikidata.org/wiki/Q210390\"},{\"@id\":\"https://www.wikidata.org/wiki/Q364\"},{\"@id\":\"https://www.wikidata.org/wiki/Q2063\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2142},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"hardware direct memory access over computer networks\",\"name\":\"Remote direct memory access\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Remote_direct_memory_access\",\"https://www.wikidata.org/wiki/Q2561570\",\"https://www.google.com/search?kgmid=/m/04599j\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - substructure search
  - smarts
  - rest api
  - cheminformatics
  - mossflux
title: "Exact substructure search API: How to call it"
---

**An [exact substructure search API](https://mossflux.com/docs/) takes a SMARTS query and returns the same rows as a brute-force scan, with no false negatives and no false positives. mossflux exposes this as `POST /v1/substructure`, and a search that passes [its deadline](https://mossflux.com/docs/cli) returns an error, never a partial list.

This guide is for anyone who has put a compound library behind an exact substructure search API, with users sending SMARTS queries that need every true hit, however long the result list runs.

Every field and limit here comes from the [mossflux API docs](https://mossflux.com/docs/api) for the open-source mossflux engine. You'll be able to send a SMARTS query, page through the reply and know what the engine ignores.

> **Key Takeaways**
> - In the RDKit differential, 114 of 114 queries agree, under the conditions the [correctness docs](https://mossflux.com/docs/correctness) state.
> - [The index](https://mossflux.com/docs/deploy) costs about 2.5 KB of RAM per molecule and is not persisted to disk, so every start rebuilds it.
> - The server deadline defaults to 10 s and tops out at 60 s; past it the answer is an error.

## What does 'exact' mean in an exact substructure search API?

In an exact substructure search API, exact means the result equals what a brute-force scan returns after testing every molecule against the query: no false negatives and no false positives. Speed-ups such as a screen may only discard molecules that provably cannot match, so the fast path and the slow path give identical answers.

![Illustration for What does 'exact' mean in an exact substructure search API?](/blog/exact-substructure-search-api/image-1-a5ee17d167b8.webp)

Take aspirin, `CC(=O)Oc1ccccc1C(=O)O`, as the query. A brute-force scan runs the matcher on every row of the database and keeps each hit. An exact engine must return that identical set of rows, however many shortcuts it takes on the way.

In an exact substructure search API, a screen is a cheap filter that runs before the matcher. If it drops one molecule that would have matched, that molecule is a false negative. Nothing in the result shows that the row's missing.

mossflux makes the claim checkable in three ways:

- mossflux's [correctness documentation](https://mossflux.com/docs/correctness) states the invariant that a prune rejects only on a proven bound.
- The test file `brute.rs` is a brute-force oracle: the slow reference that the fast path is compared with.
- A differential against RDKit compares the two tools query by query, under the conditions the correctness page states.

Agreement with RDKit on these queries doesn't tell you anything about shared fingerprint bits. [mossflux's fingerprints](https://mossflux.com/docs/api) are its own and are not RDKit-compatible bit for bit.

## How do you call an exact substructure search API?

You call an exact substructure search API by sending a POST request with a SMARTS string and paging fields, and the sulfonamide query `S(=O)(=O)N` against the public mossflux demo shows each step.

1. Check the databases. Run `curl -s https://api.mossflux.com/v1/databases`. The reply names each database the server holds, and the request body's `db` field picks one by that name.

2. Write the request body. The [mossflux API reference](https://mossflux.com/docs/api) lists the `POST /v1/substructure` fields: `smarts`, `limit`, `offset`, `timeout_ms` and `db`. A first page of 5 hits looks like this:

```json
{
  "smarts": "S(=O)(=O)N",
  "limit": 5,
  "offset": 0,
  "timeout_ms": 5000
}
```

3. Send it from a shell:

```sh
curl -s -X POST https://api.mossflux.com/v1/substructure \
  -H 'Content-Type: application/json' \
  -d '{"smarts":"S(=O)(=O)N","limit":5,"offset":0,"timeout_ms":5000}' | jq .
```

4. Read the reply. The response is JSON, and `jq .` prints it indented. Check its field names against the API reference before you write a parser. A malformed SMARTS comes back as an error, so fix the query string and send it again.

5. Page through the results. Send the same body with `"offset": 5`, then `10`, raising `offset` by the value of `limit` each time. Keep `smarts` identical across pages, because `offset` and `limit` slice one result set; a changed query gives you pages of different searches.

6. Set a deadline for queries you don't control. `timeout_ms` bounds the search time in milliseconds, and some queries take exponential time to match.

[Try the demo](https://mossflux.com/#demo) on the page, or run the same `curl` commands from your terminal.

## How does an exact substructure search API answer a query, stage by stage?

An exact substructure search answers a query in five stages (parse, perceive, path screen, census and VF2 verify), and none of them can change which rows come back. The [mossflux design page](https://mossflux.com/docs/design) lists this pipeline and notes that the parsers, perception, screen and matcher are all written in the project.

| Stage | What it does | Can it change the answer? |
|---|---|---|
| Parse | Reads the SMARTS (a SMILES is also valid SMARTS) into a query graph | No |
| Perceive | Finds rings and aromaticity with the project's own Hückel model | No |
| Path screen | Discards molecules that cannot contain the query's paths | No |
| Census | Compares 32 count classes; exact classes answer without the matcher | No |
| VF2 verify | Tests the survivors for a true subgraph match | No |

The naive approach runs a matcher over every molecule in turn. Subgraph isomorphism is [NP-complete](https://en.wikipedia.org/wiki/Subgraph_isomorphism_problem), so one hard pair can cost exponential time, and a loop pays that risk on millions of rows.

A screen is safe only when it is a superset filter. If it dropped one true match, the result would have a false negative. If it keeps extra molecules, VF2 verify removes them, so the answer stays the same as a brute-force scan.

Follow `S(=O)(=O)N` through the stages. It parses as SMARTS, and a molecule with no sulfur atom can't match, so the screen and census remove it before any matching starts. Only the survivors reach VF2 verify.

The [benchmarks page](https://mossflux.com/docs/benchmarks) reports 5.2 to 160 million molecules a second per thread, over a 17-query battery on 2M ChEMBL rows, with 8 threads on an EPYC 9645 VM, median of 3.

The timings predate the 2026-10-03 hardening pass and were not re-measured on the new code, while the exactness gates pass on it. No billion-scale substructure claim is made.

## How does SMARTS work in a substructure search request?

A SMARTS query in a substructure search request is a pattern of atoms and bonds that a molecule must contain, and the engine tests each row for it. A plain SMILES also works, because the engine treats a SMILES as a valid SMARTS.

Atom primitives sit inside square brackets, and bond primitives such as `-`, `=`, `#` and `~` sit between atoms. Logical operators combine primitives: `!` is not, `&` and `,` are and/or at high precedence, and `;` is a low-precedence and.

The query `[N;X3;!$(NC=O)]` reads as a nitrogen with three connections that is not an amide nitrogen.

The `$(...)` part is a recursive SMARTS. The manual says "Any SMARTS expression may be used to define an atomic environment by writing a SMARTS starting with the atom in question in this form: $(SMARTS)." The [mossflux engine](https://mossflux.com/docs/api) accepts recursion to depth 10.

| SMARTS feature | In a mossflux request |
|---|---|
| SMILES used as SMARTS | Supported |
| Recursive `$(...)` | Supported to depth 10 |
| `v<n>`, `X<n>`, `D`, `H` | Supported |
| Atom maps | Parsed, then ignored |
| Chirality | Parsed, then ignored |

Ignored chirality means `C[C@H](N)C(=O)O` and `C[C@@H](N)C(=O)O` match the same rows. The [RDKit Python API](https://www.rdkit.org/docs/source/rdkit.Chem.rdchem.html) lists `useChirality=False` in its `HasSubstructMatch` signature, so a caller there turns stereo matching on per call. Check that setting before you compare results across tools.

## Why does an exact substructure search API need a deadline, and what are its limits?

Substructure search needs a deadline because a short SMARTS query can take exponential time to match, and a deadline caps the work any one request can demand from the server.

A request to `POST /v1/substructure` can carry a [`timeout_ms` field](https://mossflux.com/docs/api). The server default is 10 s and the maximum is 60 s. A search that passes its deadline returns an error, never a partial list.

A partial list is unsafe because the caller can't tell a complete answer from a truncated one: a list of any length might be everything, or only what was found before the clock ran out. After an error, the caller can narrow the SMARTS, or raise `timeout_ms` (up to 60 s) and ask again.

The other limits work the same way: they refuse a request or make it wait, and never change an answer. Per request, mossflux returns at most 10,000 hits and runs 4 concurrent searches by default; more wait. A search that waits for a free slot returns the same rows it would have returned at once.

Because the index lives in memory, sizing is a memory question. The [deployment guide](https://mossflux.com/docs/deploy) covers Docker, Compose and Kubernetes setup and sizing: ChEMBL 37 (2,896,304 molecules) builds in 33 to 41 s into a 5.1 GB index.

`GET /ready` returns 503 until every database is loaded, so point your health check there.

Some things aren't offered: a persisted index, deletes and sharding. Row ids are u32. Because every start rebuilds the index, each restart costs that build time again.

## What does substructure search not do with stereochemistry, salts and tautomers?

mossflux's substructure search does not match on stereochemistry, strip salts or normalise tautomers, so the library has to be cleaned before ingestion. Here's what you do about each gap.

1. Chirality is read and ignored. The [mossflux design notes](https://mossflux.com/docs/design) say that atom maps and chirality parse and are ignored. A query written with `@` or `@@` is accepted, and both enantiomers in a row set match it. If you need one enantiomer, filter the returned rows in your own code.

2. Salts aren't stripped. A counter-ion in a row's SMILES, such as `[Na+]` in `CC(=O)Oc1ccccc1C(=O)[O-].[Na+]`, is part of that row as given. Remove counter-ions from the library file before you load it.

3. Tautomers aren't handled. A keto form and its enol form are different rows with different graphs, and a query drawn for one form won't match the other. Pick one tautomer convention and apply it to the library and to every query.

4. Input is SMILES and SMARTS only, so convert other formats to SMILES before you load them.

5. Canonicalisation belongs to the caller. Run your toolkit's standardisation once over the library, then store the result as the SMILES file the server loads.

## Can an AI agent call the same exact substructure search API?

| Transport | Call |
|---|---|
| REST | `POST /v1/substructure` |
| gRPC | service `mossflux.v1.Search`, method `Substructure` |
| MCP | tool `substructure_search` |

To register the public demo in Claude Code, run the command from the docs:

```sh
claude mcp add --transport http mossflux https://api.mossflux.com/mcp
```

The agent then sends a SMARTS such as `S(=O)(=O)N` as a tool call, the way a script posts it to REST.

Because one engine answers all three transports, exactness, deadline and limit behaviour are the same for an agent as for a script, and an agent never receives a truncated set that looks complete.

## Frequently asked questions

### What is an exact substructure search API?

An exact substructure search API is a service that takes a SMARTS query and returns every molecule containing that substructure, matching a brute-force scan row for row. mossflux exposes this as POST /v1/substructure over REST, with the same engine behind gRPC and the MCP substructure_search tool.

### Does a substructure search API return partial results on timeout?

No: [mossflux returns an error](https://mossflux.com/docs/design), never a partial list. The default deadline is 10 s and the maximum is 60 s.

### Does mossflux handle stereochemistry, salts or tautomers?

mossflux handles none of them. Chirality is parsed and ignored, and there is no salt stripping or tautomer handling, so canonicalise your SMILES before ingestion.

### How much memory does the index need?

[Plan for about 2.5 KB](https://mossflux.com/docs/deploy) of RAM per molecule. The server builds the index in memory at start and doesn't save it to disk.

### Is there a public API I can try?

The demo over ChEMBL 37 runs at api.mossflux.com, over REST and MCP. The [API docs](https://mossflux.com/docs/api) list the endpoints, fields and limits.

## Next steps: try the call today, then run it on your library

**Today:** send the sulfonamide query `S(=O)(=O)N` to the demo at `https://api.mossflux.com`, with the [API reference](https://mossflux.com/docs/api) open for the request fields.

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

The [Docker image](https://mossflux.com/docs/deploy) `loruorg/mossflux` is x86-64 only.

Then run `mossflux serve --db NAME=PATH`, with PATH set to your SMILES file.

**Then:** set `timeout_ms` for queries you don't control. [Past the deadline](https://mossflux.com/docs/design) the answer is an error, never a partial list.

[Run it on your library](https://mossflux.com/#get). mossflux is free and open source under MIT OR Apache-2.0, with no paid plan, account or hosted service, so a trial costs you nothing.

## Sources

[mossflux docs](https://mossflux.com/docs/) · [API · mossflux docs](https://mossflux.com/docs/api) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [Design · mossflux docs](https://mossflux.com/docs/design) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks) · [Daylight Theory: SMARTS - A Language for Describing…](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html). Every other source is linked where the article cites it.
