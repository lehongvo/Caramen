// ==================================================================
// PRACTICE 0782: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0782_number_of_enclaves.md
// Run the tests:          cargo test practice::p0782
// ==================================================================

//! # Problem 0782: Number of Enclaves
//!
//! - **Difficulty:** Medium
//! - **Topics:** flood fill from the border, iterator `chain` of border cells, counting with `flatten`
//! - **Source:** [LeetCode 1020. Number of Enclaves](https://leetcode.com/problems/number-of-enclaves/)
//!
//! ## Problem
//!
//! You are given an `m x n` binary matrix `grid`: `1` is land and `0` is sea. A move goes from a
//! land cell to a 4-directionally adjacent land cell, or walks off the edge of the grid. Return the
//! number of land cells from which you **cannot** walk off the grid in any number of moves.
//!
//! Implement `Solution::num_enclaves(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[0, 0, 0, 0],
//!                 [1, 0, 1, 0],
//!                 [0, 1, 1, 0],
//!                 [0, 0, 0, 0]]
//! Output: 3             (the 1 at (1, 0) touches the border; the other three are enclosed)
//!
//! Input:  grid = [[0, 1, 1, 0],
//!                 [0, 0, 1, 0],
//!                 [0, 0, 1, 0],
//!                 [0, 0, 0, 0]]
//! Output: 0             (every land cell is connected to the top border)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 500`
//! - `grid[i][j]` is `0` or `1`.
//!
//! ## Hints
//!
//! 1. Instead of asking "can this cell escape?" for every cell, start from the cells that can
//!    escape for sure: land on the border.
//! 2. Remove everything reachable from the border land. Whatever land remains is enclosed.

pub struct Solution;

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_enclosed() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_enclaves(grid), 3);
    }

    #[test]
    fn example_all_connected_to_border() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_enclaves(grid), 0);
    }

    #[test]
    fn single_cell_is_on_the_border() {
        assert_eq!(Solution::num_enclaves(vec![vec![1]]), 0);
    }

    #[test]
    fn single_enclosed_cell() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::num_enclaves(grid), 1);
    }

    #[test]
    fn diagonal_contact_with_the_border_does_not_count() {
        let grid = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::num_enclaves(grid), 1);
    }

    #[test]
    fn largest_grid() {
        // Sea on the border, land inside: 498 * 498 enclosed cells.
        let mut grid = vec![vec![1; 500]; 500];
        for i in 0..500 {
            grid[0][i] = 0;
            grid[499][i] = 0;
            grid[i][0] = 0;
            grid[i][499] = 0;
        }
        assert_eq!(Solution::num_enclaves(grid.clone()), 248_004);
        // Opening one border cell lets the whole inside escape.
        grid[0][1] = 1;
        assert_eq!(Solution::num_enclaves(grid), 0);
    }
}
