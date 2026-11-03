---
description: "What is an exact similarity search guarantee? The result set equals a brute-force scan: every qualifying row returned, none extra. Learn how to test it."
faq:
  - answer: An exact similarity search guarantee does not require a full scan, because exact means the result set equals what a brute-force scan returns. A prune may skip a row only when a proven bound shows it cannot meet the threshold. Pruning changes how much work is done while the answer stays the same.
    question: Does an exact similarity search guarantee mean a full scan?
  - answer: "Exact search is not always slower than approximate search, because pruning cuts comparisons without changing the answer. Any speed claim needs its conditions: machine, dataset, fingerprint width, threshold, threads and trials. The benchmarks page lists each run together with all of those conditions."
    question: Is exact search always slower than approximate search?
  - answer: Run a brute-force scan beside the fast path on every query and fail on any difference in rows or scores. Recall is the wrong metric, because exact means no missing and no extra rows. The correctness page lists the tests behind the claim.
    question: How do I check that my similarity search is really exact?
  - answer: A score of 1.0 means the two fingerprints have equal bits, so the compounds behind them can still be different compounds. Which structures end up sharing bits depends on the fingerprint scheme. Salts, tautomers and stereo all need canonicalising before ingestion.
    question: Does score 1.0 mean two molecules are identical?
  - answer: Setting a limit never changes the answer to the query. The limit caps how many hits you receive. A capped list is a cut of the exact result, which means it is never a different list from the one the same query returns without a limit.
    question: What happens to the answer if I set a limit?
image: /blog/exact-similarity-search-guarantee/featured-411e45d0b1a3.webp
image_alt: "Illustration for Exact similarity search guarantee: How to verify"
publishDate: "2026-11-03"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/exact-similarity-search-guarantee#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"datePublished\":\"2026-11-03\",\"description\":\"What is an exact similarity search guarantee? The result set equals a brute-force scan: every qualifying row returned, none extra. Learn how to test it.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"An exact similarity search guarantee does not require a full scan, because exact means the result set equals what a brute-force scan returns. A prune may skip a row only when a proven bound shows it cannot meet the threshold. Pruning changes how much work is done while the answer stays the same.\"},\"name\":\"Does an exact similarity search guarantee mean a full scan?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Exact search is not always slower than approximate search, because pruning cuts comparisons without changing the answer. Any speed claim needs its conditions: machine, dataset, fingerprint width, threshold, threads and trials. The benchmarks page lists each run together with all of those conditions.\"},\"name\":\"Is exact search always slower than approximate search?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Run a brute-force scan beside the fast path on every query and fail on any difference in rows or scores. Recall is the wrong metric, because exact means no missing and no extra rows. The correctness page lists the tests behind the claim.\"},\"name\":\"How do I check that my similarity search is really exact?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A score of 1.0 means the two fingerprints have equal bits, so the compounds behind them can still be different compounds. Which structures end up sharing bits depends on the fingerprint scheme. Salts, tautomers and stereo all need canonicalising before ingestion.\"},\"name\":\"Does score 1.0 mean two molecules are identical?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Setting a limit never changes the answer to the query. The limit caps how many hits you receive. A capped list is a cut of the exact result, which means it is never a different list from the one the same query returns without a limit.\"},\"name\":\"What happens to the answer if I set a limit?\"}],\"name\":\"Exact similarity search guarantee: How to verify\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/0SNUBAykp-eEfLambydmzNXJRqVRGdBO.webp\"},\"url\":\"https://mossflux.com/blog/exact-similarity-search-guarantee\"},{\"@id\":\"https://mossflux.com/blog/exact-similarity-search-guarantee#article\",\"@type\":\"BlogPosting\",\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Fast and Exact Similarity Search in less than a Blink of an Eye\",\"url\":\"https://arxiv.org/html/2411.17483v2\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Perform Exact Similarity Search - Oracle Help Center\",\"url\":\"https://docs.oracle.com/en/database/oracle/oracle-database/23/vecse/perform-exact-similarity-search.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Query Data With Similarity and Hybrid Searches - Oracle Help Center\",\"url\":\"https://docs.oracle.com/en/database/oracle/oracle-database/23/vecse/query-data-similarity-and-hybrid-searches.html\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/#demo\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"loruorg/mossflux\",\"url\":\"https://github.com/loruorg/mossflux\"}],\"dateModified\":\"2026-11-03\",\"datePublished\":\"2026-11-03\",\"description\":\"What is an exact similarity search guarantee? The result set equals a brute-force scan: every qualifying row returned, none extra. Learn how to test it.\",\"headline\":\"Exact similarity search guarantee: How to verify\",\"image\":\"https://api.birchseek.com/m/0SNUBAykp-eEfLambydmzNXJRqVRGdBO.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/exact-similarity-search-guarantee#webpage\"},\"keywords\":[\"exact similarity search guarantee\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/exact-similarity-search-guarantee#webpage\"},\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2624},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"statistical Markov model\",\"name\":\"Hidden Markov model\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hidden_Markov_model\",\"https://www.wikidata.org/wiki/Q176769\",\"https://www.google.com/search?kgmid=/m/0p8wy\"]},{\"@type\":\"Thing\",\"description\":\"specialized artificial neural networks that are designed for tasks whose inputs are graphs\",\"name\":\"Graph neural network\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Graph_neural_network\",\"https://www.wikidata.org/wiki/Q107750691\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"chemical compound\",\"name\":\"Histidine\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Histidine\",\"https://www.wikidata.org/wiki/Q485277\",\"https://www.google.com/search?kgmid=/m/0h1tz\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - similarity search
  - tanimoto
  - chemical fingerprints
  - bitbound
  - brute-force testing
  - cheminformatics
title: "Exact similarity search guarantee: How to verify"
---

**An exact similarity search guarantee means the result set equals a brute-force scan for the stated fingerprints and metric: every qualifying row is returned and every returned row qualifies. Pruning keeps that true while skipping work, because it rejects a row only on a proven bound. You check the guarantee by comparing every query's output with a brute-force oracle and failing on any difference.**

The exact similarity search guarantee matters to anyone who runs a Tanimoto threshold search over ChEMBL or an in-house library and can't afford to miss a true hit. An index can turn a search approximate and drop rows without raising an error, and that's what an exact similarity search guarantee rules out.

After reading, you can write the guarantee as a testable contract, check it against a brute-force scan and see where it stops. mossflux, an open-source Rust engine, is the worked example throughout.

> **Key Takeaways**
> - BitBound skips a row only when a rational bound, rounded down, proves it can't reach the threshold, so rounding never drops a true hit.
> - An exact similarity search guarantee for kNN also needs a stated order for equal scores, or two runs can disagree at rank k.
> - A limit caps the rows returned and never changes which rows qualify.
> - REST, gRPC and MCP share one engine in mossflux, so the three transports can't disagree.

## What is an exact similarity search guarantee?

An exact similarity search guarantee means the result set equals what a brute-force scan returns: every row that meets the threshold or ranks in the top k, with its exact score, and no row that does not. It rules out false negatives and false positives for the stated fingerprints and metric.

![Illustration for What is an exact similarity search guarantee?](/blog/exact-similarity-search-guarantee/image-1-472d4de370df.webp)

The definition can be tested. Score the query against every row, then compare that list with what the search returned. A missing row, an extra row or a different score breaks the guarantee.

The [mossflux docs](https://mossflux.com/docs/) state the same rule for their own engine: "Every answer is exactly what a brute-force scan returns."

The metric is the Tanimoto coefficient. For two fingerprints, it is the number of bits set in both, divided by the number of bits set in either.

Written with popcounts, that is `c / (a + b - c)`, where `a` and `b` are the bits set in each fingerprint and `c` is the bits they share. An exact score is that fraction computed in f64.

The guarantee applies to two query shapes:

- Threshold search returns every row with Tanimoto >= t.
- kNN search returns the k rows with the highest Tanimoto scores.

### Worked example: aspirin

Take aspirin, `CC(=O)Oc1ccccc1C(=O)O`, as the query, and set the threshold to 0.7. A brute-force scan computes the Tanimoto score between aspirin's fingerprint and every row in the library, then keeps each row that scores 0.7 or higher.

That set is the exact answer. A search that skips a row scoring 0.71 has a false negative. A search that returns a row scoring 0.69 has a false positive.

Run the same query as kNN with k = 10 and the answer is the ten highest scores. If a row outside the returned ten scores above the tenth hit, the search has dropped a true neighbour.

The exact similarity search guarantee covers the fingerprint and metric you named. Whether that fingerprint separates two compounds a chemist would call different is a separate question.

## How does exact search skip rows without losing a hit?

Exact search skips a row only when a proven bound shows that the row cannot qualify, so skipping never costs a hit. A full scan is one way to be exact, but it isn't the only one: a bound lets the search reach the same answer while scoring fewer rows.

Shared bits cannot outnumber the smaller popcount, so a Tanimoto score can never exceed `min(a, b) / max(a, b)`. The bound reads popcounts only, never the bits themselves.

Suppose a query has popcount 50 and the threshold is t = 0.8. The bound then narrows the rows that need scoring:

- A row with popcount b >= 50 can qualify only if 50 / b >= 0.8, so b <= 62.5.
- A row with popcount b < 50 can qualify only if b / 50 >= 0.8, so b >= 40.
- Only rows with popcount 40 to 62 need scoring. A row with popcount 30 has a bound of 0.6, so it's dismissed unread.

Time-series researchers call the same idea lower bounding, the GEMINI principle, which the [arXiv paper on exact data series search](https://arxiv.org/html/2411.17483v2) builds on. If a cheap bound can never overstate how close a candidate is, discarding on that bound produces no false dismissals.

The popcount bound is the Tanimoto version. It never understates the distance, so a rejected row couldn't have been a hit.

The [mossflux design page](https://mossflux.com/docs/design) applies this in three parts:

- The arena is sorted by popcount, so the candidate window is one contiguous slice of rows.
- The bound is a rational number, rounded down, so rounding cannot drop a true hit.

Written as a contract, a prune rejects a row only on a proven bound, and any shortcut that can't show that proof isn't allowed. Every allowed shortcut can then be checked against a brute-force scan.

## How do you test an exact similarity search guarantee?

You test an exact similarity search guarantee by running a brute-force oracle next to the fast path on every query and failing the run on any difference. Recall isn't the right metric for an exact similarity search guarantee.

An approximate index can be useful at a recall below 100%. An exact system has one passing result: a set difference of zero.

1. Choose query sets. Include threshold searches at several thresholds (for example 0.3, 0.5, 0.7 and 0.9) and kNN searches at several values of k. Repeat them at several fingerprint widths, because folding changes popcounts and so changes what a prune sees. Add queries that produce ties.
2. Compute the oracle result. Score every row with no index and no pruning, using exact arithmetic or an f64 reference. Keep this code separate from the fast path, or one bug will pass both.
3. Compare row ids and scores. The id sets must match, and so must each score. For kNN, the order must match too, ties included.
4. Fail on any difference. One missing id is a false negative and one extra id is a false positive, and either one fails the run. Also check that a limit never changes which rows qualify.
5. Re-run after every change to the index. A new bound, a new scan kernel or an appended segment can each break exactness. Success is zero differences on every query.

## Worked example: mossflux's own gates

mossflux gates every benchmark run on exactness: its [correctness page](https://mossflux.com/docs/correctness) says each fast path is checked against a brute-force or f64 reference on every query of the run, and the run fails on any difference. The same page describes what "exact" means in mossflux and the tests that prove it.

Substructure search has its own oracle in `brute.rs`. For a SMARTS query, the engine's hit list must equal the brute-force hit list.

mossflux also runs a differential against RDKit, in which 114 of 114 queries agree. That differential covers the SMARTS substructure engine. For similarity, the gate is the comparison with the brute-force or f64 reference.

## Does the exact similarity search guarantee hold for ties, limits and timeouts?

Exact search stays exact at the edges when equal scores have a stated order, a limit never changes an answer, and a substructure search that runs past its deadline returns an error instead of a partial list.

A kNN result is reproducible only if equal scores have a stated order. Without one, two runs over the same data can put different molecules at rank k.

A limit refuses; it never changes an answer. In the [mossflux CLI](https://mossflux.com/docs/cli), `--limit` caps only the printed hits, and the counts stay exact, so the count still covers every qualifying row.

Substructure search needs a deadline because subgraph matching is NP-complete, so a short query can take exponential time. The sulfonamide SMARTS `S(=O)(=O)N` is cheap, but a query you don't control may not be.

Past the deadline, the answer is an error, never a partial list. A partial list would look complete while missing rows, which breaks the exact similarity search guarantee with no warning. An error tells you to raise the deadline or tighten the query.

## Exact vs approximate similarity search: what does the guarantee cost?

Exact similarity search costs the shortcuts an approximate index takes: it keeps recall at 100% and can cut work only where a bound proves a row cannot qualify.

| Criterion | Brute-force scan | Approximate index | mossflux |
| --- | --- | --- | --- |
| Same rows as a brute-force scan | Yes | Not guaranteed | Yes |
| How work is cut | Not cut | Fewer comparisons | Rows skipped only on a proven bound |
| Can a missed hit go unnoticed | No | Yes | No |
| Approximate mode | None | Is the mode | None |

[Oracle's documentation on similarity searches](https://docs.oracle.com/en/database/oracle/oracle-database/23/vecse/query-data-similarity-and-hybrid-searches.html) says that approximate similarity searches "seek a balance between these goals" of speed and accuracy. That balance means recall below 100% in return for fewer comparisons.

The exact guarantee can also be lost without any error. [Oracle's page on exact similarity search](https://docs.oracle.com/en/database/oracle/oracle-database/23/vecse/perform-exact-similarity-search.html) documents the `FETCH EXACT` clause.

On a table that has a vector index, a top-10 query that leaves out `EXACT` can be answered from the approximate index. The query looks the same, and nothing in the result marks it as approximate, so you'd never see that the exact similarity search guarantee was lost.

mossflux avoids this by having no approximate mode, so there is no keyword to forget, as its [design documentation](https://mossflux.com/docs/design) states.

Speed is the other half of the cost, and a speed figure means something only with its conditions. The [benchmarks page](https://mossflux.com/docs/benchmarks) lists the measured comparisons with FPSim2 and with Arthor's published per-thread figure.

No aggregate win over Arthor's 96-thread figure is claimed. Quote a speed from that page only together with those conditions.

The timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on the new code.

Each approach fits a different job:

- Brute-force scan: small collections, and the oracle that every other method is checked against.
- Approximate index: large vector collections where a recall below 100% is acceptable and measured.
- mossflux: Tanimoto threshold and kNN search over binary fingerprints where every true hit is required.

## Does the exact similarity search guarantee hold over REST, gRPC and MCP?

The guarantee holds over all three entry points because, as the [mossflux API docs](https://mossflux.com/docs/api) describe, `mossflux serve` runs REST and MCP on port 8080 and gRPC on 50051 over one engine, so the three cannot disagree.

REST takes JSON. A threshold search with aspirin as the query looks like this:

```sh
curl -X POST http://localhost:8080/v1/similarity \
  -H 'Content-Type: application/json' \
  -d '{"smiles": "CC(=O)Oc1ccccc1C(=O)O", "threshold": 0.7, "limit": 10}'
```

The request needs only a SMILES, because the server computes its own 2048-bit ECFP4-style fingerprints. The documented fields are `smiles`, `threshold`, `k`, `limit` and `db`. Send `k` for a kNN search, and `db` to pick a database when the server loads several.

The same search is the MCP tool `similarity_search` and the gRPC method `Similarity` on `mossflux.v1.Search`. MCP also offers `molecule_properties`, and `mossflux mcp` serves it over stdio.

An agent and a script that send the same aspirin query reach the same engine and get the same rows.

[The public mossflux demo](https://mossflux.com/#demo) at `https://api.mossflux.com` serves ChEMBL 37 (2.9 million molecules) on one small 4-thread server. Try the demo with the aspirin query, then compare the rows with a brute-force scan of your own copy of ChEMBL.

## What does the guarantee not cover?

The exact similarity search guarantee covers which rows a search returns for the bits in the index. Whether those bits describe the compound you meant is outside it.

Two different molecules can share a fingerprint, so a score of 1.0 is a lead to check, not proof they're the same compound.

The fingerprint scheme also decides which rows count as hits. The [mossflux API docs](https://mossflux.com/docs/api) say the server's bits "are mossflux's own and do not match RDKit or other toolkits."

If you load an FPS file made by another toolkit, the search is exact over those bits. The same library searched under two schemes can return two different sets, and each set is correct for its own scheme.

Canonicalisation is your job before ingestion, and it's outside the exact similarity search guarantee. Aspirin `CC(=O)Oc1ccccc1C(=O)O` and its sodium salt are separate rows unless you strip the salt first.

The index lives in memory and is rebuilt at every start. The [deploy docs](https://mossflux.com/docs/deploy) give ChEMBL 37 (2,896,304 molecules) as a 33 to 41 s build, a 5.1 GB index and about 2.5 KB of RAM per molecule.

For substructure search, the [mossflux design page](https://mossflux.com/docs/design) says the path screen is a superset filter that never drops a match. VF2 verify makes the final decision on every row the screen lets through.

## Frequently asked questions

### Does an exact similarity search guarantee mean a full scan?

Exact means the result set equals what a brute-force scan returns, not that every row is scored. A prune may skip a row only when a proven bound shows it cannot meet the threshold, so pruning changes the work done and never the answer.

### Is exact search always slower than approximate search?

Exact search isn't slower by definition, because pruning cuts comparisons without changing the answer. Any speed claim needs its conditions: machine, dataset, fingerprint width, threshold, threads and trials. The [mossflux benchmarks page](https://mossflux.com/docs/benchmarks) lists each run with them.

### How do I check that my similarity search is really exact?

Run a brute-force scan beside the fast path on every query and fail on any difference in rows or scores. Recall is the wrong metric, because exact means no missing and no extra rows. The [mossflux correctness page](https://mossflux.com/docs/correctness) lists the tests behind the claim.

### Does score 1.0 mean two molecules are identical?

A score of 1.0 means the two fingerprints have equal bits, not that the compounds are the same. Which structures share bits depends on the fingerprint scheme, and salts, tautomers and stereo need canonicalising before ingestion.

### What happens to the answer if I set a limit?

A limit never changes the answer. It caps how many hits you receive, so a capped list is a cut of the exact result and never a different one.

## What to do next with an exact search guarantee

Today, write your exact similarity search guarantee as one testable sentence: "Every search returns the same rows, with the same scores, as a brute-force scan." Then list where it could be lost, such as a fallback to an approximate index or a limit that changes which rows qualify.

This week, add a brute-force oracle that runs beside the fast path on every query and fails on any difference. Cover threshold and kNN queries, ties included.

Later, check your fingerprint scheme and your canonicalisation of salts, tautomers and stereo before ingestion.

[Run mossflux on your own library](https://mossflux.com/#get).

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

The [mossflux repository](https://github.com/loruorg/mossflux) is free and open source under `MIT OR Apache-2.0`, with no paid plan, hosted service or account.

## Sources

[mossflux docs](https://mossflux.com/docs/) · [Fast and Exact Similarity Search in less than a Blink of an…](https://arxiv.org/html/2411.17483v2) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [Query Data With Similarity and Hybrid Searches - Oracle…](https://docs.oracle.com/en/database/oracle/oracle-database/23/vecse/query-data-similarity-and-hybrid-searches.html) · [Perform Exact Similarity Search - Oracle Help Center](https://docs.oracle.com/en/database/oracle/oracle-database/23/vecse/perform-exact-similarity-search.html) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks) · [API · mossflux docs](https://mossflux.com/docs/api). Every other source is linked where the article cites it.
