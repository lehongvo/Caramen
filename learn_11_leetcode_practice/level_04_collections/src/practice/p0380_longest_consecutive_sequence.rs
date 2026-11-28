// ==================================================================
// PRACTICE 0380: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0380_longest_consecutive_sequence.md
// Run the tests:          cargo test practice::p0380
// ==================================================================

//! # Problem 0380: Longest Consecutive Sequence
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashSet` membership in `O(1)`, starting runs only at their first element, linear-time reasoning
//! - **Source:** [LeetCode 128. Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/)
//!
//! ## Problem
//!
//! Given an unsorted vector of integers `nums`, return the length of the longest run of
//! **consecutive integers** (`x, x + 1, x + 2, ...`) whose values all appear in `nums`. The values
//! may appear anywhere in the vector, in any order. Duplicates count once.
//!
//! The algorithm must run in `O(n)` time, so sorting is not allowed.
//!
//! Implement `Solution::longest_consecutive(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [100, 4, 200, 1, 3, 2]
//! Output: 4       (1, 2, 3, 4)
//!
//! Input:  nums = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1]
//! Output: 9       (0 through 8)
//!
//! Input:  nums = [1, 0, 1, 2]
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `0 <= nums.len() <= 10^5`
//! - `-10^9 <= nums[i] <= 10^9`
//!
//! ## Hints
//!
//! 1. Put all values in a `HashSet` so "is `x + 1` present?" is `O(1)`.
//! 2. Only start counting at a value `x` whose predecessor `x - 1` is **not** in the set: `x` is
//!    the first element of its run.
//! 3. Each value is then visited at most twice in total, which keeps the whole thing linear.

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn example_three_duplicates() {
        assert_eq!(Solution::longest_consecutive(vec![1, 0, 1, 2]), 3);
    }

    #[test]
    fn empty_and_extremes() {
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
        assert_eq!(
            Solution::longest_consecutive(vec![1_000_000_000, -1_000_000_000]),
            1
        );
    }

    #[test]
    fn large_shuffled_runs() {
        // Two runs, 0..60_000 and 100_000..140_000, each reversed, then the whole vector rotated.
        let mut nums: Vec<i32> = (0..60_000).rev().collect();
        nums.extend((100_000..140_000).rev());
        nums.rotate_left(33_333);
        assert_eq!(Solution::longest_consecutive(nums), 60_000);
    }
}
