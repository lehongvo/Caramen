// ==================================================================
// PRACTICE 0984: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0984_n_queens.md
// Run the tests:          cargo test practice::p0984
// ==================================================================

//! # Problem 0984: N-Queens
//!
//! - **Difficulty:** Hard
//! - **Topics:** backtracking row by row, column and diagonal occupancy with `Vec<bool>`, building boards as `Vec<String>`
//! - **Source:** [LeetCode 51. N-Queens](https://leetcode.com/problems/n-queens/)
//!
//! ## Problem
//!
//! Place `n` queens on an `n x n` chessboard so that no two attack each other (no two share a row,
//! a column or a diagonal). Return **all** distinct solutions. Each solution is a board: `n` strings
//! of length `n`, with `'Q'` for a queen and `'.'` for an empty square. Solutions may be returned in
//! any order.
//!
//! Implement `Solution::solve_n_queens(n: i32) -> Vec<Vec<String>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4
//! Output: [[".Q..", "...Q", "Q...", "..Q."],
//!          ["..Q.", "Q...", "...Q", ".Q.."]]
//!
//! Input:  n = 1
//! Output: [["Q"]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 9`
//!
//! ## Hints
//!
//! 1. Exactly one queen per row: choose its column row by row.
//! 2. A square `(row, col)` is attacked through its column, its diagonal `row - col` and its
//!    anti-diagonal `row + col`; shift `row - col` by `n - 1` to index a `Vec<bool>`.
//! 3. After placing the last row, turn the chosen columns into strings.

pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(board: &[String]) -> bool {
        let queens: Vec<(i32, i32)> = board
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.char_indices()
                    .filter(|&(_, ch)| ch == 'Q')
                    .map(move |(c, _)| (r as i32, c as i32))
            })
            .collect();
        queens.len() == board.len()
            && queens.iter().enumerate().all(|(i, &(r1, c1))| {
                queens[i + 1..]
                    .iter()
                    .all(|&(r2, c2)| r1 != r2 && c1 != c2 && (r1 - r2).abs() != (c1 - c2).abs())
            })
    }

    #[test]
    fn four_queens() {
        let mut boards = Solution::solve_n_queens(4);
        boards.sort();
        assert_eq!(
            boards,
            vec![
                vec!["..Q.", "Q...", "...Q", ".Q.."],
                vec![".Q..", "...Q", "Q...", "..Q."]
            ]
        );
    }

    #[test]
    fn tiny_boards() {
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"]]);
        assert!(Solution::solve_n_queens(2).is_empty());
        assert!(Solution::solve_n_queens(3).is_empty());
    }

    #[test]
    fn counts_and_validity() {
        for (n, expected) in [(5, 10), (6, 4), (7, 40), (8, 92), (9, 352)] {
            let boards = Solution::solve_n_queens(n);
            assert_eq!(boards.len(), expected, "n = {n}");
            assert!(boards.iter().all(|b| is_valid(b)));
            let mut unique = boards.clone();
            unique.sort();
            unique.dedup();
            assert_eq!(unique.len(), expected);
        }
    }
}
