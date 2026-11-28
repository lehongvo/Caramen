// ==================================================================
// PRACTICE 0704: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0704_max_number_of_k_sum_pairs.md
// Run the tests:          cargo test practice::p0704
// ==================================================================

//! # Problem 0704: Max Number of K-Sum Pairs
//!
//! - **Difficulty:** Medium
//! - **Topics:** sorting + two pointers, `match` on `Ordering`, `i64` sums
//! - **Source:** [LeetCode 1679. Max Number of K-Sum Pairs](https://leetcode.com/problems/max-number-of-k-sum-pairs/)
//!
//! ## Problem
//!
//! You are given a vector `nums` and an integer `k`. In one operation you choose two elements at
//! different positions whose sum is exactly `k` and remove both of them from the vector.
//!
//! Return the maximum number of operations you can perform.
//!
//! Implement `Solution::max_operations(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3, 4], k = 5
//! Output: 2             (remove 1 and 4, then 2 and 3)
//!
//! Input:  nums = [3, 1, 3, 4, 3], k = 6
//! Output: 1             (remove two of the 3s; the third 3 has no partner left)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `1 <= nums[i] <= 10^9`
//! - `1 <= k <= 10^9`
//!
//! ## Hints
//!
//! 1. Removing a pair never makes another pair impossible, so you can be greedy.
//! 2. After sorting, the smallest and the largest values are at the two ends. Compare their sum
//!    with `k`.

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_pairs() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    }

    #[test]
    fn example_one_pair() {
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::max_operations(vec![1], 2), 0);
    }

    #[test]
    fn all_equal_halves() {
        assert_eq!(Solution::max_operations(vec![2, 2, 2, 2, 2], 4), 2);
    }

    #[test]
    fn large_values_no_pair() {
        assert_eq!(
            Solution::max_operations(vec![1_000_000_000, 1_000_000_000], 1_000_000_000),
            0
        );
    }

    #[test]
    fn long_input_all_pairs() {
        // 1..=100_000 with k = 100_001: (1, 100000), (2, 99999), ... gives 50_000 pairs.
        let nums: Vec<i32> = (1..=100_000).rev().collect();
        assert_eq!(Solution::max_operations(nums, 100_001), 50_000);
    }
}
