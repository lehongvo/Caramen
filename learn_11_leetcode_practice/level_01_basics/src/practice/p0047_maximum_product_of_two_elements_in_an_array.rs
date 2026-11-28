// ==================================================================
// PRACTICE 0047: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0047_maximum_product_of_two_elements_in_an_array.md
// Run the tests:          cargo test practice::p0047
// ==================================================================

//! # Problem 0047: Maximum Product of Two Elements in an Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** tracking the two largest values in one pass, `if` / `else if`
//! - **Source:** [LeetCode 1464. Maximum Product of Two Elements in an Array](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/)
//!
//! ## Problem
//!
//! Given a vector of positive integers `nums`, choose two **different** indices `i` and `j` and
//! compute `(nums[i] - 1) * (nums[j] - 1)`. Return the largest value this expression can take.
//!
//! Implement `Solution::max_product(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 4, 5, 2]
//! Output: 12
//! Explanation: pick 4 and 5: (4 - 1) * (5 - 1) = 12.
//!
//! Input:  nums = [1, 5, 4, 5]
//! Output: 16
//! Explanation: the two 5s are at different indices: (5 - 1) * (5 - 1) = 16.
//!
//! Input:  nums = [3, 7]
//! Output: 12
//! ```
//!
//! ## Constraints
//!
//! - `2 <= nums.len() <= 500`
//! - `1 <= nums[i] <= 10^3`
//!
//! ## Hints
//!
//! 1. All values are at least 1, so every factor `nums[i] - 1` is non-negative. Bigger values give bigger products.
//! 2. You only need the largest and the second largest value. Sorting works, but one pass is enough.

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
    fn example_distinct() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
    }

    #[test]
    fn example_duplicate_maximum() {
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
    }

    #[test]
    fn example_two_elements() {
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
        assert_eq!(Solution::max_product(vec![1, 1]), 0);
    }

    #[test]
    fn maximum_first_or_last() {
        assert_eq!(Solution::max_product(vec![10, 2, 3, 9]), 72);
        assert_eq!(Solution::max_product(vec![2, 3, 9, 10]), 72);
    }

    #[test]
    fn largest_values() {
        let mut nums = vec![1; 500];
        nums[100] = 1000;
        nums[400] = 999;
        assert_eq!(Solution::max_product(nums), 999 * 998);
    }
}
