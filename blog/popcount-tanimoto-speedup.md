---
description: How do you get a popcount Tanimoto speedup? Build with hardware popcount, prune rows with an exact bound, then diff against brute force. Learn the steps.
faq:
  - answer: "A popcount Tanimoto speedup comes from two sources: hardware popcount makes each comparison cheaper, and BitBound-style pruning skips comparisons that a proven popcount bound rules out. A speedup is valid only if results equal a brute-force scan and the number states its machine, ISA, dataset, width, threshold, threads and trials."
    question: What is a popcount Tanimoto speedup?
  - answer: "You need to compile for a CPU that has POPCNT: -mpopcnt in GCC, or target-cpu=native in Rust. Without it, popcount uses a slower software routine. A native build runs only on CPUs like the build machine, so build once per target CPU class."
    question: Do I need -mpopcnt or target-cpu=native to get hardware popcount?
  - answer: "BitBound pruning doesn't change the results when the bound is exact. A row is skipped only when its popcount proves Tanimoto cannot reach the threshold, and the bound rounds so it never drops a true hit. Confirm it by diffing the output against a brute-force scan."
    question: Does BitBound pruning change the results of a threshold search?
  - answer: "You don't need either for hardware popcount, because POPCNT is a separate instruction. Vector methods are alternatives: Dalke's 2011 benchmark update added Imran Haque's SSE2 and SSSE3 code. Time any vector kernel against POPCNT on your own machine before adopting it."
    question: Do I need AVX2 or AVX-512 for fast Tanimoto search?
  - answer: A 256-bit fingerprint is 4 u64 words and a 2048-bit one is 32, so each comparison does 8 times the AND and popcount work and reads 8 times the bytes. Rows read per second falls by a similar factor once memory bandwidth limits the scan.
    question: Why do 256-bit and 2048-bit fingerprints search at different speeds?
image: /blog/popcount-tanimoto-speedup/featured-f687728bcab5.webp
image_alt: "Illustration for Popcount tanimoto speedup: Faster, still exact"
publishDate: "2026-11-10"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/popcount-tanimoto-speedup#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q5645805\",\"@type\":\"Thing\",\"description\":\"number of nonzero symbols in a string\",\"name\":\"Hamming weight\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hamming_weight\",\"https://www.wikidata.org/wiki/Q5645805\",\"https://www.google.com/search?kgmid=/m/048nw2\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q496579\",\"@type\":\"Thing\",\"description\":\"open standard for parallelizing\",\"name\":\"OpenMP\",\"sameAs\":[\"https://en.wikipedia.org/wiki/OpenMP\",\"https://www.wikidata.org/wiki/Q496579\",\"https://www.google.com/search?kgmid=/m/021pv1\"]}],\"datePublished\":\"2026-11-10\",\"description\":\"How do you get a popcount Tanimoto speedup? Build with hardware popcount, prune rows with an exact bound, then diff against brute force. Learn the steps.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A popcount Tanimoto speedup comes from two sources: hardware popcount makes each comparison cheaper, and BitBound-style pruning skips comparisons that a proven popcount bound rules out. A speedup is valid only if results equal a brute-force scan and the number states its machine, ISA, dataset, width, threshold, threads and trials.\"},\"name\":\"What is a popcount Tanimoto speedup?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You need to compile for a CPU that has POPCNT: -mpopcnt in GCC, or target-cpu=native in Rust. Without it, popcount uses a slower software routine. A native build runs only on CPUs like the build machine, so build once per target CPU class.\"},\"name\":\"Do I need -mpopcnt or target-cpu=native to get hardware popcount?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"BitBound pruning doesn't change the results when the bound is exact. A row is skipped only when its popcount proves Tanimoto cannot reach the threshold, and the bound rounds so it never drops a true hit. Confirm it by diffing the output against a brute-force scan.\"},\"name\":\"Does BitBound pruning change the results of a threshold search?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You don't need either for hardware popcount, because POPCNT is a separate instruction. Vector methods are alternatives: Dalke's 2011 benchmark update added Imran Haque's SSE2 and SSSE3 code. Time any vector kernel against POPCNT on your own machine before adopting it.\"},\"name\":\"Do I need AVX2 or AVX-512 for fast Tanimoto search?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A 256-bit fingerprint is 4 u64 words and a 2048-bit one is 32, so each comparison does 8 times the AND and popcount work and reads 8 times the bytes. Rows read per second falls by a similar factor once memory bandwidth limits the scan.\"},\"name\":\"Why do 256-bit and 2048-bit fingerprints search at different speeds?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q900402\",\"@type\":\"Thing\",\"description\":\"processor supplementary instruction set first introduced by Intel\",\"name\":\"SSE2\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SSE2\",\"https://www.wikidata.org/wiki/Q900402\",\"https://www.google.com/search?kgmid=/m/01t6bn\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q1366124\",\"@type\":\"Thing\",\"description\":\"instruction set\",\"name\":\"SSSE3\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SSSE3\",\"https://www.wikidata.org/wiki/Q1366124\",\"https://www.google.com/search?kgmid=/m/0chzbr\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q92938062\",\"@type\":\"Person\",\"description\":\"researcher (ORCID 0000-0001-7782-2852)\",\"name\":\"Imran S Haque\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q92938062\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q16987723\",\"@type\":\"Thing\",\"description\":\"instruction set extension developed by Intel\",\"name\":\"AVX-512\",\"sameAs\":[\"https://en.wikipedia.org/wiki/AVX-512\",\"https://www.wikidata.org/wiki/Q16987723\",\"https://www.google.com/search?kgmid=/m/0_v0j7q\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q17600\",\"@type\":\"Thing\",\"description\":\"extensions to the x86 instruction set architecture for microprocessors from Intel and AMD\",\"name\":\"Advanced Vector Extensions\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Advanced_Vector_Extensions\",\"https://www.wikidata.org/wiki/Q17600\",\"https://www.google.com/search?kgmid=/m/0405w4y\"]}],\"name\":\"Popcount tanimoto speedup: Faster, still exact\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/MW4sFUNQJxtoMUegXj3wFD_aDuTww2VI.webp\"},\"url\":\"https://mossflux.com/blog/popcount-tanimoto-speedup\"},{\"@id\":\"https://mossflux.com/blog/popcount-tanimoto-speedup#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q5645805\"},{\"@id\":\"https://www.wikidata.org/wiki/Q496579\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Faster block Tanimoto calculations - Dalke Scientific\",\"url\":\"http://www.dalkescientific.com/writings/diary/archive/2008/07/01/faster_tanimtocat_search_search.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Update: Faster population counts - Dalke Scientific\",\"url\":\"http://www.dalkescientific.com/writings/diary/archive/2011/11/02/faster_popcount_update.html\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux API documentation\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com\"}],\"dateModified\":\"2026-11-10\",\"datePublished\":\"2026-11-10\",\"description\":\"How do you get a popcount Tanimoto speedup? Build with hardware popcount, prune rows with an exact bound, then diff against brute force. Learn the steps.\",\"headline\":\"Popcount tanimoto speedup: Faster, still exact\",\"image\":\"https://api.birchseek.com/m/MW4sFUNQJxtoMUegXj3wFD_aDuTww2VI.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/popcount-tanimoto-speedup#webpage\"},\"keywords\":[\"popcount tanimoto speedup\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/popcount-tanimoto-speedup#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q900402\"},{\"@id\":\"https://www.wikidata.org/wiki/Q1366124\"},{\"@id\":\"https://www.wikidata.org/wiki/Q92938062\"},{\"@id\":\"https://www.wikidata.org/wiki/Q16987723\"},{\"@id\":\"https://www.wikidata.org/wiki/Q17600\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2730},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"hardware direct memory access over computer networks\",\"name\":\"Remote direct memory access\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Remote_direct_memory_access\",\"https://www.wikidata.org/wiki/Q2561570\",\"https://www.google.com/search?kgmid=/m/04599j\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - tanimoto similarity
  - popcount
  - bitbound pruning
  - molecular fingerprints
  - similarity search
  - benchmarking
title: "Popcount tanimoto speedup: Faster, still exact"
---

**A popcount Tanimoto speedup has two sources: a cheaper comparison and fewer comparisons. Once popcounts a and b are precomputed, T = c / (a + b - c) needs only c = popcount(A & B) per pair, and BitBound-style pruning over popcount-sorted data skips rows on a proven bound. The speedup counts only if the rows equal a brute-force scan and the rate states its conditions.**

Your threshold search over a compound library is too slow, and the usual advice is to use popcount. That advice rarely says which share of the work popcount covers. Without that, you can't predict a popcount Tanimoto speedup or report one.

After reading, you can build with the right ISA flag so hardware popcount is on and add a bound that never drops a true hit. You can then check the output against a brute-force scan and state the speedup with its conditions.

> **Key Takeaways**
> - At t = 0.85 and query popcount 100, only rows with popcount 85 to 117 can hit.
> - Diff row ids and scores against a plain brute-force scan on every query. Equal counts can hide two swapped rows.
> - Per-thread rates and one-machine aggregates are separate results, and aggregates across different machines are not comparable.

## How does popcount speed up Tanimoto similarity?

Popcount speeds up Tanimoto similarity by cutting each comparison to one AND and one popcount per 64-bit word, plus a single division, once both bit counts are stored. Tanimoto similarity is T = c / (a + b - c), with three terms:

![Illustration for How does popcount speed up Tanimoto similarity?](/blog/popcount-tanimoto-speedup/image-1-20890fd12742.webp)

- a = popcount(A), the bits set in the first fingerprint
- b = popcount(B), the bits set in the second
- c = popcount(A & B), the bits set in both

The denominator a + b - c counts the bits set in A or B. Subtracting c stops the shared bits being counted twice.

You compute a and b once per fingerprint when the database loads, and once for the query. Per pair, only c remains.

For a 1,024-bit fingerprint stored as 16 `u64` words, a pair costs 16 ANDs, 16 popcounts and a sum. Then comes one subtraction and one division. In Rust, with no `unsafe` code:

```rust
let c: u32 = q.iter().zip(row).map(|(x, y)| (x & y).count_ones()).sum();
let t = c as f64 / (a + b - c) as f64;
```

Take two 8-bit fingerprints, A = `11010110` and B = `10010011`. Then a = 5 and b = 4. A & B = `10010010`, so c = 3, and T = 3 / (5 + 4 - 3) = 3/6 = 0.5. Counting the union directly gives the same 6 bits.

The pair (c, a + b - c) is an exact integer fraction, and the f64 from the division is a rounded copy of it. Ties in a ranking and comparisons against a threshold can be settled on the integers.

Hardware popcount makes each comparison cheaper but leaves their number unchanged: a scan of N rows still makes N pairs. Dalke Scientific's [Faster block Tanimoto calculations](http://www.dalkescientific.com/writings/diary/archive/2008/07/01/faster_tanimtocat_search_search.html) is part 5 of a series on molecular fingerprints and covers block Tanimoto calculations in C.

## How do you get hardware popcount in compiled code?

You get hardware popcount in compiled code by compiling for a CPU that has the POPCNT instruction and calling the compiler's popcount function, which then emits that instruction instead of a software routine.

1. Check the CPU flag. POPCNT is reported through a CPUID feature flag. On Linux, `grep -m1 -o popcnt /proc/cpuinfo` prints `popcnt` when the flag is set. No output means the instruction is absent, so go to step 5.

2. In C or C++, call `__builtin_popcountll(x)` on each 64-bit word and build with `gcc -O3 -mpopcnt tanimoto.c`. Without `-mpopcnt` (or `-march=native`) GCC calls a slower software routine. The results stay correct, so nothing warns you.

3. In Rust, call `u64::count_ones`, which needs no `unsafe` code:

```rust
pub fn and_popcount(a: &[u64], b: &[u64]) -> u32 {
    a.iter().zip(b).map(|(x, y)| (x & y).count_ones()).sum()
}
```

 This returns c = popcount(A & B) for two equal-length fingerprints. A default build targets a generic x86-64 CPU, so set `target-cpu` as in step 4 to get the instruction.

4. Build for the machine that will run it. The mossflux CLI installs from source with cargo, and the build line is:

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

 The flag has the same effect as `-mpopcnt` in the GCC line: `count_ones` compiles to the hardware instruction. The binary then runs only on CPUs like the build machine, so build on the host that will serve searches. The [mossflux CLI documentation](https://mossflux.com/docs/cli) lists the subcommands.

5. Pick a fallback when there is no POPCNT. Dalke's [2011 popcount update](http://www.dalkescientific.com/writings/diary/archive/2011/11/02/faster_popcount_update.html) added code from Imran Haque (an SSE2 and SSSE3 version) and from Kim Walisch (improved platform-independent bitslicing and OpenMP support). Use the SSSE3 shuffle lookup if the CPU has SSSE3, and portable bitslicing otherwise. On AVX2 or AVX-512 hardware, vector popcount kernels are a further option; benchmark them against POPCNT on your own fingerprints before switching.

The order is POPCNT where present, SSSE3 or bitslicing otherwise, and vector kernels only if they win on your data. Dalke's benchmark compares lookup tables, bitslicing, SSSE3 and POPCNT on the hardware of its day, so rerun the comparison on your CPU before trusting its ranking.

To measure the cost of the software routine, build twice, once with and once without the flag, and time the same 1,024-bit scan. Record the CPU model and thread count beside the timing.

## Does fingerprint width change the popcount Tanimoto speedup?

Fingerprint width changes the popcount speedup by setting how many 64-bit words each comparison reads, while the per-word work stays one AND and one popcount. Word count is the width divided by 64:

- 256 bits: 4 `u64` words
- 1,024 bits: 16 words
- 2,048 bits: 32 words

A 2,048-bit pair therefore takes 8 times the AND-and-popcount work of a 256-bit pair. FPSim2 and Arthor are compared at 256 bits, while mossflux's server fingerprints are 2,048 bits.

The server computes its own 2048-bit ECFP4-style fingerprints, and [the API documentation](https://mossflux.com/docs/api) says these bits do not match RDKit or other toolkits. A query fingerprint from another toolkit won't line up with them.

A width that is not a multiple of 64 leaves a tail. A 100-bit fingerprint fills one word and 36 bits of a second, so 28 bits of that second word are padding.

Those padding bits must be zero. A padding bit set in both rows would count toward a, b and c, and the Tanimoto score would change.

Folding shrinks a wider fingerprint into a narrower one. Different bits can land on the same position, so scores at 256 bits differ from scores at 2,048 bits.

The brute-force check must use the same folded bits. Fold every row once, run the plain scan on those 256-bit rows, and compare its hits with the fast path row by row.

Checking a folded fast path against an unfolded scan would flag differences that aren't bugs, and could hide ones that are.

## How does BitBound pruning add to the popcount Tanimoto speedup?

BitBound pruning adds a second speedup because popcounts alone prove that most rows in a library cannot reach your threshold, so the search never reads their bits. Hardware popcount makes each comparison cheap; pruning removes comparisons, and a full popcount Tanimoto speedup needs both.

Start from T = c / (a + b - c). The shared-bit count c can't exceed the smaller popcount, so c <= min(a, b). T grows as c grows, which gives the bound:

**T(A,B) <= min(a, b) / max(a, b)**

For a threshold t, a row with popcount b can only be a hit if t*a <= b <= a/t. Both sides use only the stored popcounts a and b, so ruling a row out needs none of its fingerprint bits.

Sort the database by popcount and the surviving rows form one contiguous block. Two lookups find its ends, and every row outside the block is skipped untouched.

At t = 0.85 with a query of popcount 40, the arithmetic is:

- Lower limit: 0.85 x 40 = 34.
- Upper limit: 40 / 0.85 = 47.06, so the last popcount that can qualify is 47.
- A row with popcount 30 gives T <= 30/40 = 0.75, so it is skipped.
- A row with popcount 48 gives T <= 40/48 = 0.833, so it is skipped.
- A row with popcount 47 gives T <= 40/47 = 0.851, so it stays and gets a full comparison.

Only popcounts 34 to 47 get read. The rest of the popcount-sorted arena is never touched.

The bound has to be computed so it can never drop a true hit.

Write t as a fraction, 17/20, and compare integers instead: keep a row when 17 x a <= 20 x b and 20 x a >= 17 x b. Nothing is rounded, and the cut-offs are exactly 34 and 47.

A prune that skips rows only on a proven bound returns the same rows as a brute-force scan, with no false negatives and no false positives. The [design notes for mossflux's two engines](https://mossflux.com/docs/design) cover how its search is built around that rule.

To see the pruned window on a real library, [try the demo](https://mossflux.com/#demo) and compare hit counts at different thresholds.

## How do you report a popcount Tanimoto speedup with its conditions?

A popcount Tanimoto speedup is reproducible only when the ratio sits beside the machine, ISA, dataset, fingerprint width, threshold, threads and number of trials. mossflux's results follow that form, and its [benchmarks page](https://mossflux.com/docs/benchmarks) holds the full conditions.

Against FPSim2 at 256 bits, mossflux is 2.64x to 2.77x faster. The run used an AMD EPYC 9645 VM, 256-bit Morgan r=2 fingerprints and ChEMBL 37 (2.9M molecules). It compared one mossflux thread with one FPSim2 worker, timed search only, and took the median of 11 trials.

Per thread, on Arthor's ISA class (AVX2, a `znver3` build), mossflux reaches 0.93 to 1.45 billion molecules a second on synthetic 50M-row, 256-bit data. Arthor's published figure is 0.9 to 1.2 billion per thread.

On 8 cores of the EPYC machine, with a native build, ChEMBL at 256 bits and t = 0.85, mossflux reaches 21.4 billion molecules a second effective. That figure is an aggregate on one machine, so it doesn't compare with the per-thread rates.

| Result | Figure | Conditions |
| --- | --- | --- |
| Against FPSim2, 256 bits | 2.64x to 2.77x faster | EPYC 9645 VM, ChEMBL 37, 1 thread vs 1 worker, median of 11 |
| Per thread, Arthor's ISA class | 0.93 to 1.45 billion/s | AVX2 `znver3`, synthetic 50M rows, 256 bits |
| 8 cores, effective | 21.4 billion/s | EPYC native, ChEMBL, 256 bits, t = 0.85 |

No aggregate win over Arthor's 96-thread figure is claimed. Aggregates across different machines are not comparable.

The timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on it.

Use the FPSim2 row as a like-for-like single-thread check at 256 bits and the per-thread row to compare kernels on the same ISA class. The 8-core row sizes one machine of this kind.

To check a comparison, read the FPSim2 and Arthor publications or documentation for the machine and thread count behind each figure.

## How do you check a faster search returns brute-force results?

A faster search is valid only if it returns exactly what a brute-force scan returns: the same rows, the same scores and the same order. Time it after it passes that check.

Approximate search is usually judged by recall, where a few missed hits are accepted. Exact search allows no false negatives (a true hit missing) and no false positives (a row listed below the threshold), so a recall of 0.99 fails.

1. Keep a plain scan as the oracle: one loop that computes T = c / (a + b - c) for every row, with no pruning and no index. Don't optimise it; it only has to be easy to read and obviously right.
2. Run every fast path against it. Compare the pruned scan, the inverted bitmap index and the plain scan with each other, as well as with the oracle.
3. Compare the full result set on every query: row ids and scores, not counts. Two wrong rows can swap in and leave the count unchanged. Compare scores bit for bit as f64.
4. Fix the tie order for kNN. Rank by exact fraction, then lower popcount, then the earlier row, in the oracle and in the fast path. Without one rule, a correct search looks wrong.
5. Test thresholds that fall exactly on a score. If a pair has a = 20, b = 22 and c = 17, then T = 17/25, so run t = 0.68 and expect that row in the output. A bound that rounds the wrong way drops it. Compare in integer fractions where you can, because a decimal 0.68 carries rounding error.
6. Fail the run on any difference. Exit non-zero and print the query and the first differing row. Success is zero differences across every query and every threshold.

A score of `1.0` means equal bits, not the same compound. Canonicalise salts, tautomers and stereo before ingestion, or two forms of one compound will score below 1.0 against each other.

mossflux's stated rule is that [every result matches a brute-force scan](https://mossflux.com). Its [correctness page](https://mossflux.com/docs/correctness) covers what "exact" means and the tests that prove it.

## Frequently asked questions

### What is a popcount Tanimoto speedup?

It is the gain from two changes to a fingerprint search: hardware popcount makes each comparison cheaper, and BitBound-style pruning skips comparisons that a proven popcount bound rules out. It's valid only if results equal a brute-force scan and the number states its machine, ISA, dataset, width, threshold, threads and trials.

### Do I need -mpopcnt or target-cpu=native to get hardware popcount?

You need to compile for a CPU that has POPCNT: -mpopcnt in GCC, or target-cpu=native in Rust. Without it, popcount uses a [slower software routine](https://mossflux.com/docs/cli).

### Does BitBound pruning change the results of a threshold search?

BitBound pruning doesn't change the results when the bound is exact. A row is skipped only when its popcount proves Tanimoto cannot reach the threshold, and the bound rounds so it never drops a true hit. Confirm it by diffing the output against a brute-force scan.

### Do I need AVX2 or AVX-512 for fast Tanimoto search?

You need neither for hardware popcount, because POPCNT is a separate instruction. Vector methods are alternatives: Dalke's [2011 benchmark update](http://www.dalkescientific.com/writings/diary/archive/2011/11/02/faster_popcount_update.html) added Imran Haque's SSE2 and SSSE3 code. Time any vector kernel against POPCNT on your own machine before adopting it.

### Why do 256-bit and 2,048-bit fingerprints search at different speeds?

A 256-bit fingerprint is 4 u64 words and a 2,048-bit one is 32, so each comparison does 8 times the AND and popcount work and reads 8 times the bytes. Rows read per second falls by a similar factor once memory bandwidth limits the scan.

## What should you do next?

Take three steps towards a popcount Tanimoto speedup, in order of effort.

**Today:** build with the target ISA and confirm the compiler emits hardware popcount. Use `-mpopcnt` with GCC or `RUSTFLAGS="-C target-cpu=native"` with Rust, then disassemble the hot loop and look for `popcnt`. Build on each kind of machine you deploy to.

**This week:** add a popcount-window prune to your threshold search. Compute the bound with integer arithmetic and round it down, so a true hit is never dropped. Then run a brute-force scan beside it on every query and fail on any difference in rows, scores or order.

**This month:** record the conditions with every popcount Tanimoto speedup you publish: machine, ISA, dataset, fingerprint width, threshold, threads and trials. A ratio without them can't be reproduced.

To try all three on your own data, [run mossflux on your library](https://mossflux.com/#get). It installs from source with `cargo install --git https://github.com/loruorg/mossflux mossflux-cli`, and you can diff its output against your current tool on the same molecules. The [mossflux docs](https://mossflux.com/docs/) are the next read.

## Sources

[Faster block Tanimoto calculations - Dalke Scientific](http://www.dalkescientific.com/writings/diary/archive/2008/07/01/faster_tanimtocat_search_search.html) · [Update: Faster population counts - Dalke Scientific](http://www.dalkescientific.com/writings/diary/archive/2011/11/02/faster_popcount_update.html) · [mossflux API documentation](https://mossflux.com/docs/api) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks) · [mossflux · exact similarity search at billions of molecules…](https://mossflux.com) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness)
