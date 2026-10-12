//! Tests for `Arena::folded` (OR-fold), `Arena::search_knn` exactness,
//! `Arena::build_with_order`, and the `BitmapScratch::words_read` bytes/molecule metric.

use mossflux_index::mossflux_core::{intersection_popcount, lanes_for_bits, popcount};
use mossflux_index::{Arena, BitmapScratch, InvertedBitmap};

fn packed(bits: &[usize], nbits: usize) -> Vec<u64> {
    let mut w = vec![0u64; lanes_for_bits(nbits)];
    for &b in bits {
        w[b / 64] |= 1u64 << (b % 64);
    }
    w
}

/// Independent OR-fold: folded bit = original_bit % new_nbits, OR'd over set bits
/// (ignoring any bit >= nbits, mirroring `Arena::folded`'s `b < nbits` guard).
fn or_fold(flat: &[u64], nbits: usize, new_nbits: usize, n: usize) -> Vec<u64> {
    let nw = lanes_for_bits(nbits);
    let nnw = lanes_for_bits(new_nbits);
    let mut out = vec![0u64; n * nnw];
    for i in 0..n {
        for wi in 0..nw {
            let mut bits = flat[i * nw + wi];
            while bits != 0 {
                let b = wi * 64 + bits.trailing_zeros() as usize;
                bits &= bits - 1;
                if b < nbits {
                    let fb = b % new_nbits;
                    out[i * nnw + fb / 64] |= 1u64 << (fb % 64);
                }
            }
        }
    }
    out
}

#[test]
fn folded_or_fold_handcomputed() {
    // 2048 -> 256: bits 5, 261(=5+256), 517(=5+2*256) all fold to 5; 300 -> 44.
    let arena = Arena::build(2048, packed(&[5, 261, 517, 300], 2048), vec!["x".into()]);
    let f = arena.folded(256);
    assert_eq!(f.nbits(), 256);
    assert_eq!(f.row(0), &packed(&[5, 44], 256)[..], "OR-fold bits wrong");
}

#[test]
fn folded_ignores_spare_high_bits() {
    // nbits=166 (3 words = 192 bits); bit 180 is a SPARE bit (>= 166). The
    // `b < nbits` guard must ignore it so it cannot fabricate a folded bit.
    let arena = Arena::build(166, packed(&[10, 180], 166), vec!["x".into()]);
    let f = arena.folded(83);
    // Only the valid bit 10 folds (-> 10); spare bit 180 (would be 180%83=14) is dropped.
    assert_eq!(
        f.row(0),
        &packed(&[10], 83)[..],
        "spare high bit must not fold"
    );
}

#[test]
fn folded_search_equiv_to_manual_fold() {
    // Folding then searching must match independently OR-folding then searching,
    // across non-divisor widths and the full-Tanimoto path (linear).
    let nbits = 2048;
    let n = 400;
    for &new in &[256usize, 166] {
        let arena = Arena::synthetic(n, nbits, 0.02, 7);
        let nw = lanes_for_bits(nbits);
        let mut flat = vec![0u64; n * nw];
        for i in 0..n {
            flat[i * nw..(i + 1) * nw].copy_from_slice(arena.row(i));
        }
        let manual = Arena::build(
            new,
            or_fold(&flat, nbits, new, n),
            (0..n).map(|i| i.to_string()).collect(),
        );
        let folded = arena.folded(new);
        let q = folded.row(3).to_vec();
        for &t in &[0.3_f64, 0.6, 0.8] {
            let mut s1: Vec<f64> = folded
                .search_threshold(&q, t)
                .iter()
                .map(|h| h.score)
                .collect();
            let mut s2: Vec<f64> = manual
                .search_threshold(&q, t)
                .iter()
                .map(|h| h.score)
                .collect();
            s1.sort_by(|a, b| a.total_cmp(b));
            s2.sort_by(|a, b| a.total_cmp(b));
            assert_eq!(
                s1, s2,
                "folded vs manual-fold score set differs (new={new}, t={t})"
            );
        }
    }
}

#[test]
fn build_masks_spare_high_bits() {
    // nbits=70 (2 lanes = 128 bits). A row with a valid bit (5) AND a spare bit
    // (100 >= 70) must have the spare masked by assemble(), else its popcount
    // inflates, it bins above nbits, and the band scan (hi clamped to nbits)
    // diverges from the brute scan (recall < 1.0).
    let nbits = 70;
    let arena = Arena::build(nbits, packed(&[5, 100], nbits), vec!["r".into()]);
    assert_eq!(
        arena.popcount_of(0),
        1,
        "spare bit must be masked out of popcount"
    );
    let q = packed(&[5], nbits);
    let pruned = arena.search_threshold(&q, 0.5);
    let brute = arena.search_threshold_brute(&q, 0.5);
    assert_eq!(pruned.len(), 1, "self must match");
    assert_eq!(
        pruned.len(),
        brute.len(),
        "pruned vs brute diverge on spare bits"
    );
}

/// Independent exact top-k: score EVERY row with no heap, no band pruning and no
/// early termination, then apply the documented total order (score descending,
/// then row index ascending) and truncate. This is a different algorithm from
/// `search_knn`, so agreement is a real differential rather than self-agreement.
/// Scores are f64: at 256 bits no two distinct fractions share an f64, so the
/// f64 order is the exact order.
fn brute_knn(arena: &Arena, query: &[u64], k: usize) -> Vec<(u32, u64)> {
    let a = popcount(query);
    if a == 0 || k == 0 {
        return Vec::new();
    }
    let mut all: Vec<(u32, f64)> = (0..arena.len())
        .map(|i| {
            let b = arena.popcount_of(i) as u32;
            let c = intersection_popcount(query, arena.row(i));
            let denom = a + b - c;
            let score = if denom == 0 {
                0.0
            } else {
                c as f64 / denom as f64
            };
            (i as u32, score)
        })
        .collect();
    all.sort_by(|x, y| y.1.total_cmp(&x.1).then(x.0.cmp(&y.0)));
    all.truncate(k);
    all.into_iter().map(|(i, s)| (i, s.to_bits())).collect()
}

#[test]
fn knn_k_edges_and_exactness() {
    let arena = Arena::synthetic(500, 256, 0.12, 21);
    let q = arena.row(3).to_vec();
    assert!(arena.search_knn(&q, 0).is_empty(), "k=0 -> empty");
    let kn = arena.search_knn(&q, arena.len());
    assert_eq!(kn.len(), arena.len(), "k==n must return all rows");
    assert_eq!(
        arena.search_knn(&q, arena.len() + 50).len(),
        arena.len(),
        "k>n must return exactly n (not panic, not k)"
    );
    for w in kn.windows(2) {
        assert!(w[0].score >= w[1].score, "knn not sorted descending");
    }
    assert_eq!(kn[0].score, 1.0, "nearest of self must be exactly 1.0");
    let zero = vec![0u64; lanes_for_bits(256)];
    assert!(
        arena.search_knn(&zero, 10).is_empty(),
        "all-zero query -> empty"
    );

    // The pruned/heap top-k must agree with the exhaustive top-k.
    //
    // kNN is exact in ROWS, not only in scores: a tie at the k-th position goes
    // to the lower row (`search_knn` docs), so the result is the first k rows of
    // the brute total order even when a tie straddles the cut. The tied case
    // is counted separately so the test proves it really ran.
    let mut compared = 0usize;
    let mut unambiguous = 0usize;
    let mut tied_boundary = 0usize;
    for qi in (0..arena.len()).step_by(97) {
        let query = arena.row(qi).to_vec();
        let a = popcount(&query);
        let full = brute_knn(&arena, &query, arena.len());
        for &k in &[1usize, 2, 20, 137, arena.len()] {
            let got: Vec<(u32, u64)> = arena
                .search_knn(&query, k)
                .iter()
                .map(|h| (h.index, h.score.to_bits()))
                .collect();
            let want = brute_knn(&arena, &query, k);
            let ctx = format!("query row {qi}, k={k}");

            // (1) The k score values, in order, are byte-identical to exhaustive.
            let got_scores: Vec<u64> = got.iter().map(|&(_, s)| s).collect();
            let want_scores: Vec<u64> = want.iter().map(|&(_, s)| s).collect();
            assert_eq!(got_scores, want_scores, "top-k SCORES differ ({ctx})");

            // (2) Rows are distinct and each reported score is that row's true score.
            let set: std::collections::BTreeSet<u32> = got.iter().map(|&(i, _)| i).collect();
            assert_eq!(set.len(), got.len(), "duplicate rows in knn result ({ctx})");
            for &(idx, bits) in &got {
                let b = arena.popcount_of(idx as usize) as u32;
                let c = intersection_popcount(&query, arena.row(idx as usize));
                let denom = a + b - c;
                let truth = if denom == 0 {
                    0.0f64
                } else {
                    c as f64 / denom as f64
                };
                assert_eq!(
                    bits,
                    truth.to_bits(),
                    "row {idx} carries a wrong score ({ctx})"
                );
            }

            // (3) Total order: score descending, then index ascending.
            for w in got.windows(2) {
                let (ia, sa) = w[0];
                let (ib, sb) = w[1];
                let (fa, fb) = (f64::from_bits(sa), f64::from_bits(sb));
                assert!(
                    fa > fb || (fa == fb && ia < ib),
                    "knn not in (score desc, index asc) order at {ia}/{ib} ({ctx})"
                );
            }

            // (4) Rows: exact, also when the k-th score is tied across the cut.
            assert_eq!(got, want, "top-k ROWS differ ({ctx})");
            if k < full.len() && full[k - 1].1 == full[k].1 {
                tied_boundary += 1;
            } else {
                unambiguous += 1;
            }
            compared += 1;
        }
    }
    assert!(
        compared >= 25,
        "knn differential ran only {compared} comparisons"
    );
    assert!(
        unambiguous >= 10,
        "only {unambiguous} tie-free cases — the exact row-set check barely ran"
    );
    assert!(
        tied_boundary > 0,
        "no boundary tie was exercised; the tie branch is untested"
    );
}

#[test]
fn build_with_order_aligns_payload() {
    // order[sorted_row] = input_row, so a parallel payload permuted by `order`
    // lines up with the molecule the arena stores at each sorted row — the
    // alignment a substructure molecule-store depends on.
    let nbits = 128;
    let nw = lanes_for_bits(nbits);
    let n = 50;
    let mut flat = vec![0u64; n * nw];
    for i in 0..n {
        for b in 0..(i % 40) {
            flat[i * nw + b / 64] |= 1u64 << (b % 64); // varying popcount -> real permutation
        }
    }
    let ids: Vec<String> = (0..n).map(|i| format!("M{i}")).collect();
    let (arena, order) = Arena::build_with_order(nbits, flat, ids.clone());
    assert_eq!(order.len(), arena.len());
    for sorted in 0..arena.len() {
        assert_eq!(
            arena.id(sorted),
            ids[order[sorted] as usize],
            "order misaligns ids"
        );
    }
    let mut seen = order.clone();
    seen.sort_unstable();
    assert_eq!(
        seen,
        (0..n as u32).collect::<Vec<u32>>(),
        "order is not a permutation"
    );
    for i in 1..arena.len() {
        assert!(arena.popcount_of(i - 1) <= arena.popcount_of(i));
    }
}

#[test]
fn words_read_counts_band_columns() {
    // The bytes/molecule metric = words_read * 8. At t=0 the whole arena is in-band
    // (band words == words-per-column) and abandon is off, so the vectorized Phase A
    // touches exactly (query popcount) columns x (wpc) words — a hand-checkable value.
    let n = 200usize;
    let nbits = 256;
    let arena = Arena::synthetic(n, nbits, 0.1, 3);
    let bm = InvertedBitmap::build(&arena);
    let mut sc = BitmapScratch::new();
    sc.set_force_abandon(Some(false)); // full vectorized band, Phase B empty
    let q = arena.row(5).to_vec();
    sc.reset_words_read();
    let _ = bm.search_threshold(&q, 0.0, &mut sc);
    let wpc = n.div_ceil(64) as u64;
    let a = popcount(&q) as u64;
    assert_eq!(
        sc.words_read(),
        a * wpc,
        "words_read should equal (query popcount) x (words-per-column) at t=0"
    );
    assert!(a > 0 && wpc == 4);
}
