//! Immutable, popcount-sorted in-memory fingerprint arena.

use mossflux_core::{lanes_for_bits, popcount};
use rayon::prelude::*;

/// The widest fingerprint an [`Arena`] holds, in bits: popcounts are stored as
/// `u16`, so the lane count times 64 must fit in `u16` (1,023 lanes).
pub const MAX_NBITS: usize = (u16::MAX as usize / 64) * 64;

/// External identifiers for arena rows.
enum IdStore {
    /// Real string identifiers, one per row, in popcount-sorted order.
    Strings(Vec<String>),
    /// Synthetic: the identifier of row `i` is `i` (no per-row storage).
    Indexed,
}

/// A set of equal-width fingerprints stored contiguously and sorted ascending
/// by popcount, with a per-popcount bin index for BitBound pruning.
///
/// Layout:
/// - `words`: `n * nwords` packed `u64` lanes, row-major, popcount-ascending.
/// - `popcounts[i]`: popcount of row `i` (equal to its bin index).
/// - `ids`: external identifiers (real strings, or implicit row indices).
/// - `popcount_index`: rows with popcount exactly `b` occupy
///   `[popcount_index[b], popcount_index[b + 1])`.
pub struct Arena {
    nbits: usize,
    nwords: usize,
    n: usize,
    words: Vec<u64>,
    popcounts: Vec<u16>,
    ids: IdStore,
    popcount_index: Vec<u32>,
}

impl Arena {
    /// Build an arena from unsorted, packed fingerprints with string ids.
    ///
    /// `flat_words` is `ids.len() * lanes_for_bits(nbits)` lanes in row order;
    /// `ids[i]` names the `i`-th fingerprint.
    ///
    /// # Panics
    /// Panics if `nbits == 0`, if
    /// `flat_words.len()` is not a multiple of `lanes_for_bits(nbits)`, if
    /// `ids.len()` does not equal the row count, or if there are more than
    /// `u32::MAX` rows.
    #[must_use]
    pub fn build(nbits: usize, flat_words: Vec<u64>, ids: Vec<String>) -> Arena {
        Self::assemble(nbits, flat_words, Some(ids)).0
    }

    /// Like [`build`](Self::build), but also returns the popcount-sort permutation
    /// `order`, where `order[sorted_row] = input_row`.
    ///
    /// A caller with a parallel per-row payload (e.g. molecules for substructure
    /// search) permutes it by `order` so it aligns with the arena rows.
    ///
    /// # Panics
    /// Same preconditions as [`build`](Self::build).
    #[must_use]
    pub fn build_with_order(
        nbits: usize,
        flat_words: Vec<u64>,
        ids: Vec<String>,
    ) -> (Arena, Vec<u32>) {
        Self::assemble(nbits, flat_words, Some(ids))
    }

    /// Build a synthetic random arena for benchmarking. Each bit is set
    /// independently with probability `density`; identifiers are row indices.
    ///
    /// # Panics
    /// Panics if `nbits == 0`, or if `n > u32::MAX`.
    #[must_use]
    pub fn synthetic(n: usize, nbits: usize, density: f64, seed: u64) -> Arena {
        let nwords = lanes_for_bits(nbits);
        let thresh = (density.clamp(0.0, 1.0) * u64::MAX as f64) as u64;
        let mut flat = vec![0u64; n * nwords];
        flat.par_chunks_mut(nwords)
            .enumerate()
            .for_each(|(i, row)| {
                let mut rng = (seed ^ (i as u64).wrapping_mul(0x9E3779B97F4A7C15)) | 1;
                for j in 0..nbits {
                    rng = rng
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    if rng < thresh {
                        row[j / 64] |= 1u64 << (j % 64);
                    }
                }
            });
        Self::assemble(nbits, flat, None).0
    }

    /// Fold to `new_nbits` bits (OR-fold: folded bit `i` = OR over `k` of original
    /// bit `i + k*new_nbits`). Identifiers become row indices.
    ///
    /// # Panics
    /// Panics unless `0 < new_nbits <= self.nbits()`.
    #[must_use]
    pub fn folded(&self, new_nbits: usize) -> Arena {
        assert!(
            new_nbits > 0 && new_nbits <= self.nbits,
            "fold target must be in (0, nbits]"
        );
        let new_nwords = lanes_for_bits(new_nbits);
        let mut flat = vec![0u64; self.n * new_nwords];
        for i in 0..self.n {
            let dst = &mut flat[i * new_nwords..(i + 1) * new_nwords];
            for (wi, &word) in self.row(i).iter().enumerate() {
                let mut bits = word;
                while bits != 0 {
                    let b = wi * 64 + bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    if b < self.nbits {
                        let fb = b % new_nbits;
                        dst[fb / 64] |= 1u64 << (fb % 64);
                    }
                }
            }
        }
        Self::assemble(new_nbits, flat, None).0
    }

    /// Assemble a popcount-sorted arena, returning it together with the sort
    /// permutation `order` (`order[sorted_row] = input_row`).
    fn assemble(
        nbits: usize,
        mut flat_words: Vec<u64>,
        string_ids: Option<Vec<String>>,
    ) -> (Arena, Vec<u32>) {
        // Zero width would divide by zero below; fail with a clear message.
        assert!(nbits > 0, "fingerprint width must be > 0 bits");
        let nwords = lanes_for_bits(nbits);
        let n = flat_words.len() / nwords;
        assert_eq!(
            flat_words.len(),
            n * nwords,
            "flat_words not a multiple of nwords"
        );
        assert!(
            n <= u32::MAX as usize,
            "arena caps at 2^32 rows; shard beyond that"
        );
        // Mask spare high bits (positions >= nbits). A set spare bit would bin the
        // row above `nbits`, where the band scan (hi clamped to nbits) never looks,
        // and the row would be missed.
        let rem = nbits % 64;
        if rem != 0 && nwords > 0 {
            let mask = (1u64 << rem) - 1;
            for r in 0..n {
                flat_words[r * nwords + (nwords - 1)] &= mask;
            }
        }
        if let Some(ids) = &string_ids {
            assert_eq!(ids.len(), n, "ids length must equal row count");
        }
        // Popcounts are u16; the bin index is sized to the lane capacity (nwords*64).
        let max_popcount = nwords * 64;
        assert!(
            max_popcount <= u16::MAX as usize,
            "fingerprint too wide: {max_popcount}-bit popcount exceeds u16"
        );

        // Per-row popcount (parallel).
        let pop: Vec<u16> = (0..n)
            .into_par_iter()
            .map(|i| popcount(&flat_words[i * nwords..(i + 1) * nwords]) as u16)
            .collect();

        // Stable counting sort by popcount: equal popcounts keep input order. The
        // prefix-summed histogram is the bin index (`popcount_index[b]` = rows with
        // popcount < b).
        let mut popcount_index = vec![0u32; max_popcount + 2];
        for &p in &pop {
            popcount_index[p as usize + 1] += 1;
        }
        for b in 1..popcount_index.len() {
            popcount_index[b] += popcount_index[b - 1];
        }
        // Scatter row indices into sorted order via a per-bin cursor.
        let mut cursor: Vec<u32> = popcount_index[..=max_popcount].to_vec();
        let mut order = vec![0u32; n];
        for (i, &p) in pop.iter().enumerate() {
            let slot = cursor[p as usize];
            order[slot as usize] = i as u32;
            cursor[p as usize] = slot + 1;
        }

        // Gather rows and popcounts into sorted order (parallel, disjoint writes).
        let mut words = vec![0u64; n * nwords];
        let mut popcounts = vec![0u16; n];
        words
            .par_chunks_mut(nwords)
            .zip(popcounts.par_iter_mut())
            .enumerate()
            .for_each(|(new_i, (wrow, pc))| {
                let oi = order[new_i] as usize;
                wrow.copy_from_slice(&flat_words[oi * nwords..(oi + 1) * nwords]);
                *pc = pop[oi];
            });

        // The id permutation stays serial (it moves each String exactly once).
        let ids = match string_ids {
            Some(v) => {
                let mut opt: Vec<Option<String>> = v.into_iter().map(Some).collect();
                let sorted: Vec<String> = order
                    .iter()
                    .map(|&oi| opt[oi as usize].take().expect("each id moved once"))
                    .collect();
                IdStore::Strings(sorted)
            }
            None => IdStore::Indexed,
        };

        (
            Arena {
                nbits,
                nwords,
                n,
                words,
                popcounts,
                ids,
                popcount_index,
            },
            order,
        )
    }

    /// Number of fingerprints.
    #[inline]
    pub fn len(&self) -> usize {
        self.n
    }

    /// True if the arena holds no fingerprints.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Fingerprint width in bits.
    #[inline]
    pub fn nbits(&self) -> usize {
        self.nbits
    }

    /// Number of `u64` lanes per fingerprint.
    #[inline]
    pub fn nwords(&self) -> usize {
        self.nwords
    }

    /// Packed lanes of row `idx`.
    ///
    /// # Panics
    /// Panics if `idx >= self.len()` (via the slice bound only; this is the
    /// innermost search read, so callers own the range check).
    #[inline]
    pub fn row(&self, idx: usize) -> &[u64] {
        &self.words[idx * self.nwords..(idx + 1) * self.nwords]
    }

    /// Popcount of row `idx` (also its bin index).
    ///
    /// # Panics
    /// Panics if `idx >= self.len()`.
    #[inline]
    pub fn popcount_of(&self, idx: usize) -> u16 {
        self.popcounts[idx]
    }

    /// External identifier of row `idx`.
    ///
    /// # Panics
    /// Panics if `idx >= self.len()` and the arena stores string ids. A
    /// synthetic arena returns `idx.to_string()` without a bounds check.
    #[inline]
    pub fn id(&self, idx: usize) -> String {
        match &self.ids {
            IdStore::Strings(v) => v[idx].clone(),
            IdStore::Indexed => idx.to_string(),
        }
    }

    /// Half-open row range `[start, end)` of fingerprints whose popcount is
    /// exactly `bin`.
    #[inline]
    pub(crate) fn bin_range(&self, bin: usize) -> (usize, usize) {
        (
            self.popcount_index[bin] as usize,
            self.popcount_index[bin + 1] as usize,
        )
    }

    /// Bin-start offsets: rows with popcount exactly `b` occupy
    /// `[offsets[b], offsets[b + 1])`. Length `nwords*64 + 2`.
    #[inline]
    pub(crate) fn popcount_offsets(&self) -> &[u32] {
        &self.popcount_index
    }
}
