// ==================================================================
// PRACTICE 0272: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0272_valid_tic_tac_toe_state.md
// Run the tests:          cargo test practice::p0272
// ==================================================================

//! # Problem 0272: Valid Tic-Tac-Toe State
//!
//! - **Difficulty:** Medium
//! - **Topics:** summarising a board into a struct, `match` on a tuple of counts and `bool`s, guards, `matches!`
//! - **Source:** [LeetCode 794. Valid Tic-Tac-Toe State](https://leetcode.com/problems/valid-tic-tac-toe-state/)
//!
//! ## Problem
//!
//! You are given a tic-tac-toe board as three strings of three characters each: `'X'`, `'O'` or
//! `' '` (empty). Return `true` if this position can be reached in a real game, where:
//!
//! - players alternate and **X moves first**;
//! - marks are only placed on empty cells;
//! - the game stops (no more moves) as soon as a player has three in a row, column or diagonal,
//!   or the board is full.
//!
//! Implement `Solution::valid_tic_tac_toe(board: Vec<String>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  board = ["O  ", "   ", "   "]
//! Output: false     (O cannot move first)
//!
//! Input:  board = ["XOX", " X ", "   "]
//! Output: false     (X has 3 marks, O only 1: players must alternate)
//!
//! Input:  board = ["XOX", "O O", "XOX"]
//! Output: true
//!
//! Input:  board = ["XXX", "   ", "OOO"]
//! Output: false     (both players cannot have won)
//! ```
//!
//! ## Constraints
//!
//! - `board.len() == 3`, each string has length 3, characters are `'X'`, `'O'` or `' '`.
//!
//! ## Hints
//!
//! 1. Only four facts matter: how many `X`, how many `O`, whether X has a line, whether O has a line.
//! 2. If X has won, X made the last move, so `x == o + 1`. If O has won, `x == o`.

pub struct Solution;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid(rows: [&str; 3]) -> bool {
        Solution::valid_tic_tac_toe(rows.iter().map(|r| r.to_string()).collect())
    }

    #[test]
    fn example_o_first() {
        assert!(!valid(["O  ", "   ", "   "]));
    }

    #[test]
    fn example_not_alternating() {
        assert!(!valid(["XOX", " X ", "   "]));
    }

    #[test]
    fn example_valid_full_board() {
        assert!(valid(["XOX", "O O", "XOX"]));
    }

    #[test]
    fn example_both_won() {
        assert!(!valid(["XXX", "   ", "OOO"]));
    }

    #[test]
    fn winner_must_have_moved_last() {
        // X won, but O moved after that.
        assert!(!valid(["XXX", "OO ", "O  "]));
        assert!(valid(["XXX", "OO ", "   "]));
        // O won, but X moved after that.
        assert!(!valid(["OOO", "XX ", "XX "]));
        assert!(valid(["OOO", "XX ", "X  "]));
    }

    #[test]
    fn empty_and_double_win_by_one_move() {
        assert!(valid(["   ", "   ", "   "]));
        // X completes both diagonals with its last mark (the centre): still valid.
        assert!(valid(["XOX", "OXO", "XOX"]));
        // Too many O marks.
        assert!(!valid(["OO ", "X  ", "   "]));
    }
}
