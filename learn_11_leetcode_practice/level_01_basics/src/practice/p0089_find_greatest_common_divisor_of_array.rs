// ==================================================================
// PRACTICE 0089: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0089_find_greatest_common_divisor_of_array.md
// Run the tests:          cargo test practice::p0089
// ==================================================================

//! # Problem 0089: Find Greatest Common Divisor of Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** min and max in one pass, Euclid's algorithm in a helper function
//! - **Source:** [LeetCode 1979. Find Greatest Common Divisor of Array](https://leetcode.com/problems/find-greatest-common-divisor-of-array/)
//!
//! ## Problem
//!
//! Given a vector of positive integers `nums`, return the greatest common divisor of its **smallest**
//! and its **largest** value. (The other elements do not matter.)
//!
//! Implement `Solution::find_gcd(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 5, 6, 9, 10]
//! Output: 2
//! Explanation: min = 2, max = 10, gcd(2, 10) = 2.
//!
//! Input:  nums = [7, 5, 6, 8, 3]
//! Output: 1
//! Explanation: min = 3, max = 8, gcd(3, 8) = 1.
//!
//! Input:  nums = [3, 3]
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `2 <= nums.len() <= 1000`
//! - `1 <= nums[i] <= 1000`
//!
//! ## Hints
//!
//! 1. Find the minimum and the maximum in a single loop.
//! 2. Reuse Euclid's algorithm from problem 0088 in a private helper.

pub struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_common_factor() {
        assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
    }

    #[test]
    fn example_coprime() {
        assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
    }

    #[test]
    fn example_equal_values() {
        assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
    }

    #[test]
    fn min_divides_max() {
        assert_eq!(Solution::find_gcd(vec![1000, 250, 500]), 250);
        assert_eq!(Solution::find_gcd(vec![1, 1000]), 1);
    }

    #[test]
    fn largest_input() {
        let mut nums = vec![500; 1000];
        nums[3] = 840;
        nums[997] = 360;
        assert_eq!(Solution::find_gcd(nums), 120);
    }
}
