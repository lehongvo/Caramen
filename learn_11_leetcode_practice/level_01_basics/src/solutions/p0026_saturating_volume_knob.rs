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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for each change, add it with saturation: non-negative changes use `saturating_add`,
//! negative ones use `saturating_sub` with the magnitude. To count clipped changes, try
//! `checked_add_signed` first: `None` means the change hit a limit, so count it and fall back to the
//! saturated value.
//!
//! **Rust concepts:**
//! - The three overflow families: `checked_*` returns `Option`, `saturating_*` clamps to `MIN`/`MAX`,
//!   `wrapping_*` wraps around (see problem 0027). Plain operators panic in debug builds.
//! - `c as u8` for a positive `i8` is safe (`0..=127` fits), and `unsigned_abs` handles `-128`.
//! - `u8::saturating_add_signed(c)` does the whole job in one call; writing it by hand first shows
//!   what it does.
// @solution-end

// @solution-start remove
fn step(volume: u8, change: i8) -> u8 {
    if change >= 0 {
        volume.saturating_add(change as u8)
    } else {
        volume.saturating_sub(change.unsigned_abs())
    }
}
// @solution-end

pub fn apply_changes(start: u8, changes: &[i8]) -> u8 {
    // @solution-start
    let mut volume = start;
    for &change in changes {
        volume = step(volume, change);
    }
    volume
    // @solution-end
}

pub fn count_clipped(start: u8, changes: &[i8]) -> usize {
    // @solution-start
    let mut volume = start;
    let mut clipped = 0;
    for &change in changes {
        volume = match volume.checked_add_signed(change) {
            Some(v) => v,
            None => {
                clipped += 1;
                step(volume, change)
            }
        };
    }
    clipped
    // @solution-end
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
