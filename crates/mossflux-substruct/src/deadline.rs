//! The per-thread search deadline.
//!
//! Subgraph matching is NP-complete, so some short queries (such as
//! `*.*.*.*.*.*.[!#6]` against a long carbon chain) take exponential time. A
//! search with a deadline must be able to stop inside any matcher and report it:
//! a timed-out search is an error, never a partial hit list.
//!
//! The VF2 matcher carries its own deadline (see [`crate::vf2::Matcher`]). The
//! recursive-environment matcher (`$(...)`) is reached through
//! [`crate::query::Pred::eval`], which has no matcher to carry one, so it reads the
//! deadline set here for the current thread. Both check the clock once every
//! [`CHECK_EVERY`] steps, so the cost without a deadline is a counter decrement.
//!
//! The verify stage sets the deadline for one candidate chunk on the thread that
//! runs it. Matching never calls back into rayon, so a worker cannot pick up
//! another search's work while the deadline is set.

use std::cell::Cell;
use std::time::Instant;

/// Matcher steps between two clock checks.
pub(crate) const CHECK_EVERY: u32 = 4_096;

thread_local! {
    static DEADLINE: Cell<Option<Instant>> = const { Cell::new(None) };
    static EXPIRED: Cell<bool> = const { Cell::new(false) };
}

/// Sets this thread's deadline until the guard drops, then restores the previous
/// state (so a nested or earlier setting is never lost).
pub(crate) struct Guard {
    prev_deadline: Option<Instant>,
    prev_expired: bool,
}

impl Drop for Guard {
    fn drop(&mut self) {
        DEADLINE.with(|d| d.set(self.prev_deadline));
        EXPIRED.with(|e| e.set(self.prev_expired));
    }
}

/// Sets `deadline` for this thread and clears the expired flag.
pub(crate) fn set(deadline: Option<Instant>) -> Guard {
    Guard {
        prev_deadline: DEADLINE.with(|d| d.replace(deadline)),
        prev_expired: EXPIRED.with(|e| e.replace(false)),
    }
}

/// True once this thread's deadline has passed (and remembers it). Called at most
/// once every [`CHECK_EVERY`] matcher steps.
pub(crate) fn check_now() -> bool {
    if EXPIRED.with(Cell::get) {
        return true;
    }
    let expired = DEADLINE
        .with(Cell::get)
        .is_some_and(|d| Instant::now() >= d);
    if expired {
        EXPIRED.with(|e| e.set(true));
    }
    expired
}

/// True if a matcher on this thread has seen the deadline pass since [`set`].
pub(crate) fn expired() -> bool {
    EXPIRED.with(Cell::get)
}
