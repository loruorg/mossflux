//! Bit-transposed dense column store.
//!
//! Stores the fingerprint matrix column-major: for every bit-column `j`, a
//! `wpc`-word bitmap where bit `p` of word `w` is set iff record `w*64 + p` has
//! bit `j`. Same total size as the arena's packed words.
//!
//! Two query operators share it:
//!  - similarity (bit-sliced popcount intersection; see [`crate::InvertedBitmap`]),
//!  - substructure AND-containment (candidates = rows with all query bits set).

use rayon::prelude::*;

use crate::arena::Arena;

/// Dense bit-transposed column store.
///
/// Column `j` occupies `cols[j*wpc .. (j+1)*wpc]`; bit `p` of word `w` (i.e.
/// `cols[j*wpc + w] >> p & 1`) is set iff record `w*64 + p` has bit `j` set.
/// Record ids are `u32` (cap ~4.29B rows; shard beyond that).
#[derive(Clone, Debug)]
pub struct ColumnBitmap {
    n: usize,
    nbits: usize,
    /// Words per column = ceil(n / 64).
    wpc: usize,
    /// `nbits * wpc` words: the transpose of the fingerprint matrix.
    cols: Vec<u64>,
    /// Per-column popcount, for rarest-first column ordering.
    col_popcount: Vec<u32>,
}

impl ColumnBitmap {
    /// Transpose a popcount-sorted [`Arena`] into dense column bitmaps.
    pub fn build(arena: &Arena) -> ColumnBitmap {
        Self::from_rows(arena.len(), arena.nbits(), |i| arena.row(i))
    }

    /// Transpose ANY fixed-width row-major bit matrix into dense column bitmaps.
    ///
    /// `n` rows, `nbits` bit-columns; `row(i)` returns row `i`'s packed words
    /// (`ceil(nbits/64)` words each).
    ///
    /// Each rayon worker owns a disjoint range of columns and scans all rows, so
    /// the output is byte-identical to a serial transpose. `row` is called from
    /// every worker, so it must be a cheap, side-effect-free borrow.
    pub fn from_rows<'a, F>(n: usize, nbits: usize, row: F) -> ColumnBitmap
    where
        F: Fn(usize) -> &'a [u64] + Sync,
    {
        assert!(
            n <= u32::MAX as usize,
            "column bitmap caps at 2^32 rows; shard beyond that"
        );
        let wpc = n.div_ceil(64).max(1);
        let mut cols = vec![0u64; nbits * wpc];
        let threads = rayon::current_num_threads().max(1);
        let chunk_cols = nbits.div_ceil(threads).max(1);
        cols.par_chunks_mut(chunk_cols * wpc)
            .enumerate()
            .for_each(|(ci, chunk)| {
                let j0 = ci * chunk_cols;
                let cols_here = chunk.len() / wpc;
                for i in 0..n {
                    let w = i / 64;
                    let bit = 1u64 << (i % 64);
                    for (wi, &word) in row(i).iter().enumerate() {
                        let mut bits = word;
                        while bits != 0 {
                            let j = wi * 64 + bits.trailing_zeros() as usize;
                            bits &= bits - 1;
                            if j >= j0 && j - j0 < cols_here {
                                chunk[(j - j0) * wpc + w] |= bit;
                            }
                        }
                    }
                }
            });
        let col_popcount: Vec<u32> = (0..nbits)
            .into_par_iter()
            .map(|j| mossflux_core::popcount(&cols[j * wpc..(j + 1) * wpc]) as u32)
            .collect();
        ColumnBitmap {
            n,
            nbits,
            wpc,
            cols,
            col_popcount,
        }
    }

    /// Number of indexed records.
    #[inline]
    pub fn n(&self) -> usize {
        self.n
    }

    /// Number of bit-columns.
    #[inline]
    pub fn nbits(&self) -> usize {
        self.nbits
    }

    /// Words per column = ceil(n / 64).
    #[inline]
    pub fn wpc(&self) -> usize {
        self.wpc
    }

    /// Column `j`'s dense record bitmap: `wpc` words, bit `p` of word `w` set iff
    /// record `w*64 + p` has bit `j`.
    #[inline]
    pub fn column(&self, j: usize) -> &[u64] {
        &self.cols[j * self.wpc..(j + 1) * self.wpc]
    }

    /// The whole transpose as one flat slice. Column `j`, word `w` lives at
    /// `cols()[j*wpc + w]`.
    #[inline]
    pub fn cols(&self) -> &[u64] {
        &self.cols
    }

    /// Per-column popcount (records with that bit set).
    #[inline]
    pub fn col_popcount(&self, j: usize) -> u32 {
        self.col_popcount[j]
    }

    /// Approximate resident size in bytes of the transpose + per-column popcounts.
    #[inline]
    pub fn mem_bytes(&self) -> usize {
        self.cols.len() * 8 + self.col_popcount.len() * 4
    }
}
