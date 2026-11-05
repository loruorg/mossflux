---
description: What is the fingerprint index memory footprint? Plan about 2.5 KB of RAM per molecule in mossflux, summed over all databases. Learn how to size your library.
faq:
  - answer: "mossflux rebuilds the fingerprint index at every start, and the index isn't saved to disk. The index lives in memory, so each restart loads every database again and needs the RAM free. Sizing for that memory is covered in the Deploy docs."
    question: Is the fingerprint index saved to disk, or rebuilt at every start?
  - answer: "Folding a fingerprint from 2048 to 256 bits can change the search results, because folding changes which bits are compared. Scores and hits can differ from the 2048-bit run. The method stays exact: mossflux returns the same rows a brute-force scan returns over the folded bits."
    question: Does folding a fingerprint from 2048 to 256 bits change the search results?
  - answer: mossflux has no sharding, so an index cannot be split across machines, and row ids are u32. Each index sits in memory on one machine, so size that host for all databases loaded together. Serve several libraries from one process with one --db NAME=PATH per database.
    question: Can I shard a mossflux index across machines?
  - answer: "Poll the GET /ready endpoint to see when the mossflux server has finished loading. It returns 503 until every database is loaded, then returns 200. Point your orchestrator's readiness probe at it so traffic arrives only after the index is built."
    question: How do I know when the server has finished loading?
image: /blog/fingerprint-index-memory-footprint/featured-39b75e7b4a0d.webp
image_alt: "Illustration for Fingerprint index memory footprint: Sizing"
publishDate: "2026-11-05"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/fingerprint-index-memory-footprint#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q3745713\",\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]}],\"datePublished\":\"2026-11-05\",\"description\":\"What is the fingerprint index memory footprint? Plan about 2.5 KB of RAM per molecule in mossflux, summed over all databases. Learn how to size your library.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux rebuilds the fingerprint index at every start, and the index isn't saved to disk. The index lives in memory, so each restart loads every database again and needs the RAM free. Sizing for that memory is covered in the Deploy docs.\"},\"name\":\"Is the fingerprint index saved to disk, or rebuilt at every start?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Folding a fingerprint from 2048 to 256 bits can change the search results, because folding changes which bits are compared. Scores and hits can differ from the 2048-bit run. The method stays exact: mossflux returns the same rows a brute-force scan returns over the folded bits.\"},\"name\":\"Does folding a fingerprint from 2048 to 256 bits change the search results?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux has no sharding, so an index cannot be split across machines, and row ids are u32. Each index sits in memory on one machine, so size that host for all databases loaded together. Serve several libraries from one process with one --db NAME=PATH per database.\"},\"name\":\"Can I shard a mossflux index across machines?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Poll the GET /ready endpoint to see when the mossflux server has finished loading. It returns 503 until every database is loaded, then returns 200. Point your orchestrator's readiness probe at it so traffic arrives only after the index is built.\"},\"name\":\"How do I know when the server has finished loading?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q580427\",\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q1182260\",\"@type\":\"Thing\",\"description\":\"data processing technique to eliminate duplicate copies of repeating data\",\"name\":\"Data deduplication\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Data_deduplication\",\"https://www.wikidata.org/wiki/Q1182260\",\"https://www.google.com/search?kgmid=/m/043n6w3\"]}],\"name\":\"Fingerprint index memory footprint: Sizing\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/pLIx5vQQQ0p6ET4ETCLoCMEajGseJUrV.webp\"},\"url\":\"https://mossflux.com/blog/fingerprint-index-memory-footprint\"},{\"@id\":\"https://mossflux.com/blog/fingerprint-index-memory-footprint#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q3745713\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Springer chapter on indexing schemes\",\"url\":\"https://link.springer.com/chapter/10.1007/978-981-19-0112-6_4\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"MossFlux API docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux benchmarks\",\"url\":\"https://mossflux.com/docs/benchmarks\"}],\"dateModified\":\"2026-11-05\",\"datePublished\":\"2026-11-05\",\"description\":\"What is the fingerprint index memory footprint? Plan about 2.5 KB of RAM per molecule in mossflux, summed over all databases. Learn how to size your library.\",\"headline\":\"Fingerprint index memory footprint: Sizing\",\"image\":\"https://api.birchseek.com/m/pLIx5vQQQ0p6ET4ETCLoCMEajGseJUrV.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/fingerprint-index-memory-footprint#webpage\"},\"keywords\":[\"fingerprint index memory footprint\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/fingerprint-index-memory-footprint#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q580427\"},{\"@id\":\"https://www.wikidata.org/wiki/Q1182260\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2184},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"statistical Markov model\",\"name\":\"Hidden Markov model\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hidden_Markov_model\",\"https://www.wikidata.org/wiki/Q176769\",\"https://www.google.com/search?kgmid=/m/0p8wy\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"specialized artificial neural networks that are designed for tasks whose inputs are graphs\",\"name\":\"Graph neural network\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Graph_neural_network\",\"https://www.wikidata.org/wiki/Q107750691\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - fingerprint index
  - memory sizing
  - tanimoto similarity
  - chembl
  - mossflux
  - molecular search
title: "Fingerprint index memory footprint: Sizing"
---

**A fingerprint index for molecular search stores one binary fingerprint per molecule, such as an ECFP4-style fingerprint of aspirin, `CC(=O)Oc1ccccc1C(=O)O`. For the fingerprint index memory footprint, the [mossflux Deploy docs](https://mossflux.com/docs/deploy) say to plan about 2.5 KB of RAM per molecule for the server index, which is built in memory at every start and not saved to disk.**

Anyone putting ChEMBL, an in-house collection or a make-on-demand space behind a search API has to decide how much RAM the service gets. Too little and loading fails. Too much and you pay for memory the service never uses.

With the steps here, you can estimate RAM for your own library, check it against mossflux's limits and set up a readiness check for the rebuild that happens at every start.

> **Key Takeaways**
> - Raw fingerprint bits cost width ÷ 8 bytes per molecule: 256 bytes at 2,048 bits, 32 at 256 bits.
> - [BitBound pruning and the inverted bitmap](https://mossflux.com/docs/design) return the same rows as a brute-force scan, so a larger index never gives approximate answers.
> - Every `--db NAME=PATH` database sits in memory at the same time, so size the host for their sum.
> - Folding shrinks raw bits but changes which bits are compared, so scores and hits can shift.

## What is a molecular fingerprint in a similarity index?

A molecular fingerprint is a fixed-length string of bits recording which structural features a molecule contains, and a fingerprint index stores one such string per molecule so that Tanimoto similarity can be computed between them.

![Illustration for What is a molecular fingerprint in a similarity index?](/blog/fingerprint-index-memory-footprint/image-1-ccfb4d61b06f.webp)

Aspirin's SMILES goes in, and one bit string of a set width comes out. Every other molecule in the library gets a string of the same width. A query is compared with each of them by Tanimoto similarity, which gives one score per pair.

The comparison doesn't read anything but the bit strings. Their width therefore sets how much memory the raw data needs.

The term has other meanings outside chemistry.

A [Springer chapter on indexing schemes](https://link.springer.com/chapter/10.1007/978-981-19-0112-6_4) uses it for deduplication, where indexing identifies and eliminates duplicate data. None of those uses applies to molecular search.

In mossflux, the server builds the bits itself, as the [mossflux API documentation](https://mossflux.com/docs/api) describes. Those bits are mossflux's own and don't match RDKit or other toolkits.

## What makes up the fingerprint index memory footprint per molecule?

The fingerprint index memory footprint per molecule is the raw fingerprint bits plus the structures the index keeps to prune the search. The bits are the smallest and most predictable part.

Each molecule stores one bit per fingerprint position, so raw bits cost width ÷ 8 bytes. Doubling the width doubles this part:

| Fingerprint width | Raw bits per molecule |
|---|---|
| 256 bits | 32 bytes |
| 1,024 bits | 128 bytes |
| 2,048 bits | 256 bytes |

mossflux's [design documentation](https://mossflux.com/docs/design) names two pruning structures on top of the bits: a popcount-sorted arena, which BitBound pruning scans, and an inverted bitmap.

The popcount of a fingerprint is its number of set bits. For a Tanimoto threshold t, a molecule can only qualify if its popcount lies within a range set by the query's popcount and t.

BitBound pruning uses that range. With rows held in an arena sorted by popcount, whole stretches of rows are skipped without being compared.

An inverted bitmap lists, for each bit position, which molecules have that bit set. It's a second view of the same fingerprints, so it adds memory on top of the raw bits.

The fingerprint index memory footprint grows linearly with the number of molecules. At 2,048 bits the raw bits come to 256 bytes per molecule, a fraction of the documented per-molecule figure, so sizing from raw bits alone would under-provision the server.

For a real plan, take the per-molecule figure that mossflux measured on a named server, width and database, and multiply it by your molecule count. Label the result an estimate, since a different SMILES mix or width changes the per-molecule cost.

## How big is the fingerprint index for ChEMBL 37?

The fingerprint index for ChEMBL 37 is built in memory when mossflux starts, and mossflux's [Deploy docs](https://mossflux.com/docs/deploy) list its measured molecule count, build time, index size and RAM per molecule. Those figures are the base for your own sizing.

1. Read the per-molecule figure, not the total. The RAM-per-molecule value is the one that scales; the index size alone says nothing about a library of a different length.
2. Count the molecules in each SMILES file you plan to load. Each database is one file, and the [CLI docs](https://mossflux.com/docs/cli) describe serving several with `--db NAME=PATH`.
3. Multiply. Required RAM is the molecule count times the per-molecule figure, summed over every `--db` entry. Time your own start rather than scaling the ChEMBL 37 build time.
4. Check the fingerprint width. The per-molecule figure applies to the server's 2,048-bit fingerprints, so measure RAM yourself before relying on another width.
5. Check the row limit. In the [mossflux design docs](https://mossflux.com/docs/design), row ids are u32, so one index holds at most 4,294,967,296 rows. There's no sharding to go beyond that.
6. Point your health check at `/ready`. The mossflux docs say that `GET /ready` returns 503 until the index is ready, then 200.

With two databases, `mossflux serve --db chembl=chembl37.smi --db inhouse=inhouse.smi`, the arithmetic is short. Call the molecule counts N1 and N2 and the per-molecule figure from the Deploy page c. Plan for (N1 + N2) × c bytes of RAM.

The estimate holds only for libraries below the row limit, at the same width, on a single index. mossflux makes no claim for libraries larger than ChEMBL.

## How do fingerprint width and folding change the memory footprint?

Fingerprint width sets the raw-bit cost of an index at width ÷ 8 bytes per molecule, and folding cuts that cost by merging bit positions, which also changes the scores.

| Width (bits) | Raw bytes per molecule (arithmetic: bits ÷ 8) | Where it applies |
| --- | --- | --- |
| 256 | 32 (arithmetic) | Folded or FPS file |
| 1,024 | 128 (arithmetic) | FPS file |
| 2,048 | 256 (arithmetic) | Server, fixed |

The server fixes the width.

`mossflux search` loads chemfp FPS files instead.

Folding 2,048 bits to 256 makes the raw bits eight times smaller (2,048 ÷ 256 = 8). Folding maps several bit positions onto one, so features that were separate at full width can share a bit.

A Tanimoto score on folded bits can therefore differ from the score at full width, and the set of hits above a threshold can differ with it.

Exactness applies to the bits that were indexed. The [mossflux docs](https://mossflux.com/docs/) say "Every answer is exactly what a brute-force scan returns." A search over folded bits returns what a brute-force scan over those folded bits returns, and it doesn't recover hits that only the unfolded fingerprint would find.

Choose the width before you build the index. If the difference matters for your library, run a sample of queries at full width and at the folded width.

Timings at 256 bits, with machine, data and method, are on the [mossflux benchmarks page](https://mossflux.com/docs/benchmarks).

Each width suits a different case:

- 256 bits: the smallest raw footprint, when you accept scores computed on folded bits.
- 1,024 bits: a middle width from an FPS file.
- 2,048 bits: served databases, where the width is fixed.

## Should a fingerprint index live in memory or on disk?

mossflux holds its fingerprint index in memory, builds it at every start and has no on-disk index.

| Criterion | In-memory index, rebuilt at start | Persisted on-disk index |
| --- | --- | --- |
| In mossflux today | Yes | No |
| Work at each start | Rebuild | Read a saved file |
| Cost paid | RAM and startup time | Disk space and a file format |
| Readiness signal | `GET /ready` returns 503 until loaded | Not needed for the build |

mossflux sits on the in-memory side. Its [Deploy docs](https://mossflux.com/docs/deploy) state that the index is built in memory at start and is not saved to disk. A restart therefore means a rebuild, and the measured ChEMBL 37 build time is listed in those docs.

The RAM pays for pruning structures that skip work without changing results. The cost is RAM plus the wait at startup.

This article doesn't claim that in-memory beats on-disk in general. Compare the documented features of each tool, since numbers measured on different setups aren't comparable.

An in-memory index rebuilt at start suits a service that restarts rarely and can spend RAM on exact pruning. A persisted on-disk index suits workflows with frequent restarts, where a saved file would skip the rebuild; mossflux doesn't offer one yet.

## Can a smaller fingerprint index still return every true hit?

A smaller fingerprint index still returns every true hit when the saving comes from the bits you chose to search, because mossflux's pruning structures never drop a hit over those bits. Footprint sets how much RAM you pay, while exactness is a property of the search code.

The [mossflux design docs](https://mossflux.com/docs/design) describe BitBound pruning with a rational bound rounded down, "so rounding can never drop a true hit".

A prune rejects a row only on a proven bound. For a given Tanimoto threshold, whole popcount ranges of the arena are skipped without reading a single fingerprint in them.

Appending rows follows the same rule.

A lossy compact filter saves memory by allowing false positives or false negatives, so the rows it returns can differ from a brute-force scan. Exact search skips that trade and accepts a larger footprint, because its extra structures exist only to skip work.

Limits follow the same logic. The [CLI docs](https://mossflux.com/docs/cli) say counts are always exact and `--limit` caps only the printed hits. A substructure search that runs past its deadline returns an error, never a partial list.

The width you pick (for example 2,048 bits, or 256 after folding) decides which bits are searched. Over those bits, the pruning structures never change the result, and RAM is the price of that guarantee.

## How do you estimate the fingerprint index memory footprint for your own library?

You size fingerprint index memory for your own library by multiplying molecule counts by the documented per-molecule RAM cost, then checking six limits before you deploy.

1. Count the molecules in each database file. `mossflux info` is the CLI's inspection subcommand, and the [CLI docs](https://mossflux.com/docs/cli) list its arguments.
2. Note the fingerprint width.
3. Multiply molecules by that figure and label the result an estimate.
4. Add all databases.
5. Check each database against the documented row limit.
6. Plan startup time. Every start rebuilds every database before the server answers a search.

Passing `--db` twice serves ChEMBL and an in-house file from one process. Give that process the ChEMBL estimate plus the in-house estimate.

```sh
mossflux serve --db chembl=chembl37.smi --db inhouse=inhouse.smi
curl http://localhost:8080/v1/databases
curl -i http://localhost:8080/ready
```

In Docker Compose or Kubernetes, point the health check or readiness probe at `/ready` so traffic waits for the load. 

The [API docs](https://mossflux.com/docs/api) list two request limits: 10,000 hits per request and 4 concurrent searches by default, with more waiting their turn. Both are separate from RAM, so a bigger machine doesn't raise them.

To get a real figure for your library, [run mossflux on your own library](https://mossflux.com/#get) and compare the result with your estimate.

## Frequently asked questions

### Is the fingerprint index saved to disk, or rebuilt at every start?

The index lives in memory, and mossflux rebuilds it at every start. It isn't saved to disk, so each restart loads every database again and needs the RAM free. The [Deploy docs](https://mossflux.com/docs/deploy) cover sizing.

### Does folding a fingerprint from 2,048 to 256 bits change the search results?

Folding changes which bits are compared, so scores and hits can differ from the 2,048-bit run.

### Can I shard a mossflux index across machines?

Each index sits in memory on one machine, so size that host for all databases loaded together.

### How do I know when the server has finished loading?

Poll `GET /ready`. The [mossflux API docs](https://mossflux.com/docs/api) say it returns 503 until every database is loaded, then 200. Point your orchestrator's readiness probe at it so traffic arrives only after the index is built.

## What should you do next to size your fingerprint index?

Three steps turn the sizing checklist into a plan for your fingerprint index memory footprint.

Today, count the molecules in each database file you intend to serve.

This week, multiply each count by the per-molecule RAM cost in the [Deploy docs](https://mossflux.com/docs/deploy) and confirm that no database reaches the u32 row-id ceiling. Then run `mossflux info` on a sample file.

Before production, wire your readiness probe to `GET /ready` and size the host for the sum of all `--db NAME=PATH` databases, because they share one machine's memory.

[Run it on your library](https://mossflux.com/#get).

## Sources

[Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [Design · mossflux docs](https://mossflux.com/docs/design) · [Springer chapter on indexing schemes](https://link.springer.com/chapter/10.1007/978-981-19-0112-6_4) · [mossflux docs](https://mossflux.com/docs/) · [mossflux benchmarks](https://mossflux.com/docs/benchmarks) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [MossFlux API docs](https://mossflux.com/docs/api)
