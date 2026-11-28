// ==================================================================
// PRACTICE 0864: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0864_swim_in_rising_water.md
// Run the tests:          cargo test practice::p0864
// ==================================================================

//! # Problem 0864: Swim in Rising Water
//!
//! - **Difficulty:** Hard
//! - **Topics:** minimax path with Dijkstra on node weights, `BinaryHeap<Reverse<_>>`, visited grid
//! - **Source:** [LeetCode 778. Swim in Rising Water](https://leetcode.com/problems/swim-in-rising-water/)
//!
//! ## Problem
//!
//! `grid` is an `n x n` grid where `grid[r][c]` is the elevation of cell `(r, c)`; the elevations
//! are a permutation of `0..n * n`. Rain starts at time `0`; at time `t` the water depth everywhere
//! is `t`. You can swim from a cell to a 4-directionally adjacent cell only if **both** elevations
//! are at most `t`; swimming itself takes no time. Return the earliest time at which you can get
//! from the top-left cell to the bottom-right cell.
//!
//! Implement `Solution::swim_in_water(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[0, 2],
//!                 [1, 3]]
//! Output: 3
//! Explanation: the target cell itself has elevation 3, so you must wait until t = 3.
//!
//! Input:  grid = [[ 0,  1,  2,  3,  4],
//!                 [24, 23, 22, 21,  5],
//!                 [12, 13, 14, 15, 16],
//!                 [11, 17, 18, 19, 20],
//!                 [10,  9,  8,  7,  6]]
//! Output: 16
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 50`
//! - The values of `grid` are a permutation of `0..n * n`.
//!
//! ## Hints
//!
//! 1. The time needed for a route is the **largest** elevation on it (including both ends).
//!    You want the route that minimizes that maximum.
//! 2. Expand cells in increasing order of "time needed to reach them" with a min-heap. The first
//!    time the target is popped, its time is optimal.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_by_two() {
        assert_eq!(Solution::swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    }

    #[test]
    fn example_spiral() {
        let g = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        assert_eq!(Solution::swim_in_water(g), 16);
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::swim_in_water(vec![vec![0]]), 0);
    }

    #[test]
    fn start_cell_is_high() {
        assert_eq!(Solution::swim_in_water(vec![vec![3, 2], vec![0, 1]]), 3);
    }

    #[test]
    fn row_major_grid() {
        // Elevations increase along each row: the best route is bounded by the last cell n^2 - 1.
        let n = 50;
        let g: Vec<Vec<i32>> = (0..n)
            .map(|r| (0..n).map(|c| r * n + c).collect())
            .collect();
        assert_eq!(Solution::swim_in_water(g), n * n - 1);
    }
}
