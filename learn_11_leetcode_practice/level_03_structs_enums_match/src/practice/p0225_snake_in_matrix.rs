// ==================================================================
// PRACTICE 0225: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0225_snake_in_matrix.md
// Run the tests:          cargo test practice::p0225
// ==================================================================

//! # Problem 0225: Snake in Matrix
//!
//! - **Difficulty:** Easy
//! - **Topics:** command `enum`, position `struct`, methods with `&mut self`, `match` on `&str`
//! - **Source:** [LeetCode 3248. Snake in Matrix](https://leetcode.com/problems/snake-in-matrix/)
//!
//! ## Problem
//!
//! An `n x n` grid numbers its cells row by row: the cell in row `i` and column `j` has the number
//! `i * n + j`. A snake starts in cell `0` (top-left corner) and follows a list of `commands`, each
//! one of `"UP"`, `"RIGHT"`, `"DOWN"` or `"LEFT"`, moving one cell per command. The commands are
//! guaranteed never to take the snake outside the grid. Return the number of the cell where the
//! snake ends.
//!
//! Implement `Solution::final_position_of_snake(n: i32, commands: Vec<String>) -> i32`.
//!
//! For this level, parse the commands into an `enum Command` and keep the snake's coordinates in a
//! small `struct Position { row, col }` with a `step(&mut self, cmd: Command)` method.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 2, commands = ["RIGHT", "DOWN"]
//! Output: 3      (0 -> 1 -> 3)
//!
//! Input:  n = 3, commands = ["DOWN", "RIGHT", "UP"]
//! Output: 1      ((0,0) -> (1,0) -> (1,1) -> (0,1), and 0 * 3 + 1 = 1)
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 10`
//! - `1 <= commands.len() <= 100`
//! - Every command is one of the four words, and the snake never leaves the grid.
//!
//! ## Hints
//!
//! 1. "Up" means row - 1 and "left" means column - 1.
//! 2. Convert the final `(row, col)` back with `row * n + col`.

pub struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmds(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::final_position_of_snake(2, cmds(&["RIGHT", "DOWN"])),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::final_position_of_snake(3, cmds(&["DOWN", "RIGHT", "UP"])),
            1
        );
    }

    #[test]
    fn back_and_forth_returns_to_zero() {
        assert_eq!(
            Solution::final_position_of_snake(4, cmds(&["RIGHT", "LEFT", "DOWN", "UP"])),
            0
        );
    }

    #[test]
    fn reaches_bottom_right_corner() {
        let mut list = vec!["DOWN"; 9];
        list.extend(vec!["RIGHT"; 9]);
        assert_eq!(Solution::final_position_of_snake(10, cmds(&list)), 99);
    }

    #[test]
    fn hundred_commands_zigzag() {
        let list: Vec<&str> = (0..100)
            .map(|i| if i % 2 == 0 { "RIGHT" } else { "LEFT" })
            .collect();
        assert_eq!(Solution::final_position_of_snake(5, cmds(&list)), 0);
    }
}
