//! `mossflux substruct-bench`: benchmark harness for
//! [`mossflux_substruct::SubstructIndex`].
//!
//! Reports build throughput, index memory, and per query the screen time,
//! verify time, candidate count and hit count for a fixed battery of SMARTS.
//!
//! # Molecule source
//! 1. `--smiles <file>`: one SMILES per line, or a 2-column `ID<TAB>SMILES`
//!    file. `.gz` is decompressed. A leading `smiles` header line is skipped.
//! 2. Otherwise a seeded synthetic generator makes `--limit` valid molecules.
//!
//! `--limit N` caps the molecule count for both sources.
//!
//! # Exact gate
//! Before timing, the harness checks `screen_query` then `verify_screened`
//! against the brute-force oracle for every probe over a small fixed corpus,
//! and prints no numbers if they differ.
//!
//! # Timing
//! Screen and verify are timed separately through the split entry points,
//! which compose to `search`. Each query runs one discarded warm-up, then
//! `--repeat` timed trials; the median of each stage is reported.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::time::Instant;

use flate2::read::MultiGzDecoder;
use mossflux_substruct::census::is_census_exact;
use mossflux_substruct::{brute, parse_smarts, parse_smiles, SubstructIndex};

use crate::CliError;

/// Arguments for `substruct-bench`.
#[derive(clap::Args)]
pub struct SubstructBenchArgs {
    /// SMILES source: one SMILES per line, or a 2-column `ID<TAB>SMILES` file.
    /// `.gz` is read transparently. Omit to use the seeded synthetic generator.
    #[arg(short, long)]
    pub smiles: Option<PathBuf>,
    /// Maximum molecules read from the file, or the synthetic molecule count.
    #[arg(short, long, default_value_t = 1_000_000)]
    pub limit: usize,
    /// Timed trials per query, after one discarded warm-up. The median is
    /// reported per stage.
    #[arg(short, long, default_value_t = 7)]
    pub repeat: usize,
    /// RNG seed for the synthetic generator.
    #[arg(long, default_value_t = 0x_5AB_57A2)]
    pub seed: u64,
    /// Worker threads for the build and the verify stage (0 = all cores).
    #[arg(long, default_value_t = 0)]
    pub threads: usize,
}

/// One benchmark query: a label, the SMARTS, and a short human description.
struct Probe {
    label: &'static str,
    smarts: &'static str,
    note: &'static str,
}

/// The benchmark SMARTS: common functional groups and scaffolds, plus `[#n]`,
/// ring, degree and recursive queries, some of which the screen cannot prune.
fn probes() -> Vec<Probe> {
    vec![
        Probe {
            label: "benzene",
            smarts: "c1ccccc1",
            note: "aromatic 6-ring",
        },
        Probe {
            label: "carbonyl",
            smarts: "C=O",
            note: "C double-bond O",
        },
        Probe {
            label: "amide",
            smarts: "C(=O)N",
            note: "acyl nitrogen",
        },
        Probe {
            label: "sulfonamide",
            smarts: "S(=O)(=O)N",
            note: "S(=O)(=O)-N",
        },
        Probe {
            label: "ester",
            smarts: "C(=O)OC",
            note: "carboxylic ester",
        },
        Probe {
            label: "nitrile",
            smarts: "C#N",
            note: "triple-bond C-N",
        },
        Probe {
            label: "any-halogen",
            smarts: "[F,Cl,Br,I]",
            note: "OR over halogens (census union)",
        },
        Probe {
            label: "chlorine",
            smarts: "Cl",
            note: "single forced Cl atom",
        },
        Probe {
            label: "fluorine",
            smarts: "F",
            note: "single forced F atom",
        },
        Probe {
            label: "fused-naph",
            smarts: "c1ccc2ccccc2c1",
            note: "fused bicyclic aromatic (naphthalene)",
        },
        Probe {
            label: "pyridine",
            smarts: "c1ccncc1",
            note: "aromatic N 6-ring",
        },
        Probe {
            label: "aromatic-N",
            smarts: "[#7;R]",
            note: "ring nitrogen by number",
        },
        Probe {
            label: "any-ring-bond",
            smarts: "[R]~[R]",
            note: "two ring atoms, any bond",
        },
        Probe {
            label: "branched-C",
            smarts: "[CD3]",
            note: "degree-3 aliphatic carbon (branch point)",
        },
        Probe {
            label: "quat-C",
            smarts: "[CD4]",
            note: "degree-4 aliphatic carbon",
        },
        Probe {
            label: "tertiary-amine",
            smarts: "[ND3]",
            note: "degree-3 aliphatic nitrogen (census-exact skip via N_DEG3)",
        },
        Probe {
            label: "phenol",
            smarts: "c1ccc(O)cc1",
            note: "hydroxyl on benzene",
        },
        Probe {
            label: "carboxylic-acid",
            smarts: "C(=O)[OH]",
            note: "acid (H-count primitive)",
        },
        // Recursive SMARTS $(...): measure the rooted sub-matcher, and check that
        // the screen and census still prune recursive queries.
        Probe {
            label: "rec-carbonyl",
            smarts: "[$(C=O)]",
            note: "recursive carbonyl C (vs plain C=O — recursion overhead)",
        },
        Probe {
            label: "rec-benzene",
            smarts: "[$(c1ccccc1)]",
            note: "recursive benzene atom (ring-rooted env)",
        },
        Probe {
            label: "rec-nonamide-amine",
            smarts: "[NX3;!$(NC=O)]",
            note: "basic amine N excluding amides (negated recursive — common medchem)",
        },
    ]
}

/// The fixed corpus for the exact gate. Every probe gets both hits and misses.
fn gate_corpus() -> Vec<&'static str> {
    vec![
        "c1ccccc1",
        "Cc1ccccc1",
        "c1ccncc1",
        "c1ccc(O)cc1",
        "Oc1ccccc1",
        "c1ccc2ccccc2c1",
        "c1ccc2ncccc2c1",
        "CC(=O)O",
        "CC(=O)N",
        "CC(=O)OC",
        "CC=O",
        "CC#N",
        "CCO",
        "CCN",
        "C1CCCCC1",
        "C1CCNCC1",
        "FC(F)(F)F",
        "ClCCBr",
        "IC",
        "CS(=O)(=O)N",
        "NS(=O)(=O)c1ccccc1",
        "CC(C)(C)C",
        "CCCCCC",
        "OC(=O)c1ccccc1",
        "Nc1ccccc1",
        "CCN(CC)CC",
    ]
}

/// Reads at most `limit` SMILES from a plain list or a 2-column `ID<TAB>SMILES`
/// file (`.gz` decompressed), in file order. A first-line `smiles` header is
/// skipped.
fn read_smiles_file(path: &PathBuf, limit: usize) -> Result<Vec<String>, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("opening {path:?}: {e}"))?;
    let reader: Box<dyn Read> = if path.extension().is_some_and(|e| e == "gz") {
        Box::new(MultiGzDecoder::new(file))
    } else {
        Box::new(file)
    };
    let reader = BufReader::with_capacity(1 << 20, reader);

    let mut out: Vec<String> = Vec::new();
    for (lineno, line) in reader.lines().enumerate() {
        if out.len() >= limit {
            break;
        }
        let line = line.map_err(|e| format!("reading {path:?}: {e}"))?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // One column: the whole line. Otherwise the second field (split on space
        // or TAB).
        let mut fields = line.split([' ', '\t']);
        let first = fields.next().unwrap_or("");
        let smiles = match fields.next() {
            Some(second) if !second.is_empty() => second,
            // Single column: the line itself is the SMILES.
            _ => first,
        };
        // Skip a `smiles` / `canonical_smiles` header on the first line.
        if lineno == 0
            && (smiles.eq_ignore_ascii_case("smiles")
                || smiles.eq_ignore_ascii_case("canonical_smiles"))
        {
            continue;
        }
        out.push(smiles.to_string());
    }
    if out.is_empty() {
        return Err(format!("{path:?} yielded no SMILES rows"));
    }
    Ok(out)
}

/// Deterministic synthetic molecules: drug-like motifs, each with a seeded
/// carbon chain before or after it. Every string is valid SMILES.
fn synth_smiles(n: usize, seed: u64) -> Vec<String> {
    // Motifs that carry the groups the probes look for.
    const MOTIFS: &[&str] = &[
        "c1ccccc1",
        "Cc1ccccc1",
        "c1ccncc1",
        "c1ccc(O)cc1",
        "c1ccc2ccccc2c1",
        "CC(=O)O",
        "CC(=O)N",
        "CC(=O)OC",
        "CC#N",
        "FC(F)(F)F",
        "ClCCBr",
        "NS(=O)(=O)c1ccccc1",
        "OC(=O)c1ccccc1",
        "Nc1ccccc1",
        "C1CCNCC1",
        "CS(=O)(=O)N",
        "CCN(CC)CC",
    ];
    let mut rng = seed | 1;
    let mut next = || {
        rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        rng >> 33
    };
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let motif = MOTIFS[(next() as usize) % MOTIFS.len()];
        let chain_len = 1 + (next() as usize) % 6;
        let chain: String = "C".repeat(chain_len);
        // Chain before or after the motif.
        if next() & 1 == 0 {
            out.push(format!("{chain}{motif}"));
        } else {
            out.push(format!("{motif}{chain}"));
        }
    }
    out
}

/// Median of a slice of `f64` (sorted copy). Empty ⇒ 0.0.
fn median(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return 0.0;
    }
    let mut v = vals.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[(v.len() - 1) / 2]
}

/// The exact gate: for every probe over the gate corpus, the split pipeline must
/// equal the brute-force oracle. Returns `Err` if any probe disagrees.
fn exact_gate() -> Result<(), String> {
    let smiles = gate_corpus();
    let idx = SubstructIndex::from_smiles(&smiles)
        .map_err(|e| format!("gate corpus failed to build: {e}"))?;
    let mut compared = 0usize;
    let mut total_hits = 0usize;
    for p in probes() {
        // The pipeline answer via the split entry points (what the bench times).
        let screened = idx
            .screen_query(p.smarts)
            .map_err(|e| format!("probe {} ({:?}) failed to parse: {e}", p.label, p.smarts))?;
        let cands = screened.rows().to_vec();
        let got = idx.verify_screened(&screened);

        // The oracle: brute force over every molecule.
        let q = parse_smarts(p.smarts)
            .map_err(|e| format!("probe {} ({:?}) failed to parse: {e}", p.label, p.smarts))?;
        let mut want: Vec<u32> = Vec::new();
        for (row, sm) in smiles.iter().enumerate() {
            // Oracle re-parses the SMILES (the index stores the compact MatchMol).
            let mol = parse_smiles(sm).map_err(|e| format!("gate parse {sm:?}: {e}"))?;
            if brute::matches(&q, &mol) {
                want.push(row as u32);
            }
        }

        if got != want {
            return Err(format!(
                "EXACT GATE FAILED on probe {} ({:?}):\n  got  {:?}\n  want {:?}",
                p.label, p.smarts, got, want
            ));
        }
        // Candidates must be a sound superset of the verified hits.
        let cand_set: std::collections::HashSet<u32> = cands.iter().copied().collect();
        for &h in &got {
            if !cand_set.contains(&h) {
                return Err(format!(
                    "EXACT GATE FAILED on probe {} ({:?}): hit {h} not in screen candidates",
                    p.label, p.smarts
                ));
            }
        }
        compared += idx.len();
        total_hits += got.len();
    }
    if compared == 0 || total_hits == 0 {
        return Err("exact gate ran vacuously (no comparisons or no hits)".into());
    }
    eprintln!(
        "exact gate: split pipeline == brute-force oracle on {} probes over {} molecules ({} hits) \u{2713}",
        probes().len(),
        gate_corpus().len(),
        total_hits
    );
    Ok(())
}

/// Entry point for `mossflux substruct-bench`.
pub fn run(a: SubstructBenchArgs, out: &mut impl Write) -> Result<(), CliError> {
    if a.limit == 0 {
        return Err("--limit must be > 0".into());
    }

    // The rayon pool for the build and the verify stage.
    let pool = if a.threads == 0 {
        rayon::ThreadPoolBuilder::new().build()
    } else {
        rayon::ThreadPoolBuilder::new()
            .num_threads(a.threads)
            .build()
    }
    .map_err(|e| e.to_string())?;
    let nthreads = pool.current_num_threads();

    // --- Exact gate. ---
    exact_gate()?;

    // --- Acquire the SMILES corpus. ---
    let (smiles, source): (Vec<String>, String) = if let Some(path) = &a.smiles {
        eprintln!("reading SMILES from {path:?} (limit {}) ...", a.limit);
        let s = read_smiles_file(path, a.limit)?;
        let label = format!("file {path:?}");
        (s, label)
    } else {
        eprintln!(
            "no --smiles file; generating {} seeded synthetic molecules ...",
            a.limit
        );
        (synth_smiles(a.limit, a.seed), "synthetic".to_string())
    };
    eprintln!(
        "corpus: {} SMILES from {source}",
        fmt_int(smiles.len() as u64)
    );

    // --- Build the index inside the pool. Unparseable rows are skipped and counted. ---
    let t_build = Instant::now();
    let (idx, skipped) = pool.install(|| SubstructIndex::from_smiles_lossy(&smiles));
    let build = t_build.elapsed();
    let n = idx.len();
    if n == 0 {
        return Err("index is empty after build (every SMILES was unparseable)".into());
    }
    if skipped > 0 {
        eprintln!(
            "  skipped {} / {} unparseable ({:.2}% — outside the supported SMILES subset)",
            fmt_int(skipped as u64),
            fmt_int(smiles.len() as u64),
            100.0 * skipped as f64 / smiles.len() as f64
        );
    }

    let mem = idx.mem_bytes();
    writeln!(out, "\n=== build ===")?;
    writeln!(out, "  molecules        : {}", fmt_int(n as u64))?;
    writeln!(
        out,
        "  build time       : {:.3} s  ({} molecules/s)",
        build.as_secs_f64(),
        fmt_int((n as f64 / build.as_secs_f64().max(1e-9)) as u64)
    )?;
    writeln!(
        out,
        "  index memory     : {:.1} MiB  ({} bytes/molecule)",
        mem as f64 / (1024.0 * 1024.0),
        fmt_int((mem as f64 / n as f64) as u64)
    )?;
    writeln!(
        out,
        "  screen memory    : {:.1} MiB  (of total)",
        idx.screen().mem_bytes() as f64 / (1024.0 * 1024.0)
    )?;
    writeln!(out, "  threads          : {nthreads}")?;

    // --- Per-query battery: screen / verify / total, candidates, hits. ---
    writeln!(
        out,
        "\n=== queries (median of {} trials, {nthreads} threads) ===",
        a.repeat.max(1)
    )?;
    // `path` is `skip` for a census-exact query (no candidate reaches VF2), else
    // `verify`; `v ns/cand` is the verify cost per candidate.
    writeln!(
        out,
        "  {:<17} {:>6} {:>11} {:>9} {:>10} {:>10} {:>11} {:>9}",
        "query", "path", "candidates", "hits", "screen ms", "verify ms", "v ns/cand", "scanned %"
    )?;
    writeln!(out, "  {}", "-".repeat(92))?;

    let mut any_hit = false;
    // Totals for the verify-bound residual summary.
    let (mut skip_probes, mut verify_probes) = (0usize, 0usize);
    let mut verify_cands_total = 0u64;
    let (mut verify_time_ms_total, mut total_query_ms) = (0.0f64, 0.0f64);
    for p in probes() {
        // A census-exact query skips VF2; others verify the screened candidates.
        let exact = parse_smarts(p.smarts)
            .map(|q| is_census_exact(&q))
            .unwrap_or(false);

        let screened = match idx.screen_query(p.smarts) {
            Ok(v) => v,
            Err(e) => {
                writeln!(out, "  {:<17} parse error: {e}", p.label)?;
                continue;
            }
        };
        let (mask_bits, cands) = (screened.mask_bits(), screened.rows().to_vec());
        let hits = idx.verify_screened(&screened);
        any_hit |= !hits.is_empty();

        // Time the screen stage and the verify stage separately, inside the pool.
        let r = a.repeat.max(1);
        let mut screen_ms = Vec::with_capacity(r);
        let mut verify_ms = Vec::with_capacity(r);

        pool.install(|| {
            // Warm-up (discarded).
            let warm = idx.screen_query(p.smarts).unwrap();
            let _ = idx.verify_screened(&warm);

            for _ in 0..r {
                // Screen-only.
                let ts = Instant::now();
                let c = idx.screen_query(p.smarts).unwrap();
                let s_el = ts.elapsed().as_secs_f64() * 1e3;

                // Verify-only over the just-screened candidates.
                let tv = Instant::now();
                let h = idx.verify_screened(&c);
                let v_el = tv.elapsed().as_secs_f64() * 1e3;

                std::hint::black_box(h.len());
                std::hint::black_box(c.rows().len());
                screen_ms.push(s_el);
                verify_ms.push(v_el);
            }
        });

        let screen_med = median(&screen_ms);
        let verify_med = median(&verify_ms);
        let scanned_pct = 100.0 * cands.len() as f64 / n as f64;
        // `*` marks a query that neither the bit-screen nor the census could prune
        // (about every row reaches verify), keyed on the final scanned fraction.
        let _ = mask_bits;
        let cand_note = if scanned_pct >= 99.5 { "*" } else { " " };
        let path = if exact { "skip" } else { "verify" };
        // Verify cost per candidate, only when VF2 runs over some candidates.
        let v_ns_cand = if !exact && !cands.is_empty() {
            format!("{:.0}", verify_med * 1e6 / cands.len() as f64)
        } else {
            "-".to_string()
        };

        // Accumulate the residual totals.
        total_query_ms += screen_med + verify_med;
        if exact {
            skip_probes += 1;
        } else {
            verify_probes += 1;
            verify_cands_total += cands.len() as u64;
            verify_time_ms_total += verify_med;
        }

        writeln!(
            out,
            "  {:<16}{}{:>6} {:>11} {:>9} {:>10.4} {:>10.4} {:>11} {:>8.2}%",
            p.label,
            cand_note,
            path,
            fmt_int(cands.len() as u64),
            fmt_int(hits.len() as u64),
            screen_med,
            verify_med,
            v_ns_cand,
            scanned_pct,
        )?;
    }
    writeln!(
        out,
        "  (* = neither bit-screen nor census could prune: ~every row reaches \
         verify — the safe verify-everything path)"
    )?;

    // --- Verify-bound residual: the verify share of total query time. ---
    let verify_share = if total_query_ms > 0.0 {
        100.0 * verify_time_ms_total / total_query_ms
    } else {
        0.0
    };
    writeln!(
        out,
        "\n=== verify-bound residual (the lever-gating measurement) ==="
    )?;
    writeln!(
        out,
        "  census-exact SKIP probes : {:>3} / {}   (0 candidates reach VF2)",
        skip_probes,
        skip_probes + verify_probes
    )?;
    writeln!(
        out,
        "  VF2-verify probes        : {:>3}        (Σ candidates reaching VF2 = {})",
        verify_probes,
        fmt_int(verify_cands_total)
    )?;
    writeln!(
        out,
        "  Σ verify time            : {:.3} ms   ({:.1}% of total battery query time)",
        verify_time_ms_total, verify_share
    )?;
    writeln!(
        out,
        "  => a small Σ-candidates-reaching-VF2 and a low verify-time share mean the\n     \
         substructure path is already drained — further pre-VF2/matcher work has low ROI.\n     \
         A large residual on specific `verify` rows (high `v ns/cand` × candidates) is\n     \
         where a new exact/forward-sound census class would actually pay."
    )?;
    writeln!(out, "\nnotes:")?;
    for p in probes() {
        writeln!(out, "  {:<16} {:<18} {}", p.label, p.smarts, p.note)?;
    }

    if !any_hit {
        return Err("no probe matched any molecule — corpus may be unrepresentative".into());
    }
    Ok(())
}

/// Format an integer with thousands separators.
fn fmt_int(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact gate passes: every probe agrees with the brute-force oracle.
    #[test]
    fn exact_gate_passes() {
        exact_gate().expect("substruct-bench exact gate must pass");
    }

    /// The synthetic generator makes the requested count of SMILES that build an
    /// index and match the benzene and carbonyl probes.
    #[test]
    fn synthetic_corpus_builds_and_matches() {
        let smiles = synth_smiles(500, 0xBEEF);
        assert_eq!(smiles.len(), 500);
        let idx = SubstructIndex::from_smiles(&smiles).expect("synthetic corpus must build");
        assert_eq!(idx.len(), 500);
        // Benzene and carbonyl must both have hits.
        let benzene = idx.search("c1ccccc1").unwrap();
        let carbonyl = idx.search("C=O").unwrap();
        assert!(!benzene.is_empty(), "synthetic corpus had no benzene");
        assert!(!carbonyl.is_empty(), "synthetic corpus had no carbonyl");
    }

    /// `median` handles even/odd lengths and empty without panicking.
    #[test]
    fn median_basic() {
        assert_eq!(median(&[]), 0.0);
        assert_eq!(median(&[3.0]), 3.0);
        assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
        // Even length: lower-middle element of the sorted slice.
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), 2.0);
    }

    /// A plain one-SMILES-per-line file (and a 2-column TSV) both parse, honour
    /// the limit, and skip blank lines / a SMILES header.
    #[test]
    fn read_smiles_file_plain_and_tsv() {
        use std::io::Write;
        let dir = std::env::temp_dir();
        // Per-process file names, removed by a guard (also on panic), so
        // concurrent test runs cannot race.
        struct Cleanup(Vec<std::path::PathBuf>);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                for p in &self.0 {
                    let _ = std::fs::remove_file(p);
                }
            }
        }
        let pid = std::process::id();

        // Plain list with a blank line.
        let plain = dir.join(format!("mossflux_bench_plain_{pid}.smi"));
        let tsv = dir.join(format!("mossflux_bench_tsv_{pid}.tsv"));
        let _cleanup = Cleanup(vec![plain.clone(), tsv.clone()]);
        {
            let mut f = std::fs::File::create(&plain).unwrap();
            writeln!(f, "CCO").unwrap();
            writeln!(f).unwrap();
            writeln!(f, "c1ccccc1").unwrap();
            writeln!(f, "CC(=O)O").unwrap();
        }
        let got = read_smiles_file(&plain, 10).unwrap();
        assert_eq!(got, vec!["CCO", "c1ccccc1", "CC(=O)O"]);
        // Limit is honoured.
        let got2 = read_smiles_file(&plain, 2).unwrap();
        assert_eq!(got2, vec!["CCO", "c1ccccc1"]);

        // 2-column ID<TAB>SMILES, no header.
        {
            let mut f = std::fs::File::create(&tsv).unwrap();
            writeln!(f, "1\tCCO").unwrap();
            writeln!(f, "2\tc1ccccc1").unwrap();
        }
        let got3 = read_smiles_file(&tsv, 10).unwrap();
        assert_eq!(got3, vec!["CCO", "c1ccccc1"]);
    }
}
