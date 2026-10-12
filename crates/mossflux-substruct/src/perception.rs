//! Chemical perception: turns a parsed bond skeleton into a fully flagged
//! [`MolGraph`] with ring membership and sizes (via SSSR) and aromaticity
//! (via a Hückel model).
//!
//! Perception runs at the end of SMILES parsing (see [`crate::smiles::parse`]).
//! After it returns, these [`MolGraph`] fields are final: [`atom_flags::AROMATIC`],
//! [`atom_flags::IN_RING`], [`bond_flags::AROMATIC`], [`bond_flags::IN_RING`],
//! aromatic [`bond_order`]s, smallest-ring sizes and implicit-hydrogen counts.
//!
//! # Why re-derive aromaticity
//!
//! The parser takes a lowercase `c` as aromatic, but a Kekulé input
//! (`C1=CC=CC=C1`) has no lowercase letters. Both spellings must search
//! identically, so perception re-derives aromaticity from ring topology and bond
//! orders and overwrites the parse-time flags.
//!
//! # SSSR
//!
//! The Smallest Set of Smallest Rings has `μ = bonds − atoms + components` rings
//! (the circuit rank). For each bond we find the shortest cycle through it (BFS
//! with the bond removed), sort the candidates by size, and greedily accept a
//! cycle if it is linearly independent over GF(2) of those already accepted,
//! until `μ` rings are accepted.
//!
//! SSSR is not unique for symmetric cages (cubane), but ring membership and the
//! per-atom/bond smallest ring size (what `R`, `r<n>` and the aromaticity model
//! read) are the same for every valid SSSR. Tests assert only on those.
//!
//! # Hückel aromaticity
//!
//! A cycle is aromatic when every atom has a π contribution in the table below
//! (it is C, N, O, S, P or B with a free p-orbital) and the sum is `4n + 2`.
//!
//! ## Per-atom π-electron contributions
//!
//! Two signals, identical for Kekulé and aromatic input, drive the table:
//!
//! * a **localised ring double**: an `order == DOUBLE` bond to another ring atom,
//!   read from the original Kekulé orders;
//! * **σ-connectivity** `σ = heavy-neighbour degree + attached H`, which fixes a
//!   heteroatom's lone-pair geometry without reading bond orders.
//!
//! | Situation                                                   | π electrons |
//! |-------------------------------------------------------------|-------------|
//! | Atom has a localised ring double (C=C, C=N, …)               | 1           |
//! | C: exocyclic double bond (carbonyl C=O / C=N / C=C)          | 0           |
//! | C: neutral, no double, σ = 3 (aromatic-typed ring carbon)    | 1           |
//! | C: neutral, no double, σ ≠ 3 (sp³ CH₂)                       | reject ring |
//! | C⁺ empty p-orbital (tropylium)                               | 0           |
//! | C⁻ lone pair (cyclopentadienyl⁻)                            | 2           |
//! | N / P: σ = 3, no ring double (pyrrole-type lone-pair donor)  | 2           |
//! | N / P: σ = 2, no ring double (pyridine-type, lone pair plane)| 1           |
//! | N⁺ / P⁺: no double, σ ≥ 3 (pyridinium-type)                 | 1           |
//! | N⁺ / P⁺: no double, σ < 3 (no lone pair to give)            | reject ring |
//! | O / S: no ring double (furan-O, thiophene-S lone pair)       | 2           |
//! | O⁺ / S⁺ with a ring double (pyrylium-type)                  | 1           |
//! | B: empty p-orbital (borabenzene)                            | 0           |
//!
//! An atom that fits no row makes the ring non-aromatic.
//!
//! ## Fused systems
//!
//! Each SSSR ring is tested alone, then the perimeter (symmetric difference) of
//! every pair of SSSR rings that share a bond. The perimeter test catches cases
//! such as azulene, where the 10-π perimeter is aromatic but a single ring is
//! not. An atom or bond is aromatic if it lies in any aromatic ring or
//! perimeter. Unions of more than two rings are not tested.
//!
//! Last, implicit hydrogens of bare (non-bracket) atoms are recomputed from the
//! perceived aromaticity, so Kekulé and aromatic spellings get the same H counts.

use crate::graph::{atom_flags, bond_flags, bond_order, MolGraph};

/// The SSSR rings, each stored as its atom cycle and its bond set.
#[derive(Clone, Debug, Default)]
struct Rings {
    /// One entry per SSSR ring: the atom indices in cyclic order.
    atom_rings: Vec<Vec<u32>>,
    /// Parallel to [`Self::atom_rings`]: the bond indices forming each ring.
    bond_rings: Vec<Vec<u32>>,
}

/// Ring perception passed [`MAX_PERCEPTION_STEPS`] BFS steps on one molecule.
#[derive(Debug)]
pub(crate) struct PerceptionTooLarge;

/// The most BFS steps ring perception may take on one molecule. Real molecules
/// stay far below it, but a hostile graph whose ring bonds all lie on very long
/// cycles makes the per-bond BFS quadratic; past this budget the molecule is
/// refused.
pub const MAX_PERCEPTION_STEPS: usize = 1 << 26;

/// Runs full perception on a parsed [`MolGraph`] in place.
///
/// Order matters: rings, then aromaticity (reads ring membership), then
/// hydrogens and valence (read aromaticity). `h_locked` marks atoms whose H
/// count is fixed by the input (bracket atoms); those are never recomputed.
/// The Kekulé bond orders are snapshotted first, so the π-electron table can
/// read localised doubles after aromatic bonds are lowered to
/// [`bond_order::AROMATIC`].
pub(crate) fn perceive(
    mol: &mut MolGraph,
    h_locked: &[bool],
    implicit_element: &[Option<u8>],
) -> Result<(), PerceptionTooLarge> {
    // Parallel-array length invariants the perception pass and the matcher rely
    // on; a missing array fails loudly in debug builds.
    let n = mol.atom_count();
    debug_assert_eq!(mol.flags.len(), n, "flags array length == atom_count");
    debug_assert_eq!(mol.charge.len(), n, "charge array length == atom_count");
    debug_assert_eq!(mol.h_count.len(), n, "h_count array length == atom_count");
    debug_assert_eq!(mol.degree.len(), n, "degree array length == atom_count");
    debug_assert_eq!(h_locked.len(), n, "h_locked length == atom_count");
    debug_assert_eq!(
        implicit_element.len(),
        n,
        "implicit_element length == atom_count"
    );
    debug_assert_eq!(
        mol.nbr_offsets.len(),
        n + 1,
        "CSR offsets length == atom_count + 1"
    );

    // Snapshot the Kekulé bond orders before we overwrite any with AROMATIC.
    let kekule: Vec<u8> = mol.bonds.iter().map(|b| b.order).collect();

    let rings = sssr(mol)?;
    mark_ring_membership(mol, &rings);
    perceive_aromaticity(mol, &rings, &kekule);
    finalise_hydrogens(mol, h_locked, implicit_element, &kekule);
    compute_valence(mol, &kekule);
    build_arom_rings(mol, &rings);
    Ok(())
}

/// Fills `mol.valence` with each atom's total bond-order valence (the SMARTS `v<n>`
/// primitive), using the same model as [`finalise_hydrogens`] so it does not depend
/// on spelling: incident bond orders with aromatic counted as 1, plus the aromatic
/// +1 π-unit for an aromatic atom that is not a lone-pair donor, plus attached H.
///
/// The Kekulé snapshot is not summed directly: for aromatic input (`c1ccccc1`) it
/// holds the aromatic code. Examples: benzene CH = 1+1+1 π+1 H = v4; pyridine N =
/// v3; pyrrole N = v3.
fn compute_valence(mol: &mut MolGraph, kekule: &[u8]) {
    let n = mol.atom_count();
    let mut valence = vec![0u8; n];
    for (i, slot) in valence.iter_mut().enumerate() {
        let mut bond_sum: u32 = 0;
        for nb in mol.neighbors(i as u32) {
            bond_sum += match mol.bonds[nb.bond as usize].order {
                bond_order::SINGLE => 1,
                bond_order::DOUBLE => 2,
                bond_order::TRIPLE => 3,
                bond_order::AROMATIC => 1,
                _ => 1,
            };
        }
        // No +1 π-unit for (a) a lone-pair donor (pyrrole N, furan O), or (b) an atom
        // with an exocyclic localised double (ring carbonyl C, as in pyridone): that
        // double is already in `bond_sum`, so +1 would give an impossible carbon v5.
        let has_exocyclic_double = mol.neighbors(i as u32).iter().any(|nb| {
            kekule[nb.bond as usize] == bond_order::DOUBLE && !mol.bonds[nb.bond as usize].in_ring()
        });
        let arom_unit = if mol.is_aromatic(i as u32)
            && !aromatic_donates_lone_pair(mol, i as u32, kekule)
            && !has_exocyclic_double
        {
            1
        } else {
            0
        };
        let total = bond_sum + arom_unit + mol.h_count[i] as u32;
        *slot = total.min(u8::MAX as u32) as u8;
    }
    mol.valence = valence;
}

/// Records the canonical `(element, aromatic)` token of every SSSR ring whose bonds
/// are all aromatic into `mol.arom_rings`, and marks benzene-ring atoms in
/// `mol.carbocyclic_arom6`.
///
/// A pure-aromatic single-ring SMARTS query (e.g. `c1ccncc1`) whose token is present
/// always matches: its aromatic-element atom predicates hold on the ring atoms and
/// its default single-or-aromatic bonds hold on the aromatic ring bonds. So the
/// matcher can answer it without VF2.
fn build_arom_rings(mol: &mut MolGraph, rings: &Rings) {
    let mut out: Vec<Box<[u8]>> = Vec::new();
    let mut benzene = vec![false; mol.atom_count()];
    for (atoms, bonds) in rings.atom_rings.iter().zip(rings.bond_rings.iter()) {
        if !bonds.iter().all(|&b| mol.bonds[b as usize].is_aromatic()) {
            continue;
        }
        let labels: Vec<(u8, bool)> = atoms
            .iter()
            .map(|&a| (mol.element[a as usize], mol.is_aromatic(a)))
            .collect();
        out.push(crate::graph::canonical_ring_token(&labels));
        // Benzene: six carbons on all-aromatic bonds.
        if atoms.len() == 6 && atoms.iter().all(|&a| mol.element[a as usize] == 6) {
            for &a in atoms {
                benzene[a as usize] = true;
            }
        }
    }
    mol.arom_rings = out;
    mol.carbocyclic_arom6 = benzene;
}

/// The circuit rank `μ = E − V + C`: the number of SSSR rings to collect.
fn circuit_rank(mol: &MolGraph) -> usize {
    let v = mol.atom_count();
    let e = mol.bond_count();
    let c = connected_components(mol);
    // E - V + C is never negative for a real graph.
    (e + c).saturating_sub(v)
}

/// Counts connected components via a union-find over the bond list.
fn connected_components(mol: &MolGraph) -> usize {
    let n = mol.atom_count();
    if n == 0 {
        return 0;
    }
    let mut parent: Vec<u32> = (0..n as u32).collect();

    fn find(parent: &mut [u32], mut x: u32) -> u32 {
        while parent[x as usize] != x {
            parent[x as usize] = parent[parent[x as usize] as usize]; // path halving
            x = parent[x as usize];
        }
        x
    }

    for b in &mol.bonds {
        let ra = find(&mut parent, b.a);
        let rb = find(&mut parent, b.b);
        if ra != rb {
            parent[ra as usize] = rb;
        }
    }
    let mut roots = 0;
    for i in 0..n as u32 {
        if find(&mut parent, i) == i {
            roots += 1;
        }
    }
    roots
}

/// Builds an SSSR for `mol` (see the module docs).
fn sssr(mol: &MolGraph) -> Result<Rings, PerceptionTooLarge> {
    let target = circuit_rank(mol);
    let mut rings = Rings::default();
    if target == 0 {
        return Ok(rings);
    }

    // Collect a candidate shortest cycle through each bond. A bridge lies on no
    // cycle (its BFS returns `None`), so skipping bridges gives the same
    // candidates without an O(atoms) BFS per chain bond.
    //
    // A block (biconnected component) that is one simple cycle yields the same
    // candidate from every bond. Only its lowest-numbered bond runs the BFS: the
    // greedy pass would accept that copy and reject the rest as dependent, so the
    // result is unchanged, and one huge ring stays linear.
    let blocks = blocks(mol);
    let mut cycle_block_done = vec![false; blocks.count];
    let mut scratch = BfsScratch::new(mol.atom_count());
    let mut candidates: Vec<RingCandidate> = Vec::new();
    for bond_idx in 0..mol.bond_count() as u32 {
        let block = blocks.of_bond[bond_idx as usize] as usize;
        if blocks.is_bridge(block) {
            continue;
        }
        if blocks.is_cycle[block] {
            if cycle_block_done[block] {
                continue;
            }
            cycle_block_done[block] = true;
        }
        if let Some(cand) = shortest_cycle_through_bond(mol, bond_idx, &mut scratch)? {
            candidates.push(cand);
        }
    }
    // Smallest rings first; ties broken by the bond set so the result is
    // deterministic.
    candidates.sort_by(|x, y| {
        x.bonds
            .len()
            .cmp(&y.bonds.len())
            .then_with(|| x.bond_mask.cmp(&y.bond_mask))
    });

    // Greedily accept GF(2)-independent cycles until there are `target`. Which
    // candidates are accepted depends only on linear independence, so the
    // elimination scheme cannot change the result.
    let mut basis = Gf2Basis::default();
    for cand in candidates {
        if rings.atom_rings.len() == target {
            break;
        }
        if basis.insert_if_independent(&cand.bond_mask) {
            rings.atom_rings.push(cand.atoms);
            rings.bond_rings.push(cand.bonds);
        }
    }
    Ok(rings)
}

/// The blocks (biconnected components) of a molecule, as bond labels.
struct Blocks {
    /// `of_bond[b]` is the block of bond `b`.
    of_bond: Vec<u32>,
    /// Number of blocks.
    count: usize,
    /// Bonds per block: a block of one bond is a bridge (on no cycle).
    bonds: Vec<u32>,
    /// True if the block is one simple cycle (as many bonds as atoms).
    is_cycle: Vec<bool>,
}

impl Blocks {
    fn is_bridge(&self, block: usize) -> bool {
        self.bonds[block] == 1
    }
}

/// Labels every bond with its block, by Tarjan's low-link DFS with an edge stack,
/// run with an explicit stack so a long chain cannot overflow the call stack.
fn blocks(mol: &MolGraph) -> Blocks {
    const UNSEEN: u32 = u32::MAX;
    let n = mol.atom_count();
    let m = mol.bond_count();
    let mut out = Blocks {
        of_bond: vec![UNSEEN; m],
        count: 0,
        bonds: Vec::new(),
        is_cycle: Vec::new(),
    };
    let mut disc = vec![UNSEEN; n];
    let mut low = vec![0u32; n];
    let mut atom_stamp = vec![UNSEEN; n];
    let mut time = 0u32;
    let mut edges: Vec<u32> = Vec::new();
    // Each frame: (atom, bond used to reach it, next neighbour offset).
    let mut stack: Vec<(u32, u32, usize)> = Vec::new();
    for root in 0..n as u32 {
        if disc[root as usize] != UNSEEN {
            continue;
        }
        disc[root as usize] = time;
        low[root as usize] = time;
        time += 1;
        stack.push((root, UNSEEN, 0));
        while let Some(frame) = stack.last_mut() {
            let (v, via, next) = *frame;
            let nbrs = mol.neighbors(v);
            if next < nbrs.len() {
                frame.2 += 1;
                let nb = nbrs[next];
                if nb.bond == via {
                    continue;
                }
                let w = nb.atom as usize;
                if disc[w] == UNSEEN {
                    edges.push(nb.bond);
                    disc[w] = time;
                    low[w] = time;
                    time += 1;
                    stack.push((nb.atom, nb.bond, 0));
                } else if disc[w] < disc[v as usize] {
                    // A back edge to an ancestor (each is seen once from below).
                    edges.push(nb.bond);
                    low[v as usize] = low[v as usize].min(disc[w]);
                }
            } else {
                stack.pop();
                if let Some(&(u, _, _)) = stack.last() {
                    let (u, v) = (u as usize, v as usize);
                    low[u] = low[u].min(low[v]);
                    if low[v] >= disc[u] {
                        // `u` separates `v`'s subtree: pop its block, down to and
                        // including the tree edge `via` that reached `v`.
                        let id = out.count as u32;
                        let (mut nbonds, mut natoms) = (0u32, 0u32);
                        loop {
                            let b = edges.pop().expect("tree edge is on the stack");
                            out.of_bond[b as usize] = id;
                            nbonds += 1;
                            let bond = &mol.bonds[b as usize];
                            for a in [bond.a, bond.b] {
                                if atom_stamp[a as usize] != id {
                                    atom_stamp[a as usize] = id;
                                    natoms += 1;
                                }
                            }
                            if b == via {
                                break;
                            }
                        }
                        out.count += 1;
                        out.bonds.push(nbonds);
                        out.is_cycle.push(nbonds >= 3 && nbonds == natoms);
                    }
                }
            }
        }
    }
    debug_assert!(out.of_bond.iter().all(|&b| b != UNSEEN));
    out
}

/// BFS scratch reused across the bonds of one molecule. `seen` is
/// epoch-stamped, so starting a new BFS costs O(1), not O(atoms); `prev_*` are
/// read only for atoms seen in the current epoch.
struct BfsScratch {
    seen: Vec<u32>,
    epoch: u32,
    /// BFS steps taken on this molecule so far, against [`MAX_PERCEPTION_STEPS`].
    steps: usize,
    prev_atom: Vec<u32>,
    prev_bond: Vec<u32>,
    queue: std::collections::VecDeque<u32>,
}

impl BfsScratch {
    fn new(n: usize) -> Self {
        BfsScratch {
            seen: vec![0; n],
            epoch: 0,
            steps: 0,
            prev_atom: vec![u32::MAX; n],
            prev_bond: vec![u32::MAX; n],
            queue: std::collections::VecDeque::new(),
        }
    }
}

/// An incrementally reduced GF(2) basis: each stored vector has a pivot bit that
/// no LATER vector has, so a vector reduces in one pass in insertion order.
#[derive(Default)]
struct Gf2Basis {
    rows: Vec<(usize, BitSet)>,
}

impl Gf2Basis {
    /// Adds `v` and returns `true` if it is independent of the basis; else
    /// returns `false` and leaves the basis as it was.
    ///
    /// Why one pass is exact: after the step for row `i`, the pivot of row `i` is
    /// clear, and later rows do not hold it, so it stays clear. A non-zero vector
    /// in the span holds the pivot of its first row, so a reduced vector that is
    /// still non-zero is outside the span. It then becomes a new row, with its
    /// lowest bit as pivot; it holds no earlier pivot, which keeps the invariant.
    fn insert_if_independent(&mut self, v: &BitSet) -> bool {
        let mut r = v.clone();
        for (pivot, row) in &self.rows {
            if r.get(*pivot) {
                r.xor(row);
            }
        }
        match r.lowest_set() {
            Some(pivot) => {
                self.rows.push((pivot, r));
                true
            }
            None => false,
        }
    }
}

/// A candidate ring discovered as the shortest cycle through one bond.
struct RingCandidate {
    atoms: Vec<u32>,
    bonds: Vec<u32>,
    /// Bond-incidence bit vector (bit `i` set iff bond `i` is in the cycle), used
    /// for the GF(2) independence test.
    bond_mask: BitSet,
}

/// Finds the shortest cycle through bond `bond_idx`: a BFS between its endpoints
/// with the bond removed, closed by the bond. Returns `None` for a bridge.
fn shortest_cycle_through_bond(
    mol: &MolGraph,
    bond_idx: u32,
    scratch: &mut BfsScratch,
) -> Result<Option<RingCandidate>, PerceptionTooLarge> {
    let bond = mol.bonds[bond_idx as usize];
    let (src, dst) = (bond.a, bond.b);

    // BFS from `src` to `dst` without `bond_idx`, recording predecessors.
    scratch.epoch = match scratch.epoch.checked_add(1) {
        Some(e) => e,
        None => {
            scratch.seen.fill(0);
            1
        }
    };
    let epoch = scratch.epoch;
    let BfsScratch {
        seen,
        prev_atom,
        prev_bond,
        queue,
        steps,
        ..
    } = scratch;
    queue.clear();
    seen[src as usize] = epoch;
    queue.push_back(src);

    while let Some(cur) = queue.pop_front() {
        if cur == dst {
            break;
        }
        *steps += 1;
        if *steps > MAX_PERCEPTION_STEPS {
            return Err(PerceptionTooLarge);
        }
        for nb in mol.neighbors(cur) {
            if nb.bond == bond_idx {
                continue; // the deleted bond
            }
            if seen[nb.atom as usize] != epoch {
                seen[nb.atom as usize] = epoch;
                prev_atom[nb.atom as usize] = cur;
                prev_bond[nb.atom as usize] = nb.bond;
                queue.push_back(nb.atom);
            }
        }
    }

    if seen[dst as usize] != epoch {
        return Ok(None); // bond is a bridge — not in any ring
    }

    // Reconstruct the path dst -> src, then add the closing bond.
    let mut atoms = Vec::new();
    let mut bonds = Vec::new();
    let mut bond_mask = BitSet::new(mol.bond_count());
    let mut cur = dst;
    atoms.push(cur);
    while cur != src {
        let pb = prev_bond[cur as usize];
        bonds.push(pb);
        bond_mask.set(pb as usize);
        cur = prev_atom[cur as usize];
        atoms.push(cur);
    }
    // Close the cycle with the deleted bond.
    bonds.push(bond_idx);
    bond_mask.set(bond_idx as usize);

    Ok(Some(RingCandidate {
        atoms,
        bonds,
        bond_mask,
    }))
}

/// A fixed-capacity bitset over bond indices, for the GF(2) cycle space.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BitSet {
    words: Vec<u64>,
}

impl BitSet {
    fn new(bits: usize) -> Self {
        BitSet {
            words: vec![0u64; bits.div_ceil(64).max(1)],
        }
    }

    #[inline]
    fn set(&mut self, bit: usize) {
        self.words[bit / 64] |= 1u64 << (bit % 64);
    }

    /// Index of the lowest set bit (the elimination pivot), or `None` if empty.
    fn lowest_set(&self) -> Option<usize> {
        for (w, &word) in self.words.iter().enumerate() {
            if word != 0 {
                return Some(w * 64 + word.trailing_zeros() as usize);
            }
        }
        None
    }

    #[inline]
    fn get(&self, bit: usize) -> bool {
        (self.words[bit / 64] >> (bit % 64)) & 1 != 0
    }

    /// `self ^= other`: GF(2) addition.
    fn xor(&mut self, other: &BitSet) {
        for (a, b) in self.words.iter_mut().zip(&other.words) {
            *a ^= *b;
        }
    }
}

/// Sets ring-membership flags, SSSR ring counts and ring sizes from the SSSR.
fn mark_ring_membership(mol: &mut MolGraph, rings: &Rings) {
    let n = mol.atom_count();
    let m = mol.bond_count();
    // 0 means "not in any ring".
    let mut atom_ring_size = vec![0u8; n];
    let mut bond_ring_size = vec![0u8; m];
    // SSSR ring count per atom (the `R<n>` primitive): a fusion atom in two
    // 6-rings has count 2.
    let mut ring_count = vec![0u8; n];

    for (atoms, bonds) in rings.atom_rings.iter().zip(&rings.bond_rings) {
        // Saturate: a ring of 256+ atoms reads as 255, never wraps to a small
        // size. The SMARTS parser refuses counts above 254, so 255 never equals
        // a query value.
        let size = atoms.len().min(u8::MAX as usize) as u8;
        for &a in atoms {
            mol.flags[a as usize] |= atom_flags::IN_RING;
            ring_count[a as usize] = ring_count[a as usize].saturating_add(1);
            let slot = &mut atom_ring_size[a as usize];
            *slot = if *slot == 0 { size } else { (*slot).min(size) };
        }
        for &b in bonds {
            mol.bonds[b as usize].flags |= bond_flags::IN_RING;
            let slot = &mut bond_ring_size[b as usize];
            *slot = if *slot == 0 { size } else { (*slot).min(size) };
        }
    }

    mol.atom_ring_size = atom_ring_size;
    mol.ring_count = ring_count;

    // All distinct ring sizes per atom (for exact `r<n>` on multi-ring atoms),
    // stored in flat CSR form: sorted sizes plus a prefix-sum offset table.
    let mut per_atom: Vec<Vec<u8>> = vec![Vec::new(); n];
    for atoms in &rings.atom_rings {
        let size = atoms.len().min(u8::MAX as usize) as u8;
        for &a in atoms {
            let v = &mut per_atom[a as usize];
            if !v.contains(&size) {
                v.push(size);
            }
        }
    }
    let mut ring_size_offsets = vec![0u32; n + 1];
    for i in 0..n {
        per_atom[i].sort_unstable();
        ring_size_offsets[i + 1] = ring_size_offsets[i] + per_atom[i].len() as u32;
    }
    let mut atom_ring_sizes = Vec::with_capacity(ring_size_offsets[n] as usize);
    for sizes in &per_atom {
        atom_ring_sizes.extend_from_slice(sizes);
    }
    mol.atom_ring_sizes = atom_ring_sizes;
    mol.ring_size_offsets = ring_size_offsets;
    mol.bond_ring_size = bond_ring_size;
}

/// Marks atoms and bonds aromatic where an SSSR ring, or the perimeter of two
/// bond-sharing rings, passes the Hückel test.
fn perceive_aromaticity(mol: &mut MolGraph, rings: &Rings, kekule: &[u8]) {
    let mut aromatic_atom = vec![false; mol.atom_count()];
    let mut aromatic_bond = vec![false; mol.bond_count()];

    // Individual SSSR rings.
    for (atoms, bonds) in rings.atom_rings.iter().zip(&rings.bond_rings) {
        if ring_is_aromatic(mol, atoms, bonds, kekule) {
            for &a in atoms {
                aromatic_atom[a as usize] = true;
            }
            for &b in bonds {
                aromatic_bond[b as usize] = true;
            }
        }
    }

    // Perimeters of every pair of rings sharing at least one bond.
    let r = rings.atom_rings.len();
    for i in 0..r {
        for j in (i + 1)..r {
            if let Some((patoms, pbonds)) =
                fused_perimeter(&rings.bond_rings[i], &rings.bond_rings[j], mol)
            {
                if ring_is_aromatic(mol, &patoms, &pbonds, kekule) {
                    for &a in &patoms {
                        aromatic_atom[a as usize] = true;
                    }
                    for &b in &pbonds {
                        aromatic_bond[b as usize] = true;
                    }
                }
            }
        }
    }

    // Set the flags, and lower aromatic bonds to the aromatic bond order.
    for (i, &arom) in aromatic_atom.iter().enumerate() {
        if arom {
            mol.flags[i] |= atom_flags::AROMATIC;
        } else {
            mol.flags[i] &= !atom_flags::AROMATIC;
        }
    }
    for (i, &arom) in aromatic_bond.iter().enumerate() {
        if arom {
            mol.bonds[i].flags |= bond_flags::AROMATIC;
            mol.bonds[i].order = bond_order::AROMATIC;
        } else {
            mol.bonds[i].flags &= !bond_flags::AROMATIC;
            // Restore the Kekulé order of a non-aromatic bond.
            //
            // Invariant: `order == AROMATIC ⇔ AROMATIC flag`. An aromatic-input
            // bond that perception rejected has no Kekulé order on record, so it
            // becomes SINGLE (as the parser treats a default bond between
            // non-aromatic atoms). Kekulé inputs keep SINGLE/DOUBLE/TRIPLE.
            let restored = kekule[i];
            mol.bonds[i].order = if restored == bond_order::AROMATIC {
                bond_order::SINGLE
            } else {
                restored
            };
        }
    }
}

/// Returns the atoms and bonds of the perimeter (symmetric difference) of two
/// rings, or `None` if they share no bond or the perimeter is not one simple cycle.
fn fused_perimeter(
    bonds_i: &[u32],
    bonds_j: &[u32],
    mol: &MolGraph,
) -> Option<(Vec<u32>, Vec<u32>)> {
    use std::collections::BTreeSet;
    let set_i: BTreeSet<u32> = bonds_i.iter().copied().collect();
    let set_j: BTreeSet<u32> = bonds_j.iter().copied().collect();
    if set_i.is_disjoint(&set_j) {
        return None;
    }
    let perim_bonds: Vec<u32> = set_i.symmetric_difference(&set_j).copied().collect();
    if perim_bonds.is_empty() {
        return None;
    }

    // Adjacency over perimeter bonds; a simple cycle has every degree exactly 2.
    let mut adj: std::collections::HashMap<u32, Vec<(u32, u32)>> = std::collections::HashMap::new();
    for &b in &perim_bonds {
        let bond = mol.bonds[b as usize];
        adj.entry(bond.a).or_default().push((bond.b, b));
        adj.entry(bond.b).or_default().push((bond.a, b));
    }
    if adj.values().any(|v| v.len() != 2) {
        return None; // not a single simple cycle
    }

    let start = *adj.keys().next()?;
    let mut atoms = vec![start];
    let mut prev = start;
    let (mut cur, _) = adj[&start][0];
    let mut guard = 0;
    while cur != start {
        atoms.push(cur);
        let nbrs = &adj[&cur];
        let next = if nbrs[0].0 == prev {
            nbrs[1].0
        } else {
            nbrs[0].0
        };
        prev = cur;
        cur = next;
        guard += 1;
        if guard > perim_bonds.len() + 1 {
            return None;
        }
    }
    if atoms.len() != perim_bonds.len() {
        return None; // more than one cycle
    }
    Some((atoms, perim_bonds))
}

/// The Hückel test on one cycle: `true` iff every atom has a π contribution and
/// their sum is `4n + 2`.
fn ring_is_aromatic(mol: &MolGraph, atoms: &[u32], bonds: &[u32], kekule: &[u8]) -> bool {
    let mut pi_sum: i32 = 0;
    for &a in atoms {
        match pi_contribution(mol, a, bonds, kekule) {
            Some(p) => pi_sum += p as i32,
            None => return false,
        }
    }
    // pi_sum ∈ {2, 6, 10, 14, ...}.
    pi_sum >= 2 && (pi_sum - 2) % 4 == 0
}

/// The π-electron contribution of ring atom `a`, or `None` if `a` cannot be in an
/// aromatic ring (see the module-level table).
///
/// A ring can arrive Kekulé (`C1=CC=CC=C1`) or aromatic (`c1ccccc1`). An aromatic
/// bond order cannot tell a pyridine N from a pyrrole N, so the atom is classified
/// from two signals that are the same in both spellings: a localised ring double,
/// and σ-connectivity (`sigma = heavy degree + H`), which tells whether a
/// heteroatom's lone pair is in the ring π system (pyrrole) or in-plane (pyridine).
///
/// `ring_bonds` is the bond set of the cycle under test; `kekule` holds the
/// pre-aromatisation bond orders.
fn pi_contribution(mol: &MolGraph, a: u32, ring_bonds: &[u32], kekule: &[u8]) -> Option<u8> {
    let element = mol.element[a as usize];
    let charge = mol.charge[a as usize];

    // Only localised (Kekulé) double/triple orders count.
    let mut has_ring_double = false; // localised DOUBLE to another ring atom
    let mut has_exocyclic_double = false; // localised DOUBLE leaving the ring
    let mut has_exocyclic_double_to_heteroatom = false;
    let mut has_triple = false;

    for nb in mol.neighbors(a) {
        let order = kekule[nb.bond as usize];
        if order == bond_order::TRIPLE {
            has_triple = true;
        }
        if order == bond_order::DOUBLE {
            if ring_bonds.contains(&nb.bond) {
                has_ring_double = true;
            } else {
                has_exocyclic_double = true;
                if matches!(mol.element[nb.atom as usize], 7 | 8 | 16) {
                    has_exocyclic_double_to_heteroatom = true;
                }
            }
        }
    }

    // A triple bond (anywhere on the atom) breaks the planar π system.
    if has_triple {
        return None;
    }

    // Pyrrole-type: σ = 3, lone pair in p-orbital. Pyridine-type: σ = 2.
    let sigma = mol.degree[a as usize] as i32 + mol.h_count[a as usize] as i32;

    match element {
        // --- Carbon ---
        6 => {
            if has_ring_double {
                return Some(1); // sp² carbon with a ring π bond
            }
            if has_exocyclic_double {
                // Exocyclic C=O / C=N / C=C (tropone, pyranone): sp², donates 0.
                let _ = has_exocyclic_double_to_heteroatom;
                return Some(0);
            }
            // No localised double: a neutral sp² ring carbon (benzene `c`) has
            // σ = 3 and donates 1.
            match charge {
                c if c > 0 => Some(0),      // tropylium-type empty p-orbital
                c if c < 0 => Some(2),      // cyclopentadienyl-type carbanion lone pair
                _ if sigma == 3 => Some(1), // aromatic-typed neutral ring carbon
                // σ ≠ 3 with no double is sp³ (CH₂): no p-orbital.
                _ => None,
            }
        }
        // --- Nitrogen / Phosphorus ---
        7 | 15 => {
            if has_ring_double {
                return Some(1); // pyridine-type: π bond, lone pair in plane
            }
            if has_exocyclic_double_to_heteroatom {
                return Some(1); // N-oxide / nitro-on-ring-N style
            }
            // No localised double: σ ≥ 3 is pyrrole-type (2 e), σ == 2 is
            // pyridine-type (1 e). A cation with σ < 3 has no lone pair: reject.
            match charge {
                // A cationic N with σ ≥ 3 is pyridinium-type (protonated,
                // quaternised, or the N-oxide `[n+][O-]`) and donates 1, not 2;
                // 2 would make the count odd and disagree with the Kekulé spelling.
                c if c > 0 && sigma >= 3 => Some(1),
                c if c > 0 => None,
                _ if sigma >= 3 => Some(2), // pyrrole-type (neutral, lone pair in p)
                _ => Some(1),               // pyridine-type (σ == 2)
            }
        }
        // --- Oxygen / Sulfur ---
        8 | 16 => {
            if has_ring_double {
                return Some(1); // pyrylium-type O⁺ / thiopyrylium ring π bond
            }
            // Furan O / thiophene S, charged or not: a lone pair (2 e).
            Some(2)
        }
        // --- Boron ---
        5 => {
            // Empty p-orbital (borole, borabenzene).
            Some(0)
        }
        _ => None,
    }
}

/// Whether an aromatic atom donates a lone pair (2 π electrons: pyrrole N, furan
/// O, thiophene S) instead of one electron through a formal double bond.
///
/// The hydrogen and valence models use this to skip the aromatic +1 unit of
/// [`crate::smiles::implicit_h`], which is correct only for a π-bond contributor.
/// It mirrors the heteroatom rows of [`pi_contribution`]:
///
/// * N / P: donor iff no localised ring double, σ ≥ 3 and not cationic.
/// * O / S: donor iff no localised ring double.
/// * Other elements: never a donor.
///
/// `kekule` holds the pre-aromatisation bond orders.
fn aromatic_donates_lone_pair(mol: &MolGraph, a: u32, kekule: &[u8]) -> bool {
    let element = mol.element[a as usize];
    if !matches!(element, 7 | 8 | 15 | 16) {
        return false;
    }
    // A localised ring double: pyridine-type or pyrylium-type, not a donor.
    let mut has_ring_double = false;
    for nb in mol.neighbors(a) {
        if kekule[nb.bond as usize] == bond_order::DOUBLE && mol.bonds[nb.bond as usize].in_ring() {
            has_ring_double = true;
        }
    }
    if has_ring_double {
        return false;
    }
    match element {
        // A cationic N with σ ≥ 3 is pyridinium-type (contributes 1), so it takes
        // the +1 unit and gets v4.
        7 | 15 => {
            let sigma = mol.degree[a as usize] as i32 + mol.h_count[a as usize] as i32;
            sigma >= 3 && mol.charge[a as usize] <= 0
        }
        8 | 16 => true,
        _ => false,
    }
}

/// Recomputes implicit hydrogens of bare organic atoms from the perceived
/// aromaticity, so Kekulé and aromatic spellings agree.
///
/// Bracket atoms (`h_locked[i]`) keep their input H count. Bare atoms use
/// [`crate::smiles::implicit_h`] on the current bond orders. A lone-pair donor
/// (see [`aromatic_donates_lone_pair`]) gets no aromatic +1 unit: it forms only σ
/// bonds, so the +1 would drop a hydrogen (Kekulé pyrrole's N-H would lose its H).
fn finalise_hydrogens(
    mol: &mut MolGraph,
    h_locked: &[bool],
    implicit_element: &[Option<u8>],
    kekule: &[u8],
) {
    for i in 0..mol.atom_count() {
        if h_locked[i] {
            continue; // bracket atom: H is fixed by input
        }
        let Some(element) = implicit_element[i] else {
            continue;
        };
        // Aromatic counts as 1; `implicit_h` adds the aromatic unit itself.
        let mut bond_sum: u32 = 0;
        for nb in mol.neighbors(i as u32) {
            let order = mol.bonds[nb.bond as usize].order;
            bond_sum += match order {
                bond_order::SINGLE => 1,
                bond_order::DOUBLE => 2,
                bond_order::TRIPLE => 3,
                bond_order::AROMATIC => 1,
                _ => 1,
            };
        }
        let aromatic = mol.is_aromatic(i as u32);
        let apply_aromatic_unit = aromatic && !aromatic_donates_lone_pair(mol, i as u32, kekule);
        mol.h_count[i] = crate::smiles::implicit_h(element, bond_sum, apply_aromatic_unit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smiles::parse;

    /// Parse + perceive (perception runs inside `parse`), panicking on error.
    fn p(s: &str) -> MolGraph {
        parse(s).unwrap_or_else(|e| panic!("parse({s:?}) failed: {e}"))
    }

    /// Number of atoms flagged aromatic.
    fn n_aromatic_atoms(g: &MolGraph) -> usize {
        (0..g.atom_count() as u32)
            .filter(|&i| g.is_aromatic(i))
            .count()
    }

    /// Every atom is flagged aromatic.
    fn all_aromatic(g: &MolGraph) -> bool {
        (0..g.atom_count() as u32).all(|i| g.is_aromatic(i))
    }

    /// Every atom is flagged in-ring.
    fn all_in_ring(g: &MolGraph) -> bool {
        (0..g.atom_count() as u32).all(|i| g.in_ring(i))
    }

    // --- Aromatic ---

    #[test]
    fn benzene_is_aromatic_and_all_in_a_six_ring() {
        let g = p("c1ccccc1");
        assert!(all_aromatic(&g), "all benzene carbons aromatic");
        assert!(all_in_ring(&g));
        // Smallest ring is size 6 for every atom and bond.
        assert!(g.atom_ring_size.iter().all(|&s| s == 6));
        assert!(g.bond_ring_size.iter().all(|&s| s == 6));
        // Every bond aromatic, lowered to the aromatic order.
        assert!(g
            .bonds
            .iter()
            .all(|b| b.is_aromatic() && b.order == bond_order::AROMATIC));
        // r6 holds, r5 does not, for atom 0.
        assert!(g.in_ring_of_size(0, 6));
        assert!(!g.in_ring_of_size(0, 5));
    }

    #[test]
    fn kekule_benzene_perceives_identically_to_aromatic_form() {
        // A Kekulé spelling carries no lowercase letters; perception must still
        // find the aromatic system, and the H counts must match aromatic benzene.
        let arom = p("c1ccccc1");
        let kek = p("C1=CC=CC=C1");
        assert!(all_aromatic(&kek), "Kekulé benzene perceived aromatic");
        assert_eq!(kek.h_count, vec![1; 6], "each aromatic CH has one H");
        assert_eq!(arom.h_count, kek.h_count, "Kekulé and aromatic H agree");
        assert!(kek
            .bonds
            .iter()
            .all(|b| b.order == bond_order::AROMATIC && b.is_aromatic()));
    }

    #[test]
    fn pyridine_is_aromatic_nitrogen_donates_one() {
        let g = p("c1ccncc1");
        assert!(all_aromatic(&g));
        // Kekulé pyridine perceives identically.
        let k = p("C1=CC=NC=C1");
        assert!(all_aromatic(&k), "Kekulé pyridine aromatic");
        // The aromatic N has no implicit H (pyridine-type).
        let n_idx = (0..g.atom_count()).find(|&i| g.element[i] == 7).unwrap();
        assert_eq!(g.h_count[n_idx], 0);
    }

    #[test]
    fn pyrrole_is_aromatic_nitrogen_donates_lone_pair() {
        // Pyrrole: 4 C + 1 N-H, the N donates 2 π electrons (σ = 3).
        let g = p("c1cc[nH]c1");
        assert!(all_aromatic(&g), "all pyrrole atoms aromatic");
        assert_eq!(n_aromatic_atoms(&g), 5);
        let n_idx = (0..g.atom_count()).find(|&i| g.element[i] == 7).unwrap();
        assert!(g.is_aromatic(n_idx as u32));
        assert!(g.in_ring_of_size(n_idx as u32, 5));
    }

    #[test]
    fn furan_oxygen_donates_lone_pair() {
        // Furan: 4 C + 1 O; O donates 2, carbons 1 each → 6 π electrons.
        let g = p("c1ccoc1");
        assert!(all_aromatic(&g), "all furan atoms aromatic");
        let o_idx = (0..g.atom_count()).find(|&i| g.element[i] == 8).unwrap();
        assert!(g.is_aromatic(o_idx as u32));
        // Aromatic furan-O has no H.
        assert_eq!(g.h_count[o_idx], 0);
    }

    #[test]
    fn thiophene_sulfur_donates_lone_pair() {
        let g = p("c1ccsc1");
        assert!(all_aromatic(&g), "all thiophene atoms aromatic");
        let s_idx = (0..g.atom_count()).find(|&i| g.element[i] == 16).unwrap();
        assert!(g.is_aromatic(s_idx as u32));
    }

    #[test]
    fn naphthalene_is_fully_aromatic_two_fused_six_rings() {
        // Fused bicyclic: 10 carbons, every one aromatic; bridgeheads sit in two
        // six-membered rings.
        let g = p("c1ccc2ccccc2c1");
        assert_eq!(g.atom_count(), 10);
        assert!(all_aromatic(&g), "all naphthalene carbons aromatic");
        assert!(all_in_ring(&g));
        // Every atom's smallest ring is size 6.
        assert!(g.atom_ring_size.iter().all(|&s| s == 6));
        // The two fusion (bridgehead) carbons have degree 3 and belong to two
        // distinct six-rings — but their *smallest* size is still 6.
        let bridgeheads: Vec<usize> = (0..g.atom_count()).filter(|&i| g.degree[i] == 3).collect();
        assert_eq!(bridgeheads.len(), 2, "naphthalene has two bridgeheads");
        for b in bridgeheads {
            assert!(g.in_ring_of_size(b as u32, 6));
            assert!(!g.in_ring_of_size(b as u32, 5));
        }
        // SSSR has exactly two rings (circuit rank = 11 bonds - 10 atoms + 1).
        assert_eq!(circuit_rank(&g), 2);
    }

    #[test]
    fn indole_fused_five_and_six_rings_are_aromatic() {
        // Indole: benzene fused to pyrrole; all 9 atoms aromatic.
        let g = p("c1ccc2[nH]ccc2c1");
        assert!(all_aromatic(&g), "all indole atoms aromatic");
        // It has a five-ring and a six-ring.
        let has5 = (0..g.atom_count() as u32).any(|i| g.in_ring_of_size(i, 5));
        let has6 = (0..g.atom_count() as u32).any(|i| g.in_ring_of_size(i, 6));
        assert!(has5 && has6, "indole has both a 5- and a 6-membered ring");
    }

    // --- NOT aromatic ---

    #[test]
    fn cyclohexane_is_not_aromatic_but_is_a_six_ring() {
        let g = p("C1CCCCC1");
        assert_eq!(n_aromatic_atoms(&g), 0, "cyclohexane is not aromatic");
        assert!(all_in_ring(&g), "but every atom is in a ring");
        assert!(g.atom_ring_size.iter().all(|&s| s == 6));
        // No bond is aromatic; all remain single.
        assert!(g.bonds.iter().all(|b| !b.is_aromatic()));
        assert!(g.bonds.iter().all(|b| b.order == bond_order::SINGLE));
        // Each carbon keeps two implicit hydrogens (CH₂).
        assert_eq!(g.h_count, vec![2; 6]);
    }

    #[test]
    fn cyclopentadiene_is_not_aromatic_due_to_sp3_carbon() {
        // The sp³ CH₂ of 1,3-cyclopentadiene has no p-orbital.
        let g = p("C1=CC=CC1");
        assert_eq!(n_aromatic_atoms(&g), 0, "cyclopentadiene not aromatic");
        assert!(all_in_ring(&g));
        assert!(g.atom_ring_size.iter().all(|&s| s == 5));
    }

    #[test]
    fn cyclobutadiene_4_pi_is_not_aromatic() {
        // 4 π electrons (4n): anti-aromatic, not flagged.
        let g = p("C1=CC=C1");
        assert_eq!(n_aromatic_atoms(&g), 0, "4π cyclobutadiene not aromatic");
        assert!(all_in_ring(&g));
    }

    #[test]
    fn acyclic_molecule_has_no_rings() {
        let g = p("CCO");
        assert_eq!(circuit_rank(&g), 0);
        assert!(g.atom_ring_size.iter().all(|&s| s == 0));
        assert!((0..g.atom_count() as u32).all(|i| !g.in_ring(i)));
        assert!((0..g.atom_count() as u32).all(|i| !g.is_aromatic(i)));
    }

    // --- Ring membership / size correctness ---

    #[test]
    fn ring_membership_distinguishes_ring_from_substituent() {
        // Toluene: a benzene ring (aromatic, in-ring) with a methyl substituent
        // (aliphatic, not in a ring).
        let g = p("Cc1ccccc1");
        // Atom 0 is the methyl carbon: aliphatic, acyclic.
        assert!(!g.is_aromatic(0));
        assert!(!g.in_ring(0));
        assert_eq!(g.atom_ring_size[0], 0);
        // The six ring carbons are aromatic and in a six-ring.
        let ring_aromatic = (1..7).filter(|&i| g.is_aromatic(i)).count();
        assert_eq!(ring_aromatic, 6);
        assert!((1..7).all(|i| g.atom_ring_size[i as usize] == 6));
    }

    #[test]
    fn spiro_atom_smallest_ring_size_is_the_smaller_ring() {
        // Spiro[4.5] skeleton: a 5-ring and a 6-ring sharing one atom. The shared
        // (spiro) atom is in both; its smallest size is 5, and it reports both.
        let g = p("C1CCC2(CC1)CCCC2");
        // Find the spiro atom: degree 4, in a ring.
        let spiro = (0..g.atom_count()).find(|&i| g.degree[i] == 4);
        let spiro = spiro.expect("a spiro atom of degree 4 exists");
        assert!(g.in_ring(spiro as u32));
        assert!(g.in_ring_of_size(spiro as u32, 5));
        assert!(g.in_ring_of_size(spiro as u32, 6));
        assert_eq!(g.atom_ring_size[spiro], 5, "smallest ring is the 5-ring");
        // Not aromatic (all sp³).
        assert_eq!(n_aromatic_atoms(&g), 0);
    }

    #[test]
    fn pyridinone_exocyclic_carbonyl_keeps_ring_aromatic() {
        // Kekulé 2-pyridone: the carbonyl C donates 0 and the NH 2, so the ring is
        // aromatic; the exocyclic C=O is not a ring bond.
        let g = p("O=C1C=CC=CN1");
        // The carbonyl O is not in a ring and not aromatic.
        let o_idx = (0..g.atom_count())
            .find(|&i| g.element[i] == 8 && g.degree[i] == 1)
            .unwrap();
        assert!(!g.in_ring(o_idx as u32));
        assert!(!g.is_aromatic(o_idx as u32));
        // The six ring atoms (5 C + 1 N) are all aromatic.
        let ring_aromatic = (0..g.atom_count() as u32)
            .filter(|&i| g.in_ring(i) && g.is_aromatic(i))
            .count();
        assert_eq!(ring_aromatic, 6, "the pyridinone ring is aromatic");
        // The exocyclic C=O bond stays a double bond, not aromatic.
        let exo = g
            .bonds
            .iter()
            .find(|b| {
                (g.element[b.a as usize] == 8 || g.element[b.b as usize] == 8) && !b.in_ring()
            })
            .unwrap();
        assert_eq!(exo.order, bond_order::DOUBLE);
        assert!(!exo.is_aromatic());
    }

    #[test]
    fn sssr_count_matches_circuit_rank_on_fused_and_bridged() {
        // Each molecule's SSSR ring count must equal its circuit rank E - V + C.
        for s in [
            "c1ccccc1",
            "c1ccc2ccccc2c1",
            "C1CC2CCC1CC2",
            "C1CCC2(CC1)CCCC2",
        ] {
            let g = p(s);
            let rank = circuit_rank(&g);
            let rings = sssr(&g).unwrap();
            assert_eq!(
                rings.atom_rings.len(),
                rank,
                "SSSR ring count for {s:?} equals circuit rank {rank}"
            );
        }
    }

    // --- Perception invariants and spelling independence ---

    /// Molecules that exercise every perception path, plus adversarial cases:
    /// pyridine N-oxide, a 5/7 non-benzenoid fused system, and 2-pyridone (a
    /// lone-pair-donor N-H heteroaromatic).
    const PERCEPTION_GATE_SMILES: &[&str] = &[
        "c1ccccc1",
        "C1=CC=CC=C1",
        "c1ccncc1",
        "c1cc[nH]c1",
        "c1ccoc1",
        "c1ccsc1",
        "c1ccc2ccccc2c1",
        "c1ccc2[nH]ccc2c1",
        "C1CCCCC1",
        "C1=CCCCC1",
        "CC(=O)O",
        "Cc1ccccc1",
        // Charged aromatic-input ring: any bond perception rejects must not keep
        // AROMATIC order with the flag cleared.
        "c1cc[n+]([O-])cc1",
        // 5/7 fused non-benzenoid: the fusion bond is in no aromatic ring or
        // perimeter, so it must not keep its parser AROMATIC order.
        "c1ccc2cccc2cc1",
        // Lone-pair-donor N-H aromatic (2-pyridone).
        "O=c1cccc[nH]1",
    ];

    /// On every perceived graph, `order == AROMATIC ⇔ AROMATIC flag` holds and an
    /// aromatic bond connects two aromatic atoms.
    #[test]
    fn perceived_graphs_uphold_aromatic_order_flag_invariant() {
        for &s in PERCEPTION_GATE_SMILES {
            let g = p(s);
            for (bi, b) in g.bonds.iter().enumerate() {
                let order_aromatic = b.order == bond_order::AROMATIC;
                let flag_aromatic = b.is_aromatic();
                assert_eq!(
                    order_aromatic, flag_aromatic,
                    "{s:?} bond {bi} (atoms {}-{}): order==AROMATIC ({order_aromatic}) \
                     must equal is_aromatic() ({flag_aromatic})",
                    b.a, b.b
                );
                if flag_aromatic {
                    assert!(
                        g.is_aromatic(b.a) && g.is_aromatic(b.b),
                        "{s:?} bond {bi}: an aromatic bond must connect aromatic atoms"
                    );
                }
            }
        }
    }

    /// Positive control: cyclohexane stays aliphatic, so `CC` and `[#6][#6]` match.
    #[test]
    fn carbon_carbon_query_matches_genuinely_aliphatic_ring() {
        let g = p("C1CCCCC1"); // cyclohexane
        for q in ["CC", "[#6][#6]"] {
            let query = crate::smarts::parse(q).unwrap();
            assert!(
                crate::brute::matches(&query, &g),
                "{q:?} must match cyclohexane (aliphatic C-C ring adjacencies)"
            );
        }
    }

    #[test]
    fn pyridine_n_oxide_is_aromatic() {
        // Pyridine N-oxide and pyridinium are aromatic: the cationic N donates 1
        // π electron. So `cc` matches and aliphatic `CC` does not.
        for smi in [
            "c1cc[n+]([O-])cc1",
            "[O-][n+]1ccccc1",
            "c1cc[n+](C)cc1",
            "[nH+]1ccccc1",
        ] {
            let g = p(smi);
            assert!(
                crate::brute::matches(&crate::smarts::parse("cc").unwrap(), &g),
                "aromatic cc must match {smi:?}"
            );
            assert!(
                !crate::brute::matches(&crate::smarts::parse("CC").unwrap(), &g),
                "aliphatic CC must NOT match the aromatic ring {smi:?}"
            );
        }
    }

    /// Cyclohexene keeps its SINGLE/DOUBLE orders: the restore step turns only an
    /// AROMATIC order into SINGLE, never a real DOUBLE.
    #[test]
    fn kekule_double_survives_perception_guard() {
        let g = p("C1=CCCCC1");
        assert_eq!(n_aromatic_atoms(&g), 0, "cyclohexene is not aromatic");
        let doubles = g
            .bonds
            .iter()
            .filter(|b| b.order == bond_order::DOUBLE)
            .count();
        assert_eq!(doubles, 1, "the one C=C double bond must survive");
        assert!(
            g.bonds.iter().all(|b| b.order != bond_order::AROMATIC),
            "no bond should be aromatic"
        );
        assert!(g.bonds.iter().all(|b| !b.is_aromatic()));
    }

    /// Heteroatom H counts are as expected in both aromatic and Kekulé spellings.
    #[test]
    fn heteroaromatic_h_counts_agree_across_spellings() {
        // (description, aromatic spelling, kekulé spelling, target element,
        //  expected H on that heteroatom)
        let cases: &[(&str, &str, &str, u8, u8)] = &[
            ("pyrrole N", "c1cc[nH]c1", "C1=CC=CN1", 7, 1),
            ("pyridine N", "c1ccncc1", "C1=CC=NC=C1", 7, 0),
            ("furan O", "c1ccoc1", "C1=CC=CO1", 8, 0),
            ("thiophene S", "c1ccsc1", "C1=CC=CS1", 16, 0),
            ("2-pyridone NH", "O=c1cccc[nH]1", "O=C1C=CC=CN1", 7, 1),
        ];
        for &(desc, arom, kek, elem, expected_h) in cases {
            for (label, smi) in [("aromatic", arom), ("kekule", kek)] {
                let g = p(smi);
                // Each case has exactly one ring atom of `elem`.
                let idx = (0..g.atom_count())
                    .find(|&i| g.element[i] == elem && g.in_ring(i as u32))
                    .unwrap_or_else(|| panic!("{desc}: no ring atom of element {elem} in {smi:?}"));
                assert_eq!(
                    g.h_count[idx], expected_h,
                    "{desc} ({label} spelling {smi:?}): H count"
                );
            }
        }
    }

    /// For (aromatic, Kekulé) spellings of one molecule, probe SMARTS give the same
    /// brute-force answers on both perceived graphs. The exactness gates compare
    /// engines on one perceived graph, so they cannot catch this.
    #[test]
    fn same_molecule_spellings_answer_queries_identically() {
        // (aromatic spelling, kekulé spelling) of the SAME molecule.
        let pairs: &[(&str, &str)] = &[
            ("c1ccccc1", "C1=CC=CC=C1"),               // benzene
            ("c1ccncc1", "C1=CC=NC=C1"),               // pyridine
            ("c1cc[nH]c1", "C1=CC=CN1"),               // pyrrole
            ("c1ccoc1", "C1=CC=CO1"),                  // furan
            ("c1ccsc1", "C1=CC=CS1"),                  // thiophene
            ("O=c1cccc[nH]1", "O=C1C=CC=CN1"),         // 2-pyridone
            ("c1ccc2[nH]ccc2c1", "C1=CC2=CC=CC=C2N1"), // indole
            ("c1ccc2ccccc2c1", "C1=CC=C2C=CC=CC2=C1"), // naphthalene
        ];
        let probes = ["[nH]", "[cH]", "cc", "c:c", "CC", "[#6][#6]", "n", "o", "s"];
        for &(arom, kek) in pairs {
            let ga = p(arom);
            let gk = p(kek);
            for q in probes {
                let query = crate::smarts::parse(q).unwrap();
                let ma = crate::brute::matches(&query, &ga);
                let mk = crate::brute::matches(&query, &gk);
                assert_eq!(
                    ma, mk,
                    "query {q:?} disagrees across spellings of the same molecule \
                     ({arom:?} -> {ma}, {kek:?} -> {mk})"
                );
            }
        }
    }
}
