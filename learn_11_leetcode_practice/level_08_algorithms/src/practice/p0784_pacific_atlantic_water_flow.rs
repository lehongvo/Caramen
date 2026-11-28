// ==================================================================
// PRACTICE 0784: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0784_pacific_atlantic_water_flow.md
// Run the tests:          cargo test practice::p0784
// ==================================================================

//! # Problem 0784: Pacific Atlantic Water Flow
//!
//! - **Difficulty:** Medium
//! - **Topics:** reverse flood fill from two sources, `Vec<Vec<bool>>`, helper taking `impl Iterator`
//! - **Source:** [LeetCode 417. Pacific Atlantic Water Flow](https://leetcode.com/problems/pacific-atlantic-water-flow/)
//!
//! ## Problem
//!
//! An `m x n` island is described by `heights[r][c]`, the height of each cell. The **Pacific**
//! ocean touches the top and left edges of the island; the **Atlantic** touches the bottom and
//! right edges. Rain water flows from a cell to a 4-directionally adjacent cell whose height is
//! **less than or equal** to its own, and from any cell next to an ocean into that ocean.
//!
//! Return the coordinates `[r, c]` of every cell from which water can reach **both** oceans, in any
//! order.
//!
//! Implement `Solution::pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  heights = [[1, 2, 2, 3, 5],
//!                    [3, 2, 3, 4, 4],
//!                    [2, 4, 5, 3, 1],
//!                    [6, 7, 1, 4, 5],
//!                    [5, 1, 1, 2, 4]]
//! Output: [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
//!
//! Input:  heights = [[1]]
//! Output: [[0, 0]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`
//! - `0 <= heights[r][c] <= 10^5`
//!
//! ## Hints
//!
//! 1. Simulating the rain from every cell repeats a lot of work.
//! 2. Reverse the flow: start at the ocean and climb **up** (to neighbours that are at least as
//!    high). Every cell you reach can drain into that ocean.
//! 3. Do it once per ocean and intersect the results.

pub struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cells = Solution::pacific_atlantic(heights);
        cells.sort(); // any order is accepted
        cells
    }

    #[test]
    fn example_five_by_five() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let expected = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        assert_eq!(run(heights), expected);
    }

    #[test]
    fn example_single_cell() {
        assert_eq!(run(vec![vec![1]]), vec![vec![0, 0]]);
    }

    #[test]
    fn flat_island_drains_everywhere() {
        assert_eq!(run(vec![vec![3; 3]; 3]).len(), 9);
    }

    #[test]
    fn central_pit_reaches_no_ocean() {
        // The middle cell is lower than all its neighbours: water stays there.
        let heights = vec![vec![5, 5, 5], vec![5, 1, 5], vec![5, 5, 5]];
        let cells = run(heights);
        assert_eq!(cells.len(), 8);
        assert!(!cells.contains(&vec![1, 1]));
    }

    #[test]
    fn single_row_and_column() {
        assert_eq!(run(vec![vec![1, 2, 3]]).len(), 3);
        assert_eq!(run(vec![vec![3], vec![2], vec![1]]).len(), 3);
    }

    #[test]
    fn large_slope() {
        // Heights grow towards the bottom-right, so water only flows up or left: every cell
        // reaches the Pacific, but only the bottom row and the right column touch the Atlantic.
        let heights: Vec<Vec<i32>> = (0..200)
            .map(|r| (0..200).map(|c| r + c).collect())
            .collect();
        let cells = run(heights);
        assert_eq!(cells.len(), 200 + 199);
        assert!(cells.iter().all(|cell| cell[0] == 199 || cell[1] == 199));
    }
}
