// ==================================================================
// PRACTICE 0026: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0026_saturating_volume_knob.md
// Run the tests:          cargo test practice::p0026
// ==================================================================

//! # Problem 0026: Saturating Volume Knob
//!
//! - **Difficulty:** Easy
//! - **Topics:** `u8`, `saturating_add`, `saturating_sub`, `checked_add_signed`, `unsigned_abs`
//! - **Source:** Original · [`u8::saturating_add`](https://doc.rust-lang.org/std/primitive.u8.html#method.saturating_add)
//!
//! ## Problem
//!
//! A speaker stores its volume as a `u8` (`0..=255`). Each button press sends a signed change
//! (an `i8`, from `-128` to `127`). The volume never goes below 0 or above 255: a change that would
//! go past a limit simply stops **at** the limit.
//!
//! Write two functions:
//!
//! 1. `apply_changes(start: u8, changes: &[i8]) -> u8` returns the final volume after applying
//!    every change in order.
//! 2. `count_clipped(start: u8, changes: &[i8]) -> usize` applies the same changes and returns how
//!    many of them hit a limit, that is, how many changes could **not** be applied in full.
//!
//! ## Examples
//!
//! ```text
//! apply_changes(100, &[50, -30])      -> 120
//! apply_changes(250, &[10])           -> 255      clipped at the top
//! apply_changes(5, &[-10, 3])         -> 3        5 - 10 stops at 0, then 0 + 3 = 3
//! count_clipped(5, &[-10, 3])         -> 1
//! count_clipped(250, &[10, 10, -5])   -> 2        255, still 255, then 250
//! ```
//!
//! ## Constraints
//!
//! - `0 <= changes.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `u8::saturating_add` and `u8::saturating_sub` stop at the limits instead of overflowing.
//! 2. A negative `i8` change `c` means "subtract `c.unsigned_abs()`" (a `u8`); `-128i8` has no
//!    positive `i8` counterpart, which is why `unsigned_abs` returns a `u8`.
//! 3. `volume.checked_add_signed(c)` returns `None` exactly when the change would leave `0..=255`.

pub fn apply_changes(start: u8, changes: &[i8]) -> u8 {
    todo!("Write your solution here")
}

pub fn count_clipped(start: u8, changes: &[i8]) -> usize {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_examples() {
        assert_eq!(apply_changes(100, &[50, -30]), 120);
        assert_eq!(apply_changes(250, &[10]), 255);
        assert_eq!(apply_changes(5, &[-10, 3]), 3);
    }

    #[test]
    fn no_changes() {
        assert_eq!(apply_changes(42, &[]), 42);
        assert_eq!(count_clipped(42, &[]), 0);
    }

    #[test]
    fn extreme_changes() {
        assert_eq!(apply_changes(0, &[i8::MIN]), 0);
        assert_eq!(apply_changes(255, &[i8::MAX]), 255);
        assert_eq!(apply_changes(128, &[i8::MIN]), 0);
        assert_eq!(apply_changes(128, &[i8::MAX]), 255);
        assert_eq!(apply_changes(0, &[127, 127, 1]), 255);
    }

    #[test]
    fn clipped_examples() {
        assert_eq!(count_clipped(5, &[-10, 3]), 1);
        assert_eq!(count_clipped(250, &[10, 10, -5]), 2);
        assert_eq!(count_clipped(100, &[50, -30]), 0);
    }

    #[test]
    fn hitting_the_limit_exactly_is_not_clipped() {
        assert_eq!(count_clipped(250, &[5]), 0);
        assert_eq!(count_clipped(5, &[-5]), 0);
        assert_eq!(count_clipped(128, &[i8::MIN]), 0);
    }

    #[test]
    fn long_sequence() {
        let changes = vec![1i8; 100_000];
        assert_eq!(apply_changes(0, &changes), 255);
        assert_eq!(count_clipped(0, &changes), 100_000 - 255);
    }
}
