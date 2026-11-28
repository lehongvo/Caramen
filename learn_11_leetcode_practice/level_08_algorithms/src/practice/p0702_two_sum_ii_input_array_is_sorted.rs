// ==================================================================
// PRACTICE 0702: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0702_two_sum_ii_input_array_is_sorted.md
// Run the tests:          cargo test practice::p0702
// ==================================================================

//! # Problem 0702: Two Sum II - Input Array Is Sorted
//!
//! - **Difficulty:** Medium
//! - **Topics:** two pointers on a sorted array, `match` on `Ordering`, `cmp`
//! - **Source:** [LeetCode 167. Two Sum II - Input Array Is Sorted](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)
//!
//! ## Problem
//!
//! You are given a vector `numbers` sorted in **non-decreasing** order and an integer `target`.
//! Find two different positions whose values add up to `target` and return them as a vector
//! `[index1, index2]` of **1-based** indices with `index1 < index2`.
//!
//! The input always has exactly one solution. You may not use the same position twice (but two
//! different positions may hold equal values). Use only constant extra space.
//!
//! Implement `Solution::two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  numbers = [2, 7, 11, 15], target = 9
//! Output: [1, 2]        (2 + 7 = 9)
//!
//! Input:  numbers = [2, 3, 4], target = 6
//! Output: [1, 3]        (2 + 4 = 6)
//!
//! Input:  numbers = [-1, 0], target = -1
//! Output: [1, 2]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= numbers.len() <= 3 * 10^4`
//! - `-1000 <= numbers[i] <= 1000`, sorted in non-decreasing order
//! - `-1000 <= target <= 1000`
//! - Exactly one valid answer exists.
//!
//! ## Hints
//!
//! 1. A hash map works but needs extra memory. The array is sorted: use that instead.
//! 2. Start with one pointer at each end. If their sum is too small, which pointer should move?

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_two() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn example_first_and_last() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn example_negative_values() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }

    #[test]
    fn equal_values_at_different_positions() {
        assert_eq!(Solution::two_sum(vec![0, 0, 3, 4], 0), vec![1, 2]);
    }

    #[test]
    fn largest_pair_in_a_long_array() {
        let numbers: Vec<i32> = (-1000..=1000).collect();
        // Only 999 + 1000 reaches 1999; value v sits at 0-based index v + 1000.
        assert_eq!(Solution::two_sum(numbers, 1999), vec![2000, 2001]);
    }
}
