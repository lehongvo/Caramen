// ==================================================================
// PRACTICE 0786: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0786_rotting_oranges.md
// Run the tests:          cargo test practice::p0786
// ==================================================================

//! # Problem 0786: Rotting Oranges
//!
//! - **Difficulty:** Medium
//! - **Topics:** multi-source BFS level by level, `VecDeque`, counting what is left
//! - **Source:** [LeetCode 994. Rotting Oranges](https://leetcode.com/problems/rotting-oranges/)
//!
//! ## Problem
//!
//! In an `m x n` grid, each cell is `0` (empty), `1` (a fresh orange) or `2` (a rotten orange).
//! Every minute, each fresh orange that is 4-directionally adjacent to a rotten orange becomes
//! rotten.
//!
//! Return the minimum number of minutes until no fresh orange is left, or `-1` if some fresh
//! orange can never rot. If there are no fresh oranges at the start, the answer is `0`.
//!
//! Implement `Solution::oranges_rotting(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[2, 1, 1],
//!                 [1, 1, 0],
//!                 [0, 1, 1]]
//! Output: 4
//!
//! Input:  grid = [[2, 1, 1],
//!                 [0, 1, 1],
//!                 [1, 0, 1]]
//! Output: -1            (the orange in the bottom-left corner is never reached)
//!
//! Input:  grid = [[0, 2]]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 10`
//! - `grid[i][j]` is `0`, `1` or `2`.
//!
//! ## Hints
//!
//! 1. All rotten oranges spread at the same time: start a breadth-first search from **all** of
//!    them at once.
//! 2. Process the queue one level (one minute) at a time: the oranges added during a minute are the
//!    ones that rot in that minute.
//! 3. Count the fresh oranges up front, and decrease the count as they rot.

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_minutes() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 4);
    }

    #[test]
    fn example_unreachable_orange() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(Solution::oranges_rotting(grid), -1);
    }

    #[test]
    fn example_nothing_fresh() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
        assert_eq!(Solution::oranges_rotting(vec![vec![0]]), 0);
    }

    #[test]
    fn fresh_orange_without_rotten_ones() {
        assert_eq!(Solution::oranges_rotting(vec![vec![1]]), -1);
    }

    #[test]
    fn several_sources_spread_together() {
        // Rotten oranges at both ends meet in the middle after 2 minutes.
        assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 2]]), 2);
    }

    #[test]
    fn largest_grid_from_one_corner() {
        let mut grid = vec![vec![1; 10]; 10];
        grid[0][0] = 2;
        assert_eq!(Solution::oranges_rotting(grid), 18);
    }
}
