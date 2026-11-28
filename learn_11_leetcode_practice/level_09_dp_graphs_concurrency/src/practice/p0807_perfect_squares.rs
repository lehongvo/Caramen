// ==================================================================
// PRACTICE 0807: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0807_perfect_squares.md
// Run the tests:          cargo test practice::p0807
// ==================================================================

//! # Problem 0807: Perfect Squares
//!
//! - **Difficulty:** Medium
//! - **Topics:** unbounded "minimum count" DP, `take_while` on an infinite range, `map`/`min`
//! - **Source:** [LeetCode 279. Perfect Squares](https://leetcode.com/problems/perfect-squares/)
//!
//! ## Problem
//!
//! A perfect square is the square of a positive integer: `1, 4, 9, 16, ...`. Given `n`, return the
//! smallest number of perfect squares (repetition allowed) whose sum is exactly `n`.
//!
//! Implement `Solution::num_squares(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 12
//! Output: 3
//! Explanation: 12 = 4 + 4 + 4.
//!
//! Input:  n = 13
//! Output: 2
//! Explanation: 13 = 4 + 9.
//!
//! Input:  n = 7
//! Output: 4
//! Explanation: 7 = 4 + 1 + 1 + 1; no shorter sum exists.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^4`
//!
//! ## Hints
//!
//! 1. Let `dp[i]` be the answer for `i`. The last square in an optimal sum is some `s * s <= i`.
//! 2. `dp[i] = 1 + min(dp[i - s * s])` over all such `s`, with `dp[0] = 0`.

pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_twelve() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn example_thirteen() {
        assert_eq!(Solution::num_squares(13), 2);
    }

    #[test]
    fn seven_needs_four() {
        assert_eq!(Solution::num_squares(7), 4);
    }

    #[test]
    fn perfect_squares_need_one() {
        assert_eq!(Solution::num_squares(1), 1);
        assert_eq!(Solution::num_squares(10_000), 1);
    }

    #[test]
    fn largest_four_square_case() {
        // 9999 = 8 * 1249 + 7, and numbers of the form 8k + 7 always need four squares.
        assert_eq!(Solution::num_squares(9_999), 4);
    }
}
