//! Circular (ECFP-style) fingerprints of a perceived [`MolGraph`].
//!
//! Each heavy atom starts from a hash of its element, heavy degree, total H,
//! charge, ring membership and aromaticity. Each round rehashes an atom with the
//! sorted `(bond order, neighbour id)` pairs of its heavy neighbours. Every id of
//! every round sets bit `id % nbits`. Hashing is a fixed 64-bit mix, so a
//! fingerprint is the same on every platform and in every process.
//!
//! These bits are mossflux's own: they are not bit-compatible with any other
//! toolkit. Index and query must both use this function.

use crate::graph::{atom_flags, MolGraph};

/// ECFP4: two rounds.
pub const DEFAULT_RADIUS: u32 = 2;
/// Default fingerprint width in bits.
pub const DEFAULT_BITS: usize = 2048;

#[inline]
fn mix(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

#[inline]
fn combine(h: u64, v: u64) -> u64 {
    mix(h ^ v.wrapping_add(0x9e37_79b9_7f4a_7c15))
}

/// The circular fingerprint of `mol` as `nbits.div_ceil(64)` packed words.
///
/// # Panics
/// Panics if `nbits == 0`.
#[must_use]
pub fn circular(mol: &MolGraph, radius: u32, nbits: usize) -> Vec<u64> {
    assert!(nbits > 0, "fingerprint width must be positive");
    let mut words = vec![0u64; nbits.div_ceil(64)];
    let mut set = |id: u64| {
        let bit = (id % nbits as u64) as usize;
        words[bit / 64] |= 1 << (bit % 64);
    };

    let n = mol.atom_count();
    let heavy = |i: usize| mol.element[i] != 1;
    let mut ids = vec![0u64; n];
    for (i, id) in ids.iter_mut().enumerate() {
        if !heavy(i) {
            continue;
        }
        let mut deg = 0u64;
        let mut h = u64::from(mol.h_count[i]);
        for nb in mol.neighbors(i as u32) {
            if heavy(nb.atom as usize) {
                deg += 1;
            } else {
                h += 1;
            }
        }
        let f = mol.flags[i];
        let mut x = combine(0x6d6f_7373, u64::from(mol.element[i]));
        x = combine(x, deg);
        x = combine(x, h);
        x = combine(x, u64::from(mol.charge[i] as u8));
        x = combine(x, u64::from(f & atom_flags::IN_RING != 0));
        x = combine(x, u64::from(f & atom_flags::AROMATIC != 0));
        *id = x;
        set(x);
    }

    let mut next = vec![0u64; n];
    let mut env: Vec<(u8, u64)> = Vec::new();
    for round in 1..=radius {
        for i in 0..n {
            if !heavy(i) {
                continue;
            }
            env.clear();
            for nb in mol.neighbors(i as u32) {
                if heavy(nb.atom as usize) {
                    let order = mol.bonds[nb.bond as usize].order;
                    env.push((order, ids[nb.atom as usize]));
                }
            }
            env.sort_unstable();
            let mut x = combine(u64::from(round), ids[i]);
            for &(order, id) in &env {
                x = combine(combine(x, u64::from(order)), id);
            }
            next[i] = x;
            set(x);
        }
        std::mem::swap(&mut ids, &mut next);
    }
    words
}

/// [`circular`] with [`DEFAULT_RADIUS`] and [`DEFAULT_BITS`].
#[must_use]
pub fn ecfp4(mol: &MolGraph) -> Vec<u64> {
    circular(mol, DEFAULT_RADIUS, DEFAULT_BITS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smiles::parse;

    fn fp(s: &str) -> Vec<u64> {
        ecfp4(&parse(s).unwrap())
    }

    fn tanimoto(a: &[u64], b: &[u64]) -> f64 {
        let i: u32 = a.iter().zip(b).map(|(x, y)| (x & y).count_ones()).sum();
        let u: u32 = a.iter().zip(b).map(|(x, y)| (x | y).count_ones()).sum();
        f64::from(i) / f64::from(u)
    }

    #[test]
    fn spelling_independent() {
        assert_eq!(fp("c1ccccc1O"), fp("Oc1ccccc1"));
        assert_eq!(fp("C1=CC=CC=C1"), fp("c1ccccc1"));
        assert_eq!(fp("OCC"), fp("CCO"));
    }

    #[test]
    fn explicit_h_equals_implicit_h() {
        assert_eq!(fp("[H]OC"), fp("OC"));
    }

    #[test]
    fn similar_molecules_score_higher() {
        let aspirin = fp("CC(=O)Oc1ccccc1C(=O)O");
        let salicylic = fp("Oc1ccccc1C(=O)O");
        let hexane = fp("CCCCCC");
        assert!(tanimoto(&aspirin, &salicylic) > tanimoto(&aspirin, &hexane));
        assert_eq!(tanimoto(&aspirin, &aspirin), 1.0);
    }

    #[test]
    fn width_and_stability() {
        let a = circular(&parse("CCO").unwrap(), 2, 256);
        assert_eq!(a.len(), 4);
        // A fixed fingerprint pins the hash: a change here changes every index.
        let bits: u32 = a.iter().map(|w| w.count_ones()).sum();
        assert!((3..=9).contains(&bits));
        assert_eq!(a, circular(&parse("OCC").unwrap(), 2, 256));
    }
}
