//! Core fingerprint primitives for the `mossflux` similarity search engine.
//!
//! A fingerprint is a fixed-width bit vector stored as packed little-endian
//! `u64` lanes, so bitwise ops and `popcount` map onto hardware instructions.
//!
//! Similarity is the Tanimoto (Jaccard) coefficient:
//!
//! ```text
//!   T(A, B) = |A & B| / |A | B| = c / (a + b - c)
//! ```
//!
//! where `a = popcount(A)`, `b = popcount(B)`, and `c = popcount(A & B)`.

#![forbid(unsafe_code)]

/// Default fingerprint width in bits (ECFP4-style Morgan fingerprint).
pub const DEFAULT_NBITS: usize = 2048;

/// Number of `u64` lanes needed to hold a fingerprint of `nbits` bits.
#[inline]
#[must_use]
pub const fn lanes_for_bits(nbits: usize) -> usize {
    nbits.div_ceil(64)
}

/// Population count (number of set bits) of a packed fingerprint.
///
/// The running sum is `u64` so the NEON reduction stays in wide lanes. The sum
/// is exact, and a fingerprint's popcount always fits `u32`.
#[inline]
#[must_use]
pub fn popcount(words: &[u64]) -> u32 {
    words.iter().map(|w| w.count_ones() as u64).sum::<u64>() as u32
}

/// `popcount(A & B)` — size of the intersection of two equal-width fingerprints.
///
/// The caller must pass equal widths. This is the innermost scan kernel, so the
/// width is checked only in debug builds; in release, lanes past the shorter
/// slice are ignored. The `mossflux-index` search entry points check the query
/// width once, before the scan.
#[inline]
#[must_use]
pub fn intersection_popcount(a: &[u64], b: &[u64]) -> u32 {
    debug_assert_eq!(a.len(), b.len(), "fingerprints must have equal width");
    // u64 accumulator: see `popcount`.
    a.iter()
        .zip(b)
        .map(|(x, y)| (x & y).count_ones() as u64)
        .sum::<u64>() as u32
}

/// `popcount(A | B)` — size of the union of two equal-width fingerprints.
///
/// # Panics
/// Panics if `a.len() != b.len()`.
#[inline]
#[must_use]
pub fn union_popcount(a: &[u64], b: &[u64]) -> u32 {
    assert_eq!(a.len(), b.len(), "fingerprints must have equal width");
    a.iter()
        .zip(b)
        .map(|(x, y)| (x | y).count_ones() as u64)
        .sum::<u64>() as u32
}

/// Tanimoto similarity between two fingerprints, computed in a single pass.
///
/// Returns the correctly rounded `f64` of the exact fraction, in `[0.0, 1.0]`;
/// the `mossflux-index` search paths report the same value. Two all-zero
/// fingerprints have similarity `0.0`.
///
/// # Panics
/// Panics if `a.len() != b.len()`.
#[inline]
#[must_use]
pub fn tanimoto(a: &[u64], b: &[u64]) -> f64 {
    assert_eq!(a.len(), b.len(), "fingerprints must have equal width");
    let mut inter = 0u64;
    let mut uni = 0u64;
    for (x, y) in a.iter().zip(b) {
        inter += (x & y).count_ones() as u64;
        uni += (x | y).count_ones() as u64;
    }
    if uni == 0 {
        0.0
    } else {
        inter as f64 / uni as f64
    }
}

/// Tanimoto similarity using precomputed popcounts.
///
/// Each comparison needs only the intersection popcount `c`:
///
/// ```text
///   T = c / (a_count + b_count - c)
/// ```
///
/// Returns the same value as [`tanimoto`].
///
/// # Precondition
/// `a_count` must equal `popcount(a)` and `b_count` must equal `popcount(b)`.
/// They are checked only in debug builds.
///
/// # Panics
/// If the counts are smaller than the true intersection popcount, the
/// denominator underflows: with `overflow-checks = true` (this workspace's
/// release profile) that panics; without, it wraps and the result is
/// meaningless.
#[inline]
#[must_use]
pub fn tanimoto_with_counts(a: &[u64], a_count: u32, b: &[u64], b_count: u32) -> f64 {
    debug_assert_eq!(a_count, popcount(a), "a_count must be popcount(a)");
    debug_assert_eq!(b_count, popcount(b), "b_count must be popcount(b)");
    let c = intersection_popcount(a, b);
    let denom = a_count + b_count - c;
    if denom == 0 {
        0.0
    } else {
        c as f64 / denom as f64
    }
}

/// Upper bound on the Tanimoto similarity achievable between two fingerprints
/// given only their popcounts: `T(A, B) <= min(a, b) / max(a, b)`.
///
/// This is the basis of BitBound pruning (Swamidass & Baldi, 2007): if the
/// bound is below the threshold, the full comparison can be skipped.
#[inline]
#[must_use]
pub fn tanimoto_upper_bound(a_count: u32, b_count: u32) -> f64 {
    let (lo, hi) = if a_count <= b_count {
        (a_count, b_count)
    } else {
        (b_count, a_count)
    };
    if hi == 0 {
        0.0
    } else {
        lo as f64 / hi as f64
    }
}

/// Inclusive `[min, max]` target popcount range that can satisfy `T >= t` for a
/// query of popcount `query_popcount`. Targets outside it can be skipped.
///
/// From `t <= T(A, B) <= min(a, b) / max(a, b)`:
/// `t * query <= target <= query / t`.
#[inline]
#[must_use]
pub fn bitbound_range(query_popcount: u32, t: f64) -> (u32, u32) {
    if t <= 0.0 {
        return (0, u32::MAX);
    }
    // Widen by an epsilon before ceil/floor: `396 / 0.55 == 719.9999999999999`,
    // and a plain `floor` would give 719, dropping valid hits at 720. The
    // rational `bitbound_bins` in `mossflux-index` needs no epsilon.
    const EPS: f64 = 1e-9;
    let q = query_popcount as f64;
    let lo = (q * t - EPS).ceil().max(0.0) as u32;
    let hi = (q / t + EPS).floor() as u32;
    (lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a fingerprint from a list of set-bit indices.
    fn fp(bits: &[usize], nbits: usize) -> Vec<u64> {
        let mut w = vec![0u64; lanes_for_bits(nbits)];
        for &b in bits {
            w[b / 64] |= 1u64 << (b % 64);
        }
        w
    }

    #[test]
    fn lanes_rounding() {
        assert_eq!(lanes_for_bits(2048), 32);
        assert_eq!(lanes_for_bits(1), 1);
        assert_eq!(lanes_for_bits(64), 1);
        assert_eq!(lanes_for_bits(65), 2);
    }

    #[test]
    fn popcount_basic() {
        assert_eq!(popcount(&fp(&[0, 1, 200], 256)), 3);
        assert_eq!(popcount(&[0u64; 4]), 0);
    }

    #[test]
    fn tanimoto_identical_is_one() {
        let a = fp(&[1, 5, 9, 100], 128);
        assert_eq!(tanimoto(&a, &a), 1.0);
    }

    #[test]
    fn tanimoto_disjoint_is_zero() {
        let a = fp(&[1, 2, 3], 128);
        let b = fp(&[4, 5, 6], 128);
        assert_eq!(tanimoto(&a, &b), 0.0);
    }

    #[test]
    fn tanimoto_known_value() {
        // A = {1,2,3,4}, B = {3,4,5}. inter = {3,4} = 2, union = {1,2,3,4,5} = 5.
        let a = fp(&[1, 2, 3, 4], 64);
        let b = fp(&[3, 4, 5], 64);
        assert_eq!(tanimoto(&a, &b), 2.0 / 5.0);
    }

    #[test]
    fn tanimoto_empty_is_zero() {
        let z = vec![0u64; 2];
        assert_eq!(tanimoto(&z, &z), 0.0);
    }

    #[test]
    fn with_counts_matches_direct() {
        let a = fp(&[1, 2, 3, 4], 256);
        let b = fp(&[3, 4, 5, 200], 256);
        let ac = popcount(&a);
        let bc = popcount(&b);
        assert_eq!(tanimoto(&a, &b), tanimoto_with_counts(&a, ac, &b, bc));
    }

    /// Deterministic LCG — fixed seed, so every sweep below reproduces exactly.
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            self.0
        }
        /// A sparse-ish random fingerprint of `nbits` (AND of draws => ~12% density).
        fn fingerprint(&mut self, nbits: usize) -> Vec<u64> {
            let mut w = vec![0u64; lanes_for_bits(nbits)];
            for x in w.iter_mut() {
                *x = self.next() & self.next() & self.next();
            }
            // Mask spare high bits so popcounts never exceed nbits.
            if !nbits.is_multiple_of(64) {
                let last = w.len() - 1;
                w[last] &= (1u64 << (nbits % 64)) - 1;
            }
            w
        }
        /// A fingerprint close to `a`. Correlated pairs approach the BitBound
        /// bound, which independent random pairs never do.
        fn near(&mut self, a: &[u64], nbits: usize) -> Vec<u64> {
            let mut w = vec![0u64; a.len()];
            for (i, x) in w.iter_mut().enumerate() {
                let keep = self.next() | self.next() | self.next(); // ~87% kept
                let add = self.next() & self.next() & self.next() & self.next();
                *x = (a[i] & keep) | add;
            }
            if !nbits.is_multiple_of(64) {
                let last = w.len() - 1;
                w[last] &= (1u64 << (nbits % 64)) - 1;
            }
            w
        }
    }

    /// Tanimoto arithmetic identities over 50k random pairs at four widths:
    /// inclusion-exclusion for `union_popcount`, symmetry and range, and
    /// `tanimoto_with_counts` bit-identical to `tanimoto`.
    #[test]
    fn tanimoto_arithmetic_identities() {
        let mut rng = Lcg(0x5EED_C0DE_1234_5678);
        let mut pairs = 0usize;
        let mut saw_partial_overlap = 0usize;
        for &nbits in &[64usize, 128, 166, 2048] {
            for _ in 0..12_500 {
                let a = rng.fingerprint(nbits);
                let b = rng.fingerprint(nbits);
                let pa = popcount(&a);
                let pb = popcount(&b);
                let c = intersection_popcount(&a, &b);
                let u = union_popcount(&a, &b);

                // (1) Inclusion-exclusion.
                assert_eq!(
                    u,
                    pa + pb - c,
                    "|A|B| != |A|+|B|-|A&B| (nbits={nbits}, a={pa}, b={pb}, c={c})"
                );
                // (2) Intersection: symmetric, and bounded by the smaller set.
                assert_eq!(
                    c,
                    intersection_popcount(&b, &a),
                    "intersection not symmetric"
                );
                assert!(
                    c <= pa.min(pb),
                    "|A&B| = {c} exceeds min(|A|,|B|) = {}",
                    pa.min(pb)
                );
                assert!(u >= pa.max(pb), "|A|B| = {u} below max(|A|,|B|)");
                assert!(u as usize <= nbits, "union {u} exceeds the width {nbits}");
                // (3) Tanimoto: symmetric and in [0, 1].
                let t = tanimoto(&a, &b);
                assert_eq!(t, tanimoto(&b, &a), "Tanimoto not symmetric");
                assert!((0.0..=1.0).contains(&t), "Tanimoto {t} outside [0,1]");
                // (4) The precomputed-popcount form is bit-identical.
                assert_eq!(
                    t.to_bits(),
                    tanimoto_with_counts(&a, pa, &b, pb).to_bits(),
                    "tanimoto_with_counts differs from tanimoto in the last bit \
                     (a={pa}, b={pb}, c={c})"
                );
                // (5) Reflexivity for any non-empty fingerprint.
                if pa > 0 {
                    assert_eq!(tanimoto(&a, &a), 1.0, "T(A,A) must be exactly 1.0");
                }
                if c > 0 && c < pa.min(pb) {
                    saw_partial_overlap += 1;
                }
                pairs += 1;
            }
        }
        // Anti-vacuity: the loop ran and produced partially-overlapping pairs.
        assert_eq!(
            pairs, 50_000,
            "sweep did not run the expected number of pairs"
        );
        assert!(
            saw_partial_overlap > 1000,
            "sweep degenerate: only {saw_partial_overlap} partially-overlapping pairs"
        );
    }

    /// The BitBound bound dominates the true Tanimoto.
    ///
    /// Checked exactly in integers (`c*max <= min*(a+b-c)`) and on the `f64`
    /// functions with no epsilon: IEEE division is monotone, so `T <= UB` must
    /// hold bit-exactly.
    #[test]
    fn upper_bound_dominates_true_value() {
        let mut rng = Lcg(0xB17B_0000_D0DE);
        let mut near_tight = 0usize;
        let mut exact_equality = 0usize;
        for &nbits in &[64usize, 256, 2048] {
            for i in 0..8_000 {
                let a = rng.fingerprint(nbits);
                // Half independent pairs (loose bound), half near-duplicates
                // (bound nearly attained, where a bound one ulp low drops a hit).
                let b = if i % 2 == 0 {
                    rng.fingerprint(nbits)
                } else {
                    rng.near(&a, nbits)
                };
                let (pa, pb) = (popcount(&a), popcount(&b));
                let c = intersection_popcount(&a, &b);
                let (mn, mx) = (pa.min(pb) as u64, pa.max(pb) as u64);
                let denom = (pa + pb - c) as u64;
                // Exact integer theorem: c/denom <= mn/mx  <=>  c*mx <= mn*denom.
                assert!(
                    c as u64 * mx <= mn * denom,
                    "BitBound domination violated exactly: c={c} denom={denom} \
                     min={mn} max={mx}"
                );
                // And on the shipped f64 API, with no epsilon.
                let t = tanimoto(&a, &b);
                let ub = tanimoto_upper_bound(pa, pb);
                assert!(t <= ub, "true Tanimoto {t} exceeded its f64 bound {ub}");
                if ub > 0.0 && t > 0.9 * ub {
                    near_tight += 1;
                }
            }
        }
        // A subset attains min/max exactly; these cases catch a bound one ulp low.
        for &(sub, sup) in &[
            (&[0usize, 1, 2][..], &[0usize, 1, 2, 3, 4][..]), // 3/5
            (&[0][..], &[0, 1][..]),                          // 1/2
            (&[0, 1, 2, 3][..], &[0, 1, 2, 3][..]),           // 1/1
            (&[5, 60, 61, 62, 63][..], &[5, 60, 61, 62, 63, 100, 200][..]), // 5/7
        ] {
            let a = fp(sub, 256);
            let b = fp(sup, 256);
            let (pa, pb) = (popcount(&a), popcount(&b));
            let t = tanimoto(&a, &b);
            let ub = tanimoto_upper_bound(pa, pb);
            assert_eq!(
                t.to_bits(),
                ub.to_bits(),
                "a subset attains the bound exactly: T={t} must equal UB={ub}"
            );
            exact_equality += 1;
        }
        assert_eq!(exact_equality, 4, "the tight cases did not run");
        assert!(
            near_tight > 100,
            "sweep never approached the bound ({near_tight} near-tight pairs) — it \
             would not detect a loose-but-wrong bound"
        );
    }

    /// The popcount band never excludes a target that could meet the threshold.
    ///
    /// Enumerates every (query, target) popcount pair in `0..=256` against a
    /// threshold grid including 0, just under 1, and 1. The best Tanimoto for
    /// popcounts (q, b) is `min(q,b)/max(q,b)` (a subset attains it); if that
    /// reaches `t`, the band must contain `b`.
    #[test]
    fn bitbound_band_never_drops_a_reachable_target() {
        const N: u32 = 256;
        let ts: Vec<f64> = {
            let mut v = vec![
                0.0f64,
                1e-9,
                1e-6,
                1.0 / 3.0,
                2.0 / 3.0,
                0.55,
                0.999_999,
                1.0,
            ];
            for i in 1..20 {
                v.push(i as f64 * 0.05);
            }
            v
        };
        let mut required = 0u64; // (q,b,t) triples the band had to keep
        let mut excluded = 0u64; // (q,b,t) triples the band legitimately pruned
        for &t in &ts {
            for q in 0..=N {
                let (lo, hi) = bitbound_range(q, t);
                for b in 0..=N {
                    let (mn, mx) = (q.min(b), q.max(b));
                    // Best Tanimoto reachable by ANY pair with these popcounts.
                    let reachable = if mx == 0 { 0.0 } else { mn as f64 / mx as f64 };
                    if reachable >= t {
                        assert!(
                            lo <= b && b <= hi,
                            "BAND DROPPED A REACHABLE TARGET: q={q} b={b} t={t} \
                             (max reachable Tanimoto {reachable} >= t) but band is \
                             [{lo}, {hi}]"
                        );
                        required += 1;
                    } else if b < lo || b > hi {
                        excluded += 1;
                    }
                }
            }
        }
        // Anti-vacuity both ways: the keep requirement was exercised, and the
        // band prunes (`[0, u32::MAX]` everywhere would be sound but useless).
        assert!(
            required > 100_000,
            "only {required} reachable triples checked"
        );
        assert!(
            excluded > 100_000,
            "band pruned only {excluded} triples — no teeth"
        );
    }

    #[test]
    fn bitbound_excludes_too_sparse_or_too_dense() {
        // Query popcount 40, threshold 0.5 -> band [20, 80].
        let (lo, hi) = bitbound_range(40, 0.5);
        assert_eq!((lo, hi), (20, 80));
    }

    #[test]
    fn bitbound_range_no_float_floor_bug() {
        // 396 / 0.55 == 719.9999999999999 in f64; the band must still include 720.
        let (_lo, hi) = bitbound_range(396, 0.55);
        assert_eq!(
            hi, 720,
            "float floor bug regressed: hi should be 720, got {hi}"
        );
    }
}
