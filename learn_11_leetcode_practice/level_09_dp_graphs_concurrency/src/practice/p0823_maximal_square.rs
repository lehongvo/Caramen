// ==================================================================
// PRACTICE 0823: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0823_maximal_square.md
// Run the tests:          cargo test practice::p0823
// ==================================================================

//! # Problem 0823: Maximal Square
//!
//! - **Difficulty:** Medium
//! - **Topics:** grid DP, `Vec<Vec<char>>`, minimum of three neighbours, one-row DP
//! - **Source:** [LeetCode 221. Maximal Square](https://leetcode.com/problems/maximal-square/)
//!
//! ## Problem
//!
//! `matrix` is an `m x n` grid of the characters `'0'` and `'1'`. Find the largest square made only
//! of `'1'` cells and return its **area**. If there is no `'1'`, return `0`.
//!
//! Implement `Solution::maximal_square(matrix: Vec<Vec<char>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  matrix = [['1','0','1','0','0'],
//!                   ['1','0','1','1','1'],
//!                   ['1','1','1','1','1'],
//!                   ['1','0','0','1','0']]
//! Output: 4
//! Explanation: the 2 x 2 square in rows 1-2, columns 3-4 (also columns 2-3).
//!
//! Input:  matrix = [['0','1'],
//!                   ['1','0']]
//! Output: 1
//!
//! Input:  matrix = [['0']]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 300`
//! - `matrix[i][j]` is `'0'` or `'1'`
//!
//! ## Hints
//!
//! 1. Let `side[i][j]` be the side of the largest all-`'1'` square whose **bottom-right** corner is
//!    `(i, j)`.
//! 2. For a `'1'` cell it is `1 + min(top, left, top-left)`; a `'0'` cell gives `0`.
//! 3. Keep one row plus the old top-left value in a variable.

pub struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn example_area_four() {
        let m = grid(&["10100", "10111", "11111", "10010"]);
        assert_eq!(Solution::maximal_square(m), 4);
    }

    #[test]
    fn example_diagonal_ones() {
        assert_eq!(Solution::maximal_square(grid(&["01", "10"])), 1);
    }

    #[test]
    fn example_no_ones() {
        assert_eq!(Solution::maximal_square(grid(&["0"])), 0);
    }

    #[test]
    fn rectangle_of_ones_contains_a_three_square() {
        let m = grid(&["1111", "1111", "0111", "0111"]);
        assert_eq!(Solution::maximal_square(m), 9);
    }

    #[test]
    fn full_large_grid() {
        let m = vec![vec!['1'; 300]; 300];
        assert_eq!(Solution::maximal_square(m), 90_000);
    }
}
