// ==================================================================
// PRACTICE 0821: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0821_minimum_path_sum.md
// Run the tests:          cargo test practice::p0821
// ==================================================================

//! # Problem 0821: Minimum Path Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** grid DP done in place, taking ownership of the input with `let mut grid = grid`
//! - **Source:** [LeetCode 64. Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/)
//!
//! ## Problem
//!
//! `grid` is an `m x n` grid of non-negative numbers. Walk from the top-left cell to the
//! bottom-right cell, moving only **right** or **down**. The cost of a path is the sum of all the
//! cells it visits (including both ends). Return the minimum cost.
//!
//! Implement `Solution::min_path_sum(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[1, 3, 1],
//!                 [1, 5, 1],
//!                 [4, 2, 1]]
//! Output: 7
//! Explanation: 1 -> 3 -> 1 -> 1 -> 1.
//!
//! Input:  grid = [[1, 2, 3],
//!                 [4, 5, 6]]
//! Output: 12
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`
//! - `0 <= grid[i][j] <= 200`
//!
//! ## Hints
//!
//! 1. The cheapest way to reach a cell comes from the cheaper of its top and left neighbours.
//! 2. The function owns `grid`, so you may overwrite each cell with its best cost: no extra memory.

pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_by_three() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }

    #[test]
    fn example_two_by_three() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(Solution::min_path_sum(grid), 12);
    }

    #[test]
    fn single_cell_row_and_column() {
        assert_eq!(Solution::min_path_sum(vec![vec![5]]), 5);
        assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3]]), 6);
        assert_eq!(Solution::min_path_sum(vec![vec![1], vec![2], vec![3]]), 6);
    }

    #[test]
    fn detour_is_cheaper() {
        let grid = vec![vec![1, 9, 1, 1], vec![1, 9, 1, 9], vec![1, 1, 1, 1]];
        assert_eq!(Solution::min_path_sum(grid), 6);
    }

    #[test]
    fn largest_uniform_grid() {
        // Any path visits 200 + 200 - 1 = 399 cells.
        assert_eq!(Solution::min_path_sum(vec![vec![1; 200]; 200]), 399);
    }
}
