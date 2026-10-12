//! The documented panics of the similarity API: a query of the wrong width has
//! no answer, so every path refuses it the same way, rather than one path
//! zero-extending it into a different fingerprint.

use mossflux_index::mossflux_core::{tanimoto, union_popcount};
use mossflux_index::{
    Arena, BitmapScratch, InvertedBitmap, InvertedIndex, InvertedScratch, SegmentedIndex,
};

fn arena128() -> Arena {
    Arena::build(128, vec![0b11, 0, 0b111, 0], vec!["X".into(), "Y".into()])
}

#[test]
#[should_panic(expected = "query width != index width")]
fn inverted_bitmap_refuses_a_narrow_query() {
    let a = arena128();
    let _ = InvertedBitmap::build(&a).search_threshold(&[0b11], 0.5, &mut BitmapScratch::new());
}

#[test]
#[should_panic(expected = "query width != index width")]
fn inverted_index_refuses_a_wide_query() {
    let a = arena128();
    let _ = InvertedIndex::build(&a).search_threshold(
        &[0b11, 0, 0],
        0.5,
        &mut InvertedScratch::new(a.len()),
    );
}

#[test]
#[should_panic(expected = "query width != arena width")]
fn arena_refuses_a_narrow_query() {
    let _ = arena128().search_knn(&[0b11], 1);
}

/// With no segment there is no per-segment check, and the documented panic
/// must not depend on that.
#[test]
#[should_panic(expected = "query width != index width")]
fn empty_segmented_index_still_checks_threshold_width() {
    let _ = SegmentedIndex::new(128).search_threshold(&[0b11], 0.5, &mut BitmapScratch::new());
}

#[test]
#[should_panic(expected = "query width != index width")]
fn segmented_knn_checks_width_even_for_k_zero() {
    let _ = SegmentedIndex::new(128).search_knn(&[0b11], 0);
}

#[test]
#[should_panic(expected = "fingerprints must have equal width")]
fn tanimoto_refuses_mismatched_widths() {
    let _ = tanimoto(&[1, 0], &[1]);
}

#[test]
#[should_panic(expected = "fingerprints must have equal width")]
fn union_popcount_refuses_mismatched_widths() {
    let _ = union_popcount(&[1, 0], &[1]);
}
