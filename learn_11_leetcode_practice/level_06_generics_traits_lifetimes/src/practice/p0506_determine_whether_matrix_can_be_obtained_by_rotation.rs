// ==================================================================
// PRACTICE 0506: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0506_determine_whether_matrix_can_be_obtained_by_rotation.md
// Run the tests:          cargo test practice::p0506
// ==================================================================

//! # Problem 0506: Determine Whether Matrix Can Be Obtained By Rotation
//!
//! - **Difficulty:** Easy
//! - **Topics:** generic helper `rotate_clockwise<T: Clone>`, `PartialEq` on nested `Vec`, simulation
//! - **Source:** [LeetCode 1886. Determine Whether Matrix Can Be Obtained By Rotation](https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/)
//!
//! ## Problem
//!
//! You are given two square binary matrices `mat` and `target`, both `n x n`. You may rotate `mat`
//! by 90 degrees clockwise as many times as you like (zero times included). Return `true` if some
//! number of rotations makes `mat` exactly equal to `target`, and `false` otherwise.
//!
//! Implement `Solution::find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool`.
//!
//! Put the rotation in a **generic** helper
//! `rotate_clockwise<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>>` that rotates any rectangular
//! matrix by 90 degrees clockwise: an `r x c` input becomes a `c x r` output where
//! `out[j][r - 1 - i] == matrix[i][j]`. An empty matrix stays empty.
//!
//! ## Examples
//!
//! ```text
//! Input:  mat = [[0, 1], [1, 0]], target = [[1, 0], [0, 1]]
//! Output: true      (one clockwise rotation)
//!
//! Input:  mat = [[0, 1], [1, 1]], target = [[1, 0], [0, 1]]
//! Output: false     (no rotation of mat has three 1s turned into two)
//!
//! Input:  mat = [[0, 0, 0], [0, 1, 0], [1, 1, 1]], target = [[1, 1, 1], [0, 1, 0], [0, 0, 0]]
//! Output: true      (two rotations)
//!
//! rotate_clockwise(&[vec!['a', 'b', 'c']]) => [['a'], ['b'], ['c']]
//! ```
//!
//! ## Constraints
//!
//! - `n == mat.len() == mat[i].len() == target.len() == target[i].len()`
//! - `1 <= n <= 10`
//! - `mat[i][j]` and `target[i][j]` are `0` or `1`.
//!
//! ## Hints
//!
//! 1. After four rotations a matrix is back where it started, so only four candidates exist.
//! 2. Row `j` of the rotated matrix is column `j` of the original read from the **bottom** up.
//! 3. `Vec<Vec<i32>>` implements `PartialEq`, so `rotated == target` compares every cell.

pub struct Solution;

/// Rotates a rectangular matrix by 90 degrees clockwise.
pub fn rotate_clockwise<T: Clone>(matrix: &[Vec<T>]) -> Vec<Vec<T>> {
    todo!("Write your solution here")
}

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_rotation() {
        assert!(Solution::find_rotation(
            vec![vec![0, 1], vec![1, 0]],
            vec![vec![1, 0], vec![0, 1]]
        ));
    }

    #[test]
    fn example_impossible() {
        assert!(!Solution::find_rotation(
            vec![vec![0, 1], vec![1, 1]],
            vec![vec![1, 0], vec![0, 1]]
        ));
    }

    #[test]
    fn example_two_rotations() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let target = vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]];
        assert!(Solution::find_rotation(mat, target));
    }

    #[test]
    fn zero_and_three_rotations() {
        assert!(Solution::find_rotation(vec![vec![1]], vec![vec![1]]));
        assert!(!Solution::find_rotation(vec![vec![1]], vec![vec![0]]));
        // Three clockwise rotations equal one counter-clockwise rotation.
        let mat = vec![vec![1, 0], vec![0, 0]];
        let target = vec![vec![0, 0], vec![1, 0]];
        assert!(Solution::find_rotation(mat, target));
    }

    #[test]
    fn generic_rotation_of_chars() {
        assert_eq!(
            rotate_clockwise(&[vec!['a', 'b', 'c']]),
            vec![vec!['a'], vec!['b'], vec!['c']]
        );
        assert_eq!(
            rotate_clockwise(&[vec!['a', 'b'], vec!['c', 'd']]),
            vec![vec!['c', 'a'], vec!['d', 'b']]
        );
        let empty: Vec<Vec<u8>> = Vec::new();
        assert!(rotate_clockwise(&empty).is_empty());
    }

    #[test]
    fn four_rotations_return_to_start() {
        let m: Vec<Vec<i32>> = (0..10)
            .map(|i| (0..10).map(|j| (i * j) % 2).collect())
            .collect();
        let mut r = m.clone();
        for _ in 0..4 {
            r = rotate_clockwise(&r);
        }
        assert_eq!(r, m);
    }
}
