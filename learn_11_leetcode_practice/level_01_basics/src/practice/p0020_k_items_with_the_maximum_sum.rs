// ==================================================================
// PRACTICE 0020: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0020_k_items_with_the_maximum_sum.md
// Run the tests:          cargo test practice::p0020
// ==================================================================

//! # Problem 0020: K Items With the Maximum Sum
//!
//! - **Difficulty:** Easy
//! - **Topics:** `if` / `else if` chains as expressions, greedy reasoning
//! - **Source:** [LeetCode 2600. K Items With the Maximum Sum](https://leetcode.com/problems/k-items-with-the-maximum-sum/)
//!
//! ## Problem
//!
//! A bag contains `num_ones` items labeled `1`, `num_zeros` items labeled `0` and `num_neg_ones`
//! items labeled `-1`. You must take **exactly** `k` items out of the bag. Return the largest
//! possible sum of the labels of the items you take.
//!
//! Implement
//! `Solution::k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num_ones = 3, num_zeros = 2, num_neg_ones = 0, k = 2
//! Output: 2
//! Explanation: take two items labeled 1.
//!
//! Input:  num_ones = 3, num_zeros = 2, num_neg_ones = 0, k = 4
//! Output: 3
//! Explanation: take the three 1s and one 0.
//!
//! Input:  num_ones = 2, num_zeros = 1, num_neg_ones = 4, k = 5
//! Output: 0
//! Explanation: 1 + 1 + 0 + (-1) + (-1) = 0.
//! ```
//!
//! ## Constraints
//!
//! - `0 <= num_ones, num_zeros, num_neg_ones <= 50`
//! - `0 <= k <= num_ones + num_zeros + num_neg_ones`
//!
//! ## Hints
//!
//! 1. Always take the 1s first, then the 0s, and the -1s only when you have no choice.
//! 2. That gives three cases depending on how big `k` is.

pub struct Solution;

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_only_ones() {
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 2), 2);
    }

    #[test]
    fn example_ones_and_zeros() {
        assert_eq!(Solution::k_items_with_maximum_sum(3, 2, 0, 4), 3);
    }

    #[test]
    fn example_needs_negative_ones() {
        assert_eq!(Solution::k_items_with_maximum_sum(2, 1, 4, 5), 0);
    }

    #[test]
    fn take_nothing() {
        assert_eq!(Solution::k_items_with_maximum_sum(5, 5, 5, 0), 0);
    }

    #[test]
    fn take_everything() {
        assert_eq!(Solution::k_items_with_maximum_sum(50, 50, 50, 150), 0);
        assert_eq!(Solution::k_items_with_maximum_sum(0, 0, 50, 50), -50);
    }
}
