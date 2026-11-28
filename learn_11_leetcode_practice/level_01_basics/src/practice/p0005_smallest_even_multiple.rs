// ==================================================================
// PRACTICE 0005: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0005_smallest_even_multiple.md
// Run the tests:          cargo test practice::p0005
// ==================================================================

//! # Problem 0005: Smallest Even Multiple
//!
//! - **Difficulty:** Easy
//! - **Topics:** `if` as an expression, `%` to test divisibility
//! - **Source:** [LeetCode 2413. Smallest Even Multiple](https://leetcode.com/problems/smallest-even-multiple/)
//!
//! ## Problem
//!
//! Given a positive integer `n`, return the smallest positive integer that is a multiple of both
//! `2` and `n`.
//!
//! Implement `Solution::smallest_even_multiple(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 5
//! Output: 10
//! Explanation: 10 is the smallest number divisible by both 5 and 2.
//!
//! Input:  n = 6
//! Output: 6
//! Explanation: 6 is already even, so it is a multiple of 2 and of itself.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 150`
//!
//! ## Hints
//!
//! 1. If `n` is even, `n` itself works. If `n` is odd, what is the next multiple of `n`?
//! 2. In Rust, `if` is an expression: both branches can produce a value and the whole `if` is that value.

pub struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_odd() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
    }

    #[test]
    fn example_even() {
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }

    #[test]
    fn one_and_two() {
        assert_eq!(Solution::smallest_even_multiple(1), 2);
        assert_eq!(Solution::smallest_even_multiple(2), 2);
    }

    #[test]
    fn largest_values() {
        assert_eq!(Solution::smallest_even_multiple(149), 298);
        assert_eq!(Solution::smallest_even_multiple(150), 150);
    }
}
