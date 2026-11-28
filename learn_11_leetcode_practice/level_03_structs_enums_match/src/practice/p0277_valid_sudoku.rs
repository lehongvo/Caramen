// ==================================================================
// PRACTICE 0277: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0277_valid_sudoku.md
// Run the tests:          cargo test practice::p0277
// ==================================================================

//! # Problem 0277: Valid Sudoku
//!
//! - **Difficulty:** Medium
//! - **Topics:** `match` on `char` with a range pattern and a binding, a `Seen` struct of bit masks, box index arithmetic
//! - **Source:** [LeetCode 36. Valid Sudoku](https://leetcode.com/problems/valid-sudoku/)
//!
//! ## Problem
//!
//! A `9 x 9` Sudoku board is partially filled: each cell is a digit `'1'`..`'9'` or `'.'` (empty).
//! Determine whether the **filled** cells are consistent with the rules:
//!
//! 1. each row contains each digit at most once;
//! 2. each column contains each digit at most once;
//! 3. each of the nine `3 x 3` boxes contains each digit at most once.
//!
//! The board does not need to be solvable; only the filled cells are checked.
//!
//! Implement `Solution::is_valid_sudoku(board: Vec<Vec<char>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  the classic example board
//!         5 3 . | . 7 . | . . .
//!         6 . . | 1 9 5 | . . .
//!         . 9 8 | . . . | . 6 .
//!         ------+-------+------
//!         8 . . | . 6 . | . . 3
//!         4 . . | 8 . 3 | . . 1
//!         7 . . | . 2 . | . . 6
//!         ------+-------+------
//!         . 6 . | . . . | 2 8 .
//!         . . . | 4 1 9 | . . 5
//!         . . . | . 8 . | . 7 9
//! Output: true
//!
//! Input:  the same board with the top-left 5 replaced by 8
//! Output: false     (two 8s in the first column and in the top-left box)
//! ```
//!
//! ## Constraints
//!
//! - `board.len() == 9`, `board[i].len() == 9`, each cell is `'1'`..`'9'` or `'.'`.
//!
//! ## Hints
//!
//! 1. The box of cell `(r, c)` has index `(r / 3) * 3 + c / 3`.
//! 2. `match ch { d @ '1'..='9' => ..., _ => continue }` filters and binds the digit in one arm.
//! 3. A `u16` bit mask per row, column and box is enough to remember which digits were seen.

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board(rows: [&str; 9]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    fn classic() -> [&'static str; 9] {
        [
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]
    }

    #[test]
    fn example_valid() {
        assert!(Solution::is_valid_sudoku(board(classic())));
    }

    #[test]
    fn example_duplicate_in_column_and_box() {
        let mut rows = classic();
        rows[0] = "83..7....";
        assert!(!Solution::is_valid_sudoku(board(rows)));
    }

    #[test]
    fn duplicate_in_row_only() {
        let mut rows = classic();
        rows[0] = "53..7...5";
        assert!(!Solution::is_valid_sudoku(board(rows)));
    }

    #[test]
    fn duplicate_in_box_only() {
        let mut rows = [
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
        ];
        rows[3] = "...1.....";
        rows[5] = ".....1...";
        assert!(!Solution::is_valid_sudoku(board(rows)));
        rows[5] = "......1..";
        assert!(Solution::is_valid_sudoku(board(rows)));
    }

    #[test]
    fn empty_and_full_solution() {
        assert!(Solution::is_valid_sudoku(vec![vec!['.'; 9]; 9]));
        let solved = [
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ];
        assert!(Solution::is_valid_sudoku(board(solved)));
    }
}
