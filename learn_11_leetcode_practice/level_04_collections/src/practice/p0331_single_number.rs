// ==================================================================
// PRACTICE 0331: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0331_single_number.md
// Run the tests:          cargo test practice::p0331
// ==================================================================

//! # Problem 0331: Single Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** toggling membership in a `HashSet`, `remove`, `into_iter().next()`, XOR
//! - **Source:** [LeetCode 136. Single Number](https://leetcode.com/problems/single-number/)
//!
//! ## Problem
//!
//! In the non-empty vector `nums`, every value appears **exactly twice**, except one value that
//! appears exactly once. Return that single value.
//!
//! LeetCode's follow-up asks for linear time and constant extra memory. Solve it first with a
//! `HashSet`, then think about the follow-up (the Solution section shows both).
//!
//! Implement `Solution::single_number(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 2, 1]
//! Output: 1
//!
//! Input:  nums = [4, 1, 2, 1, 2]
//! Output: 4
//!
//! Input:  nums = [1]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 3 * 10^4`
//! - `-3 * 10^4 <= nums[i] <= 3 * 10^4`
//! - Every element appears twice except for one element, which appears once.
//!
//! ## Hints
//!
//! 1. Keep a set of the values seen an **odd** number of times so far: insert a value the first
//!    time, remove it the second time.
//! 2. At the end, exactly one value is left in the set.

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn example_three_single_element() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }

    #[test]
    fn negative_and_zero() {
        assert_eq!(Solution::single_number(vec![0, -30_000, 0]), -30_000);
        assert_eq!(Solution::single_number(vec![-1, 0, -1]), 0);
    }

    #[test]
    fn large_input() {
        let mut nums: Vec<i32> = (0..7_499).flat_map(|i| [i, -i - 1]).collect();
        nums.extend((0..7_499).rev());
        nums.extend((0..7_499).map(|i| -i - 1));
        nums.push(29_999);
        assert_eq!(nums.len(), 4 * 7_499 + 1);
        assert_eq!(Solution::single_number(nums), 29_999);
    }
}
