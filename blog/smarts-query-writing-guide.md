---
description: "Get a SMARTS query writing guide: describe each atom with explicit primitives, combine them with logical operators, add recursive $(...), and test hits by hand."
faq:
  - answer: You write a SMARTS query by describing each atom with SMARTS primitives, combining them with the symbols !, &, comma and semicolon, and joining atoms with bond symbols. A SMILES is a valid SMARTS, but O then means any aliphatic oxygen, not water. Test the query against known hits and non-hits before running it on a library.
    question: How do I write a SMARTS query?
  - answer: The difference between H and h is that H counts total hydrogens and h counts implicit hydrogens, as the Daylight manual defines them. On an atom with explicit hydrogens the two give different counts. In mossflux queries, use H for hydrogen counts.
    question: What is the difference between H and h in SMARTS?
  - answer: mossflux does not do stereo-aware matching, salt stripping or tautomer handling in a SMARTS search. It parses chirality and atom maps and then ignores both. Because it does no salt or tautomer cleanup, canonicalise your SMILES library before you load it into mossflux.
    question: Does mossflux handle stereochemistry, salts or tautomers in a SMARTS search?
  - answer: A SMARTS search that passes its deadline returns an error, never a partial list. The server default is 10 s and the maximum is 60 s, and you set the deadline with timeout_ms on POST /v1/substructure. Subgraph matching is NP-complete, so queries from untrusted users need one.
    question: What happens if my SMARTS search takes too long?
  - answer: One request to POST /v1/substructure returns up to 10,000 hits. To read a larger result set, page through it with the limit and offset parameters, raising offset by the value of limit on each call so that each request continues where the previous one stopped.
    question: How many hits can one substructure request return?
image: /blog/smarts-query-writing-guide/featured-f83f838f213b.webp
image_alt: "Illustration for SMARTS query writing guide: Avoid bad matches"
publishDate: "2026-11-09"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/smarts-query-writing-guide#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q7544680\",\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]}],\"datePublished\":\"2026-11-09\",\"description\":\"Get a SMARTS query writing guide: describe each atom with explicit primitives, combine them with logical operators, add recursive $(...), and test hits by hand.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You write a SMARTS query by describing each atom with SMARTS primitives, combining them with the symbols !, \\u0026, comma and semicolon, and joining atoms with bond symbols. A SMILES is a valid SMARTS, but O then means any aliphatic oxygen, not water. Test the query against known hits and non-hits before running it on a library.\"},\"name\":\"How do I write a SMARTS query?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"The difference between H and h is that H counts total hydrogens and h counts implicit hydrogens, as the Daylight manual defines them. On an atom with explicit hydrogens the two give different counts. In mossflux queries, use H for hydrogen counts.\"},\"name\":\"What is the difference between H and h in SMARTS?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux does not do stereo-aware matching, salt stripping or tautomer handling in a SMARTS search. It parses chirality and atom maps and then ignores both. Because it does no salt or tautomer cleanup, canonicalise your SMILES library before you load it into mossflux.\"},\"name\":\"Does mossflux handle stereochemistry, salts or tautomers in a SMARTS search?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A SMARTS search that passes its deadline returns an error, never a partial list. The server default is 10 s and the maximum is 60 s, and you set the deadline with timeout_ms on POST /v1/substructure. Subgraph matching is NP-complete, so queries from untrusted users need one.\"},\"name\":\"What happens if my SMARTS search takes too long?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"One request to POST /v1/substructure returns up to 10,000 hits. To read a larger result set, page through it with the limit and offset parameters, raising offset by the value of limit on each call so that each request continues where the previous one stopped.\"},\"name\":\"How many hits can one substructure request return?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\",\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\",\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q170409\",\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q283\",\"@type\":\"Thing\",\"description\":\"chemical compound whose molecules are formed by two hydrogen atoms and one oxygen atom\",\"name\":\"Water\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Water\",\"https://www.wikidata.org/wiki/Q283\",\"https://www.google.com/search?kgmid=/m/0838f\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\",\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q210390\",\"@type\":\"Thing\",\"description\":\"phenomenon providing chemical stability in resonating hybrids of cyclic organic compounds\",\"name\":\"Aromaticity\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Aromaticity\",\"https://www.wikidata.org/wiki/Q210390\",\"https://www.google.com/search?kgmid=/m/01jsm7\"]}],\"name\":\"SMARTS query writing guide: Avoid bad matches\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/Bjn3od9MGEGskselARqL52bQV1zox1lM.webp\"},\"url\":\"https://mossflux.com/blog/smarts-query-writing-guide\"},{\"@id\":\"https://mossflux.com/blog/smarts-query-writing-guide#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q7544680\"},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"4. SMARTS - A Language for Describing Molecular Patterns - Daylight\",\"url\":\"https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"Cheminformatics basics: SMARTS\",\"url\":\"https://pgg1610.github.io/data_blog/posts/2022-01-10-cheminfo_basics_smarts.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"The RDKit Book\",\"url\":\"https://www.rdkit.org/docs/RDKit_Book.html\"},{\"@type\":\"CreativeWork\",\"name\":\"Security · mossflux docs\",\"url\":\"https://mossflux.com/docs/security\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/#demo\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/#get\"}],\"dateModified\":\"2026-11-09\",\"datePublished\":\"2026-11-09\",\"description\":\"Get a SMARTS query writing guide: describe each atom with explicit primitives, combine them with logical operators, add recursive $(...), and test hits by hand.\",\"headline\":\"SMARTS query writing guide: Avoid bad matches\",\"image\":\"https://api.birchseek.com/m/Bjn3od9MGEGskselARqL52bQV1zox1lM.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/smarts-query-writing-guide#webpage\"},\"keywords\":[\"smarts query writing guide\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/smarts-query-writing-guide#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\"},{\"@id\":\"https://www.wikidata.org/wiki/Q130850909\"},{\"@id\":\"https://www.wikidata.org/wiki/Q170409\"},{\"@id\":\"https://www.wikidata.org/wiki/Q283\"},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\"},{\"@id\":\"https://www.wikidata.org/wiki/Q210390\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2623},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"digital identifier to identify a certain piece of data, derived from the data by an algorithm\",\"name\":\"Fingerprint\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Fingerprint_(computing)\",\"https://www.wikidata.org/wiki/Q3745713\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"software package for sequence analysis\",\"name\":\"HMMER\",\"sameAs\":[\"https://en.wikipedia.org/wiki/HMMER\",\"https://www.wikidata.org/wiki/Q5631078\",\"https://www.google.com/search?kgmid=/m/04n5g57\"]},{\"@type\":\"Thing\",\"description\":\"hardware direct memory access over computer networks\",\"name\":\"Remote direct memory access\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Remote_direct_memory_access\",\"https://www.wikidata.org/wiki/Q2561570\",\"https://www.google.com/search?kgmid=/m/04599j\"]},{\"@type\":\"Thing\",\"description\":\"architectural style for distributed hypermedia systems\",\"name\":\"REST\",\"sameAs\":[\"https://en.wikipedia.org/wiki/REST\",\"https://www.wikidata.org/wiki/Q749568\",\"https://www.google.com/search?kgmid=/m/03nsxd\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"structural isomers that readily interconvert\",\"name\":\"Tautomer\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Tautomer\",\"https://www.wikidata.org/wiki/Q334640\"]},{\"@type\":\"Thing\",\"description\":\"all genetic material of an organism\",\"name\":\"Genome\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Genome\",\"https://www.wikidata.org/wiki/Q7020\",\"https://www.google.com/search?kgmid=/m/037_b\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - smarts
  - substructure search
  - cheminformatics
  - smiles
  - recursive smarts
  - query deadlines
title: "SMARTS query writing guide: Avoid bad matches"
---

**To write a SMARTS query, the core of any smarts query writing guide, describe each atom with primitives (`H`, `D`, `X`, `v`, ring, aromaticity) joined by `!`, `&`, `,` and `;`, and use recursive `$(...)` to constrain one atom by its surroundings. A SMILES is a valid SMARTS but a looser one, so test every query for hits that are too broad or too narrow. Matching is NP-complete, so untrusted queries need a deadline: in [mossflux's CLI](https://mossflux.com/docs/cli), past it you get an error, never a partial list.**

A `C(=O)O` query run over a SMILES library to find carboxylic acids also returns esters, and a tightened version misses carboxylates. Cheminformatics and platform engineers who run SMARTS over a SMILES file, or over a database behind an API, hit this early. That's why this SMARTS query writing guide checks every query by hand.

Every example here uses one test molecule: aspirin, `CC(=O)Oc1ccccc1C(=O)O`, which holds both an ester and an acid group. By the end you can write a query that means what you intend, check it by hand, and know how it runs and fails in mossflux (`mossflux substruct`, `POST /v1/substructure`).

> **Key Takeaways**
> - In the [Daylight SMARTS manual](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html), `&` binds tighter than `,` and `;` binds loosest, so `[c,n&H1]` and `[c,n;H1]` match different atoms.
> - The server deadline defaults to 10 s and caps at 60 s.

## SMARTS query writing guide: how is SMARTS different from SMILES?

Write a SMARTS query with explicit atom primitives, because the same characters mean different things in SMILES and SMARTS: SMILES describes one whole molecule and SMARTS describes a pattern. The tutorial [Cheminformatics basics: SMARTS](https://pgg1610.github.io/data_blog/posts/2022-01-10-cheminfo_basics_smarts.html) puts it this way: "SMILES represent whole molecule (graph), SMARTS identify a substructure (subgraph)."

In SMILES, `O` is an organic-subset atom whose implicit hydrogens fill its normal valence, so it's water. In SMARTS, `O` means any aliphatic oxygen with any hydrogen count. The SMARTS for water is `[OH2]`.

`CC(=O)O` is acetic acid as SMILES. As SMARTS it matches any aliphatic carbon bonded to a `C(=O)` carbon bonded to any aliphatic oxygen, so esters and carboxylates match too. On aspirin, that one query returns hits for both the ester and the acid.

The [Daylight SMARTS primitive definitions](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) set `H` as total hydrogens, `h` as implicit hydrogens, `D` as explicit connections, `X` as total connections including implicit hydrogens, and `v` as valence. Applied to aspirin's three oxygens, they give these counts:

| Primitive | Carbonyl O | Ester O | Acid OH oxygen |
| --- | --- | --- | --- |
| `H` | 0 | 0 | 1 |
| `h` | 0 | 0 | 1 |
| `D` | 1 | 2 | 1 |
| `X` | 1 | 2 | 2 |
| `v` | 2 | 2 | |

The acid oxygen's `v` cell is left blank here, so check it against the Daylight definition before you rely on it.

Only the acid oxygen has both `X2` and `H1`, so `[OX2H1]` selects it. The ester oxygen has `X2` but no hydrogen, and the carbonyl oxygen has `X1`.

The mossflux API takes that query as `POST /v1/substructure` with the fields `smarts`, `limit`, `offset`, `timeout_ms` and `db`:

```json
{
  "smarts": "[OX2H1]",
  "limit": 10,
  "offset": 0,
  "timeout_ms": 5000,
  "db": "<db-name>"
}
```

The API page documents the response shape. The [CLI page](https://mossflux.com/docs/cli) gives the `mossflux substruct` arguments for running the same SMARTS over a SMILES file.

Use SMILES to describe one whole compound, such as a library entry. Use SMARTS to match a family of compounds by atom and bond conditions.

## How does operator precedence work in a SMARTS query?

SMARTS evaluates logical operators in a fixed order, which the Daylight SMARTS manual, in its [Logical Operators](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) section, ranks from highest to lowest: `!` (not), `&` (high-precedence and), `,` (or), `;` (low-precedence and). Adjacent primitives are joined by an implied `&`, so `[nH]` means `[n&H]`.

1. Compare two orderings of the same primitives on 3-methylpyridine. Its SMILES is `Cc1cccnc1`: a methyl carbon, five aromatic carbons (one bears the methyl and has no H) and one aromatic nitrogen with no H.

   ```text
   [c,n&H1]   c OR (n AND one H)
   [c,n;H1]   (c OR n) AND one H
   ```

 The first matches all 5 aromatic carbons and not the ring nitrogen, because `&` binds before `,`. The second matches the 4 aromatic CH carbons and excludes the methyl-bearing carbon and the nitrogen, because `;` binds last and applies `H1` to both alternatives.

2. Repeat on indole. In `c1ccc2[nH]ccc2c1` the `[nH]` atom matches both patterns, since it's an aromatic nitrogen with one H. The two ring-fusion carbons have no H, so they match only `[c,n&H1]`. Count them by hand before you run anything.

3. Check how `!` binds: it applies to the single primitive after it. In `[C,N;!R]` the `;` makes the result (aliphatic C or aliphatic N) AND not in a ring. Write `[C,N&!R]` and the reading changes to C, or (N and not in a ring), so ring carbons match too.

4. Write the operators out. SMARTS atom expressions have no parentheses for grouping, so `&` and `;` are the only grouping tools. Put `;` where you mean "and, applied to everything on the left", and write `&` explicitly where you mean it.

5. Run both patterns. The `substruct` subcommand searches a SMILES file, and the [mossflux CLI page](https://mossflux.com/docs/cli) gives the arguments. Over REST, put the same string in the `smarts` field and send it to `POST /v1/substructure`:

   ```json
   {"smarts": "[c,n;H1]", "limit": 10, "db": "mydb"}
   ```

 If a pattern returns hits that your hand derivation excludes, the expression was grouped differently from how you read it.

## How do you write recursive SMARTS without unbalanced brackets?

Write recursive SMARTS one primitive at a time, and count the brackets on each line before you add the next. A recursive expression, written `$(...)`, is an atomic primitive that constrains the single atom it sits on. That atom is tested as the first atom of the inner pattern, and every other atom inside the parentheses only supplies context.

1. Start with the atom. Write `[NX3]`, a nitrogen with three connections. Count: 1 `[`, 1 `]`, no `$(`. Balanced.

2. Add the hydrogen condition. Write `[NX3;H2,H1]`, so the nitrogen is primary or secondary. The `,` binds tighter than `;`, so this reads as three connections AND (two hydrogens OR one hydrogen).

 Count: 1 `[`, 1 `]`, no `$(`. It's balanced.

3. Add the recursive exclusion. Write `[NX3;H2,H1;!$(NC=O)]`, which removes amide nitrogens. Count: 1 `[` and 1 `]`. The `$(` after `!` opens one parenthesis and the `)` after `O` closes it. Balanced.

 Closing the bracket without closing the parenthesis, as in `[NX3;!$(NC=O]`, leaves one `$(` open, and that's invalid.

4. Read the result as one atom. The `$(NC=O)` primitive asks only whether this nitrogen starts an `N-C=O` path. The carbon and oxygen inside it aren't part of the reported match, and they're never returned.

 Written at top level, `NC=O` matches and returns three atoms. Inside `$(...)`, the query still returns one atom, the nitrogen.

5. Reuse the primitive as a neighbour. Write `N[$(C=O)]`, where the second atom is a carbon that must itself start a `C=O` path. Count: 1 `[`, 1 `]`, 1 `$(`, 1 `)`. Balanced.

 Each hit returns two atoms, the nitrogen and the carbon. The oxygen's checked but isn't returned.

6. Send it to mossflux. Put the SMARTS in the `smarts` field of a `POST /v1/substructure` request and set a deadline, since subgraph matching is NP-complete:

```json
{
  "smarts": "[NX3;H2,H1;!$(NC=O)]",
  "limit": 100,
  "offset": 0,
  "timeout_ms": 5000
}
```

The hits should be amines, and none of them should be an amide nitrogen. If amides still appear, check that `!` sits directly before `$(` and that `;` separates the three conditions.

Nested `$(` levels count toward the recursion depth limit. The [mossflux API page](https://mossflux.com/docs/api) documents the endpoint fields.

## SMARTS query writing guide: why does a query match too much or miss hits?

A SMARTS query matches too much when one primitive is looser than you meant, and misses hits when one is tighter. Removing constraints one at a time and comparing hit counts shows which primitive is at fault.

`C(=O)O` is too general. Used as a query, this SMILES-style fragment matches esters as well as acids, because the bare `O` carries no hydrogen or degree primitive. The [Daylight SMARTS manual](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) lists `C` as an aliphatic carbon atom, and a bare `N` has the same looseness as a bare `O`.

`C(=O)[OH]` is too specific: it misses carboxylates `C(=O)[O-]`, because that oxygen carries a charge and no hydrogen. `C(=O)[O;H1,-1]` catches both. The Kekulé form `C1=CC=CC=C1` also fails on an aromatic ring, because its `C` atoms are aliphatic and mossflux decides aromaticity with [its own Hückel model](https://mossflux.com/docs/design).

1. Start from the smallest query that must match. Run it with `mossflux substruct` and note the count. Use the count, not the printed list: the [CLI docs](https://mossflux.com/docs/cli) state that counts are always exact, and `--limit` caps only the printed hits.
2. Add one primitive. Change `O` to `[OX2]`, or `[OH]` to `[O;H1,-1]`. Change one thing per run, or you can't tell which change moved the count.
3. Compare counts. A rise after a change means you widened the query, and a fall means you narrowed it. A fall that removes molecules you expected to keep marks the constraint at fault.
4. Inspect one molecule. Fetch a surprising hit with `GET /v1/databases/{db}/molecules/{id}`. For a missed molecule, send its SMILES to [`POST /v1/molecule`](https://mossflux.com/docs/api) and read the formula, weight and counts.

mossflux does no salt stripping and no tautomer handling, so canonicalise before ingestion. A neutral acid stored as its carboxylate misses `[OH]` for that reason.

### Worked example

Take aspirin `CC(=O)Oc1ccccc1C(=O)O` and its carboxylate `CC(=O)Oc1ccccc1C(=O)[O-]`. Run `C(=O)O` and it matches the acetyl ester and the acid carbon, because neither bare `O` says anything about hydrogens. Run `C(=O)[OH]` instead and the acetyl ester drops out, since its single-bonded oxygen has no hydrogen.

The carboxylate drops out too: Daylight defines `[OH]` as an oxygen with one hydrogen, and `[O-]` has none. Widen to `[O;H1,-1]` and both forms match while the ester oxygen stays excluded.

## Which SMARTS constructs does mossflux support, ignore or reject?

mossflux supports a defined subset of SMARTS, ignores chirality and atom maps, and bounds every query by size, depth and hit count. Other engines draw the line elsewhere: the RDKit Book has a section titled [SMARTS Support and Extensions](https://www.rdkit.org/docs/RDKit_Book.html) for the ways RDKit departs from Daylight, so a query that works in one engine needs its own check in another.

| Construct | Status | Source |
|---|---|---|
| A SMILES used as a SMARTS | Supported | [API docs](https://mossflux.com/docs/api) |
| Recursive `$(...)`, depth 10 | Supported | [API docs](https://mossflux.com/docs/api) |
| `v<n>`, `X<n>`, `D`, `H` | Supported | [API docs](https://mossflux.com/docs/api) |
| `h`, `R`, `r`, `x`, component-level grouping | Not listed in the docs; no support claimed | [API docs](https://mossflux.com/docs/api) |
| Stereo-aware matching, salt stripping, tautomer handling | Not done | [API docs](https://mossflux.com/docs/api) |
| Reactions | Not searched | [API docs](https://mossflux.com/docs/api) |
| More than 10,000 hits per request | Bounded | [API docs](https://mossflux.com/docs/api) |

Before you build a query on `h`, `R`, `r`, `x` or component-level grouping, check the mossflux API docs for them.

Malformed or oversized input is an error with a byte position or line number. `limit` and `offset` page through hits without changing the answer set.

A result matches a brute-force scan, with `brute.rs` as the oracle. In the RDKit differential, mossflux agrees on 114 of 114 queries. The [correctness page](https://mossflux.com/docs/correctness) gives the conditions of that run, which shows agreement on that query set rather than a proof for every possible SMARTS.

## How does mossflux run a substructure search, and what is the deadline?

mossflux runs every SMARTS through five stages under a deadline: the [mossflux server](https://mossflux.com/docs/api) defaults to a 10 s deadline and caps it at 60 s. A search that passes its deadline returns an error, never a partial list. Subgraph matching is NP-complete, so a short query can take exponential time.

The path screen is a superset filter that never drops a match. It rejects a molecule only on proof, so it changes cost and never the answer.

The census has 32 count classes, and exact classes answer without the matcher. A query the census can settle never reaches VF2, and every other query does, so query shape decides which stages do the work.

Measured speed varies widely for that reason. The [benchmarks page](https://mossflux.com/docs/benchmarks) reports substructure search at 5.2 to 160 million molecules a second per thread. The conditions: a 17-query battery, 2M ChEMBL rows, 8 threads, an EPYC 9645 VM, median of 3.

Those timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code, although the exactness gates pass on it. No billion-scale substructure claim is made.

Set the deadline per request with `timeout_ms`, or in the CLI with `--timeout`. If you accept patterns from users, cap each request below the default. This request gives a sulfonamide search 2,000 ms:

```sh
curl -s http://localhost:8080/v1/substructure \
  -H 'Content-Type: application/json' \
  -d '{"smarts": "S(=O)(=O)N", "limit": 100, "timeout_ms": 2000}'
```

Past 2,000 ms the answer is an error without the hits found so far, because a partial list would not match a brute-force scan.

The server also runs 4 concurrent searches by default, and further searches wait. A slow query delays the searches queued behind it, so short deadlines keep the queue moving.

To see the screen and census at work on real data, [try the demo](https://mossflux.com/#demo), which searches ChEMBL 37.

## Frequently asked questions

### How do I write a SMARTS query?

Describe each atom with SMARTS primitives, combine them with `!`, `&`, `,` and `;`, and join atoms with bond symbols. A SMILES is a valid SMARTS, but `O` then means any aliphatic oxygen, not water. Test the query against known hits and non-hits before running it on a library.

### What is the difference between H and h in SMARTS?

`H` is total hydrogens and `h` is implicit hydrogens, as the Daylight manual defines them. On an atom with explicit hydrogens the two give different counts. In mossflux queries, use `H` for hydrogen counts.

### Does mossflux handle stereochemistry, salts or tautomers in a SMARTS search?

mossflux parses chirality and atom maps and ignores both, so it does no stereo-aware matching. It does no salt stripping or tautomer handling either, so canonicalise your SMILES library before you load it.

### What happens if my SMARTS search takes too long?

A search that passes its deadline returns an error, never a partial list. The server default is 10 s and the maximum is 60 s, and you set it with `timeout_ms` on `POST /v1/substructure`. Subgraph matching is NP-complete, so queries from untrusted users need one.

### How many hits can one substructure request return?

One request returns up to 10,000 hits. To read a larger result set, page through it with `limit` and `offset`, raising `offset` by `limit` on each call.

## Next steps after this SMARTS query writing guide

**Today:** Rewrite one SMILES-style query, such as `C(=O)O`, with explicit primitives. Then remove constraints one at a time and compare hit counts to find the primitive that matches too much or too little.

**This week:** Set `timeout_ms` on every query that a user controls. Then install the [mossflux CLI](https://mossflux.com/docs/cli) (Rust 1.95 or later) and run your queries against your own SMILES file:

```sh
RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
```

To skip the build, use the Docker image `loruorg/mossflux` (x86-64), described in the [deploy docs](https://mossflux.com/docs/deploy).

mossflux is free and open source. [Run it on your library](https://mossflux.com/#get).

## Sources

[CLI · mossflux docs](https://mossflux.com/docs/cli) · [4. SMARTS - A Language for Describing Molecular Patterns…](https://www.daylight.com/dayhtml/doc/theory/theory.smarts.html) · [API · mossflux docs](https://mossflux.com/docs/api) · [Cheminformatics basics: SMARTS](https://pgg1610.github.io/data_blog/posts/2022-01-10-cheminfo_basics_smarts.html) · [Design · mossflux docs](https://mossflux.com/docs/design) · [The RDKit Book](https://www.rdkit.org/docs/RDKit_Book.html) · [Security · mossflux docs](https://mossflux.com/docs/security) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks). Every other source is linked where the article cites it.
