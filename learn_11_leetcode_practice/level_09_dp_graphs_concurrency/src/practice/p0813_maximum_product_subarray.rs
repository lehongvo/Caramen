// ==================================================================
// PRACTICE 0813: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0813_maximum_product_subarray.md
// Run the tests:          cargo test practice::p0813
// ==================================================================

//! # Problem 0813: Maximum Product Subarray
//!
//! - **Difficulty:** Medium
//! - **Topics:** DP that tracks both a maximum and a minimum, `std::mem::swap`
//! - **Source:** [LeetCode 152. Maximum Product Subarray](https://leetcode.com/problems/maximum-product-subarray/)
//!
//! ## Problem
//!
//! Given an integer array `nums`, find the non-empty contiguous subarray with the largest product
//! and return that product.
//!
//! Implement `Solution::max_product(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 3, -2, 4]
//! Output: 6
//! Explanation: the subarray [2, 3] has product 6.
//!
//! Input:  nums = [-2, 0, -1]
//! Output: 0
//! Explanation: [-2, -1] is not contiguous, so the best is [0].
//!
//! Input:  nums = [-2, 3, -4]
//! Output: 24
//! Explanation: two negatives make a positive: the whole array.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 2 * 10^4`
//! - `-10 <= nums[i] <= 10`
//! - The product of any subarray fits in an `i32`.
//!
//! ## Hints
//!
//! 1. With sums (Kadane's algorithm) you only track the best sum ending here. With products, a very
//!    negative product can become the best one after multiplying by a negative number.
//! 2. Track both the largest and the smallest product of a subarray ending at the current index.

pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_positive_run() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    }

    #[test]
    fn example_zero_wins() {
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }

    #[test]
    fn example_two_negatives() {
        assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
    }

    #[test]
    fn single_negative_element() {
        assert_eq!(Solution::max_product(vec![-2]), -2);
    }

    #[test]
    fn zeros_split_the_array() {
        assert_eq!(Solution::max_product(vec![-1, -2, -3, 0, -4, -5]), 20);
        assert_eq!(Solution::max_product(vec![0, 2]), 2);
    }

    #[test]
    fn long_array_of_ones_and_minus_ones() {
        let mut nums = vec![1; 20_000];
        nums[10] = -1;
        nums[19_000] = -1;
        assert_eq!(Solution::max_product(nums), 1);
    }
}
