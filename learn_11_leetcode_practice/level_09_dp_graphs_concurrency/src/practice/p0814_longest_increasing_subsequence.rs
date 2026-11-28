// ==================================================================
// PRACTICE 0814: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0814_longest_increasing_subsequence.md
// Run the tests:          cargo test practice::p0814
// ==================================================================

//! # Problem 0814: Longest Increasing Subsequence
//!
//! - **Difficulty:** Medium
//! - **Topics:** DP + binary search ("patience sorting"), `slice::partition_point`
//! - **Source:** [LeetCode 300. Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/)
//!
//! ## Problem
//!
//! Given an integer array `nums`, return the length of its longest **strictly increasing
//! subsequence**. A subsequence keeps the original order but may skip elements: `[3, 6, 2, 7]` is a
//! subsequence of `[0, 3, 1, 6, 2, 2, 7]`.
//!
//! Implement `Solution::length_of_lis(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [10, 9, 2, 5, 3, 7, 101, 18]
//! Output: 4
//! Explanation: one longest subsequence is [2, 3, 7, 101].
//!
//! Input:  nums = [0, 1, 0, 3, 2, 3]
//! Output: 4
//!
//! Input:  nums = [7, 7, 7, 7]
//! Output: 1
//! Explanation: "strictly" increasing, so equal values cannot follow each other.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 2500`
//! - `-10^4 <= nums[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. The `O(n^2)` DP: `len[i] = 1 + max(len[j])` over `j < i` with `nums[j] < nums[i]`.
//! 2. Faster: keep `tails[k]` = the smallest possible last element of an increasing subsequence of
//!    length `k + 1`. This array is always sorted.
//! 3. For each new value, binary-search the first tail that is `>= value` and replace it (or push
//!    the value if there is none).

pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn example_with_repeats() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn example_all_equal() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7]), 1);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::length_of_lis(vec![-5]), 1);
    }

    #[test]
    fn sorted_and_reversed_inputs() {
        let up: Vec<i32> = (0..2500).collect();
        let down: Vec<i32> = (0..2500).rev().collect();
        assert_eq!(Solution::length_of_lis(up), 2500);
        assert_eq!(Solution::length_of_lis(down), 1);
    }
}
