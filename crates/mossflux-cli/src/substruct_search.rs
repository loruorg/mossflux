//! `mossflux substruct` — exact SMARTS substructure search over a SMILES file.
//!
//! The file is parsed and indexed once, in memory; every query then runs
//! [`mossflux_substruct::SubstructIndex::search`], whose answer equals the
//! brute-force oracle's. Rows whose SMILES does not parse are skipped and
//! reported on stderr, or fail the run with `--strict`.
//!
//! # Input columns
//!
//! A line is split on TAB when it contains one, otherwise on runs of spaces.
//! Columns are 1-based. When `--smiles-col` / `--id-col` are not given:
//!
//! 1. A `.smi`, `.smiles`, `.ism` or `.can` file (optionally `.gz`) uses the
//!    Daylight layout: SMILES in column 1, name in column 2.
//! 2. Otherwise, if the first line has a column named `smiles` or
//!    `canonical_smiles` (any case), that line is a header: that column holds the
//!    SMILES and the first other column holds the id. The ChEMBL chemreps file
//!    is this case.
//! 3. Otherwise a one-column line is a bare SMILES, and a line with two or more
//!    columns is `ID<sep>SMILES` (PubChem `CID-SMILES`).
//!
//! A row with no id (an unnamed `.smi` row, or `--id-col 0`) uses its 1-based
//! line number as its id. A row with no SMILES is an error that names the line:
//! it means the column layout is wrong.

use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use flate2::read::MultiGzDecoder;
use mossflux_substruct::{parse_smarts, SearchError, SearchLimits, SubstructIndex};

use crate::{fmt_int, CliError, OutputFormat};

/// Arguments for `mossflux substruct`.
#[derive(clap::Args)]
pub struct SubstructArgs {
    /// SMILES file (`.gz` is read transparently). See --smiles-col for the
    /// column rules.
    #[arg(short, long)]
    pub smiles: PathBuf,
    /// SMARTS query. Repeat the flag to run several queries over one build.
    #[arg(short = 'q', long = "smarts", required_unless_present = "smarts_file")]
    pub smarts: Vec<String>,
    /// File of SMARTS queries, one per line. Blank lines and lines that start
    /// with `#` are skipped. Combines with --smarts.
    #[arg(long)]
    pub smarts_file: Option<PathBuf>,
    /// 1-based column that holds the SMILES. Default: auto (Daylight `.smi`
    /// layout, a `smiles` header column, or column 2 of a multi-column line).
    #[arg(long)]
    pub smiles_col: Option<usize>,
    /// 1-based column that holds the molecule id; 0 uses the line number.
    /// Default: auto (the column that is not the SMILES).
    #[arg(long)]
    pub id_col: Option<usize>,
    /// Maximum hits printed per query (the hit count is always exact).
    #[arg(short, long, default_value_t = 20)]
    pub limit: usize,
    /// Print only the hit count of each query.
    #[arg(long, default_value_t = false)]
    pub count: bool,
    /// Read at most this many molecule rows from the file.
    #[arg(long)]
    pub max_mols: Option<usize>,
    /// Fail if any row's SMILES does not parse (default: skip it and report).
    #[arg(long, default_value_t = false)]
    pub strict: bool,
    /// Worker threads for the build and the verify stage (0 = all cores).
    #[arg(long, default_value_t = 0)]
    pub threads: usize,
    /// Give up on a query after this many seconds, with an error (never a
    /// partial hit list). Subgraph matching is NP-complete, so a short query can
    /// take exponential time. Default: no limit.
    #[arg(long)]
    pub timeout: Option<f64>,
    /// Output format for results on stdout. Diagnostics always go to stderr.
    #[arg(long, value_enum, default_value = "text")]
    pub format: OutputFormat,
}

/// One molecule row as read from the file.
struct Row {
    /// 1-based line number in the file, for error messages.
    line: usize,
    id: String,
    smiles: String,
}

/// Which columns hold the id and the SMILES (0-based; `None` id = line number).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Columns {
    smiles: usize,
    id: Option<usize>,
}

/// Splits a line on TAB when it has one, else on runs of spaces.
fn fields(line: &str) -> Vec<&str> {
    if line.contains('\t') {
        line.split('\t').collect()
    } else {
        line.split(' ').filter(|f| !f.is_empty()).collect()
    }
}

/// True for the Daylight SMILES file extensions, with or without `.gz`.
fn is_daylight_smi(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let name = name.strip_suffix(".gz").unwrap_or(&name);
    [".smi", ".smiles", ".ism", ".can"]
        .iter()
        .any(|ext| name.ends_with(ext))
}

/// Finds a `smiles` / `canonical_smiles` header column in `first`, if any.
fn header_smiles_col(first: &[&str]) -> Option<usize> {
    first.iter().position(|f| {
        let f = f.trim();
        f.eq_ignore_ascii_case("smiles") || f.eq_ignore_ascii_case("canonical_smiles")
    })
}

/// Resolves the column layout from the flags, the path and the first line.
/// Returns the layout and whether the first line is a header to skip.
fn resolve_columns(a: &SubstructArgs, first: &[&str]) -> Result<(Columns, bool), CliError> {
    let header = header_smiles_col(first);
    let to_zero = |c: usize, flag: &str| -> Result<usize, CliError> {
        c.checked_sub(1)
            .ok_or_else(|| CliError::from(format!("{flag} is 1-based; 0 is not a column")))
    };
    let smiles = match a.smiles_col {
        Some(c) => to_zero(c, "--smiles-col")?,
        None if is_daylight_smi(&a.smiles) => 0,
        None => match header {
            Some(c) => c,
            None if first.len() >= 2 => 1,
            None => 0,
        },
    };
    let id = match a.id_col {
        Some(0) => None,
        Some(c) => Some(c - 1),
        None if is_daylight_smi(&a.smiles) => Some(1),
        None => {
            if smiles == 0 {
                (first.len() >= 2).then_some(1)
            } else {
                Some(0)
            }
        }
    };
    if Some(smiles) == id {
        return Err("--smiles-col and --id-col name the same column".into());
    }
    Ok((Columns { smiles, id }, header.is_some()))
}

fn open_text(path: &Path) -> Result<Box<dyn BufRead>, CliError> {
    let file = std::fs::File::open(path).map_err(|e| format!("opening {path:?}: {e}"))?;
    let reader: Box<dyn Read> = if path.extension().is_some_and(|e| e == "gz") {
        Box::new(MultiGzDecoder::new(file))
    } else {
        Box::new(file)
    };
    Ok(Box::new(BufReader::with_capacity(1 << 20, reader)))
}

/// Reads the molecule rows. A line with no SMILES column is an error naming the
/// line: the column layout is wrong.
fn read_rows(a: &SubstructArgs) -> Result<Vec<Row>, CliError> {
    let path = &a.smiles;
    let max = a.max_mols.unwrap_or(usize::MAX);
    let mut rows = Vec::new();
    let mut layout: Option<Columns> = None;
    for (i, line) in open_text(path)?.lines().enumerate() {
        if rows.len() >= max {
            break;
        }
        let lineno = i + 1;
        let line = line.map_err(|e| format!("reading {path:?} line {lineno}: {e}"))?;
        let line = line.trim_end_matches(['\r', '\n']);
        if line.trim().is_empty() {
            continue;
        }
        let f = fields(line);
        let cols = match layout {
            Some(c) => c,
            None => {
                let (c, is_header) = resolve_columns(a, &f)?;
                layout = Some(c);
                if is_header {
                    continue;
                }
                c
            }
        };
        let smiles = f.get(cols.smiles).map(|s| s.trim()).unwrap_or("");
        if smiles.is_empty() {
            return Err(format!(
                "{path:?} line {lineno}: no SMILES in column {} (set --smiles-col)",
                cols.smiles + 1
            )
            .into());
        }
        // A row without an id takes its line number.
        let id = match cols.id.and_then(|c| f.get(c)).map(|s| s.trim()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => lineno.to_string(),
        };
        rows.push(Row {
            line: lineno,
            id,
            smiles: smiles.to_string(),
        });
    }
    if rows.is_empty() {
        return Err(format!("{path:?} holds no molecule rows").into());
    }
    Ok(rows)
}

/// Collects the queries from `--smarts` and `--smarts-file`, in that order.
fn read_queries(a: &SubstructArgs) -> Result<Vec<String>, CliError> {
    let mut queries = a.smarts.clone();
    if let Some(path) = &a.smarts_file {
        for (i, line) in open_text(path)?.lines().enumerate() {
            let line = line.map_err(|e| format!("reading {path:?} line {}: {e}", i + 1))?;
            let q = line.trim();
            if q.is_empty() || q.starts_with('#') {
                continue;
            }
            queries.push(q.to_string());
        }
    }
    if queries.is_empty() {
        return Err("no SMARTS queries given (use --smarts or --smarts-file)".into());
    }
    Ok(queries)
}

/// Entry point for `mossflux substruct`.
pub fn run(a: SubstructArgs, out: &mut impl Write) -> Result<(), CliError> {
    // Parse every query before the build, so a typo fails fast.
    let queries = read_queries(&a)?;
    for q in &queries {
        parse_smarts(q).map_err(|e| format!("SMARTS {q:?}: {e}"))?;
    }
    let timeout = match a.timeout {
        None => None,
        Some(t) if t.is_finite() && t > 0.0 => Some(std::time::Duration::from_secs_f64(t)),
        Some(t) => {
            return Err(format!("--timeout must be a positive number of seconds, got {t}").into())
        }
    };

    let pool = if a.threads == 0 {
        rayon::ThreadPoolBuilder::new().build()
    } else {
        rayon::ThreadPoolBuilder::new()
            .num_threads(a.threads)
            .build()
    }
    .map_err(|e| e.to_string())?;

    let t = Instant::now();
    let rows = read_rows(&a)?;
    eprintln!(
        "read {} rows from {:?} in {:.2}s",
        fmt_int(rows.len() as u64),
        a.smiles,
        t.elapsed().as_secs_f64()
    );

    let t = Instant::now();
    let smiles: Vec<&str> = rows.iter().map(|r| r.smiles.as_str()).collect();
    let (idx, skipped) = pool.install(|| SubstructIndex::from_smiles_with_errors(&smiles));
    drop(smiles);
    if let Some(first) = skipped.first() {
        let r = &rows[first.row];
        let detail = format!(
            "{} rows not indexed; first is line {} (id {}): {}",
            fmt_int(skipped.len() as u64),
            r.line,
            r.id,
            first.kind
        );
        if a.strict {
            return Err(format!("--strict: {detail}").into());
        }
        eprintln!("warning: {detail}");
    }
    // Index row i is the i-th input row that was not skipped.
    let mut kept: Vec<usize> = Vec::with_capacity(idx.len());
    let mut bad = skipped.iter().map(|e| e.row).peekable();
    for i in 0..rows.len() {
        if bad.peek() == Some(&i) {
            bad.next();
        } else {
            kept.push(i);
        }
    }
    debug_assert_eq!(kept.len(), idx.len());
    eprintln!(
        "indexed {} molecules in {:.2}s",
        fmt_int(idx.len() as u64),
        t.elapsed().as_secs_f64()
    );

    if a.format == OutputFormat::Tsv {
        if a.count {
            writeln!(out, "query\thits")?;
        } else {
            writeln!(out, "query\tid\tsmiles")?;
        }
    }
    for q in &queries {
        let t = Instant::now();
        let limits = timeout.map_or(SearchLimits::none(), SearchLimits::timeout);
        let hits = pool
            .install(|| idx.search_with_limits(q, limits))
            .map_err(|e| match e {
                SearchError::Timeout => format!(
                    "SMARTS {q:?}: no answer within --timeout {}s",
                    a.timeout.unwrap_or_default()
                ),
                e => format!("SMARTS {q:?}: {e}"),
            })?;
        let ms = t.elapsed().as_secs_f64() * 1e3;
        let shown = if a.count { 0 } else { hits.len().min(a.limit) };
        match a.format {
            OutputFormat::Text if a.count => {
                writeln!(
                    out,
                    "{} hits for {q} in {ms:.3} ms",
                    fmt_int(hits.len() as u64)
                )?;
            }
            OutputFormat::Text => {
                writeln!(
                    out,
                    "{} hits for {q} in {ms:.3} ms (showing {shown})",
                    fmt_int(hits.len() as u64)
                )?;
                for &h in &hits[..shown] {
                    let r = &rows[kept[h as usize]];
                    writeln!(out, "  {}  {}", r.id, r.smiles)?;
                }
            }
            OutputFormat::Tsv => {
                eprintln!("{} hits for {q} in {ms:.3} ms", fmt_int(hits.len() as u64));
                if a.count {
                    writeln!(out, "{q}\t{}", hits.len())?;
                }
                for &h in &hits[..shown] {
                    let r = &rows[kept[h as usize]];
                    writeln!(out, "{q}\t{}\t{}", r.id, r.smiles)?;
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(path: &str, smiles_col: Option<usize>, id_col: Option<usize>) -> SubstructArgs {
        SubstructArgs {
            smiles: PathBuf::from(path),
            smarts: vec!["C".into()],
            smarts_file: None,
            smiles_col,
            id_col,
            limit: 20,
            count: false,
            max_mols: None,
            strict: false,
            threads: 1,
            timeout: None,
            format: OutputFormat::Text,
        }
    }

    fn cols(smiles: usize, id: Option<usize>) -> Columns {
        Columns { smiles, id }
    }

    #[test]
    fn splits_on_tab_first_then_spaces() {
        assert_eq!(fields("a\tb c\td"), vec!["a", "b c", "d"]);
        assert_eq!(fields("CCO  ethanol"), vec!["CCO", "ethanol"]);
        assert_eq!(fields("CCO"), vec!["CCO"]);
    }

    #[test]
    fn chembl_header_selects_named_column() {
        let first = fields("chembl_id\tcanonical_smiles\tstandard_inchi\tstandard_inchi_key");
        let (c, header) = resolve_columns(&args("chembl.txt.gz", None, None), &first).unwrap();
        assert_eq!(c, cols(1, Some(0)));
        assert!(header);
    }

    #[test]
    fn daylight_extension_puts_smiles_first() {
        let first = fields("c1ccccc1 benzene");
        for p in ["x.smi", "x.SMI.gz", "x.smiles", "x.ism", "x.can"] {
            let (c, header) = resolve_columns(&args(p, None, None), &first).unwrap();
            assert_eq!(c, cols(0, Some(1)), "{p}");
            assert!(!header, "{p}");
        }
    }

    #[test]
    fn headerless_layouts() {
        let a = args("x.tsv", None, None);
        assert_eq!(
            resolve_columns(&a, &fields("123\tCCO")).unwrap(),
            (cols(1, Some(0)), false)
        );
        assert_eq!(
            resolve_columns(&a, &fields("CCO")).unwrap(),
            (cols(0, None), false)
        );
    }

    #[test]
    fn explicit_columns_override_and_validate() {
        let first = fields("CCO\tx\t42");
        let (c, _) = resolve_columns(&args("x.tsv", Some(1), Some(3)), &first).unwrap();
        assert_eq!(c, cols(0, Some(2)));
        let (c, _) = resolve_columns(&args("x.tsv", Some(1), Some(0)), &first).unwrap();
        assert_eq!(c, cols(0, None), "--id-col 0 means line numbers");
        assert!(resolve_columns(&args("x.tsv", Some(0), None), &first).is_err());
        assert!(resolve_columns(&args("x.tsv", Some(2), Some(2)), &first).is_err());
    }
}
