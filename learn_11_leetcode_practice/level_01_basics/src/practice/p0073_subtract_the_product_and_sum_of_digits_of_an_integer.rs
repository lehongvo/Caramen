// ==================================================================
// PRACTICE 0073: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0073_subtract_the_product_and_sum_of_digits_of_an_integer.md
// Run the tests:          cargo test practice::p0073
// ==================================================================

//! # Problem 0073: Subtract the Product and Sum of Digits of an Integer
//!
//! - **Difficulty:** Easy
//! - **Topics:** extracting digits with `% 10` and `/ 10`, `while` loops, two accumulators
//! - **Source:** [LeetCode 1281. Subtract the Product and Sum of Digits of an Integer](https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/)
//!
//! ## Problem
//!
//! Given a positive integer `n`, return the product of its decimal digits minus the sum of its
//! decimal digits.
//!
//! Implement `Solution::subtract_product_and_sum(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 234
//! Output: 15
//! Explanation: product = 2 * 3 * 4 = 24, sum = 2 + 3 + 4 = 9, 24 - 9 = 15.
//!
//! Input:  n = 4421
//! Output: 21
//! Explanation: product = 32, sum = 11, 32 - 11 = 21.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^5`
//!
//! ## Hints
//!
//! 1. `n % 10` is the last digit and `n / 10` removes it.
//! 2. Start the product at `1` and the sum at `0`.

pub struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_234() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
    }

    #[test]
    fn example_4421() {
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }

    #[test]
    fn single_digits() {
        assert_eq!(Solution::subtract_product_and_sum(1), 0);
        assert_eq!(Solution::subtract_product_and_sum(9), 0);
    }

    #[test]
    fn zeros_make_the_product_zero() {
        assert_eq!(Solution::subtract_product_and_sum(100_000), -1);
        assert_eq!(Solution::subtract_product_and_sum(905), -14);
    }

    #[test]
    fn largest_product() {
        assert_eq!(Solution::subtract_product_and_sum(99_999), 59_049 - 45);
    }
}
