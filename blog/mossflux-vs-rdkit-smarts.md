---
description: "mossflux vs RDKit SMARTS: pick RDKit when one molecule is the unit of work, mossflux when a whole SMILES library is. Compare stereo handling, limits and setup."
faq:
  - answer: Mossflux agrees with RDKit on all 114 of 114 queries in its RDKit differential, and the engine is also checked against a brute-force oracle, brute.rs. Chirality and atom maps are parsed and ignored, so stereochemistry-aware matching is outside what mossflux does. RDKit remains the tool for that.
    question: Does mossflux give the same SMARTS results as RDKit?
  - answer: "Mossflux can't search with stereochemistry, salts or tautomers. Chirality is parsed and ignored, and there is no salt or tautomer handling. Atom maps are parsed and ignored as well. Canonicalise salts, tautomers and stereo before ingestion. For stereochemistry-aware matching, RDKit remains the tool."
    question: Can mossflux search with stereochemistry, salts or tautomers?
  - answer: Mossflux does not have Python bindings, so you reach it through REST, gRPC, MCP or the CLI. Running mossflux serve starts one engine behind all three network transports (REST, gRPC and MCP), so the three transports cannot disagree with one another.
    question: Does mossflux have Python bindings?
  - answer: When a SMARTS search runs past its deadline, mossflux returns an error and never a partial list of matches. Subgraph matching is NP-complete, so some searches take a long time, and the server default deadline is 10 s with a maximum of 60 s.
    question: What happens when a SMARTS search takes too long?
  - answer: "Mossflux and RDKit Tanimoto scores can't be compared directly. The mossflux server computes its own 2048-bit ECFP4-style fingerprints, and those bits don't match the ones RDKit produces, so a score from one tool isn't interchangeable with a score from the other."
    question: Can I compare mossflux and RDKit Tanimoto scores directly?
image: /blog/mossflux-vs-rdkit-smarts/featured-f024bc0f60a6.webp
image_alt: "Illustration for Mossflux vs RDKit SMARTS: Which tool when?"
publishDate: "2026-11-18"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/mossflux-vs-rdkit-smarts#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\",\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q33083031\",\"@type\":\"Thing\",\"description\":\"server processor line by Advanced Micro Devices\",\"name\":\"Epyc\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Epyc\",\"https://www.wikidata.org/wiki/Q33083031\",\"https://www.google.com/search?kgmid=/g/11dy_kks3x\"]}],\"datePublished\":\"2026-11-18\",\"description\":\"mossflux vs RDKit SMARTS: pick RDKit when one molecule is the unit of work, mossflux when a whole SMILES library is. Compare stereo handling, limits and setup.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Mossflux agrees with RDKit on all 114 of 114 queries in its RDKit differential, and the engine is also checked against a brute-force oracle, brute.rs. Chirality and atom maps are parsed and ignored, so stereochemistry-aware matching is outside what mossflux does. RDKit remains the tool for that.\"},\"name\":\"Does mossflux give the same SMARTS results as RDKit?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Mossflux can't search with stereochemistry, salts or tautomers. Chirality is parsed and ignored, and there is no salt or tautomer handling. Atom maps are parsed and ignored as well. Canonicalise salts, tautomers and stereo before ingestion. For stereochemistry-aware matching, RDKit remains the tool.\"},\"name\":\"Can mossflux search with stereochemistry, salts or tautomers?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Mossflux does not have Python bindings, so you reach it through REST, gRPC, MCP or the CLI. Running mossflux serve starts one engine behind all three network transports (REST, gRPC and MCP), so the three transports cannot disagree with one another.\"},\"name\":\"Does mossflux have Python bindings?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"When a SMARTS search runs past its deadline, mossflux returns an error and never a partial list of matches. Subgraph matching is NP-complete, so some searches take a long time, and the server default deadline is 10 s with a maximum of 60 s.\"},\"name\":\"What happens when a SMARTS search takes too long?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Mossflux and RDKit Tanimoto scores can't be compared directly. The mossflux server computes its own 2048-bit ECFP4-style fingerprints, and those bits don't match the ones RDKit produces, so a score from one tool isn't interchangeable with a score from the other.\"},\"name\":\"Can I compare mossflux and RDKit Tanimoto scores directly?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q6120337\",\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\",\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q28865\",\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q364\",\"@type\":\"Organization\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q134856\",\"@type\":\"Thing\",\"description\":\"oxoacid having the structure RC(=O)OH, used as a suffix in systematic name formation to denote the –C(=O)OH group including its carbon atom\",\"name\":\"Carboxylic acid\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Carboxylic_acid\",\"https://www.wikidata.org/wiki/Q134856\",\"https://www.google.com/search?kgmid=/m/01ts1\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q26356541\",\"@type\":\"Thing\",\"description\":\"cross-platform binary RPC protocol\",\"name\":\"GRPC\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GRPC\",\"https://www.wikidata.org/wiki/Q26356541\",\"https://www.google.com/search?kgmid=/g/11cp5mklv8\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q749568\",\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]}],\"name\":\"Mossflux vs RDKit SMARTS: Which tool when?\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/JdNcP60x3f2epeTMkWX2LfIkqleDKpRg.webp\"},\"url\":\"https://mossflux.com/blog/mossflux-vs-rdkit-smarts\"},{\"@id\":\"https://mossflux.com/blog/mossflux-vs-rdkit-smarts#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\"},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"},{\"@id\":\"https://www.wikidata.org/wiki/Q33083031\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Substructure search with many SMARTS patterns · rdkit/rdkit · Discussion #7277\",\"url\":\"https://github.com/rdkit/rdkit/discussions/7277\"},{\"@type\":\"CreativeWork\",\"name\":\"rdkit.Chem.rdchem module\",\"url\":\"https://www.rdkit.org/docs/source/rdkit.Chem.rdchem.html\"},{\"@type\":\"CreativeWork\",\"name\":\"rdkit.Chem.rdmolfiles module\",\"url\":\"https://www.rdkit.org/docs/source/rdkit.Chem.rdmolfiles.html\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Daylight Theory: SMARTS - A Language for Describing Molecular Patterns\",\"url\":\"https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux/docs/benchmarks.md at main · loruorg/mossflux · GitHub\",\"url\":\"https://github.com/loruorg/mossflux/blob/main/docs/benchmarks.md\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Getting Started with the RDKit in Python\",\"url\":\"https://www.rdkit.org/docs/GettingStartedInPython.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"Changelog · mossflux docs\",\"url\":\"https://mossflux.com/docs/changelog\"}],\"dateModified\":\"2026-11-18\",\"datePublished\":\"2026-11-18\",\"description\":\"mossflux vs RDKit SMARTS: pick RDKit when one molecule is the unit of work, mossflux when a whole SMILES library is. Compare stereo handling, limits and setup.\",\"headline\":\"Mossflux vs RDKit SMARTS: Which tool when?\",\"image\":\"https://api.birchseek.com/m/JdNcP60x3f2epeTMkWX2LfIkqleDKpRg.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/mossflux-vs-rdkit-smarts#webpage\"},\"keywords\":[\"mossflux vs rdkit smarts\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/mossflux-vs-rdkit-smarts#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q6120337\"},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\"},{\"@id\":\"https://www.wikidata.org/wiki/Q28865\"},{\"@id\":\"https://www.wikidata.org/wiki/Q364\"},{\"@id\":\"https://www.wikidata.org/wiki/Q134856\"},{\"@id\":\"https://www.wikidata.org/wiki/Q26356541\"},{\"@id\":\"https://www.wikidata.org/wiki/Q749568\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2129},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"number of nonzero symbols in a string\",\"name\":\"Hamming weight\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hamming_weight\",\"https://www.wikidata.org/wiki/Q5645805\",\"https://www.google.com/search?kgmid=/m/048nw2\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - smarts
  - substructure search
  - rdkit
  - mossflux
  - cheminformatics
  - smiles
title: "Mossflux vs RDKit SMARTS: Which tool when?"
---

**mossflux vs RDKit SMARTS comes down to scope. Pick RDKit when the molecule is the unit of work, and mossflux when the library is.**

The choice comes up when a SMILES library of millions of rows, such as [ChEMBL 37 (2.9 million molecules)](https://mossflux.com/), has to return every true hit for a SMARTS query. A missed hit means a wrong answer downstream, in a screening cascade, a filter or an agent's tool call.

After reading, you will be able to match your job to the right tool, compare how each one reads SMARTS, and plan around what each one leaves out.

> **Key Takeaways**
> - 114 of 114 queries agree with RDKit in [the mossflux differential](https://mossflux.com/docs/correctness), and `brute.rs` is a brute-force oracle.
> - Chirality and atom maps are parsed and ignored; [recursive `$(...)` goes to depth 10](https://mossflux.com/docs/design).
> - A [timed-out search returns an error](https://mossflux.com/docs/api), never a partial list (server default 10 s, maximum 60 s).
> - mossflux's 2048-bit ECFP4-style fingerprints are its own, so its similarity scores and RDKit's aren't interchangeable.

## mossflux vs RDKit SMARTS: what does each tool do?

RDKit matches a SMARTS pattern against one molecule at a time with `GetSubstructMatch` and `GetSubstructMatches`, and offers `SubstructLibrary` for collections, while mossflux is a Rust engine for exact SMARTS search over a whole SMILES library.

![Illustration for mossflux vs RDKit SMARTS: what does each tool do?](/blog/mossflux-vs-rdkit-smarts/image-1-1cc024ba4571.webp)

In RDKit, [`Chem.MolFromSmarts`](https://www.rdkit.org/docs/source/rdkit.Chem.rdmolfiles.html) builds the query from a SMARTS string. [The RDKit documentation](https://www.rdkit.org/docs/source/rdkit.Chem.rdchem.html) says `GetSubstructMatch` returns "the indices of the molecule's atoms that match a substructure query", and `GetSubstructMatches` returns tuples of those indices.

For collections, `rdSubstructLibrary.SubstructLibrary` holds the molecules.

mossflux is written from scratch.

Two different jobs share the name "substructure search". One SMARTS against millions of molecules is the job mossflux is built for. Many SMARTS patterns against one molecule is the job raised in [RDKit discussion 7277](https://github.com/rdkit/rdkit/discussions/7277), and RDKit's per-molecule matching serves it.

The thread's author wanted "to build a fixed library of SMARTS patterns" to match against molecules, and took the substructure library to be "exactly the inverse case with one pattern and many molecules in a library."

Sending `S(=O)(=O)N` to [the public demo](https://mossflux.com/) at `https://api.mossflux.com`, which searches ChEMBL 37 (2.9 million molecules) on one small 4-thread server, is the first job. Running a set of profile patterns over one candidate structure is the second.

## mossflux vs RDKit SMARTS semantics: how do they differ?

mossflux and RDKit both read Daylight SMARTS, and they differ in chirality handling, in how matches are counted and in how much one call covers.

| Criterion | RDKit | mossflux |
| --- | --- | --- |
| Chirality in matching | `useChirality`, default `False` | Parsed, ignored |
| Match count | `maxMatches` cap per molecule | Exact; `--limit` caps printed hits |
| One call covers | One molecule | One SMILES file |

[The Daylight SMARTS theory manual](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) defines the primitives both tools read: `D<n>` is explicit connections, `X<n>` total connectivity, `H<n>` total hydrogen count, `v<n>` valence and `R<n>` ring membership. It also defines recursive SMARTS, written `$(...)`.

[mossflux's docs](https://mossflux.com/docs/design) list recursive `$(...)` to depth 10, `v<n>`, `X<n>`, `D` and `H`.

[RDKit's `GetSubstructMatches` signature](https://www.rdkit.org/docs/source/rdkit.Chem.rdchem.html) carries `useChirality=False` and `maxMatches=1000`. Stereo-aware matching is therefore opt-in, and each molecule's result stops at 1000 matches unless you raise the cap.

mossflux's docs state that atom maps and chirality parse and are ignored, so it does no stereochemistry-aware matching.

Four functional-group patterns and a SMILES that each one hits follow. Every SMILES is also a valid SMARTS, so each example can be pasted back in as a query.

| SMARTS | Matches | SMILES that hits |
| --- | --- | --- |
| `S(=O)(=O)N` | Sulfonamide | `CS(=O)(=O)N` |
| `C#N` | Nitrile | `CC#N` |
| `C(=O)[OH]` | Carboxylic acid, hydroxyl with one hydrogen | `CC(=O)O` |
| `c1ccncc1` | Pyridine ring, aromatic atoms | `c1ccncc1` |

The `[OH]` in the acid pattern requires exactly one hydrogen on the oxygen. Test aspirin, `CC(=O)Oc1ccccc1C(=O)O`, against `C(=O)[OH]`: the carboxylic acid carbon matches because its single-bonded oxygen carries one hydrogen.

The ester carbonyl does not match, because its single-bonded oxygen is bonded to the ring and has no hydrogen. A molecule with both groups therefore hits only at the acid.

RDKit suits per-molecule work that needs `useChirality`, a raised or lowered `maxMatches`, or the rest of the toolkit around the match. mossflux suits exact SMARTS search over a whole SMILES file, where stereochemistry is not part of the question.

## How does library-scale substructure search stay exact?

Library-scale substructure search stays exact because each step before the last one rejects a molecule only on a proven condition, and the last step settles every molecule that remains. The [mossflux design page](https://mossflux.com/docs/design) lists the pipeline in five steps: parse, perceive (rings and aromaticity, using the project's own Huckel model), path screen, census and VF2 verify.

The path screen is a superset filter. The query `c1ccncc1` needs an aromatic carbon bonded to an aromatic nitrogen, so a molecule with no nitrogen cannot contain that path and is safe to reject.

A molecule that has the path survives, even if it turns out not to hold the whole ring. The screen can let false positives through, but it cannot drop a true match, because a prune rejects only on a proven bound.

The census applies the same rule to counts. mossflux sorts atoms and bonds into 32 count classes. If the query needs two aromatic nitrogens and a molecule has one, the molecule is out.

For the exact classes, the census can answer without calling the matcher at all.

VF2 verify then runs the full subgraph match on the survivors.

RDKit documents `SubstructLibrary` for the case of one pattern against many molecules in a library, as a user in [an RDKit discussion thread](https://github.com/rdkit/rdkit/discussions/7277) describes it.

## How exact and how fast is mossflux SMARTS search?

mossflux makes exactness the contract: a SMARTS search returns the same rows as a brute-force scan, or it returns an error.

The [correctness page](https://mossflux.com/docs/correctness) names two checks. `brute.rs` is a brute-force oracle that the matcher is tested against. In the RDKit differential, 114 of 114 queries agree.

mossflux handles this with a deadline: the server default is 10 s and the maximum is 60 s, and past it the answer is an error, never a partial list.

[RDKit's closest control](https://www.rdkit.org/docs/source/rdkit.Chem.rdchem.html) is `maxMatches`, which caps the number of matches returned per molecule.

The mossflux API enforces these limits on every request:

| Limit | Value |
| --- | --- |
| Hits per request | 10,000 |
| Concurrent searches | 4 by default (more wait) |
| SMARTS size | up to 2,048 atoms |
| Query size | up to 4,096 bytes |
| Request body | up to 64 KiB |

Substructure speed ranges from 5.2 to 160 million molecules a second per thread.

[Those timings](https://github.com/loruorg/mossflux/blob/main/docs/benchmarks.md) were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates do pass on it. No billion-scale substructure claim is made.

The benchmarks page sets mossflux against RDKit and FPSim2 for similarity search. Its substructure timings are mossflux's own measurements, not a comparison with RDKit.

## mossflux vs RDKit SMARTS in practice: how do you send the same query?

You send the sulfonamide SMARTS `S(=O)(=O)N` to RDKit as a call on one molecule, and to mossflux as a search over a whole SMILES file or served database.

1. Match one molecule in RDKit. Build the query with `Chem.MolFromSmarts`, then call `GetSubstructMatches`. Pass `uniquify`, `useChirality` and `maxMatches` by name so the call states its own limits.

```python
from rdkit import Chem

query = Chem.MolFromSmarts('S(=O)(=O)N')
mol = Chem.MolFromSmiles('Nc1ccc(cc1)S(N)(=O)=O')
matches = mol.GetSubstructMatches(
    query, uniquify=True, useChirality=False, maxMatches=1000
)
```

2. Search a SMILES file from the shell. `mossflux substruct` runs the SMARTS over a SMILES file; take the argument order from [the CLI documentation](https://mossflux.com/docs/cli) rather than from memory. Its counts are exact, and `--limit` caps only the printed hits.

3. Post the query to the server. The `db` field picks the database when the server holds several.

```json
{"smarts": "S(=O)(=O)N", "limit": 100, "offset": 0, "timeout_ms": 5000}
```

To page, repeat the call with `offset` raised by `limit`.

If the deadline passes, the response is an error, so raise `timeout_ms` or narrow the SMARTS before retrying. The [API documentation](https://mossflux.com/docs/api) lists every field.

4. Call the MCP tool or gRPC. The MCP tool is `substructure_search`, reached over HTTP at `/mcp` or over stdio with `mossflux mcp`. The documented way to register the demo in Claude Code is `claude mcp add --transport http mossflux https://api.mossflux.com/mcp`. The gRPC route is service `mossflux.v1.Search`, method `Substructure`.

[Try the demo](https://mossflux.com/), which holds ChEMBL 37 (2.9 million molecules): send `S(=O)(=O)N` once through `POST /v1/substructure` and once through `substructure_search`.

The RDKit call in step 1 answers a different question: where the pattern sits in one molecule you already hold.

To run the same query on your own SMILES file, install the CLI from source:

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

`target-cpu=native` makes the binary run only on CPUs like the build machine.

## mossflux vs RDKit SMARTS: which tool when?

RDKit is the tool when a set of patterns or a stereo-specific query meets one molecule at a time, and mossflux is the tool when one SMARTS must run over every row of a SMILES library and return every true hit.

| Criterion | RDKit | mossflux |
| --- | --- | --- |
| Case served | A pattern set on one molecule | One SMARTS over a library |
| Stereochemistry in matching | `useChirality` option | Parsed, ignored |
| Index | None needed | In memory, rebuilt each start |
| Interface | Python API | REST, gRPC, MCP |
| Similarity bits | RDKit's own | mossflux's own |

A question in [RDKit's discussion forum](https://github.com/rdkit/rdkit/discussions/7277) describes "a fixed library of SMARTS patterns" matched against molecules, which is the many-patterns-on-one-molecule case.

[RDKit's documentation](https://www.rdkit.org/docs/GettingStartedInPython.html) shows chirality-aware matching through `useChirality=True`, and its tutorials cover SMARTS reactions. [mossflux parses chirality](https://mossflux.com/docs/design) and ignores it, so stereo-specific queries belong in RDKit.

The [deploy page](https://mossflux.com/docs/deploy) gives the sizing for the served case. The index is built in memory at start and isn't saved to disk.

For ChEMBL 37 (2,896,304 molecules), the build takes 33 to 41 s, the index takes 5.1 GB, and RAM use is about 2.5 KB per molecule.

For similarity search, the [benchmarks page](https://mossflux.com/docs/benchmarks) publishes timings with their conditions. Against FPSim2, mossflux was 2.64x to 2.77x faster: AMD EPYC 9645 VM, 256-bit Morgan r=2, ChEMBL 37 2.9M, 1 mossflux thread against 1 FPSim2 worker, search only, median of 11.

The two tools also work together. RDKit can prepare and canonicalise structures, and mossflux can then search the prepared SMILES file.

- **Best for RDKit:** Python workflows, SDF input, stereochemistry-aware matching, reactions, and many patterns against one molecule.

- **Best for mossflux:** exact search of a large SMILES library behind REST, gRPC or MCP.

## Frequently asked questions

### Does mossflux give the same SMARTS results as RDKit?

On [mossflux's RDKit differential](https://mossflux.com/docs/correctness), 114 of 114 queries agree, and the engine is also checked against a brute-force oracle, `brute.rs`. [Chirality and atom maps](https://mossflux.com/docs/design) are parsed and ignored, so stereochemistry-aware matching is outside what mossflux does, and RDKit remains the tool for it.

### Can mossflux search with stereochemistry, salts or tautomers?

No. Chirality is parsed and ignored, and there is no salt or tautomer handling, so [canonicalise salts, tautomers and stereo](https://mossflux.com/docs/design) before ingestion.

### Does mossflux have Python bindings?

No. Use [REST, gRPC, MCP or the CLI](https://mossflux.com/docs/api): `mossflux serve` runs one engine behind all three network transports, so they can't disagree.

[Past the deadline](https://mossflux.com/docs/cli), the answer is an error, never a partial list.

 [The server computes its own](https://mossflux.com/docs/api) 2048-bit ECFP4-style fingerprints, and those bits don't match RDKit's, so scores from the two tools aren't interchangeable.

## Which should you set up first?

Set up one SMARTS run through both tools on your own SMILES file first, because the counts settle the mossflux vs RDKit SMARTS choice for your data.

- Today: write down your case, one pattern against a library or many patterns against one molecule. Check each primitive in your SMARTS against the [Daylight SMARTS manual](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html).
- This week: run the same SMARTS (for example `S(=O)(=O)N`) through RDKit `GetSubstructMatches` and through `mossflux substruct` on a SMILES file, then compare the counts. mossflux is [release 0.1.0](https://mossflux.com/docs/changelog), and per [its API and install docs](https://mossflux.com/) it installs with `cargo install --git https://github.com/loruorg/mossflux mossflux-cli`.
- This month: if the library case holds, put `mossflux serve` behind your service and send explicit `timeout_ms` and `limit` values with every substructure request.

The test costs one SMILES file and one install, and RDKit stays your reference throughout. [Run it on your library](https://mossflux.com/).

## Sources

[mossflux · exact similarity search at billions of molecules…](https://mossflux.com/) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [Design · mossflux docs](https://mossflux.com/docs/design) · [API · mossflux docs](https://mossflux.com/docs/api) · [rdkit.Chem.rdchem module](https://www.rdkit.org/docs/source/rdkit.Chem.rdchem.html) · [Substructure search with many SMARTS patterns · rdkit/rdkit…](https://github.com/rdkit/rdkit/discussions/7277) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks) · [CLI · mossflux docs](https://mossflux.com/docs/cli). Every other source is linked where the article cites it.
