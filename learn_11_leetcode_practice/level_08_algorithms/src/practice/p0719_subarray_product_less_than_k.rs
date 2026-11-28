// ==================================================================
// PRACTICE 0719: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0719_subarray_product_less_than_k.md
// Run the tests:          cargo test practice::p0719
// ==================================================================

//! # Problem 0719: Subarray Product Less Than K
//!
//! - **Difficulty:** Medium
//! - **Topics:** sliding window that counts subarrays, `i64` products, guard clauses
//! - **Source:** [LeetCode 713. Subarray Product Less Than K](https://leetcode.com/problems/subarray-product-less-than-k/)
//!
//! ## Problem
//!
//! Given a vector `nums` of positive integers and an integer `k`, return how many **contiguous**
//! subarrays have a product of all their elements **strictly less** than `k`.
//!
//! Implement `Solution::num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [10, 5, 2, 6], k = 100
//! Output: 8             ([10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6])
//!
//! Input:  nums = [1, 2, 3], k = 0
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 3 * 10^4`
//! - `1 <= nums[i] <= 1000`
//! - `0 <= k <= 10^6`
//!
//! ## Hints
//!
//! 1. All values are at least 1, so extending a subarray never decreases its product.
//! 2. For each right end, find the smallest left end that keeps the product below `k`. How many
//!    valid subarrays end at `right` then?

pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_eight_subarrays() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
    }

    #[test]
    fn example_k_zero() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }

    #[test]
    fn k_one_allows_nothing() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 1, 1], 1),
            0
        );
    }

    #[test]
    fn element_equal_to_k() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![5, 1, 5], 5),
            1
        );
    }

    #[test]
    fn all_ones_count_every_subarray() {
        // 30_000 * 30_001 / 2 subarrays.
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1; 30_000], 2),
            450_015_000
        );
    }
}
