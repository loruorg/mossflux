//! Adversarial edge-case probes for InvertedBitmap vs linear scan.
//! Black-box: only uses public API.

use mossflux_index::{Arena, BitmapScratch, Hit, InvertedBitmap};

fn lanes(nbits: usize) -> usize {
    nbits.div_ceil(64)
}

fn fp(bits: &[usize], nbits: usize) -> Vec<u64> {
    let mut w = vec![0u64; lanes(nbits)];
    for &b in bits {
        w[b / 64] |= 1u64 << (b % 64);
    }
    w
}

fn sorted(mut h: Vec<Hit>) -> Vec<(u32, f64)> {
    h.sort_by_key(|x| x.index);
    h.into_iter().map(|x| (x.index, x.score)).collect()
}

fn just_idx(h: Vec<Hit>) -> Vec<u32> {
    let mut v: Vec<u32> = h.into_iter().map(|x| x.index).collect();
    v.sort();
    v
}

/// Build arena from a list of (bit-set) fingerprints with given nbits.
fn build(fps: &[Vec<usize>], nbits: usize) -> Arena {
    let mut flat = Vec::new();
    let mut ids = Vec::new();
    for (i, b) in fps.iter().enumerate() {
        flat.extend_from_slice(&fp(b, nbits));
        ids.push(format!("M{i}"));
    }
    Arena::build(nbits, flat, ids)
}

fn check(fps: &[Vec<usize>], nbits: usize, queries: &[Vec<usize>], ts: &[f64], label: &str) {
    let arena = build(fps, nbits);
    let bm = InvertedBitmap::build(&arena);
    let mut sc = BitmapScratch::new();
    for q in queries {
        let query = fp(q, nbits);
        for &t in ts {
            let lin = just_idx(arena.search_threshold(&query, t));
            let b = just_idx(bm.search_threshold(&query, t, &mut sc));
            assert_eq!(lin, b, "{label}: idx mismatch q={q:?} t={t}");
            // also scores
            let ls = sorted(arena.search_threshold(&query, t));
            let bs = sorted(bm.search_threshold(&query, t, &mut sc));
            assert_eq!(ls, bs, "{label}: score mismatch q={q:?} t={t}");
        }
    }
}

// n=1
#[test]
fn n1_single_record() {
    let fps = vec![vec![1usize, 5, 9]];
    let queries = vec![vec![1, 5, 9], vec![1, 5], vec![100], vec![]];
    check(&fps, 256, &queries, &[0.1, 0.5, 0.7, 1.0], "n=1");
}

// all-ones column: a bit set in EVERY record
#[test]
fn all_ones_column() {
    // bit 0 set in every record; vary other bits
    let mut fps = Vec::new();
    for i in 0..100 {
        let mut v = vec![0usize];
        for k in 0..(i % 10) {
            v.push(10 + k);
        }
        fps.push(v);
    }
    let queries = vec![vec![0], vec![0, 11, 12], vec![0, 10, 11, 12, 13]];
    check(
        &fps,
        128,
        &queries,
        &[0.1, 0.3, 0.5, 0.7, 0.9, 1.0],
        "all-ones-col",
    );
}

// all-zero column for queried bit handled? query a bit no record sets
#[test]
fn query_unset_bit() {
    let fps = vec![vec![1, 2, 3], vec![1, 2], vec![2, 3, 4]];
    let queries = vec![vec![1, 2, 99], vec![99, 100]];
    check(
        &fps,
        128,
        &queries,
        &[0.1, 0.3, 0.5, 1.0],
        "query-unset-bit",
    );
}

// n not a multiple of 64 (boundary word masking)
#[test]
fn n_not_multiple_64() {
    for n in [63usize, 65, 100, 127, 129] {
        let mut fps = Vec::new();
        for i in 0..n {
            let mut v = vec![0usize];
            v.push(1 + (i % 20));
            v.push(30 + (i % 7));
            fps.push(v);
        }
        let queries: Vec<Vec<usize>> = (0..n).step_by(7).map(|i| fps[i].clone()).collect();
        check(
            &fps,
            128,
            &queries,
            &[0.2, 0.5, 0.7, 1.0],
            &format!("n={n}"),
        );
    }
}

// query bit beyond nbits: directly craft a query word with a high bit set.
#[test]
fn query_bit_beyond_nbits() {
    // nbits=70 -> 2 words; bits 70,71,...127 are "beyond nbits" in word 1.
    let nbits = 70;
    let arena = build(&[vec![1, 2, 3], vec![1, 2], vec![3, 4, 5]], nbits);
    let bm = InvertedBitmap::build(&arena);
    let mut sc = BitmapScratch::new();
    // Manually set a bit beyond nbits in the query (bit 100, within word 1).
    let mut query = fp(&[1, 2], nbits);
    query[1] |= 1u64 << (100 - 64); // spare bit beyond nbits=70
    let lin = just_idx(arena.search_threshold(&query, 0.5));
    let b = just_idx(bm.search_threshold(&query, 0.5, &mut sc));
    assert_eq!(lin, b, "query bit beyond nbits");
}

// p-plane count exactly at a power of two: a = 2,4,8,16 -> p = 2,3,4,5
#[test]
fn power_of_two_popcount() {
    for a in [1usize, 2, 4, 8, 16, 32] {
        let mut fps = Vec::new();
        // records that overlap the query to various degrees
        for i in 0..200 {
            let mut v = Vec::new();
            let overlap = i % (a + 1);
            for k in 0..overlap {
                v.push(k);
            }
            // extra non-overlapping bits
            for k in 0..(i % 5) {
                v.push(100 + k);
            }
            fps.push(v);
        }
        let query: Vec<usize> = (0..a).collect();
        check(
            &fps,
            256,
            &[query],
            &[0.1, 0.25, 0.5, 0.75, 1.0],
            &format!("a={a}"),
        );
    }
}

// band spanning exactly one word, and bands with cmin>a behavior at high t.
#[test]
fn dense_high_threshold() {
    let mut fps = Vec::new();
    for i in 0..300 {
        let mut v: Vec<usize> = (0..30).collect();
        // perturb a few bits
        for k in 0..(i % 11) {
            v.push(30 + k);
        }
        fps.push(v);
    }
    let q: Vec<usize> = (0..30).collect();
    check(
        &fps,
        256,
        &[q],
        &[0.5, 0.7, 0.8, 0.9, 0.95, 0.99, 1.0],
        "dense-high-t",
    );
}

// Tautomer/charge: same molecule, multiple distinct fingerprints.
// They must all be returned at appropriate thresholds.
#[test]
fn multiple_fingerprints_one_molecule() {
    // Two fingerprints differing in a few bits (tautomers), plus decoys.
    let taut_a = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let taut_b = vec![1, 2, 3, 4, 5, 6, 7, 99]; // differs in 1 bit
    let mut fps = vec![taut_a.clone(), taut_b.clone()];
    for i in 0..50 {
        fps.push(vec![200 + (i % 30), 210 + (i % 20)]);
    }
    check(
        &fps,
        256,
        &[taut_a, taut_b],
        &[0.1, 0.5, 0.7, 0.8, 0.9, 1.0],
        "tautomers",
    );
}

// Larger fuzz over many n values including all-64-boundaries.
#[test]
fn fuzz_boundary_ns() {
    for n in [1usize, 2, 63, 64, 65, 128, 129, 191, 192, 193, 256, 257] {
        let arena = Arena::synthetic(n, 256, 0.12, 0xBEEF ^ n as u64);
        let bm = InvertedBitmap::build(&arena);
        let mut sc = BitmapScratch::new();
        for &t in &[0.2_f64, 0.35, 0.5, 0.65, 0.8, 0.95] {
            for q in (0..arena.len()).step_by(3) {
                let query = arena.row(q).to_vec();
                let lin = just_idx(arena.search_threshold(&query, t));
                let b = just_idx(bm.search_threshold(&query, t, &mut sc));
                assert_eq!(lin, b, "fuzz n={n} t={t} q={q}");
            }
        }
    }
}
