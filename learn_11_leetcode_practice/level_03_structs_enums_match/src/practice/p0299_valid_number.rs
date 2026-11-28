// ==================================================================
// PRACTICE 0299: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0299_valid_number.md
// Run the tests:          cargo test practice::p0299
// ==================================================================

//! # Problem 0299: Valid Number
//!
//! - **Difficulty:** Hard
//! - **Topics:** finite state machine with an `enum`, classifying characters into an `enum`, `match (state, class)` transition table, accepting states with `matches!`
//! - **Source:** [LeetCode 65. Valid Number](https://leetcode.com/problems/valid-number/)
//!
//! ## Problem
//!
//! Decide whether the string `s` is a valid number. A valid number is
//!
//! - an **integer** or a **decimal**, optionally followed by an **exponent**: `e` or `E` and then
//!   an integer.
//!
//! where
//!
//! - an **integer** is an optional sign (`+` or `-`) followed by one or more digits;
//! - a **decimal** is an optional sign followed by one of: digits then `.` (`"4."`), digits then `.`
//!   then digits (`"4.5"`), or `.` then digits (`".5"`).
//!
//! Nothing else is allowed: no spaces, no letters other than one `e`/`E`.
//!
//! Implement `Solution::is_number(s: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Valid:   "2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"
//! Invalid: "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", ".", "+", "1e+"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 20`; `s` contains English letters, digits, `+`, `-` and `.`.
//!
//! ## Hints
//!
//! 1. Classify each character first: `enum Class { Digit, Sign, Dot, Exp, Other }`.
//! 2. Draw the states: start, after a sign, in the integer digits, after a dot with or without
//!    digits before it, in the fraction digits, after `e`, after the exponent sign, in the exponent
//!    digits. List the allowed transitions; everything else rejects.
//! 3. Only some states may end the string: those right after at least one digit of the mantissa
//!    (or `"4."`), and the exponent digits.

pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid(s: &str) -> bool {
        Solution::is_number(s.to_string())
    }

    #[test]
    fn valid_examples() {
        for s in [
            "2",
            "0089",
            "-0.1",
            "+3.14",
            "4.",
            "-.9",
            "2e10",
            "-90E3",
            "3e+7",
            "+6e-1",
            "53.5e93",
            "-123.456e789",
        ] {
            assert!(valid(s), "{s} should be valid");
        }
    }

    #[test]
    fn invalid_examples() {
        for s in [
            "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", ".", "+", "1e+",
        ] {
            assert!(!valid(s), "{s} should be invalid");
        }
    }

    #[test]
    fn dots_and_exponents() {
        assert!(valid(".5"));
        assert!(valid("5.e3"));
        assert!(!valid(".e3"));
        assert!(!valid("5..3"));
        assert!(!valid("5e3e3"));
        assert!(!valid("5e3.0"));
        assert!(!valid("1.2.3"));
    }

    #[test]
    fn signs_only_at_the_start_of_parts() {
        assert!(!valid("1+2"));
        assert!(!valid("+-1"));
        assert!(valid("-1E-0"));
        assert!(!valid("1e--1"));
        assert!(!valid("-"));
    }
}
