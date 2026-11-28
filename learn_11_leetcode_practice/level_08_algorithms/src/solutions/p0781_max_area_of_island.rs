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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** recursive DFS that counts: `sink(r, c)` returns `0` for water, out-of-range or
//! already visited cells. Otherwise it marks the cell as water (visited) and returns
//! `1 + sink(down) + sink(right) + sink(up) + sink(left)`. Calling it from every cell and keeping
//! the maximum gives the answer; cells already sunk contribute `0`, so each island is measured once.
//! The recursion depth is at most the island size (`2500` here), which is fine for the default
//! stack; for much larger grids, use an explicit stack as in the previous problems.
//!
//! **Complexity:** `O(m * n)` time, `O(m * n)` recursion depth in the worst case.
//!
//! **Rust concepts:**
//! - The helper takes `&mut [Vec<i32>]` (a mutable slice of rows): it can modify cells without
//!   owning the grid, and `&mut grid` coerces from `&mut Vec<Vec<i32>>` automatically.
//! - `r.checked_sub(1).map_or(0, |up| sink(grid, up, c))` handles "no row above" without
//!   underflowing; going down or right is checked by the bounds test at the start of `sink`.
// @solution-end

pub struct Solution;

// @solution-start remove
/// Sinks the island containing `(r, c)` and returns its area (0 for water or out of range).
fn sink(grid: &mut [Vec<i32>], r: usize, c: usize) -> i32 {
    if r >= grid.len() || c >= grid[0].len() || grid[r][c] != 1 {
        return 0;
    }
    grid[r][c] = 0; // mark as visited
    1 + sink(grid, r + 1, c)
        + sink(grid, r, c + 1)
        + r.checked_sub(1).map_or(0, |up| sink(grid, up, c))
        + c.checked_sub(1).map_or(0, |left| sink(grid, r, left))
}
// @solution-end

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut best = 0;
        for r in 0..m {
            for c in 0..n {
                best = best.max(sink(&mut grid, r, c));
            }
        }
        best
        // @solution-end
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
