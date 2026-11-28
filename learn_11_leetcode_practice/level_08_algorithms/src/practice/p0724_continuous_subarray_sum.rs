// ==================================================================
// PRACTICE 0724: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0724_continuous_subarray_sum.md
// Run the tests:          cargo test practice::p0724
// ==================================================================

//! # Problem 0724: Continuous Subarray Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** prefix remainders, `HashMap` of first positions, `i64` to avoid overflow
//! - **Source:** [LeetCode 523. Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum/)
//!
//! ## Problem
//!
//! Given a vector `nums` of non-negative integers and a positive integer `k`, return `true` if
//! `nums` has a **good** subarray, and `false` otherwise. A good subarray:
//!
//! - is contiguous and has **at least two** elements, and
//! - has a sum that is a multiple of `k` (`0` counts as a multiple of every `k`).
//!
//! Implement `Solution::check_subarray_sum(nums: Vec<i32>, k: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [23, 2, 4, 6, 7], k = 6
//! Output: true          ([2, 4] sums to 6)
//!
//! Input:  nums = [23, 2, 6, 4, 7], k = 6
//! Output: true          (the whole array sums to 42 = 7 * 6)
//!
//! Input:  nums = [23, 2, 6, 4, 7], k = 13
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `0 <= nums[i] <= 10^9`
//! - `0 <= sum(nums) <= 2^31 - 1`
//! - `1 <= k <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. `sum(nums[a..b]) = prefix(b) - prefix(a)`. When is that difference a multiple of `k`?
//! 2. Two prefixes with the same remainder modulo `k` enclose a multiple of `k`. The subarray must
//!    also be at least two long, so remember the **first** position of each remainder.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_short_subarray() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }

    #[test]
    fn example_whole_array() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    }

    #[test]
    fn example_no_good_subarray() {
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }

    #[test]
    fn single_element_is_too_short() {
        assert!(!Solution::check_subarray_sum(vec![0], 1));
        assert!(!Solution::check_subarray_sum(vec![6], 6));
    }

    #[test]
    fn zeros_sum_to_a_multiple() {
        assert!(Solution::check_subarray_sum(vec![0, 0], 1));
        assert!(Solution::check_subarray_sum(vec![5, 0, 0, 0], 3));
    }

    #[test]
    fn remainder_repeats_but_too_close() {
        // Prefix remainders mod 2: 0, 1, 1. The repeat is only one element apart.
        assert!(!Solution::check_subarray_sum(vec![1, 0], 2));
    }

    #[test]
    fn huge_values_and_k() {
        let nums = vec![1_000_000_000, 1_000_000_000];
        assert!(Solution::check_subarray_sum(nums.clone(), 2_000_000_000));
        assert!(!Solution::check_subarray_sum(nums, i32::MAX));
    }
}
