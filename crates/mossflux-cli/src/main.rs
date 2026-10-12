//! `mossflux` — exact molecular similarity and substructure search.
//!
//! `search` / `info` / `bench` load a chemfp FPS file into an in-memory
//! popcount-sorted arena and run exact BitBound Tanimoto search. `substruct`
//! indexes a SMILES file in memory and runs exact SMARTS substructure search.
//! Nothing is persisted between runs: every command loads its input afresh.
//!
//! Results go to stdout and diagnostics (load times, warnings) to stderr, so
//! stdout can be piped. Exit status: 0 on success, 1 on any error, 2 on a usage
//! error (from clap). A closed stdout pipe (`| head`) ends the run quietly with
//! status 0.

use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use clap::{Parser, Subcommand};
use mossflux_index::{fps, Arena, BitmapScratch, InvertedBitmap, MAX_NBITS};
use rayon::prelude::*;

mod serve;
mod substruct_bench;
mod substruct_search;

/// A command failure: a message for stderr, or a closed stdout.
#[derive(Debug)]
pub(crate) enum CliError {
    /// Print as `error: <message>` and exit 1.
    Msg(String),
    /// Writing stdout failed. A broken pipe is a normal end (`| head`).
    Io(io::Error),
}

impl From<String> for CliError {
    fn from(s: String) -> Self {
        CliError::Msg(s)
    }
}

impl From<&str> for CliError {
    fn from(s: &str) -> Self {
        CliError::Msg(s.to_string())
    }
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        CliError::Io(e)
    }
}

/// How results are written to stdout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub(crate) enum OutputFormat {
    /// Human-readable summary line plus indented hits.
    Text,
    /// Tab-separated values with a header row, one hit per row.
    Tsv,
}

/// Fast molecular similarity search.
#[derive(Parser)]
#[command(name = "mossflux", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Benchmark search throughput over an FPS file (search-only, warm cache).
    Bench(BenchArgs),
    /// Search an FPS file for neighbours of one query fingerprint.
    Search(SearchArgs),
    /// Exact SMARTS substructure search over a SMILES file.
    Substruct(substruct_search::SubstructArgs),
    /// Load an FPS file and print arena statistics.
    Info {
        #[arg(short, long)]
        fps: PathBuf,
    },
    /// Tanimoto similarity between two hex fingerprints (debug check).
    Tanimoto { a: String, b: String },
    /// Benchmark the from-scratch SUBSTRUCTURE (SMARTS) engine at scale: build
    /// throughput, index memory, and per-query screen/verify/total latency over a
    /// battery of realistic SMARTS. Exact-gated at startup.
    SubstructBench(substruct_bench::SubstructBenchArgs),
    /// Serve the REST, gRPC and MCP APIs over SMILES databases.
    Serve(serve::ServeArgs),
    /// Serve MCP over stdio over SMILES databases.
    Mcp(serve::EngineArgs),
}

#[derive(clap::Args)]
struct BenchArgs {
    /// FPS file (`.fps` or `.fps.gz`). Omit when using --synth-n.
    #[arg(short, long)]
    fps: Option<PathBuf>,
    /// Generate a synthetic random dataset of this many fingerprints instead.
    #[arg(long)]
    synth_n: Option<usize>,
    /// Synthetic fingerprint width in bits.
    #[arg(long, default_value_t = 256)]
    synth_nbits: usize,
    /// Synthetic per-bit set probability (density).
    #[arg(long, default_value_t = 0.1)]
    synth_density: f64,
    /// Fold loaded fingerprints to this width (e.g. 256) before searching —
    /// gives real folded ECFP at Arthor's width.
    #[arg(long)]
    fold: Option<usize>,
    /// Tanimoto threshold for threshold search.
    #[arg(short, long, default_value_t = 0.7, allow_negative_numbers = true)]
    threshold: f64,
    /// Run k-nearest-neighbour search with this k instead of threshold search.
    #[arg(short, long, conflicts_with = "threshold")]
    knn: Option<usize>,
    /// Number of query fingerprints (molecule-uniform random sample; see --seed).
    #[arg(short, long, default_value_t = 1000)]
    queries: usize,
    /// Worker threads for the parallel pass (0 = all cores).
    #[arg(long, default_value_t = 0)]
    threads: usize,
    /// Independent timed trials of the whole query set (median + spread reported).
    #[arg(short, long, default_value_t = 11)]
    repeat: usize,
    /// RNG seed for molecule-uniform random query sampling.
    #[arg(long, default_value_t = 0xB1054ED)]
    seed: u64,
    /// Also measure raw kernel throughput with a full (unpruned) scan.
    #[arg(long, default_value_t = false)]
    brute: bool,
    /// Also build and benchmark the bit-transposed inverted index.
    #[arg(long, default_value_t = false)]
    inverted: bool,
    /// M2 early-abandon mode: auto, on (force), off (force the vectorized
    /// full-band scan). There is no planner yet, so auto currently resolves to
    /// off. Lets us measure the de-vectorization cost.
    /// Invalid values hard-fail (clap), so a published number is never silently
    /// measured on a different path than requested.
    #[arg(long, value_enum, default_value = "auto")]
    abandon: AbandonMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
enum AbandonMode {
    Auto,
    On,
    Off,
}

#[derive(clap::Args)]
#[command(group(clap::ArgGroup::new("query_source").required(true).multiple(false)))]
struct SearchArgs {
    /// FPS file (`.fps`, or gzip-compressed under any name).
    #[arg(short, long)]
    fps: PathBuf,
    /// Query by record number: the 0-based position of the record in the file.
    #[arg(long, group = "query_source")]
    query_index: Option<usize>,
    /// Query by external id (e.g. CHEMBL153534); the first record with that id.
    #[arg(long, group = "query_source")]
    query_id: Option<String>,
    /// Query by hex fingerprint, in the FPS record encoding and at the file's
    /// exact width.
    #[arg(long, group = "query_source")]
    query: Option<String>,
    /// Tanimoto threshold: return every row with score >= t.
    #[arg(short, long, default_value_t = 0.7, allow_negative_numbers = true)]
    threshold: f64,
    /// k-nearest-neighbour search instead of threshold search. Ties at equal
    /// score go to the lower-popcount row, then the earlier record.
    #[arg(short, long, conflicts_with = "threshold")]
    knn: Option<usize>,
    /// Max hits to print (the hit count is always exact).
    #[arg(short, long, default_value_t = 20)]
    limit: usize,
    /// Output format for results on stdout. Diagnostics always go to stderr.
    #[arg(long, value_enum, default_value = "text")]
    format: OutputFormat,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut out = BufWriter::new(io::stdout().lock());
    let r = match cli.command {
        Command::Bench(a) => cmd_bench(a, &mut out),
        Command::Search(a) => cmd_search(a, &mut out),
        Command::Substruct(a) => substruct_search::run(a, &mut out),
        Command::Info { fps } => cmd_info(&fps, &mut out),
        Command::Tanimoto { a, b } => cmd_tanimoto(&a, &b, &mut out),
        Command::SubstructBench(a) => substruct_bench::run(a, &mut out),
        Command::Serve(a) => serve::run_serve(a),
        Command::Mcp(a) => serve::run_mcp(a),
    }
    .and_then(|()| out.flush().map_err(CliError::from));
    match r {
        Ok(()) => ExitCode::SUCCESS,
        Err(CliError::Io(e)) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(CliError::Io(e)) => {
            eprintln!("error: writing output: {e}");
            ExitCode::FAILURE
        }
        Err(CliError::Msg(e)) => {
            // Flush what was written before the failure; the error itself goes
            // to stderr, so a partial stdout is never mistaken for a result.
            let _ = out.flush();
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn load(path: &PathBuf) -> Result<Arena, String> {
    let (arena, stats) = fps::load_fps(path).map_err(|e| format!("loading {path:?}: {e}"))?;
    report_load(&stats);
    Ok(arena)
}

/// Print the load summary, and a warning for rows that can never match.
fn report_load(stats: &fps::LoadStats) {
    eprintln!(
        "loaded {} fingerprints × {} bits in {:.2}s ({:.0} mol/s)",
        fmt_int(stats.n as u64),
        stats.nbits,
        stats.elapsed.as_secs_f64(),
        stats.n as f64 / stats.elapsed.as_secs_f64()
    );
    if stats.zero_popcount > 0 {
        eprintln!(
            "warning: {} fingerprints have no bits set; they score 0 against every query",
            fmt_int(stats.zero_popcount as u64)
        );
    }
}

fn cmd_bench(a: BenchArgs, out: &mut impl Write) -> Result<(), CliError> {
    if a.queries == 0 {
        return Err("--queries must be > 0".into());
    }
    if a.repeat == 0 {
        return Err("--repeat must be > 0".into());
    }
    let arena = if let Some(sn) = a.synth_n {
        if a.synth_nbits == 0 || a.synth_nbits > MAX_NBITS {
            return Err(CliError::from(format!(
                "--synth-nbits must be in 1..={MAX_NBITS}"
            )));
        }
        if !(0.0..=1.0).contains(&a.synth_density) {
            return Err("--synth-density must be in [0, 1]".into());
        }
        if sn > u32::MAX as usize
            || sn
                .checked_mul(mossflux_core::lanes_for_bits(a.synth_nbits) * 8)
                .is_none()
        {
            return Err("--synth-n is too large (an arena holds at most u32::MAX rows)".into());
        }
        eprintln!(
            "generating synthetic {} × {} bits, density {} ...",
            fmt_int(sn as u64),
            a.synth_nbits,
            a.synth_density
        );
        let t = Instant::now();
        let ar = Arena::synthetic(sn, a.synth_nbits, a.synth_density, 0xB100D);
        eprintln!(
            "  generated in {:.1}s (mean popcount ~{:.0})",
            t.elapsed().as_secs_f64(),
            a.synth_density * a.synth_nbits as f64
        );
        ar
    } else if let Some(f) = &a.fps {
        load(f)?
    } else {
        return Err("provide --fps <file> or --synth-n <count>".into());
    };
    let arena = if let Some(fb) = a.fold {
        if fb == 0 || fb > arena.nbits() {
            return Err(CliError::from(format!(
                "--fold must be in 1..={} (the loaded width)",
                arena.nbits()
            )));
        }
        eprintln!(
            "folding {} → {fb} bits (real folded ECFP)...",
            arena.nbits()
        );
        arena.folded(fb)
    } else {
        arena
    };
    let n = arena.len();
    if n == 0 {
        return Err("empty dataset".into());
    }
    let nq = a.queries.min(n);
    // Molecule-uniform random sampling (seeded, distinct rows): each arena row is
    // a real molecule, so uniform-over-rows == uniform-over-molecules, giving the
    // dataset's natural popcount distribution. (An even stride over the
    // popcount-SORTED arena would bias queries to uniform-over-popcount — a flaw
    // flagged by the methodology audit.)
    let mut rng = a.seed | 1;
    let mut seen = std::collections::HashSet::with_capacity(nq * 2);
    let mut rows: Vec<usize> = Vec::with_capacity(nq);
    while rows.len() < nq {
        rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let r = (rng >> 33) as usize % n;
        if seen.insert(r) {
            rows.push(r);
        }
    }
    // Drop all-zero fingerprints (qpop==0 is a no-op query).
    let queries: Vec<Vec<u64>> = rows
        .iter()
        .map(|&r| arena.row(r).to_vec())
        .filter(|q| mossflux_core::popcount(q) > 0)
        .collect();
    if queries.is_empty() {
        return Err("all sampled queries were empty (zero popcount)".into());
    }
    let qcount = queries.len();
    {
        let mut qp: Vec<u32> = queries.iter().map(|q| mossflux_core::popcount(q)).collect();
        qp.sort_unstable();
        let pick = |p: f64| qp[(((qp.len() - 1) as f64) * p).round() as usize];
        eprintln!(
            "query popcount: min {} p25 {} median {} p75 {} max {} (molecule-uniform, seed {:#x})",
            qp[0],
            pick(0.25),
            pick(0.5),
            pick(0.75),
            qp[qp.len() - 1],
            a.seed
        );
    }

    eprintln!(
        "benchmarking {} queries, repeat {}, mode {}\n",
        qcount,
        a.repeat,
        match a.knn {
            Some(k) => format!("knn k={k}"),
            None => format!("threshold t={}", a.threshold),
        }
    );

    let warm_hits: usize = queries
        .iter()
        .map(|q| run_one(&arena, q, a.threshold, a.knn))
        .sum();

    // Unconditional independent-f64 exactness gate on the HEADLINE linear path
    // (threshold mode). The BitBound scan that every other number is measured
    // against (and gated against, for --inverted) must itself equal a from-scratch
    // f64 ground truth — NOT search_threshold_brute, which shares the rational +
    // integer-prefilter code, so a shared-code bug would pass both. Hard-fail.
    if a.knn.is_none() {
        let mut bad = 0usize;
        for q in &queries {
            let ap = mossflux_core::popcount(q) as u32;
            let mut truth: Vec<u32> = Vec::new();
            for i in 0..arena.len() {
                let b = arena.popcount_of(i) as u32;
                let c = mossflux_core::intersection_popcount(q, arena.row(i));
                let denom = ap + b - c;
                let score = if denom == 0 {
                    0.0
                } else {
                    c as f64 / denom as f64
                };
                if score >= a.threshold {
                    truth.push(i as u32);
                }
            }
            let mut got: Vec<u32> = arena
                .search_threshold(q, a.threshold)
                .iter()
                .map(|h| h.index)
                .collect();
            got.sort_unstable();
            truth.sort_unstable();
            if got != truth {
                bad += 1;
            }
        }
        if bad != 0 {
            return Err(CliError::from(format!(
                "HEADLINE exactness gate FAILED: {bad}/{qcount} queries differ from independent f64 ground truth"
            )));
        }
        eprintln!("headline exactness : BitBound linear == independent f64 truth on all {qcount} queries ✓");
    }

    // kNN exactness gate: search_knn's top-k must equal a from-scratch brute
    // top-k, rows AND scores, exactly. The documented tie rule (score
    // descending, then row ascending) makes the row list unique, so a tie is no
    // excuse for a looser check. Without this a `bench --knn` latency would be
    // measured on an unverified path. f64 keeps every distinct fraction of a
    // supported width distinct, so sorting by the f64 score is the exact order.
    if let Some(k) = a.knn {
        let mut bad = 0usize;
        for q in &queries {
            let ap = mossflux_core::popcount(q);
            let mut all: Vec<(u32, f64)> = (0..arena.len())
                .map(|i| {
                    let b = arena.popcount_of(i) as u32;
                    let c = mossflux_core::intersection_popcount(q, arena.row(i));
                    (i as u32, c as f64 / (ap + b - c) as f64)
                })
                .collect();
            all.sort_by(|x, y| y.1.total_cmp(&x.1).then(x.0.cmp(&y.0)));
            all.truncate(k);
            let got: Vec<(u32, f64)> = arena
                .search_knn(q, k)
                .iter()
                .map(|h| (h.index, h.score))
                .collect();
            if got != all {
                bad += 1;
            }
        }
        if bad != 0 {
            return Err(CliError::from(format!(
                "kNN exactness gate FAILED: {bad}/{qcount} queries differ from brute top-{k}"
            )));
        }
        eprintln!(
            "knn exactness      : top-{k} rows and scores == brute on all {qcount} queries ✓"
        );
    }
    let total_candidates: u64 = if a.knn.is_none() {
        queries
            .iter()
            .map(|q| arena.band_candidate_count(mossflux_core::popcount(q), a.threshold))
            .sum()
    } else {
        0
    };

    // --- Single-thread (median over `repeat` independent trials) ---
    let lin = measure(a.repeat, qcount, || {
        queries
            .iter()
            .map(|q| run_one(&arena, q, a.threshold, a.knn))
            .sum()
    });
    writeln!(out, "=== single-thread (BitBound) ===")?;
    writeln!(
        out,
        "  latency ms/query : median {:.4}  [min {:.4}, p75 {:.4}]",
        lin.median(),
        lin.min(),
        lin.p75()
    )?;
    if a.knn.is_none() {
        let cand_per_q = total_candidates as f64 / qcount as f64;
        writeln!(
            out,
            "  candidates/query : {} of {} ({:.2}% scanned, {:.1}× pruning)",
            fmt_int(cand_per_q as u64),
            fmt_int(n as u64),
            100.0 * cand_per_q / n as f64,
            n as f64 / cand_per_q.max(1.0),
        )?;
        writeln!(
            out,
            "  Tanimotos/s (eff): {:.1} M",
            cand_per_q / (lin.median() / 1e3) / 1e6
        )?;
    }
    writeln!(out, "  hits (warm-up)   : {}", fmt_int(warm_hits as u64))?;

    // --- Parallel (across-query throughput) ---
    let pool = if a.threads == 0 {
        rayon::ThreadPoolBuilder::new().build()
    } else {
        rayon::ThreadPoolBuilder::new()
            .num_threads(a.threads)
            .build()
    }
    .map_err(|e| e.to_string())?;
    let nthreads = pool.current_num_threads();
    let lin_par = measure(a.repeat, qcount, || {
        pool.install(|| {
            queries
                .par_iter()
                .map(|q| run_one(&arena, q, a.threshold, a.knn))
                .sum()
        })
    });
    writeln!(
        out,
        "\n=== {nthreads} threads (across-query throughput) ==="
    )?;
    writeln!(
        out,
        "  latency ms/query : median {:.4}  [min {:.4}]",
        lin_par.median(),
        lin_par.min()
    )?;
    writeln!(
        out,
        "  scaling          : {:.1}×",
        lin.median() / lin_par.median()
    )?;

    // --- Raw kernel throughput (optional, full scan, no pruning) ---
    if a.brute {
        let bq = queries.len().min(200);
        let t2 = Instant::now();
        let mut bsink = 0usize;
        for q in queries.iter().take(bq) {
            bsink = bsink.wrapping_add(arena.search_threshold_brute(q, a.threshold).len());
        }
        let brute = t2.elapsed();
        std::hint::black_box(bsink);
        let comparisons = (bq * n) as f64;
        writeln!(
            out,
            "\n=== raw kernel (full scan, no pruning, 1 thread) ==="
        )?;
        writeln!(
            out,
            "  Tanimotos/s      : {:.1} M  ({} comparisons in {:.3}s)",
            comparisons / brute.as_secs_f64() / 1e6,
            fmt_int(comparisons as u64),
            brute.as_secs_f64()
        )?;
        writeln!(
            out,
            "  per comparison   : {:.2} ns",
            brute.as_secs_f64() * 1e9 / comparisons
        )?;
    }

    // --- Bit-transposed inverted index (the Phase 3 / beat-Arthor path) ---
    if a.inverted {
        if a.knn.is_some() {
            writeln!(
                out,
                "\n(inverted index: threshold mode only; skipped for knn)"
            )?;
            return Ok(());
        }
        eprintln!("\nbuilding inverted (bitmap) index...");
        let tb = Instant::now();
        let inv = InvertedBitmap::build(&arena);
        eprintln!(
            "  built in {:.1}s — {:.1} MiB",
            tb.elapsed().as_secs_f64(),
            inv.mem_bytes() as f64 / (1024.0 * 1024.0)
        );

        // Early-abandon override (auto/on/off) for the measurement scratches.
        let force_abandon = match a.abandon {
            AbandonMode::On => Some(true),
            AbandonMode::Off => Some(false),
            AbandonMode::Auto => None,
        };
        // Exactness gate vs the linear scan on the FULL query set (hard fail).
        let mut sc = BitmapScratch::new();
        sc.set_force_abandon(force_abandon);
        let mut mismatched = 0usize;
        for q in &queries {
            let mut lin: Vec<u32> = arena
                .search_threshold(q, a.threshold)
                .iter()
                .map(|h| h.index)
                .collect();
            let mut iv: Vec<u32> = inv
                .search_threshold(q, a.threshold, &mut sc)
                .iter()
                .map(|h| h.index)
                .collect();
            lin.sort_unstable();
            iv.sort_unstable();
            if lin != iv {
                mismatched += 1;
            }
        }
        writeln!(out, "\n=== inverted index (bitmap + bit-addition tree) ===")?;
        if mismatched == 0 {
            writeln!(
                out,
                "  exactness        : identical to linear on ALL {} queries ✓",
                fmt_int(qcount as u64)
            )?;
        } else {
            return Err(CliError::from(format!(
                "inverted-index exactness gate FAILED: {mismatched}/{qcount} queries mismatched"
            )));
        }

        // M1: DRAM bytes-moved per molecule vs Arthor — the bandwidth-bound win
        // metric. mossflux reads only the BitBound band; Arthor scores 100% of
        // molecules. Both move ~a/8 bytes per *scored* molecule at any width, so
        // the ratio reduces to the band fraction (cache-blocking keeps the planes
        // in L2, so this column term IS the real DRAM term, not plane spill).
        let band_frac = total_candidates as f64 / (qcount as f64 * n as f64);
        writeln!(out,
            "  row-band fraction  : ≈{:.3}× (BitBound candidate share vs Arthor's 100%; = bytes/mol\n                       only under the a/8-per-scored-molecule model) → {}",
            band_frac,
            if band_frac <= 0.70 {
                "fewer rows scored (>=30%)"
            } else {
                "no row-band win at this t"
            }
        )?;
        // M2: effective bytes/mol AFTER rarest-first early-abandon (counts the
        // column-words actually read). Arthor reads ≈ avg_a/8 bytes per molecule.
        {
            let mut sc2 = BitmapScratch::new();
            sc2.set_force_abandon(Some(true)); // measure the abandon-path bytes regardless of the planner gate
            sc2.reset_words_read();
            let mut a_sum = 0u64;
            for q in &queries {
                let _ = inv.search_threshold(q, a.threshold, &mut sc2);
                a_sum += mossflux_core::popcount(q) as u64;
            }
            let mossflux_bpm = sc2.words_read() as f64 * 8.0 / (qcount as f64 * n as f64);
            // Analytic Arthor model: a/8 bytes per scored molecule (its inverted
            // index reads the query's `a` set-bit columns, 1 bit/row), zero index
            // overhead, 100% scored. This is a MODEL, not an Arthor measurement.
            let arthor_bpm = (a_sum as f64 / qcount as f64) / 8.0;
            let ratio = mossflux_bpm / arthor_bpm;
            writeln!(out,
                "  eff bytes/mol      : ≈{:.3}× analytic-Arthor a/8 model (abandon-on; words_read×8) → {}",
                ratio,
                if ratio <= 0.70 { "fewer bytes (>=30%)" } else { "no byte win at this t" }
            )?;
        }

        let bm = measure(a.repeat, qcount, || {
            let mut tot = 0usize;
            for q in &queries {
                tot += inv.search_threshold(q, a.threshold, &mut sc).len();
            }
            tot
        });
        writeln!(
            out,
            "  single-thread    : median {:.4} ms/query  [min {:.4}]  ({:.2}× vs linear)",
            bm.median(),
            bm.min(),
            lin.median() / bm.median()
        )?;
        let bm_par = measure(a.repeat, qcount, || {
            pool.install(|| {
                queries
                    .par_iter()
                    .map_init(
                        || {
                            let mut s = BitmapScratch::new();
                            s.set_force_abandon(force_abandon);
                            s
                        },
                        |scr, q| inv.search_threshold(q, a.threshold, scr).len(),
                    )
                    .sum()
            })
        });
        writeln!(
            out,
            "  {nthreads} threads        : median {:.4} ms/query  ({:.2}× vs linear)",
            bm_par.median(),
            lin_par.median() / bm_par.median()
        )?;
    }
    Ok(())
}

/// Sorted per-trial latencies (ms/query) with robust summary statistics.
struct Stats {
    vals: Vec<f64>,
}
impl Stats {
    fn q(&self, p: f64) -> f64 {
        self.vals[(((self.vals.len() - 1) as f64) * p).round() as usize]
    }
    fn median(&self) -> f64 {
        self.q(0.5)
    }
    fn min(&self) -> f64 {
        self.vals[0]
    }
    fn p75(&self) -> f64 {
        self.q(0.75)
    }
}

/// Run `f` (one full pass over the query set, returning a hit-count sink) for one
/// discarded warm-up trial then `trials` timed trials; return sorted ms/query.
fn measure(trials: usize, per_query: usize, mut f: impl FnMut() -> usize) -> Stats {
    let mut sink = 0usize;
    sink = sink.wrapping_add(f()); // warm-up, discarded
    let trials = trials.max(1);
    let mut vals = Vec::with_capacity(trials);
    for _ in 0..trials {
        let t = Instant::now();
        sink = sink.wrapping_add(f());
        vals.push(t.elapsed().as_secs_f64() * 1e3 / per_query as f64);
    }
    std::hint::black_box(sink);
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Stats { vals }
}

#[inline]
fn run_one(arena: &Arena, q: &[u64], t: f64, knn: Option<usize>) -> usize {
    match knn {
        Some(k) => arena.search_knn(q, k).len(),
        None => arena.search_threshold(q, t).len(),
    }
}

fn cmd_search(a: SearchArgs, out: &mut impl Write) -> Result<(), CliError> {
    let (arena, order) = load_with_order(&a.fps)?;
    let query: Vec<u64> = if let Some(i) = a.query_index {
        if i >= arena.len() {
            return Err(CliError::from(format!(
                "query-index {i} out of range (the file holds {} records)",
                arena.len()
            )));
        }
        // `--query-index` counts records in file order; the arena stores rows
        // popcount-sorted, and `order[row]` is the file record at `row`.
        let row = order
            .iter()
            .position(|&o| o as usize == i)
            .expect("order is a permutation of 0..len");
        arena.row(row).to_vec()
    } else if let Some(id) = &a.query_id {
        let idx = (0..arena.len())
            .find(|&i| arena.id(i) == *id)
            .ok_or_else(|| format!("id {id} not found"))?;
        arena.row(idx).to_vec()
    } else if let Some(hex) = &a.query {
        parse_query_hex(hex, arena.nbits())?
    } else {
        return Err("provide --query-index, --query-id, or --query <hex>".into());
    };

    let t0 = Instant::now();
    let mut hits = match a.knn {
        Some(k) => arena.search_knn(&query, k),
        None => {
            let mut h = arena.search_threshold(&query, a.threshold);
            // Same total order as kNN: score descending, then row ascending.
            h.sort_by(|x, y| y.score.total_cmp(&x.score).then(x.index.cmp(&y.index)));
            h
        }
    };
    let elapsed = t0.elapsed();
    let total = hits.len();
    hits.truncate(a.limit);
    match a.format {
        OutputFormat::Text => {
            writeln!(
                out,
                "{} hits in {:.3} ms (showing {})",
                fmt_int(total as u64),
                elapsed.as_secs_f64() * 1e3,
                hits.len()
            )?;
            for h in &hits {
                writeln!(out, "  {:.4}  {}", h.score, arena.id(h.index as usize))?;
            }
        }
        OutputFormat::Tsv => {
            eprintln!(
                "{} hits in {:.3} ms (showing {})",
                fmt_int(total as u64),
                elapsed.as_secs_f64() * 1e3,
                hits.len()
            );
            // The score column is the exact f64 Tanimoto, printed so it
            // parses back to the same value.
            writeln!(out, "id\tscore")?;
            for h in &hits {
                writeln!(out, "{}\t{}", arena.id(h.index as usize), h.score)?;
            }
        }
    }
    Ok(())
}

fn cmd_info(path: &PathBuf, out: &mut impl Write) -> Result<(), CliError> {
    let arena = load(path)?;
    let n = arena.len();
    let mut min = u16::MAX;
    let mut max = 0u16;
    let mut sum = 0u64;
    for i in 0..n {
        let p = arena.popcount_of(i);
        min = min.min(p);
        max = max.max(p);
        sum += p as u64;
    }
    let median = arena.popcount_of(n / 2);
    writeln!(out, "fingerprints : {}", fmt_int(n as u64))?;
    writeln!(
        out,
        "width        : {} bits ({} u64 lanes)",
        arena.nbits(),
        arena.nwords()
    )?;
    writeln!(
        out,
        "memory        : {:.1} MiB (fingerprints)",
        (n * arena.nwords() * 8) as f64 / (1024.0 * 1024.0)
    )?;
    writeln!(
        out,
        "popcount      : min {min}, median {median}, mean {:.1}, max {max}",
        sum as f64 / n as f64
    )?;
    Ok(())
}

fn cmd_tanimoto(a_hex: &str, b_hex: &str, out: &mut impl Write) -> Result<(), CliError> {
    let a = parse_hex_fp(a_hex)?;
    let b = parse_hex_fp(b_hex)?;
    // Compare the hex lengths, not the lane counts: `ff` and `ff00` both fit one
    // lane but are 8- and 16-bit fingerprints, which have no Tanimoto.
    let (la, lb) = (a_hex.trim().len(), b_hex.trim().len());
    if la == 0 || lb == 0 {
        return Err("empty fingerprint".into());
    }
    if la != lb {
        return Err(CliError::from(format!(
            "fingerprints differ in width ({} vs {} bits)",
            la * 4,
            lb * 4
        )));
    }
    writeln!(out, "{:.6}", mossflux_core::tanimoto(&a, &b))?;
    Ok(())
}

/// Parse a `--query` hex fingerprint for an index of `nbits` bits: exactly
/// `2 * ceil(nbits / 8)` hex characters (the FPS record width), and no bit set at
/// or past `nbits`. Anything else is a different fingerprint than the index
/// holds, so it is an error, never padded, truncated or masked.
fn parse_query_hex(hex: &str, nbits: usize) -> Result<Vec<u64>, String> {
    let v = parse_hex_fp(hex)?;
    let want = 2 * nbits.div_ceil(8);
    let got = hex.trim().len();
    if got != want {
        return Err(format!(
            "query has {got} hex characters, but the {nbits}-bit index needs {want}"
        ));
    }
    let rem = nbits % 64;
    if rem != 0 && v.last().is_some_and(|&w| w >> rem != 0) {
        return Err(format!(
            "query sets a bit at or past position {nbits}, outside the {nbits}-bit index"
        ));
    }
    Ok(v)
}

/// Like [`load`], plus the popcount-sort permutation: `order[row]` is the
/// 0-based file record stored at arena `row`.
fn load_with_order(path: &PathBuf) -> Result<(Arena, Vec<u32>), String> {
    let (arena, order, stats) =
        fps::load_fps_with_order(path).map_err(|e| format!("loading {path:?}: {e}"))?;
    report_load(&stats);
    Ok((arena, order))
}

/// Parse a hex string into packed little-endian `u64` lanes (byte 0 first).
fn parse_hex_fp(hex: &str) -> Result<Vec<u64>, String> {
    let hex = hex.trim();
    // Guard non-ASCII first: `&hex[i..i+2]` would panic on a multibyte char
    // boundary instead of returning an error (reachable from `--query`/`tanimoto`).
    if !hex.is_ascii() {
        return Err("hex string contains non-ASCII characters".to_string());
    }
    if !hex.len().is_multiple_of(2) {
        return Err(format!("hex string has odd length ({})", hex.len()));
    }
    let bytes: Result<Vec<u8>, _> = (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect();
    let bytes = bytes.map_err(|e| format!("invalid hex: {e}"))?;
    let mut words = vec![0u64; bytes.len().div_ceil(8)];
    for (i, &byte) in bytes.iter().enumerate() {
        words[i / 8] |= (byte as u64) << ((i % 8) * 8);
    }
    Ok(words)
}

/// Format an integer with thousands separators.
pub(crate) fn fmt_int(n: u64) -> String {
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
