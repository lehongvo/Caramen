// ==================================================================
// PRACTICE 0249: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0249_match_guards_pairs.md
// Run the tests:          cargo test practice::p0249
// ==================================================================

//! # Problem 0249: Describe Pairs with Match Guards
//!
//! - **Difficulty:** Easy
//! - **Topics:** match guards `if ...`, guards that use outer variables, guards with `|` alternatives, arm order
//! - **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#extra-conditionals-with-match-guards)
//!
//! ## Problem
//!
//! A **match guard** is an extra `if` condition on a `match` arm: the arm is chosen only if the
//! pattern matches *and* the condition is true. Implement three functions with guards:
//!
//! - `describe_point(x: i32, y: i32) -> &'static str`, checking the rules in this order:
//!   `"origin"` if both are 0; `"on an axis"` if exactly one is 0; `"diagonal"` if `x == y`;
//!   `"anti-diagonal"` if `x == -y`; `"same sign"` if both are positive or both negative;
//!   `"mixed signs"` otherwise.
//! - `compare_reading(reading: Option<i32>, limit: i32) -> &'static str`: `"missing"` for `None`,
//!   `"over"` if the value is above `limit`, `"at limit"` if equal, `"under"` otherwise. Note that
//!   `limit` is a *variable*: it must be compared in a guard (a pattern `Some(limit)` would create a
//!   **new** variable that matches anything).
//! - `ticket_price(age: u8, student: bool) -> u32`: `0..=5` free (`0`); `6..=17` costs `6`; ages
//!   `18..=25` **or** `60` and above cost `8` if `student` is true; ages `60` and above cost `9`
//!   otherwise; everybody else pays `12`.
//!
//! ## Examples
//!
//! ```text
//! describe_point(0, 0)   => "origin"
//! describe_point(0, 7)   => "on an axis"
//! describe_point(3, 3)   => "diagonal"
//! describe_point(-4, 4)  => "anti-diagonal"
//! describe_point(-1, -9) => "same sign"
//! describe_point(2, -5)  => "mixed signs"
//! compare_reading(Some(120), 100) => "over"
//! compare_reading(Some(100), 100) => "at limit"
//! compare_reading(None, 100)      => "missing"
//! ticket_price(20, true)  => 8
//! ticket_price(70, true)  => 8
//! ticket_price(70, false) => 9
//! ticket_price(30, true)  => 12
//! ```
//!
//! ## Constraints
//!
//! - No special constraints (`x == -y` must not overflow: think about `i32::MIN`).
//!
//! ## Hints
//!
//! 1. `(0, _) | (_, 0) => "on an axis"` works after the `(0, 0)` arm has been handled.
//! 2. `Some(v) if v > limit => "over"` uses the outer variable `limit` inside the guard.
//! 3. In `18..=25 | 60.. if student => 8`, the guard applies to **both** alternatives.

/// Describes where a point lies relative to the axes and diagonals.
pub fn describe_point(x: i32, y: i32) -> &'static str {
    todo!("Write your solution here")
}

/// Compares an optional reading with a limit.
pub fn compare_reading(reading: Option<i32>, limit: i32) -> &'static str {
    todo!("Write your solution here")
}

/// The ticket price for an age and student status.
pub fn ticket_price(age: u8, student: bool) -> u32 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points() {
        assert_eq!(describe_point(0, 0), "origin");
        assert_eq!(describe_point(0, 7), "on an axis");
        assert_eq!(describe_point(-2, 0), "on an axis");
        assert_eq!(describe_point(3, 3), "diagonal");
        assert_eq!(describe_point(-4, 4), "anti-diagonal");
        assert_eq!(describe_point(-1, -9), "same sign");
        assert_eq!(describe_point(5, 1), "same sign");
        assert_eq!(describe_point(2, -5), "mixed signs");
    }

    #[test]
    fn points_at_the_extremes() {
        assert_eq!(describe_point(i32::MIN, i32::MIN), "diagonal");
        assert_eq!(describe_point(i32::MAX, i32::MIN), "mixed signs");
        assert_eq!(describe_point(i32::MAX, -i32::MAX), "anti-diagonal");
    }

    #[test]
    fn readings_use_the_outer_limit() {
        assert_eq!(compare_reading(Some(120), 100), "over");
        assert_eq!(compare_reading(Some(100), 100), "at limit");
        assert_eq!(compare_reading(Some(-5), 100), "under");
        assert_eq!(compare_reading(None, 100), "missing");
        assert_eq!(compare_reading(Some(7), 7), "at limit");
    }

    #[test]
    fn ticket_prices() {
        assert_eq!(ticket_price(3, true), 0);
        assert_eq!(ticket_price(12, false), 6);
        assert_eq!(ticket_price(20, true), 8);
        assert_eq!(ticket_price(20, false), 12);
        assert_eq!(ticket_price(70, true), 8);
        assert_eq!(ticket_price(70, false), 9);
        assert_eq!(ticket_price(30, true), 12);
        assert_eq!(ticket_price(255, false), 9);
    }
}
