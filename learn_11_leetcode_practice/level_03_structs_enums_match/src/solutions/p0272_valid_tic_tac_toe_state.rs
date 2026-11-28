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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** summarise the board as `(x_count - o_count, x_wins, o_wins)` and list the valid
//! combinations in one `matches!` pattern:
//!
//! - X won: X moved last, so the difference is `1` and O must not have won;
//! - O won: O moved last, so the difference is `0`;
//! - nobody won: the difference is `0` or `1`.
//!
//! **Complexity:** `O(1)` time and space.
//!
//! **Rust concepts:**
//! - A `Summary` struct names the four facts, so the final rule reads like the explanation.
//! - `matches!` on the tuple `(diff, x_wins, o_wins)` lists the three valid shapes with literal
//!   `bool`s and `|`; everything else (including "both won") is `false`. A `match` whose arms
//!   only return `true`/`false` is exactly what `matches!` abbreviates (clippy suggests it).
//! - `bytes().filter(|&b| b == b'X').count()` counts marks; `as i32` makes the difference signed.
// @solution-end

pub struct Solution;

// @solution-start remove
const LINES: [[(usize, usize); 3]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

struct Summary {
    x_count: i32,
    o_count: i32,
    x_wins: bool,
    o_wins: bool,
}

fn summarize(board: &[&[u8]]) -> Summary {
    let count = |mark: u8| {
        board
            .iter()
            .map(|row| row.iter().filter(|&&b| b == mark).count())
            .sum::<usize>() as i32
    };
    let wins = |mark: u8| {
        LINES
            .iter()
            .any(|line| line.iter().all(|&(r, c)| board[r][c] == mark))
    };
    Summary {
        x_count: count(b'X'),
        o_count: count(b'O'),
        x_wins: wins(b'X'),
        o_wins: wins(b'O'),
    }
}
// @solution-end

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        // @solution-start
        let rows: Vec<&[u8]> = board.iter().map(|r| r.as_bytes()).collect();
        if rows.len() != 3 || rows.iter().any(|r| r.len() != 3) {
            return false;
        }
        let s = summarize(&rows);
        matches!(
            (s.x_count - s.o_count, s.x_wins, s.o_wins),
            (1, true, false) | (0, false, true) | (0 | 1, false, false)
        )
        // @solution-end
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
