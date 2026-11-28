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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** BFS on an unweighted grid graph: cells are vertices and the 8 neighbours are edges.
//! Start from `(0, 0)` with length `1` (if it is clear). BFS dequeues cells in order of path
//! length, so the first time the target is dequeued, its length is the shortest. Marking a cell
//! (writing `1` into the grid) when it is enqueued guarantees that each cell enters the queue once.
//! DFS would not work here: it finds **a** path, not the shortest one.
//!
//! **Complexity:** `O(n^2)` time and space (each cell enqueued at most once, 8 checks each).
//!
//! **Rust concepts:**
//! - The 8 offsets come from two nested ranges `-1..=1` (skipping `(0, 0)`), which avoids writing the
//!   table by hand.
//! - Tuples in the queue, `(r, c, length)`, keep related values together without a struct.
// @solution-end

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let mut grid = grid;
        let n = grid.len();
        if grid[0][0] != 0 || grid[n - 1][n - 1] != 0 {
            return -1;
        }
        grid[0][0] = 1; // visited
        let mut queue = VecDeque::from([(0usize, 0usize, 1)]);
        while let Some((r, c, length)) = queue.pop_front() {
            if (r, c) == (n - 1, n - 1) {
                return length;
            }
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if (dr, dc) == (0, 0) {
                        continue;
                    }
                    let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
                    else {
                        continue;
                    };
                    if nr < n && nc < n && grid[nr][nc] == 0 {
                        grid[nr][nc] = 1;
                        queue.push_back((nr, nc, length + 1));
                    }
                }
            }
        }
        -1
        // @solution-end
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
