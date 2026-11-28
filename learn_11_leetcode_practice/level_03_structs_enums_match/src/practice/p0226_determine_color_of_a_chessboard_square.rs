// ==================================================================
// PRACTICE 0226: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0226_determine_color_of_a_chessboard_square.md
// Run the tests:          cargo test practice::p0226
// ==================================================================

//! # Problem 0226: Determine Color of a Chessboard Square
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning an `enum` from a helper, slice pattern with `let ... else`, byte arithmetic
//! - **Source:** [LeetCode 1812. Determine Color of a Chessboard Square](https://leetcode.com/problems/determine-color-of-a-chessboard-square/)
//!
//! ## Problem
//!
//! A chessboard square is written as two characters: a file letter `'a'`..`'h'` (the column) and a
//! rank digit `'1'`..`'8'` (the row), for example `"c7"`. Square `"a1"` (bottom-left) is **black**,
//! and colors alternate along every row and column.
//!
//! Given `coordinates`, return `true` if the square is **white** and `false` if it is black.
//!
//! Implement `Solution::square_is_white(coordinates: String) -> bool`.
//!
//! For this level, write a helper that returns an `enum SquareColor { Black, White }` and turn it
//! into the `bool` at the end.
//!
//! ## Examples
//!
//! ```text
//! Input:  coordinates = "a1"
//! Output: false     (black)
//!
//! Input:  coordinates = "h3"
//! Output: true      (white)
//!
//! Input:  coordinates = "c7"
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `coordinates.len() == 2`
//! - `'a' <= coordinates[0] <= 'h'` and `'1' <= coordinates[1] <= '8'`
//!
//! ## Hints
//!
//! 1. Convert the file to `0..8` with `b - b'a'` and the rank with `b - b'1'`.
//! 2. `"a1"` gives `0 + 0 = 0` and is black: the color depends only on whether the sum is even.

pub struct Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a1_is_black() {
        assert!(!Solution::square_is_white("a1".to_string()));
    }

    #[test]
    fn example_h3_is_white() {
        assert!(Solution::square_is_white("h3".to_string()));
    }

    #[test]
    fn example_c7_is_black() {
        assert!(!Solution::square_is_white("c7".to_string()));
    }

    #[test]
    fn corners() {
        assert!(Solution::square_is_white("h1".to_string()));
        assert!(Solution::square_is_white("a8".to_string()));
        assert!(!Solution::square_is_white("h8".to_string()));
    }

    #[test]
    fn board_has_32_white_squares() {
        let mut white = 0;
        for file in 'a'..='h' {
            for rank in '1'..='8' {
                if Solution::square_is_white(format!("{file}{rank}")) {
                    white += 1;
                }
            }
        }
        assert_eq!(white, 32);
    }
}
