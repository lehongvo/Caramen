// ==================================================================
// PRACTICE 0275: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0275_spiral_matrix_ii.md
// Run the tests:          cargo test practice::p0275
// ==================================================================

//! # Problem 0275: Spiral Matrix II
//!
//! - **Difficulty:** Medium
//! - **Topics:** a direction `enum` with `delta` and `turn`, a cursor `struct`, "turn when blocked" simulation
//! - **Source:** [LeetCode 59. Spiral Matrix II](https://leetcode.com/problems/spiral-matrix-ii/)
//!
//! ## Problem
//!
//! Given a positive integer `n`, build an `n x n` matrix filled with the numbers `1` to `n²` in
//! **spiral order**: `1` in the top-left corner, then increasing to the right along the top row,
//! down the right column, left along the bottom row, up the left column, and inwards clockwise.
//!
//! Implement `Solution::generate_matrix(n: i32) -> Vec<Vec<i32>>`.
//!
//! This time, use a different technique from Problem 0274: a cursor walks one cell at a time and
//! **turns right whenever the next cell is outside the matrix or already filled**.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 3
//! Output: [[1, 2, 3],
//!          [8, 9, 4],
//!          [7, 6, 5]]
//!
//! Input:  n = 1
//! Output: [[1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 20`
//!
//! ## Hints
//!
//! 1. Start with a matrix of zeros: `0` means "not filled yet".
//! 2. `Dir::delta(self) -> (isize, isize)` and `checked_add_signed` give the next cell, or `None`
//!    when it would leave the matrix through the top or left edge.

pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn example_one() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }

    #[test]
    fn two_and_four() {
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
        assert_eq!(
            Solution::generate_matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }

    #[test]
    fn twenty_contains_every_number_once() {
        let m = Solution::generate_matrix(20);
        assert_eq!(m[0][19], 20);
        assert_eq!(m[19][19], 39);
        let mut all: Vec<i32> = m.into_iter().flatten().collect();
        all.sort_unstable();
        assert_eq!(all, (1..=400).collect::<Vec<_>>());
    }
}
