// ==================================================================
// PRACTICE 0221: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0221_robot_return_to_origin.md
// Run the tests:          cargo test practice::p0221
// ==================================================================

//! # Problem 0221: Robot Return to Origin
//!
//! - **Difficulty:** Easy
//! - **Topics:** parsing a `char` into an `enum`, `Option` from a parser, `match`, simulation
//! - **Source:** [LeetCode 657. Robot Return to Origin](https://leetcode.com/problems/robot-return-to-origin/)
//!
//! ## Problem
//!
//! A robot starts at the origin `(0, 0)` of an infinite grid. It receives a string `moves` where
//! every character is one unit step: `'U'` (up), `'D'` (down), `'L'` (left) or `'R'` (right). Return
//! `true` if the robot is back at the origin after performing all the moves in order, `false`
//! otherwise. The robot's facing does not matter; only its position does.
//!
//! Implement `Solution::judge_circle(moves: String) -> bool`.
//!
//! The intended design for this level: turn each character into a small `enum Move` first, then
//! simulate with a `match` on that enum.
//!
//! ## Examples
//!
//! ```text
//! Input:  moves = "UD"
//! Output: true       (up then down)
//!
//! Input:  moves = "LL"
//! Output: false      (ends at (-2, 0))
//!
//! Input:  moves = "RRDDLULU"
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= moves.len() <= 2 * 10^4`
//! - `moves` contains only `'U'`, `'D'`, `'L'` and `'R'`.
//!
//! ## Hints
//!
//! 1. Write `Move::from_char(c: char) -> Option<Move>` and a method `delta(self) -> (i32, i32)`.
//! 2. Sum all the deltas and compare with `(0, 0)`.

pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_up_down() {
        assert!(Solution::judge_circle("UD".to_string()));
    }

    #[test]
    fn example_left_left() {
        assert!(!Solution::judge_circle("LL".to_string()));
    }

    #[test]
    fn example_loop() {
        assert!(Solution::judge_circle("RRDDLULU".to_string()));
    }

    #[test]
    fn single_move_never_returns() {
        for m in ["U", "D", "L", "R"] {
            assert!(!Solution::judge_circle(m.to_string()));
        }
    }

    #[test]
    fn horizontal_balance_is_not_enough() {
        assert!(!Solution::judge_circle("LRU".to_string()));
    }

    #[test]
    fn long_balanced_path() {
        let moves = "UDLR".repeat(5_000);
        assert!(Solution::judge_circle(moves.clone()));
        assert!(!Solution::judge_circle(moves + "U"));
    }
}
