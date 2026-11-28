//! # Problem 0327: Reshape the Matrix
//!
//! - **Difficulty:** Easy
//! - **Topics:** flattening with `concat`, regrouping with `chunks`, `to_vec`, returning the input unchanged
//! - **Source:** [LeetCode 566. Reshape the Matrix](https://leetcode.com/problems/reshape-the-matrix/)
//!
//! ## Problem
//!
//! You are given an `m x n` matrix `mat` and two integers `r` and `c`. **Reshaping** it means
//! reading all its elements row by row (left to right, top to bottom) and writing them, in that
//! same order, into a new matrix with `r` rows and `c` columns.
//!
//! If the reshape is possible (the new matrix has exactly `m * n` cells), return the new matrix.
//! Otherwise return the original matrix unchanged.
//!
//! Implement `Solution::matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mat = [[1, 2], [3, 4]], r = 1, c = 4
//! Output: [[1, 2, 3, 4]]
//!
//! Input:  mat = [[1, 2], [3, 4]], r = 2, c = 4
//! Output: [[1, 2], [3, 4]]       (8 cells needed but only 4 elements: unchanged)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 100`
//! - `-1000 <= mat[i][j] <= 1000`
//! - `1 <= r, c <= 300`
//!
//! ## Hints
//!
//! 1. Flatten first: `mat.concat()` gives all elements in row-major order.
//! 2. Then cut the flat vector into rows of `c` elements with `chunks(c)`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** check that `m * n == r * c`. If not, return `mat` as it is (it was moved in, so it can
//! be moved back out). Otherwise flatten the matrix with `concat` and regroup the flat vector into
//! rows of length `c` with `chunks`.
//!
//! **Complexity:** `O(m * n)` time and space.
//!
//! **Rust concepts:**
//! - `concat()` on `[Vec<i32>]` allocates one `Vec<i32>` holding all rows back to back.
//! - `chunks(c)` borrows slices of the flat vector; `to_vec()` copies each into an owned row.
//! - Returning `mat` itself in the impossible case costs nothing: ownership is simply handed back.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        // @solution-start
        let (r, c) = (r as usize, c as usize);
        let total: usize = mat.iter().map(|row| row.len()).sum();
        if total != r * c {
            return mat;
        }
        mat.concat().chunks(c).map(|row| row.to_vec()).collect()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_row() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
    }

    #[test]
    fn example_impossible_returns_input() {
        assert_eq!(
            Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            vec![vec![1, 2], vec![3, 4]]
        );
    }

    #[test]
    fn to_a_column_and_back() {
        let column = Solution::matrix_reshape(vec![vec![1, 2, 3], vec![4, 5, 6]], 6, 1);
        assert_eq!(
            column,
            vec![vec![1], vec![2], vec![3], vec![4], vec![5], vec![6]]
        );
        assert_eq!(
            Solution::matrix_reshape(column, 3, 2),
            vec![vec![1, 2], vec![3, 4], vec![5, 6]]
        );
    }

    #[test]
    fn same_shape_is_identity() {
        let mat = vec![vec![-1000, 1000], vec![0, 7]];
        assert_eq!(Solution::matrix_reshape(mat.clone(), 2, 2), mat);
    }

    #[test]
    fn large_matrix() {
        let mat: Vec<Vec<i32>> = (0..100)
            .map(|r| (0..100).map(|c| (r * 100 + c) % 2001 - 1000).collect())
            .collect();
        let reshaped = Solution::matrix_reshape(mat.clone(), 50, 200);
        assert_eq!(reshaped.len(), 50);
        assert!(reshaped.iter().all(|row| row.len() == 200));
        assert_eq!(reshaped.concat(), mat.concat());
        assert_eq!(Solution::matrix_reshape(mat.clone(), 300, 300), mat);
    }
}
