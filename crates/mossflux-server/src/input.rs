//! Reads `(id, SMILES)` rows from a text file.
//!
//! A line is split on TAB when it has one, otherwise on runs of spaces. Without
//! explicit columns:
//!
//! 1. A `.smi`, `.smiles`, `.ism` or `.can` file (optionally `.gz`) is
//!    `SMILES name`.
//! 2. A first line with a `smiles` or `canonical_smiles` column is a header; the
//!    first other column is the id (the ChEMBL chemreps layout).
//! 3. Otherwise one column is a bare SMILES and two or more are `ID SMILES`.
//!
//! A row without an id takes its 1-based line number.

use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

use flate2::read::MultiGzDecoder;

/// Column layout, 1-based. `None` means automatic; an id column of 0 means
/// "use the line number".
#[derive(Clone, Copy, Debug, Default)]
pub struct Columns {
    /// Column of the SMILES.
    pub smiles: Option<usize>,
    /// Column of the id.
    pub id: Option<usize>,
}

/// One molecule row.
#[derive(Clone, Debug)]
pub struct Row {
    /// 1-based line number.
    pub line: usize,
    /// Molecule id.
    pub id: String,
    /// SMILES text.
    pub smiles: String,
}

fn fields(line: &str) -> Vec<&str> {
    if line.contains('\t') {
        line.split('\t').collect()
    } else {
        line.split(' ').filter(|f| !f.is_empty()).collect()
    }
}

fn is_daylight(path: &Path) -> bool {
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

fn invalid(msg: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, msg)
}

/// Resolves 0-based `(smiles, id)` columns and whether `first` is a header.
fn resolve(
    path: &Path,
    cols: Columns,
    first: &[&str],
) -> io::Result<((usize, Option<usize>), bool)> {
    let header = first.iter().position(|f| {
        let f = f.trim();
        f.eq_ignore_ascii_case("smiles") || f.eq_ignore_ascii_case("canonical_smiles")
    });
    let daylight = is_daylight(path);
    let smiles = match cols.smiles {
        Some(0) => {
            return Err(invalid(
                "the SMILES column is 1-based; 0 is not a column".into(),
            ))
        }
        Some(c) => c - 1,
        None if daylight => 0,
        None => header.unwrap_or(usize::from(first.len() >= 2)),
    };
    let id = match cols.id {
        Some(0) => None,
        Some(c) => Some(c - 1),
        None if daylight => Some(1),
        None if smiles == 0 => (first.len() >= 2).then_some(1),
        None => Some(0),
    };
    if Some(smiles) == id {
        return Err(invalid("the SMILES and id columns are the same".into()));
    }
    Ok(((smiles, id), header.is_some()))
}

/// Opens a text file, decompressing `.gz`.
pub fn open_text(path: &Path) -> io::Result<Box<dyn BufRead>> {
    let file = std::fs::File::open(path)?;
    let reader: Box<dyn Read> = if path.extension().is_some_and(|e| e == "gz") {
        Box::new(MultiGzDecoder::new(file))
    } else {
        Box::new(file)
    };
    Ok(Box::new(BufReader::with_capacity(1 << 20, reader)))
}

/// Reads up to `max` rows. A line with no SMILES in the SMILES column is an
/// error: the layout is wrong, and skipping would search a different corpus.
///
/// # Errors
/// I/O errors, a bad column layout, or a file with no rows.
pub fn read_rows(path: &Path, cols: Columns, max: Option<usize>) -> io::Result<Vec<Row>> {
    let max = max.unwrap_or(usize::MAX);
    let mut rows = Vec::new();
    let mut layout = None;
    for (i, line) in open_text(path)?.lines().enumerate() {
        if rows.len() >= max {
            break;
        }
        let lineno = i + 1;
        let line = line?;
        let line = line.trim_end_matches(['\r', '\n']);
        if line.trim().is_empty() {
            continue;
        }
        let f = fields(line);
        let (s_col, id_col) = match layout {
            Some(l) => l,
            None => {
                let (l, is_header) = resolve(path, cols, &f)?;
                layout = Some(l);
                if is_header {
                    continue;
                }
                l
            }
        };
        let smiles = f.get(s_col).map_or("", |s| s.trim());
        if smiles.is_empty() {
            return Err(invalid(format!(
                "{} line {lineno}: no SMILES in column {}",
                path.display(),
                s_col + 1
            )));
        }
        let id = match id_col.and_then(|c| f.get(c)).map(|s| s.trim()) {
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
        return Err(invalid(format!(
            "{} holds no molecule rows",
            path.display()
        )));
    }
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout(path: &str, cols: Columns, line: &str) -> ((usize, Option<usize>), bool) {
        resolve(Path::new(path), cols, &fields(line)).unwrap()
    }

    #[test]
    fn layouts() {
        let auto = Columns::default();
        assert_eq!(
            layout("x.tsv", auto, "chembl_id\tcanonical_smiles\tinchi"),
            ((1, Some(0)), true)
        );
        assert_eq!(layout("x.smi", auto, "CCO ethanol"), ((0, Some(1)), false));
        assert_eq!(layout("x.smi.gz", auto, "CCO"), ((0, Some(1)), false));
        assert_eq!(layout("x.txt", auto, "CCO"), ((0, None), false));
        assert_eq!(layout("x.txt", auto, "1\tCCO"), ((1, Some(0)), false));
        let explicit = Columns {
            smiles: Some(3),
            id: Some(0),
        };
        assert_eq!(layout("x.txt", explicit, "a b CCO"), ((2, None), false));
        assert!(resolve(
            Path::new("x"),
            Columns {
                smiles: Some(1),
                id: Some(1)
            },
            &["a"]
        )
        .is_err());
    }
}
