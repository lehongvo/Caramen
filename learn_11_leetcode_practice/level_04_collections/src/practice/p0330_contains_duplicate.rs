// ==================================================================
// PRACTICE 0330: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0330_contains_duplicate.md
// Run the tests:          cargo test practice::p0330
// ==================================================================

//! # Problem 0330: Contains Duplicate
//!
//! - **Difficulty:** Easy
//! - **Topics:** `HashSet::with_capacity`, `insert` returns `bool`, `Iterator::any`
//! - **Source:** [LeetCode 217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)
//!
//! ## Problem
//!
//! Given a vector of integers `nums`, return `true` if some value appears **at least twice**, and
//! `false` if all values are distinct.
//!
//! Implement `Solution::contains_duplicate(nums: Vec<i32>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3, 1]
//! Output: true
//!
//! Input:  nums = [1, 2, 3, 4]
//! Output: false
//!
//! Input:  nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2]
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-10^9 <= nums[i] <= 10^9`
//!
//! ## Hints
//!
//! 1. Remember the values seen so far in a `HashSet`.
//! 2. `insert` returns `false` if the value was already present: that is a duplicate, so you can
//!    stop immediately.

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_three() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }

    #[test]
    fn single_element_and_extremes() {
        assert!(!Solution::contains_duplicate(vec![0]));
        assert!(!Solution::contains_duplicate(vec![
            -1_000_000_000,
            1_000_000_000
        ]));
        assert!(Solution::contains_duplicate(vec![
            -1_000_000_000,
            -1_000_000_000
        ]));
    }

    #[test]
    fn large_inputs() {
        let distinct: Vec<i32> = (0..100_000).map(|i| i * 10_000 - 500_000_000).collect();
        assert!(!Solution::contains_duplicate(distinct.clone()));
        let mut with_dup = distinct;
        with_dup[99_999] = with_dup[0];
        assert!(Solution::contains_duplicate(with_dup));
    }
}
