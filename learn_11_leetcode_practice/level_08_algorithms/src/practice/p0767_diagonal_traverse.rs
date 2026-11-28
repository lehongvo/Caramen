// ==================================================================
// PRACTICE 0767: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0767_diagonal_traverse.md
// Run the tests:          cargo test practice::p0767
// ==================================================================

//! # Problem 0767: Diagonal Traverse
//!
//! - **Difficulty:** Medium
//! - **Topics:** index arithmetic on anti-diagonals (`r + c = d`), inclusive ranges, `rev`, `extend`
//! - **Source:** [LeetCode 498. Diagonal Traverse](https://leetcode.com/problems/diagonal-traverse/)
//!
//! ## Problem
//!
//! Given an `m x n` matrix `mat`, return all its elements in **zigzag diagonal order**: start at
//! the top-left cell, walk up-right along the first diagonal, then down-left along the next one,
//! then up-right again, and so on, until the bottom-right cell.
//!
//! Implement `Solution::find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mat = [[1, 2, 3],
//!                [4, 5, 6],
//!                [7, 8, 9]]
//! Output: [1, 2, 4, 7, 5, 3, 6, 8, 9]
//!         (diagonals: [1] up, [2, 4] down, [7, 5, 3] up, [6, 8] down, [9] up)
//!
//! Input:  mat = [[1, 2], [3, 4]]
//! Output: [1, 2, 3, 4]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 10^4` and `1 <= m * n <= 10^4`
//! - `-10^5 <= mat[i][j] <= 10^5`
//!
//! ## Hints
//!
//! 1. All cells of one anti-diagonal have the same `r + c`. There are `m + n - 1` of them.
//! 2. For diagonal `d`, which rows `r` give a valid column `c = d - r`?
//! 3. Even diagonals go up (row decreasing), odd ones go down (row increasing).

pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_by_three() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::find_diagonal_order(mat),
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
        );
    }

    #[test]
    fn example_two_by_two() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn single_row_and_single_column() {
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1, 2, 3]]),
            vec![1, 2, 3]
        );
        assert_eq!(
            Solution::find_diagonal_order(vec![vec![1], vec![2], vec![3]]),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn wide_matrix() {
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(Solution::find_diagonal_order(mat), vec![1, 2, 4, 5, 3, 6]);
    }

    #[test]
    fn tall_matrix() {
        let mat = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(Solution::find_diagonal_order(mat), vec![1, 2, 3, 5, 4, 6]);
    }

    #[test]
    fn large_matrix_visits_every_cell_once() {
        let mat: Vec<Vec<i32>> = (0..100)
            .map(|r| (0..100).map(|c| r * 100 + c).collect())
            .collect();
        let mut order = Solution::find_diagonal_order(mat);
        assert_eq!(&order[..4], &[0, 1, 100, 200]);
        assert_eq!(*order.last().unwrap(), 9_999);
        order.sort_unstable();
        assert_eq!(order, (0..10_000).collect::<Vec<i32>>());
    }
}
