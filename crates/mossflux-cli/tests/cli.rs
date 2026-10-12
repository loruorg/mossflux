//! End-to-end tests for the `mossflux` binary.
//!
//! Runs the real binary (`CARGO_BIN_EXE_mossflux`) and asserts on exact stdout,
//! exit status, and the subcommand list.
//!
//! Every expected number here is derived by hand from the input, not read back
//! from the program.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_mossflux")
}

fn run(args: &[&str]) -> Output {
    Command::new(bin())
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run {} {args:?}: {e}", bin()))
}

fn stdout_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}
fn stderr_of(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

static SEQ: AtomicU64 = AtomicU64::new(0);

/// A scratch file on a path unique to this process and this call, removed on
/// drop (including on a panic), so concurrent test runs cannot race on it.
struct TempFile(PathBuf);

impl TempFile {
    /// A scratch `.fps` file.
    fn new(tag: &str, contents: &str) -> Self {
        Self::with_ext(tag, "fps", contents)
    }

    /// A scratch file with the given extension (the `substruct` column rules
    /// depend on it).
    fn with_ext(tag: &str, ext: &str, contents: &str) -> Self {
        let seq = SEQ.fetch_add(1, Ordering::Relaxed);
        let mut p = std::env::temp_dir();
        p.push(format!(
            "mossflux_cli_{tag}_{}_{seq}.{ext}",
            std::process::id()
        ));
        std::fs::File::create(&p)
            .unwrap()
            .write_all(contents.as_bytes())
            .unwrap();
        Self(p)
    }
    fn str(&self) -> &str {
        self.0.to_str().unwrap()
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

// ---------------------------------------------------------------------------
// tanimoto
// ---------------------------------------------------------------------------

/// Hand-derivation of every case below. `parse_hex_fp` reads two hex chars per
/// byte, left to right, and packs byte `i` into bits `8i..8i+7` of a little-endian
/// `u64` lane. So `"0f0f"` -> bytes `[0x0f, 0x0f]` -> lane `0x0f0f`, and `"0ff0"`
/// -> bytes `[0x0f, 0xf0]` -> lane `0xf00f`.
#[test]
fn tanimoto_prints_exact_six_decimal_similarity() {
    // 0x0f0f & 0xf00f = 0x000f (4 bits); 0x0f0f | 0xf00f = 0xff0f (12 bits).
    // 4/12 = 0.333333... -> "{:.6}" -> "0.333333".
    let out = run(&["tanimoto", "0f0f", "0ff0"]);
    assert!(
        out.status.success(),
        "expected success, got {:?}",
        out.status
    );
    assert_eq!(
        stdout_of(&out),
        "0.333333\n",
        "exact stdout is part of the contract"
    );
    assert_eq!(stderr_of(&out), "", "nothing should be written to stderr");

    // Identical fingerprints: c == u, so exactly 1.
    let out = run(&["tanimoto", "0f0f", "0f0f"]);
    assert!(out.status.success());
    assert_eq!(stdout_of(&out), "1.000000\n");

    // Disjoint: 0x000f & 0xf000 == 0.
    let out = run(&["tanimoto", "0f00", "00f0"]);
    assert!(out.status.success());
    assert_eq!(stdout_of(&out), "0.000000\n");

    // Two all-zero fingerprints: the 0/0 case is defined as 0.0, not NaN.
    let out = run(&["tanimoto", "0000", "0000"]);
    assert!(out.status.success());
    assert_eq!(stdout_of(&out), "0.000000\n", "0/0 must be 0.0, never NaN");

    // A hand-checkable non-trivial value: bytes [0xff,0x00] -> 0x00ff (8 bits)
    // against [0x0f,0x00] -> 0x000f (4 bits). Intersection 4, union 8 -> 0.5.
    let out = run(&["tanimoto", "ff00", "0f00"]);
    assert!(out.status.success());
    assert_eq!(stdout_of(&out), "0.500000\n");
}

#[test]
fn tanimoto_rejects_bad_input_without_printing_a_number() {
    // Odd-length hex.
    let out = run(&["tanimoto", "abc", "0f0f"]);
    assert!(!out.status.success(), "odd-length hex must fail");
    assert_eq!(stdout_of(&out), "", "no number may be printed on failure");
    assert!(
        stderr_of(&out).contains("odd length"),
        "stderr should explain the failure, got {:?}",
        stderr_of(&out)
    );

    // Non-hex digits.
    let out = run(&["tanimoto", "zz", "0f0f"]);
    assert!(!out.status.success(), "non-hex must fail");
    assert_eq!(stdout_of(&out), "");
    assert!(
        stderr_of(&out).contains("invalid hex"),
        "got {:?}",
        stderr_of(&out)
    );

    // Mismatched widths: 8 bytes vs 9 bytes. (A same-lane-count mismatch such
    // as `ff` vs `ff00` is checked in `bad_arguments_are_errors_not_panics`.)
    let out = run(&["tanimoto", "ffffffffffffffff", "ffffffffffffffffff"]);
    assert!(!out.status.success(), "differing lane counts must fail");
    assert_eq!(stdout_of(&out), "");
    assert!(
        stderr_of(&out).contains("differ in width"),
        "got {:?}",
        stderr_of(&out)
    );

    // Non-ASCII must be rejected before any byte-slicing.
    let out = run(&["tanimoto", "0f0é", "0f0f"]);
    assert!(!out.status.success(), "non-ASCII must fail, not panic");
    assert_eq!(stdout_of(&out), "");

    // A missing operand is a clap usage error, not a panic.
    let out = run(&["tanimoto", "0f0f"]);
    assert!(!out.status.success());
    assert!(
        !stderr_of(&out).contains("panicked"),
        "clap error, not a panic"
    );
}

// ---------------------------------------------------------------------------
// the subcommand surface
// ---------------------------------------------------------------------------

/// Pull the subcommand names out of `--help` by taking the first token of each
/// line in the `Commands:` block. Substring-searching the whole help text would
/// be wrong: `substruct-bench`'s description itself contains the word "build".
fn subcommands_from_help(help: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_block = false;
    for line in help.lines() {
        if line.trim_end() == "Commands:" {
            in_block = true;
            continue;
        }
        if in_block {
            if line.trim().is_empty() {
                break;
            }
            if let Some(first) = line.split_whitespace().next() {
                names.push(first.to_string());
            }
        }
    }
    names
}

#[test]
fn help_lists_exactly_the_real_subcommands() {
    let out = run(&["--help"]);
    assert!(out.status.success(), "--help must exit 0");
    let help = stdout_of(&out);
    let mut names = subcommands_from_help(&help);
    names.sort();
    assert_eq!(
        names,
        vec![
            "bench".to_string(),
            "help".to_string(),
            "info".to_string(),
            "mcp".to_string(),
            "search".to_string(),
            "serve".to_string(),
            "substruct".to_string(),
            "substruct-bench".to_string(),
            "tanimoto".to_string(),
        ],
        "the subcommand set changed; full help was:\n{help}"
    );
    // `build` and `add` do not exist; pin their absence.
    for phantom in ["build", "add"] {
        assert!(
            !names.iter().any(|n| n == phantom),
            "a subcommand named {phantom:?} appeared; README once claimed it exists"
        );
        let out = run(&[phantom]);
        assert!(
            !out.status.success(),
            "invoking the non-existent {phantom:?} subcommand must fail"
        );
    }
}

#[test]
fn no_args_prints_usage_and_fails() {
    let out = run(&[]);
    assert!(!out.status.success(), "a bare invocation must not exit 0");
    let text = format!("{}{}", stdout_of(&out), stderr_of(&out));
    assert!(
        text.contains("Usage: mossflux"),
        "usage must be shown, got {text:?}"
    );
    assert!(!text.contains("panicked"));
}

#[test]
fn version_flag_reports_the_crate_version() {
    let out = run(&["--version"]);
    assert!(out.status.success());
    assert_eq!(
        stdout_of(&out).trim(),
        format!("mossflux {}", env!("CARGO_PKG_VERSION")),
        "--version must report the packaged version"
    );
}

// ---------------------------------------------------------------------------
// info / search over a real FPS file, with hand-computed expectations
// ---------------------------------------------------------------------------

/// Four 16-bit fingerprints with hand-picked popcounts 1, 4, 8 and 15.
///
/// | hex    | bytes        | lane   | popcount |
/// |--------|--------------|--------|----------|
/// | `0100` | 0x01, 0x00   | 0x0001 |  1       |
/// | `0f00` | 0x0f, 0x00   | 0x000f |  4       |
/// | `0f0f` | 0x0f, 0x0f   | 0x0f0f |  8       |
/// | `7fff` | 0x7f, 0xff   | 0xff7f | 15       |
const FPS: &str = "#num_bits=16\n0f0f\tB\n0100\tA\n7fff\tD\n0f00\tC\n";

#[test]
fn info_reports_hand_computed_statistics() {
    let f = TempFile::new("info", FPS);
    let out = run(&["info", "-f", f.str()]);
    assert!(out.status.success(), "info failed: {}", stderr_of(&out));
    let text = stdout_of(&out);
    // popcounts sorted ascending are 1, 4, 8, 15: median is row n/2 == row 2 == 8,
    // mean is 28/4 == 7.0, min 1, max 15. Memory is 4 rows x 1 lane x 8 B = 32 B.
    for want in [
        "fingerprints : 4",
        "width        : 16 bits (1 u64 lanes)",
        "memory        : 0.0 MiB (fingerprints)",
        "popcount      : min 1, median 8, mean 7.0, max 15",
    ] {
        assert!(text.contains(want), "info stdout missing {want:?}:\n{text}");
    }
}

#[test]
fn info_on_a_missing_file_fails_cleanly() {
    let out = run(&["info", "-f", "/definitely/not/a/real/path.fps"]);
    assert!(!out.status.success(), "a missing file must be an error");
    assert!(
        stderr_of(&out).contains("error:"),
        "got {:?}",
        stderr_of(&out)
    );
    assert!(!stderr_of(&out).contains("panicked"), "must not panic");
}

#[test]
fn search_returns_the_hand_computed_hit_set() {
    let f = TempFile::new("search", FPS);
    // Query 0x0f0f (row "B", popcount 8) at t = 0.5. Intersections, by hand:
    //   vs A 0x0001: c=1,  u=8  -> 0.1250
    //   vs C 0x000f: c=4,  u=8  -> 0.5000   (exactly AT the threshold: INCLUDED)
    //   vs B 0x0f0f: c=8,  u=8  -> 1.0000
    //   vs D 0xff7f: 0x0f0f & 0xff7f = 0x0f0f, so c=8, u=15 -> 8/15 = 0.5333
    // So t=0.5 must return B, D and C, printed score-descending.
    let out = run(&["search", "-f", f.str(), "--query", "0f0f", "-t", "0.5"]);
    assert!(out.status.success(), "search failed: {}", stderr_of(&out));
    let text = stdout_of(&out);
    let lines: Vec<&str> = text.lines().collect();
    assert!(
        lines[0].starts_with("3 hits in"),
        "expected 3 hits, got {:?}",
        lines[0]
    );
    assert!(lines[0].ends_with("(showing 3)"), "got {:?}", lines[0]);
    assert_eq!(lines[1], "  1.0000  B", "best hit must be the query itself");
    assert_eq!(
        lines[2], "  0.5333  D",
        "8/15 sorts above the 0.5 boundary hit"
    );
    assert_eq!(
        lines[3], "  0.5000  C",
        "a score exactly == t must be returned"
    );
    assert_eq!(lines.len(), 4, "no extra hits: {text}");

    // Just above 0.5 drops C (whose true score is exactly 0.5) and keeps D.
    let out = run(&["search", "-f", f.str(), "--query", "0f0f", "-t", "0.51"]);
    assert!(out.status.success());
    let text = stdout_of(&out);
    let lines: Vec<&str> = text.lines().collect();
    assert!(lines[0].starts_with("2 hits in"), "got {:?}", lines[0]);
    assert_eq!(lines[1], "  1.0000  B");
    assert_eq!(lines[2], "  0.5333  D");
    assert_eq!(lines.len(), 3);

    // Above 8/15 leaves only the exact self-match.
    let out = run(&["search", "-f", f.str(), "--query", "0f0f", "-t", "0.6"]);
    assert!(out.status.success());
    let text = stdout_of(&out);
    let lines: Vec<&str> = text.lines().collect();
    assert!(lines[0].starts_with("1 hits in"), "got {:?}", lines[0]);
    assert_eq!(lines[1], "  1.0000  B");
    assert_eq!(lines.len(), 2);

    // kNN over the same file: all four rows, score-descending.
    // `--query-index` counts records in FILE order, so index 0 is "B" (0x0f0f),
    // not the lowest-popcount row. Against B, by the table above:
    //   B 1.0000, D 8/15 = 0.5333, C 4/8 = 0.5000, A 1/8 = 0.1250.
    let out = run(&["search", "-f", f.str(), "--query-index", "0", "-k", "4"]);
    assert!(out.status.success(), "knn failed: {}", stderr_of(&out));
    let text = stdout_of(&out);
    let lines: Vec<&str> = text.lines().collect();
    assert!(lines[0].starts_with("4 hits in"), "got {:?}", lines[0]);
    assert_eq!(lines[1], "  1.0000  B");
    assert_eq!(lines[2], "  0.5333  D");
    assert_eq!(lines[3], "  0.5000  C");
    assert_eq!(lines[4], "  0.1250  A");

    // Record 1 is "A" (0x0001): A 1/1, C 1/4, B 1/8, D 1/15.
    let out = run(&["search", "-f", f.str(), "--query-index", "1", "-k", "4"]);
    assert!(out.status.success(), "knn failed: {}", stderr_of(&out));
    let text = stdout_of(&out);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        lines[1..],
        ["  1.0000  A", "  0.2500  C", "  0.1250  B", "  0.0667  D"]
    );
}

#[test]
fn search_rejects_a_mismatched_query_width() {
    let f = TempFile::new("width", FPS);
    // The file is 16 bits (1 lane); a 9-byte query is 2 lanes. Silently padding or
    // truncating would search a different fingerprint than the user asked for.
    let out = run(&["search", "-f", f.str(), "--query", "ffffffffffffffffff"]);
    assert!(!out.status.success(), "a width mismatch must be an error");
    assert!(
        stderr_of(&out).contains("query has 18 hex characters, but the 16-bit index needs 4"),
        "error should name the widths, got {:?}",
        stderr_of(&out)
    );
    // Same lane count, different width: `ff` is an 8-bit fingerprint, and
    // padding it to 16 bits would search a different fingerprint.
    let out = run(&["search", "-f", f.str(), "--query", "ff"]);
    assert!(
        !out.status.success(),
        "an 8-bit query on a 16-bit file must fail"
    );
    assert!(
        stderr_of(&out).contains("needs 4"),
        "got {:?}",
        stderr_of(&out)
    );
    // Two query selectors at once are a usage error, not "first one wins".
    let out = run(&[
        "search",
        "-f",
        f.str(),
        "--query",
        "0f0f",
        "--query-index",
        "0",
    ]);
    assert!(
        !out.status.success(),
        "two query selectors must be rejected"
    );
    assert!(
        stderr_of(&out).contains("cannot be used with"),
        "got {:?}",
        stderr_of(&out)
    );

    // No query selector at all is also an error, not a default.
    let out = run(&["search", "-f", f.str()]);
    assert!(!out.status.success());
    assert!(
        stderr_of(&out).contains("provide"),
        "got {:?}",
        stderr_of(&out)
    );

    // An out-of-range row index is an error, not a panic.
    let out = run(&["search", "-f", f.str(), "--query-index", "99"]);
    assert!(!out.status.success());
    assert!(
        stderr_of(&out).contains("out of range"),
        "got {:?}",
        stderr_of(&out)
    );
    assert!(!stderr_of(&out).contains("panicked"));
}

#[test]
fn search_tsv_prints_exact_scores_with_a_header() {
    let f = TempFile::new("tsv", FPS);
    // Same hand-computed hits as above, as TSV: exact f64 scores that parse
    // back to the same value (8/15 = 0.5333333333333333 in f64 Display).
    let out = run(&[
        "search",
        "-f",
        f.str(),
        "--query",
        "0f0f",
        "-t",
        "0.5",
        "--format",
        "tsv",
    ]);
    assert!(out.status.success(), "search failed: {}", stderr_of(&out));
    assert_eq!(
        stdout_of(&out),
        "id\tscore\nB\t1\nD\t0.5333333333333333\nC\t0.5\n",
        "TSV stdout must hold only the header and the hits"
    );
    assert!(
        stderr_of(&out).contains("3 hits in"),
        "the summary moves to stderr in TSV mode, got {:?}",
        stderr_of(&out)
    );
}

#[test]
fn a_closed_stdout_pipe_ends_quietly() {
    use std::io::{BufRead, BufReader};
    use std::process::Stdio;
    // 20,000 rows at t = 0 print far more than any pipe buffer holds, so the
    // child is still writing when the reader goes away. It must exit 0, not
    // panic with "failed printing to stdout".
    let mut fps = String::from("#num_bits=16\n");
    for i in 0..20_000 {
        fps.push_str(&format!("{:04x}\tM{i}\n", (i % 65_535) + 1));
    }
    let f = TempFile::new("pipe", &fps);
    let mut child = Command::new(bin())
        .args([
            "search",
            "-f",
            f.str(),
            "--query",
            "ffff",
            "-t",
            "0",
            "-l",
            "20000",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn mossflux");
    let mut first = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut first)
        .unwrap();
    assert!(first.starts_with("20,000 hits in"), "got {first:?}");
    // The reader (and with it the pipe) is dropped here.
    let out = child.wait_with_output().unwrap();
    let err = stderr_of(&out);
    assert!(!err.contains("panicked"), "must not panic, stderr: {err}");
    assert!(
        out.status.success(),
        "a closed pipe is a normal end, stderr: {err}"
    );
}

// ---------------------------------------------------------------------------
// substruct — exact SMARTS search over a SMILES file
// ---------------------------------------------------------------------------

/// A ChEMBL-style file: a header row, an id column, the SMILES column, and an
/// extra column. Line 5 (`C1CC`) has an unclosed ring and does not parse.
const CHEMREPS: &str = "chembl_id\tcanonical_smiles\tinchi\n\
                        M1\tOc1ccccc1\tx\n\
                        M2\tCCO\tx\n\
                        M3\tCC(=O)O\tx\n\
                        BAD\tC1CC\tx\n\
                        M4\tc1ccncc1\tx\n";

#[test]
fn substruct_returns_the_hand_checked_hit_set() {
    let f = TempFile::with_ext("chemreps", "txt", CHEMREPS);
    // By hand: an aliphatic O is in phenol (M1), ethanol (M2) and acetic acid
    // (M3), not in pyridine (M4). A benzene ring is only in M1: pyridine has an
    // aromatic n in its ring. C=O is only in M3.
    let out = run(&[
        "substruct",
        "-s",
        f.str(),
        "-q",
        "O",
        "-q",
        "c1ccccc1",
        "-q",
        "C=O",
        "--format",
        "tsv",
    ]);
    assert!(
        out.status.success(),
        "substruct failed: {}",
        stderr_of(&out)
    );
    assert_eq!(
        stdout_of(&out),
        "query\tid\tsmiles\n\
         O\tM1\tOc1ccccc1\n\
         O\tM2\tCCO\n\
         O\tM3\tCC(=O)O\n\
         c1ccccc1\tM1\tOc1ccccc1\n\
         C=O\tM3\tCC(=O)O\n"
    );
    // The bad row is skipped and reported by its file line, never silently.
    let err = stderr_of(&out);
    assert!(
        err.contains("1 rows not indexed; first is line 5 (id BAD)"),
        "got {err:?}"
    );
    assert!(err.contains("indexed 4 molecules"), "got {err:?}");

    // Text mode with --count prints only the exact counts.
    let out = run(&["substruct", "-s", f.str(), "-q", "O", "--count"]);
    assert!(out.status.success());
    assert!(
        stdout_of(&out).starts_with("3 hits for O in "),
        "got {:?}",
        stdout_of(&out)
    );
}

#[test]
fn substruct_strict_fails_on_a_bad_row() {
    let f = TempFile::with_ext("strict", "txt", CHEMREPS);
    let out = run(&["substruct", "-s", f.str(), "-q", "O", "--strict"]);
    assert!(!out.status.success(), "--strict must fail on the bad row");
    assert!(stdout_of(&out).is_empty(), "no partial result on failure");
    let err = stderr_of(&out);
    assert!(
        err.contains("error: --strict: 1 rows not indexed"),
        "got {err:?}"
    );
    assert!(!err.contains("panicked"));
}

#[test]
fn substruct_reads_daylight_smi_layout() {
    // `.smi` puts the SMILES first and the optional name second, space
    // separated. The unnamed third row takes its line number as its id.
    let f = TempFile::with_ext("daylight", "smi", "c1ccccc1 benzene\nCCO ethanol\nCCN\n");
    let out = run(&["substruct", "-s", f.str(), "-q", "C", "--format", "tsv"]);
    assert!(
        out.status.success(),
        "substruct failed: {}",
        stderr_of(&out)
    );
    // Aliphatic C: ethanol and ethylamine, not benzene.
    assert_eq!(
        stdout_of(&out),
        "query\tid\tsmiles\nC\tethanol\tCCO\nC\t3\tCCN\n"
    );
}

#[test]
fn substruct_rejects_bad_input_cleanly() {
    let f = TempFile::with_ext("badq", "txt", CHEMREPS);
    // A SMARTS error is reported with its byte position before any build.
    let out = run(&["substruct", "-s", f.str(), "-q", "C(("]);
    assert!(!out.status.success());
    let err = stderr_of(&out);
    assert!(err.contains("error: SMARTS \"C((\""), "got {err:?}");
    assert!(
        !err.contains("read "),
        "must fail before reading the file: {err:?}"
    );

    // A missing file, a 0 column and a missing query are errors, not panics.
    for args in [
        vec!["substruct", "-s", "/definitely/not/here.smi", "-q", "C"],
        vec!["substruct", "-s", f.str(), "-q", "C", "--smiles-col", "0"],
        vec!["substruct", "-s", f.str()],
    ] {
        let out = run(&args);
        assert!(!out.status.success(), "{args:?} must fail");
        assert!(!stderr_of(&out).contains("panicked"), "{args:?} panicked");
    }
}

#[test]
fn search_rejects_query_bits_past_the_width() {
    // A 12-bit file. Hex `ffff` is the right length (2 bytes) but sets bits
    // 12..15, which a 12-bit fingerprint does not have.
    let f = TempFile::new("w12", "#num_bits=12\nff0f\tFULL\n0100\tONE\n");
    let out = run(&["search", "-f", f.str(), "--query", "ffff", "-t", "0.8"]);
    assert!(
        !out.status.success(),
        "spare bits must be an error, not ignored"
    );
    assert!(
        stderr_of(&out).contains("outside the 12-bit index"),
        "got {:?}",
        stderr_of(&out)
    );
    // The in-width spelling finds the full row at score 1.
    let out = run(&["search", "-f", f.str(), "--query", "ff0f", "-t", "0.8"]);
    assert!(out.status.success(), "got {:?}", stderr_of(&out));
    assert_eq!(stdout_of(&out).lines().nth(1), Some("  1.0000  FULL"));
}

#[test]
fn zero_popcount_rows_are_loaded_with_a_warning() {
    let f = TempFile::new("zero", "#num_bits=16\n0000\tEMPTY\n0f00\tC\n");
    let out = run(&["info", "-f", f.str()]);
    assert!(out.status.success(), "got {:?}", stderr_of(&out));
    assert!(stdout_of(&out).contains("fingerprints : 2"));
    assert!(
        stderr_of(&out).contains("warning: 1 fingerprints have no bits set"),
        "got {:?}",
        stderr_of(&out)
    );
}

#[test]
fn bad_arguments_are_errors_not_panics() {
    let f = TempFile::new("args", FPS);
    for (args, want) in [
        (
            vec!["bench", "-f", f.str(), "--fold", "0"],
            "--fold must be in 1..=16",
        ),
        (
            vec!["bench", "-f", f.str(), "--fold", "300"],
            "--fold must be in 1..=16",
        ),
        (
            vec!["bench", "-f", f.str(), "--queries", "0"],
            "--queries must be > 0",
        ),
        (
            vec!["bench", "-f", f.str(), "--repeat", "0"],
            "--repeat must be > 0",
        ),
        (
            vec!["bench", "--synth-n", "10", "--synth-nbits", "70000"],
            "--synth-nbits must be in",
        ),
        (
            vec!["bench", "--synth-n", "18446744073709551615"],
            "--synth-n is too large",
        ),
        (vec!["tanimoto", "", ""], "empty fingerprint"),
        (
            vec!["tanimoto", "ff", "ff00"],
            "differ in width (8 vs 16 bits)",
        ),
    ] {
        let out = run(&args);
        let err = stderr_of(&out);
        assert!(!out.status.success(), "{args:?} must fail");
        assert!(err.contains(want), "{args:?}: want {want:?}, got {err:?}");
        assert!(!err.contains("panicked"), "{args:?} panicked: {err}");
    }
    // A huge k is the whole file, not an allocation panic.
    let out = run(&[
        "search",
        "-f",
        f.str(),
        "--query-index",
        "0",
        "-k",
        "18446744073709551615",
    ]);
    assert!(out.status.success(), "got {:?}", stderr_of(&out));
    assert!(stdout_of(&out).starts_with("4 hits in"));
}

#[test]
fn substruct_timeout_is_an_error_not_a_partial_answer() {
    // Each `[!#6]` embeds alone (the one O) but two never do, after every way
    // of placing the eight `*`: exponential without a deadline.
    let rows: String = (0..3)
        .map(|i| format!("{}O m{i}\n", "C".repeat(29)))
        .collect();
    let f = TempFile::with_ext("timeout", "smi", &rows);
    let out = run(&[
        "substruct",
        "-s",
        f.str(),
        "-q",
        "*.*.*.*.*.*.*.*.[!#6].[!#6]",
        "--timeout",
        "0.2",
    ]);
    assert!(!out.status.success(), "a timed-out query must fail");
    let err = stderr_of(&out);
    assert!(
        err.contains("no answer within --timeout 0.2s"),
        "got {err:?}"
    );
    assert!(
        !stdout_of(&out).contains("hits for"),
        "no result line for it"
    );
    for bad in ["0", "-1", "nan"] {
        let out = run(&["substruct", "-s", f.str(), "-q", "C", "--timeout", bad]);
        assert!(!out.status.success(), "--timeout {bad} must be refused");
    }
}
