// ==================================================================
// PRACTICE 0378: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0378_equal_row_and_column_pairs.md
// Run the tests:          cargo test practice::p0378
// ==================================================================

//! # Problem 0378: Equal Row and Column Pairs
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashMap<&[i32], i32>` keyed by borrowed slices, building columns, `get` with a slice key
//! - **Source:** [LeetCode 2352. Equal Row and Column Pairs](https://leetcode.com/problems/equal-row-and-column-pairs/)
//!
//! ## Problem
//!
//! Given an `n x n` integer matrix `grid`, return the number of pairs `(r, c)` such that row `r`
//! and column `c` are **equal**: they contain the same elements in the same order (row read left to
//! right, column read top to bottom).
//!
//! Implement `Solution::equal_pairs(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[3, 2, 1],
//!                 [1, 7, 6],
//!                 [2, 7, 7]]
//! Output: 1      (row 2 = column 1 = [2, 7, 7])
//!
//! Input:  grid = [[3, 1, 2, 2],
//!                 [1, 4, 4, 5],
//!                 [2, 4, 2, 2],
//!                 [2, 4, 2, 2]]
//! Output: 3      (row 0 = column 0; rows 2 and 3 both equal column 2)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 200`
//! - `1 <= grid[i][j] <= 10^5`
//!
//! ## Hints
//!
//! 1. Comparing every row with every column element by element is `O(n^3)`. Hashing is faster.
//! 2. Count the rows in a `HashMap` (several rows can be equal). Then build each column and look it
//!    up.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ]),
            3
        );
    }

    #[test]
    fn single_cell_and_symmetric() {
        assert_eq!(Solution::equal_pairs(vec![vec![5]]), 1);
        // A symmetric matrix: row i equals column i for every i.
        assert_eq!(Solution::equal_pairs(vec![vec![1, 2], vec![2, 3]]), 2);
    }

    #[test]
    fn all_equal_large_grid() {
        // Every row equals every column: 200 * 200 pairs.
        let grid = vec![vec![7; 200]; 200];
        assert_eq!(Solution::equal_pairs(grid), 40_000);
        let distinct: Vec<Vec<i32>> = (0..200)
            .map(|r| (0..200).map(|c| r * 200 + c + 1).collect())
            .collect();
        assert_eq!(Solution::equal_pairs(distinct), 0);
    }
}
