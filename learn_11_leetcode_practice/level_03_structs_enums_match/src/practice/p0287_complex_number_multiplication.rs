// ==================================================================
// PRACTICE 0287: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0287_complex_number_multiplication.md
// Run the tests:          cargo test practice::p0287
// ==================================================================

//! # Problem 0287: Complex Number Multiplication
//!
//! - **Difficulty:** Medium
//! - **Topics:** a `Complex` struct with `parse`, `times` and `text` methods, `split_once`, `strip_suffix`, `?` on `Option`
//! - **Source:** [LeetCode 537. Complex Number Multiplication](https://leetcode.com/problems/complex-number-multiplication/)
//!
//! ## Problem
//!
//! A complex number is written as the string `"real+imaginaryi"`, where `real` and `imaginary` are
//! integers in `[-100, 100]`. Negative parts keep the `+` separator: `1 - 1i` is written
//! `"1+-1i"`. The imaginary unit satisfies `i² = -1`.
//!
//! Given two complex numbers `num1` and `num2` in this format, return their product in the same
//! format.
//!
//! Implement `Solution::complex_number_multiply(num1: String, num2: String) -> String`.
//!
//! For this level, write a `Complex { re, im }` struct with `parse(&str) -> Option<Complex>`,
//! `times(self, other) -> Complex` and `text(&self) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num1 = "1+1i", num2 = "1+1i"
//! Output: "0+2i"      ((1 + i)² = 1 + 2i + i² = 2i)
//!
//! Input:  num1 = "1+-1i", num2 = "1+-1i"
//! Output: "0+-2i"     ((1 - i)² = 1 - 2i + i² = -2i)
//! ```
//!
//! ## Constraints
//!
//! - Both inputs are valid complex numbers in the format above.
//!
//! ## Hints
//!
//! 1. `(a + bi)(c + di) = (ac - bd) + (ad + bc)i`.
//! 2. `s.strip_suffix('i')?.split_once('+')?` gives the two parts as `&str`s; the imaginary part may
//!    start with `-`, which `parse::<i32>()` handles.

pub struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mul(a: &str, b: &str) -> String {
        Solution::complex_number_multiply(a.to_string(), b.to_string())
    }

    #[test]
    fn example_one() {
        assert_eq!(mul("1+1i", "1+1i"), "0+2i");
    }

    #[test]
    fn example_two() {
        assert_eq!(mul("1+-1i", "1+-1i"), "0+-2i");
    }

    #[test]
    fn negative_real_parts() {
        assert_eq!(mul("-1+0i", "-1+0i"), "1+0i");
        assert_eq!(mul("-3+4i", "2+-1i"), "-2+11i");
    }

    #[test]
    fn zero_and_extremes() {
        assert_eq!(mul("0+0i", "100+100i"), "0+0i");
        assert_eq!(mul("100+100i", "100+100i"), "0+20000i");
        assert_eq!(mul("-100+-100i", "100+-100i"), "-20000+0i");
    }
}
