// ==================================================================
// PRACTICE 0271: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0271_find_winner_on_a_tic_tac_toe_game.md
// Run the tests:          cargo test practice::p0271
// ==================================================================

//! # Problem 0271: Find Winner on a Tic Tac Toe Game
//!
//! - **Difficulty:** Easy
//! - **Topics:** a `Cell` enum grid, a `GameResult` enum, `Option<Player>` cells, checking lines with a `const` table
//! - **Source:** [LeetCode 1275. Find Winner on a Tic Tac Toe Game](https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/)
//!
//! ## Problem
//!
//! Two players, **A** and **B**, play tic-tac-toe on a `3 x 3` grid. A always moves first; they
//! alternate, each move placing the player's mark on an empty cell. The game ends as soon as one
//! player has three marks in a row, column or diagonal, or when all nine cells are filled.
//!
//! `moves[i] = [row, col]` is the `i`-th move (so even `i` are A's moves and odd `i` are B's).
//! All moves are valid and the game stops at the end of `moves`. Return:
//!
//! - `"A"` or `"B"` if that player has won;
//! - `"Draw"` if the grid is full and nobody won;
//! - `"Pending"` otherwise (the game is not finished).
//!
//! Implement `Solution::tictactoe(moves: Vec<Vec<i32>>) -> String`.
//!
//! For this level, model the board as `[[Option<Player>; 3]; 3]` with `enum Player { A, B }`, and
//! the answer as `enum GameResult { Win(Player), Draw, Pending }`.
//!
//! ## Examples
//!
//! ```text
//! Input:  moves = [[0,0],[2,0],[1,1],[2,1],[2,2]]
//! Output: "A"        (A holds the main diagonal)
//!
//! Input:  moves = [[0,0],[1,1],[0,1],[0,2],[1,0],[2,0]]
//! Output: "B"        (B holds the anti-diagonal)
//!
//! Input:  moves = [[0,0],[1,1],[2,0],[1,0],[1,2],[2,1],[0,1],[0,2],[2,2]]
//! Output: "Draw"
//!
//! Input:  moves = [[0,0],[1,1]]
//! Output: "Pending"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= moves.len() <= 9`, `moves[i].len() == 2`, `0 <= row, col <= 2`
//! - All moves are on distinct cells and follow the rules.
//!
//! ## Hints
//!
//! 1. A `const LINES: [[(usize, usize); 3]; 8]` table lists the 3 rows, 3 columns and 2 diagonals.
//! 2. A line is won by `p` when its three cells are all `Some(p)`.

pub struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(moves: &[[i32; 2]]) -> String {
        Solution::tictactoe(moves.iter().map(|m| m.to_vec()).collect())
    }

    #[test]
    fn example_a_wins_diagonal() {
        assert_eq!(play(&[[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]]), "A");
    }

    #[test]
    fn example_b_wins_anti_diagonal() {
        assert_eq!(play(&[[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]]), "B");
    }

    #[test]
    fn example_draw() {
        assert_eq!(
            play(&[
                [0, 0],
                [1, 1],
                [2, 0],
                [1, 0],
                [1, 2],
                [2, 1],
                [0, 1],
                [0, 2],
                [2, 2]
            ]),
            "Draw"
        );
    }

    #[test]
    fn example_pending() {
        assert_eq!(play(&[[0, 0], [1, 1]]), "Pending");
        assert_eq!(play(&[[1, 1]]), "Pending");
    }

    #[test]
    fn rows_and_columns() {
        assert_eq!(play(&[[0, 0], [1, 0], [0, 1], [1, 1], [0, 2]]), "A");
        assert_eq!(play(&[[0, 0], [0, 2], [1, 0], [1, 2], [2, 1], [2, 2]]), "B");
    }

    #[test]
    fn full_boards() {
        // A B A / A B B / B A A: nobody has a line.
        assert_eq!(
            play(&[
                [0, 0],
                [0, 1],
                [0, 2],
                [1, 1],
                [1, 0],
                [1, 2],
                [2, 1],
                [2, 0],
                [2, 2]
            ]),
            "Draw"
        );
        // A completes the main diagonal with the ninth move: a win, not a draw.
        assert_eq!(
            play(&[
                [0, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [1, 0],
                [2, 0],
                [2, 1],
                [2, 2]
            ]),
            "A"
        );
    }
}
