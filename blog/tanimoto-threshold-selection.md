---
description: Tanimoto threshold selection has no universal value. Measure the noise floor, recall and hit count for your fingerprint, then set t. Learn the four steps.
faq:
  - answer: Tanimoto has no single general cut-off, so 0.7 is not automatically a good one. The score that signals activity-relevant similarity depends on the fingerprint, and Landrum notes that scores from different fingerprints can be very different. Measure the noise floor and recall for your fingerprint, then set t.
    question: Is 0.7 a good Tanimoto threshold?
  - answer: "A threshold from RDKit does not carry over to mossflux. The server computes its own 2048-bit ECFP4-style fingerprints, and those bits don't match RDKit's. Repeat the noise-floor and recall steps on mossflux's bits, and state fingerprint, width and t with every score."
    question: Can I reuse a threshold from RDKit in mossflux?
  - answer: Pruning does not change which molecules mossflux returns at a given threshold, only how fast it returns them. BitBound pruning skips rows that cannot reach t, and every answer is exactly what a brute-force scan returns at the same t. A molecule scoring exactly t is kept, because a threshold means score >= t.
    question: Does pruning change which molecules mossflux returns at a given threshold?
  - answer: A score of 1.0 means the two fingerprints have equal bits, and different structures can set identical bits, so it does not confirm the same molecule. This happens more often after folding to a narrow width such as 256. If identity matters, compare canonical SMILES.
    question: Does a Tanimoto score of 1.0 mean the same molecule?
image: /blog/tanimoto-threshold-selection/featured-23c563867129.webp
image_alt: "Illustration for Tanimoto threshold selection: How to set t"
publishDate: "2026-11-11"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/tanimoto-threshold-selection#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\",\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\",\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]}],\"datePublished\":\"2026-11-11\",\"description\":\"Tanimoto threshold selection has no universal value. Measure the noise floor, recall and hit count for your fingerprint, then set t. Learn the four steps.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Tanimoto has no single general cut-off, so 0.7 is not automatically a good one. The score that signals activity-relevant similarity depends on the fingerprint, and Landrum notes that scores from different fingerprints can be very different. Measure the noise floor and recall for your fingerprint, then set t.\"},\"name\":\"Is 0.7 a good Tanimoto threshold?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A threshold from RDKit does not carry over to mossflux. The server computes its own 2048-bit ECFP4-style fingerprints, and those bits don't match RDKit's. Repeat the noise-floor and recall steps on mossflux's bits, and state fingerprint, width and t with every score.\"},\"name\":\"Can I reuse a threshold from RDKit in mossflux?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Pruning does not change which molecules mossflux returns at a given threshold, only how fast it returns them. BitBound pruning skips rows that cannot reach t, and every answer is exactly what a brute-force scan returns at the same t. A molecule scoring exactly t is kept, because a threshold means score \\u003e= t.\"},\"name\":\"Does pruning change which molecules mossflux returns at a given threshold?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A score of 1.0 means the two fingerprints have equal bits, and different structures can set identical bits, so it does not confirm the same molecule. This happens more often after folding to a narrow width such as 256. If identity matters, compare canonical SMILES.\"},\"name\":\"Does a Tanimoto score of 1.0 mean the same molecule?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q6120337\",\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q126094665\",\"@type\":\"Thing\",\"description\":\"type of molecular fingerprint\",\"name\":\"extended-connectivity fingerprint\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q126094665\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q27701587\",\"@type\":\"Thing\",\"description\":\"open access publishing platform owned by Taylor \\u0026 Francis\",\"name\":\"F1000Research\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q27701587\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q334640\",\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]}],\"name\":\"Tanimoto threshold selection: How to set t\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/hfeK_nyQD8YobjIQGqSr1NvpAJ66GBNT.webp\"},\"url\":\"https://mossflux.com/blog/tanimoto-threshold-selection\"},{\"@id\":\"https://mossflux.com/blog/tanimoto-threshold-selection#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\"},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"Please stop saying \\\"The Tanimoto similarity is\\\" - RDKit blog\",\"url\":\"https://greglandrum.github.io/rdkit-blog/posts/2025-07-17-naming-similarity-metrics.html\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"6.2: Similarity Coefficients - Chemistry LibreTexts\",\"url\":\"https://chem.libretexts.org/Courses/Intercollegiate_Courses/Cheminformatics/06:_Molecular_Similarity/6.02:_Similarity_Coefficients\"},{\"@type\":\"CreativeWork\",\"name\":\"Activity-relevant similarity values for fingerprints and implications for similarity searching (F1000Research 2016)\",\"url\":\"https://f1000research.com/articles/5-591\"},{\"@type\":\"CreativeWork\",\"name\":\"Fingerprint similarity thresholds for database searches\",\"url\":\"https://greglandrum.github.io/rdkit-blog/posts/2021-05-21-similarity-search-thresholds.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"}],\"dateModified\":\"2026-11-11\",\"datePublished\":\"2026-11-11\",\"description\":\"Tanimoto threshold selection has no universal value. Measure the noise floor, recall and hit count for your fingerprint, then set t. Learn the four steps.\",\"headline\":\"Tanimoto threshold selection: How to set t\",\"image\":\"https://api.birchseek.com/m/hfeK_nyQD8YobjIQGqSr1NvpAJ66GBNT.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/tanimoto-threshold-selection#webpage\"},\"keywords\":[\"tanimoto threshold selection\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/tanimoto-threshold-selection#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q6120337\"},{\"@id\":\"https://www.wikidata.org/wiki/Q126094665\"},{\"@id\":\"https://www.wikidata.org/wiki/Q27701587\"},{\"@id\":\"https://www.wikidata.org/wiki/Q334640\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2444},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"number of nonzero symbols in a string\",\"name\":\"Hamming weight\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hamming_weight\",\"https://www.wikidata.org/wiki/Q5645805\",\"https://www.google.com/search?kgmid=/m/048nw2\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"hardware direct memory access over computer networks\",\"name\":\"Remote direct memory access\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Remote_direct_memory_access\",\"https://www.wikidata.org/wiki/Q2561570\",\"https://www.google.com/search?kgmid=/m/04599j\"]},{\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - tanimoto similarity
  - chemical fingerprints
  - similarity search
  - cheminformatics
  - bitbound pruning
title: "Tanimoto threshold selection: How to set t"
---

**Tanimoto threshold selection is a measurement you repeat for each fingerprint: find the noise floor, decide the recall you accept, estimate result-set size, then set t. Tanimoto scores from different fingerprints [can differ very widely](https://greglandrum.github.io/rdkit-blog/posts/2025-07-17-naming-similarity-metrics.html), so 0.85 is not a general cut-off. Record the fingerprint type, radius and width beside every threshold you report.**

Run a threshold search over ChEMBL 37 and the cut-off decides two things: which analogues you find and how many rows you must read. A loose cut-off can run into the [server's per-request hit cap](https://mossflux.com/docs/api), while a strict one returns few rows.

That trade is the heart of Tanimoto threshold selection, and you've got to make it for your own fingerprint. You'll need to pick t, estimate the result-set size before the query runs, and justify the value to whoever reads the hits.

> **Key Takeaways**
> - A threshold chosen at 2048 bits must be re-chosen after folding to 256 bits, since [scores shift with the fingerprint](https://greglandrum.github.io/rdkit-blog/posts/2025-07-17-naming-similarity-metrics.html).
> - In exact search, t decides which rows come back. Pruning changes only speed, and [the rows equal a brute-force scan](https://mossflux.com/docs/) at the same t.
> - Hit rates per million molecules scale linearly with library size, so a sample predicts the full result set.
> - When a query would exceed the server's hit cap, raise t or switch to kNN.

## What does a threshold mean in Tanimoto threshold selection?

A Tanimoto threshold is the minimum score a molecule needs to be returned, and its meaning depends on the fingerprint that produced the score. Tanimoto is the number of bits set in both fingerprints divided by the number of bits set in either.

That's why the same cut-off selects different molecules under different fingerprints, radii, encodings and folded widths.

Dice is a separate coefficient in the [standard list of similarity metrics](https://chem.libretexts.org/Courses/Intercollegiate_Courses/Cheminformatics/06:_Molecular_Similarity/6.02:_Similarity_Coefficients). It divides twice the shared bits by the sum of the bits set in each fingerprint, so it scores the same pair higher than Tanimoto does.

Assume two fingerprints share 4 bits, with 8 and 6 bits set. Tanimoto is 4 / (8 + 6 - 4) = 0.4. Dice is 2 × 4 / (8 + 6) = 0.57.

Assume a second encoding of the same structures, such as a larger radius, sets 14 and 12 bits with the same 4 shared. Tanimoto falls to 4 / (14 + 12 - 4) = 0.18. A cut-off of 0.3 returns the pair under the first encoding and drops it under the second.

Four settings move the score:

- Fingerprint type: each type records different features, so the set bits differ.
- Radius: a larger radius adds bits for bigger environments, which usually lowers shared fractions.
- Bits or counts: a bit vector records presence only, while a count vector records how often a feature occurs, so the same pair scores differently.
- Folded width: folding wraps a long fingerprint onto fewer bits.

Folding makes unrelated features share a bit (a collision), and that changes both the bits set and the scores. A threshold t chosen at one width has to be chosen again at another.

## Where does the 0.85 Tanimoto cut-off come from?

The 0.85 cut-off traces to Patterson et al., a paper on neighbourhood behaviour in the [*Journal of Medicinal Chemistry*](https://chem.libretexts.org/Courses/Intercollegiate_Courses/Cheminformatics/06:_Molecular_Similarity/6.02:_Similarity_Coefficients) (1996, 39:3049-3059). Its title describes the idea as a "useful concept for validation of 'molecular diversity' descriptors".

Neighbourhood behaviour asks one narrow question: do molecules that sit close together under a descriptor tend to share biological activity? The paper used that test to judge descriptors. So 0.85 describes the score scale of the descriptors that paper measured, and it carries no meaning for Tanimoto in general.

Greg Landrum, who maintains RDKit, makes the same point in [his post on naming similarity metrics](https://greglandrum.github.io/rdkit-blog/posts/2025-07-17-naming-similarity-metrics.html): "the Tanimoto similarities calculated using different fingerprints can be very, very different."

A radius-2 Morgan fingerprint, a radius-3 one and a 256-bit folded one each give a different number. A cut-off tuned on one setup doesn't carry over to an ECFP-style fingerprint.

Jasial, Hu, Vogt and Bajorath addressed this directly in F1000Research in 2016. Their paper, ["Activity-relevant similarity values for fingerprints and implications for similarity searching"](https://f1000research.com/articles/5-591), ties similarity values to activity separately for each fingerprint, so each one has its own value.

Read the numbers from that paper for the fingerprint you use, and check that your settings match theirs before you apply one.

The same rule shows up in code. RDKit's generator interface asks you to choose the fingerprint type, radius, bits or counts, and width before any score exists. A threshold belongs to that whole setup.

## How does Tanimoto threshold selection work step by step?

Tanimoto threshold selection means measuring four things for one fingerprint, in order: the noise floor, the recall you accept, the expected result-set size, and then t.

1. Measure the noise floor. Score a few thousand random pairs from your own library with your fingerprint, then note the high end of the distribution (for example the 95th percentile). A t at or below that level returns unrelated molecules.

 Greg Landrum's RDKit blog post on [fingerprint similarity thresholds for database searches](https://greglandrum.github.io/rdkit-blog/posts/2021-05-21-similarity-search-thresholds.html) applies the same idea to one fingerprint, and it builds sets of "related compounds" to test against.

2. Decide the recall you accept. Collect pairs you know are related, such as members of one series, and score them. Pick the share you can afford to miss, then read off the t that keeps the rest.

 A published recall table holds only for the fingerprint and dataset its author states. A different fingerprint needs its own table.

3. Estimate the result-set size. Run your candidate t over a sample and count the hits. Hits per million, multiplied by library size in millions, gives the expected rows. If that count is too large to read or ship, raise t, or switch to kNN or a hit cap.

4. Set t, then verify. Run real queries and compare the returned count with your estimate. Because mossflux search is exact, [every answer is exactly what a brute-force scan returns](https://mossflux.com/docs/). A threshold means score >= t, so a molecule scoring exactly t is included.

5. Repeat after any change. Fold the width, change the radius or switch between bits and counts, and steps 1 to 4 start again.

 A t measured on RDKit Morgan2 bits isn't reusable for mossflux, whose server computes 2048-bit ECFP4-style bits that, in the [API docs'](https://mossflux.com/docs/api) words, "are mossflux's own and do not match RDKit or other toolkits".

Assume the query is aspirin, `CC(=O)Oc1ccccc1C(=O)O`, and the library holds 2.9 million molecules. Assume the noise-floor sample tops out at 0.25, and a related-compound set shows that t = 0.55 keeps the recall you accept.

A test at 0.55 on a sample then returns 40 hits per million. The expected size is 40 x 2.9 = 116 hits.

If the live query returns about that many, with every score at or above 0.55, the threshold holds. If it returns thousands, the sample wasn't representative, and you go back to step 3.

## How many hits can you afford: lower t, kNN or a hit cap?

The hits you can afford decide the control: lower t when a measured recall target allows the hit rate, kNN when you need a fixed count, and a hit cap when output must stay bounded.

| Criterion | Lower t | kNN | Hit cap |
| --- | --- | --- | --- |
| Result size | Grows with library size | Fixed at k | At most the limit |
| Rows returned | Every score >= t | Top k by exact fraction | Same rows, fewer shown or refused |
| Best when | Recall target is set | You need k neighbours | Output or payload is bounded |

Start with a hit rate per million molecules, because it scales linearly with library size. Assume a threshold returns 50 hits per million.

A library of 2,896,304 molecules, the size of [ChEMBL 37](https://mossflux.com/docs/deploy), then returns about 145 hits. If a lower t raises the rate to 5,000 per million, the same library returns about 14,500 hits. Measure the rate on a sample for your fingerprint and width before you lower t.

kNN fixes the count instead of the score. mossflux ranks by exact fraction, and [ties go to the lower popcount, then the earlier row](https://mossflux.com/docs/design). The result is the same on every run.

kNN still returns k rows when the nearest molecules score poorly, so don't trust them until you've checked the scores. It suits a downstream step that expects k rows per query.

A hit cap bounds output and never changes which molecules qualify. The server accepts [at most 10,000 hits per request](https://mossflux.com/docs/api).

In the [mossflux CLI](https://mossflux.com/docs/cli), `--limit` caps only the printed hits, and the counts stay exact. A limit refuses an oversized request; it never silently shortens the list.

If a threshold sized for 2.9 million molecules would return more than 10,000 hits, raise t or switch to kNN. Raising the cap doesn't help, because the server won't exceed it.

## How do you run a threshold query for aspirin?

A threshold query for aspirin sends its SMILES, a minimum score and a hit cap to mossflux, which returns every database molecule whose Tanimoto score is at least that minimum.

1. Install the CLI. The [mossflux docs](https://mossflux.com/docs/) install it from source with Rust 1.95 or later (release 0.1.0). There are no prebuilt binaries and no crates.io release. A `target-cpu=native` binary runs only on CPUs like the build machine, so build on the machine that will run it.

   ```sh
   RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
   ```

2. Serve a database. Start `mossflux serve --db chembl=PATH`, with your SMILES file as `PATH`. The server answers REST on port 8080 once `GET /ready` stops returning 503.

3. Send the request. Save this as `aspirin.json`. The threshold of 0.6 is an assumed value, so replace it with the one you measured for your fingerprint.

   ```json
   {"smiles": "CC(=O)Oc1ccccc1C(=O)O", "threshold": 0.6, "limit": 100, "db": "chembl"}
   ```

   ```sh
   curl -X POST http://localhost:8080/v1/similarity -H "Content-Type: application/json" -d @aspirin.json
   ```

 The [API reference](https://mossflux.com/docs/api) lists the fields of the response. A successful call returns hits with exact `f64` scores. Every score is at least 0.6, and `limit` caps how many rows come back.

4. Read the scores. Strip salts, fix tautomers and settle stereo before ingestion, because mossflux doesn't do it for you.

5. Cross-check a pair. `mossflux tanimoto` scores a single pair, and `mossflux search` runs the same query over an FPS file. The [CLI page](https://mossflux.com/docs/cli) lists the flags. To get RDKit's score for the same pair, use the generator interface:

   ```python
   from rdkit import Chem, DataStructs
   from rdkit.Chem import rdFingerprintGenerator
   gen = rdFingerprintGenerator.GetMorganGenerator(radius=2, fpSize=2048)
   fp = gen.GetFingerprint(Chem.MolFromSmiles("CC(=O)Oc1ccccc1C(=O)O"))
   ```

 Score two such fingerprints with `DataStructs.TanimotoSimilarity`.

The two scores will differ, because the server computes its own 2048-bit ECFP4-style fingerprints. Report each score with the fingerprint, width and threshold beside it, such as "mossflux ECFP4-style, 2048 bits, t = 0.6".

[Try the demo](https://mossflux.com/#demo) to run the aspirin query against ChEMBL 37.

## Does BitBound pruning change which rows a threshold returns?

BitBound pruning never changes which rows a threshold returns. It changes only how many rows the engine must score to find them.

A query with `a` bits set and a row with `b` bits set can share at most `min(a, b)` bits, and the union is at least `max(a, b)`. So the score can never exceed `min(a, b) / max(a, b)`.

Sort the arena by popcount and the bound becomes a window. Take a query with 40 bits set and t = 0.85: only rows with 34 to 47 bits set can reach the threshold, so every row outside that range is skipped unscored. A higher t narrows the window and skips more rows.

The mossflux [design docs](https://mossflux.com/docs/design) describe a rational bound rounded down, so a true hit sitting exactly on the boundary is never dropped. The [correctness docs](https://mossflux.com/docs/correctness) state the result: the rows equal a brute-force scan at the same t, and an inverted bitmap index returns the same rows as the linear scan.

One speed result has stated conditions: 8 cores, EPYC native, ChEMBL 256-bit fingerprints, t = 0.85, which gave 21.4 billion molecules a second effective. The remaining conditions are on the [benchmarks page](https://mossflux.com/docs/benchmarks).

The timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on it. No aggregate win over any other tool is claimed, and no speed figure is given for other thresholds.

## Frequently asked questions

### Is 0.7 a good Tanimoto threshold?

No single Tanimoto value is a general cut-off. The score that signals activity-relevant similarity depends on the fingerprint, and Landrum notes that scores from different fingerprints [can be very different](https://greglandrum.github.io/rdkit-blog/posts/2025-07-17-naming-similarity-metrics.html). Measure the noise floor and recall for your fingerprint, then set t.

### Can I reuse a threshold from RDKit in mossflux?

A threshold from RDKit does not carry over. The server computes its own 2048-bit ECFP4-style fingerprints, and those bits don't match RDKit's. Repeat the noise-floor and recall steps on mossflux's bits.

### Does pruning change which molecules mossflux returns at a given threshold?

Pruning changes only speed. BitBound pruning skips rows that cannot reach t, and [every answer is exactly what a brute-force scan returns](https://mossflux.com/docs/) at the same t. A molecule scoring exactly t is kept, because a threshold means score >= t.

### Does a Tanimoto score of 1.0 mean the same molecule?

A score of 1.0 means the two fingerprints have equal bits. Different structures can set identical bits, more often after folding to a narrow width such as 256. If identity matters, compare canonical SMILES.

## Next steps for Tanimoto threshold selection

The next step in Tanimoto threshold selection is to measure t on your own library, starting with the decisions that need no new tooling.

**This week:**

- Decide the recall you accept: the share of known analogues you can afford to miss.
- Estimate hits per million molecules at two or three candidate values of t.
- Set t and a hit cap, so a low cut-off can't flood your results.

**Next:**

- Re-check t after any change of fingerprint or folded width, such as 2048 to 256 bits.
- Run the threshold on your own library and compare the hit count with your estimate.

[Run it on your library with mossflux](https://mossflux.com/#get). Start with a sample of a few thousand rows, so a wrong t costs you one query. Change it and run again until the hit count fits your budget.

## Sources

[Please stop saying "The Tanimoto similarity is" - RDKit blog](https://greglandrum.github.io/rdkit-blog/posts/2025-07-17-naming-similarity-metrics.html) · [mossflux docs](https://mossflux.com/docs/) · [6.2: Similarity Coefficients - Chemistry LibreTexts](https://chem.libretexts.org/Courses/Intercollegiate_Courses/Cheminformatics/06:_Molecular_Similarity/6.02:_Similarity_Coefficients) · [Activity-relevant similarity values for fingerprints and…](https://f1000research.com/articles/5-591) · [Fingerprint similarity thresholds for database searches](https://greglandrum.github.io/rdkit-blog/posts/2021-05-21-similarity-search-thresholds.html) · [Design · mossflux docs](https://mossflux.com/docs/design) · [API · mossflux docs](https://mossflux.com/docs/api) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks). Every other source is linked where the article cites it.
