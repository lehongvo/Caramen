// ==================================================================
// PRACTICE 0326: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0326_transpose_matrix.md
// Run the tests:          cargo test practice::p0326
// ==================================================================

//! # Problem 0326: Transpose Matrix
//!
//! - **Difficulty:** Easy
//! - **Topics:** non-square `Vec<Vec<i32>>`, nested `map` + `collect`, row/column indices
//! - **Source:** [LeetCode 867. Transpose Matrix](https://leetcode.com/problems/transpose-matrix/)
//!
//! ## Problem
//!
//! The **transpose** of a matrix flips it over its main diagonal: the element at row `r`, column
//! `c` moves to row `c`, column `r`. An `m x n` matrix becomes an `n x m` matrix.
//!
//! Given a matrix as a vector of rows (all rows have the same length), return its transpose.
//!
//! Implement `Solution::transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  matrix = [[1, 2, 3],
//!                   [4, 5, 6],
//!                   [7, 8, 9]]
//! Output: [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
//!
//! Input:  matrix = [[1, 2, 3],
//!                   [4, 5, 6]]
//! Output: [[1, 4], [2, 5], [3, 6]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 1000` and `1 <= m * n <= 10^5`
//! - `-10^9 <= matrix[r][c] <= 10^9`
//!
//! ## Hints
//!
//! 1. For a non-square matrix the result has a different shape, so it cannot be done by swapping
//!    in place; build a new matrix.
//! 2. Row `c` of the answer is column `c` of the input: `matrix.iter().map(|row| row[c])`.

pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_square() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }

    #[test]
    fn example_rectangle() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }

    #[test]
    fn single_row_and_single_cell() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3]]),
            vec![vec![1], vec![2], vec![3]]
        );
        assert_eq!(Solution::transpose(vec![vec![-7]]), vec![vec![-7]]);
    }

    #[test]
    fn transposing_twice_gives_back_the_input() {
        let matrix: Vec<Vec<i32>> = (0..100)
            .map(|r| (0..1000).map(|c| r * 1000 + c).collect())
            .collect();
        let t = Solution::transpose(matrix.clone());
        assert_eq!(t.len(), 1000);
        assert_eq!(t[999][99], 99_999);
        assert_eq!(Solution::transpose(t), matrix);
    }
}
