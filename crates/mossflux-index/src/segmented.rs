//! Multi-segment index for incremental growth.
//!
//! An [`Arena`] is immutable. A [`SegmentedIndex`] appends each batch as its own
//! popcount-sorted segment (O(batch); existing segments are untouched) and fans a
//! query out to every segment, merging the results.
//!
//! Exactness: a row's Tanimoto depends only on the query and the row's bits, and
//! each segment is popcount-sorted, so per-segment BitBound pruning is valid. The
//! union of per-segment hits equals the hit set of one arena over the same rows.
//! Global ids (`base[seg] + local`) are disjoint, so no dedup is needed.
//!
//! Segments are searched serially per query; parallelism is across queries (one
//! [`BitmapScratch`] per worker).

use mossflux_core::lanes_for_bits;

use crate::arena::Arena;
use crate::inverted::{BitmapScratch, InvertedBitmap};

/// A hit with a global row id (`base[segment] + local_index`). `u64` because a
/// segmented index can hold more than `u32::MAX` rows.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlobalHit {
    /// Tanimoto similarity to the query, in `[0.0, 1.0]`, exactly as
    /// [`crate::Hit::score`] defines it.
    pub score: f64,
    /// Global row id: `base[segment] + local_index`. Resolve it with
    /// [`SegmentedIndex::id`].
    pub index: u64,
}

struct Segment {
    arena: Arena,
    bitmap: InvertedBitmap,
}

/// An append-only collection of popcount-sorted [`Arena`] segments searched as one
/// logical index. Appends never rebuild existing segments.
pub struct SegmentedIndex {
    nbits: usize,
    segments: Vec<Segment>,
    /// `base[s]` = global id of segment `s`'s row 0 (ascending prefix sum of lens).
    base: Vec<u64>,
    total: u64,
}

impl SegmentedIndex {
    /// An empty index over `nbits`-wide fingerprints.
    pub fn new(nbits: usize) -> Self {
        Self {
            nbits,
            segments: Vec::new(),
            base: Vec::new(),
            total: 0,
        }
    }

    /// Fingerprint width in bits. Every appended segment must match it.
    pub fn nbits(&self) -> usize {
        self.nbits
    }

    /// Total rows across all segments.
    pub fn len(&self) -> u64 {
        self.total
    }

    /// True if no segment holds any row.
    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
    /// Number of segments (one per non-empty append).
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Append a batch of fingerprints as a new segment. `flat_words` is
    /// `ids.len() * lanes_for_bits(nbits)` lanes in row order; `ids[i]` names the
    /// `i`-th fingerprint.
    ///
    /// # Panics
    /// Panics on the same preconditions as [`Arena::build`], including a
    /// zero-width index.
    pub fn append(&mut self, flat_words: Vec<u64>, ids: Vec<String>) {
        self.append_arena(Arena::build(self.nbits, flat_words, ids));
    }

    /// Append a pre-built [`Arena`] as a segment (must match the index width).
    /// Empty arenas are ignored so `base`/`segments` stay aligned and non-empty.
    ///
    /// # Panics
    /// Panics if `arena.nbits() != self.nbits()`, or if the total row count would
    /// exceed `u64::MAX`.
    pub fn append_arena(&mut self, arena: Arena) {
        assert_eq!(arena.nbits(), self.nbits, "segment width != index width");
        if arena.is_empty() {
            return;
        }
        let new_total = self
            .total
            .checked_add(arena.len() as u64)
            .expect("segmented index overflowed u64 rows");
        let bitmap = InvertedBitmap::build(&arena);
        self.base.push(self.total);
        self.total = new_total;
        self.segments.push(Segment { arena, bitmap });
    }

    /// All rows with Tanimoto `>= t`, as global hits: the same hit set as one
    /// arena over the same rows. One `scratch` serves every segment.
    ///
    /// # Panics
    /// Panics if `query` is not this index's fingerprint width.
    #[must_use]
    pub fn search_threshold(
        &self,
        query: &[u64],
        t: f64,
        scratch: &mut BitmapScratch,
    ) -> Vec<GlobalHit> {
        // Checked here too, so the panic also happens with no segments.
        assert_eq!(
            query.len(),
            lanes_for_bits(self.nbits),
            "query width != index width"
        );
        let mut out = Vec::new();
        for (s, seg) in self.segments.iter().enumerate() {
            let base = self.base[s];
            for h in seg.bitmap.search_threshold(query, t, scratch) {
                out.push(GlobalHit {
                    score: h.score,
                    index: base + h.index as u64,
                });
            }
        }
        out
    }

    /// Exact k-nearest-neighbours across all segments, best first.
    ///
    /// Returns the same rows, in the same order, as [`Arena::search_knn`] over one
    /// arena built from every batch in append order. Each segment returns its top
    /// `k` under that order (score, then popcount, then input order), so the global
    /// top `k` is in their union; merging by the same key (global id follows
    /// append order) picks it.
    ///
    /// # Panics
    /// Panics if `query` is not this index's fingerprint width.
    #[must_use]
    pub fn search_knn(&self, query: &[u64], k: usize) -> Vec<GlobalHit> {
        assert_eq!(
            query.len(),
            lanes_for_bits(self.nbits),
            "query width != index width"
        );
        if k == 0 {
            return Vec::new();
        }
        let mut all: Vec<(GlobalHit, u16)> = Vec::new();
        for (s, seg) in self.segments.iter().enumerate() {
            let base = self.base[s];
            for h in seg.arena.search_knn(query, k) {
                let hit = GlobalHit {
                    score: h.score,
                    index: base + h.index as u64,
                };
                all.push((hit, seg.arena.popcount_of(h.index as usize)));
            }
        }
        // Distinct Tanimoto fractions stay distinct in `f64` at every supported
        // width, so this compares exact fractions.
        all.sort_by(|(a, pa), (b, pb)| {
            b.score
                .total_cmp(&a.score)
                .then(pa.cmp(pb))
                .then(a.index.cmp(&b.index))
        });
        all.truncate(k);
        all.into_iter().map(|(h, _)| h).collect()
    }

    /// External identifier of a global row index.
    ///
    /// # Panics
    /// Panics if `global >= self.len()`.
    #[must_use]
    pub fn id(&self, global: u64) -> String {
        assert!(
            global < self.total,
            "global id {global} out of range (len = {})",
            self.total
        );
        let s = match self.base.binary_search(&global) {
            Ok(s) => s,
            Err(s) => s.saturating_sub(1),
        };
        let local = (global - self.base[s]) as usize;
        self.segments[s].arena.id(local)
    }
}
