// ==================================================================
// PRACTICE 0313: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0313_minimum_absolute_difference.md
// Run the tests:          cargo test practice::p0313
// ==================================================================

//! # Problem 0313: Minimum Absolute Difference
//!
//! - **Difficulty:** Easy
//! - **Topics:** `sort_unstable`, `slice::windows`, building a `Vec<Vec<i32>>`
//! - **Source:** [LeetCode 1200. Minimum Absolute Difference](https://leetcode.com/problems/minimum-absolute-difference/)
//!
//! ## Problem
//!
//! You are given a vector `arr` of **distinct** integers. Find the smallest absolute difference
//! between any two elements, and return every pair `[a, b]` (with `a < b`) whose difference
//! `b - a` equals that minimum.
//!
//! The pairs must be listed in increasing order of `a` (so the whole output is sorted).
//!
//! Implement `Solution::minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr = [4, 2, 1, 3]
//! Output: [[1, 2], [2, 3], [3, 4]]      (minimum difference 1)
//!
//! Input:  arr = [1, 3, 6, 10, 15]
//! Output: [[1, 3]]                      (minimum difference 2)
//!
//! Input:  arr = [3, 8, -10, 23, 19, -4, -14, 27]
//! Output: [[-14, -10], [19, 23], [23, 27]]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= arr.len() <= 10^5`
//! - `-10^6 <= arr[i] <= 10^6`, all values distinct
//!
//! ## Hints
//!
//! 1. In sorted order, the closest pair of numbers is always a pair of neighbours.
//! 2. Make two passes over `windows(2)`: one to find the minimum gap, one to collect the pairs.

pub struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
            vec![vec![1, 2], vec![2, 3], vec![3, 4]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
            vec![vec![1, 3]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]]
        );
    }

    #[test]
    fn two_extreme_values() {
        assert_eq!(
            Solution::minimum_abs_difference(vec![1_000_000, -1_000_000]),
            vec![vec![-1_000_000, 1_000_000]]
        );
    }

    #[test]
    fn large_even_numbers() {
        let arr: Vec<i32> = (0..100_000).rev().map(|i| i * 2).collect();
        let result = Solution::minimum_abs_difference(arr);
        assert_eq!(result.len(), 99_999);
        assert_eq!(result[0], vec![0, 2]);
        assert_eq!(result[99_998], vec![199_996, 199_998]);
    }
}
