//! The in-RAM substructure index: parse and perceive a corpus once, then answer
//! SMARTS queries with a screen, a census filter and an exact verify.
//!
//! # The pipeline, and why it is exact
//!
//! Each SMILES is parsed and perceived into a [`MolGraph`], which is stored in
//! compact form with its screening [`Fingerprint`] and census at the same dense
//! row id (`u32`).
//!
//! [`SubstructIndex::search`] runs:
//!
//! 1. **Screen.** The query's required-bit mask ([`query_required_mask`]) selects
//!    the rows whose fingerprint holds every required bit
//!    ([`screen_contains`](crate::screen::ScreenIndex::screen_contains)). This is a
//!    superset of the matches (proven in [`screen`](crate::screen)). A query that
//!    pins no bit gets an empty mask, and every row passes.
//! 2. **Census.** Rows whose element/bond counts cannot host the query are
//!    dropped (also a superset, see [`crate::census`]).
//! 3. **Verify.** The query is planned once into a [`PlannedQuery`] and the
//!    [`Matcher`] checks each candidate. The matcher is tested against the
//!    brute-force oracle in [`vf2`](crate::vf2).
//!
//! Each filter is a superset and the verifier is exact, so for every query:
//!
//! > `search(q) == { i : brute_force_matches(parse(q), mols[i]) }`
//!
//! # Allocation
//!
//! A search plans once and reuses one [`Matcher`] per thread (its scratch resets
//! in O(1)), so it allocates only its candidate and result vectors.

use crate::census::{query_min_census, target_census, Census, CensusColumns};
use crate::compact::MatchMol;
use crate::graph::MolGraph;
use crate::query::QueryGraph;
use crate::screen::{query_required_mask, target_fingerprint, Fingerprint, ScreenIndex};
use crate::smarts::{parse as parse_smarts, SmartsError};
use crate::smiles::{parse as parse_smiles, SmilesError};
use crate::vf2::{Matcher, PlannedQuery};
use rayon::prelude::*;
use std::time::{Duration, Instant};

/// An error building a [`SubstructIndex`] from SMILES, with the failing row.
#[derive(Debug)]
pub struct BuildError {
    /// Row index (position in the input slice) that failed.
    pub row: usize,
    /// The SMILES string that failed.
    pub smiles: String,
    /// What went wrong.
    pub kind: BuildErrorKind,
}

/// Why a row could not be indexed by the strict [`SubstructIndex::from_smiles`].
#[derive(Debug)]
pub enum BuildErrorKind {
    /// The SMILES failed to parse.
    Parse(SmilesError),
    /// The molecule exceeds the compact store's limits (`≥ 65_535` atoms, or an
    /// atom with `> 255` neighbours).
    TooLarge {
        /// Atom count of the offending molecule.
        atoms: usize,
    },
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            BuildErrorKind::Parse(source) => write!(
                f,
                "failed to parse SMILES at row {} ({:?}): {}",
                self.row, self.smiles, source
            ),
            BuildErrorKind::TooLarge { atoms } => write!(
                f,
                "molecule at row {} ({:?}) has {atoms} atoms / a >255-degree atom, beyond the \
                 compact store's limits",
                self.row, self.smiles
            ),
        }
    }
}

/// The reason alone, without the row and the SMILES (for callers that report the
/// row in their own terms, such as a file line number).
impl core::fmt::Display for BuildErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BuildErrorKind::Parse(source) => write!(f, "{source}"),
            BuildErrorKind::TooLarge { atoms } => write!(
                f,
                "molecule has {atoms} atoms / a >255-degree atom, beyond the compact \
                 store's limits"
            ),
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            BuildErrorKind::Parse(source) => Some(source),
            BuildErrorKind::TooLarge { .. } => None,
        }
    }
}

/// Limits on one [`SubstructIndex::search_with_limits`] call. The default is no
/// limit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SearchLimits {
    /// Give up at this instant. Subgraph matching is NP-complete, so a short
    /// hostile query can take exponential time; a service should always set one.
    pub deadline: Option<Instant>,
}

impl SearchLimits {
    /// No limit (the same as `SearchLimits::default()`).
    pub fn none() -> Self {
        Self::default()
    }

    /// A deadline `timeout` from now.
    pub fn timeout(timeout: Duration) -> Self {
        SearchLimits {
            deadline: Instant::now().checked_add(timeout),
        }
    }
}

/// Why [`SubstructIndex::search_with_limits`] returned no answer.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SearchError {
    /// The query did not parse.
    Parse(SmartsError),
    /// The deadline passed before the search finished. No partial result is
    /// returned: a hit list cut short would look exact and not be.
    Timeout,
}

impl core::fmt::Display for SearchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SearchError::Parse(e) => write!(f, "{e}"),
            SearchError::Timeout => write!(f, "search passed its deadline"),
        }
    }
}

impl std::error::Error for SearchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SearchError::Parse(e) => Some(e),
            SearchError::Timeout => None,
        }
    }
}

impl From<SmartsError> for SearchError {
    fn from(e: SmartsError) -> Self {
        SearchError::Parse(e)
    }
}

/// The verify stage stopped at the deadline.
struct TimedOut;

/// The screen's output for one query, made by [`SubstructIndex::screen_query`]: the
/// parsed query and its candidate rows (a superset of the matches, sorted, no
/// duplicates).
///
/// Only `screen_query` makes one and its fields are private, so
/// [`SubstructIndex::verify_screened`] can trust the rows as
/// [`SubstructIndex::search`] does: a census-exact query's candidates are the
/// answer. This lets a benchmark time the two stages separately.
#[derive(Clone, Debug)]
pub struct Screened {
    query: QueryGraph,
    mask_bits: u32,
    rows: Vec<u32>,
    /// Size of the index that made this, to refuse use with another index.
    index_len: usize,
}

impl Screened {
    /// The candidate rows, sorted ascending.
    pub fn rows(&self) -> &[u32] {
        &self.rows
    }

    /// How many required bits the screen pinned (0 means it could not prune).
    pub fn mask_bits(&self) -> u32 {
        self.mask_bits
    }
}

/// An in-RAM substructure index over a corpus of molecules.
///
/// Holds compact molecules, a screen and a census, all addressed by a dense row
/// id (`u32`). See the [module docs](self) for the search semantics.
#[derive(Clone, Debug, Default)]
pub struct SubstructIndex {
    /// One compact molecule per row, for the matcher. The [`MolGraph`] it is built
    /// from is dropped after the build.
    mols: Vec<MatchMol>,
    /// One fingerprint per row.
    screen: ScreenIndex,
    /// The element/bond census per row, column-major (see [`crate::census`]).
    census: CensusColumns,
}

impl SubstructIndex {
    /// An empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds an index from a slice of SMILES strings.
    ///
    /// Row `i` is `smiles[i]`. Rows are parsed in parallel and collected in input
    /// order, so the result is identical to a serial build.
    ///
    /// # Errors
    /// Returns a [`BuildError`] for the lowest row that fails to parse or exceeds
    /// the compact store's limits. No row is silently dropped.
    pub fn from_smiles<S: AsRef<str> + Sync>(smiles: &[S]) -> Result<Self, BuildError> {
        // `par_iter()` collects in input order, so `built[i]` is `smiles[i]`.
        let built: Vec<Result<(MolGraph, Fingerprint, Census), BuildError>> = smiles
            .par_iter()
            .enumerate()
            .map(|(row, s)| {
                let s = s.as_ref();
                let mol = parse_smiles(s).map_err(|source| BuildError {
                    row,
                    smiles: s.to_string(),
                    kind: BuildErrorKind::Parse(source),
                })?;
                let fp = target_fingerprint(&mol);
                let census = target_census(&mol);
                Ok((mol, fp, census))
            })
            .collect();

        // Scanned in row order, so the first `Err` is the lowest row.
        let mut mols: Vec<MatchMol> = Vec::with_capacity(smiles.len());
        let mut fps: Vec<Fingerprint> = Vec::with_capacity(smiles.len());
        let mut censuses: Vec<Census> = Vec::with_capacity(smiles.len());
        for (row, entry) in built.into_iter().enumerate() {
            let (mol, fp, census) = entry?;
            let mm = MatchMol::try_from_molgraph(&mol).ok_or_else(|| BuildError {
                row,
                smiles: smiles[row].as_ref().to_string(),
                kind: BuildErrorKind::TooLarge {
                    atoms: mol.atom_count(),
                },
            })?;
            mols.push(mm);
            fps.push(fp);
            censuses.push(census);
        }

        // `prepare` builds the bit-transposed column store for the column-AND
        // screen; it changes speed only, not hits.
        let mut screen = ScreenIndex::from_fingerprints(fps);
        screen.prepare();
        let census = CensusColumns::build(&censuses);
        debug_assert_eq!(screen.len(), mols.len());
        debug_assert_eq!(census.len(), mols.len());
        Ok(SubstructIndex {
            mols,
            screen,
            census,
        })
    }

    /// Like [`from_smiles`](Self::from_smiles), but skips rows that fail to parse or
    /// are too large, and returns the index with the skipped count.
    ///
    /// Row ids are positions in the surviving set; the caller tracks any mapping
    /// back to the input.
    pub fn from_smiles_lossy<S: AsRef<str> + Sync>(smiles: &[S]) -> (Self, usize) {
        let built: Vec<Option<(MolGraph, Fingerprint, Census)>> = smiles
            .par_iter()
            .map(|s| {
                let mol = parse_smiles(s.as_ref()).ok()?;
                let fp = target_fingerprint(&mol);
                let census = target_census(&mol);
                Some((mol, fp, census))
            })
            .collect();
        let total = built.len();
        let mut mols = Vec::with_capacity(total);
        let mut fps = Vec::with_capacity(total);
        let mut censuses = Vec::with_capacity(total);
        for (mol, fp, census) in built.into_iter().flatten() {
            // An oversized molecule is skipped like an unparseable one.
            if let Some(mm) = MatchMol::try_from_molgraph(&mol) {
                mols.push(mm);
                fps.push(fp);
                censuses.push(census);
            }
        }
        // Counts unparseable rows (dropped by `flatten`) and oversized rows.
        let skipped = total - mols.len();
        let mut screen = ScreenIndex::from_fingerprints(fps);
        screen.prepare();
        let census = CensusColumns::build(&censuses);
        (
            SubstructIndex {
                mols,
                screen,
                census,
            },
            skipped,
        )
    }

    /// Like [`from_smiles_lossy`](Self::from_smiles_lossy), but returns an error for
    /// every skipped row instead of a count.
    ///
    /// The errors are sorted by [`BuildError::row`] (the input position). Index row
    /// `i` is the `i`-th input position not in the error list.
    pub fn from_smiles_with_errors<S: AsRef<str> + Sync>(smiles: &[S]) -> (Self, Vec<BuildError>) {
        let built: Vec<Result<(MolGraph, Fingerprint, Census), SmilesError>> = smiles
            .par_iter()
            .map(|s| {
                let mol = parse_smiles(s.as_ref())?;
                let fp = target_fingerprint(&mol);
                let census = target_census(&mol);
                Ok((mol, fp, census))
            })
            .collect();
        let mut mols = Vec::with_capacity(built.len());
        let mut fps = Vec::with_capacity(built.len());
        let mut censuses = Vec::with_capacity(built.len());
        let mut errors = Vec::new();
        for (row, entry) in built.into_iter().enumerate() {
            let kind = match entry {
                Ok((mol, fp, census)) => match MatchMol::try_from_molgraph(&mol) {
                    Some(mm) => {
                        mols.push(mm);
                        fps.push(fp);
                        censuses.push(census);
                        continue;
                    }
                    None => BuildErrorKind::TooLarge {
                        atoms: mol.atom_count(),
                    },
                },
                Err(source) => BuildErrorKind::Parse(source),
            };
            errors.push(BuildError {
                row,
                smiles: smiles[row].as_ref().to_string(),
                kind,
            });
        }
        let mut screen = ScreenIndex::from_fingerprints(fps);
        screen.prepare();
        let census = CensusColumns::build(&censuses);
        (
            SubstructIndex {
                mols,
                screen,
                census,
            },
            errors,
        )
    }

    /// Builds an index from already parsed and perceived graphs, in order.
    /// Returns the input positions of the graphs too large for the matcher;
    /// those are left out, and row `r` of the index is the `r`-th graph kept.
    pub fn from_graphs(graphs: Vec<MolGraph>) -> (Self, Vec<usize>) {
        Self::from_graph_chunks(std::iter::once(graphs))
    }

    /// [`from_graphs`](Self::from_graphs) over consecutive chunks, so only one
    /// chunk of full graphs is in memory at a time.
    pub fn from_graph_chunks(
        chunks: impl IntoIterator<Item = Vec<MolGraph>>,
    ) -> (Self, Vec<usize>) {
        let mut mols = Vec::new();
        let mut fps = Vec::new();
        let mut censuses = Vec::new();
        let mut rejected = Vec::new();
        let mut base = 0;
        for graphs in chunks {
            let n = graphs.len();
            let built: Vec<Option<(MatchMol, Fingerprint, Census)>> = graphs
                .into_par_iter()
                .map(|mol| {
                    let mm = MatchMol::try_from_molgraph(&mol)?;
                    Some((mm, target_fingerprint(&mol), target_census(&mol)))
                })
                .collect();
            for (i, entry) in built.into_iter().enumerate() {
                match entry {
                    Some((mm, fp, census)) => {
                        mols.push(mm);
                        fps.push(fp);
                        censuses.push(census);
                    }
                    None => rejected.push(base + i),
                }
            }
            base += n;
        }
        mols.shrink_to_fit();
        let mut screen = ScreenIndex::from_fingerprints(fps);
        screen.prepare();
        let census = CensusColumns::build(&censuses);
        (
            SubstructIndex {
                mols,
                screen,
                census,
            },
            rejected,
        )
    }

    /// Number of indexed molecules.
    #[inline]
    pub fn len(&self) -> usize {
        self.mols.len()
    }

    /// True if the index holds no molecules.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.mols.is_empty()
    }

    /// Read-only access to the compact molecule by row id (for tests / diagnostics).
    ///
    /// # Panics
    /// Panics if `row >= self.len()`.
    #[inline]
    #[must_use]
    pub fn mol(&self, row: u32) -> &MatchMol {
        &self.mols[row as usize]
    }

    /// Read-only access to the underlying screen (for tests / diagnostics).
    #[inline]
    pub fn screen(&self) -> &ScreenIndex {
        &self.screen
    }

    /// Approximate heap footprint of the index in bytes: molecules, screen and
    /// census. Counts each `Vec`'s `capacity`, so it estimates resident memory,
    /// not exact allocator use. Molecule footprints are summed in parallel.
    pub fn mem_bytes(&self) -> usize {
        let mols: usize = self.mols.par_iter().map(|m| m.mem_bytes()).sum();
        // Live elements are counted by `mem_bytes()`; add the spare-capacity
        // `MatchMol` slots.
        let spine = (self.mols.capacity() - self.mols.len()) * core::mem::size_of::<MatchMol>();
        mols + spine + self.screen.mem_bytes() + self.census.mem_bytes()
    }

    /// Read-only access to the census column store (for tests / diagnostics).
    #[inline]
    pub fn census(&self) -> &CensusColumns {
        &self.census
    }

    /// The screen and census stages of [`search`](Self::search): parse the query
    /// and return its candidate rows. With
    /// [`verify_screened`](Self::verify_screened) it gives exactly `search`.
    ///
    /// # Errors
    /// Returns the [`SmartsError`] if `query` fails to parse.
    pub fn screen_query(&self, query: &str) -> Result<Screened, SmartsError> {
        let q = parse_smarts(query)?;
        let mask = query_required_mask(&q);
        let mask_bits = mask.count_ones();
        // Bit screen, then census filter: both supersets, so no true match is
        // dropped. `mask_bits` reports the bit screen only.
        let rows = self.screen.screen_contains(&mask);
        let rows = self.census.filter(&query_min_census(&q), &rows);
        Ok(Screened {
            query: q,
            mask_bits,
            rows,
            index_len: self.len(),
        })
    }

    /// The verify stage of [`search`](Self::search): the exact matches among
    /// `screened`'s candidates, sorted ascending. Runs the same code as `search`.
    ///
    /// # Panics
    /// Panics if `screened` was made by an index of a different size (it would
    /// name rows of another corpus).
    pub fn verify_screened(&self, screened: &Screened) -> Vec<u32> {
        assert_eq!(
            screened.index_len,
            self.len(),
            "Screened was made by a different index"
        );
        // Census-exact queries need no VF2: their census-filtered candidates are
        // exactly the matches (see `search_with_limits`).
        if crate::census::is_census_exact(&screened.query) {
            return screened.rows.clone();
        }
        let planned = PlannedQuery::new(&screened.query);
        self.verify_planned(&planned, &screened.rows, None)
            .unwrap_or_else(|TimedOut| unreachable!("no deadline was set"))
    }

    /// [`verify_screened`](Self::verify_screened) under `limits`: the same exact
    /// rows, or [`SearchError::Timeout`] and no rows past the deadline.
    ///
    /// # Errors
    /// [`SearchError::Timeout`] if the deadline passes first.
    ///
    /// # Panics
    /// Panics if `screened` was made by an index of a different size.
    pub fn verify_screened_with_limits(
        &self,
        screened: &Screened,
        limits: SearchLimits,
    ) -> Result<Vec<u32>, SearchError> {
        assert_eq!(
            screened.index_len,
            self.len(),
            "Screened was made by a different index"
        );
        if crate::census::is_census_exact(&screened.query) {
            return Ok(screened.rows.clone());
        }
        let planned = PlannedQuery::new(&screened.query);
        self.verify_planned(&planned, &screened.rows, limits.deadline)
            .map_err(|TimedOut| SearchError::Timeout)
    }

    /// The screen's candidate rows for `query`, as `(mask_bits, rows)`: the same
    /// as [`screen_query`](Self::screen_query), without the parsed query.
    ///
    /// # Errors
    /// Returns the [`SmartsError`] if `query` fails to parse.
    pub fn screen_candidates(&self, query: &str) -> Result<(u32, Vec<u32>), SmartsError> {
        let s = self.screen_query(query)?;
        Ok((s.mask_bits, s.rows))
    }

    /// Verifies any `candidates` set, returning the rows that match `query`, sorted
    /// ascending without duplicates.
    ///
    /// The result is exactly `{ r in candidates : r matches query }` for any
    /// in-range rows, in any order and with repeats; the set is not trusted to
    /// come from the screen. Given the output of
    /// [`screen_candidates`](Self::screen_candidates) it equals
    /// [`search`](Self::search). For a census-exact query it does one census pass
    /// that `search` skips; to time the verify stage as `search` runs it, use
    /// [`screen_query`](Self::screen_query) and [`verify_screened`](Self::verify_screened).
    ///
    /// # Errors
    /// Returns the [`SmartsError`] if `query` fails to parse.
    ///
    /// # Panics
    /// Panics if any id in `candidates` is `>= self.len()`. The check runs first,
    /// so every query panics the same way on bad input.
    pub fn verify_candidates(
        &self,
        query: &str,
        candidates: &[u32],
    ) -> Result<Vec<u32>, SmartsError> {
        let n = self.mols.len();
        if let Some(&bad) = candidates.iter().find(|&&r| r as usize >= n) {
            panic!("candidate row {bad} out of range (index holds {n} molecules)");
        }
        let q = parse_smarts(query)?;
        // Normalise to sorted and unique; screen output already is (one O(n) check).
        let sorted;
        let candidates = if candidates.windows(2).all(|w| w[0] < w[1]) {
            candidates
        } else {
            let mut v = candidates.to_vec();
            v.sort_unstable();
            v.dedup();
            sorted = v;
            &sorted[..]
        };
        // For a census-exact query, census pass ⟺ match.
        if crate::census::is_census_exact(&q) {
            return Ok(self.census.filter(&query_min_census(&q), candidates));
        }
        let planned = PlannedQuery::new(&q);
        Ok(self
            .verify_planned(&planned, candidates, None)
            .unwrap_or_else(|TimedOut| unreachable!("no deadline was set")))
    }

    /// The shared verify kernel: runs the [`Matcher`] over `candidates` (sorted,
    /// unique) and returns the matches sorted ascending. Every verify entry point
    /// calls it, so they cannot diverge.
    ///
    /// With a `deadline`, every worker's matcher carries it, and the first to see
    /// it pass makes the call return `Err(TimedOut)`.
    fn verify_planned(
        &self,
        planned: &PlannedQuery,
        candidates: &[u32],
        deadline: Option<Instant>,
    ) -> Result<Vec<u32>, TimedOut> {
        // Below this, the rayon fan-out costs more than it saves.
        const PARALLEL_VERIFY_MIN: usize = 256;

        // Census early-accept: a census count ≥ 1 for this class proves a match, so
        // VF2 is skipped. Other rows still run VF2, so the result stays exact.
        let early = planned.early_accept_class();
        let accepted = |row: u32| early.is_some_and(|c| self.census.count_at(c, row) >= 1);

        let hits = if candidates.len() < PARALLEL_VERIFY_MIN {
            // Serial: one reused Matcher. `candidates.len()` bounds the hit count.
            let mut matcher = Matcher::new();
            matcher.set_deadline(deadline);
            let _guard = crate::deadline::set(deadline);
            let mut hits = Vec::with_capacity(candidates.len());
            for &row in candidates {
                if accepted(row) || matcher.matches(planned, &self.mols[row as usize]) {
                    hits.push(row);
                }
                if deadline.is_some() && (matcher.timed_out() || crate::deadline::expired()) {
                    return Err(TimedOut);
                }
            }
            hits
        } else {
            // Parallel by contiguous chunk. Each worker owns one `Matcher` (via
            // `map_init`) and collects its chunk's hits in ascending order.
            // `par_chunks` is indexed, so `collect()` keeps chunk order and the
            // concatenation is ascending without a global sort.
            //
            // The chunk size amortises the per-chunk `Vec` while staying small
            // enough for work stealing to balance uneven candidate costs.
            const VERIFY_CHUNK: usize = 1024;
            let per_chunk: Vec<Result<Vec<u32>, TimedOut>> = candidates
                .par_chunks(VERIFY_CHUNK)
                .map_init(
                    || {
                        let mut m = Matcher::new();
                        m.set_deadline(deadline);
                        m
                    },
                    |matcher, chunk| {
                        if matcher.timed_out() || deadline.is_some_and(|d| Instant::now() >= d) {
                            return Err(TimedOut);
                        }
                        let _guard = crate::deadline::set(deadline);
                        let mut local = Vec::new();
                        for &row in chunk {
                            if accepted(row) || matcher.matches(planned, &self.mols[row as usize]) {
                                local.push(row);
                            }
                            if deadline.is_some()
                                && (matcher.timed_out() || crate::deadline::expired())
                            {
                                return Err(TimedOut);
                            }
                        }
                        Ok(local)
                    },
                )
                .collect();
            let per_chunk: Vec<Vec<u32>> = per_chunk.into_iter().collect::<Result<_, _>>()?;
            let total: usize = per_chunk.iter().map(Vec::len).sum();
            let mut hits = Vec::with_capacity(total);
            for chunk_hits in per_chunk {
                hits.extend_from_slice(&chunk_hits);
            }
            hits
        };

        // The public guarantee: sorted ascending, no duplicates. Both paths inherit
        // it from the ascending candidates.
        debug_assert!(hits.windows(2).all(|w| w[0] < w[1]));
        Ok(hits)
    }

    /// Searches the corpus for molecules that contain the SMARTS `query` as a
    /// substructure, returning their row ids **sorted ascending**.
    ///
    /// The result is exactly what the brute-force oracle returns over every
    /// molecule (see the [module docs](self)).
    ///
    /// This call has no time limit. Subgraph matching is NP-complete, so for an
    /// untrusted query use [`search_with_limits`](Self::search_with_limits).
    ///
    /// # Errors
    /// Returns the [`SmartsError`] if `query` fails to parse or uses an
    /// unsupported feature.
    pub fn search(&self, query: &str) -> Result<Vec<u32>, SmartsError> {
        match self.search_with_limits(query, SearchLimits::none()) {
            Ok(hits) => Ok(hits),
            Err(SearchError::Parse(e)) => Err(e),
            Err(SearchError::Timeout) => unreachable!("no deadline was set"),
        }
    }

    /// [`search`](Self::search) under `limits`. Within the limits the answer is
    /// the same exact row set; past the deadline the call returns
    /// [`SearchError::Timeout`] and no rows.
    ///
    /// The deadline is checked at least every few thousand matcher steps on every
    /// worker, so a search overruns it by microseconds, not by the length of the
    /// exponential search it interrupts.
    ///
    /// # Errors
    /// [`SearchError::Parse`] if `query` fails to parse, [`SearchError::Timeout`]
    /// if the deadline passes first.
    pub fn search_with_limits(
        &self,
        query: &str,
        limits: SearchLimits,
    ) -> Result<Vec<u32>, SearchError> {
        let q = parse_smarts(query)?;

        // Screen, then census: both supersets of the matches.
        let mask = query_required_mask(&q);
        let candidates = self.screen.screen_contains(&mask);
        let candidates = self.census.filter(&query_min_census(&q), &candidates);

        // Census-exact query (see `census::is_census_exact`): census pass ⟺ match,
        // so the survivors are the answer.
        if crate::census::is_census_exact(&q) {
            return Ok(candidates);
        }

        // Verify. The plan is read-only and shared across threads; matcher
        // scratch is per thread.
        let planned = PlannedQuery::new(&q);
        self.verify_planned(&planned, &candidates, limits.deadline)
            .map_err(|TimedOut| SearchError::Timeout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brute;
    use crate::smarts::parse as parse_smarts;
    use std::collections::HashSet;

    // ----------------------------------------------------------------------
    // Basic construction / accessors
    // ----------------------------------------------------------------------

    /// A bare element whose census class also holds other labels must not skip
    /// VF2. `P` shares a class with aromatic `p`, and `B`, `[Zn]`, `[Fe]` share
    /// `OTHER` with every rare element.
    #[test]
    fn single_element_queries_equal_the_oracle() {
        let corpus = [
            "c1ccpcc1",
            "CP(C)C",
            "CB(C)C",
            "[Na+].[Cl-]",
            "[Zn+2].[O-]C=O",
            "[Fe]",
            "C[Si](C)(C)C",
            "FC(F)F",
            "ClC",
            "c1ccsc1",
            "CSC",
            "c1ccoc1",
            "CO",
            "c1ccncc1",
            "CN",
            "c1ccccc1",
            "CC",
        ];
        let idx = SubstructIndex::from_smiles(&corpus).unwrap();
        let mols: Vec<MolGraph> = corpus.iter().map(|s| parse_smiles(s).unwrap()).collect();
        let every_row: Vec<u32> = (0..corpus.len() as u32).collect();
        let mut nonempty = 0;
        for q in [
            "P", "p", "B", "[Zn]", "[Fe]", "[Si]", "[Na]", "[Na+]", "F", "Cl", "S", "s", "O", "o",
            "N", "n", "C", "c",
        ] {
            let parsed = parse_smarts(q).unwrap();
            let want: Vec<u32> = (0..mols.len() as u32)
                .filter(|&i| brute::matches(&parsed, &mols[i as usize]))
                .collect();
            let (_, cands) = idx.screen_candidates(q).unwrap();
            assert_eq!(idx.search(q).unwrap(), want, "search {q:?}");
            assert_eq!(
                idx.verify_candidates(q, &cands).unwrap(),
                want,
                "split {q:?}"
            );
            // Every row as a candidate: no screen to hide a census-class mistake.
            assert_eq!(
                idx.verify_candidates(q, &every_row).unwrap(),
                want,
                "all {q:?}"
            );
            nonempty += usize::from(!want.is_empty());
        }
        assert!(nonempty >= 15, "only {nonempty} queries had a hit");

        // A screen bit collision lets this row through for `P`, although it has
        // only an aromatic `p`.
        let smi = "FOCO(F)=CC=SF(ON(F)(CCC(C(=C)=Br(Br)(SF(S))))).c1ccpcc1";
        let one = SubstructIndex::from_smiles(&[smi]).unwrap();
        let mol = parse_smiles(smi).unwrap();
        assert!(!brute::matches(&parse_smarts("P").unwrap(), &mol));
        assert_eq!(one.search("P").unwrap(), Vec::<u32>::new());
    }

    /// `verify_candidates` gives the exact matches among ANY in-range set, in
    /// any order, with repeats: it never trusts the set came from the screen.
    #[test]
    fn verify_candidates_is_exact_on_any_candidate_set() {
        let corpus = ["CCO", "C=O", "CCN"];
        let idx = SubstructIndex::from_smiles(&corpus).unwrap();
        // `N` is census-exact; `C=O` is census-exact; `CC` goes through VF2.
        assert_eq!(idx.verify_candidates("N", &[0, 1, 2]).unwrap(), vec![2]);
        assert_eq!(idx.verify_candidates("C=O", &[0, 1, 2]).unwrap(), vec![1]);
        assert_eq!(idx.verify_candidates("CC", &[2, 2, 0]).unwrap(), vec![0, 2]);
        assert_eq!(idx.verify_candidates("N", &[2, 0, 2]).unwrap(), vec![2]);
        assert!(idx.verify_candidates("N", &[]).unwrap().is_empty());
    }

    #[test]
    #[should_panic(expected = "candidate row 3 out of range")]
    fn verify_candidates_panics_on_an_out_of_range_row() {
        let idx = SubstructIndex::from_smiles(&["CCO", "C=O", "CCN"]).unwrap();
        let _ = idx.verify_candidates("N", &[0, 3]);
    }

    #[test]
    fn build_and_basic_search() {
        let smiles = ["CCO", "c1ccccc1", "CC(=O)O", "CCN", "C1CCCCC1"];
        let idx = SubstructIndex::from_smiles(&smiles).unwrap();
        assert_eq!(idx.len(), smiles.len());
        assert!(!idx.is_empty());

        // Benzene query hits only the benzene row.
        assert_eq!(idx.search("c1ccccc1").unwrap(), vec![1]);
        // Aliphatic oxygen is in ethanol (0) and acetic acid (2).
        assert_eq!(idx.search("O").unwrap(), vec![0, 2]);
        // A double-bonded oxygen (carbonyl) is only in acetic acid (2).
        assert_eq!(idx.search("C=O").unwrap(), vec![2]);
        // Nitrogen only in ethylamine (3).
        assert_eq!(idx.search("N").unwrap(), vec![3]);
        // A six-membered aliphatic ring only in cyclohexane (4).
        assert_eq!(idx.search("C1CCCCC1").unwrap(), vec![4]);
    }

    #[test]
    fn empty_index_search_is_empty() {
        let idx = SubstructIndex::new();
        assert!(idx.is_empty());
        assert!(idx.search("c1ccccc1").unwrap().is_empty());
    }

    #[test]
    fn results_are_sorted_ascending() {
        // A query matching many rows must come back sorted.
        let smiles = ["CCO", "CCC", "CCN", "CCCC", "CCO", "OCCO", "CCOCC"];
        let idx = SubstructIndex::from_smiles(&smiles).unwrap();
        let hits = idx.search("CC").unwrap();
        assert!(hits.windows(2).all(|w| w[0] < w[1]), "{hits:?} not sorted");
        // Every returned row matches the oracle (re-parsed: the index stores the
        // compact form).
        let q = parse_smarts("CC").unwrap();
        for &r in &hits {
            let mol = crate::smiles::parse(smiles[r as usize]).unwrap();
            assert!(brute::matches(&q, &mol));
        }
    }

    #[test]
    fn build_error_names_bad_row() {
        let smiles = ["CCO", "this is not smiles!!!", "CCN"];
        let err = SubstructIndex::from_smiles(&smiles).unwrap_err();
        assert_eq!(err.row, 1);
        assert_eq!(err.smiles, "this is not smiles!!!");
        assert!(format!("{err}").contains("row 1"));
    }

    #[test]
    fn bad_query_returns_parse_error() {
        let idx = SubstructIndex::from_smiles(&["CCO"]).unwrap();
        // A malformed query is a clean Err, not a panic or a wrong answer.
        assert!(
            idx.search("[$(CC]").is_err(),
            "unterminated recursive bracket"
        );
        assert!(idx.search("C(C").is_err(), "unbalanced branch");
        assert!(idx.search("[Zz]").is_err(), "not an element");
    }

    // ----------------------------------------------------------------------
    // End-to-end exactness: search == brute force over all molecules
    // ----------------------------------------------------------------------

    /// ~60 molecules: chains, branches, rings, fused/bridged systems,
    /// heteroaromatics, halogens, charges and symmetric skeletons.
    fn corpus() -> Vec<&'static str> {
        vec![
            // chains / branches
            "C",
            "CC",
            "CCC",
            "CCCC",
            "CCCCC",
            "CCO",
            "OCCO",
            "CC(C)C",
            "CC(C)(C)C",
            "CCN",
            "CCOCC",
            "CC(C)C(=O)O",
            "NCCCN",
            "ClCCBr",
            "CCCCCCCC",
            "OCC(O)CO",
            // carbonyls / acids / amides / nitriles
            "CC(=O)O",
            "CC(=O)N",
            "CC=O",
            "CC#N",
            "O=C(N)C",
            "CC(=O)OC",
            // simple rings
            "C1CCCCC1",
            "C1CCCC1",
            "C1CCC1",
            "C1CCCCCC1",
            "C1CCNCC1",
            "O1CCCC1",
            "C1CCOC1",
            "C1CCSC1",
            // aromatics + heteroaromatics
            "c1ccccc1",
            "Cc1ccccc1",
            "c1ccncc1",
            "c1ccc(O)cc1",
            "c1cc[nH]c1",
            "c1ccoc1",
            "c1ccsc1",
            "Oc1ccccc1O",
            "Nc1ccccc1",
            "c1ccc(N)cc1",
            "Clc1ccccc1",
            "c1ccc(C=O)cc1",
            "c1ccc(C(=O)O)cc1",
            // fused / bridged / spiro
            "c1ccc2ccccc2c1",
            "c1ccc2[nH]ccc2c1",
            "C1CC2CCC1CC2",
            "C1CCC2(CC1)CCCC2",
            "C1=CC=CC=C1",
            "C1=CC=CC1",
            "c1ccc2ncccc2c1",
            // charges / brackets
            "[NH4+]",
            "CC[O-]",
            "[O-]C(=O)C",
            "c1cc[n+]([O-])cc1",
            // repeated elements / symmetry / halogens
            "FC(F)(F)F",
            "ClC(Cl)Cl",
            "C1CC1C1CC1",
            "BrCCBr",
            "FCCF",
            "IC",
        ]
    }

    /// Queries: forced atoms, paths and rings (the screen prunes), logical bracket
    /// expressions, bond primitives, and disconnected fragments (no screen). Each
    /// has both hits and misses on the corpus.
    fn queries() -> Vec<&'static str> {
        vec![
            // single-atom predicates
            "C",
            "c",
            "N",
            "n",
            "O",
            "o",
            "S",
            "F",
            "Cl",
            "Br",
            "I",
            "[#6]",
            "[#7]",
            "[#8]",
            "a",
            "A",
            "[R]",
            "[r6]",
            "[r5]",
            "[O-]",
            "[!C]",
            "[C,N]",
            "[#7,#8]",
            "[c;R]",
            // two-atom paths with various bonds
            "CC",
            "CO",
            "CN",
            "C=C",
            "C=O",
            "C#N",
            "C~C",
            "cc",
            "co",
            "cn",
            "c:c",
            "C-C",
            "[#6][#8]",
            "[#6]=[#8]",
            "a~a",
            "CCl",
            "CBr",
            // branched / longer paths / rings
            "CCC",
            "CCO",
            "CC(C)C",
            "CC=O",
            "CC(=O)O",
            "OCC",
            "NCC",
            "ccc",
            "cccc",
            "c1ccccc1",
            "c1ccncc1",
            "C1CCCCC1",
            "C1CCC1",
            "c1ccc(O)cc1",
            "[#6]1[#6][#6][#6][#6][#6]1",
            // ring + property combos / bond logic / negation
            "[c;R]1[c;R][c;R][c;R][c;R][c;R]1",
            "[R]~[R]",
            "[#7;R]",
            "C-,=C",
            "C!@C",
            "[!c]~[!c]",
            // disconnected fragments
            "C.C",
            "C.O",
            "[#6].[#7]",
            "c1ccccc1.O",
        ]
    }

    /// For every query, `search` returns exactly the rows the brute-force oracle
    /// accepts over all molecules. A failure names the query and the difference.
    #[test]
    fn search_equals_brute_force_over_corpus() {
        let smiles = corpus();
        let qs = queries();
        let idx = SubstructIndex::from_smiles(&smiles).unwrap();

        let mut total_pairs = 0usize;
        let mut total_hits = 0usize;

        for q in &qs {
            let got: Vec<u32> = idx
                .search(q)
                .unwrap_or_else(|e| panic!("query {q:?} should parse for the gate: {e}"));
            let got_set: HashSet<u32> = got.iter().copied().collect();
            assert_eq!(
                got.len(),
                got_set.len(),
                "query {q:?}: duplicate rows in result"
            );

            // The oracle: brute force over every molecule, re-parsed.
            let parsed = parse_smarts(q).unwrap();
            let mut want: Vec<u32> = Vec::new();
            for (row, s) in smiles.iter().enumerate() {
                let mol = crate::smiles::parse(s).unwrap();
                if brute::matches(&parsed, &mol) {
                    want.push(row as u32);
                }
            }
            let want_set: HashSet<u32> = want.iter().copied().collect();

            assert_eq!(
                got_set,
                want_set,
                "query {q:?}: search != brute-force.\n  screen+vf2 extra: {:?}\n  \
                 brute-only (MISSED!): {:?}",
                got_set.difference(&want_set).collect::<Vec<_>>(),
                want_set.difference(&got_set).collect::<Vec<_>>(),
            );
            assert!(
                got.windows(2).all(|w| w[0] < w[1]),
                "query {q:?}: result not sorted"
            );

            total_pairs += smiles.len();
            total_hits += want.len();
        }

        // Anti-vacuity: many pairs, with both hits and misses.
        assert!(
            total_pairs > 2000,
            "gate should compare many pairs, got {total_pairs}"
        );
        assert!(
            total_hits > 100,
            "corpus should yield many true matches, got {total_hits}"
        );
        assert!(
            total_hits < total_pairs,
            "corpus should also yield non-matches, not all-hits"
        );
    }

    // ----------------------------------------------------------------------
    // The split screen/verify entry points compose to `search`
    // ----------------------------------------------------------------------

    /// `screen_candidates` then `verify_candidates` equals `search` for every
    /// query, the candidates are a superset of the hits, and they never exceed the
    /// corpus size.
    #[test]
    fn split_screen_verify_equals_search_over_corpus() {
        let smiles = corpus();
        let idx = SubstructIndex::from_smiles(&smiles).unwrap();
        let n = idx.len() as u32;

        for q in queries() {
            let via_search = idx.search(q).unwrap();
            let (mask_bits, cands) = idx.screen_candidates(q).unwrap();
            let via_split = idx.verify_candidates(q, &cands).unwrap();

            assert_eq!(
                via_split, via_search,
                "query {q:?}: split screen/verify != search"
            );

            // Candidates are a superset of the hits.
            let cand_set: HashSet<u32> = cands.iter().copied().collect();
            for &h in &via_split {
                assert!(
                    cand_set.contains(&h),
                    "query {q:?}: hit {h} missing from screen candidates"
                );
            }
            // A zero screen mask does not imply every row: the census can still
            // prune (`[#6]=[#8]` forces a double bond the screen cannot see). So
            // only `<= n` holds here; the test below covers fully unforced queries.
            assert!(
                cands.len() <= n as usize,
                "query {q:?}: candidate count {} exceeds corpus size {n}",
                cands.len(),
            );
            let _ = mask_bits;
        }
    }

    /// A query that forces no screen bit and no census class (wildcard, OR,
    /// negation) keeps every row through both filters.
    #[test]
    fn fully_unforced_query_screens_in_every_row() {
        let smiles = corpus();
        let idx = SubstructIndex::from_smiles(&smiles).unwrap();
        let n = idx.len();
        for q in ["*", "a", "A", "[!C]", "[C,N]", "[#7,#8]", "[R]", "[r6]"] {
            let (mask_bits, cands) = idx.screen_candidates(q).unwrap();
            assert_eq!(mask_bits, 0, "query {q:?} should pin no screen bit");
            assert_eq!(
                cands.len(),
                n,
                "fully-unforced query {q:?} must screen in every row"
            );
        }
    }

    // ----------------------------------------------------------------------
    // Parallel search == serial search
    // ----------------------------------------------------------------------

    /// Reference serial verifier: screen, then one reused `Matcher` over the
    /// candidates in ascending order. Independent of `search`'s parallel
    /// threshold.
    fn serial_search(idx: &SubstructIndex, query: &str) -> Vec<u32> {
        let q = parse_smarts(query).unwrap();
        let mask = query_required_mask(&q);
        let candidates = idx.screen().screen_contains(&mask);
        let planned = PlannedQuery::new(&q);
        let mut matcher = Matcher::new();
        let mut hits = Vec::new();
        for &row in &candidates {
            if matcher.matches(&planned, idx.mol(row)) {
                hits.push(row);
            }
        }
        hits
    }

    /// On a corpus large enough to take the parallel verify path, `search` returns
    /// exactly the reference serial verifier's rows, in the same order.
    #[test]
    fn parallel_search_equals_serial_search() {
        // Low-selectivity queries give thousands of candidates (parallel path);
        // selective ones stay small (serial path).
        let motifs = [
            "c1ccccc1",
            "C1CCCCC1",
            "CC(=O)O",
            "CCO",
            "CCN",
            "c1ccncc1",
            "Cc1ccccc1",
            "CC(C)C(=O)O",
            "OCC(O)CO",
            "FC(F)(F)F",
            "ClCCBr",
            "Nc1ccccc1",
            "c1ccc(O)cc1",
            "C1CCNCC1",
            "CC(=O)N",
            "CC#N",
        ];
        let mut smiles: Vec<String> = Vec::with_capacity(4000);
        for i in 0..4000usize {
            let base = motifs[i % motifs.len()];
            let chain = "C".repeat(1 + (i % 6));
            smiles.push(format!("{chain}{base}"));
        }
        let idx = SubstructIndex::from_smiles(&smiles).unwrap();

        // Low- and high-selectivity probes, bond/ring/aromatic ones, and
        // disconnected queries that the screen cannot prune.
        let probes = [
            "C",
            "CC",
            "CCC",
            "O",
            "N",
            "C=O",
            "C#N",
            "c",
            "cc",
            "ccc",
            "c1ccccc1",
            "c1ccncc1",
            "C1CCCCC1",
            "CC(=O)O",
            "FC(F)(F)F",
            "Nc1ccccc1",
            "[#6][#8]",
            "[R]~[R]",
            "C.O",
            "C.C",
        ];

        let mut saw_large = false;
        for q in probes {
            let got = idx.search(q).unwrap();
            let want = serial_search(&idx, q);
            assert_eq!(got, want, "query {q:?}: parallel search != serial search");
            assert!(
                got.windows(2).all(|w| w[0] < w[1]),
                "query {q:?}: result not strictly ascending: {got:?}"
            );
            // At least one probe must take the parallel path.
            if want.len() >= 256 {
                saw_large = true;
            }
        }
        assert!(
            saw_large,
            "gate must exercise the parallel-verify branch (no probe exceeded the \
             threshold)"
        );
    }

    /// Candidate counts on both sides of the parallel threshold give the serial
    /// answer.
    #[test]
    fn search_matches_serial_across_threshold_boundary() {
        // Every "CCO" matches "CC", so the candidate count is exactly `n`.
        for n in [255usize, 256, 257, 600] {
            let smiles: Vec<&str> = std::iter::repeat_n("CCO", n).collect();
            let idx = SubstructIndex::from_smiles(&smiles).unwrap();
            let got = idx.search("CC").unwrap();
            let want = serial_search(&idx, "CC");
            assert_eq!(got, want, "n={n}: parallel/serial disagree on CC");
            let expected: Vec<u32> = (0..n as u32).collect();
            assert_eq!(got, expected, "n={n}: expected all rows to match CC");
        }
    }

    /// Two builds of one corpus report the same memory and answer every probe the
    /// same, and each answer equals the brute-force oracle.
    ///
    /// This catches thread-scheduling nondeterminism, not hash-order dependence:
    /// `RandomState` is seeded once per process, so both builds see the same hash
    /// order. `tests/cross_process_determinism.rs` covers that.
    #[test]
    fn parallel_build_is_deterministic() {
        let motifs = [
            "c1ccccc1",
            "CC(=O)O",
            "CCO",
            "c1ccncc1",
            "Cc1ccccc1",
            "FC(F)(F)F",
            "Nc1ccccc1",
            "c1ccc(O)cc1",
            "CC(=O)N",
            "CC#N",
            "CS(=O)(=O)N",
            "c1ccc2ccccc2c1",
        ];
        let smiles: Vec<String> = (0..2000usize)
            .map(|i| format!("{}{}", "C".repeat(1 + i % 5), motifs[i % motifs.len()]))
            .collect();
        let a = SubstructIndex::from_smiles(&smiles).unwrap();
        let b = SubstructIndex::from_smiles(&smiles).unwrap();
        assert_eq!(
            a.mem_bytes(),
            b.mem_bytes(),
            "build memory must be deterministic"
        );

        // Perceived separately for the oracle.
        let mols: Vec<_> = smiles
            .iter()
            .map(|s| crate::smiles::parse(s).unwrap())
            .collect();

        let mut total_hits = 0usize;
        let mut total_pairs = 0usize;
        for q in [
            "c1ccccc1",
            "c1ccncc1",
            "C(=O)O",
            "C#N",
            "S(=O)(=O)N",
            "c1ccc2ccccc2c1",
            "[R]~[R]",
            "C(=O)N",
            "Nc1ccccc1",
            "FC(F)(F)F",
            "c1ccc(O)cc1",
        ] {
            let ra = a.search(q).unwrap();
            let rb = b.search(q).unwrap();
            assert_eq!(ra, rb, "nondeterministic build for {q:?}");
            let parsed = parse_smarts(q).unwrap();
            let want: Vec<u32> = mols
                .iter()
                .enumerate()
                .filter(|(_, m)| brute::matches(&parsed, m))
                .map(|(i, _)| i as u32)
                .collect();
            assert_eq!(
                ra, want,
                "parallel build disagrees with brute force for {q:?}"
            );
            total_hits += want.len();
            total_pairs += mols.len();
        }
        // Anti-vacuity: both hits and misses.
        assert!(total_hits > 1000, "probes found only {total_hits} hits");
        assert!(total_hits < total_pairs, "every probe matched everything");
    }

    /// `from_smiles_lossy` drops unparseable rows and renumbers the survivors. The
    /// differential fuzz pre-filters its inputs, so only this test covers a drop.
    #[test]
    fn lossy_build_renumbers_after_dropped_rows() {
        // Row 1 is unparseable; CCO→0, c1ccccc1→1.
        let smiles = [
            "CCO".to_string(),
            "@@@not-a-molecule".to_string(),
            "c1ccccc1".to_string(),
        ];
        let (idx, skipped) = SubstructIndex::from_smiles_lossy(&smiles);
        assert_eq!(skipped, 1, "exactly one row should be skipped");
        assert_eq!(
            idx.search("O").unwrap(),
            vec![0],
            "CCO is the only O-bearing row, compacted to id 0"
        );
        assert_eq!(
            idx.search("c1ccccc1").unwrap(),
            vec![1],
            "benzene compacted from id 2 to id 1"
        );
        assert!(idx.search("C#N").unwrap().is_empty(), "no nitrile present");
    }

    /// An oversized molecule is skipped and counted by the lossy builder, and is a
    /// clean `BuildError` (not a panic) from the strict builder.
    #[test]
    fn oversized_molecule_is_skipped_not_fatal() {
        // 70_000 atoms: refused by the SMILES parser.
        let big = "C".repeat(70_000);
        // An atom with 300 neighbours: parses, but the compact store refuses it.
        let high_degree = format!("C{}", "(C)".repeat(300));

        let corpus = [
            "CCO".to_string(),
            big.clone(),
            "c1ccccc1".to_string(),
            high_degree.clone(),
            "CC(=O)O".to_string(),
        ];
        let (idx, skipped) = SubstructIndex::from_smiles_lossy(&corpus);
        assert_eq!(skipped, 2, "both oversized rows skipped");
        assert_eq!(idx.len(), 3, "the three indexable rows survive");
        // Survivors are compacted: CCO=0, benzene=1, acetic acid=2.
        assert_eq!(idx.search("c1ccccc1").unwrap(), vec![1]);
        assert_eq!(idx.search("C(=O)[OH]").unwrap(), vec![2]);

        // Strict: too many atoms is a parse error...
        match SubstructIndex::from_smiles(&["CCO".to_string(), big]) {
            Err(BuildError {
                row,
                kind: BuildErrorKind::Parse(SmilesError::TooLarge { what, limit }),
                ..
            }) => {
                assert_eq!(row, 1);
                assert_eq!((what, limit), ("atoms", crate::smiles::MAX_ATOMS));
            }
            other => panic!("expected a parse-time TooLarge error, got {other:?}"),
        }
        // ...and an atom with more than 255 neighbours by the compact store.
        match SubstructIndex::from_smiles(&["CCO".to_string(), high_degree]) {
            Err(BuildError {
                row,
                kind: BuildErrorKind::TooLarge { atoms },
                ..
            }) => {
                assert_eq!(row, 1);
                assert_eq!(atoms, 301);
            }
            other => panic!("expected a TooLarge BuildError, got {other:?}"),
        }
    }

    /// A molecule just under the u16 atom-index limit builds and is searchable.
    #[test]
    fn largest_legal_molecule_builds() {
        let n = (u16::MAX - 1) as usize; // 65_534
        let chain = "C".repeat(n);
        let (idx, skipped) = SubstructIndex::from_smiles_lossy(&[chain]);
        assert_eq!(skipped, 0, "65534 atoms is within the compact-form limit");
        assert_eq!(idx.len(), 1);
        assert_eq!(
            idx.search("CCC").unwrap(),
            vec![0],
            "a 3-carbon chain is present"
        );
    }
}
