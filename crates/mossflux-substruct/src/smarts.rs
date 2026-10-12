//! A SMARTS parser producing a [`QueryGraph`].
//!
//! SMARTS uses the SMILES topology syntax (chains, `()` branches, ring-closure
//! digits and `%nn`, `.` disconnection) with predicates in place of concrete atoms
//! and bonds. The topology state machine is the same as in [`crate::smiles`]. The
//! output has one [`Pred`] per query atom and one [`BondPred`] per query bond.
//!
//! # Atom primitives
//!
//! Outside brackets, a bare symbol is one primitive:
//!
//! * Aliphatic organic symbols `B C N O P S F Cl Br I` → [`Pred::Element`].
//! * Aromatic organic symbols `b c n o p s` → [`Pred::AromaticElement`].
//! * `*` → [`Pred::Any`].
//!
//! Inside `[...]`, primitives combine with the logical operators below:
//!
//! * Element symbol (aliphatic upper / aromatic lower) → element or aromatic-element.
//! * `*` → any; `a` → [`Pred::Aromatic`]; `A` → [`Pred::Aliphatic`].
//! * `#n` → [`Pred::AtomicNumber`] (any aromaticity).
//! * `R` → [`Pred::InRing`]; `R<n>` → [`Pred::RingCount`]; `r<n>` → [`Pred::RingSize`].
//! * `D<n>` → [`Pred::Degree`]; `H<n>` → [`Pred::TotalH`] (bare `H` is `H1`, except
//!   at the atom-symbol position, where `[H]` is hydrogen).
//! * `X<n>`, `h<n>`, `x<n>`, `v<n>` → [`Pred::TotalConnections`], [`Pred::ImplicitH`],
//!   [`Pred::RingBondCount`], [`Pred::Valence`] (bare forms mean 1).
//! * `+`, `-`, `++`, `--`, `+n`, `-n` → [`Pred::Charge`].
//! * `$(...)` → [`Pred::Recursive`], nested at most `MAX_REC_DEPTH` deep.
//!
//! # Bond primitives
//!
//! * `-` single, `=` double, `#` triple → [`BondPred::Order`].
//! * `:` aromatic → [`BondPred::Aromatic`]; `@` ring bond → [`BondPred::Ring`].
//! * `~` any → [`BondPred::Any`].
//! * No symbol → [`BondPred::SingleOrAromatic`], the SMARTS default.
//! * Bond expressions use the same operators as atoms.
//!
//! # Operator precedence
//!
//! Tightest first:
//!
//! 1. `!` — NOT.
//! 2. `&` — high-precedence AND, also used for juxtaposition (`[CH2]`).
//! 3. `,` — OR.
//! 4. `;` — low-precedence AND, so `[C,N;H1]` is `(C OR N) AND H1`.
//!
//! The grammar, loosest first:
//!
//! ```text
//! low_and := or       (';' or)*
//! or      := high_and (',' high_and)*
//! high_and:= unary    (('&')? unary)*      // '&' optional ⇒ juxtaposition
//! unary   := '!' unary | primitive
//! ```
//!
//! # Errors
//!
//! Any other input is a [`SmartsError`]; nothing is silently parsed into a
//! different meaning. Recognised but unsupported constructs (aromatic
//! `Se`/`Te`/`As`) fail with [`SmartsError::Unsupported`].
//!
//! Ignored, because they do not constrain a substructure: isotopes (`[13C]`),
//! atom maps (`[C:1]`), chirality (`@`, `@@`) and bond direction (`/`, `\`, read
//! as single).

use crate::graph::bond_order;
use crate::query::{BondPred, Pred, QueryGraph, QueryGraphBuilder};

/// Everything that can go wrong while parsing a SMARTS string.
///
/// Each variant carries the byte offset (into the original input) at or near
/// which the problem was detected.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum SmartsError {
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
    /// A bracket atom's expression was empty (e.g. `[]`) or an operator had no
    /// operand (e.g. `[C,]` or `[!]`).
    EmptyExpression {
        /// Byte offset within the bracket where an operand was expected.
        pos: usize,
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
    /// A bond symbol or ring-closure digit appeared with no preceding atom.
    DanglingBond {
        /// Byte offset of the bond/closure.
        pos: usize,
    },
    /// A ring-closure label was left open at end-of-input (only one occurrence).
    UnclosedRing {
        /// The ring-bond label (1–99) that was never closed.
        label: u16,
        /// Byte offset where the (unmatched) opening occurrence appeared.
        pos: usize,
    },
    /// A ring closure would connect an atom to itself or duplicate an existing
    /// bond between the same pair.
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
    /// A numeric primitive (`#`, `r`, `D`, `H` with a count, `+n`/`-n`) was
    /// missing the digits it required.
    MissingNumber {
        /// Byte offset of the primitive whose number is missing.
        pos: usize,
    },
    /// A recognised SMARTS construct that is not supported.
    Unsupported {
        /// Byte offset of the unsupported construct.
        pos: usize,
        /// A short human-readable description (e.g. `"aromatic Se/Te/As"`).
        what: &'static str,
    },
    /// A single atom or bond expression chained more logical terms (`,`, `;`,
    /// `&`, juxtaposition) than [`MAX_EXPR_TERMS`] allows.
    ///
    /// `N` terms build an `N`-deep predicate tree, and a deep enough tree
    /// overflows the stack on any recursive walk (an abort, not a panic).
    ExpressionTooComplex {
        /// Byte offset of the term that exceeded the limit.
        pos: usize,
        /// The limit that was exceeded ([`MAX_EXPR_TERMS`]).
        limit: usize,
    },
    /// The two halves of one ring closure gave different bond expressions
    /// (e.g. `C=1CCC#1`), which is contradictory.
    RingBondMismatch {
        /// The ring-bond label.
        label: u16,
        /// Byte offset of the second (conflicting) occurrence.
        pos: usize,
    },
    /// A branch `(...)` held no atom (e.g. `C()C`).
    EmptyBranch {
        /// Byte offset of the `(`.
        pos: usize,
    },
    /// A numeric primitive is outside its range: a count (`#`, `D`, `H`, `h`,
    /// `X`, `x`, `v`, `R`, `r`) above [`MAX_COUNT`], or a charge beyond
    /// ±[`MAX_CHARGE`]. Refused, not clamped: a clamped value is a different query.
    ValueOutOfRange {
        /// Byte offset of the primitive.
        pos: usize,
        /// The largest allowed magnitude.
        max: u32,
    },
    /// The query has more than [`MAX_QUERY_ATOMS`] atoms, counting every
    /// recursive `$(...)` environment.
    TooLarge {
        /// Byte offset of the atom that passed the limit.
        pos: usize,
        /// The limit ([`MAX_QUERY_ATOMS`]).
        limit: usize,
    },
    /// The string contained no atoms.
    Empty,
}

/// The largest value a count primitive may take.
///
/// Target counts are `u8` and saturate at 255 ("255 or more"). A query value of
/// at most 254 never equals a saturated count, so every comparison stays exact.
pub const MAX_COUNT: u32 = 254;

/// The largest charge magnitude a query may state; no target can carry more.
pub const MAX_CHARGE: u32 = crate::smiles::MAX_CHARGE;

/// The most atoms one query may hold, counting every recursive `$(...)` body.
///
/// The matchers recurse once per query atom, so this bounds their stack depth.
/// It fits a 2 MiB thread stack with margin and is more than twice the size of the
/// largest ChEMBL molecule, so any molecule still parses as a query.
pub const MAX_QUERY_ATOMS: usize = 2_048;

impl core::fmt::Display for SmartsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SmartsError::UnexpectedChar { pos, ch } => {
                write!(f, "unexpected character {ch:?} at byte {pos}")
            }
            SmartsError::UnterminatedBracket { pos } => {
                write!(f, "unterminated '[' bracket atom opened at byte {pos}")
            }
            SmartsError::EmptyExpression { pos } => {
                write!(f, "empty atom expression / missing operand near byte {pos}")
            }
            SmartsError::UnbalancedClose { pos } => {
                write!(f, "')' without matching '(' at byte {pos}")
            }
            SmartsError::UnbalancedOpen { pos } => {
                write!(f, "'(' opened at byte {pos} was never closed")
            }
            SmartsError::DanglingBond { pos } => {
                write!(
                    f,
                    "bond or ring-closure at byte {pos} has no preceding atom"
                )
            }
            SmartsError::UnclosedRing { label, pos } => {
                write!(
                    f,
                    "ring-closure label {label} opened at byte {pos} was never closed"
                )
            }
            SmartsError::InvalidRingClosure { label, pos } => {
                write!(
                    f,
                    "ring-closure label {label} at byte {pos} would self-bond or duplicate a bond"
                )
            }
            SmartsError::BadPercentLabel { pos } => {
                write!(f, "'%' at byte {pos} must be followed by two digits")
            }
            SmartsError::MissingNumber { pos } => {
                write!(
                    f,
                    "primitive at byte {pos} requires a number that is missing"
                )
            }
            SmartsError::Unsupported { pos, what } => {
                write!(f, "unsupported SMARTS construct ({what}) at byte {pos}")
            }
            SmartsError::ExpressionTooComplex { pos, limit } => {
                write!(
                    f,
                    "expression at byte {pos} chains more than {limit} logical terms"
                )
            }
            SmartsError::RingBondMismatch { label, pos } => {
                write!(f, "ring-closure label {label} given conflicting bond expressions (second at byte {pos})")
            }
            SmartsError::EmptyBranch { pos } => {
                write!(f, "branch opened at byte {pos} holds no atom")
            }
            SmartsError::ValueOutOfRange { pos, max } => {
                write!(f, "value at byte {pos} is out of range (at most {max})")
            }
            SmartsError::TooLarge { pos, limit } => {
                write!(
                    f,
                    "query passes {limit} atoms (counting recursive environments) at byte {pos}"
                )
            }
            SmartsError::Empty => write!(f, "empty SMARTS (no atoms)"),
        }
    }
}

impl std::error::Error for SmartsError {}

/// Parses a SMARTS string into a [`QueryGraph`].
///
/// Safe on untrusted input: every failure is a returned error.
///
/// # Errors
/// Returns the first [`SmartsError`] found. Inputs that could exhaust the stack
/// are refused: `$(...)` nested deeper than 10, an expression of more than
/// [`MAX_EXPR_TERMS`] terms, or more than [`MAX_QUERY_ATOMS`] atoms.
pub fn parse(smarts: &str) -> Result<QueryGraph, SmartsError> {
    parse_with_depth(smarts, 0, MAX_QUERY_ATOMS).map(|(q, _)| q)
}

/// Maximum nesting depth of recursive `$(...)` environments. Bounds the stack
/// use of this parser and of the recursive matcher.
const MAX_REC_DEPTH: usize = 10;

/// Maximum number of logical terms (`,`, `;`, `&`, juxtaposition) one atom or bond
/// expression may chain.
///
/// The operators left-fold, so `N` terms build an `N`-deep [`Pred`] / [`BondPred`]
/// tree, and every recursive walk of it (`Clone`, flattening) recurses `N` times.
/// Without a bound, a long `[C,C,...,C]` overflows the stack, which aborts the
/// process. Past the bound, parsing fails with [`SmartsError::ExpressionTooComplex`].
///
/// The value is far above real SMARTS and far below the depth that threatens the
/// stack, also at full `$(...)` nesting.
pub const MAX_EXPR_TERMS: usize = 512;

/// Parses `smarts` as a (sub-)pattern at recursion `depth` (0 at the top level).
/// `budget` is how many atoms it may add, recursive bodies included; the second
/// value returned is how many it used.
fn parse_with_depth(
    smarts: &str,
    depth: usize,
    budget: usize,
) -> Result<(QueryGraph, usize), SmartsError> {
    Parser::new(smarts, depth, budget).run()
}

/// A pending ring-closure slot: the atom that opened the label and the bond
/// expression (if any) that preceded the opening digit.
#[derive(Clone)]
struct RingSlot {
    atom: u32,
    /// `Some(pred)` if an explicit bond expression preceded the opening
    /// occurrence; `None` for the SMARTS-default (single-or-aromatic) bond.
    bond: Option<BondPred>,
    /// Byte position of the opening occurrence (for error reporting).
    pos: usize,
}

/// The single-pass SMARTS parser state machine.
struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
    builder: QueryGraphBuilder,
    /// The atom the next bond/atom attaches to, or `None` at the start.
    prev: Option<u32>,
    /// Per open `(`: the atom to return to, the `(` byte position, and the atom
    /// count at the `(` (to refuse an empty branch).
    branch_stack: Vec<(u32, usize, usize)>,
    /// Bond expression (or `.`) waiting for the next atom or ring closure;
    /// `None` means the default bond.
    pending_bond: Option<PendingBond>,
    /// Ring-closure slots indexed by label (0..=99). `None` = free.
    ring_slots: [Option<RingSlot>; 100],
    /// `$(...)` nesting depth (0 at top level), checked against [`MAX_REC_DEPTH`].
    depth: usize,
    /// Terms folded into the current expression, checked against
    /// [`MAX_EXPR_TERMS`]. This bounds the predicate tree's depth.
    expr_terms: usize,
    /// Atoms this parser may add, its recursive bodies included.
    budget: usize,
    /// Atoms added so far, its recursive bodies included.
    used: usize,
}

/// A bond expression seen but not yet attached to its right-hand atom.
#[derive(Clone)]
struct PendingBond {
    /// Byte position of the expression, for error reporting.
    pos: usize,
    /// The parsed bond predicate (ignored when `disconnect`).
    pred: BondPred,
    /// `true` for `.` — a disconnection, not a bond.
    disconnect: bool,
}

impl<'a> Parser<'a> {
    fn new(smarts: &'a str, depth: usize, budget: usize) -> Self {
        // `[None; 100]` needs `Copy`, which `RingSlot` is not.
        let ring_slots = std::array::from_fn(|_| None);
        Parser {
            bytes: smarts.as_bytes(),
            pos: 0,
            builder: QueryGraphBuilder::new(),
            prev: None,
            branch_stack: Vec::new(),
            pending_bond: None,
            ring_slots,
            depth,
            expr_terms: 0,
            budget,
            used: 0,
        }
    }

    /// Counts one folded term, failing past [`MAX_EXPR_TERMS`]. Called at every
    /// fold in both the atom and the bond grammar.
    #[inline]
    fn bump_expr_term(&mut self, pos: usize) -> Result<(), SmartsError> {
        self.expr_terms += 1;
        if self.expr_terms > MAX_EXPR_TERMS {
            return Err(SmartsError::ExpressionTooComplex {
                pos,
                limit: MAX_EXPR_TERMS,
            });
        }
        Ok(())
    }

    /// Drives the parse to completion.
    fn run(mut self) -> Result<(QueryGraph, usize), SmartsError> {
        while self.pos < self.bytes.len() {
            let c = self.bytes[self.pos];
            match c {
                b'(' => self.open_branch()?,
                b')' => self.close_branch()?,
                b'[' => {
                    let pred = self.parse_bracket_atom()?;
                    self.attach_atom(pred)?;
                }
                // Bond expressions. Outside brackets `-` is always a bond (charges
                // appear only inside brackets).
                b'-' | b'=' | b'#' | b':' | b'~' | b'@' | b'!' | b'/' | b'\\' | b'.' => {
                    self.parse_bond_expr_token()?;
                }
                b'0'..=b'9' => self.parse_ring_closure()?,
                b'%' => self.parse_percent_ring_closure()?,
                b'*' => {
                    self.pos += 1;
                    self.attach_atom(Pred::Any)?;
                }
                // Bare `a` / `A`: aromatic / aliphatic wildcards.
                b'a' => {
                    self.pos += 1;
                    self.attach_atom(Pred::Aromatic)?;
                }
                b'A' => {
                    self.pos += 1;
                    self.attach_atom(Pred::Aliphatic)?;
                }
                _ if is_organic_atom_start(c) => {
                    let pred = self.parse_bare_atom()?;
                    self.attach_atom(pred)?;
                }
                _ => {
                    return Err(SmartsError::UnexpectedChar {
                        pos: self.pos,
                        ch: c as char,
                    });
                }
            }
        }

        if let Some(&(_, pos, _)) = self.branch_stack.first() {
            return Err(SmartsError::UnbalancedOpen { pos });
        }
        // A bond expression (or `.`) with no atom after it: `CC=`, `CC.`.
        if let Some(pb) = &self.pending_bond {
            return Err(SmartsError::DanglingBond { pos: pb.pos });
        }
        // Label 0 is valid (digits 0-9 and %00-%99).
        for label in 0..=99u16 {
            if let Some(slot) = &self.ring_slots[label as usize] {
                return Err(SmartsError::UnclosedRing {
                    label,
                    pos: slot.pos,
                });
            }
        }
        if self.builder.atom_count() == 0 {
            return Err(SmartsError::Empty);
        }
        Ok((self.builder.finish(), self.used))
    }

    /// Appends a query atom and bonds it to `prev` with the pending bond, if any.
    fn attach_atom(&mut self, pred: Pred) -> Result<(), SmartsError> {
        if self.used >= self.budget {
            return Err(SmartsError::TooLarge {
                pos: self.pos,
                limit: MAX_QUERY_ATOMS,
            });
        }
        self.used += 1;
        let idx = self.builder.add_atom(pred);
        let pending = self.pending_bond.take();

        if let Some(pb) = &pending {
            if pb.disconnect {
                // `.` — no bond; the new atom starts a fresh fragment.
                self.prev = Some(idx);
                return Ok(());
            }
        }

        if let Some(prev) = self.prev {
            let bond_pred = match pending {
                Some(pb) => pb.pred,
                None => BondPred::SingleOrAromatic,
            };
            self.builder.add_bond(prev, idx, bond_pred);
        }
        self.prev = Some(idx);
        Ok(())
    }

    /// Handles `(` — push the current `prev` so `)` can return to it.
    fn open_branch(&mut self) -> Result<(), SmartsError> {
        // `C.(C)`: a disconnection cannot lead into a branch.
        if let Some(pb) = &self.pending_bond {
            if pb.disconnect {
                return Err(SmartsError::DanglingBond { pos: pb.pos });
            }
        }
        match self.prev {
            Some(p) => {
                self.branch_stack
                    .push((p, self.pos, self.builder.atom_count()));
                self.pos += 1;
                Ok(())
            }
            None => Err(SmartsError::DanglingBond { pos: self.pos }),
        }
    }

    /// Handles `)` — pop back to the atom that opened the branch.
    fn close_branch(&mut self) -> Result<(), SmartsError> {
        // `C(C=)C`: the bond inside the branch has no atom after it.
        if let Some(pb) = &self.pending_bond {
            return Err(SmartsError::DanglingBond { pos: pb.pos });
        }
        match self.branch_stack.pop() {
            Some((_, open, atoms_at_open)) if self.builder.atom_count() == atoms_at_open => {
                Err(SmartsError::EmptyBranch { pos: open })
            }
            Some((atom, _, _)) => {
                self.prev = Some(atom);
                self.pos += 1;
                Ok(())
            }
            None => Err(SmartsError::UnbalancedClose { pos: self.pos }),
        }
    }

    /// Parses a bond expression or `.` between atoms and stores it as pending.
    fn parse_bond_expr_token(&mut self) -> Result<(), SmartsError> {
        let pos = self.pos;
        // Refuse a leading bond (`=C`) and two in a row (`C=.C`).
        if self.prev.is_none() {
            return Err(SmartsError::DanglingBond { pos });
        }
        if self.pending_bond.is_some() {
            return Err(SmartsError::UnexpectedChar {
                pos,
                ch: self.bytes[pos] as char,
            });
        }
        if self.bytes[pos] == b'.' {
            self.pos += 1;
            self.pending_bond = Some(PendingBond {
                pos,
                pred: BondPred::Any,
                disconnect: true,
            });
            return Ok(());
        }
        let pred = self.parse_bond_expression()?;
        self.pending_bond = Some(PendingBond {
            pos,
            pred,
            disconnect: false,
        });
        Ok(())
    }

    /// Parses a single-digit ring-closure occurrence.
    fn parse_ring_closure(&mut self) -> Result<(), SmartsError> {
        let start = self.pos;
        let label = (self.bytes[self.pos] - b'0') as u16;
        self.pos += 1;
        self.close_or_open_ring(label, start)
    }

    /// Parses `%nn` ring closures (two decimal digits after `%`).
    fn parse_percent_ring_closure(&mut self) -> Result<(), SmartsError> {
        let start = self.pos;
        if self.pos + 2 >= self.bytes.len() {
            return Err(SmartsError::BadPercentLabel { pos: start });
        }
        let d1 = self.bytes[self.pos + 1];
        let d2 = self.bytes[self.pos + 2];
        if !d1.is_ascii_digit() || !d2.is_ascii_digit() {
            return Err(SmartsError::BadPercentLabel { pos: start });
        }
        let label = ((d1 - b'0') as u16) * 10 + (d2 - b'0') as u16;
        self.pos += 3;
        self.close_or_open_ring(label, start)
    }

    /// Closes ring label `label` if it is open (adding the bond), else opens it on
    /// the current atom.
    fn close_or_open_ring(&mut self, label: u16, at: usize) -> Result<(), SmartsError> {
        let prev = match self.prev {
            Some(p) => p,
            None => return Err(SmartsError::DanglingBond { pos: at }),
        };
        // A bond may precede a ring digit; a `.` may not (`C.1CC1`).
        let pending = self.pending_bond.take();
        let pending_bond_pred = match pending {
            Some(pb) if pb.disconnect => return Err(SmartsError::DanglingBond { pos: pb.pos }),
            Some(pb) => Some(pb.pred),
            None => None,
        };

        let slot = self.ring_slots[label as usize].take();
        match slot {
            None => {
                self.ring_slots[label as usize] = Some(RingSlot {
                    atom: prev,
                    bond: pending_bond_pred,
                    pos: at,
                });
                Ok(())
            }
            Some(open) => {
                if open.atom == prev {
                    return Err(SmartsError::InvalidRingClosure { label, pos: at });
                }
                if self.bond_exists(open.atom, prev) {
                    return Err(SmartsError::InvalidRingClosure { label, pos: at });
                }
                // If both halves give a bond expression, they must be equal
                // (`C=1CCC#1` is refused). Else use the one given, or the default.
                let bond_pred = match (open.bond, pending_bond_pred) {
                    (Some(a), Some(b)) if a != b => {
                        return Err(SmartsError::RingBondMismatch { label, pos: at })
                    }
                    (Some(b), _) => b,
                    (None, Some(b)) => b,
                    (None, None) => BondPred::SingleOrAromatic,
                };
                self.builder.add_bond(open.atom, prev, bond_pred);
                Ok(())
            }
        }
    }

    /// True if a bond already connects `a` and `b` in the query built so far.
    fn bond_exists(&self, a: u32, b: u32) -> bool {
        self.builder.has_bond(a, b)
    }

    /// Parses a bare (unbracketed) atom token into a [`Pred`].
    ///
    /// Only organic-subset element symbols; the caller handles `*`, `a` and `A`.
    fn parse_bare_atom(&mut self) -> Result<Pred, SmartsError> {
        let c = self.bytes[self.pos];
        // Two-letter aliphatic halogens.
        if c == b'C' && self.peek(1) == Some(b'l') {
            self.pos += 2;
            return Ok(Pred::Element(17));
        }
        if c == b'B' && self.peek(1) == Some(b'r') {
            self.pos += 2;
            return Ok(Pred::Element(35));
        }
        let pred = match c {
            b'B' => Pred::Element(5),
            b'C' => Pred::Element(6),
            b'N' => Pred::Element(7),
            b'O' => Pred::Element(8),
            b'P' => Pred::Element(15),
            b'S' => Pred::Element(16),
            b'F' => Pred::Element(9),
            b'I' => Pred::Element(53),
            b'b' => Pred::AromaticElement(5),
            b'c' => Pred::AromaticElement(6),
            b'n' => Pred::AromaticElement(7),
            b'o' => Pred::AromaticElement(8),
            b'p' => Pred::AromaticElement(15),
            b's' => Pred::AromaticElement(16),
            _ => {
                return Err(SmartsError::UnexpectedChar {
                    pos: self.pos,
                    ch: c as char,
                })
            }
        };
        self.pos += 1;
        Ok(pred)
    }

    /// Parses a bracket atom `[ expr ]` into one [`Pred`].
    fn parse_bracket_atom(&mut self) -> Result<Pred, SmartsError> {
        let open = self.pos;
        self.pos += 1; // consume '['
                       // Isotope digits are ignored; the SMILES parser drops isotopes too.
        while self.peek(0).is_some_and(|b| b.is_ascii_digit()) {
            self.pos += 1;
        }
        // The term budget is per bracket atom.
        self.expr_terms = 0;
        // `leading`: the cursor is at the atom-symbol position, where a bare `H` is
        // the hydrogen element (`[H]` = `[#1]`). `&`, `;`, `!` and juxtaposition
        // clear it; each `,` branch starts leading again.
        let pred = self.parse_atom_low_and(open, true)?;
        // Atom map `[C:1]` is ignored. Inside a bracket `:` can only be a map.
        if self.peek(0) == Some(b':') {
            self.pos += 1;
            while self.peek_is_digit() {
                self.pos += 1;
            }
        }
        match self.peek(0) {
            Some(b']') => {
                self.pos += 1;
                Ok(pred)
            }
            Some(_) => Err(SmartsError::UnexpectedChar {
                pos: self.pos,
                ch: self.bytes[self.pos] as char,
            }),
            None => Err(SmartsError::UnterminatedBracket { pos: open }),
        }
    }

    // --- Atom expression grammar (precedence: ! > & > , > ;) ---

    /// `low_and := or (';' or)*`. Terms after `;` are not leading.
    fn parse_atom_low_and(&mut self, open: usize, leading: bool) -> Result<Pred, SmartsError> {
        let mut acc = self.parse_atom_or(open, leading)?;
        while self.peek(0) == Some(b';') {
            self.bump_expr_term(self.pos)?;
            self.pos += 1;
            let rhs = self.parse_atom_or(open, false)?;
            acc = Pred::and(acc, rhs);
        }
        Ok(acc)
    }

    /// `or := high_and (',' high_and)*`. Each branch keeps the `leading` flag.
    fn parse_atom_or(&mut self, open: usize, leading: bool) -> Result<Pred, SmartsError> {
        let mut acc = self.parse_atom_high_and(open, leading)?;
        while self.peek(0) == Some(b',') {
            self.bump_expr_term(self.pos)?;
            self.pos += 1;
            let rhs = self.parse_atom_high_and(open, leading)?;
            acc = Pred::or(acc, rhs);
        }
        Ok(acc)
    }

    /// `high_and := unary (('&')? unary)*`. Only the first operand is leading.
    fn parse_atom_high_and(&mut self, open: usize, leading: bool) -> Result<Pred, SmartsError> {
        let mut acc = self.parse_atom_unary(open, leading)?;
        loop {
            if self.peek(0) == Some(b'&') {
                self.bump_expr_term(self.pos)?;
                self.pos += 1;
                let rhs = self.parse_atom_unary(open, false)?;
                acc = Pred::and(acc, rhs);
                continue;
            }
            // Juxtaposition.
            if self.at_atom_primitive_start() {
                self.bump_expr_term(self.pos)?;
                let rhs = self.parse_atom_unary(open, false)?;
                acc = Pred::and(acc, rhs);
                continue;
            }
            break;
        }
        Ok(acc)
    }

    /// `unary := '!'* primitive`.
    ///
    /// `!` runs are consumed in a loop, not by recursion, so a long run cannot
    /// overflow the stack. An even run collapses to the bare predicate and an odd
    /// run to one `Not`. A negated operand is not leading.
    fn parse_atom_unary(&mut self, open: usize, leading: bool) -> Result<Pred, SmartsError> {
        let mut negations = 0usize;
        while self.peek(0) == Some(b'!') {
            self.pos += 1;
            negations += 1;
        }
        let pred = self.parse_atom_primitive(open, leading && negations == 0)?;
        Ok(if negations % 2 == 1 {
            Pred::not(pred)
        } else {
            pred
        })
    }

    /// True if the cursor is at a byte that can begin an atom primitive (to
    /// detect juxtaposition). Letters, `@` and `$` count, so `[C@H]` and `[C$(..)]`
    /// reach [`parse_atom_primitive`](Self::parse_atom_primitive).
    fn at_atom_primitive_start(&self) -> bool {
        match self.peek(0) {
            None => false,
            Some(b) => {
                matches!(b, b'*' | b'#' | b'+' | b'-' | b'@' | b'$') || b.is_ascii_alphabetic()
            }
        }
    }

    /// Parses one atom primitive (the leaves of the expression tree).
    ///
    /// `open` is the byte offset of the enclosing `[`, for errors.
    fn parse_atom_primitive(&mut self, open: usize, leading: bool) -> Result<Pred, SmartsError> {
        let pos = self.pos;
        let Some(c) = self.peek(0) else {
            return Err(SmartsError::UnterminatedBracket { pos: open });
        };
        // Longest match: a two-letter element beats a one-letter primitive, so
        // `[As]` is arsenic, not `A` + `s` (also `[He]`, `[Hf]`, `[Re]`, `[Xe]`, ...).
        if c.is_ascii_uppercase() {
            if let Some(c2) = self.peek(1) {
                if c2.is_ascii_lowercase() {
                    if let Some(num) = crate::smiles::two_letter_element(c, c2) {
                        self.pos += 2;
                        return Ok(Pred::Element(num));
                    }
                }
            }
        }
        // Aromatic `se`, `te`, `as` are unsupported; refuse them rather than split
        // `[as]` into `a` + `s`. No target can hold them (SMILES refuses them too).
        if matches!(
            (c, self.peek(1)),
            (b'a', Some(b's')) | (b's', Some(b'e')) | (b't', Some(b'e'))
        ) {
            return Err(SmartsError::Unsupported {
                pos,
                what: "aromatic Se/Te/As",
            });
        }
        match c {
            b'*' => {
                self.pos += 1;
                Ok(Pred::Any)
            }
            b'a' => {
                self.pos += 1;
                Ok(Pred::Aromatic)
            }
            b'A' => {
                self.pos += 1;
                Ok(Pred::Aliphatic)
            }
            b'#' => {
                self.pos += 1;
                let n = self.parse_number(pos)?;
                Ok(Pred::AtomicNumber(count_value(n, pos)?))
            }
            b'R' => {
                // Bare `R`: in any ring. `R<n>`: SSSR ring count (`R0` = acyclic).
                self.pos += 1;
                if self.peek_is_digit() {
                    let n = self.parse_number(pos)?;
                    return Ok(Pred::RingCount(count_value(n, pos)?));
                }
                Ok(Pred::InRing)
            }
            b'r' => {
                // `r<n>`: in a ring of size n. Bare `r` is refused as ambiguous.
                self.pos += 1;
                if !self.peek_is_digit() {
                    return Err(SmartsError::Unsupported {
                        pos,
                        what: "bare 'r' (ring-size with no number)",
                    });
                }
                let n = self.parse_number(pos)?;
                Ok(Pred::RingSize(count_value(n, pos)?))
            }
            b'D' => {
                self.pos += 1;
                let n = self.parse_number(pos)?;
                Ok(Pred::Degree(count_value(n, pos)?))
            }
            b'H' => {
                // A leading bare `H` is the hydrogen element (`[H]` = `[#1]`).
                // Otherwise `H` is a total-H count: `H` = 1, `H<n>` = n.
                self.pos += 1;
                if leading && !self.peek_is_digit() {
                    return Ok(Pred::AtomicNumber(1));
                }
                let n = if self.peek_is_digit() {
                    self.parse_number(pos)?
                } else {
                    1
                };
                Ok(Pred::TotalH(count_value(n, pos)?))
            }
            b'+' | b'-' => {
                let charge = self.parse_charge()?;
                Ok(Pred::Charge(charge))
            }
            // Chirality (`@`, `@@`, `@TH1`, ...) is ignored: matching is
            // stereo-agnostic. It yields `Pred::Any`, the AND identity, so `[C@H]`
            // is `[CH]`. A `@` between atoms is the ring-bond primitive.
            b'@' => {
                self.pos += 1;
                if self.peek(0) == Some(b'@') {
                    self.pos += 1;
                }
                if self.peek(0) == Some(b'?') {
                    self.pos += 1;
                }
                // Optional class (TH/AL/SP/TB/OH) + digits. Consumed only for a
                // known class, so the `H` in `[C@H]` stays an H-count.
                if let (Some(a), Some(b)) = (self.peek(0), self.peek(1)) {
                    if matches!(
                        (a, b),
                        (b'T', b'H') | (b'A', b'L') | (b'S', b'P') | (b'T', b'B') | (b'O', b'H')
                    ) {
                        self.pos += 2;
                        while self.peek_is_digit() {
                            self.pos += 1;
                        }
                    }
                }
                Ok(Pred::Any)
            }
            // `X<n>` total connectivity (heavy connections + attached H); `h<n>`
            // implicit-H; `x<n>` ring-bond count. Bare forms mean 1.
            b'X' => {
                self.pos += 1;
                let n = if self.peek_is_digit() {
                    self.parse_number(pos)?
                } else {
                    1
                };
                Ok(Pred::TotalConnections(count_value(n, pos)?))
            }
            b'h' => {
                self.pos += 1;
                let n = if self.peek_is_digit() {
                    self.parse_number(pos)?
                } else {
                    1
                };
                Ok(Pred::ImplicitH(count_value(n, pos)?))
            }
            b'x' => {
                self.pos += 1;
                let n = if self.peek_is_digit() {
                    self.parse_number(pos)?
                } else {
                    1
                };
                Ok(Pred::RingBondCount(count_value(n, pos)?))
            }
            // `v<n>` total bond-order valence (bare `v` defaults to 1).
            b'v' => {
                self.pos += 1;
                let n = if self.peek_is_digit() {
                    self.parse_number(pos)?
                } else {
                    1
                };
                Ok(Pred::Valence(count_value(n, pos)?))
            }
            // `$(env)`: the host atom must be atom 0 of an embedding of `env`.
            // Nesting is bounded by `MAX_REC_DEPTH`.
            b'$' => {
                self.pos += 1; // consume '$'
                if self.peek(0) != Some(b'(') {
                    return Err(SmartsError::Unsupported {
                        pos,
                        what: "recursive '$' must be followed by '('",
                    });
                }
                if self.depth + 1 > MAX_REC_DEPTH {
                    return Err(SmartsError::Unsupported {
                        pos,
                        what: "recursive SMARTS '$(...)' nested too deep",
                    });
                }
                // Find the matching ')'. No other construct holds unbalanced parens,
                // so counting raw bytes is exact.
                let inner_start = self.pos + 1; // first byte after '('
                let mut paren = 1usize;
                let mut j = inner_start;
                while j < self.bytes.len() {
                    match self.bytes[j] {
                        b'(' => paren += 1,
                        b')' => {
                            paren -= 1;
                            if paren == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                if paren != 0 {
                    return Err(SmartsError::Unsupported {
                        pos,
                        what: "unterminated recursive '$(' — missing ')'",
                    });
                }
                let inner_end = j; // index of the matching ')'
                if inner_end == inner_start {
                    return Err(SmartsError::EmptyExpression { pos: inner_start });
                }
                // The delimiters are ASCII, so this cannot fail; error, don't panic.
                let inner =
                    std::str::from_utf8(&self.bytes[inner_start..inner_end]).map_err(|_| {
                        SmartsError::Unsupported {
                            pos,
                            what: "non-UTF-8 in recursive SMARTS body",
                        }
                    })?;
                let (env, used) = parse_with_depth(inner, self.depth + 1, self.budget - self.used)?;
                self.used += used;
                self.pos = inner_end + 1; // consume the matching ')'
                Ok(Pred::Recursive(Box::new(env)))
            }
            // Element symbols (aliphatic upper / aromatic lower).
            _ if c.is_ascii_uppercase() => self.parse_bracket_element(pos),
            _ if c.is_ascii_lowercase() => self.parse_bracket_aromatic_element(pos),
            // An operator where a primitive was expected: empty operand.
            b']' | b',' | b';' | b'&' => Err(SmartsError::EmptyExpression { pos }),
            _ => Err(SmartsError::UnexpectedChar { pos, ch: c as char }),
        }
    }

    /// Parses an aliphatic element symbol inside a bracket into [`Pred::Element`].
    /// Two-letter symbols are tried first, from the full periodic table.
    fn parse_bracket_element(&mut self, pos: usize) -> Result<Pred, SmartsError> {
        let c = self.bytes[self.pos];
        if let Some(c2) = self.peek(1) {
            if c2.is_ascii_lowercase() {
                if let Some(num) = crate::smiles::two_letter_element(c, c2) {
                    self.pos += 2;
                    return Ok(Pred::Element(num));
                }
            }
        }
        if let Some(num) = one_letter_aliphatic(c) {
            self.pos += 1;
            return Ok(Pred::Element(num));
        }
        Err(SmartsError::UnexpectedChar { pos, ch: c as char })
    }

    /// Parses an aromatic element symbol inside a bracket into
    /// [`Pred::AromaticElement`].
    fn parse_bracket_aromatic_element(&mut self, pos: usize) -> Result<Pred, SmartsError> {
        let c = self.bytes[self.pos];
        if let Some(num) = aromatic_lower(c) {
            self.pos += 1;
            return Ok(Pred::AromaticElement(num));
        }
        Err(SmartsError::UnexpectedChar { pos, ch: c as char })
    }

    /// Parses a charge primitive: `+`, `-`, `++`, `--`, `+n`, `-n`. The cursor is
    /// on the sign byte.
    fn parse_charge(&mut self) -> Result<i8, SmartsError> {
        let at = self.pos;
        let sign_byte = self.bytes[self.pos];
        let sign: i32 = if sign_byte == b'+' { 1 } else { -1 };
        self.pos += 1;
        let mag = if self.peek_is_digit() {
            // `parse_number` saturates; the range check below refuses it.
            self.parse_number(at)?
        } else {
            // Repeated signs (`++`).
            let mut mag = 1u32;
            while self.peek(0) == Some(sign_byte) && mag <= MAX_CHARGE {
                mag += 1;
                self.pos += 1;
            }
            mag
        };
        if mag > MAX_CHARGE {
            return Err(SmartsError::ValueOutOfRange {
                pos: at,
                max: MAX_CHARGE,
            });
        }
        Ok((sign * mag as i32) as i8)
    }

    // --- Bond expression grammar (same precedence as atoms) ---

    /// Parses a bond expression, with the same precedence as atom expressions.
    fn parse_bond_expression(&mut self) -> Result<BondPred, SmartsError> {
        // The term budget is per bond expression.
        self.expr_terms = 0;
        self.parse_bond_low_and()
    }

    /// `low_and := or (';' or)*`.
    fn parse_bond_low_and(&mut self) -> Result<BondPred, SmartsError> {
        let mut acc = self.parse_bond_or()?;
        while self.peek(0) == Some(b';') {
            self.bump_expr_term(self.pos)?;
            self.pos += 1;
            let rhs = self.parse_bond_or()?;
            acc = BondPred::and(acc, rhs);
        }
        Ok(acc)
    }

    /// `or := high_and (',' high_and)*`.
    fn parse_bond_or(&mut self) -> Result<BondPred, SmartsError> {
        let mut acc = self.parse_bond_high_and()?;
        while self.peek(0) == Some(b',') {
            self.bump_expr_term(self.pos)?;
            self.pos += 1;
            let rhs = self.parse_bond_high_and()?;
            acc = BondPred::or(acc, rhs);
        }
        Ok(acc)
    }

    /// `high_and := unary (('&')? unary)*`.
    fn parse_bond_high_and(&mut self) -> Result<BondPred, SmartsError> {
        let mut acc = self.parse_bond_unary()?;
        loop {
            if self.peek(0) == Some(b'&') {
                self.bump_expr_term(self.pos)?;
                self.pos += 1;
                let rhs = self.parse_bond_unary()?;
                acc = BondPred::and(acc, rhs);
                continue;
            }
            if self.at_bond_primitive_start() {
                self.bump_expr_term(self.pos)?;
                let rhs = self.parse_bond_unary()?;
                acc = BondPred::and(acc, rhs);
                continue;
            }
            break;
        }
        Ok(acc)
    }

    /// `unary := '!'* primitive`. See [`parse_atom_unary`](Self::parse_atom_unary).
    fn parse_bond_unary(&mut self) -> Result<BondPred, SmartsError> {
        let mut negations = 0usize;
        while self.peek(0) == Some(b'!') {
            self.pos += 1;
            negations += 1;
        }
        let pred = self.parse_bond_primitive()?;
        Ok(if negations % 2 == 1 {
            BondPred::not(pred)
        } else {
            pred
        })
    }

    /// True if the cursor is at a byte that can begin a bond primitive.
    fn at_bond_primitive_start(&self) -> bool {
        matches!(
            self.peek(0),
            Some(b'-' | b'=' | b'#' | b':' | b'~' | b'@' | b'/' | b'\\')
        )
    }

    /// Parses one bond primitive leaf.
    fn parse_bond_primitive(&mut self) -> Result<BondPred, SmartsError> {
        let pos = self.pos;
        let Some(c) = self.peek(0) else {
            return Err(SmartsError::EmptyExpression { pos });
        };
        let pred = match c {
            b'-' => BondPred::Order(bond_order::SINGLE),
            b'=' => BondPred::Order(bond_order::DOUBLE),
            b'#' => BondPred::Order(bond_order::TRIPLE),
            b':' => BondPred::Aromatic,
            b'~' => BondPred::Any,
            b'@' => BondPred::Ring,
            // `/` `\` carry E/Z geometry, which matching ignores: read as single.
            b'/' | b'\\' => BondPred::Order(bond_order::SINGLE),
            _ => return Err(SmartsError::UnexpectedChar { pos, ch: c as char }),
        };
        self.pos += 1;
        Ok(pred)
    }

    // --- Small lexer helpers ---

    /// Parses one or more decimal digits as a saturating `u32`. `pos` is the
    /// owning primitive's offset, for `MissingNumber`.
    fn parse_number(&mut self, pos: usize) -> Result<u32, SmartsError> {
        if !self.peek_is_digit() {
            return Err(SmartsError::MissingNumber { pos });
        }
        let mut n = 0u32;
        while let Some(d) = self.peek(0) {
            if d.is_ascii_digit() {
                n = n.saturating_mul(10).saturating_add((d - b'0') as u32);
                self.pos += 1;
            } else {
                break;
            }
        }
        Ok(n)
    }

    /// True if the next byte is an ASCII digit.
    #[inline]
    fn peek_is_digit(&self) -> bool {
        self.peek(0).map(|b| b.is_ascii_digit()).unwrap_or(false)
    }

    /// Returns the byte `offset` positions ahead of the cursor, if in range.
    #[inline]
    fn peek(&self, offset: usize) -> Option<u8> {
        self.bytes.get(self.pos + offset).copied()
    }
}

/// A count primitive's value, refused above [`MAX_COUNT`].
fn count_value(n: u32, pos: usize) -> Result<u8, SmartsError> {
    if n > MAX_COUNT {
        return Err(SmartsError::ValueOutOfRange {
            pos,
            max: MAX_COUNT,
        });
    }
    Ok(n as u8)
}

/// True if byte `c` can begin a bare organic-subset atom token in SMARTS.
fn is_organic_atom_start(c: u8) -> bool {
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

/// Maps a single upper-case letter to an aliphatic element's atomic number.
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
        _ => None,
    }
}

/// Maps a single lower-case aromatic letter to its atomic number.
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

    /// Parse and unwrap, panicking with the error on failure.
    fn q(s: &str) -> QueryGraph {
        parse(s).unwrap_or_else(|e| panic!("parse({s:?}) failed: {e}"))
    }

    #[test]
    fn benzene_topology_and_aromatic_bonds() {
        // c1ccccc1 — six aromatic-carbon predicates, six default (implicit) bonds
        // each meaning single-or-aromatic, in a 6-ring.
        let g = q("c1ccccc1");
        assert_eq!(g.atom_count(), 6);
        assert_eq!(g.bond_count(), 6);
        for a in &g.atoms {
            assert_eq!(*a, Pred::AromaticElement(6));
        }
        // Default bonds: SingleOrAromatic.
        for b in &g.bonds {
            assert_eq!(b.pred, BondPred::SingleOrAromatic);
        }
        // Topology forms a cycle: every atom has degree 2.
        for inc in &g.incident {
            assert_eq!(inc.len(), 2);
        }
    }

    #[test]
    fn atomic_number_primitive() {
        // [#6] — atomic number 6, any aromaticity.
        let g = q("[#6]");
        assert_eq!(g.atom_count(), 1);
        assert_eq!(g.atoms[0], Pred::AtomicNumber(6));
    }

    #[test]
    fn or_of_two_elements() {
        // [N,O] — aliphatic N OR aliphatic O.
        let g = q("[N,O]");
        assert_eq!(g.atom_count(), 1);
        assert_eq!(g.atoms[0], Pred::or(Pred::Element(7), Pred::Element(8)));
    }

    #[test]
    fn low_and_of_element_and_h_count() {
        // [C;H2] — aliphatic carbon AND total-H exactly 2, joined by low-and ';'.
        let g = q("[C;H2]");
        assert_eq!(g.atom_count(), 1);
        assert_eq!(g.atoms[0], Pred::and(Pred::Element(6), Pred::TotalH(2)));
    }

    #[test]
    fn not_in_ring() {
        // [!R] — NOT in a ring.
        let g = q("[!R]");
        assert_eq!(g.atom_count(), 1);
        assert_eq!(g.atoms[0], Pred::not(Pred::InRing));
    }

    #[test]
    fn explicit_aromatic_bond_between_aromatics() {
        // a-a — two "aromatic atom" predicates joined by an explicit single bond.
        let g = q("a-a");
        assert_eq!(g.atom_count(), 2);
        assert_eq!(g.bond_count(), 1);
        assert_eq!(g.atoms[0], Pred::Aromatic);
        assert_eq!(g.atoms[1], Pred::Aromatic);
        assert_eq!(g.bonds[0].pred, BondPred::Order(bond_order::SINGLE));
    }

    #[test]
    fn any_atom_any_bond() {
        // *~* — any atom, any bond, any atom.
        let g = q("*~*");
        assert_eq!(g.atom_count(), 2);
        assert_eq!(g.bond_count(), 1);
        assert_eq!(g.atoms[0], Pred::Any);
        assert_eq!(g.atoms[1], Pred::Any);
        assert_eq!(g.bonds[0].pred, BondPred::Any);
    }

    // --- Precedence ---

    #[test]
    fn precedence_or_binds_tighter_than_low_and() {
        // [C,N;H1] parses as (C OR N) AND H1: low-and ';' is the outermost node.
        let g = q("[C,N;H1]");
        let expected = Pred::and(
            Pred::or(Pred::Element(6), Pred::Element(7)),
            Pred::TotalH(1),
        );
        assert_eq!(g.atoms[0], expected);
    }

    #[test]
    fn precedence_high_and_binds_tighter_than_or() {
        // [C&H1,N] parses as (C AND H1) OR N: the OR is the outermost node.
        let g = q("[C&H1,N]");
        let expected = Pred::or(
            Pred::and(Pred::Element(6), Pred::TotalH(1)),
            Pred::Element(7),
        );
        assert_eq!(g.atoms[0], expected);
    }

    #[test]
    fn juxtaposition_is_high_and() {
        // [CH2] = C juxtaposed with H2 = C AND H2 at the high-and level. To see it
        // is *high*-and (tighter than OR) check [CH2,N] = (C AND H2) OR N.
        let g = q("[CH2,N]");
        let expected = Pred::or(
            Pred::and(Pred::Element(6), Pred::TotalH(2)),
            Pred::Element(7),
        );
        assert_eq!(g.atoms[0], expected);
    }

    #[test]
    fn not_binds_tightest() {
        // [!C;R] = (NOT C) AND R, with ';' the outer node and NOT wrapping only C.
        let g = q("[!C;R]");
        let expected = Pred::and(Pred::not(Pred::Element(6)), Pred::InRing);
        assert_eq!(g.atoms[0], expected);
    }

    #[test]
    fn ring_size_and_degree_and_charge() {
        // [r5] ring of size 5.
        assert_eq!(q("[r5]").atoms[0], Pred::RingSize(5));
        // [D3] degree 3.
        assert_eq!(q("[D3]").atoms[0], Pred::Degree(3));
        // [N+] charge +1, [O-] charge -1, [Ca+2] charge +2, [N--] charge -2.
        assert_eq!(
            q("[N+]").atoms[0],
            Pred::and(Pred::Element(7), Pred::Charge(1))
        );
        assert_eq!(
            q("[O-]").atoms[0],
            Pred::and(Pred::Element(8), Pred::Charge(-1))
        );
        assert_eq!(
            q("[Ca+2]").atoms[0],
            Pred::and(Pred::Element(20), Pred::Charge(2))
        );
        assert_eq!(
            q("[N--]").atoms[0],
            Pred::and(Pred::Element(7), Pred::Charge(-2))
        );
    }

    #[test]
    fn bare_charge_only_bracket() {
        // [+] is just a charge predicate.
        assert_eq!(q("[+]").atoms[0], Pred::Charge(1));
        assert_eq!(q("[-2]").atoms[0], Pred::Charge(-2));
    }

    #[test]
    fn aromatic_and_aliphatic_wildcards() {
        assert_eq!(q("[a]").atoms[0], Pred::Aromatic);
        assert_eq!(q("[A]").atoms[0], Pred::Aliphatic);
        assert_eq!(q("[*]").atoms[0], Pred::Any);
    }

    #[test]
    fn bond_primitives_each_lower_correctly() {
        assert_eq!(q("C-C").bonds[0].pred, BondPred::Order(bond_order::SINGLE));
        assert_eq!(q("C=C").bonds[0].pred, BondPred::Order(bond_order::DOUBLE));
        assert_eq!(q("C#C").bonds[0].pred, BondPred::Order(bond_order::TRIPLE));
        assert_eq!(q("c:c").bonds[0].pred, BondPred::Aromatic);
        assert_eq!(q("C@C").bonds[0].pred, BondPred::Ring);
        assert_eq!(q("C~C").bonds[0].pred, BondPred::Any);
        // Default (no symbol) is single-or-aromatic.
        assert_eq!(q("CC").bonds[0].pred, BondPred::SingleOrAromatic);
    }

    #[test]
    fn bond_expression_with_operators() {
        // C-,=C — single OR double bond.
        let g = q("C-,=C");
        assert_eq!(
            g.bonds[0].pred,
            BondPred::or(
                BondPred::Order(bond_order::SINGLE),
                BondPred::Order(bond_order::DOUBLE)
            )
        );
        // C!@C — a non-ring bond.
        let g2 = q("C!@C");
        assert_eq!(g2.bonds[0].pred, BondPred::not(BondPred::Ring));
    }

    #[test]
    fn branches_and_ring_closures_reuse_topology() {
        // c1ccc(O)cc1 — phenol-like query: branch on the 4th atom.
        let g = q("c1ccc(O)cc1");
        assert_eq!(g.atom_count(), 7);
        // 6 ring bonds + 1 branch bond.
        assert_eq!(g.bond_count(), 7);
        // The branch atom O is aliphatic oxygen.
        assert!(g.atoms.contains(&Pred::Element(8)));
    }

    #[test]
    fn ring_closure_with_explicit_bond() {
        // C1CCC=1 — the ring-closure bond carries an explicit double bond.
        let g = q("C1CCC=1");
        assert_eq!(g.atom_count(), 4);
        assert_eq!(g.bond_count(), 4);
        let has_double = g
            .bonds
            .iter()
            .any(|b| b.pred == BondPred::Order(bond_order::DOUBLE));
        assert!(has_double, "ring-closure double bond must be present");
    }

    #[test]
    fn disconnection_makes_two_components() {
        // C.C — two carbons, no bond.
        let g = q("C.C");
        assert_eq!(g.atom_count(), 2);
        assert_eq!(g.bond_count(), 0);
    }

    #[test]
    fn complex_real_query() {
        // [#6;R]~[#7,#8] — a ring carbon, any bond, to N or O. Verifies a
        // bracket low-and, a tilde bond, and a bracket OR all in one query.
        let g = q("[#6;R]~[#7,#8]");
        assert_eq!(g.atom_count(), 2);
        assert_eq!(g.atoms[0], Pred::and(Pred::AtomicNumber(6), Pred::InRing));
        assert_eq!(g.bonds[0].pred, BondPred::Any);
        assert_eq!(
            g.atoms[1],
            Pred::or(Pred::AtomicNumber(7), Pred::AtomicNumber(8))
        );
    }

    // --- Recursive, extended and error cases ---

    #[test]
    fn recursive_smarts_parses_into_an_environment_graph() {
        // `$(env)` parses to a `Pred::Recursive` holding the sub-pattern's graph.
        let q = parse("[$(CC)]").unwrap();
        match &q.atoms[0] {
            Pred::Recursive(env) => {
                assert_eq!(env.atom_count(), 2, "CC env has two atoms");
                assert_eq!(env.atoms[0], Pred::Element(6));
                assert_eq!(env.bond_count(), 1);
            }
            other => panic!("expected Pred::Recursive, got {other:?}"),
        }
        // AND-ed with an element: `[C$(C=O)]`.
        match &parse("[C$(C=O)]").unwrap().atoms[0] {
            Pred::And(l, r) => {
                assert_eq!(**l, Pred::Element(6));
                assert!(
                    matches!(**r, Pred::Recursive(_)),
                    "rhs is the recursive env"
                );
            }
            other => panic!("expected And(Element, Recursive), got {other:?}"),
        }
        // Nested: `[$(c[$(n)])]`.
        match &parse("[$(c[$(n)])]").unwrap().atoms[0] {
            Pred::Recursive(env) => {
                assert_eq!(env.atom_count(), 2);
                assert!(
                    matches!(env.atoms[1], Pred::Recursive(_)),
                    "inner atom is recursive"
                );
            }
            other => panic!("expected outer Pred::Recursive, got {other:?}"),
        }
        // Malformed recursive forms are errors.
        assert!(parse("[$CC]").is_err(), "'$' without '(' is an error");
        assert!(parse("[$(CC]").is_err(), "missing ')' is an error");
        assert!(parse("[$()]").is_err(), "empty '$()' is an error");
        // Over-deep nesting is an error, not a stack overflow.
        let bomb = format!("[{}*{}]", "$([".repeat(40), "])".repeat(40));
        assert!(parse(&bomb).is_err(), "over-deep nesting is rejected");
    }

    #[test]
    fn connectivity_implicit_h_ring_bond_primitives_parse() {
        // Bare forms mean 1.
        assert_eq!(parse("[X3]").unwrap().atoms[0], Pred::TotalConnections(3));
        assert_eq!(parse("[h1]").unwrap().atoms[0], Pred::ImplicitH(1));
        assert_eq!(parse("[x2]").unwrap().atoms[0], Pred::RingBondCount(2));
        assert_eq!(parse("[X]").unwrap().atoms[0], Pred::TotalConnections(1));
        assert_eq!(parse("[h]").unwrap().atoms[0], Pred::ImplicitH(1));
        assert_eq!(parse("[x]").unwrap().atoms[0], Pred::RingBondCount(1));
        // AND-ed with an element:
        assert_eq!(
            parse("[CX4]").unwrap().atoms[0],
            Pred::and(Pred::Element(6), Pred::TotalConnections(4))
        );
        // `v<n>` valence and `R<n>` ring count.
        assert_eq!(parse("[v4]").unwrap().atoms[0], Pred::Valence(4));
        assert_eq!(parse("[v]").unwrap().atoms[0], Pred::Valence(1));
        assert_eq!(parse("[R2]").unwrap().atoms[0], Pred::RingCount(2));
        assert_eq!(parse("[R0]").unwrap().atoms[0], Pred::RingCount(0));
        assert_eq!(parse("[R]").unwrap().atoms[0], Pred::InRing); // bare R = in any ring
    }

    #[test]
    fn chirality_is_parsed_and_ignored() {
        // `[C@H]` parses to C AND `Any` AND H1, i.e. `[CH]`. Matching equivalence is
        // tested in tests/golden_smarts.rs.
        for q in [
            "[C@H]", "[C@@H]", "[C@TH1]", "[C@]", "[C@@]", "[N@@H]", "[C@?]",
        ] {
            assert!(parse(q).is_ok(), "{q:?} should parse");
        }
        let g = parse("[C@H]").unwrap();
        assert_eq!(g.atoms.len(), 1, "one atom");
        // The descriptor must not eat a following H-count: [C@H] still carries TotalH(1).
        fn has_total_h1(p: &Pred) -> bool {
            match p {
                Pred::TotalH(1) => true,
                Pred::And(l, r) => has_total_h1(l) || has_total_h1(r),
                _ => false,
            }
        }
        assert!(has_total_h1(&g.atoms[0]), "[C@H] keeps its H-count");
        // A between-atom `@` is still the ring-bond primitive, not chirality.
        assert!(matches!(
            parse("C@C").unwrap().bonds[0].pred,
            BondPred::Ring
        ));
    }

    #[test]
    fn directional_bonds_are_parsed_as_single() {
        // `/` and `\` parse as single bonds.
        let g = parse("C/C=C/C").unwrap();
        assert!(matches!(g.bonds[0].pred, BondPred::Order(o) if o == bond_order::SINGLE));
        assert!(parse("F/C=C\\F").is_ok());
    }

    #[test]
    fn empty_operand_is_rejected() {
        // [C,] — OR with a missing right operand.
        assert!(matches!(
            parse("[C,]"),
            Err(SmartsError::EmptyExpression { .. })
        ));
        // [!] — NOT with no operand.
        assert!(matches!(
            parse("[!]"),
            Err(SmartsError::EmptyExpression { .. })
        ));
        // [] — empty bracket.
        assert!(matches!(
            parse("[]"),
            Err(SmartsError::EmptyExpression { .. })
        ));
    }

    #[test]
    fn missing_number_is_rejected() {
        // [#] — atomic-number primitive with no number.
        assert!(matches!(
            parse("[#]"),
            Err(SmartsError::MissingNumber { .. })
        ));
        // [D] — degree with no number.
        assert!(matches!(
            parse("[D]"),
            Err(SmartsError::MissingNumber { .. })
        ));
    }

    #[test]
    fn unterminated_bracket_is_rejected() {
        assert!(matches!(
            parse("[C"),
            Err(SmartsError::UnterminatedBracket { .. })
        ));
    }

    #[test]
    fn topology_errors_match_smiles_layer() {
        assert!(matches!(
            parse("CC("),
            Err(SmartsError::UnbalancedOpen { .. })
        ));
        assert!(matches!(
            parse("CC)"),
            Err(SmartsError::UnbalancedClose { .. })
        ));
        assert!(matches!(
            parse("C1CCC"),
            Err(SmartsError::UnclosedRing { .. })
        ));
        assert!(matches!(
            parse("=CC"),
            Err(SmartsError::DanglingBond { .. })
        ));
        assert!(matches!(parse(""), Err(SmartsError::Empty)));
    }

    #[test]
    fn bare_r_without_number_is_rejected() {
        // Bare `r` is refused.
        assert!(matches!(
            parse("[r]"),
            Err(SmartsError::Unsupported {
                what: "bare 'r' (ring-size with no number)",
                ..
            })
        ));
        // `R<n>` is the ring count, distinct from bare `R`.
        assert_eq!(parse("[R2]").unwrap().atoms[0], Pred::RingCount(2));
    }
}
