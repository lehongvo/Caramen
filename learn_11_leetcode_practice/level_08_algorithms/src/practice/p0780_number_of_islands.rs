// ==================================================================
// PRACTICE 0780: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0780_number_of_islands.md
// Run the tests:          cargo test practice::p0780
// ==================================================================

//! # Problem 0780: Number of Islands
//!
//! - **Difficulty:** Medium
//! - **Topics:** connected components with DFS, sinking visited land, `Vec<Vec<char>>`
//! - **Source:** [LeetCode 200. Number of Islands](https://leetcode.com/problems/number-of-islands/)
//!
//! ## Problem
//!
//! You are given an `m x n` map `grid` where `'1'` is land and `'0'` is water. An **island** is a
//! group of land cells connected **horizontally or vertically** (not diagonally), surrounded by
//! water. Everything outside the grid counts as water. Return the number of islands.
//!
//! Implement `Solution::num_islands(grid: Vec<Vec<char>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [['1', '1', '1', '1', '0'],
//!                 ['1', '1', '0', '1', '0'],
//!                 ['1', '1', '0', '0', '0'],
//!                 ['0', '0', '0', '0', '0']]
//! Output: 1
//!
//! Input:  grid = [['1', '1', '0', '0', '0'],
//!                 ['1', '1', '0', '0', '0'],
//!                 ['0', '0', '1', '0', '0'],
//!                 ['0', '0', '0', '1', '1']]
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 300`
//! - `grid[i][j]` is `'0'` or `'1'`.
//!
//! ## Hints
//!
//! 1. Scan the grid. The first time you meet a land cell, you have found a new island.
//! 2. Before continuing the scan, visit that whole island (DFS or BFS) and mark it, so its other
//!    cells are not counted again.

pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|row| row.chars().collect()).collect()
    }

    #[test]
    fn example_one_island() {
        let grid = parse(&["11110", "11010", "11000", "00000"]);
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn example_three_islands() {
        let grid = parse(&["11000", "11000", "00100", "00011"]);
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn only_water_or_only_land() {
        assert_eq!(Solution::num_islands(parse(&["000", "000"])), 0);
        assert_eq!(Solution::num_islands(parse(&["111", "111"])), 1);
    }

    #[test]
    fn diagonal_cells_are_separate_islands() {
        assert_eq!(Solution::num_islands(parse(&["101", "010", "101"])), 5);
    }

    #[test]
    fn island_with_a_lake() {
        assert_eq!(Solution::num_islands(parse(&["111", "101", "111"])), 1);
    }

    #[test]
    fn large_checkerboard() {
        let grid: Vec<Vec<char>> = (0..300)
            .map(|r| {
                (0..300)
                    .map(|c| if (r + c) % 2 == 0 { '1' } else { '0' })
                    .collect()
            })
            .collect();
        assert_eq!(Solution::num_islands(grid), 45_000);
    }

    #[test]
    fn large_single_snake_island() {
        // Every other row is land; the rows are joined alternately at the right and left ends.
        let grid: Vec<Vec<char>> = (0..299)
            .map(|r| {
                (0..300)
                    .map(|c| {
                        let joined = (r % 4 == 1 && c == 299) || (r % 4 == 3 && c == 0);
                        if r % 2 == 0 || joined { '1' } else { '0' }
                    })
                    .collect()
            })
            .collect();
        assert_eq!(Solution::num_islands(grid), 1);
    }
}
