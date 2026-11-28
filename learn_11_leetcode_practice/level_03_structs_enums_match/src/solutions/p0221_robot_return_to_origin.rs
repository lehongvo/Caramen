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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** parse every character into a `Move`, map each move to its `(dx, dy)` step and add up
//! the steps. The robot is home exactly when both sums are zero.
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - Separating *parsing* (`char -> Option<Move>`) from *logic* (`match` on `Move`) keeps each
//!   `match` small, and the logic `match` is exhaustive without a `_` arm.
//! - `filter_map(Move::from_char)` keeps the `Some` values and drops the `None`s.
//! - `fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy))` destructures tuples directly in the
//!   closure parameters.
// @solution-end

pub struct Solution;

// @solution-start remove
/// One unit step of the robot.
#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_char(c: char) -> Option<Move> {
        match c {
            'U' => Some(Move::Up),
            'D' => Some(Move::Down),
            'L' => Some(Move::Left),
            'R' => Some(Move::Right),
            _ => None,
        }
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Move::Up => (0, 1),
            Move::Down => (0, -1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        }
    }
}
// @solution-end

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        // @solution-start
        let end = moves
            .chars()
            .filter_map(Move::from_char)
            .map(Move::delta)
            .fold((0, 0), |(x, y), (dx, dy)| (x + dx, y + dy));
        end == (0, 0)
        // @solution-end
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
