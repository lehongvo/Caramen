// ==================================================================
// PRACTICE 0057: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0057_divisible_and_non_divisible_sums_difference.md
// Run the tests:          cargo test practice::p0057
// ==================================================================

//! # Problem 0057: Divisible and Non-divisible Sums Difference
//!
//! - **Difficulty:** Easy
//! - **Topics:** `for` loops with `if` / `else`, two accumulators, integer formulas
//! - **Source:** [LeetCode 2894. Divisible and Non-divisible Sums Difference](https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/)
//!
//! ## Problem
//!
//! You are given two positive integers `n` and `m`. Split the integers `1, 2, ..., n` into two groups:
//!
//! - `num1` is the sum of those **not** divisible by `m`;
//! - `num2` is the sum of those divisible by `m`.
//!
//! Return `num1 - num2`.
//!
//! Implement `Solution::difference_of_sums(n: i32, m: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 10, m = 3
//! Output: 19
//! Explanation: num1 = 1+2+4+5+7+8+10 = 37 and num2 = 3+6+9 = 18; 37 - 18 = 19.
//!
//! Input:  n = 5, m = 6
//! Output: 15
//! Explanation: nothing in 1..=5 is divisible by 6, so num1 = 15 and num2 = 0.
//!
//! Input:  n = 5, m = 1
//! Output: -15
//! Explanation: everything is divisible by 1, so num1 = 0 and num2 = 15.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n, m <= 1000`
//!
//! ## Hints
//!
//! 1. Loop over `1..=n` and add each number to one of two sums.
//! 2. Or: the total `1 + ... + n` is `n * (n + 1) / 2`, and the multiples of `m` up to `n` are
//!    `m * (1 + 2 + ... + n / m)`.

pub struct Solution;

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ten_three() {
        assert_eq!(Solution::difference_of_sums(10, 3), 19);
    }

    #[test]
    fn example_no_multiples() {
        assert_eq!(Solution::difference_of_sums(5, 6), 15);
    }

    #[test]
    fn example_all_multiples() {
        assert_eq!(Solution::difference_of_sums(5, 1), -15);
    }

    #[test]
    fn matches_closed_formula() {
        for n in 1..=200 {
            for m in [1, 2, 3, 7, 50, 199, 1000] {
                let total = n * (n + 1) / 2;
                let k = n / m;
                let multiples = m * k * (k + 1) / 2;
                assert_eq!(Solution::difference_of_sums(n, m), total - 2 * multiples);
            }
        }
    }

    #[test]
    fn largest_input() {
        assert_eq!(Solution::difference_of_sums(1000, 1000), 500_500 - 2000);
    }
}
