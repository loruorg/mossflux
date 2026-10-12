//! Exactness + capability tests for the incremental-growth `SegmentedIndex`:
//! appending molecules in batches must give a BYTE-IDENTICAL hit set to a single
//! monolithic arena over the same molecules, and the append must be O(batch)
//! (one new segment, existing segments untouched).

use std::collections::BTreeSet;

use mossflux_index::mossflux_core::lanes_for_bits;
use mossflux_index::{Arena, BitmapScratch, SegmentedIndex};

/// Deterministic sparse fingerprints + unique ids (no rand dep).
fn make(n: usize, nbits: usize, seed: u64) -> (Vec<u64>, Vec<String>) {
    let nw = lanes_for_bits(nbits);
    let mut s = seed | 1;
    let mut flat = vec![0u64; n * nw];
    for w in flat.iter_mut() {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let t = s.wrapping_mul(0x9E3779B97F4A7C15);
        *w = s & t & s.rotate_left(17); // ~sparse, ECFP-like
    }
    let ids = (0..n).map(|i| format!("M{i}")).collect();
    (flat, ids)
}

fn build_segmented(flat: &[u64], ids: &[String], nbits: usize, batch: usize) -> SegmentedIndex {
    let nw = lanes_for_bits(nbits);
    let n = ids.len();
    let mut seg = SegmentedIndex::new(nbits);
    let mut start = 0;
    while start < n {
        let end = (start + batch).min(n);
        seg.append(
            flat[start * nw..end * nw].to_vec(),
            ids[start..end].to_vec(),
        );
        start = end;
    }
    seg
}

#[test]
fn segmented_threshold_equals_monolith() {
    let nbits = 256;
    let n = 2000;
    let (flat, ids) = make(n, nbits, 99);
    let nw = lanes_for_bits(nbits);
    let monolith = Arena::build(nbits, flat.clone(), ids.clone());
    // Non-aligned batch size so segments don't fall on convenient boundaries.
    let seg = build_segmented(&flat, &ids, nbits, 277);
    assert_eq!(seg.len(), n as u64);
    assert!(seg.segment_count() >= 7, "expected several segments");

    let mut scratch = BitmapScratch::new();
    for &t in &[0.3f64, 0.5, 0.7, 0.85, 0.9, 0.95, 1.0] {
        for qi in (0..n).step_by(311) {
            let q = &flat[qi * nw..(qi + 1) * nw];
            // Compare (id, exact-score-bits) SETS — partition-independent.
            let mono: BTreeSet<(String, u64)> = monolith
                .search_threshold(q, t)
                .iter()
                .map(|h| (monolith.id(h.index as usize), h.score.to_bits()))
                .collect();
            let segd: BTreeSet<(String, u64)> = seg
                .search_threshold(q, t, &mut scratch)
                .iter()
                .map(|h| (seg.id(h.index), h.score.to_bits()))
                .collect();
            assert_eq!(mono, segd, "segmented != monolith at t={t}, query {qi}");
        }
    }
}

#[test]
fn segmented_knn_scores_equal_monolith() {
    let nbits = 256;
    let n = 1500;
    let (flat, ids) = make(n, nbits, 7);
    let nw = lanes_for_bits(nbits);
    let monolith = Arena::build(nbits, flat.clone(), ids.clone());
    let seg = build_segmented(&flat, &ids, nbits, 201);

    for &k in &[1usize, 10, 50] {
        for qi in (0..n).step_by(409) {
            let q = &flat[qi * nw..(qi + 1) * nw];
            // The tie rule (score, then popcount, then input order) does not
            // depend on the partition, so the ROWS and their order match too.
            let mono: Vec<(String, u64)> = monolith
                .search_knn(q, k)
                .iter()
                .map(|h| (monolith.id(h.index as usize), h.score.to_bits()))
                .collect();
            let segd: Vec<(String, u64)> = seg
                .search_knn(q, k)
                .iter()
                .map(|h| (seg.id(h.index), h.score.to_bits()))
                .collect();
            assert_eq!(mono, segd, "knn rows differ k={k}, query {qi}");
        }
    }
    // k edges on the segmented index.
    assert!(seg.search_knn(&flat[0..nw], 0).is_empty(), "k=0 -> empty");
    assert_eq!(
        seg.search_knn(&flat[0..nw], n + 50).len() as u64,
        seg.len(),
        "k>=total must return all rows"
    );
}

#[test]
fn append_is_incremental_not_rebuild() {
    // Each non-empty append adds exactly one segment; empty appends are ignored;
    // ids round-trip through the global-id mapping.
    let nbits = 128;
    let mut seg = SegmentedIndex::new(nbits);
    assert!(seg.is_empty());
    let mut expect_total = 0u64;
    for (b, &batch) in [50usize, 1, 300, 64, 65].iter().enumerate() {
        let (flat, ids) = make(batch, nbits, 1000 + b as u64);
        seg.append(flat, ids);
        expect_total += batch as u64;
        assert_eq!(seg.len(), expect_total);
        assert_eq!(seg.segment_count(), b + 1);
    }
    // Empty append is a no-op (no phantom segment).
    seg.append(vec![0u64; 0], vec![]);
    assert_eq!(seg.segment_count(), 5);
    // Global id mapping: segment 1 holds a single molecule (batch size 1), so its
    // only global id (50) maps back to that batch's sole id "M0". (Row 0 of a
    // multi-row segment is the lowest-popcount molecule, not necessarily "M0".)
    assert_eq!(seg.id(50), "M0");
}

/// Re-partition invariance: layout never changes the hit set.
///
/// The same rows laid out nine ways (eight segment batch sizes, including the
/// degenerate `segment_count() == n` and `== 1`, plus one monolithic arena) must
/// produce one identical `(id, exact score bits)` hit set. Any compactor must
/// preserve this.
#[test]
fn hit_set_is_invariant_under_repartition() {
    let nbits = 256;
    let n = 800;
    let (flat, ids) = make(n, nbits, 4242);
    let nw = lanes_for_bits(nbits);
    let monolith = Arena::build(nbits, flat.clone(), ids.clone());

    // batch == 1 -> one segment per row; batch == n -> a single segment.
    let batches = [1usize, 2, 63, 64, 65, 277, 799, 800];
    let segs: Vec<SegmentedIndex> = batches
        .iter()
        .map(|&b| build_segmented(&flat, &ids, nbits, b))
        .collect();
    assert_eq!(
        segs[0].segment_count(),
        n,
        "batch=1 must give one segment per row"
    );
    assert_eq!(
        segs[7].segment_count(),
        1,
        "batch=n must give a single segment"
    );

    let mut scratch = BitmapScratch::new();
    let mut nonempty = 0usize;
    for &t in &[0.0f64, 0.3, 0.5, 0.7, 0.85, 0.95, 1.0] {
        for qi in (0..n).step_by(89) {
            let q = &flat[qi * nw..(qi + 1) * nw];
            let reference: BTreeSet<(String, u64)> = monolith
                .search_threshold(q, t)
                .iter()
                .map(|h| (monolith.id(h.index as usize), h.score.to_bits()))
                .collect();
            for (bi, seg) in segs.iter().enumerate() {
                let got: BTreeSet<(String, u64)> = seg
                    .search_threshold(q, t, &mut scratch)
                    .iter()
                    .map(|h| (seg.id(h.index), h.score.to_bits()))
                    .collect();
                assert_eq!(
                    got, reference,
                    "layout batch={} differs from the monolith at t={t}, query {qi}",
                    batches[bi]
                );
            }
            if !reference.is_empty() {
                nonempty += 1;
            }
        }
    }
    // Anti-vacuity: the comparisons above were not all `{} == {}`.
    assert!(
        nonempty > 20,
        "only {nonempty} non-empty reference hit sets"
    );
}

/// The append-only guarantee: a hit before an append is still a hit after it,
/// with the same global index, the same external id and a byte-identical score.
/// Catches a `base[]`/prefix-sum off-by-one that would slide earlier global ids.
#[test]
fn append_never_changes_an_existing_hit() {
    let nbits = 256;
    let nw = lanes_for_bits(nbits);
    let mut seg = SegmentedIndex::new(nbits);
    let mut scratch = BitmapScratch::new();

    // Pre-generate every batch, so the queries can be REAL rows of the first
    // batch: each is then guaranteed to match itself (Tanimoto 1.0) at every
    // threshold, which is what makes "the hit survives the next append" a
    // non-vacuous claim.
    let batches: Vec<(Vec<u64>, Vec<String>)> = [50usize, 1, 300, 64, 65, 277, 2, 500]
        .iter()
        .enumerate()
        .map(|(step, &b)| make(b, nbits, 900 + step as u64))
        .collect();
    let queries: Vec<Vec<u64>> = (0..10)
        .map(|i| batches[0].0[i * nw..(i + 1) * nw].to_vec())
        .collect();
    let ts = [0.2f64, 0.5, 1.0];

    // (query, t) -> the results recorded after the previous append.
    let mut prev: Vec<Vec<(u64, String, u64)>> = vec![Vec::new(); queries.len() * ts.len()];
    let mut total = 0u64;
    let mut carried = 0usize;
    let mut grew = 0usize;

    for (step, (flat, ids)) in batches.iter().enumerate() {
        let batch = ids.len();
        seg.append(flat.clone(), ids.clone());
        let before = total;
        total += batch as u64;
        assert_eq!(seg.len(), total, "len must be the running total");
        assert!(
            total > before,
            "len must grow strictly on a non-empty append"
        );

        for (qi, q) in queries.iter().enumerate() {
            for (ti, &t) in ts.iter().enumerate() {
                let slot = qi * ts.len() + ti;
                let now: Vec<(u64, String, u64)> = {
                    let mut v: Vec<(u64, String, u64)> = seg
                        .search_threshold(q, t, &mut scratch)
                        .iter()
                        .map(|h| (h.index, seg.id(h.index), h.score.to_bits()))
                        .collect();
                    v.sort();
                    v
                };
                // Every previously-returned hit survives unchanged: same global
                // index, same id, same score bits.
                let now_set: BTreeSet<&(u64, String, u64)> = now.iter().collect();
                for old in &prev[slot] {
                    assert!(
                        now_set.contains(old),
                        "append {step} (batch {batch}) changed an existing hit: \
                         {old:?} is gone from query {qi} at t={t}"
                    );
                    carried += 1;
                }
                // Anything newly returned must live in the rows just appended.
                for (gidx, _, _) in &now {
                    if !prev[slot].iter().any(|(g, _, _)| g == gidx) {
                        assert!(
                            *gidx >= before,
                            "append {step} produced a NEW hit at global index \
                             {gidx}, below the pre-append total {before}"
                        );
                        grew += 1;
                    }
                }
                prev[slot] = now;
            }
        }
    }
    assert_eq!(total, 1259, "eight batches should total 1259 rows");
    // Anti-vacuity: hits really were carried across appends, and really were added.
    assert!(
        carried > 200,
        "only {carried} hits were carried across appends"
    );
    assert!(
        grew > 20,
        "only {grew} new hits appeared across eight appends"
    );
}

#[test]
#[should_panic(expected = "out of range")]
fn id_one_past_end_panics_cleanly() {
    // `id(len())` is the global id ONE PAST the last valid row. Valid ids are
    // [0, len()); passing len() must fail the explicit bounds check, not index out
    // of bounds deep inside an Arena.
    let nbits = 128;
    let (flat, ids) = make(50, nbits, 7);
    let mut seg = SegmentedIndex::new(nbits);
    seg.append(flat, ids);
    assert_eq!(seg.len(), 50);
    let _ = seg.id(seg.len());
}
