// ==================================================================
// PRACTICE 0332: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0332_missing_number.md
// Run the tests:          cargo test practice::p0332
// ==================================================================

//! # Problem 0332: Missing Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** building a `HashSet` with `collect`, `contains`, `find` over a range, the sum formula
//! - **Source:** [LeetCode 268. Missing Number](https://leetcode.com/problems/missing-number/)
//!
//! ## Problem
//!
//! The vector `nums` contains `n` **distinct** numbers taken from the range `0..=n` (which has
//! `n + 1` numbers). Exactly one number of that range is therefore missing. Return it.
//!
//! Implement `Solution::missing_number(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 0, 1]
//! Output: 2          (n = 3, the range is 0..=3)
//!
//! Input:  nums = [0, 1]
//! Output: 2          (n = 2, the range is 0..=2)
//!
//! Input:  nums = [9, 6, 4, 2, 3, 5, 7, 0, 1]
//! Output: 8
//! ```
//!
//! ## Constraints
//!
//! - `n == nums.len()`, `1 <= n <= 10^4`
//! - `0 <= nums[i] <= n`, all values distinct
//!
//! ## Hints
//!
//! 1. Put the numbers in a `HashSet`, then look for the first value of `0..=n` that is not in it.
//! 2. Follow-up: the numbers `0..=n` add up to `n * (n + 1) / 2`. What does that tell you?

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn missing_zero_and_single() {
        assert_eq!(Solution::missing_number(vec![1]), 0);
        assert_eq!(Solution::missing_number(vec![0]), 1);
        assert_eq!(Solution::missing_number(vec![2, 1]), 0);
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..=10_000).rev().filter(|&x| x != 4_321).collect();
        assert_eq!(nums.len(), 10_000);
        assert_eq!(Solution::missing_number(nums), 4_321);
    }
}
