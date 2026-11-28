// ==================================================================
// PRACTICE 0288: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0288_fraction_addition_and_subtraction.md
// Run the tests:          cargo test practice::p0288
// ==================================================================

//! # Problem 0288: Fraction Addition and Subtraction
//!
//! - **Difficulty:** Medium
//! - **Topics:** a `Fraction` struct that keeps itself normalised, a constructor enforcing invariants, scanning signed numbers, `fold`
//! - **Source:** [LeetCode 592. Fraction Addition and Subtraction](https://leetcode.com/problems/fraction-addition-and-subtraction/)
//!
//! ## Problem
//!
//! You are given a string `expression` that adds and subtracts fractions, such as `"-1/2+1/3"`.
//! Compute the result and return it as an **irreducible** fraction `"numerator/denominator"`:
//!
//! - the denominator is always positive;
//! - a whole number `n` is written `"n/1"` (so zero is `"0/1"`);
//! - a negative result has the sign on the numerator (`"-1/6"`).
//!
//! Every fraction in the input has the form `±a/b` (the first one may have no sign), with
//! `1 <= a, b <= 10`, and there are at most 10 fractions.
//!
//! Implement `Solution::fraction_addition(expression: String) -> String`.
//!
//! For this level, write a `Fraction { num, den }` struct whose constructor always reduces it and
//! keeps `den > 0`, and a `plus` method.
//!
//! ## Examples
//!
//! ```text
//! Input:  expression = "-1/2+1/2"
//! Output: "0/1"
//!
//! Input:  expression = "-1/2+1/2+1/3"
//! Output: "1/3"
//!
//! Input:  expression = "1/3-1/2"
//! Output: "-1/6"
//! ```
//!
//! ## Constraints
//!
//! - `expression` contains only digits, `/`, `+` and `-`; it is valid.
//! - Numerators and denominators stay within `i32` during the computation.
//!
//! ## Hints
//!
//! 1. `a/b + c/d = (a*d + c*b) / (b*d)`; reduce by `gcd(|num|, den)` after every addition.
//! 2. To read the fractions, start a new piece at every `+` or `-` that is not at position 0:
//!    `"1/3-1/2"` becomes `"1/3"` and `"-1/2"`.

pub struct Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add(e: &str) -> String {
        Solution::fraction_addition(e.to_string())
    }

    #[test]
    fn example_zero() {
        assert_eq!(add("-1/2+1/2"), "0/1");
    }

    #[test]
    fn example_one_third() {
        assert_eq!(add("-1/2+1/2+1/3"), "1/3");
    }

    #[test]
    fn example_negative() {
        assert_eq!(add("1/3-1/2"), "-1/6");
    }

    #[test]
    fn whole_numbers_and_single_fractions() {
        assert_eq!(add("5/3+1/3"), "2/1");
        assert_eq!(add("2/4"), "1/2");
        assert_eq!(add("-10/10"), "-1/1");
        assert_eq!(add("+3/9"), "1/3");
    }

    #[test]
    fn ten_fractions() {
        // 1/1 + 1/2 + ... + 1/10 = 7381/2520
        let e = "1/1+1/2+1/3+1/4+1/5+1/6+1/7+1/8+1/9+1/10";
        assert_eq!(add(e), "7381/2520");
        let neg = "-1/1-1/2-1/3-1/4-1/5-1/6-1/7-1/8-1/9-1/10";
        assert_eq!(add(neg), "-7381/2520");
    }
}
