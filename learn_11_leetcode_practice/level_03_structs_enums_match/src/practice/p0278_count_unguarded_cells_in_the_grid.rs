// ==================================================================
// PRACTICE 0278: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0278_count_unguarded_cells_in_the_grid.md
// Run the tests:          cargo test practice::p0278
// ==================================================================

//! # Problem 0278: Count Unguarded Cells in the Grid
//!
//! - **Difficulty:** Medium
//! - **Topics:** a `Cell` enum grid, `matches!` to stop a ray, `while let` over an `Option` cursor, counting with `filter`
//! - **Source:** [LeetCode 2257. Count Unguarded Cells in the Grid](https://leetcode.com/problems/count-unguarded-cells-in-the-grid/)
//!
//! ## Problem
//!
//! An `m x n` grid contains some **guards** and some **walls**, given as lists of `[row, col]`
//! positions. Every guard watches in the four directions (up, down, left, right) as far as the
//! edge of the grid, but its view is stopped by a wall or by another guard. A cell is **guarded**
//! if at least one guard sees it.
//!
//! Return the number of cells that are **not** guarded and contain neither a guard nor a wall.
//!
//! Implement
//! `Solution::count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32`.
//!
//! For this level, store the grid as `Vec<Vec<Cell>>` with `enum Cell { Free, Guard, Wall, Watched }`.
//!
//! ## Examples
//!
//! ```text
//! Input:  m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
//! Output: 7
//!
//! Input:  m = 3, n = 3, guards = [[1,1]], walls = [[0,1],[1,0],[2,1],[1,2]]
//! Output: 4      (the guard is boxed in by walls; the four corners are unguarded)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 10^5`, `2 <= m * n <= 10^5`
//! - `1 <= guards.len() + walls.len() <= m * n`; all positions are distinct and inside the grid.
//!
//! ## Hints
//!
//! 1. Place all guards and walls first, then cast four rays from every guard.
//! 2. A ray stops at `matches!(cell, Cell::Guard | Cell::Wall)`; free cells along it become `Watched`.
//! 3. At the end, count the cells that are still `Free`.

pub struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(list: &[[i32; 2]]) -> Vec<Vec<i32>> {
        list.iter().map(|p| p.to_vec()).collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::count_unguarded(
                4,
                6,
                cells(&[[0, 0], [1, 1], [2, 3]]),
                cells(&[[0, 1], [2, 2], [1, 4]])
            ),
            7
        );
    }

    #[test]
    fn example_boxed_in_guard() {
        assert_eq!(
            Solution::count_unguarded(
                3,
                3,
                cells(&[[1, 1]]),
                cells(&[[0, 1], [1, 0], [2, 1], [1, 2]])
            ),
            4
        );
    }

    #[test]
    fn guards_block_each_other() {
        // Row of 5: G G . . .  The second guard sees the rest of the row; nothing is free.
        assert_eq!(
            Solution::count_unguarded(1, 5, cells(&[[0, 0], [0, 1]]), vec![]),
            0
        );
        // 2 x 3 with a wall between: G W .
        //                            . . .
        assert_eq!(
            Solution::count_unguarded(2, 3, cells(&[[0, 0]]), cells(&[[0, 1]])),
            3
        );
    }

    #[test]
    fn long_thin_grid() {
        // 1 x 100_000 with one guard in the middle: everything is watched.
        assert_eq!(
            Solution::count_unguarded(1, 100_000, cells(&[[0, 50_000]]), vec![]),
            0
        );
        // A wall right next to the guard stops the view to the right.
        assert_eq!(
            Solution::count_unguarded(1, 100_000, cells(&[[0, 0]]), cells(&[[0, 1]])),
            99_998
        );
    }
}
