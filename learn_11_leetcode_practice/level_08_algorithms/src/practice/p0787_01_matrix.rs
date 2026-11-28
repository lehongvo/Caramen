// ==================================================================
// PRACTICE 0787: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0787_01_matrix.md
// Run the tests:          cargo test practice::p0787
// ==================================================================

//! # Problem 0787: 01 Matrix
//!
//! - **Difficulty:** Medium
//! - **Topics:** multi-source BFS for distances, `VecDeque`, sentinel for "not visited yet"
//! - **Source:** [LeetCode 542. 01 Matrix](https://leetcode.com/problems/01-matrix/)
//!
//! ## Problem
//!
//! Given an `m x n` binary matrix `mat`, return a matrix of the same size where each cell holds
//! the distance from that cell to the **nearest `0`** of `mat`. The distance between two
//! 4-directionally adjacent cells is `1`. The matrix contains at least one `0`.
//!
//! Implement `Solution::update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mat = [[0, 0, 0],          Output: [[0, 0, 0],
//!                [0, 1, 0],                   [0, 1, 0],
//!                [0, 0, 0]]                   [0, 0, 0]]
//!
//! Input:  mat = [[0, 0, 0],          Output: [[0, 0, 0],
//!                [0, 1, 0],                   [0, 1, 0],
//!                [1, 1, 1]]                   [1, 2, 1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 10^4` and `1 <= m * n <= 10^4`
//! - `mat[i][j]` is `0` or `1`, and there is at least one `0`.
//!
//! ## Hints
//!
//! 1. A BFS from one `1` to the nearest `0` for every cell is too slow. Reverse the direction.
//! 2. Start a single BFS from **all** zeros at once (distance `0`). The first time the BFS reaches
//!    a cell, it arrives with the shortest distance.

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_single_one() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::update_matrix(mat.clone()), mat);
    }

    #[test]
    fn example_bottom_row() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        assert_eq!(
            Solution::update_matrix(mat),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }

    #[test]
    fn single_zero_cell() {
        assert_eq!(Solution::update_matrix(vec![vec![0]]), vec![vec![0]]);
    }

    #[test]
    fn single_row() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 1, 1, 1]]),
            vec![vec![0, 1, 2, 3]]
        );
        assert_eq!(
            Solution::update_matrix(vec![vec![1, 1, 0, 1]]),
            vec![vec![2, 1, 0, 1]]
        );
    }

    #[test]
    fn nearest_of_several_zeros() {
        let mat = vec![vec![0, 1, 1, 1, 0]];
        assert_eq!(Solution::update_matrix(mat), vec![vec![0, 1, 2, 1, 0]]);
    }

    #[test]
    fn largest_matrix_with_one_zero_in_a_corner() {
        let mut mat = vec![vec![1; 100]; 100];
        mat[0][0] = 0;
        let dist = Solution::update_matrix(mat);
        for (r, row) in dist.iter().enumerate() {
            for (c, &d) in row.iter().enumerate() {
                assert_eq!(d, (r + c) as i32);
            }
        }
    }
}
