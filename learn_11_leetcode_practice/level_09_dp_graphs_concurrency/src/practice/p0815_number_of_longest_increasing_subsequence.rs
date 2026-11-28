// ==================================================================
// PRACTICE 0815: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0815_number_of_longest_increasing_subsequence.md
// Run the tests:          cargo test practice::p0815
// ==================================================================

//! # Problem 0815: Number of Longest Increasing Subsequence
//!
//! - **Difficulty:** Medium
//! - **Topics:** `O(n^2)` DP with two arrays (length and count), `std::cmp::Ordering`
//! - **Source:** [LeetCode 673. Number of Longest Increasing Subsequence](https://leetcode.com/problems/number-of-longest-increasing-subsequence/)
//!
//! ## Problem
//!
//! Given an integer array `nums`, return how many **strictly increasing subsequences** have the
//! maximum possible length. Two subsequences are different if they use a different set of
//! indices, even when their values are equal.
//!
//! Implement `Solution::find_number_of_lis(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 3, 5, 4, 7]
//! Output: 2
//! Explanation: the longest length is 4: [1, 3, 4, 7] and [1, 3, 5, 7].
//!
//! Input:  nums = [2, 2, 2, 2, 2]
//! Output: 5
//! Explanation: the longest length is 1 and there are five such subsequences.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 2000`
//! - `-10^6 <= nums[i] <= 10^6`
//! - The answer fits in an `i32`.
//!
//! ## Hints
//!
//! 1. Extend the classic `O(n^2)` LIS DP: for every index `i`, store `len[i]` (longest increasing
//!    subsequence ending at `i`) **and** `cnt[i]` (how many of them).
//! 2. When `nums[j] < nums[i]`: a longer `len[j] + 1` resets the count; an equal one adds to it.

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_ways() {
        assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    }

    #[test]
    fn example_all_equal() {
        assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::find_number_of_lis(vec![42]), 1);
    }

    #[test]
    fn several_endings() {
        assert_eq!(
            Solution::find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2]),
            3
        );
    }

    #[test]
    fn doubled_values_multiply_the_count() {
        // [1, 1, 2, 2, ..., 15, 15]: each of the 15 levels offers 2 choices.
        let nums: Vec<i32> = (1..=15).flat_map(|v| [v, v]).collect();
        assert_eq!(Solution::find_number_of_lis(nums), 32_768);
    }

    #[test]
    fn huge_counts_of_shorter_subsequences() {
        // A chain of length 40 comes first; then 32 doubled values create 2^32 subsequences of
        // length 32, which overflows an i32 count but is not the longest length.
        let mut nums: Vec<i32> = (100..140).collect();
        nums.extend((1..=32).flat_map(|v| [v, v]));
        assert_eq!(Solution::find_number_of_lis(nums), 1);
    }
}
