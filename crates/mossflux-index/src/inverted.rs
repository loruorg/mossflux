//! Bit-transposed inverted indexes for Tanimoto similarity.
//!
//! Each bit-column `j` lists the records that set bit `j`. A query with popcount
//! `a` touches only its `a` set columns, so it reads a small fraction of the data.
//!
//! [`InvertedIndex`] accumulates the intersection popcount per touched record
//! from posting lists; [`InvertedBitmap`] keeps the counts bit-sliced over dense
//! column bitmaps. Both apply the linear path's integer pre-filter and exact
//! `score >= t` gate, and return the same hits as
//! [`crate::Arena::search_threshold`].

use crate::arena::Arena;
use crate::column::ColumnBitmap;
use crate::search::{
    all_rows_score_zero, bitbound_bins, query_popcount, threshold_to_rational, Hit,
};

/// CSR-encoded posting lists: column `j`'s record ids are
/// `col_ids[col_offsets[j] .. col_offsets[j + 1]]`.
pub struct InvertedIndex {
    n: usize,
    nbits: usize,
    col_offsets: Vec<u64>,
    col_ids: Vec<u32>,
    pop_b: Vec<u16>,
}

/// Reusable per-query scratch: an intersection-count accumulator and a touched
/// list. One per worker thread; the touched list resets the accumulator to zero
/// after each query.
pub struct InvertedScratch {
    acc: Vec<u16>,
    touched: Vec<u32>,
}

impl InvertedScratch {
    /// Scratch sized for an index of `n` rows.
    ///
    /// `n` must be at least the `len()` of every [`InvertedIndex`] this scratch is
    /// passed to; [`InvertedIndex::search_threshold`] asserts it.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            acc: vec![0u16; n],
            touched: Vec::new(),
        }
    }
}

impl InvertedIndex {
    /// Build the inverted index from a popcount-sorted [`Arena`]. Record ids are
    /// `u32` arena row indices.
    ///
    /// # Panics
    /// Panics if `arena.len() > u32::MAX`.
    #[must_use]
    pub fn build(arena: &Arena) -> InvertedIndex {
        let n = arena.len();
        let nbits = arena.nbits();
        assert!(
            n <= u32::MAX as usize,
            "inverted index caps at 2^32 rows; shard beyond that"
        );

        // Pass 1: column lengths (the per-column popcount).
        let mut col_len = vec![0u64; nbits];
        for i in 0..n {
            for (wi, &w) in arena.row(i).iter().enumerate() {
                let mut bits = w;
                while bits != 0 {
                    let j = wi * 64 + bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    if j < nbits {
                        col_len[j] += 1;
                    }
                }
            }
        }
        let mut col_offsets = vec![0u64; nbits + 1];
        for j in 0..nbits {
            col_offsets[j + 1] = col_offsets[j] + col_len[j];
        }
        let total = col_offsets[nbits] as usize;

        // Pass 2: scatter record ids into their columns.
        let mut col_ids = vec![0u32; total];
        let mut cursor: Vec<u64> = col_offsets[..nbits].to_vec();
        for i in 0..n {
            for (wi, &w) in arena.row(i).iter().enumerate() {
                let mut bits = w;
                while bits != 0 {
                    let j = wi * 64 + bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    if j < nbits {
                        col_ids[cursor[j] as usize] = i as u32;
                        cursor[j] += 1;
                    }
                }
            }
        }

        let pop_b = (0..n).map(|i| arena.popcount_of(i)).collect();
        InvertedIndex {
            n,
            nbits,
            col_offsets,
            col_ids,
            pop_b,
        }
    }

    /// Total number of postings (= sum of all fingerprint popcounts).
    pub fn total_postings(&self) -> u64 {
        *self.col_offsets.last().unwrap()
    }

    /// Number of indexed fingerprints.
    pub fn len(&self) -> usize {
        self.n
    }

    /// True if no fingerprints are indexed.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Approximate resident size in bytes.
    pub fn mem_bytes(&self) -> usize {
        self.col_ids.len() * 4 + self.col_offsets.len() * 8 + self.pop_b.len() * 2
    }

    /// All fingerprints with Tanimoto `>= t` against `query`. Exact: the same hit
    /// set as the linear scan.
    ///
    /// # Panics
    /// Panics if `query` is not this index's fingerprint width, or if `scratch`
    /// was built for fewer rows than this index holds.
    #[must_use]
    pub fn search_threshold(
        &self,
        query: &[u64],
        t: f64,
        scratch: &mut InvertedScratch,
    ) -> Vec<Hit> {
        assert_eq!(
            query.len(),
            mossflux_core::lanes_for_bits(self.nbits),
            "query width != index width"
        );
        assert!(
            scratch.acc.len() >= self.n,
            "InvertedScratch sized for {} rows, index holds {}",
            scratch.acc.len(),
            self.n
        );
        let mut hits = Vec::new();
        if t.is_nan() || t > 1.0 {
            return hits;
        }
        let a = query_popcount(query, self.nbits);
        if a == 0 {
            return if t <= 0.0 {
                all_rows_score_zero(self.n)
            } else {
                hits
            };
        }
        let (num, den) = threshold_to_rational(t);
        let lhs_mul = (den + num) as u64;
        let rhs_mul = num as u64;

        let acc = &mut scratch.acc;
        let touched = &mut scratch.touched;
        touched.clear();

        // Accumulate intersection counts over the query's set-bit columns.
        for (wi, &w) in query.iter().enumerate() {
            let mut bits = w;
            while bits != 0 {
                let j = wi * 64 + bits.trailing_zeros() as usize;
                bits &= bits - 1;
                if j >= self.nbits {
                    continue;
                }
                let start = self.col_offsets[j] as usize;
                let end = self.col_offsets[j + 1] as usize;
                for &r in &self.col_ids[start..end] {
                    let ru = r as usize;
                    if acc[ru] == 0 {
                        touched.push(r);
                    }
                    acc[ru] += 1;
                }
            }
        }

        // t <= 0 matches every row; acc holds the exact intersection count
        // (0 for untouched rows), so emit all rows with accurate scores.
        if t <= 0.0 {
            let mut all = Vec::with_capacity(self.n);
            for (r, (&accr, &popr)) in acc.iter().zip(self.pop_b.iter()).enumerate().take(self.n) {
                let c = accr as u64;
                let bpop = popr as u64;
                let denom = a as u64 + bpop - c;
                let score = if denom == 0 {
                    0.0
                } else {
                    c as f64 / denom as f64
                };
                all.push(Hit {
                    score,
                    index: r as u32,
                });
            }
            for &r in touched.iter() {
                acc[r as usize] = 0;
            }
            return all;
        }

        // Evaluate touched candidates, resetting the accumulator as we go.
        for &r in touched.iter() {
            let ru = r as usize;
            let c = acc[ru] as u64;
            acc[ru] = 0;
            let bpop = self.pop_b[ru] as u64;
            // Loose integer pre-filter (num/den <= t), then exact f64 gate.
            if c * lhs_mul >= rhs_mul * (a as u64 + bpop) {
                let denom = a as u64 + bpop - c;
                let score = c as f64 / denom as f64;
                if score >= t {
                    hits.push(Hit { score, index: r });
                }
            }
        }
        hits
    }
}

/// Bit-transposed index storing each column as a dense bitmap over records.
///
/// Intersection counts are kept bit-sliced in `ceil(log2(a))` plane bitmaps, and
/// each query column is ripple-added into them 64 records at a time, with no
/// scatter. The arena is popcount-sorted, so the BitBound band is a contiguous
/// row range: the sweep reads only the band's words of the query's columns.
///
/// The [`ColumnBitmap`] is the same size as the arena's fingerprints. Record ids
/// are `u32`.
pub struct InvertedBitmap {
    /// Bit-transposed dense column store.
    columns: ColumnBitmap,
    pop_b: Vec<u16>,
    /// Copy of the arena popcount offsets, mapping a popcount band to a row range.
    band_index: Vec<u32>,
}

/// Reusable per-query scratch (bit-planes, carry buffer, live-word list), grown on
/// demand. One per worker thread.
pub struct BitmapScratch {
    planes: Vec<u64>,
    carry: Vec<u64>,
    live: Vec<usize>,
    words_read: u64,
    /// Early-abandon override: `Some(true/false)` forces it on/off; `None` uses the
    /// default (off).
    force_abandon: Option<bool>,
    /// Test/tuning override of the cache-block word count.
    block_override: Option<usize>,
}

impl Default for BitmapScratch {
    fn default() -> Self {
        Self::new()
    }
}

impl BitmapScratch {
    /// An empty scratch. Unlike [`InvertedScratch`] it has no size contract: its
    /// buffers grow to fit any index.
    #[must_use]
    pub fn new() -> Self {
        Self {
            planes: Vec::new(),
            carry: Vec::new(),
            live: Vec::new(),
            words_read: 0,
            force_abandon: None,
            block_override: None,
        }
    }
    /// Force the early-abandon path on/off (`None` = default, off).
    pub fn set_force_abandon(&mut self, v: Option<bool>) {
        self.force_abandon = v;
    }
    /// Override the cache-block size in words (for tests that need many blocks).
    pub fn set_block_override(&mut self, v: Option<usize>) {
        self.block_override = v;
    }
    #[inline]
    fn ensure(&mut self, plane_words: usize, carry_words: usize) {
        if self.planes.len() < plane_words {
            self.planes.resize(plane_words, 0);
        }
        if self.carry.len() < carry_words {
            self.carry.resize(carry_words, 0);
        }
        if self.live.len() < carry_words {
            self.live.resize(carry_words, 0);
        }
    }
    /// Column words read since the last reset (times 8 = bytes).
    pub fn words_read(&self) -> u64 {
        self.words_read
    }

    /// Zeroes the [`words_read`](Self::words_read) counter, starting a fresh
    /// measurement window.
    pub fn reset_words_read(&mut self) {
        self.words_read = 0;
    }
}

/// True iff any of word `w`'s 64 records has a bit-sliced count `>= thr`.
#[inline]
fn word_ge(planes: &[u64], wpc: usize, w: usize, thr: usize, p: usize) -> bool {
    let mut gt = 0u64;
    let mut eq = !0u64;
    for k in (0..p).rev() {
        let vbit = planes[k * wpc + w];
        if (thr >> k) & 1 == 1 {
            eq &= vbit;
        } else {
            gt |= eq & vbit;
            eq &= !vbit;
        }
    }
    (gt | eq) != 0
}

/// Extract the hits from one 64-record word `w`: bit-sliced `count >= cmin` mask,
/// band-boundary masking, then the exact integer pre-filter + f64 `score >= t` gate.
#[allow(clippy::too_many_arguments)]
#[inline]
fn extract_word(
    planes: &[u64],
    wpc: usize,
    w: usize,
    p: usize,
    cmin: usize,
    row_lo: usize,
    row_hi: usize,
    lhs_mul: u64,
    rhs_mul: u64,
    a: u32,
    pop_b: &[u16],
    t: f64,
    hits: &mut Vec<Hit>,
) {
    let mut gt = 0u64;
    let mut eq = !0u64;
    for k in (0..p).rev() {
        let vbit = planes[k * wpc + w];
        if (cmin >> k) & 1 == 1 {
            eq &= vbit;
        } else {
            gt |= eq & vbit;
            eq &= !vbit;
        }
    }
    let mut cand = gt | eq;
    let word_first_row = w * 64;
    if word_first_row < row_lo {
        cand &= !((1u64 << (row_lo - word_first_row)) - 1);
    }
    if word_first_row + 64 > row_hi {
        let valid = row_hi - word_first_row;
        if valid < 64 {
            cand &= (1u64 << valid) - 1;
        }
    }
    while cand != 0 {
        let pbit = cand.trailing_zeros() as usize;
        cand &= cand - 1;
        let r = w * 64 + pbit;
        let mut c = 0u64;
        for k in 0..p {
            c |= ((planes[k * wpc + w] >> pbit) & 1) << k;
        }
        let b = pop_b[r] as u64;
        if c * lhs_mul >= rhs_mul * (a as u64 + b) {
            let denom = a as u64 + b - c;
            let score = c as f64 / denom as f64;
            if score >= t {
                hits.push(Hit {
                    score,
                    index: r as u32,
                });
            }
        }
    }
}

impl InvertedBitmap {
    /// Transpose a popcount-sorted [`Arena`] into dense column bitmaps, with the
    /// per-row popcounts and band offsets.
    #[must_use]
    pub fn build(arena: &Arena) -> InvertedBitmap {
        let columns = ColumnBitmap::build(arena);
        let pop_b = (0..arena.len()).map(|i| arena.popcount_of(i)).collect();
        let band_index = arena.popcount_offsets().to_vec();
        InvertedBitmap {
            columns,
            pop_b,
            band_index,
        }
    }

    /// The shared bit-transposed column store backing this similarity index.
    #[inline]
    pub fn columns(&self) -> &ColumnBitmap {
        &self.columns
    }

    /// Approximate resident size in bytes.
    pub fn mem_bytes(&self) -> usize {
        self.columns.mem_bytes() + self.pop_b.len() * 2 + self.band_index.len() * 4
    }

    /// Number of indexed fingerprints.
    pub fn len(&self) -> usize {
        self.columns.n()
    }

    /// True if no fingerprints are indexed.
    pub fn is_empty(&self) -> bool {
        self.columns.n() == 0
    }

    /// All fingerprints with Tanimoto `>= t` against `query`. Exact: the same hit
    /// set as the linear scan.
    ///
    /// # Panics
    /// Panics if `query` is not this index's fingerprint width.
    #[must_use]
    pub fn search_threshold(&self, query: &[u64], t: f64, scratch: &mut BitmapScratch) -> Vec<Hit> {
        assert_eq!(
            query.len(),
            mossflux_core::lanes_for_bits(self.columns.nbits()),
            "query width != index width"
        );
        let mut hits = Vec::new();
        if t.is_nan() || t > 1.0 {
            return hits;
        }
        let a = query_popcount(query, self.columns.nbits());
        if a == 0 {
            return if t <= 0.0 {
                all_rows_score_zero(self.columns.n())
            } else {
                hits
            };
        }
        let cols = self.columns.cols();
        let wpc = self.columns.wpc();
        let nbits = self.columns.nbits();
        let (num, den) = threshold_to_rational(t);
        let (lo, hi) = bitbound_bins(a, num, den, nbits);
        if lo > hi {
            return hits;
        }
        // BitBound band -> contiguous row range -> word range in the bitmaps.
        let row_lo = self.band_index[lo as usize] as usize;
        let row_hi = self.band_index[hi as usize + 1] as usize; // exclusive
        if row_lo >= row_hi {
            return hits;
        }
        let w_lo = row_lo / 64;
        let w_hi = (row_hi - 1) / 64; // inclusive

        // Minimum intersection any record could need (using the smallest band
        // popcount `lo`); records below this can't pass for any popcount.
        let denpnum = (den + num) as u64;
        let cmin_raw = (num as u64 * (a as u64 + lo as u64)).div_ceil(denpnum);
        if cmin_raw > a as u64 {
            return hits;
        }
        // num == 0 (t <= 0): cmin = 0 so zero-intersection rows stay candidates;
        // the exact f64 gate then keeps them iff score >= t.
        let cmin = if num == 0 {
            0
        } else {
            (cmin_raw as usize).max(1)
        };

        // Bit-planes needed to hold a count up to `a`.
        let p = (usize::BITS - (a as usize).leading_zeros()) as usize;
        let band_words = w_hi - w_lo + 1;
        let force_abandon = scratch.force_abandon;
        // Cache-block the band so a block's p bit-planes stay in L2 across the
        // column ripples.
        let block = scratch
            .block_override
            .unwrap_or(65536 / p.max(1))
            .max(1)
            .min(band_words);
        scratch.ensure(p * wpc, block);
        let planes = &mut scratch.planes;
        let live = &mut scratch.live;

        // Query's set-bit columns, rarest first, so records fall below cmin earliest.
        let mut qcols: Vec<usize> = Vec::with_capacity(a as usize);
        for (wi, &word) in query.iter().enumerate() {
            let mut bits = word;
            while bits != 0 {
                let j = wi * 64 + bits.trailing_zeros() as usize;
                bits &= bits - 1;
                if j < nbits {
                    qcols.push(j * wpc);
                }
            }
        }
        qcols.sort_by_key(|&base| self.columns.col_popcount(base / wpc));
        let ncols = qcols.len();
        // Early-abandon reads fewer bytes but runs a scalar Phase B. In RAM the
        // kernel is compute-bound, so it is off unless forced; it pays only when
        // bandwidth-bound.
        let use_abandon = force_abandon.unwrap_or(false);
        // Columns before this can never drop a record (max final count =
        // count_so_far + remaining, and remaining >= cmin until here).
        let abandon_start = if use_abandon {
            ncols.saturating_sub(cmin)
        } else {
            ncols
        };

        let lhs_mul = denpnum;
        let rhs_mul = num as u64;
        const REFRESH_K: usize = 4;
        let mut words: u64 = 0;

        let mut wb = w_lo;
        while wb <= w_hi {
            let blen = block.min(w_hi - wb + 1);
            // Phase-A strips overwrite their region wholesale (copy_from_slice), so
            // only the final < 8-word scalar tail is rippled-into and needs zeroing.
            let strip_end = wb + (blen / 8) * 8;
            for k in 0..p {
                planes[k * wpc + strip_end..k * wpc + wb + blen].fill(0);
            }
            // Phase A: register-strip ripple. The p bit-planes of one 8-word strip
            // accumulate in a stack-local `ps` across all columns, so `planes` is
            // written once per strip. Exact: the same bit-sliced sums, reordered
            // over independent record lanes. The carry out of plane p-1 is always 0
            // because counts stay < 2^p.
            const PMAX: usize = 16; // p <= bit_length(popcount) <= 16 for any fp width
            let acols = abandon_start;
            let mut ib = wb;
            while ib < strip_end {
                let mut ps = [[0u64; 8]; PMAX];
                for &base in qcols.iter().take(acols) {
                    let mut cy: [u64; 8] = cols[base + ib..base + ib + 8].try_into().unwrap();
                    for psk in ps.iter_mut().take(p) {
                        for l in 0..8 {
                            let pk = psk[l];
                            psk[l] = pk ^ cy[l];
                            cy[l] &= pk;
                        }
                    }
                }
                for k in 0..p {
                    planes[k * wpc + ib..k * wpc + ib + 8].copy_from_slice(&ps[k]);
                }
                ib += 8;
            }
            // Scalar tail for the final < 8 words, rippled into the planes zeroed
            // above.
            for &base in qcols.iter().take(acols) {
                for w in strip_end..wb + blen {
                    let mut cy = cols[base + w];
                    let mut k = 0;
                    while cy != 0 && k < p {
                        let idx = k * wpc + w;
                        let pk = planes[idx];
                        planes[idx] = pk ^ cy;
                        cy &= pk;
                        k += 1;
                    }
                }
            }
            words += acols as u64 * blen as u64;

            // Phase B: word-granular early-abandon over the remaining columns. A
            // word is dropped only once all 64 records are provably dead
            // (count_so_far + remaining < cmin), so no hit is lost.
            let livew = &mut live[..blen];
            for (i, w) in (wb..wb + blen).enumerate() {
                livew[i] = w;
            }
            let mut nlive = blen;
            for (s, &base) in qcols.iter().enumerate().skip(abandon_start) {
                if nlive == 0 {
                    break;
                }
                if s > abandon_start && (s - abandon_start) % REFRESH_K == 0 {
                    let thr = cmin as i64 - (ncols - s) as i64;
                    if thr > 0 {
                        let thru = thr as usize;
                        let mut kept = 0;
                        for idx in 0..nlive {
                            let w = livew[idx];
                            if word_ge(planes, wpc, w, thru, p) {
                                livew[kept] = w;
                                kept += 1;
                            }
                        }
                        nlive = kept;
                        if nlive == 0 {
                            break;
                        }
                    }
                }
                for &w in &livew[..nlive] {
                    let mut cy = cols[base + w];
                    let mut k = 0;
                    while cy != 0 && k < p {
                        let idx = k * wpc + w;
                        let pk = planes[idx];
                        planes[idx] = pk ^ cy;
                        cy &= pk;
                        k += 1;
                    }
                }
                words += nlive as u64;
            }

            // Extract from the surviving words (dropped words have no candidate).
            for &w in &livew[..nlive] {
                extract_word(
                    planes,
                    wpc,
                    w,
                    p,
                    cmin,
                    row_lo,
                    row_hi,
                    lhs_mul,
                    rhs_mul,
                    a,
                    &self.pop_b,
                    t,
                    &mut hits,
                );
            }
            wb += blen;
        }
        scratch.words_read += words;
        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut h: Vec<Hit>) -> Vec<u32> {
        h.sort_by_key(|x| x.index);
        h.into_iter().map(|x| x.index).collect()
    }

    #[test]
    fn inverted_matches_linear() {
        let arena = Arena::synthetic(4000, 256, 0.12, 123);
        let inv = InvertedIndex::build(&arena);
        let mut scratch = InvertedScratch::new(arena.len());
        for &t in &[0.3_f64, 0.5, 0.7, 0.85] {
            for q in (0..arena.len()).step_by(91) {
                let query = arena.row(q).to_vec();
                let lin = sorted(arena.search_threshold(&query, t));
                let invh = sorted(inv.search_threshold(&query, t, &mut scratch));
                assert_eq!(lin, invh, "inverted vs linear mismatch at t={t}, q={q}");
            }
        }
    }

    #[test]
    fn inverted_matches_linear_wider() {
        // Sparser, wider fingerprints (closer to ECFP4 2048-bit density).
        let arena = Arena::synthetic(3000, 512, 0.05, 7);
        let inv = InvertedIndex::build(&arena);
        let mut scratch = InvertedScratch::new(arena.len());
        for &t in &[0.6_f64, 0.7, 0.8, 0.9] {
            for q in (0..arena.len()).step_by(123) {
                let query = arena.row(q).to_vec();
                assert_eq!(
                    sorted(arena.search_threshold(&query, t)),
                    sorted(inv.search_threshold(&query, t, &mut scratch)),
                    "mismatch at t={t}, q={q}"
                );
            }
        }
    }

    #[test]
    fn bitmap_matches_linear() {
        // Vary n (incl. non-64-multiples for boundary-word masking), width,
        // density, and threshold. The bitmap index must return exactly the
        // linear scan's hits.
        for &(n, nb, d, seed) in &[
            (4000usize, 256usize, 0.12f64, 1u64),
            (3000, 512, 0.05, 9),
            (2000, 256, 0.20, 3),
            (4097, 256, 0.10, 5), // n not a multiple of 64
            (1, 256, 0.10, 2),    // single record
            (130, 128, 0.15, 8),  // just over two words
        ] {
            let arena = Arena::synthetic(n, nb, d, seed);
            let bm = InvertedBitmap::build(&arena);
            let mut sc = BitmapScratch::new();
            for &t in &[0.3_f64, 0.5, 0.7, 0.8, 0.85, 0.9, 0.95] {
                for q in (0..arena.len()).step_by(71) {
                    let query = arena.row(q).to_vec();
                    let lin = sorted(arena.search_threshold(&query, t));
                    let b = sorted(bm.search_threshold(&query, t, &mut sc));
                    assert_eq!(
                        lin, b,
                        "bitmap vs linear mismatch n={n} nb={nb} t={t} q={q}"
                    );
                }
            }
        }
    }
}
