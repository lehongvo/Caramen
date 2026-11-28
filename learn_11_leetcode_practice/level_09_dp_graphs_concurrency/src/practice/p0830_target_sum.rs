// ==================================================================
// PRACTICE 0830: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0830_target_sum.md
// Run the tests:          cargo test practice::p0830
// ==================================================================

//! # Problem 0830: Target Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** reducing a +/- assignment to subset counting, 0/1 knapsack (counting), `checked`
//!   arithmetic for impossible targets
//! - **Source:** [LeetCode 494. Target Sum](https://leetcode.com/problems/target-sum/)
//!
//! ## Problem
//!
//! You are given an array of non-negative integers `nums` and an integer `target`. Put a `+` or a
//! `-` sign in front of **every** element and evaluate the resulting expression. Return the number
//! of different sign assignments whose value equals `target`.
//!
//! Implement `Solution::find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 1, 1, 1], target = 3
//! Output: 5
//! Explanation: -1+1+1+1+1, +1-1+1+1+1, +1+1-1+1+1, +1+1+1-1+1, +1+1+1+1-1.
//!
//! Input:  nums = [1], target = 1
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 20`
//! - `0 <= nums[i] <= 1000`
//! - `0 <= sum(nums) <= 1000`
//! - `-1000 <= target <= 1000`
//!
//! ## Hints
//!
//! 1. Let `P` be the sum of the elements with `+` and `N` the sum of those with `-`. Then
//!    `P - N = target` and `P + N = total`.
//! 2. So `P = (total + target) / 2`: count the subsets with that sum. If `total + target` is
//!    negative or odd, there is no way at all.
//! 3. Zeros are fine: a `0` can take either sign, and the counting DP doubles the count naturally.

pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_five_ones() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn example_single() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }

    #[test]
    fn unreachable_targets() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 2), 0);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 2], -4), 0);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 2], 1000), 0);
    }

    #[test]
    fn zeros_double_the_count() {
        let nums = vec![0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(Solution::find_target_sum_ways(nums, 1), 256);
    }

    #[test]
    fn negative_target() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 2, 3], -6), 1);
    }

    #[test]
    fn mixed_values() {
        let nums = vec![2, 7, 4, 1, 8, 1, 9, 3, 5, 6, 2, 4];
        assert_eq!(Solution::find_target_sum_ways(nums, 10), 156);
    }

    #[test]
    fn twenty_zeros() {
        assert_eq!(Solution::find_target_sum_ways(vec![0; 20], 0), 1 << 20);
    }
}
