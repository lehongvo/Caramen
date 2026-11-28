// ==================================================================
// PRACTICE 0371: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0371_largest_number.md
// Run the tests:          cargo test practice::p0371
// ==================================================================

//! # Problem 0371: Largest Number
//!
//! - **Difficulty:** Medium
//! - **Topics:** a comparator that compares concatenations, `Iterator::cmp` on chained byte iterators, `concat`
//! - **Source:** [LeetCode 179. Largest Number](https://leetcode.com/problems/largest-number/)
//!
//! ## Problem
//!
//! Given a vector of non-negative integers `nums`, arrange them (as decimal strings, glued
//! together) to form the **largest** possible number, and return that number as a `String`. The
//! result must not have leading zeros, except for the number `"0"` itself.
//!
//! Implement `Solution::largest_number(nums: Vec<i32>) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [10, 2]
//! Output: "210"
//!
//! Input:  nums = [3, 30, 34, 5, 9]
//! Output: "9534330"
//!
//! Input:  nums = [0, 0]
//! Output: "0"          (not "00")
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 100`
//! - `0 <= nums[i] <= 10^9`
//!
//! ## Hints
//!
//! 1. Numeric order fails (`[10, 2]` must give `"210"`), and no simple per-number key works
//!    either. Compare **pairs** of numbers instead.
//! 2. `a` should come before `b` exactly when the string `a + b` is larger than `b + a`.
//! 3. If the largest piece is `"0"`, every piece is `"0"`.

pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210");
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330");
    }

    #[test]
    fn example_three_all_zeros() {
        assert_eq!(Solution::largest_number(vec![0, 0]), "0");
        assert_eq!(Solution::largest_number(vec![0]), "0");
    }

    #[test]
    fn tricky_prefixes() {
        assert_eq!(Solution::largest_number(vec![121, 12]), "12121");
        assert_eq!(Solution::largest_number(vec![824, 8247]), "8248247");
        assert_eq!(Solution::largest_number(vec![0, 1]), "10");
    }

    #[test]
    fn large_values() {
        assert_eq!(
            Solution::largest_number(vec![1_000_000_000, 999_999_999, 9]),
            "99999999991000000000"
        );
        let nums = vec![1; 100];
        assert_eq!(Solution::largest_number(nums), "1".repeat(100));
    }
}
