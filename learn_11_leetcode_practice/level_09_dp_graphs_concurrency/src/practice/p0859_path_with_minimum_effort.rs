// ==================================================================
// PRACTICE 0859: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0859_path_with_minimum_effort.md
// Run the tests:          cargo test practice::p0859
// ==================================================================

//! # Problem 0859: Path With Minimum Effort
//!
//! - **Difficulty:** Medium
//! - **Topics:** "minimax" Dijkstra on a grid (path cost = largest step), `Reverse`, direction
//!   arrays with `checked_add_signed`
//! - **Source:** [LeetCode 1631. Path With Minimum Effort](https://leetcode.com/problems/path-with-minimum-effort/)
//!
//! ## Problem
//!
//! A hiker stands on the top-left cell of a `rows x columns` grid `heights` and wants to reach the
//! bottom-right cell, moving up, down, left or right. The **effort** of a route is the largest
//! absolute height difference between two consecutive cells on it. Return the minimum effort over
//! all routes.
//!
//! Implement `Solution::minimum_effort_path(heights: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  heights = [[1, 2, 2],
//!                    [3, 8, 2],
//!                    [5, 3, 5]]
//! Output: 2
//! Explanation: the route 1 -> 3 -> 5 -> 3 -> 5 has a largest step of 2.
//!
//! Input:  heights = [[1, 2, 3],
//!                    [3, 8, 4],
//!                    [5, 3, 5]]
//! Output: 1
//! Explanation: 1 -> 2 -> 3 -> 4 -> 5.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= rows, columns <= 100`
//! - `1 <= heights[i][j] <= 10^6`
//!
//! ## Hints
//!
//! 1. Replace "sum of edge weights" in Dijkstra by "maximum of edge weights". Extending a route can
//!    never lower its maximum, which is the property Dijkstra needs.
//! 2. The first time the target cell is popped from the min-heap, its effort is optimal.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_effort_two() {
        let h = vec![vec![1, 2, 2], vec![3, 8, 2], vec![5, 3, 5]];
        assert_eq!(Solution::minimum_effort_path(h), 2);
    }

    #[test]
    fn example_effort_one() {
        let h = vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]];
        assert_eq!(Solution::minimum_effort_path(h), 1);
    }

    #[test]
    fn flat_path_exists() {
        let h = vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1],
        ];
        assert_eq!(Solution::minimum_effort_path(h), 0);
    }

    #[test]
    fn single_cell_and_single_row() {
        assert_eq!(Solution::minimum_effort_path(vec![vec![7]]), 0);
        let row = vec![vec![1, 10, 6, 7, 9, 10, 4, 9]];
        assert_eq!(Solution::minimum_effort_path(row), 9);
    }

    #[test]
    fn large_grid_with_a_cliff() {
        // Height 1 everywhere except a wall of 1_000_000 on column 50 with one gap at row 99.
        let mut h = vec![vec![1; 100]; 100];
        for row in h.iter_mut().take(99) {
            row[50] = 1_000_000;
        }
        assert_eq!(Solution::minimum_effort_path(h.clone()), 0);
        h[99][50] = 1_000_000;
        assert_eq!(Solution::minimum_effort_path(h), 999_999);
    }
}
