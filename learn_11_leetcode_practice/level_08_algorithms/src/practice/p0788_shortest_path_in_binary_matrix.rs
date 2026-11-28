// ==================================================================
// PRACTICE 0788: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0788_shortest_path_in_binary_matrix.md
// Run the tests:          cargo test practice::p0788
// ==================================================================

//! # Problem 0788: Shortest Path in Binary Matrix
//!
//! - **Difficulty:** Medium
//! - **Topics:** BFS for shortest paths, 8 directions, `VecDeque` of `(row, col, length)`
//! - **Source:** [LeetCode 1091. Shortest Path in Binary Matrix](https://leetcode.com/problems/shortest-path-in-binary-matrix/)
//!
//! ## Problem
//!
//! Given an `n x n` binary matrix `grid`, return the length of the shortest **clear path** from the
//! top-left cell `(0, 0)` to the bottom-right cell `(n - 1, n - 1)`, or `-1` if there is none.
//!
//! A clear path visits only cells with value `0`, and each step moves to one of the **8**
//! neighbouring cells (edges or corners touching). Its length is the number of cells it visits,
//! including both ends.
//!
//! Implement `Solution::shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[0, 1],
//!                 [1, 0]]
//! Output: 2             (one diagonal step)
//!
//! Input:  grid = [[0, 0, 0],
//!                 [1, 1, 0],
//!                 [1, 1, 0]]
//! Output: 4             ((0,0) -> (0,1) -> (1,2) -> (2,2))
//!
//! Input:  grid = [[1, 0, 0],
//!                 [1, 1, 0],
//!                 [1, 1, 0]]
//! Output: -1            (the start is blocked)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 100`
//! - `grid[i][j]` is `0` or `1`.
//!
//! ## Hints
//!
//! 1. Every step has the same cost, so breadth-first search finds shortest paths.
//! 2. Store the path length with each queued cell, and mark cells when you **enqueue** them.

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_diagonal() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
            2
        );
    }

    #[test]
    fn example_around_the_wall() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 4);
    }

    #[test]
    fn example_blocked_start() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(grid), -1);
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![0]]), 1);
        assert_eq!(Solution::shortest_path_binary_matrix(vec![vec![1]]), -1);
    }

    #[test]
    fn blocked_target_or_no_path() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0, 0], vec![0, 1]]),
            -1
        );
        let wall = vec![vec![0, 1, 0], vec![1, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::shortest_path_binary_matrix(wall), -1);
    }

    #[test]
    fn largest_open_grid_uses_the_diagonal() {
        assert_eq!(
            Solution::shortest_path_binary_matrix(vec![vec![0; 100]; 100]),
            100
        );
    }

    #[test]
    fn winding_path() {
        // Walls force a path down the left column, along the bottom, then up the right column.
        let grid = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
        ];
        // (0,0) down to (3,0), (4,1), (3,2) up to (1,2), (0,3), (1,4) down to (4,4): 13 cells.
        assert_eq!(Solution::shortest_path_binary_matrix(grid), 13);
    }
}
