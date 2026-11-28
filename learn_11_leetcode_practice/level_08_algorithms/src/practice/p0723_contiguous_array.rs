// ==================================================================
// PRACTICE 0723: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0723_contiguous_array.md
// Run the tests:          cargo test practice::p0723
// ==================================================================

//! # Problem 0723: Contiguous Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** prefix balance, first occurrence in a `HashMap`, `entry().or_insert`
//! - **Source:** [LeetCode 525. Contiguous Array](https://leetcode.com/problems/contiguous-array/)
//!
//! ## Problem
//!
//! Given a binary vector `nums` (values `0` and `1`), return the length of the longest contiguous
//! subarray that contains the **same number** of `0`s and `1`s. Return `0` if there is none.
//!
//! Implement `Solution::find_max_length(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [0, 1]
//! Output: 2
//!
//! Input:  nums = [0, 1, 0]
//! Output: 2             ([0, 1] or [1, 0])
//!
//! Input:  nums = [0, 1, 1, 1, 1, 1, 0, 0, 0]
//! Output: 6             ([1, 1, 1, 0, 0, 0])
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `nums[i]` is `0` or `1`.
//!
//! ## Hints
//!
//! 1. Count a `1` as `+1` and a `0` as `-1`. A subarray is balanced exactly when its total is `0`.
//! 2. A subarray's total is the difference of two prefix totals. When are two prefix totals equal?
//! 3. For the longest subarray, remember the **first** place each prefix total appears.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_pair() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }

    #[test]
    fn example_long_block() {
        assert_eq!(
            Solution::find_max_length(vec![0, 1, 1, 1, 1, 1, 0, 0, 0]),
            6
        );
    }

    #[test]
    fn no_balanced_subarray() {
        assert_eq!(Solution::find_max_length(vec![1, 1, 1]), 0);
    }

    #[test]
    fn balanced_block_in_the_middle() {
        assert_eq!(Solution::find_max_length(vec![0, 0, 1, 0, 0, 0, 1, 1]), 6);
    }

    #[test]
    fn long_alternating_input() {
        let nums: Vec<i32> = (0..100_000).map(|i| i % 2).collect();
        assert_eq!(Solution::find_max_length(nums), 100_000);
    }
}
