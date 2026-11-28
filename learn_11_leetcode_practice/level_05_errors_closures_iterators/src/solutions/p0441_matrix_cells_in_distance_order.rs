//! # Problem 0441: Matrix Cells in Distance Order
//!
//! - **Difficulty:** Easy
//! - **Topics:** nested `flat_map` with an inner `move` closure, `sort_by_key` with a capturing closure, Manhattan distance
//! - **Source:** [LeetCode 1030. Matrix Cells in Distance Order](https://leetcode.com/problems/matrix-cells-in-distance-order/)
//!
//! ## Problem
//!
//! A grid has `rows` rows and `cols` columns; cell `(r, c)` is in row `r` and column `c`
//! (both 0-based). One cell, `(r_center, c_center)`, is the *center*.
//!
//! Return the coordinates `[r, c]` of **every** cell of the grid, sorted by their Manhattan
//! distance to the center, `|r - r_center| + |c - c_center|`, from the closest to the farthest.
//! Cells at the same distance may appear in any order.
//!
//! Implement `Solution::all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  rows = 1, cols = 2, r_center = 0, c_center = 0
//! Output: [[0,0],[0,1]]
//!
//! Input:  rows = 2, cols = 2, r_center = 0, c_center = 1
//! Output: [[0,1],[0,0],[1,1],[1,0]]
//! Explanation: distances are 0, 1, 1, 2. [[0,1],[1,1],[0,0],[1,0]] is also accepted.
//!
//! Input:  rows = 2, cols = 3, r_center = 1, c_center = 2
//! Output: [[1,2],[0,2],[1,1],[0,1],[1,0],[0,0]]   (distances 0, 1, 1, 2, 2, 3)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= rows, cols <= 100`
//! - `0 <= r_center < rows`, `0 <= c_center < cols`
//!
//! ## Hints
//!
//! 1. Generate all cells with `(0..rows).flat_map(|r| (0..cols).map(move |c| vec![r, c]))`. The
//!    inner closure needs `move` because it outlives the outer closure's call that owns `r`.
//! 2. Sort with `sort_by_key`, passing a closure that **captures** the center and computes the
//!    distance of a cell.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** list every cell, then sort the list by distance. The key closure captures
//! `r_center` and `c_center` from the function's parameters, which is exactly the kind of context a
//! plain function could not carry.
//!
//! **Complexity:** `O(R * C * log(R * C))` time, `O(R * C)` space for the output. (A bucket sort by
//! distance would be linear, but the sort is simpler and fast enough.)
//!
//! **Rust concepts:**
//! - `flat_map` turns "for each row, an iterator of cells" into one flat iterator.
//! - The inner `move |c|` copies `r` into the inner closure; without it, the inner iterator would
//!   borrow `r`, a variable that dies when the outer closure returns (error E0373).
//! - `sort_by_key` is stable, so ties stay in row-major order (any order is accepted anyway).
// @solution-end

pub struct Solution;

impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        // @solution-start
        let mut cells: Vec<Vec<i32>> = (0..rows)
            .flat_map(|r| (0..cols).map(move |c| vec![r, c]))
            .collect();
        cells.sort_by_key(|cell| (cell[0] - r_center).abs() + (cell[1] - c_center).abs());
        cells
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Checks that `cells` contains every cell exactly once, in non-decreasing distance order.
    fn check(rows: i32, cols: i32, rc: i32, cc: i32) -> Vec<Vec<i32>> {
        let cells = Solution::all_cells_dist_order(rows, cols, rc, cc);
        assert_eq!(cells.len(), (rows * cols) as usize);
        let unique: HashSet<&Vec<i32>> = cells.iter().collect();
        assert_eq!(unique.len(), cells.len());
        assert!(
            cells
                .iter()
                .all(|c| (0..rows).contains(&c[0]) && (0..cols).contains(&c[1]))
        );
        let dist: Vec<i32> = cells
            .iter()
            .map(|c| (c[0] - rc).abs() + (c[1] - cc).abs())
            .collect();
        assert!(dist.windows(2).all(|w| w[0] <= w[1]));
        cells
    }

    #[test]
    fn example_one_row() {
        assert_eq!(check(1, 2, 0, 0), vec![vec![0, 0], vec![0, 1]]);
    }

    #[test]
    fn example_two_by_two() {
        let cells = check(2, 2, 0, 1);
        assert_eq!(cells[0], vec![0, 1]);
        assert_eq!(cells[3], vec![1, 0]);
    }

    #[test]
    fn example_two_by_three() {
        let cells = check(2, 3, 1, 2);
        assert_eq!(cells[0], vec![1, 2]);
        assert_eq!(cells[5], vec![0, 0]);
    }

    #[test]
    fn single_cell_and_large_grid() {
        assert_eq!(check(1, 1, 0, 0), vec![vec![0, 0]]);
        let cells = check(100, 100, 37, 81);
        assert_eq!(cells[0], vec![37, 81]);
        assert_eq!(cells[9_999], vec![99, 0]);
    }
}
