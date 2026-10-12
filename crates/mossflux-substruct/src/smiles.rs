//! A SMILES parser producing a [`MolGraph`].
//!
//! # Supported syntax
//!
//! * **Organic-subset atoms** without brackets: aliphatic `B C N O P S F Cl Br I`
//!   and aromatic `b c n o p s`. They get implicit H from a valence model (see
//!   [`implicit_h`]).
//! * **Bracket atoms** `[...]`: optional isotope, element symbol (upper-case,
//!   aromatic lower-case, or `*`), optional `H`/`H<n>`, optional charge (`+`, `-`,
//!   `++`, `--`, `+n`, `-n`). The H count is exactly what is written (default 0).
//! * **Bonds**: `-`, `=`, `#`, `:`, and `.` (disconnection). No symbol means an
//!   implicit bond: aromatic between two aromatic atoms, otherwise single.
//! * **Ring closures**: a digit `0`–`9` or `%nn`. A bond symbol may precede either
//!   occurrence.
//! * **Branches**: `(` pushes the current atom, `)` pops back to it.
//!
//! Stereo markers `/`, `\` and `@`/`@@` are accepted and ignored.
//!
//! # Aromaticity
//!
//! Lowercase atoms set [`atom_flags::AROMATIC`] and `:` sets [`bond_flags::AROMATIC`]
//! as written. Perception, run at the end of [`parse`], then sets ring data,
//! re-derives aromaticity from ring topology, and re-finalises bare-atom H counts.
//!
//! # Errors
//!
//! Parsing returns a [`SmilesError`] for the first problem, with its byte
//! position, so an index build can skip a bad row. One left-to-right pass with a
//! branch stack and a ring-closure table; no backtracking.

use crate::graph::{atom_flags, bond_flags, bond_order, MolGraph, MolGraphBuilder};

/// Everything that can go wrong while parsing a SMILES string.
///
/// Most variants carry the byte offset at or near the problem.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SmilesError {
    /// A character that cannot begin a token in the current state.
    UnexpectedChar {
        /// Byte offset of the offending character.
        pos: usize,
        /// The character itself.
        ch: char,
    },
    /// A `[` bracket atom was opened but never closed before end-of-input.
    UnterminatedBracket {
        /// Byte offset of the opening `[`.
        pos: usize,
    },
    /// A bracket atom contained no element symbol (e.g. `[]` or `[+]`).
    MissingElement {
        /// Byte offset within the bracket.
        pos: usize,
    },
    /// An element symbol is not supported.
    UnknownElement {
        /// Byte offset of the symbol.
        pos: usize,
        /// The symbol text as read.
        symbol: String,
    },
    /// A `)` with no matching `(` on the branch stack.
    UnbalancedClose {
        /// Byte offset of the `)`.
        pos: usize,
    },
    /// End-of-input reached with branches still open (missing `)`).
    UnbalancedOpen {
        /// Byte offset of the first still-open `(`.
        pos: usize,
    },
    /// A bond symbol or ring-closure digit appeared with no preceding atom to
    /// attach it to (e.g. a leading `=` or `1`).
    DanglingBond {
        /// Byte offset of the bond/closure.
        pos: usize,
    },
    /// A ring-closure label was left open at end-of-input (only one occurrence).
    UnclosedRing {
        /// The ring-bond label (0–99) that was never closed.
        label: u16,
        /// Byte offset where the (unmatched) opening occurrence appeared.
        pos: usize,
    },
    /// The two halves of one ring closure have different bond symbols (e.g.
    /// `C=1CCCCC#1`).
    RingBondMismatch {
        /// The ring-bond label.
        label: u16,
        /// Byte offset of the second (conflicting) occurrence.
        pos: usize,
    },
    /// A ring closure would connect an atom to itself or duplicate an existing
    /// bond between the same pair (e.g. `C11`).
    InvalidRingClosure {
        /// The ring-bond label.
        label: u16,
        /// Byte offset of the closing occurrence.
        pos: usize,
    },
    /// A `%` ring label was not followed by two decimal digits.
    BadPercentLabel {
        /// Byte offset of the `%`.
        pos: usize,
    },
    /// A branch `(...)` held no atom (e.g. `C()C` or `C(=)C`).
    EmptyBranch {
        /// Byte offset of the `(`.
        pos: usize,
    },
    /// A bracket-atom number is out of range (isotope above 999, charge beyond
    /// ±15). It is refused, not clamped.
    ValueOutOfRange {
        /// Byte offset of the field.
        pos: usize,
        /// The field (`"isotope"`, `"charge"`).
        what: &'static str,
        /// The largest allowed magnitude.
        max: u32,
    },
    /// The molecule exceeds a size limit: [`MAX_ATOMS`] atoms, [`MAX_RINGS`] ring
    /// closures, or [`crate::perception::MAX_PERCEPTION_STEPS`] ring-perception
    /// steps. The limits keep one hostile row from stalling an index build.
    TooLarge {
        /// What was counted (`"atoms"`, `"ring closures"`).
        what: &'static str,
        /// The limit that was exceeded.
        limit: usize,
    },
    /// The string contained no atoms.
    Empty,
}

/// The most atoms one SMILES may hold. The compact molecule store addresses atoms
/// with a `u16` and refuses 65,535 or more; checking here avoids the perception
/// cost for a molecule that could never be indexed.
pub const MAX_ATOMS: usize = 65_534;

/// The most ring closures one SMILES may hold. Chain bonds form a forest, so this
/// is also the ring count (circuit rank), which drives ring-perception cost.
pub const MAX_RINGS: usize = 1_024;

/// The largest formal charge magnitude a bracket atom may state (OpenSMILES).
pub const MAX_CHARGE: u32 = 15;

impl core::fmt::Display for SmilesError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SmilesError::UnexpectedChar { pos, ch } => {
                write!(f, "unexpected character {ch:?} at byte {pos}")
            }
            SmilesError::UnterminatedBracket { pos } => {
                write!(f, "unterminated '[' bracket atom opened at byte {pos}")
            }
            SmilesError::MissingElement { pos } => {
                write!(f, "bracket atom near byte {pos} has no element symbol")
            }
            SmilesError::UnknownElement { pos, symbol } => {
                write!(f, "unknown element symbol {symbol:?} at byte {pos}")
            }
            SmilesError::UnbalancedClose { pos } => {
                write!(f, "')' without matching '(' at byte {pos}")
            }
            SmilesError::UnbalancedOpen { pos } => {
                write!(f, "'(' opened at byte {pos} was never closed")
            }
            SmilesError::DanglingBond { pos } => {
                write!(
                    f,
                    "bond or ring-closure at byte {pos} has no preceding atom"
                )
            }
            SmilesError::UnclosedRing { label, pos } => {
                write!(
                    f,
                    "ring-closure label {label} opened at byte {pos} was never closed"
                )
            }
            SmilesError::RingBondMismatch { label, pos } => {
                write!(f, "ring-closure label {label} given conflicting bond orders (second at byte {pos})")
            }
            SmilesError::InvalidRingClosure { label, pos } => {
                write!(
                    f,
                    "ring-closure label {label} at byte {pos} would self-bond or duplicate a bond"
                )
            }
            SmilesError::BadPercentLabel { pos } => {
                write!(f, "'%' at byte {pos} must be followed by two digits")
            }
            SmilesError::EmptyBranch { pos } => {
                write!(f, "branch opened at byte {pos} holds no atom")
            }
            SmilesError::ValueOutOfRange { pos, what, max } => {
                write!(f, "{what} at byte {pos} is out of range (at most {max})")
            }
            SmilesError::TooLarge { what, limit } => {
                write!(f, "molecule exceeds the limit of {limit} {what}")
            }
            SmilesError::Empty => write!(f, "empty SMILES (no atoms)"),
        }
    }
}

impl std::error::Error for SmilesError {}

/// Normal valences of an organic-subset element, in ascending order, or `None` if
/// the element needs a bracket atom.
///
/// Implicit H fills the lowest normal valence `>=` the atom's bond sum; past the
/// highest, no implicit H is added. These are the standard organic-subset valences.
fn normal_valences(element: u8) -> Option<&'static [u8]> {
    // Atomic numbers: B=5 C=6 N=7 O=8 P=15 S=16 F=9 Cl=17 Br=35 I=53.
    match element {
        5 => Some(&[3]),        // B
        6 => Some(&[4]),        // C
        7 => Some(&[3]),        // N  (charged or 5-valent N needs a bracket)
        8 => Some(&[2]),        // O
        15 => Some(&[3, 5]),    // P
        16 => Some(&[2, 4, 6]), // S
        9 => Some(&[1]),        // F
        17 => Some(&[1]),       // Cl
        35 => Some(&[1]),       // Br
        53 => Some(&[1]),       // I
        _ => None,
    }
}

/// Implicit H count of a bare organic-subset atom.
///
/// `bond_sum` is the sum of incident bond orders, with aromatic bonds counted
/// as 1. An aromatic atom adds one more unit, so benzene `c` (2 + 1) fills
/// valence 4 with one H. Returns `0` for elements outside the organic subset.
pub fn implicit_h(element: u8, bond_sum: u32, aromatic: bool) -> u8 {
    let Some(valences) = normal_valences(element) else {
        return 0;
    };
    let effective = bond_sum + if aromatic { 1 } else { 0 };
    for &v in valences {
        let v = v as u32;
        if effective <= v {
            return (v - effective) as u8;
        }
    }
    0
}

/// Parses a SMILES string into a perceived [`MolGraph`].
///
/// Isotopes are dropped. Ring data, aromaticity and H counts come from the
/// perception pass.
///
/// # Errors
/// Returns the first [`SmilesError`] encountered.
pub fn parse(smiles: &str) -> Result<MolGraph, SmilesError> {
    Parser::new(smiles).run()
}

/// A pending ring-closure slot: the atom that opened the label and any bond
/// symbol that preceded the opening digit.
#[derive(Clone, Copy)]
struct RingSlot {
    atom: u32,
    /// `Some(order)` if an explicit bond symbol preceded the opening occurrence;
    /// `None` for the default (single/aromatic) bond.
    order: Option<u8>,
    /// True if the explicit preceding bond symbol was `:` (aromatic).
    aromatic_bond: bool,
}

/// The single-pass SMILES parser state machine.
struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
    builder: MolGraphBuilder,
    /// Parse-time aromatic flag per atom.
    aromatic: Vec<bool>,
    /// Summed bond order per atom (aromatic bond counts as 1), for implicit H.
    bond_sum: Vec<u32>,
    /// Element of each bare organic atom (for implicit H); `None` for bracket atoms.
    implicit_element: Vec<Option<u8>>,
    /// Explicit H per atom (nonzero only for bracket atoms).
    explicit_h: Vec<u8>,
    /// The atom the next bond attaches to; `None` at the start or after `.`.
    prev: Option<u32>,
    /// Per open `(`: the `prev` to return to, the `(` byte position, and the atom
    /// count at the `(` (to refuse an empty branch).
    branch_stack: Vec<(u32, usize, usize)>,
    /// For each atom, the earlier atom its chain bond joins (`u32::MAX` for none).
    /// Chain bonds always join a new atom to an earlier one, so only a ring
    /// closure can duplicate a bond.
    chain_parent: Vec<u32>,
    /// Every ring-closure bond added so far, as `(a, b)`.
    closures: Vec<(u32, u32)>,
    /// Bond symbol waiting for the next atom or ring closure.
    pending_bond: Option<PendingBond>,
    /// Open ring-closure slots indexed by label (0..=99).
    ring_slots: [Option<RingSlot>; 100],
    /// Byte position where each ring label was opened.
    ring_open_pos: [usize; 100],
}

/// A bond symbol seen but not yet attached to its right-hand atom.
#[derive(Clone, Copy)]
struct PendingBond {
    /// Byte position of the symbol.
    pos: usize,
    /// The bond order (`SINGLE` for `.`, which `disconnect` marks).
    order: u8,
    /// True for `:`.
    aromatic: bool,
    /// True for `.`: a disconnection, not a bond.
    disconnect: bool,
}

impl<'a> Parser<'a> {
    fn new(smiles: &'a str) -> Self {
        Parser {
            bytes: smiles.as_bytes(),
            pos: 0,
            builder: MolGraphBuilder::new(),
            aromatic: Vec::new(),
            bond_sum: Vec::new(),
            implicit_element: Vec::new(),
            explicit_h: Vec::new(),
            prev: None,
            branch_stack: Vec::new(),
            chain_parent: Vec::new(),
            closures: Vec::new(),
            pending_bond: None,
            ring_slots: [None; 100],
            ring_open_pos: [0usize; 100],
        }
    }

    /// Drives the parse to completion.
    fn run(mut self) -> Result<MolGraph, SmilesError> {
        while self.pos < self.bytes.len() {
            let c = self.bytes[self.pos];
            match c {
                b'(' => self.open_branch()?,
                b')' => self.close_branch()?,
                b'[' => {
                    let atom = self.parse_bracket_atom()?;
                    self.attach_atom(atom)?;
                }
                b'-' | b'=' | b'#' | b':' | b'.' => self.parse_bond_symbol()?,
                b'/' | b'\\' => {
                    // Stereo bond markers are ignored; the bond stays the default.
                    self.pos += 1;
                }
                b'0'..=b'9' => self.parse_ring_closure()?,
                b'%' => self.parse_percent_ring_closure()?,
                _ if is_organic_start(c) => {
                    let atom = self.parse_organic_atom()?;
                    self.attach_atom(atom)?;
                }
                _ => {
                    return Err(SmilesError::UnexpectedChar {
                        pos: self.pos,
                        ch: c as char,
                    });
                }
            }
        }

        if let Some(&(_, pos, _)) = self.branch_stack.first() {
            return Err(SmilesError::UnbalancedOpen { pos });
        }
        // A trailing bond symbol or `.`: `CC=`.
        if let Some(pb) = self.pending_bond {
            return Err(SmilesError::DanglingBond { pos: pb.pos });
        }
        // Label 0 is valid (digits 0-9 and %00-%99), so it is checked too.
        for label in 0..=99u16 {
            if self.ring_slots[label as usize].is_some() {
                return Err(SmilesError::UnclosedRing {
                    label,
                    pos: self.ring_open_pos[label as usize],
                });
            }
        }
        if self.builder.atom_count() == 0 {
            return Err(SmilesError::Empty);
        }

        self.finish()
    }

    /// Sets initial H counts, builds the [`MolGraph`], and runs perception.
    ///
    /// Initial implicit H uses the parse-time aromatic flag. Perception re-derives
    /// aromaticity and re-finalises bare-atom H, so Kekulé and aromatic spellings of
    /// one molecule converge. Bracket atoms keep their written H count.
    fn finish(mut self) -> Result<MolGraph, SmilesError> {
        let n = self.builder.atom_count();
        for i in 0..n {
            let extra = match self.implicit_element[i] {
                Some(element) => implicit_h(element, self.bond_sum[i], self.aromatic[i]),
                None => 0, // bracket atom: explicit H already accounted for
            };
            let total = self.explicit_h[i].saturating_add(extra);
            self.builder.set_h_count(i as u32, total);
        }
        let mut mol = self.builder.finish();

        // Bracket atoms have a fixed H count; perception must not recompute it.
        let h_locked: Vec<bool> = self.implicit_element.iter().map(|e| e.is_none()).collect();
        crate::perception::perceive(&mut mol, &h_locked, &self.implicit_element).map_err(|_| {
            SmilesError::TooLarge {
                what: "ring-perception steps",
                limit: crate::perception::MAX_PERCEPTION_STEPS,
            }
        })?;
        Ok(mol)
    }

    /// Adds `atom` to the graph and bonds it to `prev` unless a `.` is pending.
    fn attach_atom(&mut self, atom: ParsedAtom) -> Result<(), SmilesError> {
        if self.builder.atom_count() >= MAX_ATOMS {
            return Err(SmilesError::TooLarge {
                what: "atoms",
                limit: MAX_ATOMS,
            });
        }
        let idx = self
            .builder
            .add_atom(atom.element, atom.charge, 0, atom.flags);
        self.chain_parent.push(u32::MAX);
        self.aromatic.push(atom.aromatic);
        self.bond_sum.push(0);
        self.implicit_element.push(atom.implicit_element);
        self.explicit_h.push(atom.explicit_h);

        let pending = self.pending_bond.take();
        if let Some(pb) = pending {
            if pb.disconnect {
                self.prev = Some(idx);
                return Ok(());
            }
        }

        if let Some(prev) = self.prev {
            let (order, arom_bond) = match pending {
                Some(pb) => (pb.order, pb.aromatic),
                None => {
                    // Implicit bond: aromatic iff both endpoints are aromatic.
                    if self.aromatic[prev as usize] && atom.aromatic {
                        (bond_order::AROMATIC, true)
                    } else {
                        (bond_order::SINGLE, false)
                    }
                }
            };
            self.add_bond(prev, idx, order, arom_bond);
            self.chain_parent[idx as usize] = prev;
        }
        self.prev = Some(idx);
        Ok(())
    }

    /// Adds a bond with its aromatic flag and updates `bond_sum`.
    fn add_bond(&mut self, a: u32, b: u32, order: u8, aromatic_bond: bool) {
        let mut flags = 0u8;
        if aromatic_bond {
            flags |= bond_flags::AROMATIC;
        }
        self.builder.add_bond(a, b, order, flags);
        // Aromatic bonds count 1; `implicit_h` adds the aromatic unit.
        let contribution = match order {
            bond_order::SINGLE => 1,
            bond_order::DOUBLE => 2,
            bond_order::TRIPLE => 3,
            bond_order::AROMATIC => 1,
            _ => 1,
        };
        self.bond_sum[a as usize] += contribution;
        self.bond_sum[b as usize] += contribution;
    }

    /// Handles `(`: pushes `prev` so `)` can return to it.
    fn open_branch(&mut self) -> Result<(), SmilesError> {
        // `C.(C)`: a disconnection cannot lead into a branch.
        if let Some(pb) = self.pending_bond {
            if pb.disconnect {
                return Err(SmilesError::DanglingBond { pos: pb.pos });
            }
        }
        match self.prev {
            Some(p) => {
                self.branch_stack
                    .push((p, self.pos, self.builder.atom_count()));
                self.pos += 1;
                Ok(())
            }
            None => Err(SmilesError::DanglingBond { pos: self.pos }),
        }
    }

    /// Handles `)`: returns to the atom that opened the branch.
    fn close_branch(&mut self) -> Result<(), SmilesError> {
        // `C(C=)C`: the bond inside the branch has no atom after it.
        if let Some(pb) = self.pending_bond {
            return Err(SmilesError::DanglingBond { pos: pb.pos });
        }
        match self.branch_stack.pop() {
            Some((_, open, atoms_at_open)) if self.builder.atom_count() == atoms_at_open => {
                Err(SmilesError::EmptyBranch { pos: open })
            }
            Some((atom, _, _)) => {
                self.prev = Some(atom);
                self.pos += 1;
                Ok(())
            }
            None => Err(SmilesError::UnbalancedClose { pos: self.pos }),
        }
    }

    /// Handles a bond symbol `- = # : .`: records it for the next atom or ring
    /// closure.
    fn parse_bond_symbol(&mut self) -> Result<(), SmilesError> {
        let c = self.bytes[self.pos];
        let pos = self.pos;
        let (order, aromatic, disconnect) = match c {
            b'-' => (bond_order::SINGLE, false, false),
            b'=' => (bond_order::DOUBLE, false, false),
            b'#' => (bond_order::TRIPLE, false, false),
            b':' => (bond_order::AROMATIC, true, false),
            b'.' => (bond_order::SINGLE, false, true),
            _ => unreachable!("parse_bond_symbol called on non-bond byte"),
        };
        let pb = PendingBond {
            pos,
            order,
            aromatic,
            disconnect,
        };
        // A leading "=C" or ".C".
        if self.prev.is_none() {
            return Err(SmilesError::DanglingBond { pos });
        }
        // Two symbols in a row (`C-=C`, `C=.C`) are refused, not last-one-wins.
        if self.pending_bond.is_some() {
            return Err(SmilesError::UnexpectedChar { pos, ch: c as char });
        }
        self.pending_bond = Some(pb);
        self.pos += 1;
        Ok(())
    }

    /// Parses a single-digit ring-closure occurrence.
    fn parse_ring_closure(&mut self) -> Result<(), SmilesError> {
        let start = self.pos;
        let label = (self.bytes[self.pos] - b'0') as u16;
        self.pos += 1;
        self.close_or_open_ring(label, start)
    }

    /// Parses `%nn` ring closures (two decimal digits after `%`).
    fn parse_percent_ring_closure(&mut self) -> Result<(), SmilesError> {
        let start = self.pos;
        if self.pos + 2 >= self.bytes.len() {
            return Err(SmilesError::BadPercentLabel { pos: start });
        }
        let d1 = self.bytes[self.pos + 1];
        let d2 = self.bytes[self.pos + 2];
        if !d1.is_ascii_digit() || !d2.is_ascii_digit() {
            return Err(SmilesError::BadPercentLabel { pos: start });
        }
        let label = ((d1 - b'0') as u16) * 10 + (d2 - b'0') as u16;
        self.pos += 3;
        self.close_or_open_ring(label, start)
    }

    /// Closes the ring label if open (adding the bond), else opens it on `prev`.
    fn close_or_open_ring(&mut self, label: u16, at: usize) -> Result<(), SmilesError> {
        let prev = match self.prev {
            Some(p) => p,
            None => return Err(SmilesError::DanglingBond { pos: at }),
        };
        // A bond symbol may precede a ring digit (`C=1...1`); `.` may not (`C.1CC1`).
        let pending = self.pending_bond.take();
        let (pending_order, pending_arom) = match pending {
            Some(pb) if pb.disconnect => return Err(SmilesError::DanglingBond { pos: pb.pos }),
            Some(pb) => (Some(pb.order), pb.aromatic),
            None => (None, false),
        };

        let slot = self.ring_slots[label as usize].take();
        match slot {
            None => {
                self.ring_slots[label as usize] = Some(RingSlot {
                    atom: prev,
                    order: pending_order,
                    aromatic_bond: pending_arom,
                });
                self.ring_open_pos[label as usize] = at;
                Ok(())
            }
            Some(open) => {
                if open.atom == prev {
                    return Err(SmilesError::InvalidRingClosure { label, pos: at });
                }
                let explicit_on_either = open.order.is_some() || pending_order.is_some();
                let (mut order, mut arom) =
                    reconcile_ring_bond(&open, pending_order, pending_arom, label, at)?;

                // Implicit closure bond: aromatic iff both endpoints are aromatic,
                // as for any implicit bond (`c1ccccc1`).
                if !explicit_on_either
                    && self.aromatic[open.atom as usize]
                    && self.aromatic[prev as usize]
                {
                    order = bond_order::AROMATIC;
                    arom = true;
                }

                // A duplicate bond (`C12C12`) gets a clean error, not a builder panic.
                if self.bond_exists(open.atom, prev) {
                    return Err(SmilesError::InvalidRingClosure { label, pos: at });
                }
                if self.closures.len() >= MAX_RINGS {
                    return Err(SmilesError::TooLarge {
                        what: "ring closures",
                        limit: MAX_RINGS,
                    });
                }
                self.add_bond(open.atom, prev, order, arom);
                self.closures.push((open.atom, prev));
                Ok(())
            }
        }
    }

    /// True if a bond already connects `a` and `b`. Every bond is a chain bond or a
    /// ring closure, so checking both is exact, and only closures are scanned (a
    /// scan of all bonds is quadratic on `C1CC1C1CC1…`).
    fn bond_exists(&self, a: u32, b: u32) -> bool {
        self.chain_parent[a as usize] == b
            || self.chain_parent[b as usize] == a
            || self
                .closures
                .iter()
                .any(|&(x, y)| (x == a && y == b) || (x == b && y == a))
    }

    /// Parses a bare organic-subset atom (`Cl` and `Br` are the two-letter ones).
    fn parse_organic_atom(&mut self) -> Result<ParsedAtom, SmilesError> {
        let start = self.pos;
        let c = self.bytes[self.pos];
        if c == b'C' && self.peek(1) == Some(b'l') {
            self.pos += 2;
            return Ok(self.bare_atom(17, false));
        }
        if c == b'B' && self.peek(1) == Some(b'r') {
            self.pos += 2;
            return Ok(self.bare_atom(35, false));
        }
        let (element, aromatic) = match c {
            b'B' => (5, false),
            b'C' => (6, false),
            b'N' => (7, false),
            b'O' => (8, false),
            b'P' => (15, false),
            b'S' => (16, false),
            b'F' => (9, false),
            b'I' => (53, false),
            b'b' => (5, true),
            b'c' => (6, true),
            b'n' => (7, true),
            b'o' => (8, true),
            b'p' => (15, true),
            b's' => (16, true),
            _ => {
                return Err(SmilesError::UnknownElement {
                    pos: start,
                    symbol: (c as char).to_string(),
                });
            }
        };
        self.pos += 1;
        Ok(self.bare_atom(element, aromatic))
    }

    /// Builds a [`ParsedAtom`] for a bare organic atom; implicit H is set in
    /// `finish`.
    fn bare_atom(&self, element: u8, aromatic: bool) -> ParsedAtom {
        let mut flags = 0u8;
        if aromatic {
            flags |= atom_flags::AROMATIC;
        }
        ParsedAtom {
            element,
            charge: 0,
            aromatic,
            flags,
            explicit_h: 0,
            implicit_element: Some(element),
        }
    }

    /// Parses a bracket atom `[isotope? symbol chiral? hcount? charge? class?]`.
    ///
    /// Fields follow the OpenSMILES order and each may appear at most once, so
    /// `[CH2H3]`, `[C+-]` and `[C+H]` are refused. The isotope (up to 999),
    /// chirality (`@`, `@@`) and atom class (`:n`) are parsed and dropped. The H
    /// count takes up to two digits; the charge is a sign, a repeated sign, or a
    /// sign with up to two digits, at most [`MAX_CHARGE`] in magnitude.
    fn parse_bracket_atom(&mut self) -> Result<ParsedAtom, SmilesError> {
        let open = self.pos;
        self.pos += 1;
        // Isotope: parsed and dropped.
        let iso_start = self.pos;
        while self.peek(0).map(|b| b.is_ascii_digit()).unwrap_or(false) {
            self.pos += 1;
        }
        if self.pos - iso_start > 3 {
            return Err(SmilesError::ValueOutOfRange {
                pos: iso_start,
                what: "isotope",
                max: 999,
            });
        }

        let (element, aromatic) = self.parse_bracket_element(open)?;

        let mut explicit_h = 0u8;
        let mut charge: i8 = 0;

        // The remaining fields, in order. `stage` is the last field read; a field
        // is accepted only after every earlier one, so none can repeat.
        const CHIRAL: u8 = 1;
        const HCOUNT: u8 = 2;
        const CHARGE: u8 = 3;
        const CLASS: u8 = 4;
        let mut stage = 0u8;
        loop {
            let Some(b) = self.peek(0) else {
                return Err(SmilesError::UnterminatedBracket { pos: open });
            };
            let field = match b {
                b']' => {
                    self.pos += 1;
                    break;
                }
                b'@' => CHIRAL,
                b'H' => HCOUNT,
                b'+' | b'-' => CHARGE,
                b':' => CLASS,
                _ => 0,
            };
            if field <= stage {
                return Err(SmilesError::UnexpectedChar {
                    pos: self.pos,
                    ch: b as char,
                });
            }
            stage = field;
            match field {
                CHIRAL => {
                    // Consume only `@` or `@@`: in `[C@H]` the `H` is the H count.
                    // Extended designators (`@TH1`, …) are unsupported and their
                    // letters raise an error rather than corrupt the H count.
                    self.pos += 1;
                    if self.peek(0) == Some(b'@') {
                        self.pos += 1;
                    }
                }
                HCOUNT => {
                    self.pos += 1;
                    // Bare `H` means 1.
                    let mut count = 1u8;
                    if let Some(d) = self.peek(0) {
                        if d.is_ascii_digit() {
                            count = d - b'0';
                            self.pos += 1;
                            if let Some(d2) = self.peek(0) {
                                if d2.is_ascii_digit() {
                                    count = count * 10 + (d2 - b'0');
                                    self.pos += 1;
                                }
                            }
                        }
                    }
                    explicit_h = count;
                }
                CHARGE => {
                    let at = self.pos;
                    let sign: i32 = if b == b'+' { 1 } else { -1 };
                    self.pos += 1;
                    // `+2`, `++`, or a lone sign.
                    let mut mag: u32 = 1;
                    if self.peek(0).is_some_and(|d| d.is_ascii_digit()) {
                        mag = 0;
                        for _ in 0..2 {
                            match self.peek(0) {
                                Some(d) if d.is_ascii_digit() => {
                                    mag = mag * 10 + (d - b'0') as u32;
                                    self.pos += 1;
                                }
                                _ => break,
                            }
                        }
                    } else {
                        while self.peek(0) == Some(b) && mag <= MAX_CHARGE {
                            mag += 1;
                            self.pos += 1;
                        }
                    }
                    if mag > MAX_CHARGE {
                        return Err(SmilesError::ValueOutOfRange {
                            pos: at,
                            what: "charge",
                            max: MAX_CHARGE,
                        });
                    }
                    charge = (sign * mag as i32) as i8;
                }
                CLASS => {
                    // Atom class `:nn`, at least one digit; dropped.
                    self.pos += 1;
                    let digits = self.pos;
                    while self.peek(0).map(|x| x.is_ascii_digit()).unwrap_or(false) {
                        self.pos += 1;
                    }
                    if self.pos == digits {
                        return Err(SmilesError::UnexpectedChar {
                            pos: digits - 1,
                            ch: ':',
                        });
                    }
                }
                _ => unreachable!("every other byte was refused above"),
            }
        }

        let mut flags = 0u8;
        if aromatic {
            flags |= atom_flags::AROMATIC;
        }
        Ok(ParsedAtom {
            element,
            charge,
            aromatic,
            flags,
            explicit_h,
            implicit_element: None,
        })
    }

    /// Parses a bracket element symbol as `(atomic_number, is_aromatic)`: `*`
    /// (atomic number 0), an upper-case symbol, or an aromatic lower-case symbol.
    fn parse_bracket_element(&mut self, open: usize) -> Result<(u8, bool), SmilesError> {
        let start = self.pos;
        let Some(c) = self.peek(0) else {
            return Err(SmilesError::UnterminatedBracket { pos: open });
        };

        if c == b'*' {
            self.pos += 1;
            return Ok((0, false));
        }

        if c.is_ascii_uppercase() {
            // Two-letter symbol first.
            if let Some(c2) = self.peek(1) {
                if c2.is_ascii_lowercase() {
                    if let Some(num) = two_letter_element(c, c2) {
                        self.pos += 2;
                        return Ok((num, false));
                    }
                }
            }
            if let Some(num) = one_letter_aliphatic(c) {
                self.pos += 1;
                return Ok((num, false));
            }
            return Err(SmilesError::UnknownElement {
                pos: start,
                symbol: (c as char).to_string(),
            });
        }

        if c.is_ascii_lowercase() {
            if let Some(num) = aromatic_lower(c) {
                self.pos += 1;
                return Ok((num, true));
            }
            return Err(SmilesError::UnknownElement {
                pos: start,
                symbol: (c as char).to_string(),
            });
        }

        // Not a letter or '*': the bracket has no element.
        Err(SmilesError::MissingElement { pos: start })
    }

    /// Returns the byte `offset` positions ahead of the cursor, if in range.
    #[inline]
    fn peek(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }
}

/// A fully parsed atom, ready to be wired into the graph.
struct ParsedAtom {
    element: u8,
    charge: i8,
    aromatic: bool,
    flags: u8,
    explicit_h: u8,
    /// `Some(element)` for a bare organic atom (takes implicit H); `None` for a
    /// bracket atom.
    implicit_element: Option<u8>,
}

/// Reconciles bond symbols on the two halves of a ring closure.
///
/// Two explicit orders must agree (else [`SmilesError::RingBondMismatch`]); one
/// explicit order wins; none gives `(SINGLE, false)`, which the caller
/// (`close_or_open_ring`) upgrades to aromatic when both endpoints are aromatic.
fn reconcile_ring_bond(
    open: &RingSlot,
    close_order: Option<u8>,
    close_arom: bool,
    label: u16,
    pos: usize,
) -> Result<(u8, bool), SmilesError> {
    match (open.order, close_order) {
        (Some(o1), Some(o2)) => {
            if o1 != o2 {
                return Err(SmilesError::RingBondMismatch { label, pos });
            }
            Ok((o1, open.aromatic_bond || close_arom))
        }
        (Some(o1), None) => Ok((o1, open.aromatic_bond)),
        (None, Some(o2)) => Ok((o2, close_arom)),
        (None, None) => Ok((bond_order::SINGLE, false)),
    }
}

/// True if byte `c` can begin a bare organic-subset atom token.
fn is_organic_start(c: u8) -> bool {
    matches!(
        c,
        b'B' | b'C'
            | b'N'
            | b'O'
            | b'P'
            | b'S'
            | b'F'
            | b'I'
            | b'b'
            | b'c'
            | b'n'
            | b'o'
            | b'p'
            | b's'
    )
}

/// Maps a two-letter element symbol to its atomic number, over the full periodic
/// table, so a bracket atom of any element parses and the molecule is indexed.
///
/// Shared with the SMARTS parser so both read `[Sn]`, `[As]`, … the same way.
pub(crate) fn two_letter_element(c1: u8, c2: u8) -> Option<u8> {
    Some(match (c1, c2) {
        (b'H', b'e') => 2,
        (b'L', b'i') => 3,
        (b'B', b'e') => 4,
        (b'N', b'e') => 10,
        (b'N', b'a') => 11,
        (b'M', b'g') => 12,
        (b'A', b'l') => 13,
        (b'S', b'i') => 14,
        (b'C', b'l') => 17,
        (b'A', b'r') => 18,
        (b'C', b'a') => 20,
        (b'S', b'c') => 21,
        (b'T', b'i') => 22,
        (b'C', b'r') => 24,
        (b'M', b'n') => 25,
        (b'F', b'e') => 26,
        (b'C', b'o') => 27,
        (b'N', b'i') => 28,
        (b'C', b'u') => 29,
        (b'Z', b'n') => 30,
        (b'G', b'a') => 31,
        (b'G', b'e') => 32,
        (b'A', b's') => 33,
        (b'S', b'e') => 34,
        (b'B', b'r') => 35,
        (b'K', b'r') => 36,
        (b'R', b'b') => 37,
        (b'S', b'r') => 38,
        (b'Z', b'r') => 40,
        (b'N', b'b') => 41,
        (b'M', b'o') => 42,
        (b'T', b'c') => 43,
        (b'R', b'u') => 44,
        (b'R', b'h') => 45,
        (b'P', b'd') => 46,
        (b'A', b'g') => 47,
        (b'C', b'd') => 48,
        (b'I', b'n') => 49,
        (b'S', b'n') => 50,
        (b'S', b'b') => 51,
        (b'T', b'e') => 52,
        (b'X', b'e') => 54,
        (b'C', b's') => 55,
        (b'B', b'a') => 56,
        (b'L', b'a') => 57,
        (b'C', b'e') => 58,
        (b'P', b'r') => 59,
        (b'N', b'd') => 60,
        (b'P', b'm') => 61,
        (b'S', b'm') => 62,
        (b'E', b'u') => 63,
        (b'G', b'd') => 64,
        (b'T', b'b') => 65,
        (b'D', b'y') => 66,
        (b'H', b'o') => 67,
        (b'E', b'r') => 68,
        (b'T', b'm') => 69,
        (b'Y', b'b') => 70,
        (b'L', b'u') => 71,
        (b'H', b'f') => 72,
        (b'T', b'a') => 73,
        (b'R', b'e') => 75,
        (b'O', b's') => 76,
        (b'I', b'r') => 77,
        (b'P', b't') => 78,
        (b'A', b'u') => 79,
        (b'H', b'g') => 80,
        (b'T', b'l') => 81,
        (b'P', b'b') => 82,
        (b'B', b'i') => 83,
        (b'P', b'o') => 84,
        (b'A', b't') => 85,
        (b'R', b'n') => 86,
        (b'F', b'r') => 87,
        (b'R', b'a') => 88,
        (b'A', b'c') => 89,
        (b'T', b'h') => 90,
        (b'P', b'a') => 91,
        (b'N', b'p') => 93,
        (b'P', b'u') => 94,
        (b'A', b'm') => 95,
        (b'C', b'm') => 96,
        (b'B', b'k') => 97,
        (b'C', b'f') => 98,
        (b'E', b's') => 99,
        (b'F', b'm') => 100,
        (b'M', b'd') => 101,
        (b'N', b'o') => 102,
        (b'L', b'r') => 103,
        (b'R', b'f') => 104,
        (b'D', b'b') => 105,
        (b'S', b'g') => 106,
        (b'B', b'h') => 107,
        (b'H', b's') => 108,
        (b'M', b't') => 109,
        (b'D', b's') => 110,
        (b'R', b'g') => 111,
        (b'C', b'n') => 112,
        (b'N', b'h') => 113,
        (b'F', b'l') => 114,
        (b'M', b'c') => 115,
        (b'L', b'v') => 116,
        (b'T', b's') => 117,
        (b'O', b'g') => 118,
        _ => return None,
    })
}

/// Maps a single upper-case letter to an element's atomic number.
fn one_letter_aliphatic(c: u8) -> Option<u8> {
    match c {
        b'B' => Some(5),
        b'C' => Some(6),
        b'N' => Some(7),
        b'O' => Some(8),
        b'P' => Some(15),
        b'S' => Some(16),
        b'F' => Some(9),
        b'I' => Some(53),
        b'H' => Some(1),
        b'K' => Some(19),
        b'V' => Some(23),
        b'Y' => Some(39),
        b'W' => Some(74),
        b'U' => Some(92),
        _ => None,
    }
}

/// Maps an aromatic letter `b c n o p s` to its atomic number.
fn aromatic_lower(c: u8) -> Option<u8> {
    match c {
        b'b' => Some(5),
        b'c' => Some(6),
        b'n' => Some(7),
        b'o' => Some(8),
        b'p' => Some(15),
        b's' => Some(16),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bond_order;

    fn p(s: &str) -> MolGraph {
        parse(s).unwrap_or_else(|e| panic!("parse({s:?}) failed: {e}"))
    }

    /// `(element, aromatic)` per atom.
    fn atoms(g: &MolGraph) -> Vec<(u8, bool)> {
        (0..g.atom_count() as u32)
            .map(|i| (g.element[i as usize], g.is_aromatic(i)))
            .collect()
    }

    #[test]
    fn ethanol_cco() {
        let g = p("CCO");
        assert_eq!(g.atom_count(), 3);
        assert_eq!(g.bond_count(), 2);
        assert_eq!(g.element, vec![6, 6, 8]);
        assert_eq!(g.h_count, vec![3, 2, 1]);
        assert_eq!(g.degree, vec![1, 2, 1]);
        assert!(g.bonds.iter().all(|b| b.order == bond_order::SINGLE));
    }

    #[test]
    fn benzene_is_six_aromatic_carbons_in_a_ring() {
        let g = p("c1ccccc1");
        assert_eq!(g.atom_count(), 6);
        // Five chain bonds plus one ring closure.
        assert_eq!(g.bond_count(), 6);
        assert!(atoms(&g).iter().all(|&(e, a)| e == 6 && a));
        assert_eq!(g.h_count, vec![1; 6]);
        assert_eq!(g.degree, vec![2; 6]);
        assert!(g
            .bonds
            .iter()
            .all(|b| b.order == bond_order::AROMATIC && b.is_aromatic()));
    }

    #[test]
    fn pyridine_has_one_aromatic_nitrogen() {
        let g = p("c1ccncc1");
        assert_eq!(g.atom_count(), 6);
        assert_eq!(g.bond_count(), 6);
        let n_aromatic = atoms(&g).iter().filter(|&&(e, a)| e == 7 && a).count();
        assert_eq!(n_aromatic, 1);
        // Two aromatic ring bonds plus the aromatic unit fill valence 3: no H.
        let n_idx = (0..g.atom_count()).find(|&i| g.element[i] == 7).unwrap();
        assert_eq!(g.h_count[n_idx], 0);
    }

    #[test]
    fn pyrrole_nh_uses_bracket_h() {
        let g = p("c1cc[nH]c1");
        assert_eq!(g.atom_count(), 5);
        let n_idx = (0..g.atom_count()).find(|&i| g.element[i] == 7).unwrap();
        assert!(g.is_aromatic(n_idx as u32));
        assert_eq!(g.h_count[n_idx], 1);
    }

    #[test]
    fn branches_build_correct_tree() {
        // Isobutane: the central atom is index 1.
        let g = p("CC(C)C");
        assert_eq!(g.atom_count(), 4);
        assert_eq!(g.bond_count(), 3);
        assert_eq!(g.degree, vec![1, 3, 1, 1]);
    }

    #[test]
    fn nested_branches_and_double_bonds() {
        // Isobutyric acid.
        let g = p("CC(C)C(=O)O");
        assert_eq!(g.atom_count(), 6);
        assert_eq!(g.element, vec![6, 6, 6, 6, 8, 8]);
        let dbl = g
            .bonds
            .iter()
            .filter(|b| b.order == bond_order::DOUBLE)
            .count();
        assert_eq!(dbl, 1);
        // Carbonyl C (3), carbonyl O (4), hydroxyl O (5).
        assert_eq!(g.degree[3], 3);
        assert_eq!(g.h_count[4], 0);
        assert_eq!(g.h_count[5], 1);
    }

    #[test]
    fn ring_closure_two_digit_percent() {
        let g = p("C%10CCCC%10");
        assert_eq!(g.atom_count(), 5);
        assert_eq!(g.bond_count(), 5);
        assert_eq!(g.degree, vec![2, 2, 2, 2, 2]);
    }

    #[test]
    fn explicit_charge_and_bracket() {
        let g = p("[NH4+]");
        assert_eq!(g.atom_count(), 1);
        assert_eq!(g.element[0], 7);
        assert_eq!(g.h_count[0], 4);
        assert_eq!(g.charge[0], 1);
    }

    #[test]
    fn double_charge_forms() {
        let a = p("[Mg+2]");
        let b = p("[Mg++]");
        assert_eq!(a.charge[0], 2);
        assert_eq!(b.charge[0], 2);
        assert_eq!(a.element[0], 12);
        let c = p("[O-2]");
        assert_eq!(c.charge[0], -2);
    }

    #[test]
    fn isotope_and_chirality_are_tolerated() {
        let g = p("[13CH3]");
        assert_eq!(g.element[0], 6);
        assert_eq!(g.h_count[0], 3);
        let g2 = p("N[C@H](C)C(=O)O");
        assert_eq!(g2.atom_count(), 6);
    }

    #[test]
    fn chiral_carbon_h_is_not_eaten_by_at_sign() {
        // In `[C@H]` the `H` is the H count, not part of the chirality.
        let g = p("N[C@H](C)C(=O)O");
        assert_eq!(g.element[1], 6);
        assert_eq!(g.h_count[1], 1);
        let g2 = p("[C@@H](N)(C)O");
        assert_eq!(g2.element[0], 6);
        assert_eq!(g2.h_count[0], 1);
    }

    #[test]
    fn explicit_bond_on_ring_closure() {
        // Chain double bond; the closure bond 0-3 is an implicit single.
        let g = p("C1=CCC1");
        assert_eq!(g.atom_count(), 4);
        assert_eq!(g.bond_count(), 4);
        let dbl = g
            .bonds
            .iter()
            .filter(|b| b.order == bond_order::DOUBLE)
            .count();
        assert_eq!(dbl, 1);
        // Agreeing explicit orders on both halves: the closure is the double bond.
        let g2 = p("C=1CCC=1");
        let dbl2 = g2
            .bonds
            .iter()
            .filter(|b| b.order == bond_order::DOUBLE)
            .count();
        assert_eq!(dbl2, 1);
    }

    #[test]
    fn ring_bond_conflict_is_rejected() {
        assert!(matches!(
            parse("C=1CCC#1"),
            Err(SmilesError::RingBondMismatch { label: 1, .. })
        ));
    }

    #[test]
    fn aromatic_ring_closure_bond_is_aromatic_without_colon() {
        let g = p("c1ccccc1");
        assert!(g.bonds.iter().all(|b| b.is_aromatic()));
    }

    #[test]
    fn disconnection_creates_two_fragments() {
        let g = p("[Na+].[Cl-]");
        assert_eq!(g.atom_count(), 2);
        assert_eq!(g.bond_count(), 0);
        assert_eq!(g.charge, vec![1, -1]);
        assert_eq!(g.element, vec![11, 17]);
    }

    #[test]
    fn caffeine_parses_with_expected_counts() {
        let g = p("CN1C=NC2=C1C(=O)N(C)C(=O)N2C");
        assert_eq!(g.atom_count(), 14);
        let c = g.element.iter().filter(|&&e| e == 6).count();
        let n = g.element.iter().filter(|&&e| e == 7).count();
        let o = g.element.iter().filter(|&&e| e == 8).count();
        assert_eq!((c, n, o), (8, 4, 2));
        // 13 spanning-tree bonds plus 2 ring closures.
        assert_eq!(g.bond_count(), 15);
    }

    #[test]
    fn cl_and_br_two_letter_bare() {
        let g = p("CCCl");
        assert_eq!(g.element, vec![6, 6, 17]);
        assert_eq!(g.h_count, vec![3, 2, 0]);
        let g2 = p("BrCCBr");
        assert_eq!(g2.element, vec![35, 6, 6, 35]);
    }

    // --- Error cases ---

    #[test]
    fn rejects_unbalanced_parens() {
        assert!(matches!(
            parse("CC("),
            Err(SmilesError::UnbalancedOpen { .. })
        ));
        assert!(matches!(
            parse("CC)"),
            Err(SmilesError::UnbalancedClose { .. })
        ));
    }

    #[test]
    fn rejects_unclosed_ring() {
        assert!(matches!(
            parse("C1CCCC"),
            Err(SmilesError::UnclosedRing { label: 1, .. })
        ));
    }

    #[test]
    fn rejects_dangling_bond() {
        assert!(matches!(
            parse("=CC"),
            Err(SmilesError::DanglingBond { .. })
        ));
        assert!(matches!(
            parse("1CC"),
            Err(SmilesError::DanglingBond { .. })
        ));
    }

    #[test]
    fn rejects_unterminated_bracket() {
        assert!(matches!(
            parse("C[NH"),
            Err(SmilesError::UnterminatedBracket { .. })
        ));
    }

    #[test]
    fn rejects_empty() {
        assert!(matches!(parse(""), Err(SmilesError::Empty)));
    }

    /// Valid SMILES parse to the expected atom and bond counts, deterministically.
    #[test]
    fn batch_of_valid_smiles_all_parse() {
        let cases: &[(&str, usize, usize)] = &[
            ("C", 1, 0),
            ("O", 1, 0),
            ("CC", 2, 1),
            ("CCO", 3, 2),
            ("C=C", 2, 1),
            ("C#N", 2, 1),
            ("c1ccccc1", 6, 6),
            ("c1ccncc1", 6, 6),
            ("C1CCCCC1", 6, 6),
            ("CC(=O)O", 4, 3),
            ("CC(C)C", 4, 3),
            ("OC(=O)c1ccccc1", 9, 9),   // benzoic acid
            ("Clc1ccccc1", 7, 7),       // chlorobenzene
            ("c1ccc2ccccc2c1", 10, 11), // naphthalene (fused)
            ("[NH4+]", 1, 0),
            ("[Na+].[Cl-]", 2, 0),
            ("CN1C=NC2=C1C(=O)N(C)C(=O)N2C", 14, 15), // caffeine
        ];
        for &(s, na, nb) in cases {
            let g = parse(s).unwrap_or_else(|e| panic!("parse({s:?}) failed: {e}"));
            assert_eq!(g.atom_count(), na, "atom count for {s:?}");
            assert_eq!(g.bond_count(), nb, "bond count for {s:?}");
            let g2 = parse(s).unwrap();
            assert_eq!(g.atom_count(), g2.atom_count());
            assert_eq!(g.bond_count(), g2.bond_count());
            // CSR offsets end at 2 * bonds.
            assert_eq!(*g.nbr_offsets.last().unwrap() as usize, 2 * g.bond_count());
        }
    }
}
