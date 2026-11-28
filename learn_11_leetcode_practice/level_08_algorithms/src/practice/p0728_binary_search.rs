// ==================================================================
// PRACTICE 0728: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0728_binary_search.md
// Run the tests:          cargo test practice::p0728
// ==================================================================

//! # Problem 0728: Binary Search
//!
//! - **Difficulty:** Easy
//! - **Topics:** binary search, half-open bounds `lo..hi`, `match` on `Ordering`, overflow-free midpoint
//! - **Source:** [LeetCode 704. Binary Search](https://leetcode.com/problems/binary-search/)
//!
//! ## Problem
//!
//! Given a vector `nums` sorted in ascending order with **distinct** values, and an integer
//! `target`, return the index of `target` in `nums`, or `-1` if it is not there. Your algorithm
//! must run in `O(log n)` time, so write the search yourself (do not scan the vector).
//!
//! Implement `Solution::search(nums: Vec<i32>, target: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [-1, 0, 3, 5, 9, 12], target = 9
//! Output: 4
//!
//! Input:  nums = [-1, 0, 3, 5, 9, 12], target = 2
//! Output: -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`
//! - `-10^4 < nums[i], target < 10^4`
//! - All values of `nums` are distinct and sorted in ascending order.
//!
//! ## Hints
//!
//! 1. Keep a range of indices `lo..hi` that still may contain the target, and look at its middle.
//! 2. With a half-open range, the loop runs while `lo < hi`; decide carefully whether the new
//!    bound is `mid` or `mid + 1`.

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<i32> {
        vec![-1, 0, 3, 5, 9, 12]
    }

    #[test]
    fn example_found() {
        assert_eq!(Solution::search(sample(), 9), 4);
    }

    #[test]
    fn example_missing() {
        assert_eq!(Solution::search(sample(), 2), -1);
    }

    #[test]
    fn first_and_last_positions() {
        assert_eq!(Solution::search(sample(), -1), 0);
        assert_eq!(Solution::search(sample(), 12), 5);
    }

    #[test]
    fn outside_the_range() {
        assert_eq!(Solution::search(sample(), -5), -1);
        assert_eq!(Solution::search(sample(), 100), -1);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::search(vec![5], 5), 0);
        assert_eq!(Solution::search(vec![5], -5), -1);
    }

    #[test]
    fn many_values_of_a_long_array() {
        let nums: Vec<i32> = (0..10_000).map(|i| i * 2 - 9_999).collect();
        for (i, &x) in nums.iter().enumerate().step_by(97) {
            assert_eq!(Solution::search(nums.clone(), x), i as i32);
            assert_eq!(Solution::search(nums.clone(), x + 1), -1);
        }
    }
}
