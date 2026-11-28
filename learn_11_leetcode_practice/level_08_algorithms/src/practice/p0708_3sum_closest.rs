// ==================================================================
// PRACTICE 0708: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0708_3sum_closest.md
// Run the tests:          cargo test practice::p0708
// ==================================================================

//! # Problem 0708: 3Sum Closest
//!
//! - **Difficulty:** Medium
//! - **Topics:** sorting + two pointers, tracking the best candidate, `abs`, early return
//! - **Source:** [LeetCode 16. 3Sum Closest](https://leetcode.com/problems/3sum-closest/)
//!
//! ## Problem
//!
//! Given a vector `nums` and an integer `target`, choose three values at three different positions
//! whose sum is as close as possible to `target` (smallest absolute difference). Return that
//! **sum** (not the difference).
//!
//! The inputs are chosen so that exactly one closest sum exists.
//!
//! Implement `Solution::three_sum_closest(nums: Vec<i32>, target: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [-1, 2, 1, -4], target = 1
//! Output: 2             (-1 + 2 + 1 = 2 is at distance 1 from the target)
//!
//! Input:  nums = [0, 0, 0], target = 1
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `3 <= nums.len() <= 500`
//! - `-1000 <= nums[i] <= 1000`
//! - `-10^4 <= target <= 10^4`
//!
//! ## Hints
//!
//! 1. This is 3Sum with "equal to zero" replaced by "as close as possible". The same sorted
//!    two-pointer scan applies.
//! 2. When the sum is too small, moving the left pointer is the only move that can bring it closer.

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed_signs() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn example_three_zeros() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn target_far_below() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    }

    #[test]
    fn exact_match_exists() {
        // Sorted: -5 -5 -4 0 0 3 3 4 5 and -5 + 0 + 3 = -2.
        assert_eq!(
            Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        );
    }

    #[test]
    fn target_far_above() {
        let nums: Vec<i32> = (-250..250).collect();
        // The largest possible sum is 249 + 248 + 247.
        assert_eq!(Solution::three_sum_closest(nums, 10_000), 744);
    }
}
