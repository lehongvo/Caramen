// ==================================================================
// PRACTICE 0822: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0822_triangle.md
// Run the tests:          cargo test practice::p0822
// ==================================================================

//! # Problem 0822: Triangle
//!
//! - **Difficulty:** Medium
//! - **Topics:** bottom-up DP over rows, `iter().rev()`, `split_last`, `windows`-free updates
//! - **Source:** [LeetCode 120. Triangle](https://leetcode.com/problems/triangle/)
//!
//! ## Problem
//!
//! `triangle` has rows of lengths `1, 2, 3, ...`. Start at the top and move to the bottom row one
//! row at a time; from index `j` of a row you may move to index `j` or `j + 1` of the next row.
//! Return the minimum possible sum of the numbers on such a path.
//!
//! Implement `Solution::minimum_total(triangle: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  triangle = [[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]]
//! Output: 11
//! Explanation:
//!        2
//!       3 4
//!      6 5 7
//!     4 1 8 3
//! The path 2 -> 3 -> 5 -> 1 has sum 11.
//!
//! Input:  triangle = [[-10]]
//! Output: -10
//! ```
//!
//! ## Constraints
//!
//! - `1 <= triangle.len() <= 200`
//! - `triangle[0].len() == 1` and `triangle[i].len() == triangle[i - 1].len() + 1`
//! - `-10^4 <= triangle[i][j] <= 10^4`
//!
//! ## Hints
//!
//! 1. Top-down needs special cases on both edges of each row. Bottom-up has none: every cell has
//!    exactly two children.
//! 2. Start from a copy of the bottom row; each cell above becomes
//!    `value + min(left child, right child)`.

pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_rows() {
        let t = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(t), 11);
    }

    #[test]
    fn example_single_row() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }

    #[test]
    fn two_rows() {
        assert_eq!(Solution::minimum_total(vec![vec![1], vec![2, 3]]), 3);
    }

    #[test]
    fn negative_values() {
        let t = vec![vec![-1], vec![2, 3], vec![1, -1, -3]];
        assert_eq!(Solution::minimum_total(t), -1);
    }

    #[test]
    fn tall_triangle() {
        // Every row is [0, 0, ..., 0, -1]: the path along the right edge collects -1 per row.
        let t: Vec<Vec<i32>> = (1..=200)
            .map(|len| {
                let mut row = vec![0; len];
                row[len - 1] = -1;
                row
            })
            .collect();
        assert_eq!(Solution::minimum_total(t), -200);
    }
}
