// ==================================================================
// PRACTICE 0766: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0766_set_matrix_zeroes.md
// Run the tests:          cargo test practice::p0766
// ==================================================================

//! # Problem 0766: Set Matrix Zeroes
//!
//! - **Difficulty:** Medium
//! - **Topics:** in-place markers in the first row and column, `split_at_mut`, `fill`, `&mut Vec<Vec<i32>>`
//! - **Source:** [LeetCode 73. Set Matrix Zeroes](https://leetcode.com/problems/set-matrix-zeroes/)
//!
//! ## Problem
//!
//! Given an `m x n` integer matrix, whenever an element is `0`, set its **entire row and column**
//! to `0`. Only the zeros of the original matrix count (zeros you write do not spread further).
//! Modify the matrix **in place**; the function returns nothing.
//!
//! Try to use only `O(1)` extra memory.
//!
//! Implement `Solution::set_zeroes(matrix: &mut Vec<Vec<i32>>)`.
//!
//! ## Examples
//!
//! ```text
//! Input:  [[1, 1, 1],          Output: [[1, 0, 1],
//!          [1, 0, 1],                   [0, 0, 0],
//!          [1, 1, 1]]                   [1, 0, 1]]
//!
//! Input:  [[0, 1, 2, 0],       Output: [[0, 0, 0, 0],
//!          [3, 4, 5, 2],                [0, 4, 5, 0],
//!          [1, 3, 1, 5]]                [0, 3, 1, 0]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`
//! - `-2^31 <= matrix[i][j] <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Writing zeros while scanning destroys information. First record **which** rows and columns
//!    must be cleared, then clear them.
//! 2. Two boolean vectors take `O(m + n)` memory. The first row and the first column of the matrix
//!    itself can store those flags instead.
//! 3. The first row and column then need two extra flags of their own, computed before they are
//!    overwritten.

pub struct Solution;

impl Solution {
    // LeetCode's signature uses `&mut Vec<Vec<i32>>`; Clippy would suggest `&mut [Vec<i32>]`.
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::set_zeroes(&mut matrix);
        matrix
    }

    /// Straightforward version with two flag vectors, used to check the in-place one.
    fn naive(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let rows: Vec<bool> = matrix.iter().map(|r| r.contains(&0)).collect();
        let cols: Vec<bool> = (0..matrix[0].len())
            .map(|j| matrix.iter().any(|r| r[j] == 0))
            .collect();
        matrix
            .iter()
            .enumerate()
            .map(|(i, r)| {
                r.iter()
                    .enumerate()
                    .map(|(j, &x)| if rows[i] || cols[j] { 0 } else { x })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn example_center_zero() {
        assert_eq!(
            run(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]
        );
    }

    #[test]
    fn example_zeros_in_first_row() {
        assert_eq!(
            run(vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]]),
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }

    #[test]
    fn single_cell() {
        assert_eq!(run(vec![vec![1]]), vec![vec![1]]);
        assert_eq!(run(vec![vec![0]]), vec![vec![0]]);
    }

    #[test]
    fn zero_only_in_first_row_or_first_column() {
        assert_eq!(
            run(vec![vec![1, 0], vec![1, 1]]),
            vec![vec![0, 0], vec![1, 0]]
        );
        assert_eq!(
            run(vec![vec![1, 1], vec![0, 1]]),
            vec![vec![0, 1], vec![0, 0]]
        );
    }

    #[test]
    fn single_row_and_single_column() {
        assert_eq!(run(vec![vec![1, 0, 3]]), vec![vec![0, 0, 0]]);
        assert_eq!(
            run(vec![vec![1], vec![0], vec![3]]),
            vec![vec![0], vec![0], vec![0]]
        );
    }

    #[test]
    fn large_matrix_matches_naive_version() {
        let matrix: Vec<Vec<i32>> = (0..200)
            .map(|i| {
                (0..200)
                    .map(|j| {
                        if (i * 37 + j * 11) % 97 == 0 {
                            0
                        } else {
                            i + j + 1
                        }
                    })
                    .collect()
            })
            .collect();
        assert_eq!(run(matrix.clone()), naive(&matrix));
    }
}
