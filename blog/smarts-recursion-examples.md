---
description: What does $() do in SMARTS? It tests one atom and hits when that atom starts a match of the inner pattern. Learn SMARTS recursion examples with worked SMILES.
faq:
  - answer: A recursive SMARTS atom, written as $( ) around a SMARTS pattern, is an atom primitive that is true when the atom it is tested on is the first atom of a match of the inner pattern. For example, [$([CX3]=[OX1])] is true for a carbonyl carbon. It never matches a whole molecule.
    question: What does `$()` mean in SMARTS?
  - answer: They differ because inside brackets & binds before , and , binds before ;, so [N,O&H1] means N, or O with one hydrogen, while [N,O;H1] means N or O, then one hydrogen. On CCN the first hits the nitrogen and the second misses, because that nitrogen carries two hydrogens.
    question: Why do `[N,O&H1]` and `[N,O;H1]` differ?
  - answer: mossflux supports recursive SMARTS up to a depth of 10. A $() nested inside another $() uses two of those levels, so ordinary nested queries sit well inside the limit. The $(...) syntax tests whether an atom is the first atom of a match of the inner pattern.
    question: How deep can recursive SMARTS go in mossflux?
  - answer: "mossflux does not use stereochemistry in SMARTS: it parses chirality and ignores it, so a chiral mark in the query doesn't narrow the hits. Salts and tautomers aren't normalised either, so canonicalise your SMILES file before you search it with mossflux."
    question: Does mossflux use stereochemistry in SMARTS?
image: /blog/smarts-recursion-examples/featured-ac0eaa7d4dfd.webp
image_alt: "Illustration for SMARTS recursion examples: Hits and misses"
publishDate: "2026-11-19"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/smarts-recursion-examples#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q170409\",\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]}],\"datePublished\":\"2026-11-19\",\"description\":\"What does $() do in SMARTS? It tests one atom and hits when that atom starts a match of the inner pattern. Learn SMARTS recursion examples with worked SMILES.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A recursive SMARTS atom, written as $( ) around a SMARTS pattern, is an atom primitive that is true when the atom it is tested on is the first atom of a match of the inner pattern. For example, [$([CX3]=[OX1])] is true for a carbonyl carbon. It never matches a whole molecule.\"},\"name\":\"What does `$()` mean in SMARTS?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"They differ because inside brackets \\u0026 binds before , and , binds before ;, so [N,O\\u0026H1] means N, or O with one hydrogen, while [N,O;H1] means N or O, then one hydrogen. On CCN the first hits the nitrogen and the second misses, because that nitrogen carries two hydrogens.\"},\"name\":\"Why do `[N,O\\u0026H1]` and `[N,O;H1]` differ?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux supports recursive SMARTS up to a depth of 10. A $() nested inside another $() uses two of those levels, so ordinary nested queries sit well inside the limit. The $(...) syntax tests whether an atom is the first atom of a match of the inner pattern.\"},\"name\":\"How deep can recursive SMARTS go in mossflux?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux does not use stereochemistry in SMARTS: it parses chirality and ignores it, so a chiral mark in the query doesn't narrow the hits. Salts and tautomers aren't normalised either, so canonicalise your SMILES file before you search it with mossflux.\"},\"name\":\"Does mossflux use stereochemistry in SMARTS?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q188777\",\"@type\":\"Thing\",\"description\":\"any derivative of an oxoacid in which a (possibly substituted) amino group replaces an acidic hydroxy group\",\"name\":\"Amide\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Amide\",\"https://www.wikidata.org/wiki/Q188777\",\"https://www.google.com/search?kgmid=/m/0psh\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q101487\",\"@type\":\"Thing\",\"description\":\"Compound derived from an acid\",\"name\":\"Ester\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Ester\",\"https://www.wikidata.org/wiki/Q101487\",\"https://www.google.com/search?kgmid=/m/02mwl\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q167198\",\"@type\":\"Thing\",\"description\":\"organic compound that is a derivative of ammonia\",\"name\":\"Amine\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Amine\",\"https://www.wikidata.org/wiki/Q167198\",\"https://www.google.com/search?kgmid=/m/0pq0\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q134856\",\"@type\":\"Thing\",\"description\":\"oxoacid having the structure RC(=O)OH, used as a suffix in systematic name formation to denote the –C(=O)OH group including its carbon atom\",\"name\":\"Carboxylic acid\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Carboxylic_acid\",\"https://www.wikidata.org/wiki/Q134856\",\"https://www.google.com/search?kgmid=/m/01ts1\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\",\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]}],\"name\":\"SMARTS recursion examples: Hits and misses\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/Shs0H808icDjZDJAA_u_UlDYyOSzLsw8.webp\"},\"url\":\"https://mossflux.com/blog/smarts-recursion-examples\"},{\"@id\":\"https://mossflux.com/blog/smarts-recursion-examples#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"},{\"@id\":\"https://www.wikidata.org/wiki/Q170409\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"SMARTS Tutorial - Daylight\",\"url\":\"https://www.daylight.com/dayhtml_tutorials/languages/smarts/index.html\"},{\"@type\":\"CreativeWork\",\"name\":\"SMARTS Tutorial - Daylight\",\"url\":\"https://en.wikipedia.org/wiki/Smiles_arbitrary_target_specification\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"Security · mossflux docs\",\"url\":\"https://mossflux.com/docs/security\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux · exact similarity search at billions of molecules per second\",\"url\":\"https://mossflux.com/#demo\"},{\"@type\":\"CreativeWork\",\"name\":\"mossflux correctness docs\",\"url\":\"https://mossflux.com/docs/correctness\"}],\"dateModified\":\"2026-11-19\",\"datePublished\":\"2026-11-19\",\"description\":\"What does $() do in SMARTS? It tests one atom and hits when that atom starts a match of the inner pattern. Learn SMARTS recursion examples with worked SMILES.\",\"headline\":\"SMARTS recursion examples: Hits and misses\",\"image\":\"https://api.birchseek.com/m/Shs0H808icDjZDJAA_u_UlDYyOSzLsw8.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/smarts-recursion-examples#webpage\"},\"keywords\":[\"smarts recursion examples\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/smarts-recursion-examples#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q188777\"},{\"@id\":\"https://www.wikidata.org/wiki/Q101487\"},{\"@id\":\"https://www.wikidata.org/wiki/Q167198\"},{\"@id\":\"https://www.wikidata.org/wiki/Q134856\"},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2342},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"number of nonzero symbols in a string\",\"name\":\"Hamming weight\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Hamming_weight\",\"https://www.wikidata.org/wiki/Q5645805\",\"https://www.google.com/search?kgmid=/m/048nw2\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"process in bioinformatics that aligns (identifies equivalent sites within) molecular sequences\",\"name\":\"Sequence alignment\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Sequence_alignment\",\"https://www.wikidata.org/wiki/Q827246\",\"https://www.google.com/search?kgmid=/m/01353w\"]},{\"@type\":\"Thing\",\"description\":\"server processor line by Advanced Micro Devices\",\"name\":\"Epyc\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Epyc\",\"https://www.wikidata.org/wiki/Q33083031\",\"https://www.google.com/search?kgmid=/g/11dy_kks3x\"]},{\"@type\":\"Thing\",\"description\":\"instruction set extension developed by Intel\",\"name\":\"AVX-512\",\"sameAs\":[\"https://en.wikipedia.org/wiki/AVX-512\",\"https://www.wikidata.org/wiki/Q16987723\",\"https://www.google.com/search?kgmid=/m/0_v0j7q\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - smarts
  - recursive smarts
  - substructure search
  - cheminformatics
  - mossflux
title: "SMARTS recursion examples: Hits and misses"
---

**SMARTS recursion examples all follow one rule: `$(...)` is an atom primitive, tested against a single atom, and it's true when that atom is the first atom of a match of the inner pattern. `[$([CX3]=[OX1])]` matches a three-connected carbon double-bonded to a terminal oxygen. `[NX3;H2,H1;!$(NC=O)]` matches a primary or secondary amine nitrogen that isn't an amide nitrogen.**

A SMARTS query run over ChEMBL or an in-house collection can return zero hits, or hits on the wrong atom, when `$(...)` is read as a test on the whole molecule. Every pattern in these SMARTS recursion examples comes with target SMILES and the expected hit or miss.

With these SMARTS recursion examples, you'll be able to write recursive patterns, predict hit or miss for a given SMILES, nest them, avoid the mistakes that break them, and run the same query through the mossflux CLI, REST API and MCP tool.

> **Key Takeaways**
> - In brackets, `&` binds before `,` and `,` before `;`, so `[N,O&H1]` and `[N,O;H1]` differ on `CCN`.
> - `X` counts total connections and `D` explicit ones, so the nitrogen in `CCN` is X3 but D1.
> - mossflux supports recursive `$(...)` to depth 10; a search past its deadline returns an error, never a partial list.

## How do SMARTS recursion examples pick the atom that hits?

Recursive SMARTS works by testing atoms one at a time: a toolkit keeps an atom when it's the first atom of a match of the pattern inside `$()`. Inside a bracket atom, the primitive combines with others through `&`, `,`, `;` and `!`.

A recursive atom doesn't match a whole molecule. In all SMARTS recursion examples, the result is a set of atom positions.

The first atom written inside `$()` is the one that hits, and the rest of the inner pattern is context that must exist around it. In `[$([CX3]=[OX1])]` the hit is the `[CX3]`, and `=[OX1]` only has to sit next to it. Reverse the order to `[$([OX1]=[CX3])]` and the oxygen becomes the hit.

Ordinary primitives sit beside the recursive one in the same bracket. `[NX3;H2,H1;!$(NC=O)]` keeps three-connected nitrogens with one or two hydrogens and rejects any nitrogen that is the first atom of an `NC=O` match.

### Worked case: aspirin

Run `[$([CX3]=[OX1])]` against aspirin, `CC(=O)Oc1ccccc1C(=O)O`, and the toolkit tests each heavy atom as the first atom of `[CX3]=[OX1]`:

- The acetyl carbonyl carbon has three connections and a double bond to a one-connection oxygen, so it hits.
- The carboxylic acid carbon passes the same test, so it hits too.
- Both carbonyl oxygens, the ester oxygen and the acid hydroxyl fail, because the first atom must satisfy `[CX3]`.
- The methyl carbon has four connections (implicit hydrogens count in `X`), and the ring carbons are aromatic, so `C` rejects them.

The answer is two atoms: the two carbonyl carbons.

## SMARTS recursion examples: which atoms hit and which miss?

In each of these SMARTS recursion examples, the pattern hits only the atoms that start a match of its inner SMARTS. The outcomes are worked out from the pattern semantics; none of them is measured output.

**1. Positive environment: `[$([CX3]=[OX1])]`**

- `CC(C)=O` (acetone): hits the carbonyl carbon. It has three connections and a double bond to an oxygen with one connection.
- `CCO` (ethanol): misses. No atom has a double bond to oxygen.

**2. Negation: `[NX3;H2,H1;!$(NC=O)]`**

- `CCN` (ethylamine): hits the nitrogen, which has three total connections and H2 and isn't an amide nitrogen.
- `CCNCC` (diethylamine): hits the nitrogen (X3, H1).
- `CC(=O)N` (acetamide): misses, because the nitrogen passes `NX3` and `H2` but `!$(NC=O)` excludes it.
- `CCN(CC)CC` (triethylamine): misses, because its nitrogen has H0 and fails `H2,H1`.

**3. Either-position OR: `[$(c1c([OH])cccc1),$(c1ccc([OH])cc1)]`**

The first environment matches a ring carbon next to a hydroxyl-bearing carbon (ortho). The second matches a ring carbon opposite one (para).

- `Oc1ccccc1` (phenol): 3 hits, the two ortho carbons and the para carbon.
- Same molecule: the hydroxyl-bearing carbon and the two meta carbons miss, because neither environment places an `[OH]` at their ring positions.
- `COc1ccccc1` (anisole): 0 hits. The oxygen carries no hydrogen, so `[OH]` fails.

**4. AND of two environments: `[$(*O);$(*CC)]`**

The `;` requires the same atom to touch an oxygen and to start a `*CC` path.

- `CCCO` (propan-1-ol): hits the CH2 bonded to oxygen. It touches O and starts the path C, C, C.
- `CCO` (ethanol): misses. The CH2 touches O but has no path of two further carbons, and the methyl carbon doesn't touch O.

**5. Two written forms of one group: `[$([CX3]=[OX1]),$([CX3+]-[OX1-])]`**

This is an OR of two recursive environments. The first matches the carbonyl carbon in `CC(C)=O`. The second matches the same carbon when a structure is written in its charge-separated form, `C[C+](C)[O-]`. Either environment makes the atom a hit.

## How do you nest SMARTS recursion examples, and which operator binds first?

A recursive SMARTS nests by placing one `$(...)` inside another, and the logical operators inside a bracket atom bind in a fixed order that changes what a pattern means.

`[$([CX3](=O)[$([OX2H1]),$([OX2][CH3])])]` uses two levels of recursion. It marks a carbonyl carbon whose single-bonded neighbour is a hydroxyl oxygen or a methoxy oxygen. Build it in five steps:

1. Open the outer layer: `[$(` starts the bracket atom and the first recursive primitive (depth 1).
2. Write the carbon and its double bond: `[CX3](=O)`, whose parentheses open and close on the same branch.
3. Open the inner bracket: `[$(` starts the second recursive primitive (depth 2), here `[OX2H1]`, then `)` closes it.

4. Add the OR: `,` joins it to a second `$(` holding `[OX2][CH3]`, then `)` closes that.
5. Close in reverse order: `]` ends the inner bracket, `)` ends depth 1, and the final `]` ends the outer atom, giving 4 opening and 4 closing parentheses.

Only the carbon is marked as a hit. Acetic acid `CC(=O)O` hits through the OH. Methyl acetate `CC(=O)OC` hits through `[CH3]`. Acetamide `CC(=O)N` misses.

Ethyl acetate `CC(=O)OCC` misses, because the oxygen's other neighbour is a CH2, so `[CH3]` fails. Aspirin `CC(=O)Oc1ccccc1C(=O)O` hits once: the acid carbon passes, and the acetyl carbon fails because its oxygen bonds to an aromatic carbon.

The order `!`, then `&` (also implicit), then `,`, then `;` follows the Logical Operators section of the [Daylight SMARTS tutorial](https://www.daylight.com/dayhtml_tutorials/languages/smarts/index.html). Two patterns that differ only in the operator before `H1` show the effect:

| SMILES | `[N,O&H1]` (N, or O with one H) | `[N,O;H1]` ((N or O) with one H) |
|---|---|---|
| `CCN` | hit | miss |
| `CCO` | hit | hit |
| `CCNC` | hit | hit |

On `CCN` the nitrogen carries two hydrogens, so only the first pattern accepts it. `!` applies only to the primitive after it: `[!C,N]` means (not carbon) or nitrogen.

## SMARTS recursion examples or the dot operator: what's the difference?

Recursive SMARTS describes what surrounds one atom, while the dot and the grouping parentheses describe which components several matched atoms sit in.

| Target | `[#8].[#8]` | `([#8].[#8])` | `([#8]).([#8])` |
| --- | --- | --- | --- |
| `OCCO` (one component) | hit | hit | miss |
| `O.O` (two components) | hit | miss | hit |
| Oxygens must sit in | any components | the same component | different components |

A recursive `$(...)` lives inside an atom's brackets and is tested against one atom, as in `[$([CX3]=[OX1])]`. It never spans components and needs no dot.

The bare dot in `[#8].[#8]` joins two patterns with no constraint on which component each match falls in. It hits `OCCO` because the molecule holds two oxygens, and it hits `O.O` for the same reason.

Parentheses add the constraint. In `([#8].[#8])` both oxygens must sit in the same component, so `OCCO` hits and `O.O` misses. In `([#8]).([#8])` each oxygen sits in its own group, so the two must be in different components: `O.O` hits and `OCCO`, a single component, misses.

These SMARTS recursion examples follow Daylight semantics; they aren't mossflux output. Check the supported SMARTS in the [mossflux API docs](https://mossflux.com/docs/api) before you rely on the dot or the parenthesised forms.

Choose recursion to describe the neighbourhood of one atom, such as a carbonyl carbon or an ortho carbon. The bare `[#8].[#8]` fits when two features must appear anywhere in the input, in one component or in several. The grouped forms `([#8].[#8])` and `([#8]).([#8])` require the features in the same component or in different ones, for example one molecule against a mixture of two.

## What mistakes break the SMARTS recursion examples you write?

Four mistakes make a recursive SMARTS query return nothing or hit the wrong atom: unbalanced parentheses, bare atoms inside `$()`, reading `X` as `D`, and writing the wrong atom first.

1. **Unbalanced parentheses.** `[$(C(=O)N]` leaves the `$(` open, so the query can't parse. The balanced form is `[$(C(=O)N)]`, so count every `(` against a `)` before you run it. mossflux returns an error with a byte position for malformed input, which points at the break.

2. **Bare atoms inside `$()`.** In `[$(NC=O)]`, `N` and `C` are unbracketed organic-subset atoms, so they're aliphatic and carry no hydrogen or connectivity constraint, and an aromatic nitrogen or carbon won't match. Bracket and constrain when it matters: `[$([NX3][CX3]=[OX1])]`. Use `[#7]` or `[#6]` when aromatic atoms should count too.

3. **`X` read as `D`.** `X` counts total connections including implicit hydrogens, and `D` counts explicit connections. In `CCN` the nitrogen is X3 and D1, so `[NX3]` hits it and `[ND3]` doesn't.

4. **The wrong atom hits.** The first atom inside `$()` is the one the primitive tests. Against `CC(=O)O`, `[$(C=O)]` returns the carbonyl carbon, while `[$(O=C)]` returns the carbonyl oxygen. Write the atom you want back first.

These corrected SMARTS recursion examples are plain SMARTS you can reuse. Run them over a SMILES file with the `substruct` subcommand described in the [mossflux CLI docs](https://mossflux.com/docs/cli).

## How do you run a recursive SMARTS query with mossflux?

You run a recursive SMARTS query with mossflux by sending one string to one search engine through the CLI, REST or MCP. The query here finds amine nitrogens with one or two hydrogens that aren't amide nitrogens: `[NX3;H2,H1;!$(NC=O)]`.

1. **Run it from the shell.** Quote the query with single quotes, because a shell reads an unquoted `$` as an expansion. The [CLI reference](https://mossflux.com/docs/cli) documents the `substruct` subcommand:

   ```sh
   mossflux substruct compounds.smi '[NX3;H2,H1;!$(NC=O)]' --limit 20
   ```

 `--limit` caps only the printed hits, and the counts stay exact. Add `--timeout` for queries you don't control.

2. **Post it to REST.** The [API reference](https://mossflux.com/docs/api) lists five fields for `POST /v1/substructure`:

   ```json
   {
     "smarts": "[NX3;H2,H1;!$(NC=O)]",
     "limit": 100,
     "offset": 0,
     "timeout_ms": 10000,
     "db": "chembl"
   }
   ```

 Past it you get an error, never a partial list.

3. **Call it over MCP.** Give the same SMARTS string to the `substructure_search` tool.

[Try the public demo](https://mossflux.com/#demo) first: `https://api.mossflux.com` serves REST and MCP over ChEMBL 37 (2.9 million molecules) on one small 4-thread server. If you post the body from step 2 to `https://api.mossflux.com/v1/substructure`, the engine returns the same rows a brute-force scan would, or an error if the deadline passes.

To run the same three steps on your own file, start the server as described in the [deploy guide](https://mossflux.com/docs/deploy).

## What are the limits of recursive SMARTS search, and what does exact mean?

Recursive SMARTS search in mossflux is exact within a fixed set of limits, and the [correctness docs](https://mossflux.com/docs/correctness) define exact as returning the same rows as a brute-force scan. A hit is never missed and a non-match is never reported.

Every recursive query runs within these limits:

- Recursive `$(...)` is supported to depth 10, according to the [design docs](https://mossflux.com/docs/design).

The design docs describe the pipeline as parse, perceive, path screen, census and VF2 verify. The path screen is a superset filter that never drops a match, so it removes candidates without changing the answer.

Subgraph isomorphism is NP-complete, so even a short query can be slow on some molecules. A search therefore runs under a deadline, set per request with `timeout_ms` on `POST /v1/substructure`, and past it the answer is an error, never a partial list.

Two checks back the exactness claim. The matcher is tested against a brute-force oracle (`brute.rs`), and 114 of 114 queries agree in the RDKit differential.

These behaviours affect what a query matches:

- Chirality in a SMARTS parses and is ignored, so it doesn't constrain matches.
- Atom maps parse and are ignored.

## Frequently asked questions

### What does `$()` mean in SMARTS?

A recursive SMARTS atom, written $(...), is an atom primitive that's true when the atom it's tested on is the first atom of a match of the inner pattern. The pattern [$([CX3]=[OX1])] is true for a carbonyl carbon. It never matches a whole molecule.

### Why do `[N,O&H1]` and `[N,O;H1]` differ?

Inside brackets the ampersand binds before the comma, and the comma binds before the semicolon. So [N,O&H1] means N, or O with one hydrogen, while [N,O;H1] means N or O, either way with one hydrogen. On CCN the first hits the nitrogen and the second misses, because that nitrogen carries two hydrogens.

### How deep can recursive SMARTS go in mossflux?

mossflux supports recursive $(...) to depth 10. A $() nested inside another $() uses two of those levels, so ordinary nested queries sit well inside the limit.

### Does mossflux use stereochemistry in SMARTS?

No. mossflux parses chirality in SMARTS and ignores it, so a chiral mark doesn't narrow the hits.

## What should you do next with SMARTS recursion examples?

Start with one of your existing patterns: confirm the first atom of each `$(...)` is the atom you want back, balance every parenthesis, and check whether each connectivity test should use `X` or `D`.

Next, run `[$([CX3]=[OX1])]` and `[NX3;H2,H1;!$(NC=O)]` over a SMILES file from your own library with `mossflux substruct`, then compare the hit count with a brute-force scan.

Before queries go into a pipeline, put `timeout_ms` on every query you don't control, and canonicalise salts and tautomers before ingestion.

## Sources

[SMARTS Tutorial - Daylight](https://www.daylight.com/dayhtml_tutorials/languages/smarts/index.html) · [SMARTS Tutorial - Daylight](https://en.wikipedia.org/wiki/Smiles_arbitrary_target_specification) · [Design · mossflux docs](https://mossflux.com/docs/design) · [Security · mossflux docs](https://mossflux.com/docs/security) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [API · mossflux docs](https://mossflux.com/docs/api) · [mossflux · exact similarity search at billions of molecules…](https://mossflux.com/#demo) · [mossflux correctness docs](https://mossflux.com/docs/correctness)
