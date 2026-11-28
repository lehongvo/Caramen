// ==================================================================
// PRACTICE 0811: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0811_combination_sum_iv.md
// Run the tests:          cargo test practice::p0811
// ==================================================================

//! # Problem 0811: Combination Sum IV
//!
//! - **Difficulty:** Medium
//! - **Topics:** counting ordered sequences with DP, loop order vs problem 0810, `wrapping_add`
//! - **Source:** [LeetCode 377. Combination Sum IV](https://leetcode.com/problems/combination-sum-iv/)
//!
//! ## Problem
//!
//! Given an array of **distinct** positive integers `nums` and a positive `target`, return the
//! number of **ordered** sequences of elements of `nums` (each element may be used any number of
//! times) whose sum is exactly `target`. Unlike problem 0810, order matters here: `1 + 2` and
//! `2 + 1` are two different sequences.
//!
//! Implement `Solution::combination_sum4(nums: Vec<i32>, target: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3], target = 4
//! Output: 7
//! Explanation: (1,1,1,1) (1,1,2) (1,2,1) (2,1,1) (2,2) (1,3) (3,1).
//!
//! Input:  nums = [9], target = 3
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 200`
//! - `1 <= nums[i] <= 1000`, all values distinct
//! - `1 <= target <= 1000`
//! - The answer fits in an `i32` (intermediate counts may not).
//!
//! ## Hints
//!
//! 1. Let `ways[t]` be the number of sequences that sum to `t`. The **last** element of such a
//!    sequence is some `x` in `nums` with `x <= t`.
//! 2. That gives `ways[t] = sum of ways[t - x]`. Compare with problem 0810: here the target is the
//!    outer loop and the numbers the inner loop.
//! 3. Counts for small targets can be astronomically large even when the final answer is small.

pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_target_four() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }

    #[test]
    fn example_unreachable() {
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
    }

    #[test]
    fn ones_and_twos_give_fibonacci() {
        assert_eq!(Solution::combination_sum4(vec![1, 2], 10), 89);
        assert_eq!(Solution::combination_sum4(vec![1], 1), 1);
    }

    #[test]
    fn large_answer() {
        assert_eq!(Solution::combination_sum4(vec![2, 1, 3], 35), 1_132_436_852);
    }

    #[test]
    fn intermediate_overflow_is_harmless() {
        let nums: Vec<i32> = (10..=1000).step_by(10).collect();
        assert_eq!(Solution::combination_sum4(nums, 999), 0);
    }
}
