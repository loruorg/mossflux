---
description: How do you write a chemfp FPS Rust reader? Stream lines, decode hex to bytes and stop on a bad line with its number. Learn the loop and search with mossflux.
faq:
  - answer: "Open the file with a buffered reader, read the # header lines, then split each record at the tab, hex-decode the fingerprint into bytes or words and keep the identifier. On a malformed line, return an error with its line number. mossflux loads plain or gzip FPS files."
    question: How do I read a chemfp FPS file in Rust?
  - answer: You should not import chemfp.fps_io directly, because chemfp documents it as an internal module that programs using the public API should not import. Use chemfp.open() instead when you need to read an FPS file. It returns an FPSReader for that file.
    question: Can I use chemfp.fps_io directly?
  - answer: A Tanimoto score of 1.0 means the two fingerprints have equal bits, so a result with score == 1.0 is not proof that the two records are the same compound. Canonicalisation of salts, tautomers and stereo is your job, and you need to do it before ingestion.
    question: What does a Tanimoto score of 1.0 mean in an FPS search?
  - answer: "mossflux results are the same as a brute-force scan: it returns exactly what a brute-force scan returns, with no false negatives and no false positives. A hit that a brute-force scan would return is never missed, and a hit it would not return is never added. The correctness docs describe the tests behind that guarantee."
    question: Are mossflux results the same as a brute-force scan?
image: /blog/chemfp-fps-rust-reader/featured-61e08b033386.webp
image_alt: "Illustration for Chemfp FPS rust reader: Stop on bad lines"
publishDate: "2026-11-24"
schema_json: "{\"@context\":\"https://schema.org\",\"@graph\":[{\"@id\":\"https://mossflux.com/blog/chemfp-fps-rust-reader#webpage\",\"@type\":[\"WebPage\",\"FAQPage\"],\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q59265935\",\"@type\":\"Thing\",\"name\":\"chemfp - fast and portable fingerprint formats and tools\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q59265935\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q28865\",\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]}],\"datePublished\":\"2026-11-24\",\"description\":\"How do you write a chemfp FPS Rust reader? Stream lines, decode hex to bytes and stop on a bad line with its number. Learn the loop and search with mossflux.\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/#website\"},\"mainEntity\":[{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"Open the file with a buffered reader, read the # header lines, then split each record at the tab, hex-decode the fingerprint into bytes or words and keep the identifier. On a malformed line, return an error with its line number. mossflux loads plain or gzip FPS files.\"},\"name\":\"How do I read a chemfp FPS file in Rust?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"You should not import chemfp.fps_io directly, because chemfp documents it as an internal module that programs using the public API should not import. Use chemfp.open() instead when you need to read an FPS file. It returns an FPSReader for that file.\"},\"name\":\"Can I use chemfp.fps_io directly?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"A Tanimoto score of 1.0 means the two fingerprints have equal bits, so a result with score == 1.0 is not proof that the two records are the same compound. Canonicalisation of salts, tautomers and stereo is your job, and you need to do it before ingestion.\"},\"name\":\"What does a Tanimoto score of 1.0 mean in an FPS search?\"},{\"@type\":\"Question\",\"acceptedAnswer\":{\"@type\":\"Answer\",\"text\":\"mossflux results are the same as a brute-force scan: it returns exactly what a brute-force scan returns, with no false negatives and no false positives. A hit that a brute-force scan would return is never missed, and a hit it would not return is never added. The correctness docs describe the tests behind that guarantee.\"},\"name\":\"Are mossflux results the same as a brute-force scan?\"}],\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q165194\",\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\",\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\",\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q100889364\",\"@type\":\"Person\",\"description\":\"researcher, ORCiD ID=0000-0001-8933-8830\",\"name\":\"Andrew Dalke\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q100889364\"]},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\",\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]}],\"name\":\"Chemfp FPS rust reader: Stop on bad lines\",\"primaryImageOfPage\":{\"@type\":\"ImageObject\",\"url\":\"https://api.birchseek.com/m/aEMOHihnz1Hra3NMG6qRNeTEXxAGRA9v.webp\"},\"url\":\"https://mossflux.com/blog/chemfp-fps-rust-reader\"},{\"@id\":\"https://mossflux.com/blog/chemfp-fps-rust-reader#article\",\"@type\":\"BlogPosting\",\"about\":[{\"@id\":\"https://www.wikidata.org/wiki/Q59265935\"},{\"@id\":\"https://www.wikidata.org/wiki/Q28865\"}],\"author\":{\"@id\":\"https://mossflux.com/#organization\"},\"citation\":[{\"@type\":\"CreativeWork\",\"name\":\"mossflux docs\",\"url\":\"https://mossflux.com/docs/\"},{\"@type\":\"CreativeWork\",\"name\":\"Design · mossflux docs\",\"url\":\"https://mossflux.com/docs/design\"},{\"@type\":\"CreativeWork\",\"name\":\"The chemfp project, Journal of Cheminformatics\",\"url\":\"https://doi.org/10.1186/s13321-019-0398-8\"},{\"@type\":\"CreativeWork\",\"name\":\"chemfp.fps_io module — chemfp 4.1 documentation\",\"url\":\"https://chemfp.readthedocs.io/en/latest/chemfp_fps_io.html\"},{\"@type\":\"CreativeWork\",\"name\":\"CLI · mossflux docs\",\"url\":\"https://mossflux.com/docs/cli\"},{\"@type\":\"CreativeWork\",\"name\":\"Correctness · mossflux docs\",\"url\":\"https://mossflux.com/docs/correctness\"},{\"@type\":\"CreativeWork\",\"name\":\"API · mossflux docs\",\"url\":\"https://mossflux.com/docs/api\"},{\"@type\":\"CreativeWork\",\"name\":\"Benchmarks · mossflux docs\",\"url\":\"https://mossflux.com/docs/benchmarks\"},{\"@type\":\"CreativeWork\",\"name\":\"Deploy · mossflux docs\",\"url\":\"https://mossflux.com/docs/deploy\"}],\"dateModified\":\"2026-11-24\",\"datePublished\":\"2026-11-24\",\"description\":\"How do you write a chemfp FPS Rust reader? Stream lines, decode hex to bytes and stop on a bad line with its number. Learn the loop and search with mossflux.\",\"headline\":\"Chemfp FPS rust reader: Stop on bad lines\",\"image\":\"https://api.birchseek.com/m/aEMOHihnz1Hra3NMG6qRNeTEXxAGRA9v.webp\",\"inLanguage\":\"en\",\"isPartOf\":{\"@id\":\"https://mossflux.com/blog/chemfp-fps-rust-reader#webpage\"},\"keywords\":[\"chemfp fps rust reader\"],\"mainEntityOfPage\":{\"@id\":\"https://mossflux.com/blog/chemfp-fps-rust-reader#webpage\"},\"mentions\":[{\"@id\":\"https://www.wikidata.org/wiki/Q165194\"},{\"@id\":\"https://www.wikidata.org/wiki/Q910164\"},{\"@id\":\"https://www.wikidata.org/wiki/Q54152347\"},{\"@id\":\"https://www.wikidata.org/wiki/Q100889364\"},{\"@id\":\"https://www.wikidata.org/wiki/Q466769\"}],\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"wordCount\":2612},{\"@id\":\"https://mossflux.com/#website\",\"@type\":\"WebSite\",\"inLanguage\":\"en\",\"name\":\"mossflux\",\"publisher\":{\"@id\":\"https://mossflux.com/#organization\"},\"url\":\"https://mossflux.com/\"},{\"@id\":\"https://mossflux.com/#organization\",\"@type\":\"Organization\",\"knowsAbout\":[{\"@type\":\"Thing\",\"description\":\"ASCII line notation for the structure of chemical species\",\"name\":\"Simplified Molecular Input Line Entry System\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Simplified_Molecular_Input_Line_Entry_System\",\"https://www.wikidata.org/wiki/Q466769\",\"https://www.google.com/search?kgmid=/m/0729b\"]},{\"@type\":\"Thing\",\"description\":\"cheminformatics software\",\"name\":\"RDKit\",\"sameAs\":[\"https://en.wikipedia.org/wiki/RDKit\",\"https://www.wikidata.org/wiki/Q54152347\"]},{\"@type\":\"Thing\",\"description\":\"method of finding chemicals in a database\",\"name\":\"Substructure search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Substructure_search\",\"https://www.wikidata.org/wiki/Q130850909\"]},{\"@type\":\"Thing\",\"description\":\"general-purpose programming language\",\"name\":\"Python\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Python_(programming_language)\",\"https://www.wikidata.org/wiki/Q28865\",\"https://www.google.com/search?kgmid=/m/05z1_\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science\",\"name\":\"Cheminformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Cheminformatics\",\"https://www.wikidata.org/wiki/Q910164\",\"https://www.google.com/search?kgmid=/m/02rnwp\"]},{\"@type\":\"Thing\",\"description\":\"software interface between computers and/or programs\",\"name\":\"API\",\"sameAs\":[\"https://en.wikipedia.org/wiki/API\",\"https://www.wikidata.org/wiki/Q165194\",\"https://www.google.com/search?kgmid=/m/0z5n\"]},{\"@type\":\"Thing\",\"description\":\"notation used for specifying substructural patterns in chemical molecules\",\"name\":\"SMILES arbitrary target specification\",\"sameAs\":[\"https://en.wikipedia.org/wiki/SMILES_arbitrary_target_specification\",\"https://www.wikidata.org/wiki/Q7544680\",\"https://www.google.com/search?kgmid=/m/03gzpd6\"]},{\"@type\":\"Thing\",\"description\":\"(as a form of proximity search (metric space)) optimization problem of finding the point in a given set that is closest (or most similar) to a given point\",\"name\":\"Nearest neighbor search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Nearest_neighbor_search\",\"https://www.wikidata.org/wiki/Q608751\",\"https://www.google.com/search?kgmid=/m/025ytbk\"]},{\"@type\":\"Thing\",\"description\":\"chemical information database\",\"name\":\"PubChem\",\"sameAs\":[\"https://en.wikipedia.org/wiki/PubChem\",\"https://www.wikidata.org/wiki/Q278487\",\"https://www.google.com/search?kgmid=/m/087mnk\"]},{\"@type\":\"Thing\",\"description\":\"hosting service for software projects using Git\",\"name\":\"GitHub\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GitHub\",\"https://www.wikidata.org/wiki/Q364\",\"https://www.google.com/search?kgmid=/m/04g0kcw\"]},{\"@type\":\"Thing\",\"description\":\"data structure for database query optimization at the expense of write and storage to maintain the index structure\",\"name\":\"Database index\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Database_index\",\"https://www.wikidata.org/wiki/Q580427\",\"https://www.google.com/search?kgmid=/m/05q990\"]},{\"@type\":\"Thing\",\"description\":\"chemical database of bioactive molecules with drug-like properties\",\"name\":\"ChEMBL\",\"sameAs\":[\"https://en.wikipedia.org/wiki/ChEMBL\",\"https://www.wikidata.org/wiki/Q6120337\",\"https://www.google.com/search?kgmid=/m/0dljnsn\"]},{\"@type\":\"Thing\",\"description\":\"identifier for chemical substances\",\"name\":\"International Chemical Identifier\",\"sameAs\":[\"https://en.wikipedia.org/wiki/International_Chemical_Identifier\",\"https://www.wikidata.org/wiki/Q203250\",\"https://www.google.com/search?kgmid=/m/06cpm3\"]},{\"@type\":\"Thing\",\"description\":\"ultra fast and sensitive sequence search and clustering suite\",\"name\":\"MMSeqs2\",\"sameAs\":[\"https://www.wikidata.org/wiki/Q114840759\"]},{\"@type\":\"Thing\",\"description\":\"biomolecule or biomolecule complex largely consisting of chains of amino acid residues\",\"name\":\"Protein\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Protein\",\"https://www.wikidata.org/wiki/Q8054\",\"https://www.google.com/search?kgmid=/m/05wvs\"]},{\"@type\":\"Thing\",\"description\":\"interdisciplinary science that combines biology, computer science and statistics to help in the collection, analysis and understanding of biological data\",\"name\":\"Bioinformatics\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Bioinformatics\",\"https://www.wikidata.org/wiki/Q128570\",\"https://www.google.com/search?kgmid=/m/01c62\"]},{\"@type\":\"Thing\",\"description\":\"cross-platform binary RPC protocol\",\"name\":\"GRPC\",\"sameAs\":[\"https://en.wikipedia.org/wiki/GRPC\",\"https://www.wikidata.org/wiki/Q26356541\",\"https://www.google.com/search?kgmid=/g/11cp5mklv8\"]},{\"@type\":\"Thing\",\"description\":\"contextual queries\",\"name\":\"Semantic search\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Semantic_search\",\"https://www.wikidata.org/wiki/Q1891170\",\"https://www.google.com/search?kgmid=/m/02r4_2d\"]},{\"@type\":\"Thing\",\"description\":\"database designed to store vector embeddings and perform similarity search in high-dimensional spaces\",\"name\":\"Vector database\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Vector_database\",\"https://www.wikidata.org/wiki/Q118396004\",\"https://www.google.com/search?kgmid=/g/11kph5bck7\"]},{\"@type\":\"Thing\",\"description\":\"group of atoms in a molecule which imparts specific chemical properties\",\"name\":\"Functional group\",\"sameAs\":[\"https://en.wikipedia.org/wiki/Functional_group\",\"https://www.wikidata.org/wiki/Q170409\",\"https://www.google.com/search?kgmid=/m/02ydd\"]}],\"name\":\"mossflux\",\"url\":\"https://mossflux.com/\"}]}"
tags:
  - chemfp
  - fps format
  - rust
  - tanimoto search
  - fingerprints
  - mossflux
title: "Chemfp FPS rust reader: Stop on bad lines"
---

**A chemfp FPS Rust reader streams the file through a buffered line parser, skips the header lines, decodes each hex fingerprint to bytes and stops on a malformed line with its line number. mossflux loads FPS files this way, and the [mossflux docs](https://mossflux.com/docs/) state that `mossflux search` over them returns exactly what a brute-force scan returns.**

You have a chemfp FPS file from an upstream pipeline, and you need a chemfp FPS Rust reader that fails loudly instead of skipping bad lines.

A reader that drops a line quietly searches fewer molecules than you think, and every score it returns still looks normal.

> **Key Takeaways**
> - kNN ties go to lower popcount, then the earlier row, as the [mossflux design notes](https://mossflux.com/docs/design) set out.

## What does a chemfp FPS Rust reader have to parse?

A chemfp FPS file is plain text: header lines that describe the fingerprints, then one record per line, a hex-encoded fingerprint followed by a tab and an identifier. One line is one molecule, and that's the unit a chemfp FPS Rust reader works through.

Andrew Dalke describes chemfp and its tools in ["The chemfp project"](https://doi.org/10.1186/s13321-019-0398-8) (J. Cheminformatics 11, 76, 2019, doi 10.1186/s13321-019-0398-8), the paper to cite for the format and the toolkit.

This sample uses made-up 16-bit fingerprints so it's easy to see each field. Real files carry the widths and fingerprint types of the toolkit that wrote them.

```text
#FPS1
#num_bits=16
#type=Illustration-Fingerprint/1
#software=made-up/0
a1f0	mol-0001
00ff	mol-0002
8001	mol-0003
```

- `#FPS1` is the format line. Every line that starts with `#` is a header line, and a reader consumes these before the first record.
- `#num_bits=16` gives the fingerprint size. A parser checks each hex string against it: 16 bits is 4 hex digits, or 2 bytes.
- `#type=` and `#software=` record which fingerprint and which program produced the bits.
- `a1f0` is a fingerprint in hex. Each pair of hex digits is one byte, so `a1f0` decodes to the bytes `a1` and `f0`.
- A tab separates the fingerprint from the identifier (`mol-0001`). The identifiers in this sample are invented, so don't look them up.

chemfp's public entry point is `chemfp.open()`. The [`chemfp.fps_io` page](https://chemfp.readthedocs.io/en/latest/chemfp_fps_io.html) says that "The chemfp.open() function returns a FPSReader which reads from an FPS file."

The same page documents the FPSReader constructor with a `metadata` argument, next to `infile`, `close`, `location` and `block_reader`.

That page also labels the module as internal: "This is an internal chemfp module. It should not be imported by programs which use the public API." Call `chemfp.open()` and leave `chemfp.fps_io` alone.

## How do you write a chemfp FPS Rust reader?

A chemfp FPS Rust reader is a buffered line loop that separates header lines from records, decodes each hex field to bytes and returns an error carrying the line number the first time a line is malformed. This std-only sketch is illustrative; it isn't the mossflux source.

```rust
use std::io::{BufRead, BufReader, Read};

pub struct Record { pub id: String, pub bytes: Vec<u8> }

fn nib(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

pub fn read_fps<R: Read>(input: R) -> Result<Vec<Record>, String> {
    let (mut out, mut bits): (Vec<Record>, Option<usize>) = (Vec::new(), None);
    for (i, line) in BufReader::new(input).lines().enumerate() {
        let n = i + 1;
        let line = line.map_err(|e| format!("line {n}: {e}"))?;
        if let Some(h) = line.strip_prefix('#') {
            if !out.is_empty() { return Err(format!("line {n}: header after a record")); }
            if let Some(v) = h.strip_prefix("num_bits=") {
                bits = Some(v.trim().parse().map_err(|_| format!("line {n}: bad num_bits"))?);
            }
            continue;
        }
        let (hex, id) = line.split_once('\t').ok_or(format!("line {n}: no tab"))?;
        if id.trim().is_empty() { return Err(format!("line {n}: missing identifier")); }
        if hex.len() % 2 != 0 { return Err(format!("line {n}: odd-length hex")); }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        for p in hex.as_bytes().chunks(2) {
            match (nib(p[0]), nib(p[1])) {
                (Some(a), Some(b)) => bytes.push(a << 4 | b),
                _ => return Err(format!("line {n}: non-hex character")),
            }
        }
        if bits.map_or(false, |b| bytes.len() != (b + 7) / 8) {
            return Err(format!("line {n}: length differs from num_bits"));
        }
        out.push(Record { id: id.trim().to_string(), bytes });
    }
    Ok(out)
}
```

1. Wrap the file in `BufReader` and number every line. `enumerate()` plus one gives the line number that every error message carries.
2. Handle `#` lines first and read `num_bits` from them. A header line after a record is an error, so the size you check against is always the one the file declared.
3. Split each record at the tab. A line without a tab, or with an empty identifier, is an error.
4. Decode the hex pairwise. Odd length and non-hex characters both fail with the line number. Words of `u64` work the same way if your search loop wants them.
5. Compare the decoded length with the header's size. A mismatch is an error.

On success the chemfp FPS Rust reader returns one `Record` per molecule. On failure you get a message such as `line 4: odd-length hex`.

A chemfp FPS Rust reader should never skip a bad line and carry on. A skipped record is a false negative that no later search can recover, and every score afterwards describes a smaller library than you think.

For gzip, put a decompressing reader in front of the same loop. `read_fps` takes any `Read`, so nothing else changes. The [mossflux CLI](https://mossflux.com/docs/cli) loads plain and gzip FPS files this way.

## How do you search a chemfp FPS file with mossflux?

mossflux searches a chemfp FPS file from the shell with `mossflux search`, and the binary has to be built from source because no prebuilt binaries exist. The [mossflux CLI docs](https://mossflux.com/docs/cli) describe the command: it searches files from the shell, and results go to stdout while diagnostics go to stderr.

1. Install the CLI with Rust 1.95 or later, following the [mossflux install steps](https://mossflux.com/#get), on the machine that will do the searching:

   ```sh
   RUSTFLAGS="-C target-cpu=native" cargo install --git https://github.com/loruorg/mossflux mossflux-cli
   ```

 Build and search on the same machine. There's no crates.io release and no Python binding, so `cargo install --git` is the only route.

2. Run `mossflux search --help` to list the options for your build. It prints the flag names for the query, threshold, k and output format, and the CLI docs list the same set.

3. Point the command at your FPS file. mossflux loads chemfp FPS files plain or gzip, so a `.fps.gz` file needs no unpacking. If the load fails on a bad line, fix it in the source rather than looking for a skip flag.

4. Choose text or TSV output. Text suits a terminal. TSV suits a script, and because diagnostics go to stderr, `mossflux search ... > hits.tsv` keeps messages out of the file.

5. Read the counts, then the hits. The count is always exact. `--limit` caps only the printed hits, so `--limit 10` never turns a 4,000-hit search into a 10-hit count.

To check a new build, run your own query over a small FPS file you can scan by hand. The count should equal the number of rows you find yourself.

## How is a loaded FPS file searched: threshold or kNN?

A loaded FPS file is searched in one of two modes: threshold search returns every molecule with Tanimoto >= t, and kNN returns the k nearest molecules to the query.

Tanimoto is shared set bits divided by the bits set in either fingerprint. kNN ranks by the exact fraction, so two scores that differ in the last digit never swap places because of rounding.

Ties follow a fixed rule: the row with the lower popcount (the count of set bits) comes first, then the earlier row. If rows 3 and 9 both score 0.75 and row 9 has 40 set bits against 48 for row 3, row 9 ranks first. If both have 40, row 3 ranks first.

The [mossflux Correctness page](https://mossflux.com/docs/correctness) describes what "exact" means and the tests that prove it: threshold and kNN results must equal a brute-force scan, with no false negatives and no false positives.

Speed comes from pruning, and every prune has to be provable. The arena is sorted by popcount. For a query with popcount a, a molecule with popcount b can reach Tanimoto t only if min(a, b) / max(a, b) >= t, so rows outside that popcount window are never scored.

mossflux computes the window edge as a rational number rounded down, so floating-point rounding can never push a true hit outside it. A prune rejects a row only on a proven bound.

An inverted bitmap index is the second route to the same answer. It returns the same rows as the linear scan, so the choice between them changes the time taken and never the hits.

## Where do FPS files come from, and what does a score of 1.0 mean?

An FPS file comes from whatever toolkit computed the fingerprints, and a score of 1.0 means two fingerprints have the same bits, not that two molecules are the same compound.

chemfp is one source: its documentation says the [`open_fingerprint_writer()` function](https://chemfp.readthedocs.io/en/latest/chemfp_fps_io.html) "returns an FPSWriter to write to an FPS file". Other toolkits can export to the same layout, so the bits depend on the program and settings that wrote the file.

A fingerprint works like a checklist of structural features, with one tick box per bit. Tanimoto divides the ticks two checklists share by the ticks either one has, so identical checklists score 1.0.

Two different molecules can tick exactly the same boxes. Folding many features into a small number of bits makes this more likely, and a fingerprint records which features exist, not how they connect.

Treat a 1.0 as "same bits, check the structures" and compare the SMILES before you call it a duplicate. A chemfp FPS Rust reader can't make that call for you.

The reverse also holds. The sodium salt of aspirin, `CC(=O)Oc1ccccc1C(=O)[O-].[Na+]`, and the free acid, `CC(=O)Oc1ccccc1C(=O)O`, are the same parent compound. Their bits will usually differ, so the pair scores below 1.0.

Canonicalise before you write the FPS file:

- Strip salts and keep the parent.
- Pick one tautomer form.
- Decide how to treat stereo.

A chemfp FPS Rust reader sees only hex digits and identifiers, so it can't tell which form you meant. mossflux reads the bits as they are and leaves this step to you.

Fingerprints from different toolkits aren't interchangeable. Search a query against a database only when both were made by the same program with the same fingerprint type, radius and width.

## Should a chemfp FPS Rust reader stream the file or load an index?

Streaming reads the FPS file again for every query, while an in-memory index parses it once and answers each query from RAM.

| Property | Stream the FPS file | In-memory index (mossflux) |
| --- | --- | --- |
| Start-up | None beyond opening the file | Index built at start |
| Memory | One record at a time | Whole library resident |
| Cost per query | Parse the file again | Search the resident index |
| After a restart | Nothing to rebuild | Rebuilt from the file |
| Adding molecules | Append lines to the file | Segmented append in memory |

mossflux always loads a chemfp FPS file into memory. There's no on-disk index yet.

A loaded index can grow without a rebuild: new fingerprints join as a segment in memory, and no answer changes. The [design notes](https://mossflux.com/docs/design) describe the BitBound pruning that keeps the in-memory path exact.

When many queries hit one library, `mossflux serve` keeps the index resident. The API docs say it "answers the same searches over REST, MCP and gRPC", and all three transports run over one engine, so they can't disagree.

Stream the FPS file for a library you don't want to hold in RAM, or for a process that restarts often and can't wait for an index build.

Use the in-memory index for a library that stays loaded while queries arrive, either in one `mossflux search` run or behind `mossflux serve`.

## What are the limits, and how fast is FPS search?

mossflux has a few limits to know before you rely on it, and one measured speed comparison, against FPSim2 at 256 bits.

1. The index lives in memory. The [mossflux deploy docs](https://mossflux.com/docs/deploy) say the index is built in memory at start and is not saved to disk, so every start rebuilds it. Plan for about 2.5 KB of RAM per molecule.
2. Row ids are u32. A database holds at most what a u32 can number.
3. Fingerprints reach 65,472 bits.
4. Canonicalisation happens in your pipeline: salts, tautomers and stereo are handled before the file reaches mossflux.
5. Server fingerprints are mossflux's own. The [API docs](https://mossflux.com/docs/api) say `mossflux serve` computes 2048-bit ECFP4-style fingerprints that are not RDKit bit for bit. An FPS file written by another toolkit keeps that toolkit's bits.
6. There's no approximate mode, so every search is exact.

The [mossflux benchmarks](https://mossflux.com/docs/benchmarks) report that mossflux was 2.64x to 2.77x faster than FPSim2 on an AMD EPYC 9645 VM, with 256-bit Morgan r=2, ChEMBL 37 2.9M, 1 mossflux thread against 1 FPSim2 worker, search only, median of 11.

| Tool | Threads | Search time against FPSim2 |
| --- | --- | --- |
| mossflux | 1 thread | 2.64x to 2.77x faster |
| FPSim2 | 1 worker | baseline |

The timings were taken before the 2026-10-03 hardening pass and were not re-measured on the new code. The exactness gates pass on it.

No claim is made about chemfp, and no aggregate win over Arthor is claimed.

## Frequently asked questions

### How do I read a chemfp FPS file in Rust?

Open the file with a buffered reader, read the `#` header lines, then split each record at the tab, hex-decode the fingerprint into bytes or words and keep the identifier. On a malformed line, return an error with its line number. The [mossflux CLI](https://mossflux.com/docs/cli) loads plain or gzip FPS files.

### Can I use chemfp.fps_io directly?

Use `chemfp.open()` instead. The [chemfp documentation](https://chemfp.readthedocs.io/en/latest/chemfp_fps_io.html) describes `fps_io` as an internal module that programs using the public API should not import, and `chemfp.open()` returns an `FPSReader` for an FPS file.

### What does a Tanimoto score of 1.0 mean in an FPS search?

It means the two fingerprints have the same bits, not that the two molecules are the same compound. Compare the SMILES before you call the pair a duplicate.

### Are mossflux results the same as a brute-force scan?

mossflux returns exactly what a brute-force scan returns, with no false negatives and no false positives. The [correctness docs](https://mossflux.com/docs/correctness) describe the tests behind that guarantee.

## What should you do next with your FPS files?

Check a chemfp FPS Rust reader in stages, starting with the file itself.

Today, open one FPS file. Compare its `#` header lines with the format, then check a few records for a hex fingerprint, a tab and an identifier. Confirm the hex length matches the declared bit count.

This week, write or adopt a reader that stops on a bad line and reports its line number. Build mossflux from source with `cargo install --git`, run `mossflux search` over a library, and compare the hits with a brute-force scan of the same file. The two lists should be identical, including the order of kNN ties.

Later, once you know how many queries you run, choose whether your chemfp FPS Rust reader streams the file or feeds an in-memory index.

## Sources

[mossflux docs](https://mossflux.com/docs/) · [Design · mossflux docs](https://mossflux.com/docs/design) · [The chemfp project, Journal of Cheminformatics](https://doi.org/10.1186/s13321-019-0398-8) · [chemfp.fps_io module — chemfp 4.1 documentation](https://chemfp.readthedocs.io/en/latest/chemfp_fps_io.html) · [CLI · mossflux docs](https://mossflux.com/docs/cli) · [Correctness · mossflux docs](https://mossflux.com/docs/correctness) · [API · mossflux docs](https://mossflux.com/docs/api) · [Benchmarks · mossflux docs](https://mossflux.com/docs/benchmarks). Every other source is linked where the article cites it.
