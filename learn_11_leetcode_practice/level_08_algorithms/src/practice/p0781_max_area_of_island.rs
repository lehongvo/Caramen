// ==================================================================
// PRACTICE 0781: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0781_max_area_of_island.md
// Run the tests:          cargo test practice::p0781
// ==================================================================

//! # Problem 0781: Max Area of Island
//!
//! - **Difficulty:** Medium
//! - **Topics:** recursive DFS returning a count, `&mut [Vec<i32>]`, `checked_sub` + `map_or`
//! - **Source:** [LeetCode 695. Max Area of Island](https://leetcode.com/problems/max-area-of-island/)
//!
//! ## Problem
//!
//! You are given an `m x n` binary matrix `grid` where `1` is land and `0` is water. An island is
//! a group of `1`s connected **4-directionally** (horizontally or vertically). The **area** of an
//! island is its number of cells. Return the largest area of an island, or `0` if there is no land.
//!
//! Implement `Solution::max_area_of_island(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
//!                 [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
//!                 [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
//!                 [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
//!                 [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
//!                 [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
//!                 [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
//!                 [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]]
//! Output: 6             (the island around rows 3-5, columns 8-10)
//!
//! Input:  grid = [[0, 0, 0, 0, 0, 0, 0, 0]]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 50`
//! - `grid[i][j]` is `0` or `1`.
//!
//! ## Hints
//!
//! 1. This is Number of Islands, but each search must also report how many cells it visited.
//! 2. A recursive DFS can return "1 for this cell + the areas found from its four neighbours".

pub struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_six() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);
    }

    #[test]
    fn example_no_land() {
        assert_eq!(Solution::max_area_of_island(vec![vec![0; 8]]), 0);
    }

    #[test]
    fn single_cell_island() {
        assert_eq!(Solution::max_area_of_island(vec![vec![1]]), 1);
    }

    #[test]
    fn diagonal_cells_do_not_connect() {
        let grid = vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]];
        assert_eq!(Solution::max_area_of_island(grid), 1);
    }

    #[test]
    fn ring_shaped_island() {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert_eq!(Solution::max_area_of_island(grid), 8);
    }

    #[test]
    fn largest_grid_all_land() {
        assert_eq!(Solution::max_area_of_island(vec![vec![1; 50]; 50]), 2500);
    }
}
