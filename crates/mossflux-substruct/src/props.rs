//! Molecular formula and average molecular weight of a [`MolGraph`].

use crate::graph::MolGraph;

/// Element symbols by atomic number (index 0 is the wildcard `*`).
const SYMBOLS: [&str; 119] = [
    "*", "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S",
    "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge",
    "As", "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd",
    "In", "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd",
    "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg",
    "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm",
    "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn",
    "Nh", "Fl", "Mc", "Lv", "Ts", "Og",
];

/// Standard atomic weights (IUPAC conventional values; mass number of the most
/// stable isotope for elements without one).
const WEIGHTS: [f64; 119] = [
    0.0, 1.008, 4.0026, 6.94, 9.0122, 10.81, 12.011, 14.007, 15.999, 18.998, 20.180, 22.990,
    24.305, 26.982, 28.085, 30.974, 32.06, 35.45, 39.95, 39.098, 40.078, 44.956, 47.867, 50.942,
    51.996, 54.938, 55.845, 58.933, 58.693, 63.546, 65.38, 69.723, 72.630, 74.922, 78.971, 79.904,
    83.798, 85.468, 87.62, 88.906, 91.224, 92.906, 95.95, 98.0, 101.07, 102.91, 106.42, 107.87,
    112.41, 114.82, 118.71, 121.76, 127.60, 126.90, 131.29, 132.91, 137.33, 138.91, 140.12, 140.91,
    144.24, 145.0, 150.36, 151.96, 157.25, 158.93, 162.50, 164.93, 167.26, 168.93, 173.05, 174.97,
    178.49, 180.95, 183.84, 186.21, 190.23, 192.22, 195.08, 196.97, 200.59, 204.38, 207.2, 208.98,
    209.0, 210.0, 222.0, 223.0, 226.0, 227.0, 232.04, 231.04, 238.03, 237.0, 244.0, 243.0, 247.0,
    247.0, 251.0, 252.0, 257.0, 258.0, 259.0, 266.0, 267.0, 268.0, 269.0, 270.0, 277.0, 278.0,
    281.0, 282.0, 285.0, 286.0, 289.0, 290.0, 293.0, 294.0, 294.0,
];

/// The symbol of atomic number `z`, or `"*"` if out of range.
#[must_use]
pub fn symbol(z: u8) -> &'static str {
    SYMBOLS.get(usize::from(z)).copied().unwrap_or("*")
}

fn element_counts(mol: &MolGraph) -> [u32; 119] {
    let mut counts = [0u32; 119];
    for (i, &z) in mol.element.iter().enumerate() {
        if let Some(c) = counts.get_mut(usize::from(z)) {
            *c += 1;
        }
        counts[1] += u32::from(mol.h_count[i]);
    }
    counts
}

/// Hill-order molecular formula with net charge, for example `C9H8O4` or
/// `C2H3O2-`.
#[must_use]
pub fn formula(mol: &MolGraph) -> String {
    let counts = element_counts(mol);
    let mut order: Vec<usize> = (1..counts.len()).filter(|&z| counts[z] > 0).collect();
    if counts[6] > 0 {
        order.sort_by_key(|&z| match z {
            6 => (0, ""),
            1 => (1, ""),
            _ => (2, SYMBOLS[z]),
        });
    } else {
        order.sort_by_key(|&z| SYMBOLS[z]);
    }
    let mut out = String::new();
    for z in order {
        out.push_str(SYMBOLS[z]);
        if counts[z] > 1 {
            out.push_str(&counts[z].to_string());
        }
    }
    let charge: i32 = mol.charge.iter().map(|&c| i32::from(c)).sum();
    match charge {
        0 => {}
        1 => out.push('+'),
        -1 => out.push('-'),
        c if c > 0 => out.push_str(&format!("{c}+")),
        c => out.push_str(&format!("{}-", -c)),
    }
    out
}

/// Average molecular weight in g/mol.
#[must_use]
pub fn weight(mol: &MolGraph) -> f64 {
    element_counts(mol)
        .iter()
        .zip(WEIGHTS)
        .map(|(&n, w)| f64::from(n) * w)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smiles::parse;

    #[test]
    fn aspirin() {
        let m = parse("CC(=O)Oc1ccccc1C(=O)O").unwrap();
        assert_eq!(formula(&m), "C9H8O4");
        assert!((weight(&m) - 180.159).abs() < 0.01);
    }

    #[test]
    fn charged_and_inorganic() {
        assert_eq!(formula(&parse("CC(=O)[O-]").unwrap()), "C2H3O2-");
        assert_eq!(formula(&parse("[Na+].[Cl-]").unwrap()), "ClNa");
        assert_eq!(formula(&parse("O").unwrap()), "H2O");
        assert_eq!(symbol(17), "Cl");
    }
}
