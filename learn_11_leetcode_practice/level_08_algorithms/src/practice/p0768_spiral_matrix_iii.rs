// ==================================================================
// PRACTICE 0768: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0768_spiral_matrix_iii.md
// Run the tests:          cargo test practice::p0768
// ==================================================================

//! # Problem 0768: Spiral Matrix III
//!
//! - **Difficulty:** Medium
//! - **Topics:** simulation with direction vectors, signed coordinates, `Range::contains`
//! - **Source:** [LeetCode 885. Spiral Matrix III](https://leetcode.com/problems/spiral-matrix-iii/)
//!
//! ## Problem
//!
//! You stand on cell `(r_start, c_start)` of a `rows x cols` grid, facing east (towards larger
//! column numbers). Walk in a clockwise spiral: east, south, west, north, east, ... making the
//! spiral wider as you go. You may leave the grid during the walk (and come back later). Continue
//! until you have visited every cell of the grid.
//!
//! Return the grid cells as `[row, col]` pairs in the order you first visit them (cells outside
//! the grid are not listed).
//!
//! Implement `Solution::spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  rows = 1, cols = 4, r_start = 0, c_start = 0
//! Output: [[0, 0], [0, 1], [0, 2], [0, 3]]
//!
//! Input:  rows = 3, cols = 3, r_start = 1, c_start = 1
//! Output: [[1, 1], [1, 2], [2, 2], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [0, 2]]
//!
//!   The walk: 1 step east, 1 south, 2 west, 2 north, 3 east, 3 south, 4 west, ...
//! ```
//!
//! ## Constraints
//!
//! - `1 <= rows, cols <= 100`
//! - `0 <= r_start < rows` and `0 <= c_start < cols`
//!
//! ## Hints
//!
//! 1. The segment lengths of the spiral are `1, 1, 2, 2, 3, 3, 4, 4, ...`: each length is used for
//!    two directions in a row.
//! 2. Walk the spiral even outside the grid, and only record the positions that are inside.
//! 3. Coordinates can become negative during the walk, so keep them signed.

pub struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn pairs(cells: &[[i32; 2]]) -> Vec<Vec<i32>> {
        cells.iter().map(|c| c.to_vec()).collect()
    }

    #[test]
    fn example_single_row() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            pairs(&[[0, 0], [0, 1], [0, 2], [0, 3]])
        );
    }

    #[test]
    fn example_center_of_three_by_three() {
        assert_eq!(
            Solution::spiral_matrix_iii(3, 3, 1, 1),
            pairs(&[
                [1, 1],
                [1, 2],
                [2, 2],
                [2, 1],
                [2, 0],
                [1, 0],
                [0, 0],
                [0, 1],
                [0, 2]
            ])
        );
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::spiral_matrix_iii(1, 1, 0, 0), pairs(&[[0, 0]]));
    }

    #[test]
    fn five_by_six_from_inside() {
        let expected = pairs(&[
            [1, 4],
            [1, 5],
            [2, 5],
            [2, 4],
            [2, 3],
            [1, 3],
            [0, 3],
            [0, 4],
            [0, 5],
            [3, 5],
            [3, 4],
            [3, 3],
            [3, 2],
            [2, 2],
            [1, 2],
            [0, 2],
            [4, 5],
            [4, 4],
            [4, 3],
            [4, 2],
            [4, 1],
            [3, 1],
            [2, 1],
            [1, 1],
            [0, 1],
            [4, 0],
            [3, 0],
            [2, 0],
            [1, 0],
            [0, 0],
        ]);
        assert_eq!(Solution::spiral_matrix_iii(5, 6, 1, 4), expected);
    }

    #[test]
    fn large_grid_from_a_corner_visits_every_cell_once() {
        let visited = Solution::spiral_matrix_iii(100, 100, 99, 0);
        assert_eq!(visited.len(), 10_000);
        let distinct: HashSet<&Vec<i32>> = visited.iter().collect();
        assert_eq!(distinct.len(), 10_000);
        assert_eq!(visited[0], vec![99, 0]);
        assert_eq!(visited[1], vec![99, 1]);
    }
}
