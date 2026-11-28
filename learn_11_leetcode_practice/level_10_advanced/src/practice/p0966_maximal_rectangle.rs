// ==================================================================
// PRACTICE 0966: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0966_maximal_rectangle.md
// Run the tests:          cargo test practice::p0966
// ==================================================================

//! # Problem 0966: Maximal Rectangle
//!
//! - **Difficulty:** Hard
//! - **Topics:** reducing a 2D problem to row histograms, monotonic stack, `Vec<Vec<char>>` input, reusing a helper
//! - **Source:** [LeetCode 85. Maximal Rectangle](https://leetcode.com/problems/maximal-rectangle/)
//!
//! ## Problem
//!
//! Given a `rows x cols` binary matrix of `'0'` and `'1'` characters, return the area of the largest
//! rectangle that contains only `'1'`s.
//!
//! Implement `Solution::maximal_rectangle(matrix: Vec<Vec<char>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  matrix = ["10100",
//!                   "10111",
//!                   "11111",
//!                   "10010"]
//! Output: 6          (rows 1..=2, columns 2..=4)
//!
//! Input:  matrix = ["0"]   => 0
//! Input:  matrix = ["1"]   => 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= rows, cols <= 200`
//! - `matrix[i][j]` is `'0'` or `'1'`.
//!
//! ## Hints
//!
//! 1. For each row, let `heights[c]` be the number of consecutive `'1'`s ending at this row in
//!    column `c` (reset to 0 on a `'0'`).
//! 2. Every all-ones rectangle has its bottom edge on some row; in that row's histogram it is a
//!    rectangle under the bars.
//! 3. Run "Largest Rectangle in Histogram" (problem 0965) on every row's heights.

pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    fn naive(m: &[Vec<char>]) -> i32 {
        let (rows, cols) = (m.len(), m[0].len());
        let mut best = 0;
        for r1 in 0..rows {
            for c1 in 0..cols {
                for r2 in r1..rows {
                    for c2 in c1..cols {
                        let all_ones = (r1..=r2).all(|r| (c1..=c2).all(|c| m[r][c] == '1'));
                        if all_ones {
                            best = best.max(((r2 - r1 + 1) * (c2 - c1 + 1)) as i32);
                        }
                    }
                }
            }
        }
        best
    }

    #[test]
    fn examples() {
        let m = grid(&["10100", "10111", "11111", "10010"]);
        assert_eq!(Solution::maximal_rectangle(m), 6);
        assert_eq!(Solution::maximal_rectangle(grid(&["0"])), 0);
        assert_eq!(Solution::maximal_rectangle(grid(&["1"])), 1);
    }

    #[test]
    fn single_row_and_column() {
        assert_eq!(Solution::maximal_rectangle(grid(&["0111011110"])), 4);
        assert_eq!(Solution::maximal_rectangle(grid(&["1", "1", "0", "1"])), 2);
    }

    #[test]
    fn matches_naive() {
        let rows: Vec<String> = (0..8)
            .map(|r| {
                (0..9)
                    .map(|c| if (r * 7 + c * 5) % 4 == 0 { '0' } else { '1' })
                    .collect()
            })
            .collect();
        let refs: Vec<&str> = rows.iter().map(String::as_str).collect();
        let m = grid(&refs);
        assert_eq!(Solution::maximal_rectangle(m.clone()), naive(&m));
    }

    #[test]
    fn full_matrix() {
        let m = vec![vec!['1'; 200]; 200];
        assert_eq!(Solution::maximal_rectangle(m), 40_000);
    }
}
