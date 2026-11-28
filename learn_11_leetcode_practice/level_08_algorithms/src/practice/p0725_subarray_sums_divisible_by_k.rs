// ==================================================================
// PRACTICE 0725: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0725_subarray_sums_divisible_by_k.md
// Run the tests:          cargo test practice::p0725
// ==================================================================

//! # Problem 0725: Subarray Sums Divisible by K
//!
//! - **Difficulty:** Medium
//! - **Topics:** prefix remainders, counting equal pairs, `rem_euclid` for negative numbers
//! - **Source:** [LeetCode 974. Subarray Sums Divisible by K](https://leetcode.com/problems/subarray-sums-divisible-by-k/)
//!
//! ## Problem
//!
//! Given a vector `nums` (values may be negative) and an integer `k`, return the number of
//! non-empty **contiguous** subarrays whose sum is divisible by `k`.
//!
//! Implement `Solution::subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [4, 5, 0, -2, -3, 1], k = 5
//! Output: 7
//!   ([4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3])
//!
//! Input:  nums = [5], k = 9
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 3 * 10^4`
//! - `-10^4 <= nums[i] <= 10^4`
//! - `2 <= k <= 10^4`
//!
//! ## Hints
//!
//! 1. A subarray is divisible by `k` exactly when the two prefix sums around it have the same
//!    remainder modulo `k`.
//! 2. Count how many earlier prefixes had each remainder.
//! 3. In Rust, `-3 % 5` is `-3`, not `2`. Which method gives the mathematical remainder?

pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_seven_subarrays() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }

    #[test]
    fn example_none() {
        assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
    }

    #[test]
    fn negative_values_need_euclidean_remainder() {
        // [2] and [-1, 2, 9] are even.
        assert_eq!(Solution::subarrays_div_by_k(vec![-1, 2, 9], 2), 2);
        assert_eq!(Solution::subarrays_div_by_k(vec![-5, 5], 5), 3);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(Solution::subarrays_div_by_k(vec![0, 0, 0], 3), 6);
    }

    #[test]
    fn every_subarray_counts() {
        // 30_000 * 30_001 / 2 subarrays, all divisible by 5.
        assert_eq!(
            Solution::subarrays_div_by_k(vec![5; 30_000], 5),
            450_015_000
        );
    }
}
