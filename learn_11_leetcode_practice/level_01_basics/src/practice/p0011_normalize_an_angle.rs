// ==================================================================
// PRACTICE 0011: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0011_normalize_an_angle.md
// Run the tests:          cargo test practice::p0011
// ==================================================================

//! # Problem 0011: Normalize an Angle
//!
//! - **Difficulty:** Easy
//! - **Topics:** `let`, `mut`, shadowing (including shadowing that changes the type), `rem_euclid`, `as`
//! - **Source:** Original · [The Rust Book 3.1](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)
//!
//! ## Problem
//!
//! A robot receives turning commands in whole degrees. Positive values turn clockwise, negative
//! values counter-clockwise, and values can be larger than a full turn. Write two functions:
//!
//! 1. `normalize_angle(degrees: i64) -> u16` returns the equivalent heading in the range
//!    `0..360` (so `370` becomes `10` and `-90` becomes `270`).
//! 2. `final_heading(start: u16, turns: &[i64]) -> u16` starts at heading `start` (already in
//!    `0..360`), applies every turn in order and returns the final heading, also in `0..360`.
//!
//! Try to use **shadowing** in `normalize_angle`: rebind the name `degrees` to the reduced value
//! instead of inventing a new name. In `final_heading`, use a `mut` variable for the current heading.
//!
//! ## Examples
//!
//! ```text
//! normalize_angle(370)   -> 10
//! normalize_angle(-90)   -> 270
//! normalize_angle(720)   -> 0
//!
//! final_heading(0, &[90, 90, 90])     -> 270
//! final_heading(350, &[20])           -> 10
//! final_heading(10, &[-30, 400, -5])  -> 15     10 - 30 + 400 - 5 = 375 -> 15
//! ```
//!
//! ## Constraints
//!
//! - `-10^15 <= degrees <= 10^15`
//! - `0 <= start < 360`, `0 <= turns.len() <= 10^4`, every turn is in `-10^15..=10^15`
//!
//! ## Hints
//!
//! 1. `-90 % 360` is `-90` in Rust; `rem_euclid` gives a non-negative remainder.
//! 2. `let x = 5; let x = x * 2;` creates a **new** variable named `x` that hides the old one. The
//!    new one may even have a different type.
//! 3. Normalize after every turn so the running heading stays small.

pub fn normalize_angle(degrees: i64) -> u16 {
    todo!("Write your solution here")
}

pub fn final_heading(start: u16, turns: &[i64]) -> u16 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_angles() {
        assert_eq!(normalize_angle(370), 10);
        assert_eq!(normalize_angle(-90), 270);
        assert_eq!(normalize_angle(720), 0);
    }

    #[test]
    fn angles_already_in_range() {
        assert_eq!(normalize_angle(0), 0);
        assert_eq!(normalize_angle(359), 359);
        assert_eq!(normalize_angle(-360), 0);
        assert_eq!(normalize_angle(-1), 359);
    }

    #[test]
    fn huge_angles() {
        assert_eq!(normalize_angle(1_000_000_000_000_000), 280);
        assert_eq!(normalize_angle(-1_000_000_000_000_000), 80);
    }

    #[test]
    fn example_headings() {
        assert_eq!(final_heading(0, &[90, 90, 90]), 270);
        assert_eq!(final_heading(350, &[20]), 10);
        assert_eq!(final_heading(10, &[-30, 400, -5]), 15);
    }

    #[test]
    fn no_turns_keeps_start() {
        assert_eq!(final_heading(123, &[]), 123);
    }

    #[test]
    fn many_huge_turns_do_not_overflow() {
        let turns = vec![1_000_000_000_000_000; 10_000];
        // Each turn is worth 280 degrees, and 10_000 * 280 = 2_800_000 = 7_777 * 360 + 280.
        assert_eq!(final_heading(0, &turns), 280);
    }
}
