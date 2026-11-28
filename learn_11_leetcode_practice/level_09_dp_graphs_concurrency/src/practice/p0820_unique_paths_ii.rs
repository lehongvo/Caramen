// ==================================================================
// PRACTICE 0820: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0820_unique_paths_ii.md
// Run the tests:          cargo test practice::p0820
// ==================================================================

//! # Problem 0820: Unique Paths II
//!
//! - **Difficulty:** Medium
//! - **Topics:** grid DP with obstacles, one-row DP, `iter().zip()`, `wrapping_add`
//! - **Source:** [LeetCode 63. Unique Paths II](https://leetcode.com/problems/unique-paths-ii/)
//!
//! ## Problem
//!
//! Same robot as problem 0819 (it moves only right or down, from the top-left to the bottom-right
//! cell), but now some cells are blocked. `obstacle_grid[i][j]` is `1` for an obstacle and `0` for
//! a free cell. The robot can never enter an obstacle. Return the number of different paths; if
//! the start or the end is blocked, the answer is `0`.
//!
//! Implement `Solution::unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  obstacle_grid = [[0, 0, 0],
//!                          [0, 1, 0],
//!                          [0, 0, 0]]
//! Output: 2
//! Explanation: right-right-down-down or down-down-right-right.
//!
//! Input:  obstacle_grid = [[0, 1],
//!                          [0, 0]]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 100` (the grid has `m` rows and `n` columns)
//! - `obstacle_grid[i][j]` is `0` or `1`
//! - The answer is at most `2 * 10^9`.
//!
//! ## Hints
//!
//! 1. Same recurrence as problem 0819, except that an obstacle cell has `0` paths.
//! 2. Do not initialise the first row/column with ones: a cell after an obstacle in the first row
//!    is unreachable.
//! 3. Path counts of cells that cannot reach the end can be huge even when the answer is small.

pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_center_obstacle() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 2);
    }

    #[test]
    fn example_two_by_two() {
        let grid = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 1);
    }

    #[test]
    fn blocked_start_or_end() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1]]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0, 1]]), 0);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0]]), 1);
    }

    #[test]
    fn obstacle_in_first_row_blocks_the_rest_of_it() {
        let grid = vec![vec![0, 1, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 0);
    }

    #[test]
    fn intermediate_overflow_is_harmless() {
        // Row 98 is a wall except column 0, so the huge counts of the top area cannot reach the
        // end; only the path down column 0 and along the last row survives.
        let mut grid = vec![vec![0; 100]; 100];
        for cell in grid[98].iter_mut().skip(1) {
            *cell = 1;
        }
        assert_eq!(Solution::unique_paths_with_obstacles(grid), 1);
    }
}
