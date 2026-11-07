---
description: "Can an incremental molecular search index skip the rebuild? Yes: mossflux appends in memory and returns the same rows as a brute-force scan. Learn the limits."
faq:
  - answer: "In mossflux, segmented append adds molecules to the index in memory with no rebuild and no change to any answer. Append is an engine capability. The index isn't saved to disk, so a restart rebuilds it from the SMILES file. There are no deletes and no compaction."
    question: Can I add molecules to a similarity search index without rebuilding it?
  - answer: "Appending does not change the results: threshold and kNN search return the same rows as a brute-force scan of the full set. A prune rejects only on a proven bound, so a new segment can't hide a true hit or add a false one."
    question: Does appending change the results?
  - answer: "Release 0.1.0 cannot delete molecules from the index: it has no deletes, no compaction and no sharding, and row ids are u32. To drop a molecule, remove it from the SMILES file and restart. The rebuild on ChEMBL 37 (2,896,304 molecules) takes 33 to 41 s."
    question: Can I delete molecules from the index?
  - answer: Incremental search and merging search results are two different things. iSeqSearch is a tool that merges protein search results from separate runs. mossflux extends a live index, so new molecules become searchable with no rebuild and with no result lists to merge.
    question: Is incremental search the same as merging search results?
image: /blog/incremental-molecular-search-index/featured-1d549248fced.webp
image_alt: "Illustration for Incremental molecular search index: No rebuild"
publishDate: "2026-11-07"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/incremental-molecular-search-index#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q8054\",\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]}],\"datePublished\":\"2026-11-07\",\"description\":\"Can an incremental molecular search index skip the rebuild? Yes: mossflux appends in memory and returns the same rows as a brute-force scan. Learn the limits.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"In mossflux, segmented append adds molecules to the index in memory with no rebuild and no change to any answer. Append is an engine capability. The index isn't saved to disk, so a restart rebuilds it from the SMILES file. There are no deletes and no compaction.\"},\"name\":\"Can I add molecules to a similarity search index without rebuilding it?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Appending does not change the results: threshold and kNN search return the same rows as a brute-force scan of the full set. A prune rejects only on a proven bound, so a new segment can't hide a true hit or add a false one.\"},\"name\":\"Does appending change the results?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Release 0.1.0 cannot delete molecules from the index: it has no deletes, no compaction and no sharding, and row ids are u32. To drop a molecule, remove it from the SMILES file and restart. The rebuild on ChEMBL 37 (2,896,304 molecules) takes 33 to 41 s.\"},\"name\":\"Can I delete molecules from the index?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Incremental search and merging search results are two different things. iSeqSearch is a tool that merges protein search results from separate runs. mossflux extends a live index, so new molecules become searchable with no rebuild and with no result lists to merge.\"},\"name\":\"Is incremental search the same as merging search results?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q364\",\"@type\":\"Organization\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q608751\",\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]}],\"name\":\"Incremental molecular search index: No rebuild\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/IMth_7hR6Q1QfM_7H1_sBzysa9Q32HkK.webp\"},\"url\":\"https://mossflux.com/blog/incremental-molecular-search-index\"},{\"@id\":\"https://mossflux.com/blog/incremental-molecular-search-index#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q8054\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"mossflux: exact molecular search in Rust\",\"url\":\"https://github.com/loruorg/mossflux\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"GitHub - EESI/Incremental-Protein-Search\",\"url\":\"https://github.com/EESI/Incremental-Protein-Search\"},{\"@type\":\"CreativeWork\",\"name\":\"Incremental IVF Index Maintenance for Streaming Vector Search\",\"url\":\"https://arxiv.org/html/2411.00970v1\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux changelog\",\"url\":\"https://mossflux.com/docs/changelog\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux benchmarks\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux public demo\",\"url\":\"https://mossflux.com/#demo\"}],\"dateModified\":\"2026-11-07\",\"datePublished\":\"2026-11-07\",\"description\":\"Can an incremental molecular search index skip the rebuild? Yes: mossflux appends in memory and returns the same rows as a brute-force scan. Learn the limits.\",\"headline\":\"Incremental molecular search index: No rebuild\",\"image\":\"https://api.birchseek.com/m/IMth_7hR6Q1QfM_7H1_sBzysa9Q32HkK.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/incremental-molecular-search-index#webpage\"},\"keywords\":[\"incremental molecular search index\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/incremental-molecular-search-index#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q364\"},{\"@id\":\"https://www.wikidata.org/wiki/Q608751\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2255},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"hardware direct memory access over computer networks\",\"name\":\"Remote direct memory access\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Remote_direct_memory_access\",\"https://www.wikidata.org/wiki/Q2561570\",\"https://www.google.com/search?kgmid=/m/04599j\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"process by which new candidate medications are discovered\",\"name\":\"Drug discovery\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Drug_discovery\",\"https://www.wikidata.org/wiki/Q1418791\",\"https://www.google.com/search?kgmid=/m/02r6nx\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - cheminformatics
  - molecular search
  - tanimoto similarity
  - fingerprints
  - mossflux
  - chembl
title: "Incremental molecular search index: No rebuild"
---

An incremental molecular search index makes new molecules searchable without rebuilding the index. [mossflux](https://github.com/loruorg/mossflux) does this with segmented append in memory, and threshold and kNN results still match [a brute-force scan](https://mossflux.com/docs/) of the full set.

Compound libraries gain molecules between releases: a ChEMBL update, a week of in-house registrations. With a full rebuild, nobody can search the new rows until the index is rebuilt and memory is re-sized.

After reading, you can tell the two meanings of "incremental" apart, test the exactness guarantee on your own data, weigh an append against a rebuild and plan around the limits of release 0.1.0.

> **Key Takeaways**
> - A full [in-memory build](https://mossflux.com/docs/deploy) of ChEMBL 37 (2,896,304 molecules) takes 33 to 41 s; an append skips that work for indexed molecules.
> - A prune rejects a row only on a proven bound, so adding a segment cannot change an answer.
> - No append-throughput number is claimed.

## What is an incremental molecular search index?

An incremental molecular search index is an index that accepts new molecules without being rebuilt. mossflux works this way: it [appends a segment in memory](https://mossflux.com/docs/design), so new molecules are searchable at once and every answer matches a brute-force scan.

![Illustration for What is an incremental molecular search index?](/blog/incremental-molecular-search-index/image-1-24490c5c5d98.webp)

In sequence search, "incremental" has a second meaning: merging the results of separate searches. Tools such as [iSeqSearch](https://github.com/EESI/Incremental-Protein-Search) work this way. The searches stay separate, and only their outputs are combined.

For a growing compound library, the first meaning is the useful one. The incremental molecular search index itself grows, and a query sees old and new rows through one engine.

### What it means for SMILES, fingerprints and SMARTS

In cheminformatics, a new molecule arrives as a SMILES string. It becomes a fixed-width bit-vector fingerprint, and Tanimoto similarity compares those bits. SMARTS substructure queries run against the same rows.

New fingerprints go into a new segment of the incremental molecular search index in memory, and existing segments stay as they are.

Appending a segment avoids repeating the full build for molecules already indexed. Search for aspirin, `CC(=O)Oc1ccccc1C(=O)O`, at a Tanimoto threshold of 0.7, append 10,000 new SMILES and repeat the query: the second answer lists the same rows a brute-force scan of the old and new molecules together would return.

## How does an incremental molecular search index append without a rebuild?

An incremental molecular search index appends without a rebuild by placing new molecules in a new segment and leaving the existing segments as they are. mossflux calls this [segmented append](https://mossflux.com/docs/design). It happens in memory, and it changes no answer.

An answer can't change because the prune is exact. The BitBound prune skips a row when the Tanimoto threshold t is impossible given the popcounts of the two fingerprints.

Two fingerprints share at most min(a, b) bits, where a and b are their popcounts, and their union has at least max(a, b) bits. The Tanimoto score is therefore at most min(a, b) / max(a, b). If that ceiling is below t, the row can't be a hit.

Take a query with 20 set bits and t = 0.8. A row with 25 set bits has a ceiling of 20/25 = 0.8, so it stays a candidate. A row with 26 set bits has a ceiling of 20/26, about 0.77, so it's rejected without computing a score.

Smaller rows follow the same rule. A row with 15 set bits (15/20 = 0.75) is rejected, while a row with 16 set bits (16/20 = 0.8) stays a candidate.

mossflux runs this prune over a popcount-sorted arena. It uses a rational bound that is rounded down, so rounding can never drop a true hit.

The verdict of such a prune depends on three things: the row's popcount, the query's popcount and t. No other segment enters into it. A row rejected in segment one is rejected whether or not segment two exists.

Correctness across segments follows from that:

- Each old row is accepted or rejected exactly as before.
- In threshold search, the full answer is the old hits plus whatever the new segment contributes.
- In kNN, a new row enters the k nearest only if its exact score beats an old one, which is what a scan of the full set would return.

Appending adds rows to consider. It can't turn a rejected row into a hit or a hit into a miss, so the incremental molecular search index stays equal to a brute-force scan of every molecule.

## Does an incremental molecular search index return the same rows as a scan?

Yes: after an append, mossflux returns the same rows as [a brute-force scan](https://mossflux.com/docs/) of the full set for threshold and kNN similarity search, with no false negatives and no false positives.

| Criterion | mossflux | Merge-based search (iSeqSearch) | Clustered vector index (Ada-IVF) |
| --- | --- | --- | --- |
| Data searched | Molecular fingerprints | Protein sequences | Vectors |
| What gets updated | A live index | Search results | An IVF index |
| Input unit | New molecules | Result files (`.m8`) | Streaming vectors |

The mossflux [correctness page](https://mossflux.com/docs/correctness) covers what "exact" means in mossflux and the tests that prove it. A scan of the full set is the reference, so an append is correct only if it leaves every answer equal to that scan.

Merge-based search solves a different problem. The [iSeqSearch GitHub page](https://github.com/EESI/Incremental-Protein-Search) says "Incremental Protein Search is a tool designed to merge protein search results." Each search runs on its own data, and the tool combines the outputs.

mossflux instead extends one incremental molecular search index, so a new molecule goes through the same engine as the old ones.

Clustered vector indexes are a third design. The [Ada-IVF paper](https://arxiv.org/html/2411.00970v1), "Incremental IVF Index Maintenance for Streaming Vector Search," concerns an IVF index that has to be maintained as vectors stream in. That's a different index family from a popcount-sorted fingerprint arena.

Each design fits a different job:

- mossflux suits teams that search SMILES-derived fingerprints and need every true hit after each append, checked against a brute-force scan.
- iSeqSearch suits protein searches that run in separate batches, where the goal is to merge the result files.
- Ada-IVF suits streaming vector collections served from a clustered IVF index that needs maintenance as data arrives.

## How do you check that a grown index matches a brute-force scan?

You check a grown index by running one query before and after an append, then diffing both answers against a brute-force scan of the full set. The query here is aspirin, `CC(=O)Oc1ccccc1C(=O)O`.

The [CLI page](https://mossflux.com/docs/cli) says the binary "searches files from the shell and runs the server." The [API page](https://mossflux.com/docs/api) "lists the endpoints, their fields and the limits."

Neither page lists an append command or an append route, so append is an engine capability. Don't assume a flag or endpoint for it.

1. Start `mossflux serve --db lib=first.smi`, then wait until `GET /ready` stops returning 503.
2. Send the aspirin query and save the response.

```sh
curl -s -X POST http://localhost:8080/v1/similarity \
  -H 'content-type: application/json' \
  -d '{"smiles":"CC(=O)Oc1ccccc1C(=O)O","threshold":0.7,"limit":1000,"db":"lib"}'
```

3. Append the second batch through the engine, in memory, with no rebuild. Keep a list of the new SMILES.
4. Send the identical request again. Don't change `threshold` or `db` between the two calls.
5. Compute the Tanimoto score of aspirin against every row of the first and second batches, with no pruning. Use the same fingerprint bits on both sides: the server's bits are mossflux's own, so a scan with another toolkit's fingerprints compares different things.
6. Match the row ids, the scores and the hit count.

A run passes when steps 4 and 5 return the same rows, including every hit that sits in the appended batch. A row missing from step 4 is a false negative, and a row present only in step 4 is a false positive. Either one is a failure.

The [docs home](https://mossflux.com/docs/) states the rule: "Every answer is exactly what a brute-force scan returns."

If you compare through the CLI, `--limit` caps only the printed hits, and the counts stay exact. Compare the counts as well as the printed lines.

You can try the aspirin query first on the [public demo](https://mossflux.com/#demo), which searches ChEMBL 37.

## Append or rebuild: what does each cost?

A full rebuild costs time that grows with the whole library, while a segmented append costs work for the new rows only.

| | Full rebuild | Segmented append |
| --- | --- | --- |
| Work done | Every molecule | New rows only |
| ChEMBL 37 timing | 33 to 41 s | None claimed |
| RAM sizing | About 2.5 KB per molecule, whole library | Same rate, grown library |
| Entry point | Start-up from the SMILES file | Where the documented interface exposes it |

A rebuild processes every molecule again, so adding 1,000 rows to a library of millions still repeats the work for the millions.

The [deploy page](https://mossflux.com/docs/deploy) gives the rebuild figures. ChEMBL 37 has 2,896,304 molecules. The build takes 33 to 41 s, the index takes 5.1 GB, and each molecule needs about 2.5 KB of RAM.

Those figures apply to that dataset and to the in-memory build at start. Every restart repeats the full build from the SMILES file.

Memory sizing applies to the grown library as well. Size RAM for the total row count of the grown incremental molecular search index, not for the rows you started with.

The [benchmarks page](https://mossflux.com/docs/benchmarks) holds the measured results with their conditions. Use the rebuild figure as the ceiling you can plan around.

Choose by what your interface exposes:

- Append where the interface you run documents it, and size RAM for the grown library.
- Otherwise, and on every restart, rebuild from the SMILES file and budget the full build time.
- Do not assume an append endpoint that the [API docs](https://mossflux.com/docs/api) or the [CLI docs](https://mossflux.com/docs/cli) do not show.

A full rebuild fits a library that changes at release boundaries, where one restart per release is acceptable. Segmented append fits a live library that gains rows between releases, such as a week of in-house registrations.

## What can the mossflux index not do yet?

As of release 0.1.0, the mossflux index lives in memory, runs in one process, and has no persistence, deletes, compaction, sharding or write-ahead log; row ids are u32. The [changelog](https://mossflux.com/docs/changelog) lists 0.1.0 as the initial release, so the limits below describe that release.

1. Not persisted yet. The index is [built in memory at start](https://mossflux.com/docs/deploy) and is not saved to disk, so every start rebuilds from the SMILES file. [`GET /ready`](https://mossflux.com/docs/api) returns 503 until the build finishes.
2. No deletes. To drop a molecule, remove its line from the SMILES file and restart.
3. No compaction. Nothing merges segments after an append, and a restart rebuilds the index from the file.
4. No sharding. One process holds the whole index, so size its RAM for the full library at about 2.5 KB per molecule.
5. No write-ahead log. A molecule appended in memory but missing from the SMILES file is gone after a restart, so keep the file as the source of truth.
6. Row ids are u32, which caps how many rows one index can address.
7. The fingerprints are mossflux's own. The server computes 2048-bit ECFP4-style fingerprints, and these bits don't match RDKit or other toolkits, so scores can't be compared bit for bit.
8. Canonicalisation is your job. `score == 1.0` means equal bits, not the same compound.

No append-throughput number is claimed for 0.1.0, and [no billion-scale substructure claim](https://mossflux.com/docs/benchmarks) is made.

## Frequently asked questions

### Can I add molecules to a similarity search index without rebuilding it?

In mossflux, segmented append adds molecules to the index in memory with no rebuild and no change to any answer. The index isn't saved to disk, so a restart rebuilds it from the SMILES file.

### Does appending change the results?

Appending leaves the rule for a result unchanged: threshold and kNN search return the same rows as a brute-force scan of the full set. A prune rejects only on a proven bound, so a new segment can't hide a true hit or add a false one.

### Can I delete molecules from the index?

To drop a molecule, remove it from the SMILES file and restart.

### Is incremental search the same as merging search results?

They aren't the same. iSeqSearch is a tool that merges protein search results from separate runs. mossflux extends a live index, so new molecules become searchable with no rebuild and no result lists to merge.

## Next steps for an append-friendly molecular search workflow

The next steps are to check the 0.1.0 limits against your library, install mossflux from source and test an append against a brute-force scan of your own data.

Start with the [design](https://mossflux.com/docs/design) and [correctness](https://mossflux.com/docs/correctness) pages, then compare the limits with your library's size and update pattern.

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

Build an incremental molecular search index over your own SMILES file. Size memory at about 2.5 KB of RAM per molecule, [the rate measured on ChEMBL 37](https://mossflux.com/docs/deploy).

Run the aspirin query, `CC(=O)Oc1ccccc1C(=O)O`, before and after an append, and diff both answers against a brute-force scan of your own data.

[Run mossflux on your library](https://mossflux.com/#get) and check that the grown index returns the same rows as your scan.

## Sources

[mossflux: exact molecular search in Rust](https://github.com/loruorg/mossflux) · [mossflux docs](https://mossflux.com/docs/) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [Design · mossflux docs](https://mossflux.com/docs/design) · [GitHub - EESI/Incremental-Protein-Search](https://github.com/EESI/Incremental-Protein-Search) · [API · mossflux docs](https://mossflux.com/docs/api) · [Deploy · mossflux docs](https://mossflux.com/docs/deploy) · [mossflux benchmarks](https://mossflux.com/docs/benchmarks). Every other source is linked where the article cites it.
