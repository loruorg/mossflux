//! The per-molecule **census**: a fixed-width histogram of atom and bond classes,
//! used as a cheap reject between the [`screen`](crate::screen) and the VF2
//! [`verify`](crate::vf2) step.
//!
//! Many screen survivors can be rejected by an `O(atoms)` counting check. The
//! census is that check: a few `u16` comparisons per candidate, run before any
//! backtracking.
//!
//! # Soundness
//!
//! For each census **class** we store, per target, how many of its atoms (or
//! bonds) fall in that class, and per query, a **lower bound** on how many a
//! matching target must have. The filter keeps a target iff, for every class,
//! `target_count(class) >= query_min(class)`. A query class's minimum is raised
//! only by a query atom/bond whose class is **forced**: its label or bond order is
//! fixed, with no `OR`/wildcard/negation that could let a match avoid the class
//! (the same notion as in [`crate::screen`]).
//!
//! > **No false negatives.** Let target `T` match query `Q` via an injective
//! > embedding `f`. Fix a class `c`. Every query atom forced into `c` maps to a
//! > target atom with the same label, so in class `c`. Because `f` is injective,
//! > `T` has at least `query_min(c)` atoms in class `c`. The same argument holds
//! > for bonds (a monomorphism maps distinct query bonds to distinct target bonds
//! > and keeps a forced order). So the filter keeps `T`. ∎
//!
//! Grouping several elements into one class only weakens the bound; it costs
//! selectivity, never soundness. A query that forces nothing yields an all-zero
//! minimum and the filter keeps every candidate.
//!
//! # Layout
//!
//! A [`Census`] is [`NUM_CLASSES`] `u16` counts, `Copy` and stack-sized. The index
//! stores them column-major ([`CensusColumns`]) so the filter reads one class
//! across all candidates as a contiguous `&[u16]` slice.

use crate::graph::{bond_order, AtomView, MolGraph};
use crate::query::{Pred, QueryGraph};
use crate::screen::{forced_atom_label, forced_bond_order};

/// Census class indices.
///
/// Atom classes key on the `(element, aromatic)` label of common elements, with one
/// catch-all for the rest. Bond classes key on order; single bonds are not counted
/// because they would never prune. Target and query sides agree because both call
/// [`atom_class`] / [`bond_class`].
mod class {
    /// Aliphatic carbon `C`.
    pub const C: usize = 0;
    /// Aromatic carbon `c`.
    pub const C_AROM: usize = 1;
    /// Aliphatic nitrogen `N`.
    pub const N: usize = 2;
    /// Aromatic nitrogen `n`.
    pub const N_AROM: usize = 3;
    /// Aliphatic oxygen `O`.
    pub const O: usize = 4;
    /// Aromatic oxygen `o`.
    pub const O_AROM: usize = 5;
    /// Aliphatic sulfur `S`.
    pub const S: usize = 6;
    /// Aromatic sulfur `s`.
    pub const S_AROM: usize = 7;
    /// Fluorine `F`.
    pub const F: usize = 8;
    /// Chlorine `Cl`.
    pub const CL: usize = 9;
    /// Bromine `Br`.
    pub const BR: usize = 10;
    /// Iodine `I`.
    pub const I: usize = 11;
    /// Phosphorus, either aromaticity.
    pub const P: usize = 12;
    /// Every other element (catch-all).
    pub const OTHER: usize = 13;

    /// Double-bonded edges (`=`).
    pub const BOND_DOUBLE: usize = 14;
    /// Triple-bonded edges (`#`).
    pub const BOND_TRIPLE: usize = 15;
    /// Aromatic edges (`:`).
    pub const BOND_AROM: usize = 16;

    // Property classes below overlap the element classes above. Each bound holds
    // on its own, so the overlap is sound.

    /// Any halogen (F, Cl, Br, I). Lets `[F,Cl,Br,I]` force a count even though no
    /// single element is forced.
    pub const ANY_HALOGEN: usize = 17;
    /// Aliphatic carbon of degree exactly 4 (`[CD4]`).
    pub const C_DEG4: usize = 18;
    /// Aliphatic carbon of degree exactly 3 (`[CD3]`).
    pub const C_DEG3: usize = 19;
    /// Ring nitrogen of either aromaticity (`[#7;R]`).
    pub const N_RING: usize = 20;
    /// Bond class: a double bond between aliphatic C and aliphatic O (`C=O`).
    pub const C_DBL_O: usize = 21;
    /// Amide carbon: an aliphatic C with a double-bonded aliphatic O and a
    /// single-or-aromatic-bonded aliphatic N (`C(=O)N`).
    pub const AMIDE_C: usize = 22;
    /// Carboxylic-acid carbon: an aliphatic C with a double-bonded aliphatic O and a
    /// single-or-aromatic-bonded aliphatic O with exactly one H (`C(=O)[OH]`).
    pub const ACID_C: usize = 23;
    /// Bond class: a bond whose two endpoints are both ring atoms (`[R]~[R]`).
    /// Not the same as a ring bond: biphenyl's pivot bond counts here.
    pub const RING_RING_BOND: usize = 24;
    /// Ester carbon: an aliphatic C with a double-bonded aliphatic O and a
    /// single-or-aromatic-bonded aliphatic O that has a second aliphatic-C
    /// neighbour (`C(=O)OC`).
    pub const ESTER_C: usize = 25;
    /// Phenol carbon: a benzene-ring carbon (see [`MolGraph::carbocyclic_arom6`])
    /// with an exocyclic single-or-aromatic-bonded aliphatic O.
    ///
    /// **Early-accept only.** It is SSSR-derived, so `PHENOL_C >= 1` implies a match
    /// but not the reverse. [`query_min_census`] never bumps it, so it never rejects
    /// a candidate. See [`census_early_accept_class`].
    pub const PHENOL_C: usize = 26;
    /// Aromatic carbon with at least 3 aromatic-carbon neighbours (a fusion carbon
    /// of an all-carbon fused system, or a biaryl pivot). A local count, so it is a
    /// sound filter class.
    pub const FUSION_AROM_C: usize = 27;
    /// Bond in at least 2 distinct 6-cycles of aromatic carbons (the fusion bond of
    /// naphthalene). Carbazole, fluorene and dibenzofuran have none. Counted by
    /// direct 6-cycle enumeration (not SSSR), so it never under-counts and is a
    /// sound filter class.
    pub const FUSED_BENZENE_PAIR: usize = 28;
    /// Bond class: a triple bond between aliphatic C and aliphatic N (`C#N`).
    pub const C_TRP_N: usize = 29;
    /// Sulfonamide sulfur: an aliphatic S with at least 2 double-bonded aliphatic O
    /// and at least 1 single-or-aromatic-bonded aliphatic N (`S(=O)(=O)N`).
    pub const SULFONAMIDE_S: usize = 30;
    /// Aliphatic nitrogen of degree exactly 3 (`[ND3]`).
    pub const N_DEG3: usize = 31;
}

/// Number of census classes (atom classes + counted bond classes).
pub const NUM_CLASSES: usize = 32;

/// Atomic numbers used by [`atom_class`].
mod z {
    pub const CARBON: u8 = 6;
    pub const NITROGEN: u8 = 7;
    pub const OXYGEN: u8 = 8;
    pub const FLUORINE: u8 = 9;
    pub const PHOSPHORUS: u8 = 15;
    pub const SULFUR: u8 = 16;
    pub const CHLORINE: u8 = 17;
    pub const BROMINE: u8 = 35;
    pub const IODINE: u8 = 53;
}

/// Maps a concrete `(element, aromatic)` atom label to its census class.
///
/// Target and query sides both call this (on the real and the forced label), so
/// identical labels always land in the same class.
#[inline]
fn atom_class(element: u8, aromatic: bool) -> usize {
    match (element, aromatic) {
        (z::CARBON, false) => class::C,
        (z::CARBON, true) => class::C_AROM,
        (z::NITROGEN, false) => class::N,
        (z::NITROGEN, true) => class::N_AROM,
        (z::OXYGEN, false) => class::O,
        (z::OXYGEN, true) => class::O_AROM,
        (z::SULFUR, false) => class::S,
        (z::SULFUR, true) => class::S_AROM,
        (z::FLUORINE, _) => class::F,
        (z::CHLORINE, _) => class::CL,
        (z::BROMINE, _) => class::BR,
        (z::IODINE, _) => class::I,
        (z::PHOSPHORUS, _) => class::P,
        _ => class::OTHER,
    }
}

/// The four halogen atomic numbers grouped into [`class::ANY_HALOGEN`].
const HALOGEN_Z: [u8; 4] = [z::FLUORINE, z::CHLORINE, z::BROMINE, z::IODINE];

/// True iff `n` is one of the grouped halogens.
#[inline]
fn is_halogen(n: u8) -> bool {
    HALOGEN_Z.contains(&n)
}

/// True iff every atom satisfying `pred` is provably a halogen, so the query atom
/// forces [`class::ANY_HALOGEN`].
///
/// Conservative: a wrong `true` could drop a true hit.
/// * A halogen element or atomic number forces it.
/// * An `AND` forces it if either branch does.
/// * An `OR` forces it only if both branches do.
/// * Anything else, including any negation, returns `false`.
fn forces_halogen(pred: &Pred) -> bool {
    match pred {
        Pred::Element(n) | Pred::AromaticElement(n) | Pred::AtomicNumber(n) => is_halogen(*n),
        Pred::And(l, r) => forces_halogen(l) || forces_halogen(r),
        Pred::Or(l, r) => forces_halogen(l) && forces_halogen(r),
        _ => false,
    }
}

/// The degree `d` that `pred` forces, if any: every satisfying atom has degree `d`.
///
/// Only `D<n>` leaves and `AND`s over them force a degree. `OR`, negation and other
/// predicates yield `None`. A contradictory `AND` of two degrees also yields `None`
/// (it matches no atom, so no count is needed).
fn forced_degree(pred: &Pred) -> Option<u8> {
    match pred {
        Pred::Degree(d) => Some(*d),
        Pred::And(l, r) => match (forced_degree(l), forced_degree(r)) {
            (Some(a), Some(b)) => (a == b).then_some(a),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        },
        _ => None,
    }
}

/// The atomic number that `pred` forces, if any, across both aromaticities.
/// `OR`, `NOT`, other predicates and contradictory `AND`s yield `None`.
fn forced_atomic_number(pred: &Pred) -> Option<u8> {
    match pred {
        Pred::Element(n) | Pred::AromaticElement(n) | Pred::AtomicNumber(n) => Some(*n),
        Pred::And(l, r) => match (forced_atomic_number(l), forced_atomic_number(r)) {
            (Some(a), Some(b)) => (a == b).then_some(a),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        },
        _ => None,
    }
}

/// True iff `pred` forces ring membership (`R`, or an `AND` containing it).
fn forces_in_ring(pred: &Pred) -> bool {
    match pred {
        Pred::InRing => true,
        Pred::And(l, r) => forces_in_ring(l) || forces_in_ring(r),
        _ => false,
    }
}

/// Maps a bond order to its census class, or `None` if the order is not counted.
#[inline]
fn bond_class(order: u8) -> Option<usize> {
    match order {
        bond_order::DOUBLE => Some(class::BOND_DOUBLE),
        bond_order::TRIPLE => Some(class::BOND_TRIPLE),
        bond_order::AROMATIC => Some(class::BOND_AROM),
        _ => None,
    }
}

/// A fixed-width per-molecule histogram: [`NUM_CLASSES`] saturating `u16` counts.
///
/// Saturation keeps the `>=` filter sound: a saturated target count is still `>=`
/// any query minimum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Census {
    counts: [u16; NUM_CLASSES],
}

impl Default for Census {
    fn default() -> Self {
        Census {
            counts: [0; NUM_CLASSES],
        }
    }
}

impl Census {
    /// An all-zero census.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// The count in class `c`.
    #[inline]
    pub fn count(&self, c: usize) -> u16 {
        self.counts[c]
    }

    /// Adds one to class `c`, saturating at `u16::MAX`.
    #[inline]
    fn bump(&mut self, c: usize) {
        self.counts[c] = self.counts[c].saturating_add(1);
    }

    /// True iff `self[c] >= min[c]` for every class `c`.
    ///
    /// A target that does not dominate the query minimum cannot match.
    #[inline]
    pub fn dominates(&self, min: &Census) -> bool {
        // Branchless accumulate so the fixed-length loop vectorises.
        let mut deficit = 0u32;
        for (s, m) in self.counts.iter().zip(min.counts.iter()) {
            deficit |= (*m > *s) as u32;
        }
        deficit == 0
    }
}

/// Counts bonds that lie in at least 2 distinct 6-cycles of `eligible` atoms.
///
/// Enumerates 6-cycles directly by DFS. Each undirected cycle is counted once
/// (start is its minimum atom and `path[1] < path[5]`). Direct enumeration (not
/// SSSR) never under-counts, so the result is a sound filter lower bound.
/// `adj[v]` lists `(neighbour, bond id)`.
fn fused_six_cycle_shared_bonds(
    n: usize,
    bond_count: usize,
    adj: &[Vec<(u32, u32)>],
    eligible: &[bool],
) -> u16 {
    // A fused pair needs ten distinct atoms.
    if eligible.iter().filter(|&&e| e).count() < 10 {
        return 0;
    }
    let mut tally = vec![0u16; bond_count];
    let mut path: Vec<u32> = Vec::with_capacity(6);
    let mut pbonds: Vec<u32> = Vec::with_capacity(6);
    let mut on = vec![false; n];

    // A local fn, not a closure, because it borrows several mutable buffers.
    #[allow(clippy::too_many_arguments)]
    fn walk(
        start: u32,
        last: u32,
        adj: &[Vec<(u32, u32)>],
        eligible: &[bool],
        path: &mut Vec<u32>,
        pbonds: &mut Vec<u32>,
        on: &mut [bool],
        tally: &mut [u16],
    ) {
        if path.len() == 6 {
            // Try to close the 6-walk back to `start`.
            for &(nb, b) in &adj[last as usize] {
                if nb == start {
                    let is_min = path.iter().all(|&p| p >= start);
                    if is_min && path[1] < path[5] {
                        for &pb in pbonds.iter() {
                            tally[pb as usize] = tally[pb as usize].saturating_add(1);
                        }
                        tally[b as usize] = tally[b as usize].saturating_add(1);
                    }
                    break;
                }
            }
            return;
        }
        for &(nb, b) in &adj[last as usize] {
            if on[nb as usize] || !eligible[nb as usize] {
                continue;
            }
            on[nb as usize] = true;
            path.push(nb);
            pbonds.push(b);
            walk(start, nb, adj, eligible, path, pbonds, on, tally);
            pbonds.pop();
            path.pop();
            on[nb as usize] = false;
        }
    }

    for v in 0..n as u32 {
        if !eligible[v as usize] {
            continue;
        }
        on[v as usize] = true;
        path.push(v);
        walk(
            v,
            v,
            adj,
            eligible,
            &mut path,
            &mut pbonds,
            &mut on,
            &mut tally,
        );
        path.pop();
        on[v as usize] = false;
    }
    // Saturate: a wrapped count could read 0 and reject a true match.
    let pairs = tally.iter().filter(|&&t| t >= 2).count();
    u16::try_from(pairs).unwrap_or(u16::MAX)
}

/// Target `FUSED_BENZENE_PAIR` count: aromatic carbons are eligible.
fn target_fused_benzene_pair(mol: &MolGraph) -> u16 {
    let n = mol.atom_count();
    // A fused pair needs ten aromatic carbons; check before allocating scratch.
    let arom_c = (0..n as u32)
        .filter(|&a| mol.element[a as usize] == z::CARBON && mol.is_aromatic(a))
        .count();
    if arom_c < 10 {
        return 0;
    }
    let eligible: Vec<bool> = (0..n as u32)
        .map(|a| mol.element[a as usize] == z::CARBON && mol.is_aromatic(a))
        .collect();
    let adj: Vec<Vec<(u32, u32)>> = (0..n as u32)
        .map(|a| {
            mol.neighbors(a)
                .iter()
                .map(|nb| (nb.atom, nb.bond))
                .collect()
        })
        .collect();
    fused_six_cycle_shared_bonds(n, mol.bond_count(), &adj, &eligible)
}

/// Query `FUSED_BENZENE_PAIR` lower bound: atoms that force an aromatic carbon are
/// eligible. A match maps each counted cycle to an aromatic-carbon 6-cycle in the
/// target, so a match has at least this many fused-pair bonds.
fn query_fused_benzene_pair(query: &QueryGraph) -> u16 {
    let n = query.atom_count();
    // Same gate as the target side.
    let forced_arom_c = query
        .atoms
        .iter()
        .filter(|p| forced_atom_label(p) == Some((z::CARBON, true)))
        .count();
    if forced_arom_c < 10 {
        return 0;
    }
    let eligible: Vec<bool> = query
        .atoms
        .iter()
        .map(|p| forced_atom_label(p) == Some((z::CARBON, true)))
        .collect();
    let adj: Vec<Vec<(u32, u32)>> = (0..n)
        .map(|i| {
            query.incident[i]
                .iter()
                .map(|&bi| {
                    let b = &query.bonds[bi as usize];
                    (if b.a as usize == i { b.b } else { b.a }, bi)
                })
                .collect()
        })
        .collect();
    fused_six_cycle_shared_bonds(n, query.bonds.len(), &adj, &eligible)
}

/// Builds the census of a perceived target molecule.
pub fn target_census(mol: &MolGraph) -> Census {
    let mut c = Census::new();
    for a in 0..mol.atom_count() as u32 {
        let idx = a as usize;
        let element = mol.element[idx];
        let aromatic = mol.is_aromatic(a);
        c.bump(atom_class(element, aromatic));
        // Property classes (an atom may bump several).
        if is_halogen(element) {
            c.bump(class::ANY_HALOGEN);
        }
        if element == z::CARBON && !aromatic && mol.degree[idx] == 4 {
            c.bump(class::C_DEG4);
        }
        if element == z::CARBON && !aromatic && mol.degree[idx] == 3 {
            c.bump(class::C_DEG3);
        }
        if element == z::NITROGEN && mol.in_ring(a) {
            c.bump(class::N_RING);
        }
        if element == z::NITROGEN && !aromatic && mol.degree[idx] == 3 {
            c.bump(class::N_DEG3);
        }
        if element == z::CARBON && aromatic {
            let mut arom_c_nbrs = 0u32;
            for nb in mol.neighbors(a) {
                if mol.element[nb.atom as usize] == z::CARBON && mol.is_aromatic(nb.atom) {
                    arom_c_nbrs += 1;
                }
            }
            if arom_c_nbrs >= 3 {
                c.bump(class::FUSION_AROM_C);
            }
        }
        // Phenol carbon. The benzene flag is empty on un-perceived graphs, so
        // `.get(..)` makes this a no-op there.
        if element == z::CARBON
            && aromatic
            && mol.carbocyclic_arom6.get(idx).copied().unwrap_or(false)
        {
            for nb in mol.neighbors(a) {
                let bond = &mol.bonds[nb.bond as usize];
                let ni = nb.atom as usize;
                if (bond.order == bond_order::SINGLE || bond.is_aromatic())
                    && mol.element[ni] == z::OXYGEN
                    && !mol.is_aromatic(nb.atom)
                {
                    c.bump(class::PHENOL_C);
                    break;
                }
            }
        }
        // Acyl classes, from an aliphatic carbon's neighbourhood.
        if element == z::CARBON && !aromatic {
            let mut dbl_o = false; // double bond to aliphatic O
            let mut sgl_n = false; // single-or-aromatic bond to aliphatic N
            let mut sgl_oh = false; // single bond to aliphatic O carrying exactly 1 H
            let mut sgl_o_c = false; // single-bonded aliphatic O bearing a 2nd aliphatic C
            for nb in mol.neighbors(a) {
                let bond = &mol.bonds[nb.bond as usize];
                let ni = nb.atom as usize;
                let ne = mol.element[ni];
                let nar = mol.is_aromatic(nb.atom);
                if nar {
                    continue;
                }
                if bond.order == bond_order::DOUBLE && ne == z::OXYGEN {
                    dbl_o = true;
                }
                let single_or_arom = bond.order == bond_order::SINGLE || bond.is_aromatic();
                if single_or_arom && ne == z::NITROGEN {
                    sgl_n = true;
                }
                // Total H (implicit + bonded `[H]`), to match the query `[OH]` = TotalH(1).
                if single_or_arom
                    && ne == z::OXYGEN
                    && mol.h_count[ni] as u32 + mol.av_explicit_h(nb.atom) as u32 == 1
                {
                    sgl_oh = true;
                }
                // Ester bridging O: has a second aliphatic-carbon neighbour.
                if single_or_arom && ne == z::OXYGEN {
                    for nb2 in mol.neighbors(nb.atom) {
                        if nb2.atom == a {
                            continue;
                        }
                        let bond2 = &mol.bonds[nb2.bond as usize];
                        let nj = nb2.atom as usize;
                        if (bond2.order == bond_order::SINGLE || bond2.is_aromatic())
                            && mol.element[nj] == z::CARBON
                            && !mol.is_aromatic(nb2.atom)
                        {
                            sgl_o_c = true;
                        }
                    }
                }
            }
            if dbl_o && sgl_n {
                c.bump(class::AMIDE_C);
            }
            if dbl_o && sgl_oh {
                c.bump(class::ACID_C);
            }
            if dbl_o && sgl_o_c {
                c.bump(class::ESTER_C);
            }
        }
        // Sulfonamide class.
        if element == z::SULFUR && !aromatic {
            let mut dbl_o = 0u32; // double-bonded aliphatic O count
            let mut sgl_n = false; // single-or-aromatic bond to aliphatic N
            for nb in mol.neighbors(a) {
                let bond = &mol.bonds[nb.bond as usize];
                let ni = nb.atom as usize;
                let ne = mol.element[ni];
                if mol.is_aromatic(nb.atom) {
                    continue;
                }
                if bond.order == bond_order::DOUBLE && ne == z::OXYGEN {
                    dbl_o += 1;
                }
                if (bond.order == bond_order::SINGLE || bond.is_aromatic()) && ne == z::NITROGEN {
                    sgl_n = true;
                }
            }
            if dbl_o >= 2 && sgl_n {
                c.bump(class::SULFONAMIDE_S);
            }
        }
    }
    for bond in &mol.bonds {
        if let Some(cls) = bond_class(bond.order) {
            c.bump(cls);
        }
        // Carbonyl bond.
        if bond.order == bond_order::DOUBLE {
            let ea = mol.element[bond.a as usize];
            let eb = mol.element[bond.b as usize];
            let aa = mol.is_aromatic(bond.a);
            let ab = mol.is_aromatic(bond.b);
            let is_c_o = (ea == z::CARBON && !aa && eb == z::OXYGEN && !ab)
                || (eb == z::CARBON && !ab && ea == z::OXYGEN && !aa);
            if is_c_o {
                c.bump(class::C_DBL_O);
            }
        }
        // Nitrile bond.
        if bond.order == bond_order::TRIPLE {
            let ea = mol.element[bond.a as usize];
            let eb = mol.element[bond.b as usize];
            let aa = mol.is_aromatic(bond.a);
            let ab = mol.is_aromatic(bond.b);
            let is_c_n = (ea == z::CARBON && !aa && eb == z::NITROGEN && !ab)
                || (eb == z::CARBON && !ab && ea == z::NITROGEN && !aa);
            if is_c_n {
                c.bump(class::C_TRP_N);
            }
        }
        if mol.in_ring(bond.a) && mol.in_ring(bond.b) {
            c.bump(class::RING_RING_BOND);
        }
    }
    // A whole-molecule count, set directly.
    c.counts[class::FUSED_BENZENE_PAIR] = target_fused_benzene_pair(mol);
    c
}

/// Builds the query's minimum census: for each class, a lower bound on how many
/// atoms or bonds of that class any matching target must have.
///
/// A class is bumped only for a query atom or bond that forces it, so each count is
/// a valid lower bound (see the [module docs](self)). Unforced atoms and bonds
/// contribute nothing.
pub fn query_min_census(query: &QueryGraph) -> Census {
    let mut c = Census::new();
    let forces_arom_c = |p: &Pred| forced_atom_label(p) == Some((z::CARBON, true));
    for (i, pred) in query.atoms.iter().enumerate() {
        let label = forced_atom_label(pred);
        if let Some((e, a)) = label {
            c.bump(atom_class(e, a));
        }
        // Property classes: bump only when the predicate provably forces them.
        if forces_halogen(pred) {
            c.bump(class::ANY_HALOGEN);
        }
        if label == Some((z::CARBON, false)) && forced_degree(pred) == Some(4) {
            c.bump(class::C_DEG4);
        }
        if label == Some((z::CARBON, false)) && forced_degree(pred) == Some(3) {
            c.bump(class::C_DEG3);
        }
        if label == Some((z::NITROGEN, false)) && forced_degree(pred) == Some(3) {
            c.bump(class::N_DEG3);
        }
        if forced_atomic_number(pred) == Some(z::NITROGEN) && forces_in_ring(pred) {
            c.bump(class::N_RING);
        }
        // Fusion carbon: forces an aromatic carbon and at least 3 query neighbours
        // also do. Its image then has at least those 3 aromatic-C neighbours.
        if forces_arom_c(pred) {
            let arom_c_nbrs = query.incident[i]
                .iter()
                .filter(|&&bi| {
                    let b = &query.bonds[bi as usize];
                    let other = if b.a as usize == i { b.b } else { b.a };
                    forces_arom_c(&query.atoms[other as usize])
                })
                .count();
            if arom_c_nbrs >= 3 {
                c.bump(class::FUSION_AROM_C);
            }
        }
    }
    for bond in &query.bonds {
        if let Some(order) = forced_bond_order(&bond.pred) {
            if let Some(cls) = bond_class(order) {
                c.bump(cls);
            }
            // Carbonyl: forced double bond whose ends force aliphatic C and O.
            if order == bond_order::DOUBLE {
                let la = forced_atom_label(&query.atoms[bond.a as usize]);
                let lb = forced_atom_label(&query.atoms[bond.b as usize]);
                let c_o = (la == Some((z::CARBON, false)) && lb == Some((z::OXYGEN, false)))
                    || (lb == Some((z::CARBON, false)) && la == Some((z::OXYGEN, false)));
                if c_o {
                    c.bump(class::C_DBL_O);
                }
            }
            // Nitrile: forced triple bond whose ends force aliphatic C and N.
            if order == bond_order::TRIPLE {
                let la = forced_atom_label(&query.atoms[bond.a as usize]);
                let lb = forced_atom_label(&query.atoms[bond.b as usize]);
                let c_n = (la == Some((z::CARBON, false)) && lb == Some((z::NITROGEN, false)))
                    || (lb == Some((z::CARBON, false)) && la == Some((z::NITROGEN, false)));
                if c_n {
                    c.bump(class::C_TRP_N);
                }
            }
        }
        // Ring–ring bond: both endpoints forced in-ring.
        if forces_in_ring(&query.atoms[bond.a as usize])
            && forces_in_ring(&query.atoms[bond.b as usize])
        {
            c.bump(class::RING_RING_BOND);
        }
    }
    // Group classes: a target matching the group has its centre atom in the class.
    if is_amide_query(query) {
        c.bump(class::AMIDE_C);
    }
    if is_acid_query(query) {
        c.bump(class::ACID_C);
    }
    if is_ester_query(query) {
        c.bump(class::ESTER_C);
    }
    if is_sulfonamide_query(query) {
        c.bump(class::SULFONAMIDE_S);
    }
    // Set directly.
    c.counts[class::FUSED_BENZENE_PAIR] = query_fused_benzene_pair(query);
    c
}

/// Flattens a pure conjunction of leaf predicates to its leaves, or `None` if it
/// contains an `OR`, `NOT` or recursive `$(...)`.
fn conjunction_leaves(pred: &Pred) -> Option<Vec<&Pred>> {
    match pred {
        Pred::And(l, r) => {
            let mut out = conjunction_leaves(l)?;
            out.extend(conjunction_leaves(r)?);
            Some(out)
        }
        Pred::Or(..) | Pred::Not(..) | Pred::Recursive(..) => None,
        leaf => Some(vec![leaf]),
    }
}

/// True iff `leaves` is exactly `expected` as a multiset (same leaves, any order).
fn leaves_equal(leaves: &[&Pred], expected: &[Pred]) -> bool {
    if leaves.len() != expected.len() {
        return false;
    }
    let mut used = vec![false; expected.len()];
    for l in leaves {
        match expected
            .iter()
            .enumerate()
            .find(|(i, e)| !used[*i] && *e == *l)
        {
            Some((i, _)) => used[i] = true,
            None => return false,
        }
    }
    true
}

/// True iff `pred` is exactly the disjunction `[F,Cl,Br,I]`.
fn is_exact_four_halogen(pred: &Pred) -> bool {
    fn or_leaves<'a>(p: &'a Pred, out: &mut Vec<&'a Pred>) -> bool {
        match p {
            Pred::Or(l, r) => or_leaves(l, out) && or_leaves(r, out),
            Pred::And(..) | Pred::Not(..) | Pred::Recursive(..) => false,
            leaf => {
                out.push(leaf);
                true
            }
        }
    }
    let mut leaves = Vec::new();
    if !or_leaves(pred, &mut leaves) {
        return false;
    }
    leaves_equal(
        &leaves,
        &[
            Pred::Element(z::FLUORINE),
            Pred::Element(z::CHLORINE),
            Pred::Element(z::BROMINE),
            Pred::Element(z::IODINE),
        ],
    )
}

/// For a 3-atom, 2-bond query with one centre atom, returns
/// `(centre_idx, [(bond_pred, leaf_idx); 2])`.
fn three_atom_star(query: &QueryGraph) -> Option<(u32, [(&crate::query::BondPred, u32); 2])> {
    if query.atom_count() != 3 || query.bonds.len() != 2 {
        return None;
    }
    let mut inc = [0u8; 3];
    for b in &query.bonds {
        inc[b.a as usize] += 1;
        inc[b.b as usize] += 1;
    }
    let center = (0..3u32).find(|&a| inc[a as usize] == 2)?;
    let leaf = |b: &crate::query::QueryBond| if b.a == center { b.b } else { b.a };
    let b0 = &query.bonds[0];
    let b1 = &query.bonds[1];
    Some((center, [(&b0.pred, leaf(b0)), (&b1.pred, leaf(b1))]))
}

/// `pred` is exactly an oxygen carrying one hydrogen (`[OH]` = `Element(8) & H1`).
fn is_oxygen_h1(pred: &Pred) -> bool {
    conjunction_leaves(pred)
        .is_some_and(|leaves| leaves_equal(&leaves, &[Pred::Element(z::OXYGEN), Pred::TotalH(1)]))
}

/// `query` is exactly the amide `C(=O)N` with default bonds except the `=`.
fn is_amide_query(query: &QueryGraph) -> bool {
    use crate::query::BondPred;
    let Some((center, leaves)) = three_atom_star(query) else {
        return false;
    };
    if !matches!(query.atoms[center as usize], Pred::Element(z::CARBON)) {
        return false;
    }
    let (mut dbl_o, mut san_n) = (false, false);
    for (bp, leaf) in leaves {
        let lp = &query.atoms[leaf as usize];
        match bp {
            BondPred::Order(o) if *o == bond_order::DOUBLE => {
                dbl_o |= matches!(lp, Pred::Element(z::OXYGEN));
            }
            BondPred::SingleOrAromatic => {
                san_n |= matches!(lp, Pred::Element(z::NITROGEN));
            }
            _ => {}
        }
    }
    dbl_o && san_n
}

/// `query` is exactly the carboxylic acid `C(=O)[OH]`.
fn is_acid_query(query: &QueryGraph) -> bool {
    use crate::query::BondPred;
    let Some((center, leaves)) = three_atom_star(query) else {
        return false;
    };
    if !matches!(query.atoms[center as usize], Pred::Element(z::CARBON)) {
        return false;
    }
    let (mut dbl_o, mut oh) = (false, false);
    for (bp, leaf) in leaves {
        let lp = &query.atoms[leaf as usize];
        match bp {
            BondPred::Order(o) if *o == bond_order::DOUBLE => {
                dbl_o |= matches!(lp, Pred::Element(z::OXYGEN));
            }
            BondPred::SingleOrAromatic => oh |= is_oxygen_h1(lp),
            _ => {}
        }
    }
    dbl_o && oh
}

/// `query` is exactly the ester `C(=O)OC` with default bonds except the `=`.
fn is_ester_query(query: &QueryGraph) -> bool {
    use crate::query::BondPred;
    if query.atom_count() != 4 || query.bonds.len() != 3 {
        return false;
    }
    let is_c = |i: u32| matches!(query.atoms[i as usize], Pred::Element(z::CARBON));
    let is_o = |i: u32| matches!(query.atoms[i as usize], Pred::Element(z::OXYGEN));
    // Incident bonds of atom `i` as (bond predicate, the other endpoint).
    let nbrs = |i: u32| -> Vec<(&BondPred, u32)> {
        query.incident[i as usize]
            .iter()
            .map(|&bi| {
                let b = &query.bonds[bi as usize];
                (&b.pred, if b.a == i { b.b } else { b.a })
            })
            .collect()
    };
    for center in 0..4u32 {
        if !is_c(center) {
            continue;
        }
        let cn = nbrs(center);
        if cn.len() != 2 {
            continue;
        }
        let mut bridge_o = None;
        let mut has_dbl_o = false;
        for (bp, other) in &cn {
            match bp {
                BondPred::Order(o) if *o == bond_order::DOUBLE => has_dbl_o |= is_o(*other),
                BondPred::SingleOrAromatic if is_o(*other) => bridge_o = Some(*other),
                _ => {}
            }
        }
        let (true, Some(ob)) = (has_dbl_o, bridge_o) else {
            continue;
        };
        // The bridging O needs a second aliphatic-carbon neighbour (≠ centre).
        for (bp, other) in nbrs(ob) {
            if other != center && matches!(bp, BondPred::SingleOrAromatic) && is_c(other) {
                return true;
            }
        }
    }
    false
}

/// `query` is exactly the sulfonamide `S(=O)(=O)N` and nothing else.
///
/// Must be exact: accepting e.g. `S(=O)(=O)O` would make the census-exact path
/// return wrong matches.
fn is_sulfonamide_query(query: &QueryGraph) -> bool {
    use crate::query::BondPred;
    if query.atom_count() != 4 || query.bonds.len() != 3 {
        return false;
    }
    let is_s = |i: u32| matches!(query.atoms[i as usize], Pred::Element(z::SULFUR));
    let is_o = |i: u32| matches!(query.atoms[i as usize], Pred::Element(z::OXYGEN));
    let is_n = |i: u32| matches!(query.atoms[i as usize], Pred::Element(z::NITROGEN));
    let nbrs = |i: u32| -> Vec<(&BondPred, u32)> {
        query.incident[i as usize]
            .iter()
            .map(|&bi| {
                let b = &query.bonds[bi as usize];
                (&b.pred, if b.a == i { b.b } else { b.a })
            })
            .collect()
    };
    for center in 0..4u32 {
        if !is_s(center) {
            continue;
        }
        let cn = nbrs(center);
        if cn.len() != 3 {
            continue;
        }
        let (mut dbl_o, mut san_n, mut other) = (0u32, 0u32, 0u32);
        for (bp, leaf) in &cn {
            match bp {
                BondPred::Order(o) if *o == bond_order::DOUBLE && is_o(*leaf) => dbl_o += 1,
                BondPred::SingleOrAromatic if is_n(*leaf) => san_n += 1,
                _ => other += 1,
            }
        }
        if dbl_o == 2 && san_n == 1 && other == 0 {
            return true;
        }
    }
    false
}

/// `query` is exactly the phenol `c1ccc(O)cc1` with default bonds.
///
/// Must be exact: a looser match would early-accept candidates of another query.
fn is_phenol_query(query: &QueryGraph) -> bool {
    use crate::query::BondPred;
    if query.atom_count() != 7 || query.bonds.len() != 7 {
        return false;
    }
    // Every bond must be a default bond.
    if !query
        .bonds
        .iter()
        .all(|b| matches!(b.pred, BondPred::SingleOrAromatic))
    {
        return false;
    }
    let deg = |i: u32| query.incident[i as usize].len();
    let other_end = |bi: u32, i: u32| {
        let b = &query.bonds[bi as usize];
        if b.a == i {
            b.b
        } else {
            b.a
        }
    };
    // Exactly one substituent oxygen: a degree-1 aliphatic O.
    let mut o_atom = None;
    for i in 0..7u32 {
        if matches!(query.atoms[i as usize], Pred::Element(z::OXYGEN)) {
            if deg(i) != 1 {
                return false;
            }
            if o_atom.is_some() {
                return false;
            }
            o_atom = Some(i);
        }
    }
    let Some(o) = o_atom else {
        return false;
    };
    let anchor = other_end(query.incident[o as usize][0], o);
    // The other six atoms are bare aromatic carbons of degree 2 (the anchor: 3).
    for i in (0..7u32).filter(|&i| i != o) {
        if !matches!(query.atoms[i as usize], Pred::AromaticElement(z::CARBON)) {
            return false;
        }
        let expect = if i == anchor { 3 } else { 2 };
        if deg(i) != expect {
            return false;
        }
    }
    // Walk the ring to confirm one simple 6-cycle (two triangles also pass the
    // degree test).
    let mut order = Vec::with_capacity(6);
    let (mut cur, mut prev) = (anchor, o);
    loop {
        order.push(cur);
        let mut next = None;
        for &bi in &query.incident[cur as usize] {
            let nb = other_end(bi, cur);
            if nb != prev && nb != o {
                next = Some(nb);
                break;
            }
        }
        let Some(nb) = next else {
            return false;
        };
        if nb == anchor {
            break;
        }
        if order.len() >= 6 {
            return false;
        }
        prev = cur;
        cur = nb;
    }
    order.len() == 6
}

/// The census class whose count `>= 1` guarantees a match for `query`, or `None`.
///
/// Unlike [`is_census_exact`], this holds in one direction only: candidates with a
/// zero count still go through VF2. Used for ring patterns whose reverse direction
/// would depend on SSSR (phenol).
pub fn census_early_accept_class(query: &QueryGraph) -> Option<usize> {
    if is_phenol_query(query) {
        return Some(class::PHENOL_C);
    }
    None
}

/// True iff the census class of a bare `(element, aromatic)` query atom holds
/// exactly the target atoms with that label, so `class count >= 1` is equivalent
/// to "the target has such an atom".
///
/// * C, N, O and S have one class per aromaticity: exact for both labels.
/// * F, Cl, Br and I share one class across aromaticity. It is exact for the
///   aliphatic label, because no indexed target has an aromatic halogen: the
///   SMILES parser has no aromatic halogen symbol, and perception re-derives every
///   atom's aromatic flag and gives a p-orbital only to B, C, N, O, P and S
///   (`perception::pi_contribution`).
/// * P shares one class across aromatic and aliphatic P, which both occur
///   (`c1ccpcc1`), and every other element shares [`class::OTHER`]. Neither is
///   exact: `[Zn]` would return every molecule with any `OTHER` atom.
fn bare_element_class_is_exact(element: u8, aromatic: bool) -> bool {
    matches!(
        (element, aromatic),
        (z::CARBON | z::NITROGEN | z::OXYGEN | z::SULFUR, _)
            | (z::FLUORINE | z::CHLORINE | z::BROMINE | z::IODINE, false)
    )
}

/// True iff the [census filter](CensusColumns::filter) computes the match set of
/// `query` exactly, so the matcher may return the census survivors and skip VF2.
///
/// Each accepted query is matched by a target iff the target has at least one
/// member of one census class: a bare element, `[CD3]`, `[CD4]`, `[ND3]`,
/// `[#7;R]`, `[F,Cl,Br,I]`, `C=O`, `C#N`, `[R]~[R]`, and the amide, acid, ester and
/// sulfonamide groups. Any other query returns `false` and is verified normally.
/// The differential fuzz test checks this against the brute-force oracle.
pub fn is_census_exact(query: &QueryGraph) -> bool {
    // Single-atom queries.
    if query.atom_count() == 1 && query.bonds.is_empty() {
        let p = &query.atoms[0];
        // A bare element is exact only when its class holds no other label.
        match *p {
            Pred::Element(n) => return bare_element_class_is_exact(n, false),
            Pred::AromaticElement(n) => return bare_element_class_is_exact(n, true),
            _ => {}
        }
        if is_exact_four_halogen(p) {
            return true;
        }
        // Exactly [CD3] / [CD4] / [ND3] / [#7;R].
        if let Some(leaves) = conjunction_leaves(p) {
            return leaves_equal(&leaves, &[Pred::Element(z::CARBON), Pred::Degree(3)])
                || leaves_equal(&leaves, &[Pred::Element(z::CARBON), Pred::Degree(4)])
                || leaves_equal(&leaves, &[Pred::Element(z::NITROGEN), Pred::Degree(3)])
                || leaves_equal(&leaves, &[Pred::AtomicNumber(z::NITROGEN), Pred::InRing]);
        }
        return false;
    }

    // Two-atom bond classes: `C=O`, `C#N`, `[R]~[R]`.
    if query.atom_count() == 2 && query.bonds.len() == 1 {
        let bond = &query.bonds[0];
        let forced_double =
            matches!(bond.pred, crate::query::BondPred::Order(o) if o == bond_order::DOUBLE);
        let la = &query.atoms[0];
        let lb = &query.atoms[1];
        let forced_triple =
            matches!(bond.pred, crate::query::BondPred::Order(o) if o == bond_order::TRIPLE);
        let is_c = |p: &Pred| matches!(p, Pred::Element(z::CARBON));
        let is_o = |p: &Pred| matches!(p, Pred::Element(z::OXYGEN));
        let is_n = |p: &Pred| matches!(p, Pred::Element(z::NITROGEN));
        if forced_double && ((is_c(la) && is_o(lb)) || (is_o(la) && is_c(lb))) {
            return true;
        }
        if forced_triple && ((is_c(la) && is_n(lb)) || (is_n(la) && is_c(lb))) {
            return true;
        }
        // `[R]~[R]` only: the class ignores order, so `[R]-[R]` is not exact.
        if matches!(bond.pred, crate::query::BondPred::Any)
            && matches!(la, Pred::InRing)
            && matches!(lb, Pred::InRing)
        {
            return true;
        }
    }

    // Group classes: a centre atom in the class is itself a valid embedding.
    if is_amide_query(query) || is_acid_query(query) || is_ester_query(query) {
        return true;
    }
    if is_sulfonamide_query(query) {
        return true;
    }
    false
}

/// Per-molecule censuses stored column-major: one dense `Vec<u16>` per class.
///
/// Rows use the same dense row id (`u32`) as the screen and the molecule store.
/// Costs `NUM_CLASSES * 2` bytes per molecule.
#[derive(Clone, Debug, Default)]
pub struct CensusColumns {
    /// Number of rows (molecules).
    n: usize,
    /// `NUM_CLASSES` dense columns, each `n` counts long: `class_counts[c][row]`.
    class_counts: Vec<Vec<u16>>,
}

impl CensusColumns {
    /// Builds the column store; `rows[i]` becomes row id `i`.
    pub fn build(rows: &[Census]) -> CensusColumns {
        let n = rows.len();
        let mut class_counts = vec![vec![0u16; n]; NUM_CLASSES];
        for (i, census) in rows.iter().enumerate() {
            for (c, col) in class_counts.iter_mut().enumerate() {
                col[i] = census.counts[c];
            }
        }
        CensusColumns { n, class_counts }
    }

    /// Number of rows.
    #[inline]
    pub fn len(&self) -> usize {
        self.n
    }

    /// True if no rows are stored.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// The count of class `class` at row `row`. Used for early-accept (see
    /// [`census_early_accept_class`]).
    ///
    /// # Panics
    /// Panics unless `class < NUM_CLASSES` and `row < self.len()`.
    #[inline]
    #[must_use]
    pub fn count_at(&self, class: usize, row: u32) -> u16 {
        self.class_counts[class][row as usize]
    }

    /// Keeps the `candidates` whose census dominates `min`: a superset of the true
    /// matches. Only classes with `min[c] > 0` are checked. Order is preserved.
    ///
    /// When `min` is all-zero, returns `candidates` unchanged.
    ///
    /// # Panics
    /// Panics if any id in `candidates` is `>= self.len()`. The all-zero-`min`
    /// shortcut returns such an id unchecked, so pass only real row ids.
    #[must_use]
    pub fn filter(&self, min: &Census, candidates: &[u32]) -> Vec<u32> {
        // The classes the query constrains.
        let mut active: Vec<usize> = Vec::new();
        for c in 0..NUM_CLASSES {
            if min.counts[c] > 0 {
                active.push(c);
            }
        }
        if active.is_empty() {
            return candidates.to_vec();
        }

        let mut out = Vec::with_capacity(candidates.len());
        for &row in candidates {
            let r = row as usize;
            let mut ok = true;
            for &c in &active {
                if self.class_counts[c][r] < min.counts[c] {
                    ok = false;
                    break;
                }
            }
            if ok {
                out.push(row);
            }
        }
        out
    }

    /// Approximate heap footprint in bytes, by reserved capacity.
    pub fn mem_bytes(&self) -> usize {
        use core::mem::size_of;
        let mut bytes = self.class_counts.capacity() * size_of::<Vec<u16>>();
        for col in &self.class_counts {
            bytes += col.capacity() * size_of::<u16>();
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smarts::parse as parse_smarts;
    use crate::smiles::parse as parse_smiles;

    #[test]
    fn target_census_counts_atoms_and_bonds() {
        // Acetic acid CC(=O)O: 2 aliphatic C, 2 aliphatic O, one C=O double bond.
        let mol = parse_smiles("CC(=O)O").unwrap();
        let c = target_census(&mol);
        assert_eq!(c.count(class::C), 2);
        assert_eq!(c.count(class::O), 2);
        assert_eq!(c.count(class::BOND_DOUBLE), 1);
        assert_eq!(c.count(class::C_AROM), 0);
        assert_eq!(c.count(class::N), 0);
    }

    #[test]
    fn aromatic_classes_are_distinct_from_aliphatic() {
        // Pyridine c1ccncc1: 5 aromatic C, 1 aromatic N, and aromatic bonds.
        let mol = parse_smiles("c1ccncc1").unwrap();
        let c = target_census(&mol);
        assert_eq!(c.count(class::C_AROM), 5);
        assert_eq!(c.count(class::N_AROM), 1);
        assert_eq!(
            c.count(class::C),
            0,
            "aromatic carbons must not count as aliphatic"
        );
        assert!(c.count(class::BOND_AROM) >= 6, "ring has aromatic bonds");
    }

    #[test]
    fn query_min_only_counts_forced_classes() {
        // `c1ccccc1` forces six aromatic carbons.
        let q = parse_smarts("c1ccccc1").unwrap();
        let m = query_min_census(&q);
        assert_eq!(m.count(class::C_AROM), 6);
        assert_eq!(m.count(class::C), 0);

        // An OR / wildcard query forces nothing ⇒ all-zero minimum (degrades).
        for qs in ["[C,N]", "*", "[!C]", "a"] {
            let q = parse_smarts(qs).unwrap();
            let m = query_min_census(&q);
            assert!(
                (0..NUM_CLASSES).all(|c| m.count(c) == 0),
                "query {qs:?} should force no census class"
            );
        }
    }

    #[test]
    fn fused_benzene_pair_counts() {
        // Target side: only carbon 6/6 ortho-fusions count.
        let cases = [
            ("c1ccccc1", 0u16),              // benzene: one ring
            ("c1ccc2ccccc2c1", 1),           // naphthalene: one fusion bond
            ("c1ccc2cc3ccccc3cc2c1", 2),     // anthracene: two fusion bonds (linear)
            ("c1ccc2ccc3ccccc3c2c1", 2),     // phenanthrene: two fusion bonds (angular)
            ("c1ccc(-c2ccccc2)cc1", 0),      // biphenyl: single-bond join
            ("C1c2ccccc2-c2ccccc21", 0),     // fluorene: benzene+5-ring fusions
            ("c1ccc2c(c1)[nH]c1ccccc12", 0), // carbazole: benzene+pyrrole, no 6/6 fusion
            ("c1ccc2c(c1)oc1ccccc12", 0),    // dibenzofuran: benzene+furan, no 6/6 fusion
            ("O=C(O)c1ccccc1", 0),           // benzoic acid: one ring
        ];
        for (smi, want) in cases {
            let mol = parse_smiles(smi).unwrap();
            assert_eq!(
                target_fused_benzene_pair(&mol),
                want,
                "target fused-benzene-pair count for {smi:?}"
            );
        }
        // Query side: naphthalene forces one fused pair; benzene none.
        assert_eq!(
            query_fused_benzene_pair(&parse_smarts("c1ccc2ccccc2c1").unwrap()),
            1
        );
        assert_eq!(
            query_fused_benzene_pair(&parse_smarts("c1ccccc1").unwrap()),
            0
        );
        assert_eq!(
            query_fused_benzene_pair(&parse_smarts("c1ccc(O)cc1").unwrap()),
            0
        );
        // The pre-allocation gate must return the identical count as the full path.
        for smi in ["c1ccc2ccccc2c1", "c1ccccc1", "CCO", "c1ccc2cc3ccccc3cc2c1"] {
            let mol = parse_smiles(smi).unwrap();
            let n = mol.atom_count();
            let eligible: Vec<bool> = (0..n as u32)
                .map(|a| mol.element[a as usize] == z::CARBON && mol.is_aromatic(a))
                .collect();
            let adj: Vec<Vec<(u32, u32)>> = (0..n as u32)
                .map(|a| {
                    mol.neighbors(a)
                        .iter()
                        .map(|nb| (nb.atom, nb.bond))
                        .collect()
                })
                .collect();
            assert_eq!(
                target_fused_benzene_pair(&mol),
                fused_six_cycle_shared_bonds(n, mol.bond_count(), &adj, &eligible),
                "alloc-gated == ungated for {smi:?}"
            );
        }
    }

    #[test]
    fn fusion_arom_c_counts() {
        // A biphenyl pivot has 3 aromatic-C neighbours (biphenyl = 2). Quinoline has
        // one all-carbon fusion carbon; the other neighbours the ring N.
        let target = [
            ("c1ccccc1", 0u16),          // benzene
            ("c1ccc2ccccc2c1", 2),       // naphthalene: both fusion carbons
            ("c1ccc2cc3ccccc3cc2c1", 4), // anthracene: four
            ("c1ccc(-c2ccccc2)cc1", 2),  // biphenyl: the two pivot carbons
            ("c1ccc2ncccc2c1", 1),       // quinoline: one all-carbon fusion carbon
        ];
        for (smi, want) in target {
            let mol = parse_smiles(smi).unwrap();
            assert_eq!(
                target_census(&mol).count(class::FUSION_AROM_C),
                want,
                "target FUSION_AROM_C for {smi:?}"
            );
        }
        assert_eq!(
            query_min_census(&parse_smarts("c1ccc2ccccc2c1").unwrap()).count(class::FUSION_AROM_C),
            2
        );
        assert_eq!(
            query_min_census(&parse_smarts("c1ccccc1").unwrap()).count(class::FUSION_AROM_C),
            0
        );
    }

    #[test]
    fn phenol_c_on_fused_and_hetero_targets() {
        // Fires on fused phenols; silent on hetero-ring and aliphatic-ring cases.
        let cases = [
            ("c1ccc(O)cc1", 1u16),  // phenol
            ("Oc1ccc2ccccc2c1", 1), // 2-naphthol: O on a benzene ring of naphthalene
            ("Oc1cccc2ccccc12", 1), // 1-naphthol
            ("COc1ccccc1", 1),      // anisole: O (bare aliphatic) directly on the benzene
            ("Oc1ccccc1O", 1),      // catechol
            ("Cc1ccccc1", 0),       // toluene: no O
            ("OCc1ccccc1", 0),      // benzyl alcohol: O is one bond OFF the ring
            ("c1ccncc1", 0),        // pyridine: no benzene, no O
            ("Oc1ccncc1", 0),       // hydroxypyridine: O on a non-all-carbon ring
            ("OC1CCCCC1", 0),       // cyclohexanol: aliphatic ring, not benzene
        ];
        for (smi, want) in cases {
            let mol = parse_smiles(smi).unwrap();
            assert_eq!(
                target_census(&mol).count(class::PHENOL_C).min(1),
                want,
                "PHENOL_C presence for {smi:?}"
            );
        }
    }

    #[test]
    fn census_columns_count_at_indexing() {
        // Pins the argument order: `count_at(class, row)`.
        let mut a = Census::new();
        a.counts[class::C] = 3;
        a.counts[class::PHENOL_C] = 1;
        let mut b = Census::new();
        b.counts[class::C] = 5;
        b.counts[class::PHENOL_C] = 0;
        let cols = CensusColumns::build(&[a, b]);
        assert_eq!(cols.count_at(class::C, 0), 3);
        assert_eq!(cols.count_at(class::C, 1), 5);
        assert_eq!(cols.count_at(class::PHENOL_C, 0), 1);
        assert_eq!(cols.count_at(class::PHENOL_C, 1), 0);
    }

    #[test]
    fn phenol_early_accept_detection() {
        // The phenol pattern in several spellings ⇒ early-accept on PHENOL_C.
        for q in ["c1ccc(O)cc1", "Oc1ccccc1", "c1cc(O)ccc1"] {
            assert_eq!(
                census_early_accept_class(&parse_smarts(q).unwrap()),
                Some(class::PHENOL_C),
                "{q:?} should map to the PHENOL_C early-accept class"
            );
        }
        // Not the phenol shape ⇒ no early-accept (must verify normally).
        for q in [
            "c1ccccc1",       // plain benzene, no O
            "c1ccncc1",       // pyridine: ring atom is N
            "Nc1ccccc1",      // aniline: substituent is N, not O
            "c1ccc(O)nc1",    // O on a pyridine, not all-carbon ring
            "Oc1ccc(O)cc1",   // two O substituents
            "OC1CCCCC1",      // aliphatic ring, not aromatic
            "c1ccc2ccccc2c1", // naphthalene, no O
            "C=O",            // not a ring at all
            "OCc1ccccc1",     // benzyl alcohol: O one bond too far from the ring
        ] {
            assert_eq!(
                census_early_accept_class(&parse_smarts(q).unwrap()),
                None,
                "{q:?} must NOT be treated as a phenol early-accept"
            );
        }
    }

    #[test]
    fn dominates_is_classwise_ge() {
        let mut t = Census::new();
        t.counts[class::C] = 5;
        t.counts[class::O] = 2;
        let mut q = Census::new();
        q.counts[class::C] = 5;
        q.counts[class::O] = 1;
        assert!(t.dominates(&q), "5>=5 and 2>=1");
        q.counts[class::O] = 3;
        assert!(!t.dominates(&q), "2 < 3 in O fails");
        // Empty minimum is dominated by anything (degrade case).
        assert!(t.dominates(&Census::new()));
    }

    #[test]
    fn halogen_union_and_quaternary_carbon_classes() {
        // Each halogen counts in ANY_HALOGEN and in its element class.
        let mol = parse_smiles("FC(Cl)Br").unwrap();
        let t = target_census(&mol);
        assert_eq!(t.count(class::ANY_HALOGEN), 3, "F + Cl + Br = 3 halogens");
        assert_eq!(t.count(class::F), 1);
        assert_eq!(t.count(class::CL), 1);
        assert_eq!(t.count(class::BR), 1);

        // An OR of halogens forces ANY_HALOGEN though no single element is forced.
        let m = query_min_census(&parse_smarts("[F,Cl,Br,I]").unwrap());
        assert_eq!(
            m.count(class::ANY_HALOGEN),
            1,
            "the disjunction forces one halogen"
        );
        assert!(
            (0..NUM_CLASSES).all(|c| c == class::ANY_HALOGEN || m.count(c) == 0),
            "[F,Cl,Br,I] forces nothing except the union class"
        );

        // A non-halogen alternative must not force the union, or a molecule matching
        // through it would be dropped.
        let m = query_min_census(&parse_smarts("[F,Cl,#7]").unwrap());
        assert_eq!(
            m.count(class::ANY_HALOGEN),
            0,
            "a non-halogen alternative breaks the union"
        );

        // C_DEG4: a target counts aliphatic carbons of degree exactly 4.
        let neo = parse_smiles("CC(C)(C)C").unwrap(); // neopentane: central C has degree 4
        assert_eq!(
            target_census(&neo).count(class::C_DEG4),
            1,
            "one quaternary carbon"
        );
        assert_eq!(
            target_census(&parse_smiles("CCC").unwrap()).count(class::C_DEG4),
            0
        );

        // [CD4] forces the quaternary class; [CD3], bare C, and an OR do not.
        assert_eq!(
            query_min_census(&parse_smarts("[CD4]").unwrap()).count(class::C_DEG4),
            1
        );
        assert_eq!(
            query_min_census(&parse_smarts("[CD3]").unwrap()).count(class::C_DEG4),
            0
        );
        assert_eq!(
            query_min_census(&parse_smarts("C").unwrap()).count(class::C_DEG4),
            0
        );
        assert_eq!(
            query_min_census(&parse_smarts("[CD4,N]").unwrap()).count(class::C_DEG4),
            0
        );

        // N_DEG3: a target counts ALIPHATIC nitrogens of degree exactly 3.
        let tea = parse_smiles("CCN(CC)CC").unwrap(); // triethylamine: central N degree 3
        assert_eq!(
            target_census(&tea).count(class::N_DEG3),
            1,
            "one tertiary amine N"
        );
        assert_eq!(
            target_census(&parse_smiles("CCN").unwrap()).count(class::N_DEG3),
            0,
            "primary amine N is degree 1"
        );
        // Aromatic nitrogen of degree 3 (N-methylpyrrole's N) is NOT aliphatic ⇒ not counted.
        assert_eq!(
            target_census(&parse_smiles("Cn1cccc1").unwrap()).count(class::N_DEG3),
            0,
            "aromatic N is excluded"
        );

        // [ND3] forces the tertiary-amine class; [ND4]/[nD3]/bare N / OR do not.
        assert_eq!(
            query_min_census(&parse_smarts("[ND3]").unwrap()).count(class::N_DEG3),
            1
        );
        assert_eq!(
            query_min_census(&parse_smarts("[ND4]").unwrap()).count(class::N_DEG3),
            0
        );
        assert_eq!(
            query_min_census(&parse_smarts("[nD3]").unwrap()).count(class::N_DEG3),
            0,
            "aromatic n is not the aliphatic class"
        );
        assert_eq!(
            query_min_census(&parse_smarts("N").unwrap()).count(class::N_DEG3),
            0
        );
        assert_eq!(
            query_min_census(&parse_smarts("[ND3,O]").unwrap()).count(class::N_DEG3),
            0,
            "an OR alternative breaks the force"
        );
    }

    #[test]
    fn census_exact_single_atom_detection() {
        // Accepted (census class is EXACTLY the predicate ⇒ skip verify):
        for q in [
            "C",
            "c",
            "N",
            "n",
            "O",
            "F",
            "Cl",
            "[CD3]",
            "[CD4]",
            "[ND3]",
            "[#7;R]",
            "[F,Cl,Br,I]",
            "C=O",
            "O=C",
            // Augmented-atom acyl stars.
            "C(=O)N",
            "NC=O",
            "C(=O)[OH]",
            "[OH]C=O", // amide / acid written both ways
            "[R]~[R]", // any bond between two ring atoms
            "C(=O)OC",
            "COC=O", // ester chain (both spellings)
            "C#N",
            "N#C", // nitrile triple bond (both spellings)
            "S(=O)(=O)N",
            "NS(=O)=O",
            "O=S(=O)N", // sulfonamide (various spellings)
        ] {
            assert!(
                is_census_exact(&parse_smarts(q).unwrap()),
                "{q:?} should be census-exact"
            );
        }
        // Rejected (NOT exactly a census class, or multi-atom ⇒ must verify):
        for q in [
            "[#6]",        // any-aromaticity carbon: spans two element classes
            "P",           // the P class also holds aromatic `p`
            "p",           // ... and aliphatic `P`
            "B",           // B is in OTHER, with every rare element
            "[Zn]",        // OTHER
            "[Fe]",        // OTHER
            "[Si]",        // OTHER
            "[CD3;+]",     // extra charge constraint the census can't capture
            "[C,N]",       // OR not pinned to one class
            "[F,Cl]",      // halogen subset ≠ ANY_HALOGEN
            "[cD3]",       // aromatic carbon degree 3: no such class
            "[nD3]",       // aromatic nitrogen degree 3: no such class (N_DEG3 is aliphatic)
            "[ND3;+]",     // extra charge constraint the N_DEG3 class can't capture
            "[ND4]",       // aliphatic N degree 4 (e.g. ammonium): not the degree-3 class
            "[#7;R]C",     // multi-atom
            "C-O",         // single bond, not the carbonyl double
            "C=N",         // C=N double, not C=O
            "C#C",         // C≡C triple, not C≡N
            "N#N",         // N≡N triple, not C≡N
            "C#O",         // C≡O triple, not C≡N
            "S(=O)(=O)O",  // sulfonic acid: third leaf is O, not N
            "S(=O)N",      // sulfinamide: only one =O
            "S(=O)(=O)C",  // sulfone: no N
            "S(=O)(=O)Cl", // sulfonyl chloride: no N
            "C=O.N",       // extra disconnected atom
            "[!C]",        // negation
            "C(=O)O",      // acid w/o the [OH] H-constraint: ester O (0 H) would match
            "C(=O)C",      // ketone: no N/OH leaf
            "C(=O)Cl",     // acyl chloride: no N/OH leaf
            "N(=O)N",      // centre is N, not the aliphatic-carbon AMIDE_C class
            "C(=N)N",      // amidine: =N not =O, not the AMIDE_C class
            "[R]-[R]",     // forced SINGLE: census ignores order, so not exact
            "[R]~C",       // second atom unconstrained: not both-in-ring
            "[R]~[R]~[R]", // three atoms: not the 2-atom bond class
            "C(=O)O",      // acid-O without the alkyl C: not the ester chain
            "CC(=O)OC",    // 5 atoms: an extra carbon the ESTER_C class can't force
            "C(=O)ON",     // bridging O leads to N, not the alkyl carbon
        ] {
            assert!(
                !is_census_exact(&parse_smarts(q).unwrap()),
                "{q:?} must NOT be treated as census-exact"
            );
        }
    }

    #[test]
    fn filter_is_a_sound_superset_matching_per_row_dominates() {
        // The column filter must equal per-row `dominates` and keep every brute match.
        let targets = [
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
            "CC(C)(C)C",
            "Brc1ccccc1",
            "FC(Cl)Br",
            "CC(C)(O)C",
        ];
        let queries = [
            "C",
            "c",
            "N",
            "O",
            "c1ccccc1",
            "C=O",
            "C#N",
            "FC(F)(F)F",
            "Cl",
            "c1ccncc1",
            "[C,N]",
            "*",
            "CC(=O)O",
            "S(=O)(=O)N",
            // Property classes, and a mixed OR that must force nothing.
            "[F,Cl,Br,I]",
            "[CD4]",
            "[CD3]",
            "[F,Cl,#7]",
            "[Br,I]",
        ];
        let mols: Vec<_> = targets.iter().map(|s| parse_smiles(s).unwrap()).collect();
        let censuses: Vec<Census> = mols.iter().map(target_census).collect();
        let cols = CensusColumns::build(&censuses);
        let all: Vec<u32> = (0..mols.len() as u32).collect();

        for qs in queries {
            let q = parse_smarts(qs).unwrap();
            let min = query_min_census(&q);

            // Oracle: per-row dominates over the full candidate set.
            let oracle: Vec<u32> = all
                .iter()
                .copied()
                .filter(|&r| censuses[r as usize].dominates(&min))
                .collect();
            let got = cols.filter(&min, &all);
            assert_eq!(
                got, oracle,
                "census column filter != per-row oracle for {qs:?}"
            );

            // No false negative: every true brute match survives the census filter.
            for (row, mol) in mols.iter().enumerate() {
                if crate::brute::matches(&q, mol) {
                    assert!(
                        got.contains(&(row as u32)),
                        "FALSE NEGATIVE: census dropped a true match for {qs:?} on {:?}",
                        targets[row]
                    );
                }
            }
        }
    }
}
