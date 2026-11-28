// ==================================================================
// PRACTICE 0033: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0033_sign_of_the_product_of_an_array.md
// Run the tests:          cargo test practice::p0033
// ==================================================================

//! # Problem 0033: Sign of the Product of an Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** avoiding overflow by reasoning, `match` with guards, early `return`
//! - **Source:** [LeetCode 1822. Sign of the Product of an Array](https://leetcode.com/problems/sign-of-the-product-of-an-array/)
//!
//! ## Problem
//!
//! Let `sign(x)` be `1` if `x` is positive, `-1` if `x` is negative and `0` if `x` is zero. Given a
//! vector `nums`, let `product` be the product of all its values. Return `sign(product)`.
//!
//! Implement `Solution::array_sign(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [-1, -2, -3, -4, 3, 2, 1]
//! Output: 1
//! Explanation: the product is 144, which is positive.
//!
//! Input:  nums = [1, 5, 0, 2, -3]
//! Output: 0
//!
//! Input:  nums = [-1, 1, -1, 1, -1]
//! Output: -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 1000`
//! - `-100 <= nums[i] <= 100`
//!
//! ## Hints
//!
//! 1. 1000 values of size 100 have a product of up to `10^2000`: computing it overflows every
//!    integer type. You only need the sign.
//! 2. A single zero makes the product zero. Otherwise, only the **number** of negative values matters.

pub struct Solution;

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_positive() {
        assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    }

    #[test]
    fn example_zero() {
        assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
    }

    #[test]
    fn example_negative() {
        assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }

    #[test]
    fn single_elements() {
        assert_eq!(Solution::array_sign(vec![7]), 1);
        assert_eq!(Solution::array_sign(vec![-7]), -1);
        assert_eq!(Solution::array_sign(vec![0]), 0);
    }

    #[test]
    fn huge_product_does_not_overflow() {
        assert_eq!(Solution::array_sign(vec![-100; 1000]), 1);
        assert_eq!(Solution::array_sign(vec![-100; 999]), -1);
        let mut with_zero = vec![100; 1000];
        with_zero[999] = 0;
        assert_eq!(Solution::array_sign(with_zero), 0);
    }
}
