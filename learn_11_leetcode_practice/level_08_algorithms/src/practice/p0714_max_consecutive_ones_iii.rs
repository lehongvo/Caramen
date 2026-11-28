// ==================================================================
// PRACTICE 0714: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0714_max_consecutive_ones_iii.md
// Run the tests:          cargo test practice::p0714
// ==================================================================

//! # Problem 0714: Max Consecutive Ones III
//!
//! - **Difficulty:** Medium
//! - **Topics:** sliding window with a budget, counting inside the window
//! - **Source:** [LeetCode 1004. Max Consecutive Ones III](https://leetcode.com/problems/max-consecutive-ones-iii/)
//!
//! ## Problem
//!
//! You are given a binary vector `nums` (every value is `0` or `1`) and an integer `k`. You may
//! flip **at most** `k` zeros into ones. Return the length of the longest block of consecutive
//! ones you can obtain.
//!
//! Implement `Solution::longest_ones(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], k = 2
//! Output: 6             (flip the zeros at indices 5 and 10: [1, 1, 1, 1, 1, 1] at 5..=10)
//!
//! Input:  nums = [0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], k = 3
//! Output: 10            (indices 2..=11 after flipping the zeros at 4, 5 and 9)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `nums[i]` is `0` or `1`.
//! - `0 <= k <= nums.len()`
//!
//! ## Hints
//!
//! 1. Rephrase the question: find the longest window that contains at most `k` zeros.
//! 2. Grow the window on the right and shrink it on the left whenever it holds too many zeros.

pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_budget_two() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
    }

    #[test]
    fn example_budget_three() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        assert_eq!(Solution::longest_ones(nums, 3), 10);
    }

    #[test]
    fn no_flips_allowed() {
        assert_eq!(Solution::longest_ones(vec![1, 0, 1, 1], 0), 2);
        assert_eq!(Solution::longest_ones(vec![0, 0, 0], 0), 0);
    }

    #[test]
    fn enough_flips_for_everything() {
        assert_eq!(Solution::longest_ones(vec![0, 0, 0], 3), 3);
    }

    #[test]
    fn long_alternating_input() {
        // 0, 1, 0, 1, ...: with k = 1000 the best window holds 1000 zeros and 1001 ones.
        let nums: Vec<i32> = (0..100_000).map(|i| i % 2).collect();
        assert_eq!(Solution::longest_ones(nums, 1000), 2001);
    }
}
