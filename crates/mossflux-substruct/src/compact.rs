//! A cache-local compact molecule for the matcher's hot path.
//!
//! [`MolGraph`] is a struct of arrays with many separate allocations, so one VF2
//! candidate touches several cold cache lines. [`MatchMol`] packs everything the
//! matcher reads into one contiguous byte blob per molecule: each atom's scalars
//! and its neighbour list, with bond order and flags inlined, sit together. It is
//! built once at index time from the perceived [`MolGraph`] and is the only form
//! the production matcher reads; `MolGraph` serves the brute-force oracle and the
//! build pipeline.
//!
//! No unsafe code: reads use `from_le_bytes` on `&[u8]` slices. The layout is
//! checked field by field against the source `MolGraph` by
//! `tests::matchmol_matches_molgraph`.
//!
//! # Blob layout (all little-endian)
//!
//! ```text
//! [ atom_off : u32 × (n_atoms + 1) ]   // atom_off[t] = byte offset of atom t's record
//! [ atom records … ]                   // record for atom t at blob[atom_off[t]]
//! ```
//!
//! Each atom record:
//!
//! ```text
//! element : u8
//! flags   : u8        // packed atom_flags (aromatic / in-ring)
//! degree  : u8
//! h_count : u8
//! charge  : i8 (as u8)
//! n_nbr   : u8
//! [ neighbour : (atom u16, order u8, bond_flags u8) × n_nbr ]   // bond data INLINED
//! ```
//!
//! Ring sizes (`r<n>`), valence (`v<n>`) and ring count (`R<n>`) live in separate
//! cold arrays.

use crate::graph::{atom_flags, bond_flags, AtomView, MolGraph};

/// One neighbour of an atom, with the connecting bond's order and flags inlined.
#[derive(Clone, Copy, Debug)]
pub struct Nbr {
    /// The neighbour atom's index.
    pub atom: u32,
    /// The connecting bond's order code.
    pub order: u8,
    /// The connecting bond's packed flags (aromatic / in-ring).
    pub flags: u8,
}

impl Nbr {
    /// True iff the connecting bond is aromatic.
    #[inline]
    pub fn bond_is_aromatic(&self) -> bool {
        self.flags & bond_flags::AROMATIC != 0
    }
    /// True iff the connecting bond is in a ring.
    #[inline]
    pub fn bond_in_ring(&self) -> bool {
        self.flags & bond_flags::IN_RING != 0
    }
}

/// Bytes per inlined neighbour entry: atom u16 + order u8 + flags u8.
const NBR_BYTES: usize = 4;
/// Bytes of fixed per-atom header before the neighbour entries.
const ATOM_HDR: usize = 6;

/// A molecule packed for cache-local matching. See the [module docs](self).
#[derive(Clone, Debug, Default)]
pub struct MatchMol {
    /// The contiguous blob: atom-offset table followed by the atom records.
    blob: Box<[u8]>,
    /// Number of atoms.
    n_atoms: u32,
    /// CSR of distinct SSSR ring sizes per atom (`r<n>`): atom `i`'s sizes are
    /// `ring_sizes[ring_off[i] .. ring_off[i + 1]]`.
    ring_off: Box<[u32]>,
    ring_sizes: Box<[u8]>,
    /// Canonical aromatic-ring tokens, copied from [`MolGraph::arom_rings`], for
    /// the matcher's ring-skip fast path.
    pub arom_rings: Box<[Box<[u8]>]>,
    /// FNV-1a hash of each token in [`Self::arom_rings`], same order. The ring-skip
    /// scan compares hashes first and byte-compares a token only on a hash hit, so
    /// the hash is a pure prefilter.
    pub arom_ring_hashes: Box<[u64]>,
    /// Per-atom `v<n>` and `R<n>` values, kept out of the hot record. See
    /// [`MolGraph::valence`] and [`MolGraph::ring_count`].
    valence: Box<[u8]>,
    ring_count: Box<[u8]>,
}

#[inline]
fn rd_u16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}
#[inline]
fn rd_u32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

impl MatchMol {
    /// Builds the compact form from a fully perceived [`MolGraph`].
    ///
    /// Returns `None` if the molecule exceeds the blob limits (`>= 65_535` atoms, or
    /// an atom with `> 255` neighbours), where the `u16` index or `u8` count would
    /// silently wrap. A bulk ingest can then skip and count the row instead of
    /// aborting. The degree check reads the true neighbour-slice length, not the
    /// u8-saturated `mol.degree`, which would hide the overflow.
    pub fn try_from_molgraph(mol: &MolGraph) -> Option<MatchMol> {
        let n = mol.atom_count();
        if n >= u16::MAX as usize {
            return None;
        }
        // A short cold array would read 0 through `.get().unwrap_or(0)`: a silently
        // wrong `v<n>`/`R<n>`. Refuse the graph instead.
        if mol.valence.len() != n || mol.ring_count.len() != n {
            return None;
        }
        if (0..n).any(|i| mol.neighbors(i as u32).len() > u8::MAX as usize) {
            return None;
        }

        // Size the blob: the offset table, then each atom's header + its neighbours.
        let table = (n + 1) * 4;
        let mut body = 0usize;
        for i in 0..n {
            body += ATOM_HDR + mol.neighbors(i as u32).len() * NBR_BYTES;
        }
        let mut blob = vec![0u8; table + body];

        // Lay out the records, recording each atom's start offset in the table.
        let mut cursor = table;
        for i in 0..n {
            let off = cursor;
            blob[i * 4..i * 4 + 4].copy_from_slice(&(off as u32).to_le_bytes());

            let nbrs = mol.neighbors(i as u32);
            blob[off] = mol.element[i];
            blob[off + 1] = mol.flags[i];
            blob[off + 2] = mol.degree[i];
            blob[off + 3] = mol.h_count[i];
            blob[off + 4] = mol.charge[i] as u8;
            blob[off + 5] = nbrs.len() as u8;

            let mut p = off + ATOM_HDR;
            for nb in nbrs {
                let bond = &mol.bonds[nb.bond as usize];
                blob[p..p + 2].copy_from_slice(&(nb.atom as u16).to_le_bytes());
                blob[p + 2] = bond.order;
                blob[p + 3] = bond.flags;
                p += NBR_BYTES;
            }
            cursor = p;
        }
        // Final sentinel offset (end of the last record).
        blob[n * 4..n * 4 + 4].copy_from_slice(&(cursor as u32).to_le_bytes());

        // Cold ring-size CSR: flatten the source per-atom ring-size lists.
        let mut ring_off = Vec::with_capacity(n + 1);
        let mut ring_sizes = Vec::new();
        ring_off.push(0u32);
        for i in 0..n {
            ring_sizes.extend_from_slice(mol.atom_ring_sizes(i as u32));
            ring_off.push(ring_sizes.len() as u32);
        }

        let arom_ring_hashes: Box<[u64]> = mol
            .arom_rings
            .iter()
            .map(|t| crate::graph::token_hash(t))
            .collect();

        Some(MatchMol {
            blob: blob.into_boxed_slice(),
            n_atoms: n as u32,
            ring_off: ring_off.into_boxed_slice(),
            ring_sizes: ring_sizes.into_boxed_slice(),
            arom_rings: mol.arom_rings.clone().into_boxed_slice(),
            arom_ring_hashes,
            valence: mol.valence.clone().into_boxed_slice(),
            ring_count: mol.ring_count.clone().into_boxed_slice(),
        })
    }

    /// Infallible form of [`try_from_molgraph`](Self::try_from_molgraph).
    ///
    /// # Panics
    /// If the molecule exceeds the compact-form limits. Bulk builders use
    /// `try_from_molgraph` so one oversized row is skipped, not fatal.
    pub fn from_molgraph(mol: &MolGraph) -> MatchMol {
        Self::try_from_molgraph(mol)
            .expect("molecule exceeds the compact-form limits (>=65535 atoms or degree >255)")
    }

    /// Number of atoms.
    #[inline]
    pub fn atom_count(&self) -> usize {
        self.n_atoms as usize
    }

    /// Byte offset of atom `t`'s record within the blob.
    ///
    /// `t` must be `< self.n_atoms`. An out-of-range `t` reads body bytes as an
    /// offset: usually a panic, but possibly an in-range garbage offset that gives
    /// silently wrong data. The bound is asserted in debug builds only, because this
    /// is the innermost accessor and callers pass atom ids the matcher produced.
    #[inline]
    fn record(&self, t: u32) -> usize {
        debug_assert!(
            t < self.n_atoms,
            "atom index {t} out of range (molecule has {} atoms)",
            self.n_atoms
        );
        rd_u32(&self.blob, t as usize * 4) as usize
    }

    /// Atomic number of atom `t`.
    ///
    /// # Panics
    /// Every per-atom accessor here (`element`, `is_aromatic`, `in_ring`, `degree`,
    /// `h_count`, `charge`, `neighbors`, `explicit_h`, `ring_bond_count`) requires
    /// `t < self.atom_count()`. Out of range it panics or returns data for no atom.
    /// Only `valence`, `ring_count` and `in_ring_of_size` bounds-check and return a
    /// default.
    #[inline]
    #[must_use]
    pub fn element(&self, t: u32) -> u8 {
        self.blob[self.record(t)]
    }
    /// True iff atom `t` is aromatic.
    #[inline]
    pub fn is_aromatic(&self, t: u32) -> bool {
        self.blob[self.record(t) + 1] & atom_flags::AROMATIC != 0
    }
    /// True iff atom `t` is in a ring.
    #[inline]
    pub fn in_ring(&self, t: u32) -> bool {
        self.blob[self.record(t) + 1] & atom_flags::IN_RING != 0
    }
    /// Explicit-neighbour degree of atom `t`.
    #[inline]
    pub fn degree(&self, t: u32) -> u8 {
        self.blob[self.record(t) + 2]
    }
    /// Implicit + bracket-explicit H count of atom `t` (excludes bonded `[H]` nodes).
    #[inline]
    pub fn h_count(&self, t: u32) -> u8 {
        self.blob[self.record(t) + 3]
    }
    /// Count of atom `t`'s neighbours that are hydrogen nodes. Total H for the
    /// SMARTS `H<n>` primitive is `h_count + explicit_h`.
    #[inline]
    pub fn explicit_h(&self, t: u32) -> u8 {
        self.neighbors(t)
            .filter(|nb| self.element(nb.atom) == 1)
            .count() as u8
    }
    /// Number of atom `t`'s ring bonds (the SMARTS `x<n>` primitive).
    #[inline]
    pub fn ring_bond_count(&self, t: u32) -> u8 {
        self.neighbors(t)
            .filter(|nb| nb.flags & crate::graph::bond_flags::IN_RING != 0)
            .count() as u8
    }
    /// Total bond-order valence of atom `t` (the SMARTS `v<n>` primitive); cold array.
    #[inline]
    pub fn valence(&self, t: u32) -> u8 {
        self.valence.get(t as usize).copied().unwrap_or(0)
    }
    /// SSSR ring-membership count of atom `t` (the SMARTS `R<n>` primitive); cold array.
    #[inline]
    pub fn ring_count(&self, t: u32) -> u8 {
        self.ring_count.get(t as usize).copied().unwrap_or(0)
    }
    /// Formal charge of atom `t`.
    #[inline]
    pub fn charge(&self, t: u32) -> i8 {
        self.blob[self.record(t) + 4] as i8
    }

    /// True iff atom `t` belongs to an SSSR ring of size exactly `size`.
    #[inline]
    pub fn in_ring_of_size(&self, t: u32, size: u8) -> bool {
        let i = t as usize;
        if i + 1 >= self.ring_off.len() {
            return false;
        }
        let s = self.ring_off[i] as usize;
        let e = self.ring_off[i + 1] as usize;
        self.ring_sizes[s..e].contains(&size)
    }

    /// Iterates atom `t`'s neighbours (with inlined bond order/flags).
    ///
    /// # Panics
    /// Requires `t < self.atom_count()`; see [`element`](Self::element).
    #[inline]
    pub fn neighbors(&self, t: u32) -> NbrIter<'_> {
        let off = self.record(t);
        let n = self.blob[off + 5] as usize;
        NbrIter {
            blob: &self.blob,
            pos: off + ATOM_HDR,
            remaining: n,
        }
    }

    /// Approximate heap footprint in bytes.
    pub fn mem_bytes(&self) -> usize {
        // `size_of::<Self>()` covers only the box headers; count the token boxes,
        // their bytes and the hashes explicitly.
        let tokens: usize = self.arom_rings.iter().map(|r| r.len()).sum::<usize>()
            + self.arom_rings.len() * core::mem::size_of::<Box<[u8]>>();
        self.blob.len()
            + self.ring_off.len() * 4
            + self.ring_sizes.len()
            + tokens
            + self.arom_ring_hashes.len() * 8
            + self.valence.len()
            + self.ring_count.len()
            + core::mem::size_of::<Self>()
    }
}

impl AtomView for MatchMol {
    #[inline]
    fn av_element(&self, i: u32) -> u8 {
        self.element(i)
    }
    #[inline]
    fn av_aromatic(&self, i: u32) -> bool {
        self.is_aromatic(i)
    }
    #[inline]
    fn av_in_ring(&self, i: u32) -> bool {
        self.in_ring(i)
    }
    #[inline]
    fn av_ring_size(&self, i: u32, size: u8) -> bool {
        self.in_ring_of_size(i, size)
    }
    #[inline]
    fn av_degree(&self, i: u32) -> u8 {
        self.degree(i)
    }
    #[inline]
    fn av_h_count(&self, i: u32) -> u8 {
        self.h_count(i)
    }
    #[inline]
    fn av_explicit_h(&self, i: u32) -> u8 {
        self.explicit_h(i)
    }
    #[inline]
    fn av_ring_bond_count(&self, i: u32) -> u8 {
        self.ring_bond_count(i)
    }
    #[inline]
    fn av_valence(&self, i: u32) -> u8 {
        self.valence(i)
    }
    #[inline]
    fn av_ring_count(&self, i: u32) -> u8 {
        self.ring_count(i)
    }
    #[inline]
    fn av_charge(&self, i: u32) -> i8 {
        self.charge(i)
    }
}

impl crate::graph::GraphView for MatchMol {
    #[inline]
    fn gv_atom_count(&self) -> u32 {
        self.n_atoms
    }
    #[inline]
    fn gv_degree(&self, i: u32) -> u32 {
        // Use `n_nbr`, not the cached `degree` byte, so this agrees with
        // `neighbors()` and the entries actually in the blob.
        self.blob[self.record(i) + 5] as u32
    }
    #[inline]
    fn gv_neighbor(&self, i: u32, k: u32) -> (u32, u8, u8) {
        let off = self.record(i) + ATOM_HDR + k as usize * NBR_BYTES;
        (
            rd_u16(&self.blob, off) as u32,
            self.blob[off + 2],
            self.blob[off + 3],
        )
    }
}

/// Iterator over an atom's inlined neighbour entries.
pub struct NbrIter<'a> {
    blob: &'a [u8],
    pos: usize,
    remaining: usize,
}

impl Iterator for NbrIter<'_> {
    type Item = Nbr;
    #[inline]
    fn next(&mut self) -> Option<Nbr> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let p = self.pos;
        self.pos += NBR_BYTES;
        Some(Nbr {
            atom: rd_u16(self.blob, p) as u32,
            order: self.blob[p + 2],
            flags: self.blob[p + 3],
        })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for NbrIter<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::AtomView;
    use crate::smiles::parse as parse_smiles;

    /// The compact form reports exactly what the source `MolGraph` does: every atom
    /// field, every neighbour with bond order and flags, and `in_ring_of_size`.
    #[test]
    fn matchmol_matches_molgraph() {
        let corpus = [
            "C",
            "CC",
            "CCO",
            "CC(=O)O",
            "c1ccccc1",
            "Cc1ccccc1",
            "c1ccncc1",
            "FC(F)(F)F",
            "ClCCBr",
            "CC#N",
            "Nc1ccccc1",
            "C1CCCCC1",
            "CS(=O)(=O)N",
            "c1ccc2ccccc2c1",
            "CC(=O)Oc1ccccc1C(=O)O",
            "CN1C=NC2=C1C(=O)N(C(=O)N2C)C",
            "[NH4+]",
            "CC(=O)[O-]",
            "[13CH4]",
            "c1cc[nH]c1",
            "OP(=O)(O)O",
            "C[N+](C)(C)C",
        ];
        for s in corpus {
            let mol = parse_smiles(s).unwrap_or_else(|e| panic!("parse {s:?}: {e}"));
            let cm = MatchMol::from_molgraph(&mol);
            assert_eq!(cm.atom_count(), mol.atom_count(), "atom_count {s:?}");
            for t in 0..mol.atom_count() as u32 {
                assert_eq!(
                    cm.element(t),
                    mol.element[t as usize],
                    "element {s:?} atom {t}"
                );
                assert_eq!(
                    cm.is_aromatic(t),
                    mol.is_aromatic(t),
                    "aromatic {s:?} atom {t}"
                );
                assert_eq!(cm.in_ring(t), mol.in_ring(t), "in_ring {s:?} atom {t}");
                assert_eq!(
                    cm.degree(t),
                    mol.degree[t as usize],
                    "degree {s:?} atom {t}"
                );
                assert_eq!(
                    cm.h_count(t),
                    mol.h_count[t as usize],
                    "h_count {s:?} atom {t}"
                );
                assert_eq!(
                    cm.charge(t),
                    mol.charge[t as usize],
                    "charge {s:?} atom {t}"
                );
                // Derived accessors used by the SMARTS X/x primitives must agree.
                assert_eq!(
                    cm.explicit_h(t),
                    mol.av_explicit_h(t),
                    "explicit_h {s:?} atom {t}"
                );
                assert_eq!(
                    cm.ring_bond_count(t),
                    mol.av_ring_bond_count(t),
                    "ring_bond_count {s:?} atom {t}"
                );
                assert_eq!(
                    cm.valence(t),
                    mol.valence[t as usize],
                    "valence {s:?} atom {t}"
                );
                assert_eq!(
                    cm.ring_count(t),
                    mol.ring_count[t as usize],
                    "ring_count {s:?} atom {t}"
                );
                for sz in 0..=8u8 {
                    assert_eq!(
                        cm.in_ring_of_size(t, sz),
                        mol.in_ring_of_size(t, sz),
                        "in_ring_of_size({t},{sz}) {s:?}"
                    );
                }
                // Neighbours must match as a set of (atom, order, flags).
                let mut got: Vec<(u32, u8, u8)> = cm
                    .neighbors(t)
                    .map(|nb| (nb.atom, nb.order, nb.flags))
                    .collect();
                let mut want: Vec<(u32, u8, u8)> = mol
                    .neighbors(t)
                    .iter()
                    .map(|nb| {
                        let b = &mol.bonds[nb.bond as usize];
                        (nb.atom, b.order, b.flags)
                    })
                    .collect();
                got.sort_unstable();
                want.sort_unstable();
                assert_eq!(got, want, "neighbours {s:?} atom {t}");

                // The recursive `$(...)` matcher uses the indexed GraphView
                // accessors, not NbrIter, so they must agree too.
                use crate::graph::GraphView;
                assert_eq!(
                    GraphView::gv_degree(&cm, t),
                    GraphView::gv_degree(&mol, t),
                    "gv_degree {s:?} atom {t}"
                );
                let mut gv_got: Vec<(u32, u8, u8)> = (0..GraphView::gv_degree(&cm, t))
                    .map(|k| GraphView::gv_neighbor(&cm, t, k))
                    .collect();
                let mut gv_want: Vec<(u32, u8, u8)> = (0..GraphView::gv_degree(&mol, t))
                    .map(|k| GraphView::gv_neighbor(&mol, t, k))
                    .collect();
                gv_got.sort_unstable();
                gv_want.sort_unstable();
                assert_eq!(gv_got, gv_want, "gv_neighbor set {s:?} atom {t}");
                // gv_bond agrees for every ordered atom pair (presence + order/flags).
                for u in 0..mol.atom_count() as u32 {
                    assert_eq!(
                        GraphView::gv_bond(&cm, t, u),
                        GraphView::gv_bond(&mol, t, u),
                        "gv_bond({t},{u}) {s:?}"
                    );
                }
            }
            {
                use crate::graph::GraphView;
                assert_eq!(
                    GraphView::gv_atom_count(&cm),
                    GraphView::gv_atom_count(&mol),
                    "gv_atom_count {s:?}"
                );
            }
            // Ring-skip tokens are copied verbatim and each cached hash equals the
            // token's hash (required for the prefilter to be sound).
            assert_eq!(
                cm.arom_rings,
                mol.arom_rings.clone().into_boxed_slice(),
                "arom_rings {s:?}"
            );
            assert_eq!(
                cm.arom_ring_hashes.len(),
                cm.arom_rings.len(),
                "hash len {s:?}"
            );
            for (h, tok) in cm.arom_ring_hashes.iter().zip(cm.arom_rings.iter()) {
                assert_eq!(*h, crate::graph::token_hash(tok), "ring token hash {s:?}");
            }
        }
    }
}
