//! # Problem 0732: Search a 2D Matrix
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary search over a virtual flattened array, index arithmetic `k / n`, `k % n`
//! - **Source:** [LeetCode 74. Search a 2D Matrix](https://leetcode.com/problems/search-a-2d-matrix/)
//!
//! ## Problem
//!
//! You are given an `m x n` integer matrix with two properties:
//!
//! - every row is sorted in non-decreasing order;
//! - the first value of every row is greater than the last value of the previous row.
//!
//! Return `true` if `target` is in the matrix and `false` otherwise, in `O(log(m * n))` time.
//!
//! Implement `Solution::search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  matrix = [[ 1,  3,  5,  7],
//!                   [10, 11, 16, 20],
//!                   [23, 30, 34, 60]], target = 3
//! Output: true
//!
//! Input:  same matrix, target = 13
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 100`
//! - `-10^4 <= matrix[i][j], target <= 10^4`
//!
//! ## Hints
//!
//! 1. Read the rows one after another: the result is a single sorted sequence of `m * n` values.
//! 2. You do not need to build that sequence. Which row and column hold its `k`-th element?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** binary search on a flattened view: concatenating the rows gives one sorted sequence
//! of length `m * n`, and its element number `k` is `matrix[k / n][k % n]`. Run an ordinary
//! binary search over `0..m * n` and translate each midpoint into a row and a column. No copy of
//! the matrix is made.
//!
//! **Complexity:** `O(log(m * n))` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - Integer division and remainder map a linear index to 2-D coordinates; the same trick is used
//!   for row-major storage of grids in a single `Vec`.
//! - `match` on `cmp` keeps the three outcomes explicit.
// @solution-end

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // @solution-start
        let n = matrix[0].len();
        let (mut lo, mut hi) = (0, matrix.len() * n);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match matrix[mid / n][mid % n].cmp(&target) {
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
                Ordering::Equal => return true,
            }
        }
        false
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Vec<i32>> {
        vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]]
    }

    #[test]
    fn example_found() {
        assert!(Solution::search_matrix(sample(), 3));
    }

    #[test]
    fn example_missing() {
        assert!(!Solution::search_matrix(sample(), 13));
    }

    #[test]
    fn corners_and_outside_values() {
        assert!(Solution::search_matrix(sample(), 1));
        assert!(Solution::search_matrix(sample(), 60));
        assert!(!Solution::search_matrix(sample(), 0));
        assert!(!Solution::search_matrix(sample(), 61));
    }

    #[test]
    fn single_cell() {
        assert!(Solution::search_matrix(vec![vec![1]], 1));
        assert!(!Solution::search_matrix(vec![vec![1]], 2));
    }

    #[test]
    fn single_column() {
        let matrix = vec![vec![1], vec![3], vec![5]];
        assert!(Solution::search_matrix(matrix.clone(), 5));
        assert!(!Solution::search_matrix(matrix, 4));
    }

    #[test]
    fn largest_matrix() {
        // Even numbers from -10_000 upwards, 100 per row.
        let matrix: Vec<Vec<i32>> = (0..100)
            .map(|r| (0..100).map(|c| (r * 100 + c) * 2 - 10_000).collect())
            .collect();
        assert!(Solution::search_matrix(matrix.clone(), -10_000));
        assert!(Solution::search_matrix(matrix.clone(), 9_998));
        assert!(Solution::search_matrix(matrix.clone(), 1_234));
        assert!(!Solution::search_matrix(matrix, 1_235));
    }
}
