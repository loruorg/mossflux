//! Cross-path exactness sweep. Every search path (linear, brute, bitmap auto,
//! bitmap force-abandon, inverted) must equal an independent f64 ground truth
//! (not a sibling that shares `threshold_to_rational`) across widths 64..2048,
//! dataset shapes, densities, and the full threshold contract: boundary 2/3,
//! high thresholds 0.8/0.9/0.95, t<=0 matches all, NaN/Inf returns nothing.

use mossflux_index::mossflux_core::{intersection_popcount, lanes_for_bits, popcount};
use mossflux_index::{Arena, BitmapScratch, Hit, InvertedBitmap, InvertedIndex, InvertedScratch};

fn idx_set(mut h: Vec<Hit>) -> Vec<u32> {
    h.sort_by_key(|x| x.index);
    h.into_iter().map(|x| x.index).collect()
}

/// Independent ground truth: indices whose exact f64 Tanimoto satisfies `>= t`.
/// NaN/Inf `t` => empty; `t<=0` => every row (invariant 5), and that includes an
/// all-zero query, whose score is 0 against every row (`0/0` is defined as 0).
fn gt_set(arena: &Arena, query: &[u64], t: f64) -> Vec<u32> {
    let a = popcount(query);
    let mut v = Vec::new();
    if a == 0 {
        if t <= 0.0 {
            v.extend(0..arena.len() as u32);
        }
        return v;
    }
    for i in 0..arena.len() {
        let b = arena.popcount_of(i) as u32;
        let c = intersection_popcount(query, arena.row(i));
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
    v.sort_unstable();
    v
}

#[test]
fn cross_path_property_sweep() {
    let widths = [64usize, 128, 166, 256, 512, 2048];
    let ns = [1usize, 64, 200, 257, 500];
    let densities = [0.05f64, 0.12, 0.30];
    let ts = [
        -1.0f64,
        0.0,
        // Sub-granularity and small-`num` thresholds: 1e-9 floors to num == 0 (no
        // band pruning at all), while 1e-6/1e-3 give num == 1/1000, where
        // `hi = q*den/num` is largest and a u32 cast could overflow.
        1e-9,
        1e-6,
        0.001,
        0.01,
        0.05,
        2.0 / 3.0,
        0.5,
        0.7,
        0.8,
        0.85,
        0.9,
        0.95,
        1.0,
        1.5,
        f64::NAN,
        f64::INFINITY,
    ];
    let mut seed = 0xC0FFEEu64;
    for &nb in &widths {
        for &n in &ns {
            for &d in &densities {
                seed = seed.wrapping_add(0x9E3779B9);
                let arena = Arena::synthetic(n, nb, d, seed);
                let bm = InvertedBitmap::build(&arena);
                let inv = InvertedIndex::build(&arena);
                let mut bsc = BitmapScratch::new();
                let mut bsc_ab = BitmapScratch::new();
                bsc_ab.set_force_abandon(Some(true)); // exercise the abandon path everywhere
                let mut isc = InvertedScratch::new(arena.len());

                let mut queries: Vec<Vec<u64>> = (0..arena.len())
                    .step_by((n / 4).max(1))
                    .map(|i| arena.row(i).to_vec())
                    .collect();
                queries.push(vec![0u64; lanes_for_bits(nb)]); // all-zero query

                for q in &queries {
                    for &t in &ts {
                        let truth = gt_set(&arena, q, t);
                        let ctx = format!("nb={nb} n={n} d={d} t={t}");
                        assert_eq!(idx_set(arena.search_threshold(q, t)), truth, "linear {ctx}");
                        assert_eq!(
                            idx_set(arena.search_threshold_brute(q, t)),
                            truth,
                            "brute {ctx}"
                        );
                        assert_eq!(
                            idx_set(bm.search_threshold(q, t, &mut bsc)),
                            truth,
                            "bitmap {ctx}"
                        );
                        assert_eq!(
                            idx_set(bm.search_threshold(q, t, &mut bsc_ab)),
                            truth,
                            "bitmap-abandon {ctx}"
                        );
                        assert_eq!(
                            idx_set(inv.search_threshold(q, t, &mut isc)),
                            truth,
                            "inverted {ctx}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn multi_block_and_abandon_exactness() {
    // block_override forces many cache-blocks at unit-test scale, exercising the
    // wb-stitching loop, per-block plane re-zeroing, and block-boundary masking —
    // unreachable otherwise without ~700K rows. n=257 is not a multiple of 64.
    let arena = Arena::synthetic(257, 256, 0.15, 9);
    let bm = InvertedBitmap::build(&arena);
    let mut sc = BitmapScratch::new();
    sc.set_block_override(Some(1)); // 1 word/block -> 5 blocks
    for &abandon in &[false, true] {
        sc.set_force_abandon(Some(abandon));
        for &t in &[0.0f64, 0.5, 0.7, 0.85, 0.95, 1.0] {
            for q in (0..arena.len()).step_by(31) {
                let query = arena.row(q).to_vec();
                assert_eq!(
                    idx_set(bm.search_threshold(&query, t, &mut sc)),
                    gt_set(&arena, &query, t),
                    "multiblock abandon={abandon} t={t} q={q}"
                );
            }
        }
    }
}

/// Sorted (index, score-bits) so two Hit vecs can be compared byte-for-byte.
fn hit_pairs(mut h: Vec<Hit>) -> Vec<(u32, u64)> {
    h.sort_by_key(|x| x.index);
    h.into_iter()
        .map(|x| (x.index, x.score.to_bits()))
        .collect()
}

#[test]
fn multiblock_register_strip_stitch() {
    // The 8-word register-strip Phase A only runs when blen >= 8; this covers the
    // strip/tail/writeback seam across a block boundary. Blocks of 16/24 words (each
    // runs >= 1 full 8-word strip plus a tail) and a band spanning several blocks.
    // Exact vs the independent f64 oracle, abandon on and off.
    for &(n, nb, d, seed) in &[(3000usize, 256usize, 0.12f64, 11u64), (3201, 256, 0.08, 5)] {
        let arena = Arena::synthetic(n, nb, d, seed);
        let bm = InvertedBitmap::build(&arena);
        for &blk in &[16usize, 24] {
            for &abandon in &[false, true] {
                let mut sc = BitmapScratch::new();
                sc.set_block_override(Some(blk));
                sc.set_force_abandon(Some(abandon));
                // Low t => wide band spanning many words => several blocks.
                for &t in &[0.3f64, 0.5, 0.7, 0.85, 0.95] {
                    for q in (0..arena.len()).step_by(263) {
                        let query = arena.row(q).to_vec();
                        assert_eq!(
                            idx_set(bm.search_threshold(&query, t, &mut sc)),
                            gt_set(&arena, &query, t),
                            "strip-stitch n={n} blk={blk} abandon={abandon} t={t} q={q}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn abandon_three_way_equivalence_with_scores() {
    // The default path (force_abandon=None -> off), the forced-off path, and the
    // forced-on (de-vectorized Phase B) path must return BYTE-IDENTICAL hits —
    // same indices AND same f32 scores — not merely each == truth.
    let arena = Arena::synthetic(2500, 256, 0.15, 7);
    let bm = InvertedBitmap::build(&arena);
    let mut sc_auto = BitmapScratch::new();
    let mut sc_off = BitmapScratch::new();
    sc_off.set_force_abandon(Some(false));
    let mut sc_on = BitmapScratch::new();
    sc_on.set_force_abandon(Some(true));
    for &t in &[0.3f64, 0.5, 0.7, 0.8, 0.85, 0.9, 0.95, 1.0] {
        for q in (0..arena.len()).step_by(137) {
            let query = arena.row(q).to_vec();
            let auto = hit_pairs(bm.search_threshold(&query, t, &mut sc_auto));
            let off = hit_pairs(bm.search_threshold(&query, t, &mut sc_off));
            let on = hit_pairs(bm.search_threshold(&query, t, &mut sc_on));
            assert_eq!(auto, off, "auto != forced-off (t={t} q={q})");
            assert_eq!(auto, on, "auto != forced-on abandon (t={t} q={q})");
        }
    }
}

#[test]
fn bitmap_exact_boundary_band() {
    // True Tanimoto == t exactly, on the BITMAP's own cmin/integer-gate path, at
    // several boundary thresholds (the round->floor false-negative class).
    let nbits = 64;
    let packed = |bits: &[usize]| {
        let mut w = vec![0u64; lanes_for_bits(nbits)];
        for &b in bits {
            w[b / 64] |= 1u64 << (b % 64);
        }
        w
    };
    // (query bits, target bits, exact Tanimoto) cases.
    let cases: &[(&[usize], &[usize], f64)] = &[
        (&[0, 1], &[0, 1, 2], 2.0 / 3.0),             // c=2,a=2,b=3 -> 2/3
        (&[0, 1, 2, 3], &[0, 1, 2, 3, 4], 4.0 / 5.0), // 4/5 = 0.8
        (
            &[0, 1, 2, 3, 4, 5, 6, 7, 8],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            9.0 / 10.0,
        ), // 0.9
    ];
    for &(qb, tb, tan) in cases {
        let arena = Arena::build(nbits, packed(tb), vec!["T".into()]);
        let bm = InvertedBitmap::build(&arena);
        let mut sc = BitmapScratch::new();
        let q = packed(qb);
        // At t == exact Tanimoto, the row MUST be returned (>= is inclusive).
        assert_eq!(
            idx_set(bm.search_threshold(&q, tan, &mut sc)),
            vec![0],
            "boundary t={tan}"
        );
        // Just above must drop it.
        assert!(
            bm.search_threshold(&q, tan + 1e-4, &mut sc).is_empty(),
            "above-boundary t={tan}"
        );
    }
}
