//! Exact BitBound similarity search over an [`Arena`].
//!
//! Two pruning ideas combine, both exact (zero false negatives):
//! 1. **Popcount band** — a target with popcount `b` can reach Tanimoto `>= t`
//!    against a query with popcount `a` only if `a*t <= b <= a/t`. With the
//!    arena sorted by popcount this is a contiguous slice of bins.
//! 2. **Intersection-popcount test** — within a bin, accept a target iff its
//!    intersection popcount `c` satisfies `c*(den+num) >= num*(a+b)` (the
//!    integer form of `c >= t*(a+b)/(1+t)`), so the reject path needs only one
//!    popcount and no division.
//!
//! All bound math is integer/rational: in `f64`, `396 / 0.55 = 719.9999…`, and
//! a plain `floor` would drop valid hits at 720.

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

use mossflux_core::intersection_popcount;

use crate::arena::Arena;

/// A search result: similarity `score` for the fingerprint at row `index`.
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    /// Tanimoto similarity to the query, in `[0.0, 1.0]`: the correctly rounded
    /// `f64` of the exact fraction `c / (a + b - c)`. The threshold gate compares
    /// this value, so a threshold hit always has `score >= t`. At every supported
    /// width two different fractions round to different `f64`s, so ordering by
    /// `score` is ordering by the exact Tanimoto.
    pub score: f64,
    /// Row index of the matching fingerprint within the [`Arena`] that was searched.
    pub index: u32,
}

/// Convert a floating-point Tanimoto threshold in `(0, 1]` to a rational
/// `num/den` that is guaranteed `<= t` (rounds DOWN).
///
/// Rounding down keeps the popcount band and integer pre-filter looser than `t`,
/// so they cannot drop a valid hit; the final `f64` `score >= t` check restores
/// exactness. Rounding to nearest could give `num/den > t` and drop a hit whose
/// Tanimoto equals `t` (e.g. `t = 2/3`).
#[inline]
#[must_use]
pub fn threshold_to_rational(t: f64) -> (u32, u32) {
    const DEN: u32 = 1_000_000;
    // num == 0 (t <= 0, or below the 1e-6 granularity) means "no popcount
    // pruning": the f64 gate alone decides. Clamping num up to 1 would make the
    // band stricter than t.
    let num = (t * DEN as f64).floor().clamp(0.0, DEN as f64) as u32;
    (num, DEN)
}

/// Inclusive popcount-bin band `[lo, hi]` that a target must fall in to possibly
/// reach Tanimoto `>= num/den` against a query of popcount `qpop`.
///
/// `lo = ceil(qpop*num/den)`, `hi = min(nbits, floor(qpop*den/num))`, both exact
/// integer arithmetic.
///
/// # Panics
/// Panics if `den == 0`. [`threshold_to_rational`] always returns
/// `den = 1_000_000`. `den >= num` is only a `debug_assert`; `num > den` yields
/// an empty band.
#[inline]
#[must_use]
pub fn bitbound_bins(qpop: u32, num: u32, den: u32, nbits: usize) -> (u32, u32) {
    assert!(
        den != 0,
        "bitbound_bins: den must be non-zero (it is the divisor)"
    );
    debug_assert!(den >= num, "threshold num/den must be <= 1");
    if num == 0 {
        // t <= 0 (or below the granularity): no pruning; the f64 gate filters.
        // This also avoids dividing by `num` below.
        return (0, nbits as u32);
    }
    let q = qpop as u64;
    let lo = (q * num as u64).div_ceil(den as u64); // ceil
    let hi = ((q * den as u64) / num as u64).min(nbits as u64); // floor, clamped
    (lo as u32, hi as u32)
}

/// Popcount of `query` over its first `nbits` bits only.
///
/// Stored rows never carry bits `>= nbits`, but counted in the query popcount
/// they would lower every score. Every search path uses this function, so all
/// paths agree.
#[inline]
#[must_use]
pub(crate) fn query_popcount(query: &[u64], nbits: usize) -> u32 {
    let mut p = mossflux_core::popcount(query);
    let rem = nbits % 64;
    if rem != 0 {
        if let Some(&last) = query.get(nbits / 64) {
            p -= (last & !((1u64 << rem) - 1)).count_ones();
        }
    }
    p
}

/// Every row of an `n`-row index with score 0: the answer for an empty query at
/// `t <= 0`.
pub(crate) fn all_rows_score_zero(n: usize) -> Vec<Hit> {
    (0..n)
        .map(|i| Hit {
            score: 0.0,
            index: i as u32,
        })
        .collect()
}

/// A kNN candidate: the exact score `c / d` (`d > 0`) of row `index`.
///
/// Ranking compares fractions exactly by cross-multiplication and breaks a tie
/// by the lower row (lower popcount, then earlier input). That rule does not
/// depend on segmentation, so [`crate::SegmentedIndex::search_knn`] returns the
/// same rows as one arena over the same input.
#[derive(Clone, Copy, PartialEq, Eq)]
struct HeapItem {
    c: u32,
    d: u32,
    index: u32,
}
impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HeapItem {
    /// `Greater` means `self` ranks above `other`.
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.c as u64 * other.d as u64;
        let rhs = other.c as u64 * self.d as u64;
        lhs.cmp(&rhs).then(other.index.cmp(&self.index))
    }
}

/// `lo / hi < c / d`, exactly (all operands fit u32, `hi, d > 0`).
#[inline]
fn frac_lt(lo: u32, hi: u32, c: u32, d: u32) -> bool {
    (lo as u64) * (d as u64) < (c as u64) * (hi as u64)
}

impl Arena {
    /// All fingerprints with Tanimoto `>= t` against `query`, using BitBound +
    /// the intersection-popcount test. Unordered.
    ///
    /// # Panics
    /// Panics if `query.len() != self.nwords()`.
    ///
    /// Query bits at positions `>= nbits` are ignored. An empty query matches
    /// every row (score 0) when `t <= 0` and no row otherwise; `t > 1` and
    /// `t = NaN` match no row.
    #[must_use]
    pub fn search_threshold(&self, query: &[u64], t: f64) -> Vec<Hit> {
        assert_eq!(query.len(), self.nwords(), "query width != arena width");
        let mut hits = Vec::new();
        if t.is_nan() || t > 1.0 {
            return hits;
        }
        let qpop = query_popcount(query, self.nbits());
        if qpop == 0 {
            return if t <= 0.0 {
                all_rows_score_zero(self.len())
            } else {
                hits
            };
        }
        let (num, den) = threshold_to_rational(t);
        let (lo, hi) = bitbound_bins(qpop, num, den, self.nbits());
        let lhs_mul = (den + num) as u64;
        for b in lo..=hi {
            let (start, end) = self.bin_range(b as usize);
            if start == end {
                continue;
            }
            let rhs = num as u64 * (qpop as u64 + b as u64);
            for idx in start..end {
                let c = intersection_popcount(query, self.row(idx));
                // Loose integer pre-filter (num/den <= t): admits all true hits
                // plus a few near-misses, no division on rejects.
                if (c as u64) * lhs_mul >= rhs {
                    let denom = qpop + b - c; // a + b - c, always > 0 here
                    let score = c as f64 / denom as f64;
                    // Exact gate against the user's t: removes the near-misses.
                    if score >= t {
                        hits.push(Hit {
                            score,
                            index: idx as u32,
                        });
                    }
                }
            }
        }
        hits
    }

    /// Reference threshold search that scans every fingerprint (no band pruning).
    /// Used to verify that BitBound pruning never drops a hit.
    ///
    /// # Panics
    /// Panics if `query.len() != self.nwords()` (see [`search_threshold`](Self::search_threshold)).
    #[must_use]
    pub fn search_threshold_brute(&self, query: &[u64], t: f64) -> Vec<Hit> {
        assert_eq!(query.len(), self.nwords(), "query width != arena width");
        let mut hits = Vec::new();
        if t.is_nan() || t > 1.0 {
            return hits;
        }
        let qpop = query_popcount(query, self.nbits());
        if qpop == 0 {
            return if t <= 0.0 {
                all_rows_score_zero(self.len())
            } else {
                hits
            };
        }
        let (num, den) = threshold_to_rational(t);
        let lhs_mul = (den + num) as u64;
        for idx in 0..self.len() {
            let b = self.popcount_of(idx) as u32;
            let c = intersection_popcount(query, self.row(idx));
            if (c as u64) * lhs_mul >= num as u64 * (qpop as u64 + b as u64) {
                let denom = qpop + b - c;
                if denom > 0 {
                    let score = c as f64 / denom as f64;
                    if score >= t {
                        hits.push(Hit {
                            score,
                            index: idx as u32,
                        });
                    }
                }
            }
        }
        hits
    }

    /// The `k` nearest neighbours of `query` by Tanimoto, best first.
    ///
    /// Returns the first `min(k, len)` rows of the order "exact score descending,
    /// then row ascending" (a tie goes to the lower popcount, then the earlier
    /// input row), never dependent on scan order. Rows with score 0 fill the
    /// rest when fewer than `k` rows share a bit with the query. Query bits at
    /// positions `>= nbits` are ignored; an empty query returns no rows.
    ///
    /// Scans popcount bins outward from the query popcount and stops once a full
    /// heap's worst hit beats every remaining bin's upper bound.
    ///
    /// # Panics
    /// Panics if `query.len() != self.nwords()` (see [`search_threshold`](Self::search_threshold)).
    #[must_use]
    pub fn search_knn(&self, query: &[u64], k: usize) -> Vec<Hit> {
        assert_eq!(query.len(), self.nwords(), "query width != arena width");
        let a = query_popcount(query, self.nbits());
        if a == 0 || k == 0 || self.is_empty() {
            return Vec::new();
        }
        let nb = self.nbits() as i64;
        let ai = a as i64;
        // Size by min(k, len): `k` alone overflows for a huge `k`.
        let mut heap: BinaryHeap<Reverse<HeapItem>> =
            BinaryHeap::with_capacity(k.min(self.len()) + 1);

        let mut d: i64 = 0;
        loop {
            let mut any_valid = false;
            // Best bound `lo/hi` over this distance's bins (`hi > 0` as `a > 0`).
            let mut max_ub: (u32, u32) = (0, 1);
            // Candidate bins at distance d from the query popcount.
            let candidates = if d == 0 {
                [ai, i64::MIN]
            } else {
                [ai - d, ai + d]
            };
            for &b in &candidates {
                if b < 0 || b > nb {
                    continue;
                }
                any_valid = true;
                let bb = b as u32;
                let ub = (a.min(bb), a.max(bb));
                if frac_lt(max_ub.0, max_ub.1, ub.0, ub.1) {
                    max_ub = ub;
                }
                // Skip only if the worst hit beats the bin's bound strictly: a bin
                // at the bound can hold an equal score on a lower row.
                if heap.len() == k {
                    let w = heap.peek().unwrap().0;
                    if frac_lt(ub.0, ub.1, w.c, w.d) {
                        continue;
                    }
                }
                let (start, end) = self.bin_range(bb as usize);
                for idx in start..end {
                    let c = intersection_popcount(query, self.row(idx));
                    let item = HeapItem {
                        c,
                        d: a + bb - c, // > 0, as a > 0 and c <= min(a, b)
                        index: idx as u32,
                    };
                    if heap.len() < k {
                        heap.push(Reverse(item));
                    } else if item > heap.peek().unwrap().0 {
                        heap.pop();
                        heap.push(Reverse(item));
                    }
                }
            }

            // Terminate: out of range on both sides, or the best possible bound
            // at this (and hence every larger) distance is strictly below the
            // k-th hit, so no later row can tie it or beat it.
            if d > 0 && !any_valid {
                break;
            }
            if d > 0 && heap.len() == k {
                let w = heap.peek().unwrap().0;
                if frac_lt(max_ub.0, max_ub.1, w.c, w.d) {
                    break;
                }
            }
            d += 1;
            if d > nb {
                break;
            }
        }

        let mut items: Vec<HeapItem> = heap.into_iter().map(|Reverse(h)| h).collect();
        items.sort_unstable_by(|x, y| y.cmp(x));
        items
            .into_iter()
            .map(|h| Hit {
                score: h.c as f64 / h.d as f64,
                index: h.index,
            })
            .collect()
    }

    /// Number of fingerprints inside the BitBound band for `(qpop, t)`: the
    /// candidates [`search_threshold`](Self::search_threshold) evaluates.
    #[must_use]
    pub fn band_candidate_count(&self, qpop: u32, t: f64) -> u64 {
        if qpop == 0 || t.is_nan() || t > 1.0 {
            return 0;
        }
        let (num, den) = threshold_to_rational(t);
        let (lo, hi) = bitbound_bins(qpop, num, den, self.nbits());
        if hi < lo {
            return 0;
        }
        let (start, _) = self.bin_range(lo as usize);
        let (_, end) = self.bin_range(hi as usize);
        (end - start) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mossflux_core::lanes_for_bits;

    /// Tiny deterministic LCG so tests are reproducible without a rand dep.
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.0
        }
    }

    fn random_arena(n: usize, nbits: usize, seed: u64) -> Arena {
        let nwords = lanes_for_bits(nbits);
        let mut rng = Lcg(seed);
        let mut flat = vec![0u64; n * nwords];
        for w in flat.iter_mut() {
            // ~12% bit density so fingerprints are sparse like real ones.
            *w = rng.next() & rng.next() & rng.next();
        }
        let ids: Vec<String> = (0..n).map(|i| format!("M{i}")).collect();
        Arena::build(nbits, flat, ids)
    }

    fn sorted_indices(mut hits: Vec<Hit>) -> Vec<u32> {
        hits.sort_by_key(|a| a.index);
        hits.into_iter().map(|h| h.index).collect()
    }

    /// Build a packed fingerprint from set-bit indices.
    fn packed(bits: &[usize], nbits: usize) -> Vec<u64> {
        let mut w = vec![0u64; mossflux_core::lanes_for_bits(nbits)];
        for &b in bits {
            w[b / 64] |= 1u64 << (b % 64);
        }
        w
    }

    /// Independent f64 ground truth: every row whose exact Tanimoto >= t. It does
    /// not use `threshold_to_rational`, so it catches rounding bugs the
    /// brute-force path cannot.
    fn groundtruth(arena: &Arena, query: &[u64], t: f64) -> Vec<u32> {
        let a = mossflux_core::popcount(query);
        let mut v = Vec::new();
        if a == 0 {
            // Every score is 0/b = 0 (and 0/0 is defined as 0).
            if t <= 0.0 {
                v.extend(0..arena.len() as u32);
            }
            return v;
        }
        for i in 0..arena.len() {
            let b = arena.popcount_of(i) as u32;
            let c = mossflux_core::intersection_popcount(query, arena.row(i));
            let denom = a + b - c;
            let score = if denom == 0 {
                0.0
            } else {
                c as f64 / denom as f64
            };
            if score >= t {
                v.push(i as u32);
            }
        }
        v.sort();
        v
    }

    #[test]
    fn arena_is_popcount_sorted() {
        let a = random_arena(2000, 256, 1);
        for i in 1..a.len() {
            assert!(a.popcount_of(i - 1) <= a.popcount_of(i));
        }
        // Bin ranges must agree with stored popcounts.
        for idx in 0..a.len() {
            let b = a.popcount_of(idx) as usize;
            let (start, end) = a.bin_range(b);
            assert!(start <= idx && idx < end);
        }
    }

    #[test]
    fn bitbound_matches_bruteforce_threshold() {
        let arena = random_arena(3000, 512, 42);
        for &t in &[0.3_f64, 0.5, 0.7, 0.8, 0.9] {
            for q in (0..arena.len()).step_by(137) {
                let query = arena.row(q).to_vec();
                let pruned = sorted_indices(arena.search_threshold(&query, t));
                let brute = sorted_indices(arena.search_threshold_brute(&query, t));
                let truth = groundtruth(&arena, &query, t);
                assert_eq!(
                    pruned, brute,
                    "BitBound vs brute mismatch at t={t}, row {q}"
                );
                assert_eq!(
                    pruned, truth,
                    "BitBound vs f64 ground truth mismatch at t={t}, row {q}"
                );
                // A fingerprint is always similar to itself.
                assert!(pruned.contains(&(q as u32)));
            }
        }
    }

    #[test]
    fn threshold_returns_exact_boundary_two_thirds() {
        // target {0,1,2} (b=3), query {0,1} (a=2), c=2 -> Tanimoto = 2/3 exactly.
        let nbits = 64;
        let arena = Arena::build(nbits, packed(&[0, 1, 2], nbits), vec!["T".to_string()]);
        let query = packed(&[0, 1], nbits);
        let hits = arena.search_threshold(&query, 2.0 / 3.0);
        assert_eq!(hits.len(), 1, "true Tanimoto == t (2/3) must be returned");
        assert_eq!(
            hits[0].score,
            2.0 / 3.0,
            "the score is the exact f64 of 2/3"
        );
    }

    /// The returned score is the value the gate compared, so it is never below
    /// `t` (7/10 must not come back as 0.699999988).
    #[test]
    fn returned_score_is_never_below_t() {
        let nbits = 64;
        let arena = Arena::build(
            nbits,
            packed(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], nbits),
            vec!["T".to_string()],
        );
        let query = packed(&[0, 1, 2, 3, 4, 5, 6], nbits);
        for hits in [
            arena.search_threshold(&query, 0.7),
            arena.search_threshold_brute(&query, 0.7),
        ] {
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].score, 0.7, "7/10 must come back as the f64 0.7");
            assert!(hits[0].score >= 0.7);
        }
    }

    /// Bits at positions `>= nbits` are not part of the fingerprint. On a 12-bit
    /// arena, a query with all 16 low bits set is the 12-bit all-ones query.
    #[test]
    fn query_bits_beyond_nbits_are_ignored() {
        let nbits = 12;
        let arena = Arena::build(nbits, vec![0x0fff], vec!["FULL".to_string()]);
        let hits = arena.search_threshold(&[0xffff], 0.8);
        assert_eq!(hits.len(), 1, "the spare bits must not lower the score");
        assert_eq!(hits[0].score, 1.0);
        let brute = arena.search_threshold_brute(&[0xffff], 0.8);
        assert_eq!(brute.len(), 1);
        let knn = arena.search_knn(&[0xffff], 5);
        assert_eq!(knn.len(), 1, "kNN must not stop early on a > nbits");
        assert_eq!(knn[0].score, 1.0);
    }

    /// `t <= 0` returns every row, also for an empty query.
    #[test]
    fn empty_query_at_t_le_0_returns_every_row() {
        let arena = random_arena(300, 128, 5);
        let zero = vec![0u64; arena.nwords()];
        for t in [0.0, -0.5, f64::NEG_INFINITY] {
            for hits in [
                arena.search_threshold(&zero, t),
                arena.search_threshold_brute(&zero, t),
            ] {
                assert_eq!(sorted_indices(hits.clone()), (0..300).collect::<Vec<u32>>());
                assert!(hits.iter().all(|h| h.score == 0.0));
            }
        }
        for t in [1e-9, 0.5, 1.0, f64::NAN] {
            assert!(arena.search_threshold(&zero, t).is_empty(), "t={t}");
            assert!(arena.search_threshold_brute(&zero, t).is_empty(), "t={t}");
        }
        assert!(arena.search_knn(&zero, 10).is_empty());
    }

    /// Brute-force top-k under the documented total order: exact score
    /// descending (compared as fractions), then row ascending.
    fn brute_knn(arena: &Arena, query: &[u64], k: usize) -> Vec<(u32, f64)> {
        let a = mossflux_core::popcount(query);
        if a == 0 {
            return Vec::new();
        }
        let mut all: Vec<(u32, u32, u32)> = (0..arena.len())
            .map(|i| {
                let b = arena.popcount_of(i) as u32;
                let c = mossflux_core::intersection_popcount(query, arena.row(i));
                (c, a + b - c, i as u32)
            })
            .collect();
        all.sort_by(|x, y| {
            let l = x.0 as u64 * y.1 as u64;
            let r = y.0 as u64 * x.1 as u64;
            r.cmp(&l).then(x.2.cmp(&y.2))
        });
        all.into_iter()
            .take(k)
            .map(|(c, d, i)| (i, c as f64 / d as f64))
            .collect()
    }

    #[test]
    fn knn_matches_bruteforce_topk() {
        let arena = random_arena(2500, 512, 7);
        for &k in &[1usize, 10, 50] {
            for q in (0..arena.len()).step_by(211) {
                let query = arena.row(q).to_vec();
                let got: Vec<(u32, f64)> = arena
                    .search_knn(&query, k)
                    .iter()
                    .map(|h| (h.index, h.score))
                    .collect();
                // Rows AND scores, exactly: the tie rule makes the row set unique.
                assert_eq!(got, brute_knn(&arena, &query, k), "k={k}, row {q}");
                assert_eq!(got[0].1, 1.0, "nearest of self must be 1.0");
            }
        }
    }

    /// Ties at the k-th place go to the lower row, never to scan order. Rows
    /// R0 {0,1}, R1 {0..3}, R2 {0,1,2,4,5}, R3 {0..7}, query {0..3}: R1 scores 1
    /// and R0, R2, R3 all tie at 0.5. The scan visits R2's bin (popcount 5)
    /// before R0's (popcount 2), but the tie rule keeps R0.
    #[test]
    fn knn_breaks_kth_place_ties_by_lower_row() {
        let nbits = 64;
        let mut flat = Vec::new();
        for bits in [
            &[0usize, 1][..],
            &[0, 1, 2, 3],
            &[0, 1, 2, 4, 5],
            &[0, 1, 2, 3, 4, 5, 6, 7],
        ] {
            flat.extend(packed(bits, nbits));
        }
        let ids = ["R0", "R1", "R2", "R3"].map(String::from).to_vec();
        let arena = Arena::build(nbits, flat, ids);
        let query = packed(&[0, 1, 2, 3], nbits);
        let got: Vec<(String, f64)> = arena
            .search_knn(&query, 2)
            .iter()
            .map(|h| (arena.id(h.index as usize), h.score))
            .collect();
        assert_eq!(got, vec![("R1".to_string(), 1.0), ("R0".to_string(), 0.5)]);
        let got: Vec<String> = arena
            .search_knn(&query, 4)
            .iter()
            .map(|h| arena.id(h.index as usize))
            .collect();
        assert_eq!(got, ["R1", "R0", "R2", "R3"], "ties ordered by popcount");
    }

    /// Two scores that are equal in f32 but not in exact arithmetic must still
    /// rank correctly. At 8192 bits, 2046/8183 > 2047/8187 although both round
    /// to the same f32.
    #[test]
    fn knn_ranks_by_exact_fraction_at_wide_widths() {
        let nbits = 8192;
        let query: Vec<usize> = (0..6000).collect();
        let r0: Vec<usize> = (0..2047).chain(6000..8187).collect();
        let r1: Vec<usize> = (0..2046).chain(6000..8183).collect();
        assert_eq!(
            (2047.0f32 / 8187.0).to_bits(),
            (2046.0f32 / 8183.0).to_bits()
        );
        let mut flat = packed(&r0, nbits);
        flat.extend(packed(&r1, nbits));
        let arena = Arena::build(nbits, flat, vec!["R0".into(), "R1".into()]);
        let top = arena.search_knn(&packed(&query, nbits), 1);
        assert_eq!(arena.id(top[0].index as usize), "R1");
        assert_eq!(top[0].score, 2046.0 / 8183.0);
    }

    #[test]
    fn knn_with_a_huge_k_does_not_overflow() {
        let arena = random_arena(50, 128, 3);
        let q = arena.row(10).to_vec();
        for k in [usize::MAX, 1 << 62] {
            assert_eq!(arena.search_knn(&q, k).len(), 50, "k={k}");
        }
    }

    #[test]
    fn threshold_to_rational_exact_values() {
        // Pin the public mapping: rounds DOWN, num may be 0, den fixed at 1e6.
        const DEN: u32 = 1_000_000;
        assert_eq!(threshold_to_rational(0.5), (500_000, DEN));
        // 2/3 must floor to 666666, never 666667.
        assert_eq!(threshold_to_rational(2.0 / 3.0), (666_666, DEN));
        assert_eq!(threshold_to_rational(0.8), (800_000, DEN));
        assert_eq!(threshold_to_rational(0.9), (900_000, DEN));
        assert_eq!(threshold_to_rational(0.95), (950_000, DEN));
        assert_eq!(threshold_to_rational(1.0), (DEN, DEN));
        // Below the 1e-6 granularity / non-positive -> num == 0 (no band pruning).
        assert_eq!(threshold_to_rational(1e-7), (0, DEN));
        assert_eq!(threshold_to_rational(0.0), (0, DEN));
        assert_eq!(threshold_to_rational(-1.0), (0, DEN));
        // t > 1 clamps num to den (band collapses; exact gate still applies).
        assert_eq!(threshold_to_rational(1.000_000_1), (DEN, DEN));
        assert_eq!(threshold_to_rational(2.0), (DEN, DEN));
        // Core invariant: num/den <= t for every finite t in (0, 1].
        for &t in &[1e-6, 0.01, 1.0 / 3.0, 0.5, 2.0 / 3.0, 0.7, 0.999_999, 1.0] {
            let (num, den) = threshold_to_rational(t);
            assert!(num as f64 / den as f64 <= t + 1e-12, "num/den > t at t={t}");
        }
    }

    #[test]
    fn bitbound_bins_exact_lo_hi() {
        // qpop=396, t=0.55: a/t = 719.9999… in f64; integer math must give hi=720.
        // nbits=1024 so the .min(nbits) clamp does not mask it.
        let (num, den) = (550_000u32, 1_000_000u32); // t = 0.55
        assert_eq!(bitbound_bins(396, num, den, 1024), (218, 720));
        // Clean case: a=100, t=0.8 -> lo=ceil(80)=80, hi=floor(125)=125.
        assert_eq!(bitbound_bins(100, 800_000, 1_000_000, 256), (80, 125));
        // num == 0 (t <= 0): whole [0, nbits] band, no pruning.
        assert_eq!(bitbound_bins(100, 0, 1_000_000, 256), (0, 256));
        // hi clamps to nbits when floor(a/t) exceeds the width.
        assert_eq!(bitbound_bins(396, 550_000, 1_000_000, 256), (218, 256));
        // lo is a true ceiling: a=10, t=0.95 -> ceil(9.5)=10.
        assert_eq!(bitbound_bins(10, 950_000, 1_000_000, 256).0, 10);
    }

    /// `bitbound_bins` never drops a reachable target, checked over every
    /// `(qpop, target popcount)` pair in `0..=nbits` and a grid of thresholds.
    ///
    /// The best Tanimoto for popcounts `(q, b)` is `min/max` (a subset attains
    /// it). Reachability is tested in exact integers (`mn*den >= mx*num`): if a
    /// target can reach `num/den`, the band must keep it.
    #[test]
    fn bitbound_bins_never_drops_a_reachable_target() {
        const NB: usize = 256;
        let mut rationals: Vec<(u32, u32)> = vec![
            (0, 1_000_000),         // t <= 0 / below granularity: no pruning
            (1, 1_000_000),         // the smallest non-zero band (hi = q * 1e6)
            (999_999, 1_000_000),   // just under 1.0
            (1_000_000, 1_000_000), // exactly 1.0
        ];
        for &t in &[
            1e-6f64,
            0.01,
            0.05,
            0.25,
            1.0 / 3.0,
            0.5,
            0.55,
            2.0 / 3.0,
            0.7,
            0.8,
            0.9,
            0.95,
            0.99,
            1.0,
        ] {
            rationals.push(threshold_to_rational(t));
        }

        let mut required = 0u64;
        let mut excluded = 0u64;
        for &(num, den) in &rationals {
            for q in 0..=NB as u32 {
                let (lo, hi) = bitbound_bins(q, num, den, NB);
                assert!(
                    hi as usize <= NB,
                    "hi={hi} exceeds nbits={NB} (u32 cast overflow?)"
                );
                for b in 0..=NB as u32 {
                    let (mn, mx) = (q.min(b) as u64, q.max(b) as u64);
                    // reachable >= num/den, in exact integers (0/0 is defined 0).
                    let reachable = if mx == 0 {
                        num == 0
                    } else {
                        mn * den as u64 >= mx * num as u64
                    };
                    if reachable {
                        assert!(
                            lo <= b && b <= hi,
                            "BINS DROPPED A REACHABLE TARGET: q={q} b={b} \
                             num/den={num}/{den} but band is [{lo}, {hi}]"
                        );
                        required += 1;
                    } else if b < lo || b > hi {
                        excluded += 1;
                    }
                }
            }
        }
        // Anti-vacuity both ways: the keep requirement fired, and the band prunes
        // (`[0, nbits]` everywhere would be sound but useless).
        assert!(
            required > 100_000,
            "only {required} reachable triples checked"
        );
        assert!(
            excluded > 100_000,
            "bins pruned only {excluded} triples — no teeth"
        );
    }

    /// The rational band (`bitbound_bins`, built from `num/den <= t`) contains the
    /// f64 band (`mossflux_core::bitbound_range`) at every threshold.
    #[test]
    fn bins_band_contains_float_band() {
        const NB: usize = 2048;
        let mut compared = 0usize;
        for &t in &[
            1e-6f64,
            0.05,
            0.25,
            1.0 / 3.0,
            0.5,
            0.55,
            2.0 / 3.0,
            0.7,
            0.8,
            0.9,
            0.95,
            1.0,
        ] {
            let (num, den) = threshold_to_rational(t);
            for q in 0..=NB as u32 {
                let (blo, bhi) = bitbound_bins(q, num, den, NB);
                let (flo, fhi) = mossflux_core::bitbound_range(q, t);
                let fhi = fhi.min(NB as u32); // the float form is unclamped
                assert!(
                    blo <= flo,
                    "rational lo {blo} is STRICTER than the float lo {flo} (q={q}, t={t})"
                );
                assert!(
                    bhi >= fhi,
                    "rational hi {bhi} is STRICTER than the float hi {fhi} (q={q}, t={t})"
                );
                compared += 1;
            }
        }
        assert_eq!(compared, 12 * (NB + 1), "sweep did not run");
    }

    #[test]
    fn band_count_equals_scanned() {
        let arena = random_arena(1500, 256, 99);
        for q in (0..arena.len()).step_by(53) {
            let query = arena.row(q).to_vec();
            let qpop = mossflux_core::popcount(&query);
            let (num, den) = threshold_to_rational(0.7);
            let (lo, hi) = bitbound_bins(qpop, num, den, arena.nbits());
            let mut scanned = 0u64;
            for b in lo..=hi {
                let (s, e) = arena.bin_range(b as usize);
                scanned += (e - s) as u64;
            }
            assert_eq!(scanned, arena.band_candidate_count(qpop, 0.7));
        }
    }
}
