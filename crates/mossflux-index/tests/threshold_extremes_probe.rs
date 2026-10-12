//! Threshold-EXTREME exactness: the far ends of the `t` contract, where the
//! rational band collapses (`t >= 1`), disappears (`t <= 0`, sub-granularity
//! `t`), or is not an order at all (`NaN`).
//!
//! Every assertion compares a search path against the independent f64
//! `groundtruth` helper below, which does not share `threshold_to_rational` with
//! the engine. Low-threshold coverage lives in `property_sweep.rs`; the band
//! arithmetic is proved exhaustively in
//! `search.rs::bitbound_bins_never_drops_a_reachable_target`.

use mossflux_index::{
    bitbound_bins, threshold_to_rational, Arena, BitmapScratch, InvertedBitmap, InvertedIndex,
    InvertedScratch,
};

fn packed(bits: &[usize], nbits: usize) -> Vec<u64> {
    let nwords = mossflux_index::mossflux_core::lanes_for_bits(nbits);
    let mut w = vec![0u64; nwords];
    for &b in bits {
        w[b / 64] |= 1u64 << (b % 64);
    }
    w
}

fn sorted(mut h: Vec<mossflux_index::Hit>) -> Vec<u32> {
    h.sort_by_key(|x| x.index);
    h.into_iter().map(|x| x.index).collect()
}

/// Independent f64 ground truth not sharing threshold_to_rational. An all-zero
/// query scores 0 against every row, so it matches every row at `t <= 0`.
fn groundtruth(arena: &Arena, query: &[u64], t: f64) -> Vec<u32> {
    let a = mossflux_index::mossflux_core::popcount(query);
    let mut v = Vec::new();
    if a == 0 {
        if t <= 0.0 {
            v.extend(0..arena.len() as u32);
        }
        return v;
    }
    for i in 0..arena.len() {
        let b = arena.popcount_of(i) as u32;
        let c = mossflux_index::mossflux_core::intersection_popcount(query, arena.row(i));
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
fn probe_t_one_exact_only() {
    // t = 1.0: only fingerprints with identical bits should match.
    // Distinct fingerprints with same popcount must NOT match.
    let nbits = 64;
    let flat = {
        let mut f = Vec::new();
        f.extend(packed(&[0, 1, 2], nbits)); // A
        f.extend(packed(&[0, 1, 3], nbits)); // B (same popcount, distinct)
        f.extend(packed(&[0, 1, 2], nbits)); // A' duplicate of A
        f
    };
    let arena = Arena::build(nbits, flat, vec!["A".into(), "B".into(), "Adup".into()]);
    let bm = InvertedBitmap::build(&arena);
    let inv = InvertedIndex::build(&arena);
    let mut bsc = BitmapScratch::new();
    let mut isc = InvertedScratch::new(arena.len());
    let query = packed(&[0, 1, 2], nbits);
    let lin = sorted(arena.search_threshold(&query, 1.0));
    let bmh = sorted(bm.search_threshold(&query, 1.0, &mut bsc));
    let ivh = sorted(inv.search_threshold(&query, 1.0, &mut isc));
    let truth = groundtruth(&arena, &query, 1.0);
    // Hand-computed: rows are popcount-sorted but all three have popcount 3, so
    // the build's stable sort keeps input order — A=0, B=1, Adup=2. Only the two
    // copies of A are bit-identical to the query.
    assert_eq!(
        truth,
        vec![0, 2],
        "only the two identical fingerprints reach 1.0"
    );
    assert_eq!(lin, truth);
    assert_eq!(bmh, truth, "bitmap wrong at t=1.0");
    assert_eq!(ivh, truth, "inverted wrong at t=1.0");
}

#[test]
fn probe_nan_and_negative() {
    // Contract: NaN -> empty (no panic); t < 0 -> match-all (Tanimoto >= t<0).
    let arena = Arena::synthetic(300, 256, 0.12, 5);
    let bm = InvertedBitmap::build(&arena);
    let mut bsc = BitmapScratch::new();
    for q in (0..arena.len()).step_by(50) {
        let query = arena.row(q).to_vec();
        assert!(
            arena.search_threshold(&query, f64::NAN).is_empty(),
            "NaN linear"
        );
        assert!(
            arena.search_threshold_brute(&query, f64::NAN).is_empty(),
            "NaN brute"
        );
        assert!(
            bm.search_threshold(&query, f64::NAN, &mut bsc).is_empty(),
            "NaN bitmap"
        );
        let truth = groundtruth(&arena, &query, -1.0);
        // t < 0 means *every* row qualifies — pin that, so a truth helper that
        // silently returned nothing could not make the three comparisons vacuous.
        assert_eq!(truth.len(), arena.len(), "t<0 must select every row");
        assert_eq!(
            sorted(arena.search_threshold(&query, -1.0)),
            truth,
            "neg linear"
        );
        assert_eq!(
            sorted(arena.search_threshold_brute(&query, -1.0)),
            truth,
            "neg brute"
        );
        assert_eq!(
            sorted(bm.search_threshold(&query, -1.0, &mut bsc)),
            truth,
            "neg bitmap"
        );
    }
}

#[test]
fn probe_t_above_one() {
    let nbits = 64;
    let arena = Arena::build(nbits, packed(&[0, 1, 2], nbits), vec!["A".into()]);
    let bm = InvertedBitmap::build(&arena);
    let mut bsc = BitmapScratch::new();
    let query = packed(&[0, 1, 2], nbits);
    for &t in &[1.0000001_f64, 1.5, 2.0] {
        let lin = sorted(arena.search_threshold(&query, t));
        let bmh = sorted(bm.search_threshold(&query, t, &mut bsc));
        let truth = groundtruth(&arena, &query, t);
        // Tanimoto never exceeds 1.0, so any t > 1 must select nothing — even for
        // a fingerprint compared against itself.
        assert!(truth.is_empty(), "t={t} > 1.0 must be unsatisfiable");
        assert_eq!(lin, truth, "lin wrong at t={t}");
        assert_eq!(bmh, truth, "bitmap wrong at t={t}");
    }
}

/// The `hi = qpop*den/num` branch at the SMALLEST non-zero `num`, where the
/// product is largest and a `u32` cast could wrap.
///
/// Every `qpop` in `0..=2048` is checked against lo/hi recomputed in `u128`
/// (which cannot wrap here), for the widest fingerprint the engine ships.
/// `lo <= hi` alone would not catch a `hi` that wrapped to a small value.
#[test]
fn probe_bitbound_bins_overflow() {
    const NBITS: usize = 2048;
    // 1e-9 floors to num == 0 -> no pruning at all; 1e-6 is the smallest non-zero
    // num the rational mapping can produce, so hi = q * 1_000_000 (up to 2.048e9,
    // which OVERFLOWS an i32/u32 intermediate if the math is not done in u64).
    assert_eq!(
        threshold_to_rational(1e-9),
        (0, 1_000_000),
        "1e-9 floors to num=0"
    );
    let (num, den) = threshold_to_rational(1e-6);
    assert_eq!(
        (num, den),
        (1, 1_000_000),
        "1e-6 is the smallest non-zero num"
    );

    for qpop in 0..=NBITS as u32 {
        for &(n, d) in &[(0u32, den), (num, den), (999_999, den), (den, den)] {
            let (lo, hi) = bitbound_bins(qpop, n, d, NBITS);
            // Independent reference, in u128 so it cannot itself wrap.
            let (want_lo, want_hi) = if n == 0 {
                (0u128, NBITS as u128)
            } else {
                let q = qpop as u128;
                (
                    (q * n as u128).div_ceil(d as u128),
                    ((q * d as u128) / n as u128).min(NBITS as u128),
                )
            };
            assert_eq!(lo as u128, want_lo, "lo wrong: qpop={qpop} num={n} den={d}");
            assert_eq!(hi as u128, want_hi, "hi wrong: qpop={qpop} num={n} den={d}");
            assert!(hi as usize <= NBITS, "hi={hi} escaped the nbits clamp");
        }
    }
    // The specific overflow shape: q at the full width, num == 1. Unclamped this
    // is 2_048_000_000 — past i32::MAX — and must clamp to nbits, not wrap.
    assert_eq!(bitbound_bins(2048, 1, 1_000_000, NBITS), (1, 2048));
}

/// Dense fingerprints (large popcount => many bit-planes in the bitmap kernel)
/// at low `t`, where the band covers the whole arena.
///
/// Every path is compared against the independent f64 oracle, not against a
/// sibling, so a shared plane-count assumption cannot hide.
#[test]
fn probe_dense_low_t_many_planes() {
    let nbits = 512;
    // Alternating half-density rows: popcount 256 each, so the bitmap needs 9
    // bit-planes to hold an accumulator value.
    let mut flat = Vec::new();
    let ids: Vec<String> = (0..200).map(|i| format!("M{i}")).collect();
    for i in 0..200 {
        let bits: Vec<usize> = (0..nbits).filter(|b| (b + i) % 2 == 0).collect();
        flat.extend(packed(&bits, nbits));
    }
    let arena = Arena::build(nbits, flat, ids);
    for i in 0..arena.len() {
        assert_eq!(arena.popcount_of(i), 256, "row {i} should be half-dense");
    }
    let bm = InvertedBitmap::build(&arena);
    let inv = InvertedIndex::build(&arena);
    let mut bsc = BitmapScratch::new();
    let mut bsc_ab = BitmapScratch::new();
    bsc_ab.set_force_abandon(Some(true));
    let mut isc = InvertedScratch::new(arena.len());
    let mut nonempty = 0usize;
    for &t in &[0.0_f64, 1e-6, 0.01, 0.5, 1.0] {
        for q in (0..arena.len()).step_by(13) {
            let query = arena.row(q).to_vec();
            let truth = groundtruth(&arena, &query, t);
            let ctx = format!("dense t={t} q={q}");
            assert_eq!(
                sorted(arena.search_threshold(&query, t)),
                truth,
                "linear {ctx}"
            );
            assert_eq!(
                sorted(bm.search_threshold(&query, t, &mut bsc)),
                truth,
                "bitmap {ctx}"
            );
            assert_eq!(
                sorted(bm.search_threshold(&query, t, &mut bsc_ab)),
                truth,
                "bitmap-abandon {ctx}"
            );
            assert_eq!(
                sorted(inv.search_threshold(&query, t, &mut isc)),
                truth,
                "inverted {ctx}"
            );
            if !truth.is_empty() {
                nonempty += 1;
            }
        }
    }
    // Anti-vacuity: the oracle produced real hits, so the comparisons above were
    // not all `[] == []`.
    assert!(
        nonempty > 20,
        "sweep degenerate: only {nonempty} non-empty results"
    );
}
