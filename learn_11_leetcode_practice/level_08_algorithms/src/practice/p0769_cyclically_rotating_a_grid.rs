// ==================================================================
// PRACTICE 0769: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0769_cyclically_rotating_a_grid.md
// Run the tests:          cargo test practice::p0769
// ==================================================================

//! # Problem 0769: Cyclically Rotating a Grid
//!
//! - **Difficulty:** Medium
//! - **Topics:** matrix layers, collecting a layer's positions, `slice::rotate_left`, `k % len`
//! - **Source:** [LeetCode 1914. Cyclically Rotating a Grid](https://leetcode.com/problems/cyclically-rotating-a-grid/)
//!
//! ## Problem
//!
//! You are given an `m x n` grid where `m` and `n` are both **even**, and an integer `k`. The grid
//! is made of concentric **layers** (rings): the outer border, the border of what remains inside,
//! and so on.
//!
//! One cyclic rotation moves every element of every layer one position **counter-clockwise**
//! along its own layer. Return the grid after applying `k` rotations.
//!
//! Implement `Solution::rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[40, 10],
//!                 [30, 20]], k = 1
//! Output: [[10, 20],
//!          [40, 30]]
//!
//! Input:  grid = [[ 1,  2,  3,  4],        Output: [[ 3,  4,  8, 12],
//!                 [ 5,  6,  7,  8],                 [ 2, 11, 10, 16],
//!                 [ 9, 10, 11, 12],                 [ 1,  7,  6, 15],
//!                 [13, 14, 15, 16]], k = 2          [ 5,  9, 13, 14]]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= m, n <= 50`, both even
//! - `1 <= grid[i][j] <= 5000`
//! - `1 <= k <= 10^9`
//!
//! ## Hints
//!
//! 1. Handle each layer separately. List the positions of a layer in clockwise order.
//! 2. In that order, a counter-clockwise rotation by one shifts the **values** one place to the
//!    left. A layer of length `len` is back to its start after `len` rotations.

pub struct Solution;

impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn four_by_four() -> Vec<Vec<i32>> {
        (0..4)
            .map(|r| (1..=4).map(|c| r * 4 + c).collect())
            .collect()
    }

    #[test]
    fn example_two_by_two() {
        assert_eq!(
            Solution::rotate_grid(vec![vec![40, 10], vec![30, 20]], 1),
            vec![vec![10, 20], vec![40, 30]]
        );
    }

    #[test]
    fn example_four_by_four() {
        assert_eq!(
            Solution::rotate_grid(four_by_four(), 2),
            vec![
                vec![3, 4, 8, 12],
                vec![2, 11, 10, 16],
                vec![1, 7, 6, 15],
                vec![5, 9, 13, 14]
            ]
        );
    }

    #[test]
    fn full_turns_change_nothing() {
        // The outer layer has 12 cells and the inner one 4: 12 rotations restore both.
        assert_eq!(Solution::rotate_grid(four_by_four(), 12), four_by_four());
    }

    #[test]
    fn rectangular_grid() {
        let grid = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]];
        // One layer, clockwise: 1 2 3 4 8 7 6 5; rotated left by one: 2 3 4 8 7 6 5 1.
        assert_eq!(
            Solution::rotate_grid(grid, 1),
            vec![vec![2, 3, 4, 8], vec![1, 5, 6, 7]]
        );
    }

    #[test]
    fn huge_k() {
        // 10^9 % 12 = 4 for the outer layer and 10^9 % 4 = 0 for the inner one, exactly like k = 4.
        let rotated = Solution::rotate_grid(four_by_four(), 1_000_000_000);
        assert_eq!(rotated, Solution::rotate_grid(four_by_four(), 4));
        assert_eq!(rotated[1][1..3], [6, 7]); // the inner layer did not move
        assert_eq!(rotated[2][1..3], [10, 11]);
    }

    #[test]
    fn largest_grid_single_rotation() {
        let grid: Vec<Vec<i32>> = (0..50)
            .map(|r| (0..50).map(|c| r * 50 + c + 1).collect())
            .collect();
        let rotated = Solution::rotate_grid(grid.clone(), 1);
        // Outer layer: the top-left corner receives its right neighbour, and the old corner value
        // moves down one cell.
        assert_eq!(rotated[0][0], grid[0][1]);
        assert_eq!(rotated[1][0], grid[0][0]);
        assert_eq!(rotated[49][49], grid[49][48]); // the bottom row moves right
        assert_eq!(rotated[48][49], grid[49][49]); // the right column moves up
        // Second layer: same rule one ring further in.
        assert_eq!(rotated[1][1], grid[1][2]);
        // No value is lost or duplicated.
        let mut values = rotated.concat();
        values.sort_unstable();
        assert_eq!(values, (1..=2500).collect::<Vec<i32>>());
    }
}
