// ==================================================================
// PRACTICE 0274: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0274_spiral_matrix.md
// Run the tests:          cargo test practice::p0274
// ==================================================================

//! # Problem 0274: Spiral Matrix
//!
//! - **Difficulty:** Medium
//! - **Topics:** a `Bounds` struct that shrinks, a direction `enum` with `turn`, `match` on the direction, simulation
//! - **Source:** [LeetCode 54. Spiral Matrix](https://leetcode.com/problems/spiral-matrix/)
//!
//! ## Problem
//!
//! Given an `m x n` matrix, return all of its elements in **spiral order**: start at the top-left
//! corner, go right along the top row, then down the right column, then left along the bottom row,
//! then up the left column, and continue inwards in the same clockwise way until every element has
//! been visited exactly once.
//!
//! Implement `Solution::spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! For this level, keep the not-yet-visited rectangle in a `struct Bounds { top, bottom, left, right }`
//! and the current direction in an `enum Dir { Right, Down, Left, Up }`.
//!
//! ## Examples
//!
//! ```text
//! Input:  matrix = [[1, 2, 3],
//!                   [4, 5, 6],
//!                   [7, 8, 9]]
//! Output: [1, 2, 3, 6, 9, 8, 7, 4, 5]
//!
//! Input:  matrix = [[1, 2, 3, 4],
//!                   [5, 6, 7, 8],
//!                   [9, 10, 11, 12]]
//! Output: [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 10`, `-100 <= matrix[i][j] <= 100`
//!
//! ## Hints
//!
//! 1. Each pass along one side visits that whole side of the remaining rectangle, then removes it:
//!    going right consumes the top row (`top += 1`), going down consumes the right column, and so on.
//! 2. Stop as soon as the rectangle is empty (`top > bottom` or `left > right`).

pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_square() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn example_wide() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn single_row_and_single_column() {
        assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3]]), vec![1, 2, 3]);
        assert_eq!(
            Solution::spiral_order(vec![vec![1], vec![2], vec![3]]),
            vec![1, 2, 3]
        );
        assert_eq!(Solution::spiral_order(vec![vec![-7]]), vec![-7]);
    }

    #[test]
    fn tall_matrix() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            vec![1, 2, 4, 6, 8, 7, 5, 3]
        );
    }

    #[test]
    fn largest_matrix_visits_everything_once() {
        let m: Vec<Vec<i32>> = (0..10)
            .map(|r| (0..10).map(|c| r * 10 + c).collect())
            .collect();
        let mut out = Solution::spiral_order(m);
        assert_eq!(&out[..11], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 19]);
        out.sort_unstable();
        assert_eq!(out, (0..100).collect::<Vec<_>>());
    }
}
