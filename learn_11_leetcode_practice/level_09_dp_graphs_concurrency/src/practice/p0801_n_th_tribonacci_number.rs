// ==================================================================
// PRACTICE 0801: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0801_n_th_tribonacci_number.md
// Run the tests:          cargo test practice::p0801
// ==================================================================

//! # Problem 0801: N-th Tribonacci Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** dynamic programming intro, rolling state, destructuring assignment `(a, b, c) = (...)`
//! - **Source:** [LeetCode 1137. N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/)
//!
//! ## Problem
//!
//! The Tribonacci sequence is like Fibonacci, but each term is the sum of the **three** terms
//! before it:
//!
//! - `T(0) = 0`, `T(1) = 1`, `T(2) = 1`;
//! - `T(k + 3) = T(k) + T(k + 1) + T(k + 2)` for every `k >= 0`.
//!
//! Given `n`, return `T(n)`.
//!
//! Implement `Solution::tribonacci(n: i32) -> i32`.
//!
//! This is the first dynamic programming problem of the level: the answer for `n` is built from
//! the answers of smaller inputs, and you only ever need the last three of them.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4
//! Output: 4
//! Explanation: T(3) = 0 + 1 + 1 = 2, T(4) = 1 + 1 + 2 = 4.
//!
//! Input:  n = 25
//! Output: 1389537
//!
//! Input:  n = 0
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 37`
//! - The answer is guaranteed to fit in an `i32` (so `T(37) = 2082876103` is the largest value).
//!
//! ## Hints
//!
//! 1. A naive recursive function recomputes the same terms an exponential number of times. Compute
//!    the terms from small to large instead.
//! 2. You do not need a whole vector: keep three variables `a, b, c` for the last three terms and
//!    "shift" them on every step.
//! 3. Careful with overflow: `T(38)` does not fit in an `i32`, so do not compute any term past `T(n)`.

pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_n_4() {
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn example_n_25() {
        assert_eq!(Solution::tribonacci(25), 1_389_537);
    }

    #[test]
    fn base_cases() {
        assert_eq!(Solution::tribonacci(0), 0);
        assert_eq!(Solution::tribonacci(1), 1);
        assert_eq!(Solution::tribonacci(2), 1);
        assert_eq!(Solution::tribonacci(3), 2);
    }

    #[test]
    fn largest_input_does_not_overflow() {
        assert_eq!(Solution::tribonacci(37), 2_082_876_103);
    }
}
