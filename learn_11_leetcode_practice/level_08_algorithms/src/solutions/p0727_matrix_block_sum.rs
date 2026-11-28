//! # Problem 0727: Matrix Block Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** 2-D prefix sums, inclusion-exclusion, `saturating_sub`, nested `map`/`collect`
//! - **Source:** [LeetCode 1314. Matrix Block Sum](https://leetcode.com/problems/matrix-block-sum/)
//!
//! ## Problem
//!
//! You are given an `m x n` matrix `mat` and an integer `k`. Return an `m x n` matrix `answer`
//! where `answer[i][j]` is the sum of every `mat[r][c]` with `i - k <= r <= i + k` and
//! `j - k <= c <= j + k` that lies **inside** the matrix (cells outside are ignored).
//!
//! Implement `Solution::matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mat = [[1, 2, 3],
//!                [4, 5, 6],
//!                [7, 8, 9]], k = 1
//! Output: [[12, 21, 16],
//!          [27, 45, 33],
//!          [24, 39, 28]]
//!         (answer[0][0] = 1 + 2 + 4 + 5 = 12; answer[1][1] sums the whole matrix)
//!
//! Input:  same mat, k = 2
//! Output: [[45, 45, 45], [45, 45, 45], [45, 45, 45]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n, k <= 100`
//! - `1 <= mat[i][j] <= 100`
//!
//! ## Hints
//!
//! 1. Summing each block directly costs `O(k^2)` per cell. Precompute something instead.
//! 2. Let `pre[r][c]` be the sum of the rectangle of rows `0..r` and columns `0..c`. Any rectangle
//!    sum is then a combination of four `pre` values.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** 2-D prefix sums: build `pre` with one extra row and column of zeros, where
//! `pre[r + 1][c + 1] = mat[r][c] + pre[r][c + 1] + pre[r + 1][c] - pre[r][c]` (the top and left
//! rectangles overlap in `pre[r][c]`, which is counted twice and subtracted once). The sum of rows
//! `r0..r1` and columns `c0..c1` (half-open) is then
//! `pre[r1][c1] - pre[r0][c1] - pre[r1][c0] + pre[r0][c0]` by inclusion-exclusion. For each cell,
//! clamp the block to the matrix: `r0 = i - k` (at least 0) and `r1 = i + k + 1` (at most `m`).
//!
//! **Complexity:** `O(m * n)` time and space.
//!
//! **Rust concepts:**
//! - `i.saturating_sub(k)` stops at `0` instead of underflowing a `usize`; `.min(m)` clamps the
//!   other side.
//! - The result is built with nested `(0..m).map(|i| (0..n).map(|j| ...).collect()).collect()`;
//!   the outer `collect` produces `Vec<Vec<i32>>` from the return type.
//! - `for (r, row) in mat.iter().enumerate()` avoids indexing `mat` by hand.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // @solution-start
        let (m, n, k) = (mat.len(), mat[0].len(), k as usize);
        let mut pre = vec![vec![0; n + 1]; m + 1];
        for (r, row) in mat.iter().enumerate() {
            for (c, &value) in row.iter().enumerate() {
                pre[r + 1][c + 1] = value + pre[r][c + 1] + pre[r + 1][c] - pre[r][c];
            }
        }
        (0..m)
            .map(|i| {
                let (r0, r1) = (i.saturating_sub(k), (i + k + 1).min(m));
                (0..n)
                    .map(|j| {
                        let (c0, c1) = (j.saturating_sub(k), (j + k + 1).min(n));
                        pre[r1][c1] - pre[r0][c1] - pre[r1][c0] + pre[r0][c0]
                    })
                    .collect()
            })
            .collect()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_by_three() -> Vec<Vec<i32>> {
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
    }

    #[test]
    fn example_k_one() {
        assert_eq!(
            Solution::matrix_block_sum(three_by_three(), 1),
            vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]]
        );
    }

    #[test]
    fn example_k_two_covers_everything() {
        assert_eq!(
            Solution::matrix_block_sum(three_by_three(), 2),
            vec![vec![45; 3]; 3]
        );
    }

    #[test]
    fn single_cell() {
        assert_eq!(Solution::matrix_block_sum(vec![vec![5]], 1), vec![vec![5]]);
    }

    #[test]
    fn single_row() {
        assert_eq!(
            Solution::matrix_block_sum(vec![vec![1, 2, 3, 4]], 1),
            vec![vec![3, 6, 9, 7]]
        );
    }

    #[test]
    fn rectangular_matrix() {
        let mat = vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1]];
        assert_eq!(
            Solution::matrix_block_sum(mat, 1),
            vec![vec![4, 6, 6, 4], vec![4, 6, 6, 4]]
        );
    }

    #[test]
    fn largest_input() {
        let answer = Solution::matrix_block_sum(vec![vec![100; 100]; 100], 100);
        assert!(answer.iter().flatten().all(|&s| s == 1_000_000));
    }
}
