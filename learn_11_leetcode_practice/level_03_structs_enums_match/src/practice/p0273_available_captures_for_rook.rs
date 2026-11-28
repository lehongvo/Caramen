// ==================================================================
// PRACTICE 0273: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0273_available_captures_for_rook.md
// Run the tests:          cargo test practice::p0273
// ==================================================================

//! # Problem 0273: Available Captures for Rook
//!
//! - **Difficulty:** Easy
//! - **Topics:** a `Piece` enum parsed from `char`, walking a grid in four directions, `match` deciding when to stop, `find_map`
//! - **Source:** [LeetCode 999. Available Captures for Rook](https://leetcode.com/problems/available-captures-for-rook/)
//!
//! ## Problem
//!
//! An `8 x 8` chessboard contains exactly one white rook `'R'`, some white bishops `'B'`, some
//! black pawns `'p'`, and empty squares `'.'`. The rook moves any number of squares horizontally
//! or vertically, but cannot jump over pieces. It **attacks** a pawn if it can reach the pawn's
//! square in one move (a bishop, or another pawn, in the way blocks it).
//!
//! Return the number of pawns the rook attacks (at most one per direction, so at most 4).
//!
//! Implement `Solution::num_rook_captures(board: Vec<Vec<char>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  board = [". . . . . . . .",
//!                  ". . . p . . . .",
//!                  ". . . R . . . p",
//!                  ". . . . . . . .",
//!                  ". . . . . . . .",
//!                  ". . . p . . . .",
//!                  ". . . . . . . .",
//!                  ". . . . . . . ."]     (spaces added for readability)
//! Output: 3        (up, right and down)
//!
//! Input:  board = [". . . . . . . .",
//!                  ". p p p p p . .",
//!                  ". p p B p p . .",
//!                  ". p B R B p . .",
//!                  ". p p B p p . .",
//!                  ". p p p p p . .",
//!                  ". . . . . . . .",
//!                  ". . . . . . . ."]
//! Output: 0        (bishops block all four directions)
//! ```
//!
//! ## Constraints
//!
//! - `board.len() == 8`, `board[i].len() == 8`, exactly one `'R'`.
//!
//! ## Hints
//!
//! 1. Find the rook first: `find_map` over the rows with `position`.
//! 2. Walk each of the four directions until the edge or a piece; a `match` on the piece says
//!    whether it counts (`Pawn`), blocks (`Bishop`), or lets you continue (`Empty`).

pub struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board(rows: [&str; 8]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn example_three_captures() {
        let b = board([
            "........", "...p....", "...R...p", "........", "........", "...p....", "........",
            "........",
        ]);
        assert_eq!(Solution::num_rook_captures(b), 3);
    }

    #[test]
    fn example_blocked_by_bishops() {
        let b = board([
            "........", ".ppppp..", ".ppBpp..", ".pBRBp..", ".ppBpp..", ".ppppp..", "........",
            "........",
        ]);
        assert_eq!(Solution::num_rook_captures(b), 0);
    }

    #[test]
    fn pawn_blocks_pawn_behind_it() {
        let b = board([
            "........", "...p....", "...p....", "pp.R.B.p", "........", "...B....", "...p....",
            "........",
        ]);
        // Up: the nearer pawn counts. Left: pawn at column 1 counts. Right: bishop blocks. Down: bishop blocks.
        assert_eq!(Solution::num_rook_captures(b), 2);
    }

    #[test]
    fn rook_in_the_corner() {
        let b = board([
            "R......p", "........", "........", "........", "........", "........", "........",
            "p.......",
        ]);
        assert_eq!(Solution::num_rook_captures(b), 2);
    }

    #[test]
    fn four_captures() {
        let b = board([
            "...p....", "........", "........", "p..R...p", "........", "........", "........",
            "...p....",
        ]);
        assert_eq!(Solution::num_rook_captures(b), 4);
    }
}
