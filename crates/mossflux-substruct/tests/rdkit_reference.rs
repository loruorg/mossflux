//! External differential: mossflux vs RDKit.
//!
//! The other matcher gates use `brute` as the oracle, which shares the parser and
//! perceived graph with the pipeline, so they cannot see a shared parse or
//! perception error. This test compares against an independent implementation.
//! It is gated on an environment variable so the default `cargo test` needs no
//! Python:
//!
//! ```text
//! MOSSFLUX_RDKIT_PYTHON=/path/to/python cargo test -p mossflux-substruct \
//!     --test rdkit_reference -- --nocapture
//! ```
//!
//! The named interpreter must have `rdkit` importable. Unset, the test reports
//! that it did not run; set, a missing or broken interpreter is a failure.
//!
//! Scope: element identity, explicit bond orders, `X`, `D`, total-H `H`, valence
//! `v`, charge, ring membership (`R`, `R0`, `!R`, `r<n>`, `x<n>`), aromaticity
//! (`c`, `n`, `a`, `A`, `:`), recursive `$(...)`, disconnected components and
//! multi-atom motifs are all asserted.
//!
//! Excluded: `R<n>` for `n >= 1` (SSSR ring count per atom). The SSSR of a
//! bridged polycycle is not unique; RDKit uses a symmetrized SSSR, mossflux a
//! plain one. For bicyclo[2.2.2]octane RDKit's `GetSymmSSSR` returns three rings
//! and `[R1]` matches nothing; a plain SSSR keeps two, and four atoms are in
//! exactly one. `ring_count_model_difference_with_rdkit_is_pinned` asserts this
//! difference.
//!
//! Any other disagreement is a defect, reported as false negatives (mossflux
//! dropped an RDKit match) and false positives.

use std::collections::BTreeSet;
use std::io::Write;
use std::process::Command;

use mossflux_substruct::index::SubstructIndex;

const ENV_PYTHON: &str = "MOSSFLUX_RDKIT_PYTHON";

/// Molecules written so that RDKit and mossflux must perceive the same graph:
/// no isotopes, no explicit-H brackets, no multi-component salts.
const CORPUS: &[&str] = &[
    // chains, branches, heteroatoms
    "C",
    "CC",
    "CCC",
    "CCCC",
    "CCCCCC",
    "CC(C)C",
    "CC(C)(C)C",
    "CCO",
    "CCN",
    "CCS",
    "OCCO",
    "NCCN",
    "CC(O)CC(O)C",
    "CCOCC",
    "OCC(O)CO",
    "CCCCCCCCCC",
    // halogens
    "CF",
    "CCl",
    "CBr",
    "CI",
    "FC(F)(F)F",
    "ClC(Cl)Cl",
    "ClCCBr",
    "FCCF",
    "Fc1ccccc1",
    "Clc1ccccc1",
    "Brc1ccccc1",
    "Fc1ccc(Cl)cc1",
    "FC(F)(F)c1ccccc1",
    // benzene and substituted benzenes, both spellings
    "c1ccccc1",
    "C1=CC=CC=C1",
    "Cc1ccccc1",
    "CC1=CC=CC=C1",
    "Oc1ccccc1",
    "Nc1ccccc1",
    "c1ccc(cc1)O",
    "Cc1ccc(C)cc1",
    // fused / polycyclic
    "c1ccc2ccccc2c1",
    "c1ccc2c(c1)cccc2",
    "c1ccc2ncccc2c1",
    "c1ccc2c(c1)nccn2",
    "c1ccc2c(c1)[nH]c1ccccc12",
    "Clc1ccc2nccnc2c1",
    // heteroaromatics
    "c1ccncc1",
    "C1=CC=NC=C1",
    "c1ccnc(c1)C",
    "c1cncnc1",
    "c1ccoc1",
    "c1ccsc1",
    "c1cc[nH]c1",
    "c1cnccn1",
    "c1ccc(nc1)N",
    // carbonyls, acids, amides, esters, nitriles
    "CC=O",
    "CC(=O)C",
    "CC(=O)O",
    "CC(=O)N",
    "CC(=O)OC",
    "CC(=O)Cl",
    "O=C=O",
    "CC#N",
    "N#CC#N",
    "O=C(O)c1ccccc1",
    "CC(=O)Nc1ccccc1",
    "COC(=O)c1ccccc1",
    "NC(=O)N",
    "NC(=O)OC",
    "CC(=O)OC(=O)C",
    "CC(=O)SC",
    "COC(=O)OC",
    "NC(=S)N",
    "CC(=S)N",
    "CC(=S)O",
    // sulfur / phosphorus / nitro
    "CS(=O)(=O)N",
    "CS(=O)(=O)O",
    "CSC",
    "CSSC",
    "OP(=O)(O)O",
    "C[N+](=O)[O-]",
    "NS(=O)(=O)N",
    "OS(=O)(=O)N",
    "CS(=O)N",
    "CS(=O)(=O)Cl",
    "CS(=O)(=O)C",
    // charged
    "CC(=O)[O-]",
    "C[N+](C)(C)C",
    "[O-]c1ccccc1",
    // saturated rings
    "C1CCCCC1",
    "C1CCCC1",
    "C1CC1",
    "C1CCNCC1",
    "O1CCCC1",
    "C1CCOC1",
    "C1CCSC1",
    "C1CC2CCC1CC2",
    "C1CCC2(CC1)CCCC2",
    // real drugs
    "CC(=O)Oc1ccccc1C(=O)O",
    "CC(C)Cc1ccc(cc1)C(C)C(=O)O",
    "Cn1cnc2c1c(=O)n(C)c(=O)n2C",
    "c1ccc(cc1)S(=O)(=O)Nc1ccccc1",
    "OC(=O)CCCCCCCCC",
];

/// The asserted battery: every primitive on which the two engines are specified
/// to mean the same thing. See the module docs for the one exclusion.
const QUERIES: &[&str] = &[
    // element identity, aromatic-agnostic
    "[#6]",
    "[#7]",
    "[#8]",
    "[#9]",
    "[#15]",
    "[#16]",
    "[#17]",
    "[#35]",
    "[#53]",
    "[!#6]",
    "[!#6;!#1]",
    "[#7,#8]",
    "[#9,#17,#35,#53]",
    "[#6,#7]",
    // explicit bonds between explicit elements
    "[#6]-[#6]",
    "[#6]=[#6]",
    "[#6]#[#6]",
    "[#6]-[#8]",
    "[#6]=[#8]",
    "[#6]-[#7]",
    "[#6]#[#7]",
    "[#6]=[#16]",
    "[#16]=[#8]",
    "[#6]-[#17]",
    "[#6]-[#9]",
    "[#7]-[#6]=[#8]",
    "[#8]-[#6]=[#8]",
    "[#7]-[#16](=[#8])=[#8]",
    "[#6]-[#6]-[#6]",
    "[#6]-[#6]-[#8]",
    "[#8]-[#6]-[#6]-[#8]",
    "[#6]-[#6](-[#6])-[#6]",
    // connectivity / degree / total-H, all element-anchored
    "[#6X4]",
    "[#6X3]",
    "[#8X1]",
    "[#8X2]",
    "[#7X3]",
    "[#16X4]",
    "[#6D1]",
    "[#6D2]",
    "[#6D3]",
    "[#6D4]",
    "[#8D1]",
    "[#8D2]",
    "[#7D1]",
    "[#6H3]",
    "[#6H2]",
    "[#6H1]",
    "[#6H0]",
    "[#8H1]",
    "[#8H0]",
    "[#7H2]",
    // charge
    "[#7+]",
    "[#8-]",
    "[+]",
    "[-]",
    "[#7X4+]",
    // multi-atom motifs, explicit throughout
    "[#6](=[#8])-[#8]",
    "[#6](=[#8])-[#7]",
    "[#16](=[#8])(=[#8])-[#7]",
    "[#9]-[#6](-[#9])(-[#9])-[#6]",
    "[#6]-[#16]-[#6]",
    "[#6]-[#16]-[#16]-[#6]",
    "[#6]1-[#6]-[#6]-[#6]-[#6]-[#6]-1",
    "[#6]1[#6][#6][#6][#6][#6]1",
    "[#6]1[#6][#6][#7][#6][#6]1",
    "[#6]1[#6][#6][#6][#6][#7]1",
    // disconnected components
    "[#6].[#7]",
    "[#6].[#8]",
    "[#9].[#6]1[#6][#6][#6][#6][#6]1",
    // Aromaticity and ring membership: the two aromaticity models coincide on
    // this corpus.
    "c",
    "n",
    "o",
    "s",
    "a",
    "A",
    "C",
    "N",
    "O",
    "S",
    "[R]",
    "[R0]",
    "[!R]",
    "[r3]",
    "[r4]",
    "[r5]",
    "[r6]",
    "cc",
    "CC",
    "c:c",
    "C=C",
    "c1ccccc1",
    "c1ccncc1",
    "C1CCCCC1",
    "[c;R]",
    "[n;R]",
    "[R]~[R]",
    "[x0]",
    "[x2]",
    "[x3]",
    "[cH1]",
    "[nH1]",
    "[cv4]",
    "[nv3]",
    "[Cv4]",
    "C(=O)O",
    "C(=O)N",
    "S(=O)(=O)N",
    "Nc1ccccc1",
    "Oc1ccccc1",
    "c1ccc2ccccc2c1",
    "[$(C=O)]",
    "[!$(C=O)]",
    "[$(c1ccccc1)]",
    // `[R1]` / `[R2]` are absent on purpose: see the module docs and
    // `ring_count_model_difference_with_rdkit_is_pinned`.
];

/// The RDKit side, run once in a subprocess. Reads a file of
/// `M<TAB>smiles` / `Q<TAB>smarts` lines and prints `<query index><TAB><row ids>`.
const PY: &str = r#"
import sys
from rdkit import Chem
from rdkit import RDLogger
RDLogger.DisableLog('rdApp.*')

mols, queries = [], []
for line in open(sys.argv[1]):
    line = line.rstrip('\n')
    if not line:
        continue
    kind, payload = line.split('\t', 1)
    (mols if kind == 'M' else queries).append(payload)

parsed = []
for i, s in enumerate(mols):
    m = Chem.MolFromSmiles(s)
    if m is None:
        print('MOLFAIL\t%d\t%s' % (i, s))
        sys.exit(3)
    parsed.append(m)

for qi, q in enumerate(queries):
    p = Chem.MolFromSmarts(q)
    if p is None:
        print('QFAIL\t%d\t%s' % (qi, q))
        sys.exit(4)
    hits = [str(i) for i, m in enumerate(parsed) if m.HasSubstructMatch(p)]
    print('%d\t%s' % (qi, ','.join(hits)))
print('OK\t%d\t%d' % (len(parsed), len(queries)))
"#;

#[test]
fn matcher_agrees_with_rdkit_on_unambiguous_primitives() {
    let Ok(python) = std::env::var(ENV_PYTHON) else {
        eprintln!(
            "SKIPPED: set {ENV_PYTHON}=<python with rdkit> to run the external \
             RDKit differential. Nothing was compared."
        );
        return;
    };

    // Hand the corpus and battery to Python through a unique scratch file.
    let mut input = std::env::temp_dir();
    input.push(format!("mossflux_rdkit_{}.tsv", std::process::id()));
    {
        let mut f = std::fs::File::create(&input).expect("create scratch input");
        for m in CORPUS {
            writeln!(f, "M\t{m}").unwrap();
        }
        for q in QUERIES {
            writeln!(f, "Q\t{q}").unwrap();
        }
    }
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    let _cleanup = Cleanup(input.clone());

    let out = Command::new(&python)
        .args(["-c", PY])
        .arg(&input)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {python:?}: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        out.status.success(),
        "the RDKit reference process failed ({:?})\n--- stdout ---\n{stdout}\n\
         --- stderr ---\n{stderr}",
        out.status
    );

    // Parse the reference answers.
    let mut reference: Vec<Option<BTreeSet<u32>>> = vec![None; QUERIES.len()];
    let mut saw_ok = false;
    for line in stdout.lines() {
        if let Some(rest) = line.strip_prefix("OK\t") {
            let mut it = rest.split('\t');
            assert_eq!(it.next(), Some(CORPUS.len().to_string().as_str()));
            assert_eq!(it.next(), Some(QUERIES.len().to_string().as_str()));
            saw_ok = true;
            continue;
        }
        let (qi, ids) = line.split_once('\t').unwrap_or((line, ""));
        let qi: usize = qi
            .parse()
            .unwrap_or_else(|_| panic!("unparsable reference line {line:?}"));
        let set: BTreeSet<u32> = ids
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().expect("row id"))
            .collect();
        reference[qi] = Some(set);
    }
    assert!(saw_ok, "the reference process did not finish:\n{stdout}");

    // The mossflux side.
    let smiles: Vec<String> = CORPUS.iter().map(|s| s.to_string()).collect();
    let idx = SubstructIndex::from_smiles(&smiles).expect("corpus must build");
    assert_eq!(idx.len(), CORPUS.len());

    let mut false_negatives: Vec<String> = Vec::new(); // RDKit matched, we dropped
    let mut false_positives: Vec<String> = Vec::new(); // we matched, RDKit did not
    let mut agreed = 0usize;
    let mut total_ref_hits = 0usize;
    for (qi, q) in QUERIES.iter().enumerate() {
        let want = reference[qi]
            .as_ref()
            .unwrap_or_else(|| panic!("no reference answer for query {q:?}"));
        let got: BTreeSet<u32> = idx
            .search(q)
            .unwrap_or_else(|e| panic!("mossflux failed to search {q:?}: {e:?}"))
            .into_iter()
            .collect();
        total_ref_hits += want.len();
        if got == *want {
            agreed += 1;
            continue;
        }
        let missing: Vec<String> = want
            .difference(&got)
            .map(|&i| format!("{i}:{}", CORPUS[i as usize]))
            .collect();
        let extra: Vec<String> = got
            .difference(want)
            .map(|&i| format!("{i}:{}", CORPUS[i as usize]))
            .collect();
        if !missing.is_empty() {
            false_negatives.push(format!("{q:?} missed {missing:?}"));
        }
        if !extra.is_empty() {
            false_positives.push(format!("{q:?} extra {extra:?}"));
        }
    }

    // Anti-vacuity: the reference really matched things, so agreement is not
    // "both returned nothing" repeated.
    assert!(
        total_ref_hits > 500,
        "the RDKit reference produced only {total_ref_hits} hits — nothing was \
         really compared"
    );
    eprintln!(
        "RDKit differential: {agreed}/{} queries agree exactly over {} molecules \
         ({total_ref_hits} reference hits)",
        QUERIES.len(),
        CORPUS.len()
    );
    assert!(
        false_negatives.is_empty() && false_positives.is_empty(),
        "MOSSFLUX DISAGREES WITH RDKIT.\n  FALSE NEGATIVES (RDKit matched, \
         mossflux dropped — the cardinal sin): {false_negatives:#?}\n  false \
         positives (mossflux matched, RDKit did not): {false_positives:#?}"
    );
}

/// Pins the one known model difference between mossflux and RDKit.
///
/// `[R1]` asks for atoms in exactly one SSSR ring. Bicyclo[2.2.2]octane has
/// cyclomatic number 2: a plain SSSR holds two six-rings, and the atoms of the
/// unshared bridges are in exactly one. RDKit's `GetSymmSSSR` returns all three
/// six-rings, so `[R1]` matches nothing.
///
/// If either side changes its SSSR model, this test fails and the exclusion
/// must be revisited.
#[test]
fn ring_count_model_difference_with_rdkit_is_pinned() {
    const BRIDGED: &str = "C1CC2CCC1CC2"; // bicyclo[2.2.2]octane
    let idx = SubstructIndex::from_smiles(&[BRIDGED]).expect("must build");

    // mossflux (plain SSSR, 2 rings): some atom is in exactly one ring.
    assert_eq!(
        idx.search("[R1]").expect("[R1] must parse"),
        vec![0],
        "mossflux uses a plain SSSR, under which {BRIDGED} has [R1] atoms"
    );
    // Sanity that the molecule really is the bridged bicycle and is fully cyclic.
    assert_eq!(
        idx.search("[R0]").expect("[R0] must parse"),
        Vec::<u32>::new()
    );
    assert_eq!(idx.search("[R]").expect("[R] must parse"), vec![0]);
    assert_eq!(idx.search("[r6]").expect("[r6] must parse"), vec![0]);

    let Ok(python) = std::env::var(ENV_PYTHON) else {
        eprintln!(
            "PARTIAL: set {ENV_PYTHON} to also confirm RDKit's half of this pinned              difference."
        );
        return;
    };
    // A raw string, not a `format!` with line continuations, which would indent
    // the second line and break Python. The molecule arrives via argv.
    const PROBE: &str = r#"
import sys
from rdkit import Chem
m = Chem.MolFromSmiles(sys.argv[1])
print(len(Chem.GetSymmSSSR(m)), len(m.GetSubstructMatches(Chem.MolFromSmarts('[R1]'))))
"#;
    let out = Command::new(&python)
        .args(["-c", PROBE])
        .arg(BRIDGED)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {python:?}: {e}"));
    assert!(
        out.status.success(),
        "rdkit probe failed: {:?}\n{}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        text.trim(),
        "3 0",
        "RDKit's symmetrized SSSR is expected to give 3 rings and 0 [R1] atoms \
         for {BRIDGED}; it gave {text:?}. The documented exclusion above no longer \
         describes reality."
    );
}
