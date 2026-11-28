// ==================================================================
// PRACTICE 0367: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0367_the_k_weakest_rows_in_a_matrix.md
// Run the tests:          cargo test practice::p0367
// ==================================================================

//! # Problem 0367: The K Weakest Rows in a Matrix
//!
//! - **Difficulty:** Easy
//! - **Topics:** a max-heap bounded to `k` elements, `(strength, index)` tuples, `into_sorted_vec`, `partition_point`
//! - **Source:** [LeetCode 1337. The K Weakest Rows in a Matrix](https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/)
//!
//! ## Problem
//!
//! You are given an `m x n` matrix `mat` of `1`s (soldiers) and `0`s (civilians). In every row, all
//! soldiers stand **before** all civilians (each row looks like `1, 1, ..., 0, 0`).
//!
//! Row `i` is **weaker** than row `j` if it has fewer soldiers, or the same number of soldiers and
//! `i < j`. Return the indices of the `k` weakest rows, from the weakest to the strongest.
//!
//! Implement `Solution::k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mat = [[1, 1, 0, 0, 0],
//!                [1, 1, 1, 1, 0],
//!                [1, 0, 0, 0, 0],
//!                [1, 1, 0, 0, 0],
//!                [1, 1, 1, 1, 1]],  k = 3
//! Output: [2, 0, 3]
//! Explanation: soldiers per row are 2, 4, 1, 2, 5; the weakest are row 2 (1), row 0 (2), row 3 (2).
//!
//! Input:  mat = [[1, 0, 0, 0], [1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0]],  k = 2
//! Output: [0, 2]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= m, n <= 100`
//! - `1 <= k <= m`
//! - `mat[i][j]` is `0` or `1`, with the soldiers first in each row.
//!
//! ## Hints
//!
//! 1. Describe each row by the pair `(soldiers, index)`; comparing those pairs is exactly the
//!    "weaker" rule.
//! 2. Keep a **max**-heap of at most `k` pairs: when it grows to `k + 1`, pop the strongest.
//! 3. Because soldiers come first, `row.partition_point(|&x| x == 1)` counts them by binary search.

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::k_weakest_rows(mat, 3), vec![2, 0, 3]);
    }

    #[test]
    fn example_two() {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        assert_eq!(Solution::k_weakest_rows(mat, 2), vec![0, 2]);
    }

    #[test]
    fn all_rows_and_empty_rows() {
        let mat = vec![vec![0, 0], vec![1, 1], vec![0, 0], vec![1, 0]];
        assert_eq!(Solution::k_weakest_rows(mat, 4), vec![0, 2, 3, 1]);
    }

    #[test]
    fn hundred_rows() {
        // Row i has (100 - i) soldiers, so the weakest rows are the last ones.
        let mat: Vec<Vec<i32>> = (0..100)
            .map(|i| (0..100).map(|j| if j < 100 - i { 1 } else { 0 }).collect())
            .collect();
        assert_eq!(Solution::k_weakest_rows(mat, 3), vec![99, 98, 97]);
    }
}
