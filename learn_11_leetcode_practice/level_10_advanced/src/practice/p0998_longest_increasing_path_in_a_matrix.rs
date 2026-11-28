// ==================================================================
// PRACTICE 0998: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0998_longest_increasing_path_in_a_matrix.md
// Run the tests:          cargo test practice::p0998
// ==================================================================

//! # Problem 0998: Longest Increasing Path in a Matrix
//!
//! - **Difficulty:** Hard
//! - **Topics:** DFS with memoisation on a DAG, or topological "peeling" with a `VecDeque` (used here to avoid deep recursion), out-degrees
//! - **Source:** [LeetCode 329. Longest Increasing Path in a Matrix](https://leetcode.com/problems/longest-increasing-path-in-a-matrix/)
//!
//! ## Problem
//!
//! In an `m x n` integer matrix, a path moves between horizontally or vertically adjacent cells
//! (no diagonals, no wrap-around) and must be **strictly increasing**. Return the length (number of
//! cells) of the longest such path.
//!
//! Implement `Solution::longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! matrix = [[9, 9, 4],
//!           [6, 6, 8],
//!           [2, 1, 1]]     => 4     (1 -> 2 -> 6 -> 9)
//!
//! matrix = [[3, 4, 5],
//!           [3, 2, 6],
//!           [2, 2, 1]]     => 4     (3 -> 4 -> 5 -> 6)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`, `0 <= matrix[i][j] <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Edges go from smaller to larger neighbours, so the cells form a directed **acyclic** graph.
//! 2. The longest path in a DAG can be found by peeling layers: first remove every cell with no
//!    larger neighbour (out-degree 0), then the cells that only pointed at those, and so on.
//! 3. The number of layers is the answer. (A memoised DFS works too, but recursion depth can reach
//!    `40 000` on a snake-shaped matrix.)

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(matrix: &[Vec<i32>]) -> i32 {
        fn longest(m: &[Vec<i32>], r: usize, c: usize) -> i32 {
            let mut best = 1;
            let moves = [
                (r.wrapping_sub(1), c),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r, c + 1),
            ];
            for (nr, nc) in moves {
                if nr < m.len() && nc < m[0].len() && m[nr][nc] > m[r][c] {
                    best = best.max(1 + longest(m, nr, nc));
                }
            }
            best
        }
        (0..matrix.len())
            .flat_map(|r| (0..matrix[0].len()).map(move |c| (r, c)))
            .map(|(r, c)| longest(matrix, r, c))
            .max()
            .unwrap_or(0)
    }

    #[test]
    fn examples() {
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
            4
        );
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
    }

    #[test]
    fn flat_and_line() {
        assert_eq!(Solution::longest_increasing_path(vec![vec![7; 5]; 5]), 1);
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![1, 2, 3, 4, 5]]),
            5
        );
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![i32::MAX], vec![0]]),
            2
        );
    }

    #[test]
    fn matches_exhaustive_search() {
        let matrix: Vec<Vec<i32>> = (0..6)
            .map(|r: i32| (0..7).map(|c: i32| (r * 17 + c * 29) % 13).collect())
            .collect();
        assert_eq!(
            Solution::longest_increasing_path(matrix.clone()),
            naive(&matrix)
        );
    }

    #[test]
    fn snake_through_the_whole_matrix() {
        // Row-wise "boustrophedon" numbering: one increasing path through all 40_000 cells.
        let n = 200;
        let matrix: Vec<Vec<i32>> = (0..n)
            .map(|r| {
                (0..n)
                    .map(|c| {
                        let col = if r % 2 == 0 { c } else { n - 1 - c };
                        r * n + col
                    })
                    .collect()
            })
            .collect();
        assert_eq!(Solution::longest_increasing_path(matrix), 40_000);
    }
}
