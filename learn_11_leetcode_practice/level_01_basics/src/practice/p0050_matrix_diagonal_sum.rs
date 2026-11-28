// ==================================================================
// PRACTICE 0050: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0050_matrix_diagonal_sum.md
// Run the tests:          cargo test practice::p0050
// ==================================================================

//! # Problem 0050: Matrix Diagonal Sum
//!
//! - **Difficulty:** Easy
//! - **Topics:** 2D indexing, `enumerate`, the center cell of an odd-sized square
//! - **Source:** [LeetCode 1572. Matrix Diagonal Sum](https://leetcode.com/problems/matrix-diagonal-sum/)
//!
//! ## Problem
//!
//! Given a square matrix `mat` (`n x n`), return the sum of the elements on its two diagonals:
//!
//! - the **primary** diagonal goes from the top-left to the bottom-right corner (`mat[i][i]`);
//! - the **secondary** diagonal goes from the top-right to the bottom-left corner (`mat[i][n - 1 - i]`).
//!
//! When `n` is odd, the two diagonals cross at the center cell; count that cell **only once**.
//!
//! Implement `Solution::diagonal_sum(mat: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mat = [[1, 2, 3],
//!                [4, 5, 6],
//!                [7, 8, 9]]
//! Output: 25
//! Explanation: 1 + 5 + 9 + 3 + 7 = 25 (the 5 in the center is counted once).
//!
//! Input:  mat = [[1, 1, 1, 1],
//!                [1, 1, 1, 1],
//!                [1, 1, 1, 1],
//!                [1, 1, 1, 1]]
//! Output: 8
//!
//! Input:  mat = [[5]]
//! Output: 5
//! ```
//!
//! ## Constraints
//!
//! - `n == mat.len() == mat[i].len()`
//! - `1 <= n <= 100`
//! - `1 <= mat[i][j] <= 100`
//!
//! ## Hints
//!
//! 1. Row `i` contributes `mat[i][i]` and `mat[i][n - 1 - i]`.
//! 2. Those two are the same cell exactly when `i == n - 1 - i`.

pub struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_odd_size() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::diagonal_sum(mat), 25);
    }

    #[test]
    fn example_even_size() {
        assert_eq!(Solution::diagonal_sum(vec![vec![1; 4]; 4]), 8);
    }

    #[test]
    fn example_single_cell() {
        assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
    }

    #[test]
    fn two_by_two() {
        assert_eq!(Solution::diagonal_sum(vec![vec![1, 2], vec![3, 4]]), 10);
    }

    #[test]
    fn largest_matrices() {
        assert_eq!(Solution::diagonal_sum(vec![vec![100; 100]; 100]), 20_000);
        assert_eq!(Solution::diagonal_sum(vec![vec![100; 99]; 99]), 19_700);
    }
}
