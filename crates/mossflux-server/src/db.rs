//! One searchable database: SMILES rows indexed for both engines.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use mossflux_index::Arena;
use mossflux_substruct::fingerprint::{self, DEFAULT_BITS};
use mossflux_substruct::{
    parse_smiles, props, MolGraph, SearchError, SearchLimits, SubstructIndex,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::input::{self, Columns};
use crate::{ApiError, ApiResult};

/// Where a database comes from.
#[derive(Clone, Debug)]
pub struct DbSpec {
    /// Name used in requests.
    pub name: String,
    /// SMILES file (`.gz` allowed); see [`input`] for the column rules.
    pub path: PathBuf,
    /// Column layout.
    pub columns: Columns,
    /// Read at most this many rows.
    pub max_mols: Option<usize>,
}

impl DbSpec {
    /// Parses `name=path` or a bare `path` (the name is then the file stem).
    #[must_use]
    pub fn parse(arg: &str) -> DbSpec {
        let (name, path) = match arg.split_once('=') {
            Some((n, p)) if !n.is_empty() && !n.contains('/') => (n.to_string(), PathBuf::from(p)),
            _ => {
                let path = PathBuf::from(arg);
                let stem = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("db")
                    .split('.')
                    .next()
                    .unwrap_or("db")
                    .to_string();
                (stem, path)
            }
        };
        DbSpec {
            name,
            path,
            columns: Columns::default(),
            max_mols: None,
        }
    }
}

/// Database summary.
#[derive(Clone, Debug, Serialize)]
pub struct DbInfo {
    /// Name used in requests.
    pub name: String,
    /// Indexed molecules.
    pub molecules: u64,
    /// Rows skipped because their SMILES did not parse or was too large.
    pub skipped: u64,
    /// Similarity fingerprint.
    pub fingerprint: FingerprintInfo,
    /// Build time in milliseconds.
    pub build_ms: u64,
    /// Approximate index memory in bytes.
    pub memory_bytes: u64,
    /// Always true for a loaded database.
    pub ready: bool,
}

/// The fingerprint used for similarity search.
#[derive(Clone, Debug, Serialize)]
pub struct FingerprintInfo {
    /// `ecfp4`: mossflux's own circular fingerprint, radius 2.
    pub kind: &'static str,
    /// Width in bits.
    pub bits: u32,
}

/// A similarity request.
#[derive(Clone, Debug, Deserialize)]
pub struct SimilarityRequest {
    /// Database name; the first database when absent.
    #[serde(default)]
    pub db: Option<String>,
    /// Query molecule.
    pub smiles: String,
    /// Tanimoto threshold in `(0, 1]`. Default 0.7. Ignored when `k` is set.
    #[serde(default)]
    pub threshold: Option<f64>,
    /// Return the `k` nearest molecules instead of a threshold search.
    #[serde(default)]
    pub k: Option<usize>,
    /// Maximum hits returned; `count` is always exact.
    #[serde(default)]
    pub limit: Option<usize>,
}

/// A substructure request.
#[derive(Clone, Debug, Deserialize)]
pub struct SubstructureRequest {
    /// Database name; the first database when absent.
    #[serde(default)]
    pub db: Option<String>,
    /// SMARTS (a SMILES is also a valid SMARTS).
    pub smarts: String,
    /// Maximum hits returned; `count` is always exact.
    #[serde(default)]
    pub limit: Option<usize>,
    /// Skip this many hits (in row order) before returning `limit`.
    #[serde(default)]
    pub offset: Option<usize>,
    /// Deadline in milliseconds, capped by the server.
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

/// A molecule request.
#[derive(Clone, Debug, Deserialize)]
pub struct MoleculeRequest {
    /// The molecule.
    pub smiles: String,
}

/// A similarity hit.
#[derive(Clone, Debug, Serialize)]
pub struct SimilarityHit {
    /// Molecule id.
    pub id: String,
    /// Stored SMILES.
    pub smiles: String,
    /// Exact Tanimoto.
    pub score: f64,
}

/// A substructure hit.
#[derive(Clone, Debug, Serialize)]
pub struct Hit {
    /// Molecule id.
    pub id: String,
    /// Stored SMILES.
    pub smiles: String,
}

/// Similarity response.
#[derive(Clone, Debug, Serialize)]
pub struct SimilarityResponse {
    /// Hits, best first.
    pub hits: Vec<SimilarityHit>,
    /// Exact number of matches.
    pub count: u64,
    /// Search time in microseconds (fingerprint included).
    pub took_us: u64,
    /// Rows whose score was computed; the rest were pruned by BitBound.
    pub scanned: u64,
    /// Molecules in the database.
    pub total: u64,
}

/// Substructure response.
#[derive(Clone, Debug, Serialize)]
pub struct SubstructureResponse {
    /// Hits in row order.
    pub hits: Vec<Hit>,
    /// Exact number of matches.
    pub count: u64,
    /// Search time in microseconds.
    pub took_us: u64,
    /// Candidates left after the screen and census, before exact matching.
    pub screened: u64,
    /// Molecules in the database.
    pub total: u64,
}

/// Molecule properties.
#[derive(Clone, Debug, Serialize)]
pub struct MoleculeResponse {
    /// Hill formula with charge.
    pub formula: String,
    /// Average molecular weight, g/mol.
    pub weight: f64,
    /// Atoms in the graph (explicit H atoms included).
    pub atoms: u32,
    /// Non-hydrogen atoms.
    pub heavy_atoms: u32,
    /// Bonds.
    pub bonds: u32,
    /// Ring count (cyclomatic number).
    pub rings: u32,
    /// Aromatic atoms.
    pub aromatic_atoms: u32,
    /// Bits set in the similarity fingerprint.
    pub fingerprint_bits_set: u32,
}

/// An indexed database.
pub struct Database {
    name: String,
    ids: Vec<Box<str>>,
    smiles: Vec<Box<str>>,
    substruct: SubstructIndex,
    arena: Arena,
    /// `arena_rows[sorted_row]` = molecule row.
    arena_rows: Vec<u32>,
    skipped: usize,
    build: Duration,
}

/// Parses a query SMILES into a perceived graph.
pub(crate) fn parse_query_smiles(smiles: &str) -> ApiResult<MolGraph> {
    parse_smiles(smiles).map_err(|e| ApiError::bad_request(format!("invalid SMILES: {e}")))
}

impl Database {
    /// Reads and indexes `spec`.
    ///
    /// # Errors
    /// I/O or layout errors from the file.
    pub fn load(spec: &DbSpec) -> std::io::Result<Database> {
        let rows = input::read_rows(&spec.path, spec.columns, spec.max_mols)?;
        let total = rows.len();
        let (db, _) = Self::build(&spec.name, rows.into_iter().map(|r| (r.id, r.smiles)));
        tracing::info!(
            db = %spec.name,
            molecules = db.len(),
            skipped = total - db.len(),
            secs = db.build.as_secs_f64(),
            "database loaded"
        );
        Ok(db)
    }

    /// Indexes `(id, smiles)` rows. Rows that do not parse, or are too large,
    /// are skipped; their input positions are returned.
    pub fn build(
        name: &str,
        rows: impl IntoIterator<Item = (String, String)>,
    ) -> (Database, Vec<usize>) {
        const CHUNK: usize = 1 << 16;
        let start = Instant::now();
        let mut rows = rows.into_iter();
        let mut skipped = Vec::new();
        // Parsed rows in order: (input position, id, smiles, fingerprint).
        let mut kept: Vec<(usize, String, String, Vec<u64>)> = Vec::new();
        let mut pos = 0;
        let chunks = std::iter::from_fn(|| {
            let chunk: Vec<(String, String)> = rows.by_ref().take(CHUNK).collect();
            if chunk.is_empty() {
                return None;
            }
            let parsed: Vec<Option<(MolGraph, Vec<u64>)>> = chunk
                .par_iter()
                .map(|(_, s)| {
                    let g = parse_smiles(s).ok()?;
                    let fp = fingerprint::ecfp4(&g);
                    Some((g, fp))
                })
                .collect();
            let mut graphs = Vec::with_capacity(chunk.len());
            for ((id, smi), p) in chunk.into_iter().zip(parsed) {
                match p {
                    Some((g, fp)) => {
                        kept.push((pos, id, smi, fp));
                        graphs.push(g);
                    }
                    None => skipped.push(pos),
                }
                pos += 1;
            }
            Some(graphs)
        });
        let (substruct, too_large) = SubstructIndex::from_graph_chunks(chunks);

        let mut too_large = too_large.into_iter().peekable();
        let mut ids = Vec::with_capacity(kept.len());
        let mut smiles = Vec::with_capacity(kept.len());
        let mut words = Vec::with_capacity(kept.len() * DEFAULT_BITS / 64);
        for (k, (i, id, smi, fp)) in kept.into_iter().enumerate() {
            if too_large.peek() == Some(&k) {
                too_large.next();
                skipped.push(i);
                continue;
            }
            ids.push(id.into_boxed_str());
            smiles.push(smi.into_boxed_str());
            words.extend_from_slice(&fp);
        }
        skipped.sort_unstable();
        let n = ids.len();
        let (arena, arena_rows) =
            Arena::build_with_order(DEFAULT_BITS, words, vec![String::new(); n]);
        let db = Database {
            name: name.to_string(),
            ids,
            smiles,
            substruct,
            arena,
            arena_rows,
            skipped: skipped.len(),
            build: start.elapsed(),
        };
        debug_assert_eq!(db.substruct.len(), db.ids.len());
        (db, skipped)
    }

    /// Name used in requests.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Indexed molecules.
    #[must_use]
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// True when no molecule is indexed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Summary.
    #[must_use]
    pub fn info(&self) -> DbInfo {
        let strings: usize = self
            .ids
            .iter()
            .chain(&self.smiles)
            .map(|s| s.len() + 16)
            .sum();
        DbInfo {
            name: self.name.clone(),
            molecules: self.len() as u64,
            skipped: self.skipped as u64,
            fingerprint: FingerprintInfo {
                kind: "ecfp4",
                bits: DEFAULT_BITS as u32,
            },
            build_ms: self.build.as_millis() as u64,
            memory_bytes: (self.substruct.mem_bytes()
                + self.arena.len() * self.arena.nwords() * 8
                + self.arena_rows.len() * 4
                + strings) as u64,
            ready: true,
        }
    }

    /// The `(id, smiles)` of molecule `row`.
    #[must_use]
    pub fn molecule(&self, row: u32) -> (&str, &str) {
        (&self.ids[row as usize], &self.smiles[row as usize])
    }

    /// Looks a molecule up by id (linear scan).
    #[must_use]
    pub fn find_id(&self, id: &str) -> Option<u32> {
        self.ids.iter().position(|x| &**x == id).map(|i| i as u32)
    }

    /// Exact Tanimoto search. `limit` caps the returned hits only.
    ///
    /// # Errors
    /// A bad SMILES or threshold.
    pub fn similarity(
        &self,
        smiles: &str,
        threshold: Option<f64>,
        k: Option<usize>,
        limit: usize,
    ) -> ApiResult<SimilarityResponse> {
        let start = Instant::now();
        let fp = fingerprint::ecfp4(&parse_query_smiles(smiles)?);
        let qpop = mossflux_core::popcount(&fp);
        let (mut hits, scanned) = match k {
            Some(k) => {
                let hits = self.arena.search_knn(&fp, k);
                let kth = hits.last().map_or(1.0, |h| h.score);
                (
                    hits,
                    self.arena
                        .band_candidate_count(qpop, kth.max(f64::MIN_POSITIVE)),
                )
            }
            None => {
                let t = threshold.unwrap_or(0.7);
                if !(t > 0.0 && t <= 1.0) {
                    return Err(ApiError::bad_request("threshold must be in (0, 1]"));
                }
                let hits = self.arena.search_threshold(&fp, t);
                (hits, self.arena.band_candidate_count(qpop, t))
            }
        };
        hits.sort_by(|a, b| {
            b.score
                .total_cmp(&a.score)
                .then(self.arena_rows[a.index as usize].cmp(&self.arena_rows[b.index as usize]))
        });
        let count = hits.len() as u64;
        let hits = hits
            .iter()
            .take(limit)
            .map(|h| {
                let (id, smiles) = self.molecule(self.arena_rows[h.index as usize]);
                SimilarityHit {
                    id: id.to_string(),
                    smiles: smiles.to_string(),
                    score: h.score,
                }
            })
            .collect();
        Ok(SimilarityResponse {
            hits,
            count,
            took_us: start.elapsed().as_micros() as u64,
            scanned,
            total: self.len() as u64,
        })
    }

    /// Exact SMARTS search under a deadline. Past the deadline the answer is
    /// [`ApiError`] timeout, never a partial list.
    ///
    /// # Errors
    /// A bad SMARTS or a timeout.
    pub fn substructure(
        &self,
        smarts: &str,
        offset: usize,
        limit: usize,
        timeout: Duration,
    ) -> ApiResult<SubstructureResponse> {
        let start = Instant::now();
        let limits = SearchLimits::timeout(timeout);
        let screened = self
            .substruct
            .screen_query(smarts)
            .map_err(|e| ApiError::bad_request(format!("invalid SMARTS: {e}")))?;
        let n_screened = screened.rows().len() as u64;
        let rows = self
            .substruct
            .verify_screened_with_limits(&screened, limits)
            .map_err(|e| match e {
                SearchError::Timeout => ApiError::timeout(),
                e => ApiError::bad_request(format!("invalid SMARTS: {e:?}")),
            })?;
        let hits = rows
            .iter()
            .skip(offset)
            .take(limit)
            .map(|&r| {
                let (id, smiles) = self.molecule(r);
                Hit {
                    id: id.to_string(),
                    smiles: smiles.to_string(),
                }
            })
            .collect();
        Ok(SubstructureResponse {
            hits,
            count: rows.len() as u64,
            took_us: start.elapsed().as_micros() as u64,
            screened: n_screened,
            total: self.len() as u64,
        })
    }
}

/// Properties of one molecule.
///
/// # Errors
/// A bad SMILES.
pub fn molecule(smiles: &str) -> ApiResult<MoleculeResponse> {
    let g = parse_query_smiles(smiles)?;
    let heavy = g.element.iter().filter(|&&z| z != 1).count();
    let aromatic = (0..g.atom_count() as u32)
        .filter(|&i| g.is_aromatic(i))
        .count();
    let components = components(&g);
    let fp = fingerprint::ecfp4(&g);
    Ok(MoleculeResponse {
        formula: props::formula(&g),
        weight: (props::weight(&g) * 1000.0).round() / 1000.0,
        atoms: g.atom_count() as u32,
        heavy_atoms: heavy as u32,
        bonds: g.bond_count() as u32,
        rings: (g.bond_count() + components).saturating_sub(g.atom_count()) as u32,
        aromatic_atoms: aromatic as u32,
        fingerprint_bits_set: mossflux_core::popcount(&fp),
    })
}

/// Connected components.
fn components(g: &MolGraph) -> usize {
    let n = g.atom_count();
    let mut seen = vec![false; n];
    let mut stack = Vec::new();
    let mut count = 0;
    for s in 0..n {
        if seen[s] {
            continue;
        }
        count += 1;
        seen[s] = true;
        stack.push(s as u32);
        while let Some(a) = stack.pop() {
            for nb in g.neighbors(a) {
                if !seen[nb.atom as usize] {
                    seen[nb.atom as usize] = true;
                    stack.push(nb.atom);
                }
            }
        }
    }
    count
}
