//! The compact target-molecule graph: [`MolGraph`].
//!
//! Each atom property is a parallel `Vec` (struct of arrays). Adjacency is CSR:
//! atom `i`'s neighbours are `nbr[nbr_offsets[i] .. nbr_offsets[i + 1]]`, and each
//! entry holds the adjacent atom and the bond index.
//!
//! # Invariants (upheld by [`MolGraphBuilder`])
//!
//! * All per-atom `Vec`s have length `atom_count`.
//! * Every bond's `(a, b)` endpoints are valid atom indices with `a != b`.
//! * `nbr_offsets` has length `atom_count + 1`, is non-decreasing, starts at `0`
//!   and ends at `nbr.len() == 2 * bonds.len()` (each bond contributes one
//!   neighbour entry to *each* endpoint).
//! * Flag bits live in [`atom_flags`] / [`bond_flags`]; unused bits are zero.

/// Per-atom boolean properties, packed one bit each.
pub mod atom_flags {
    /// The atom is aromatic (set by perception, not parsing).
    pub const AROMATIC: u8 = 0b0000_0001;
    /// The atom belongs to at least one ring.
    pub const IN_RING: u8 = 0b0000_0010;
}

/// Per-bond boolean properties, packed one bit each.
pub mod bond_flags {
    /// The bond is a ring bond.
    pub const IN_RING: u8 = 0b0000_0001;
    /// The bond is aromatic: the canonical marker after perception (the order is
    /// also set to `bond_order::AROMATIC`).
    pub const AROMATIC: u8 = 0b0000_0010;
}

/// Bond-order codes shared by [`MolGraph`] and the query graph.
pub mod bond_order {
    /// Single bond (`-` or an implicit bond between two aliphatic atoms).
    pub const SINGLE: u8 = 1;
    /// Double bond (`=`).
    pub const DOUBLE: u8 = 2;
    /// Triple bond (`#`).
    pub const TRIPLE: u8 = 3;
    /// Aromatic bond (`:` or perceived aromatic).
    pub const AROMATIC: u8 = 4;
}

/// A single bond: an unordered pair of atom indices plus order and flags.
///
/// Endpoints are stored in the order given; `a < b` is not required.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bond {
    /// First endpoint atom index.
    pub a: u32,
    /// Second endpoint atom index.
    pub b: u32,
    /// Bond order, one of [`bond_order`].
    pub order: u8,
    /// Packed [`bond_flags`].
    pub flags: u8,
}

impl Bond {
    /// Given one endpoint of this bond, returns the other endpoint.
    ///
    /// # Panics
    /// Panics if `atom` is not an endpoint of this bond.
    #[inline]
    pub fn other(&self, atom: u32) -> u32 {
        if atom == self.a {
            self.b
        } else if atom == self.b {
            self.a
        } else {
            panic!(
                "Bond::other called with atom {atom} that is not an endpoint of bond ({}, {})",
                self.a, self.b
            )
        }
    }

    /// True if this bond's [`bond_flags::AROMATIC`] bit is set.
    #[inline]
    pub fn is_aromatic(&self) -> bool {
        self.flags & bond_flags::AROMATIC != 0
    }

    /// True if this bond's [`bond_flags::IN_RING`] bit is set.
    #[inline]
    pub fn in_ring(&self) -> bool {
        self.flags & bond_flags::IN_RING != 0
    }
}

/// One entry in the CSR neighbour list: an adjacent atom and the bond reaching it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Neighbor {
    /// The adjacent atom's index.
    pub atom: u32,
    /// The index (into [`MolGraph::bonds`]) of the connecting bond.
    pub bond: u32,
}

/// Canonical token for an aromatic ring, used by the matcher's ring-skip fast path.
///
/// Encodes the `(element, aromatic)` labels around the ring as the least of its `2k`
/// rotations in both directions, so it is invariant under rotation and reflection.
/// Bond orders are not encoded: the skip consults only rings whose bonds are all
/// aromatic-order, which always satisfy a pure-aromatic ring query's default bonds.
/// Query and molecule sides both use this function, so identical rings give
/// identical tokens; the skip's soundness depends on that.
///
/// Each direction's least rotation is found in O(k); a test checks the result
/// against the exhaustive form.
pub fn canonical_ring_token(labels: &[(u8, bool)]) -> Box<[u8]> {
    let encode = |seq: &[(u8, bool)], start: usize| -> Vec<u8> {
        let k = seq.len();
        let mut buf = Vec::with_capacity(k * 2);
        for i in 0..k {
            let (e, a) = seq[(start + i) % k];
            buf.push(e);
            buf.push(a as u8);
        }
        buf
    };
    // Reading backwards from position `s` is reading `reversed` forwards from
    // `k - 1 - s`, so the backward rotations are the rotations of `reversed`.
    let reversed: Vec<(u8, bool)> = labels.iter().rev().copied().collect();
    let fwd = encode(labels, least_rotation(labels));
    let bwd = encode(&reversed, least_rotation(&reversed));
    fwd.min(bwd).into_boxed_slice()
}

/// Start index of the lexicographically least rotation of `s` (the first such
/// start when several are equal), in O(len) comparisons.
fn least_rotation<T: Ord>(s: &[T]) -> usize {
    let n = s.len();
    let (mut i, mut j, mut k) = (0usize, 1usize, 0usize);
    while i < n && j < n && k < n {
        match s[(i + k) % n].cmp(&s[(j + k) % n]) {
            std::cmp::Ordering::Equal => k += 1,
            std::cmp::Ordering::Greater => {
                i += k + 1;
                if i <= j {
                    i = j + 1;
                }
                k = 0;
            }
            std::cmp::Ordering::Less => {
                j += k + 1;
                if j <= i {
                    j = i + 1;
                }
                k = 0;
            }
        }
    }
    i.min(j)
}

/// 64-bit FNV-1a hash of a canonical ring token.
///
/// The ring-skip path compares hashes before token bytes. Identical tokens hash
/// identically and a hash hit is confirmed by a byte compare, so the result stays
/// exact.
#[inline]
pub(crate) fn token_hash(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325; // FNV-1a offset basis
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3); // FNV prime
    }
    h
}

/// The per-atom data a [`crate::query::Pred`] reads, implemented by [`MolGraph`]
/// and [`crate::compact::MatchMol`]. The `av_` prefix avoids clashes with inherent
/// accessors.
pub trait AtomView {
    /// Atomic number of atom `i`.
    fn av_element(&self, i: u32) -> u8;
    /// True iff atom `i` is aromatic.
    fn av_aromatic(&self, i: u32) -> bool;
    /// True iff atom `i` is in a ring.
    fn av_in_ring(&self, i: u32) -> bool;
    /// True iff atom `i` belongs to an SSSR ring of size exactly `size`.
    fn av_ring_size(&self, i: u32, size: u8) -> bool;
    /// Explicit-neighbour degree of atom `i`.
    fn av_degree(&self, i: u32) -> u8;
    /// Implicit plus bracket H count of atom `i`, excluding bonded `[H]` nodes
    /// (see [`Self::av_explicit_h`]).
    fn av_h_count(&self, i: u32) -> u8;
    /// Count of atom `i`'s neighbours that are hydrogen nodes. SMARTS `H<n>` is
    /// `av_h_count + av_explicit_h`, so it does not depend on how H is written.
    fn av_explicit_h(&self, i: u32) -> u8;
    /// Number of atom `i`'s ring bonds (the SMARTS `x<n>` primitive).
    fn av_ring_bond_count(&self, i: u32) -> u8;
    /// Total bond-order valence of atom `i` (the SMARTS `v<n>` primitive).
    fn av_valence(&self, i: u32) -> u8;
    /// Number of SSSR rings atom `i` is a member of (the SMARTS `R<n>` primitive).
    fn av_ring_count(&self, i: u32) -> u8;
    /// Formal charge of atom `i`.
    fn av_charge(&self, i: u32) -> i8;
}

/// [`AtomView`] plus neighbour iteration and bond lookup.
///
/// Recursive SMARTS `$(...)` must walk the target graph, so
/// [`crate::query::Pred::eval`] is generic over this trait. [`MolGraph`] (brute-force
/// oracle) and [`crate::compact::MatchMol`] (production matcher) both implement it,
/// so a recursive query gives the same answer on both. The test
/// `crate::compact::tests::matchmol_matches_molgraph` checks the two impls agree.
pub trait GraphView: AtomView {
    /// Number of atoms in the graph.
    fn gv_atom_count(&self) -> u32;
    /// Number of explicit neighbours (incident bonds) of atom `i`.
    fn gv_degree(&self, i: u32) -> u32;
    /// The `k`-th neighbour of atom `i` as `(neighbour_atom, bond_order, bond_flags)`.
    /// `k` must be `< gv_degree(i)`.
    fn gv_neighbor(&self, i: u32, k: u32) -> (u32, u8, u8);
    /// The connecting bond's `(order, flags)` between atoms `a` and `b`, or `None`
    /// if they are not bonded. The default scans `a`'s neighbours.
    #[inline]
    fn gv_bond(&self, a: u32, b: u32) -> Option<(u8, u8)> {
        for k in 0..self.gv_degree(a) {
            let (t, order, flags) = self.gv_neighbor(a, k);
            if t == b {
                return Some((order, flags));
            }
        }
        None
    }
}

impl GraphView for MolGraph {
    #[inline]
    fn gv_atom_count(&self) -> u32 {
        self.atom_count() as u32
    }
    #[inline]
    fn gv_degree(&self, i: u32) -> u32 {
        self.neighbors(i).len() as u32
    }
    #[inline]
    fn gv_neighbor(&self, i: u32, k: u32) -> (u32, u8, u8) {
        let nb = self.neighbors(i)[k as usize];
        let bond = &self.bonds[nb.bond as usize];
        (nb.atom, bond.order, bond.flags)
    }
}

impl AtomView for MolGraph {
    #[inline]
    fn av_element(&self, i: u32) -> u8 {
        self.element[i as usize]
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
        self.degree[i as usize]
    }
    #[inline]
    fn av_h_count(&self, i: u32) -> u8 {
        self.h_count[i as usize]
    }
    #[inline]
    fn av_explicit_h(&self, i: u32) -> u8 {
        self.neighbors(i)
            .iter()
            .filter(|nb| self.element[nb.atom as usize] == 1)
            .count()
            .min(u8::MAX as usize) as u8
    }
    #[inline]
    fn av_ring_bond_count(&self, i: u32) -> u8 {
        self.neighbors(i)
            .iter()
            .filter(|nb| self.bonds[nb.bond as usize].in_ring())
            .count()
            .min(u8::MAX as usize) as u8
    }
    #[inline]
    fn av_valence(&self, i: u32) -> u8 {
        self.valence.get(i as usize).copied().unwrap_or(0)
    }
    #[inline]
    fn av_ring_count(&self, i: u32) -> u8 {
        self.ring_count.get(i as usize).copied().unwrap_or(0)
    }
    #[inline]
    fn av_charge(&self, i: u32) -> i8 {
        self.charge[i as usize]
    }
}

/// A perceived target molecule in struct-of-arrays form.
///
/// Build with [`MolGraphBuilder`]. Fields are public but read-only once built; the
/// builder upholds the [module invariants](self).
#[derive(Clone, Debug, Default)]
pub struct MolGraph {
    // --- Per-atom arrays (all length == atom_count) ---
    /// Atomic number per atom. `0` is a wildcard sentinel not normally present.
    pub element: Vec<u8>,
    /// Packed [`atom_flags`] per atom.
    pub flags: Vec<u8>,
    /// Formal charge per atom.
    pub charge: Vec<i8>,
    /// Implicit plus bracket H count per atom, excluding bonded `[H]` nodes.
    pub h_count: Vec<u8>,
    /// Number of bonds per atom (H counts only as an explicit atom), saturated
    /// at 255. Cached for the `D<n>` test.
    pub degree: Vec<u8>,

    // --- Bonds ---
    /// All bonds, indexed by the `bond` field of [`Neighbor`].
    pub bonds: Vec<Bond>,

    // --- CSR adjacency (see module docs) ---
    /// `nbr_offsets[i] .. nbr_offsets[i + 1]` is atom `i`'s slice of [`Self::nbr`].
    /// Length is `atom_count + 1`.
    pub nbr_offsets: Vec<u32>,
    /// Flattened neighbour entries for all atoms, grouped by atom via
    /// [`Self::nbr_offsets`].
    pub nbr: Vec<Neighbor>,

    // --- Perception products (empty until perception runs) ---
    /// Smallest SSSR ring size per atom, or `0` if acyclic.
    pub atom_ring_size: Vec<u8>,
    /// Distinct SSSR ring sizes per atom (sorted, deduplicated), in CSR form over
    /// [`Self::ring_size_offsets`]. Lets `r<n>` match an atom in several rings of
    /// different sizes. An empty slice means acyclic.
    pub atom_ring_sizes: Vec<u8>,
    /// CSR offsets: atom `i`'s ring sizes are
    /// `atom_ring_sizes[ring_size_offsets[i] .. ring_size_offsets[i + 1]]`.
    /// Length `atom_count + 1`.
    pub ring_size_offsets: Vec<u32>,
    /// Smallest SSSR ring size per bond, or `0` if not in a ring.
    pub bond_ring_size: Vec<u8>,

    /// [`canonical_ring_token`]s of the SSSR rings whose bonds are all
    /// aromatic-order. The ring-skip path tests a pure-aromatic ring query's token
    /// here: a hit is a proven match (VF2 skipped); a miss falls through to VF2, so
    /// an SSSR under-count cannot drop a hit.
    pub arom_rings: Vec<Box<[u8]>>,

    /// Per atom: in a 6-membered SSSR ring of all carbons and all aromatic-order
    /// bonds (a benzene ring). Drives the census `PHENOL_C` class. Like `arom_rings`
    /// it is SSSR-derived, so it is only an early-accept signal, never a reject.
    pub carbocyclic_arom6: Vec<bool>,

    /// Per-atom valence (`v<n>`): sum of incident Kekulé bond orders plus attached
    /// H. Aromatic bonds count their Kekulé order (benzene CH is v4, pyridine N v3).
    pub valence: Vec<u8>,
    /// Per-atom number of SSSR rings the atom belongs to (`R<n>`).
    pub ring_count: Vec<u8>,
}

impl MolGraph {
    /// Number of atoms in the molecule.
    #[inline]
    pub fn atom_count(&self) -> usize {
        self.element.len()
    }

    /// Number of bonds in the molecule.
    #[inline]
    pub fn bond_count(&self) -> usize {
        self.bonds.len()
    }

    /// The CSR neighbour slice for atom `i`.
    #[inline]
    pub fn neighbors(&self, i: u32) -> &[Neighbor] {
        let start = self.nbr_offsets[i as usize] as usize;
        let end = self.nbr_offsets[i as usize + 1] as usize;
        &self.nbr[start..end]
    }

    /// True if atom `i` is flagged aromatic.
    #[inline]
    pub fn is_aromatic(&self, i: u32) -> bool {
        self.flags[i as usize] & atom_flags::AROMATIC != 0
    }

    /// True if atom `i` is flagged as belonging to a ring.
    #[inline]
    pub fn in_ring(&self, i: u32) -> bool {
        self.flags[i as usize] & atom_flags::IN_RING != 0
    }

    /// Distinct SSSR ring sizes of atom `i` (sorted). Empty for an acyclic atom or
    /// an unperceived graph.
    #[inline]
    pub fn atom_ring_sizes(&self, i: u32) -> &[u8] {
        let idx = i as usize;
        if idx + 1 >= self.ring_size_offsets.len() {
            return &[];
        }
        let start = self.ring_size_offsets[idx] as usize;
        let end = self.ring_size_offsets[idx + 1] as usize;
        &self.atom_ring_sizes[start..end]
    }

    /// True if atom `i` belongs to an SSSR ring of size exactly `size`.
    ///
    /// Backs the SMARTS `r<n>` primitive. `false` on an unperceived graph.
    #[inline]
    pub fn in_ring_of_size(&self, i: u32, size: u8) -> bool {
        self.atom_ring_sizes(i).contains(&size)
    }

    /// Approximate heap footprint in bytes, from `Vec` capacities.
    pub fn mem_bytes(&self) -> usize {
        use core::mem::size_of;
        let mut bytes = size_of::<Self>();
        bytes += self.element.capacity() * size_of::<u8>();
        bytes += self.flags.capacity() * size_of::<u8>();
        bytes += self.charge.capacity() * size_of::<i8>();
        bytes += self.h_count.capacity() * size_of::<u8>();
        bytes += self.degree.capacity() * size_of::<u8>();
        bytes += self.bonds.capacity() * size_of::<Bond>();
        bytes += self.nbr_offsets.capacity() * size_of::<u32>();
        bytes += self.nbr.capacity() * size_of::<Neighbor>();
        bytes += self.atom_ring_size.capacity() * size_of::<u8>();
        bytes += self.bond_ring_size.capacity() * size_of::<u8>();
        bytes += self.atom_ring_sizes.capacity() * size_of::<u8>();
        bytes += self.ring_size_offsets.capacity() * size_of::<u32>();
        bytes
    }
}

/// Incremental builder for a [`MolGraph`] that upholds the module invariants.
///
/// Add every atom, then every bond, then call [`finish`](Self::finish), which
/// derives CSR adjacency and `degree`. Flags stay as the caller set them;
/// perception fills in aromatic and ring data.
#[derive(Clone, Debug, Default)]
pub struct MolGraphBuilder {
    element: Vec<u8>,
    flags: Vec<u8>,
    charge: Vec<i8>,
    h_count: Vec<u8>,
    bonds: Vec<Bond>,
}

impl MolGraphBuilder {
    /// A fresh, empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an atom and returns its index.
    ///
    /// `flags` is the initial [`atom_flags`] bitset (usually `0`; perception sets
    /// aromatic and ring bits).
    pub fn add_atom(&mut self, element: u8, charge: i8, h_count: u8, flags: u8) -> u32 {
        let idx = self.element.len() as u32;
        self.element.push(element);
        self.flags.push(flags);
        self.charge.push(charge);
        self.h_count.push(h_count);
        idx
    }

    /// Appends a bond between two existing atoms and returns its index.
    ///
    /// # Panics
    /// Panics if `a == b` or either endpoint is out of range.
    pub fn add_bond(&mut self, a: u32, b: u32, order: u8, flags: u8) -> u32 {
        let n = self.element.len() as u32;
        assert!(
            a < n && b < n,
            "bond endpoint out of range: ({a}, {b}) with {n} atoms"
        );
        assert!(a != b, "self-bond on atom {a} is not allowed");
        let idx = self.bonds.len() as u32;
        self.bonds.push(Bond { a, b, order, flags });
        idx
    }

    /// Number of atoms added so far.
    #[inline]
    pub fn atom_count(&self) -> usize {
        self.element.len()
    }

    /// Replaces the packed flag byte of an added atom.
    #[inline]
    pub fn set_atom_flags(&mut self, atom: u32, flags: u8) {
        self.flags[atom as usize] = flags;
    }

    /// Replaces the H count of an added atom. The SMILES parser sets implicit H
    /// once all bonds, including later ring closures, are known.
    #[inline]
    pub fn set_h_count(&mut self, atom: u32, h_count: u8) {
        self.h_count[atom as usize] = h_count;
    }

    /// Read-only view of the bonds added so far.
    #[inline]
    pub fn bonds_slice(&self) -> &[Bond] {
        &self.bonds
    }

    /// Finalises the builder into a [`MolGraph`], deriving CSR adjacency and the
    /// cached per-atom degree.
    pub fn finish(self) -> MolGraph {
        let atom_count = self.element.len();

        // Count neighbours per atom.
        let mut degree_u32 = vec![0u32; atom_count];
        for bond in &self.bonds {
            degree_u32[bond.a as usize] += 1;
            degree_u32[bond.b as usize] += 1;
        }

        // Prefix-sum into CSR offsets.
        let mut nbr_offsets = vec![0u32; atom_count + 1];
        for i in 0..atom_count {
            nbr_offsets[i + 1] = nbr_offsets[i] + degree_u32[i];
        }

        // Scatter each bond into both endpoints' slices; `cursor[i]` is the next
        // free slot of atom i.
        let total = nbr_offsets[atom_count] as usize;
        let placeholder = Neighbor { atom: 0, bond: 0 };
        let mut nbr = vec![placeholder; total];
        let mut cursor: Vec<u32> = nbr_offsets[..atom_count].to_vec();
        for (bond_idx, bond) in self.bonds.iter().enumerate() {
            let bond_idx = bond_idx as u32;
            let sa = cursor[bond.a as usize];
            nbr[sa as usize] = Neighbor {
                atom: bond.b,
                bond: bond_idx,
            };
            cursor[bond.a as usize] += 1;

            let sb = cursor[bond.b as usize];
            nbr[sb as usize] = Neighbor {
                atom: bond.a,
                bond: bond_idx,
            };
            cursor[bond.b as usize] += 1;
        }

        // Saturate at 255 rather than truncate.
        let degree: Vec<u8> = degree_u32
            .iter()
            .map(|&d| u8::try_from(d).unwrap_or(u8::MAX))
            .collect();

        MolGraph {
            element: self.element,
            flags: self.flags,
            charge: self.charge,
            h_count: self.h_count,
            degree,
            bonds: self.bonds,
            nbr_offsets,
            nbr,
            // Empty, not zero-filled, so an unperceived graph is distinguishable
            // from a perceived acyclic one.
            atom_ring_size: Vec::new(),
            atom_ring_sizes: Vec::new(),
            ring_size_offsets: Vec::new(),
            bond_ring_size: Vec::new(),
            arom_rings: Vec::new(),
            carbocyclic_arom6: Vec::new(),
            valence: Vec::new(),
            ring_count: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    /// The O(k) token equals the exhaustive "least of all 2k readings" token.
    #[test]
    fn canonical_ring_token_equals_exhaustive_form() {
        fn exhaustive(labels: &[(u8, bool)]) -> Box<[u8]> {
            let k = labels.len();
            let mut best: Vec<u8> = Vec::new();
            let mut buf: Vec<u8> = Vec::with_capacity(k * 2);
            for dir in [1i64, -1] {
                for s in 0..k {
                    buf.clear();
                    for i in 0..k {
                        let idx = (s as i64 + dir * i as i64).rem_euclid(k as i64) as usize;
                        let (e, a) = labels[idx];
                        buf.push(e);
                        buf.push(a as u8);
                    }
                    if best.is_empty() || buf < best {
                        best.clone_from(&buf);
                    }
                }
            }
            best.into_boxed_slice()
        }
        let mut rng = 0x1234_5678_9abc_def1u64;
        let mut next = || {
            rng = rng
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            rng >> 33
        };
        let mut checked = 0;
        for k in 1..=12usize {
            for _ in 0..400 {
                // A small alphabet makes equal and periodic rotations common.
                let labels: Vec<(u8, bool)> = (0..k)
                    .map(|_| {
                        let r = next();
                        (6 + (r % 3) as u8, r & 4 != 0)
                    })
                    .collect();
                assert_eq!(
                    canonical_ring_token(&labels),
                    exhaustive(&labels),
                    "{labels:?}"
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 4800);
        let periodic = [(6, true), (7, true)].repeat(5);
        assert_eq!(canonical_ring_token(&periodic), exhaustive(&periodic));
    }

    use super::*;

    /// Methanol C-O: array lengths, CSR adjacency, degree and `Bond::other`.
    #[test]
    fn builds_csr_adjacency_for_a_two_atom_molecule() {
        let mut b = MolGraphBuilder::new();
        let c = b.add_atom(6, 0, 3, 0); // carbon
        let o = b.add_atom(8, 0, 1, 0); // oxygen
        let bond = b.add_bond(c, o, bond_order::SINGLE, 0);
        let g = b.finish();

        assert_eq!(g.atom_count(), 2);
        assert_eq!(g.bond_count(), 1);
        // All SoA arrays are parallel.
        assert_eq!(g.element.len(), 2);
        assert_eq!(g.flags.len(), 2);
        assert_eq!(g.charge.len(), 2);
        assert_eq!(g.h_count.len(), 2);
        assert_eq!(g.degree.len(), 2);

        // CSR: 3 offsets, 2 neighbour entries (one per endpoint).
        assert_eq!(g.nbr_offsets, vec![0, 1, 2]);
        assert_eq!(g.nbr.len(), 2);

        // Carbon's single neighbour is oxygen via the one bond.
        let c_nbrs = g.neighbors(c);
        assert_eq!(c_nbrs.len(), 1);
        assert_eq!(c_nbrs[0].atom, o);
        assert_eq!(c_nbrs[0].bond, bond);

        // Oxygen's single neighbour is carbon.
        let o_nbrs = g.neighbors(o);
        assert_eq!(o_nbrs.len(), 1);
        assert_eq!(o_nbrs[0].atom, c);

        assert_eq!(g.degree, vec![1, 1]);
        assert_eq!(g.bonds[0].other(c), o);
        assert_eq!(g.bonds[0].other(o), c);
    }

    /// A central carbon bonded to three others: several neighbours per atom.
    #[test]
    fn csr_groups_multiple_neighbors_per_atom() {
        let mut b = MolGraphBuilder::new();
        let c0 = b.add_atom(6, 0, 1, 0);
        let c1 = b.add_atom(6, 0, 3, 0);
        let c2 = b.add_atom(6, 0, 3, 0);
        let c3 = b.add_atom(6, 0, 3, 0);
        b.add_bond(c0, c1, bond_order::SINGLE, 0);
        b.add_bond(c0, c2, bond_order::SINGLE, 0);
        b.add_bond(c0, c3, bond_order::SINGLE, 0);
        let g = b.finish();

        assert_eq!(g.degree, vec![3, 1, 1, 1]);
        // c0's neighbour slice holds all three terminal carbons.
        let mut c0_atoms: Vec<u32> = g.neighbors(c0).iter().map(|n| n.atom).collect();
        c0_atoms.sort_unstable();
        assert_eq!(c0_atoms, vec![c1, c2, c3]);

        // Offsets are a valid non-decreasing prefix sum ending at 2 * bonds.
        assert_eq!(g.nbr_offsets.first(), Some(&0));
        assert_eq!(*g.nbr_offsets.last().unwrap() as usize, 2 * g.bond_count());
        for w in g.nbr_offsets.windows(2) {
            assert!(w[0] <= w[1]);
        }
    }

    #[test]
    #[should_panic(expected = "self-bond")]
    fn rejects_self_bonds() {
        let mut b = MolGraphBuilder::new();
        let a = b.add_atom(6, 0, 0, 0);
        b.add_bond(a, a, bond_order::SINGLE, 0);
    }

    #[test]
    fn flag_helpers_read_packed_bits() {
        let mut b = MolGraphBuilder::new();
        let a = b.add_atom(6, 0, 1, atom_flags::AROMATIC | atom_flags::IN_RING);
        let _ = b.add_atom(8, 0, 1, 0);
        b.add_bond(
            0,
            1,
            bond_order::AROMATIC,
            bond_flags::AROMATIC | bond_flags::IN_RING,
        );
        let g = b.finish();
        assert!(g.is_aromatic(a));
        assert!(g.in_ring(a));
        assert!(!g.is_aromatic(1));
        assert!(g.bonds[0].is_aromatic());
        assert!(g.bonds[0].in_ring());
    }
}
