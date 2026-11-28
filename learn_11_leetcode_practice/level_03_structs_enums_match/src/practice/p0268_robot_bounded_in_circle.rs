// ==================================================================
// PRACTICE 0268: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0268_robot_bounded_in_circle.md
// Run the tests:          cargo test practice::p0268
// ==================================================================

//! # Problem 0268: Robot Bounded In Circle
//!
//! - **Difficulty:** Medium
//! - **Topics:** a heading `enum` with turn methods, a robot `struct` with `&mut self` methods, simulation, reasoning about periodicity
//! - **Source:** [LeetCode 1041. Robot Bounded In Circle](https://leetcode.com/problems/robot-bounded-in-circle/)
//!
//! ## Problem
//!
//! A robot stands at `(0, 0)` on an infinite plane, facing **north**. It understands three
//! instructions:
//!
//! - `'G'`: go straight one unit;
//! - `'L'`: turn 90 degrees to the left (counter-clockwise);
//! - `'R'`: turn 90 degrees to the right (clockwise).
//!
//! The robot executes the string `instructions` in order and then **repeats it forever**. Return
//! `true` if the robot stays within some circle around the origin forever (it never wanders off to
//! infinity).
//!
//! Implement `Solution::is_robot_bounded(instructions: String) -> bool`.
//!
//! For this level, write a `Heading` enum (`North`, `East`, `South`, `West`) with `left`/`right`
//! methods and a `Robot` struct with a `step(&mut self, instruction: char)` method.
//!
//! ## Examples
//!
//! ```text
//! Input:  instructions = "GGLLGG"
//! Output: true     (it walks to (0, 2), turns around, walks back to (0, 0), and repeats)
//!
//! Input:  instructions = "GG"
//! Output: false    (it keeps going north)
//!
//! Input:  instructions = "GL"
//! Output: true     (after 4 repetitions it is back at the origin facing north)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= instructions.len() <= 100`
//! - `instructions` contains only `'G'`, `'L'` and `'R'`.
//!
//! ## Hints
//!
//! 1. Simulate one pass only.
//! 2. If the robot is back at the origin after one pass, it is clearly bounded. If it is somewhere
//!    else but **not facing north**, then after 2 or 4 passes the displacements cancel out.

pub struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounded(s: &str) -> bool {
        Solution::is_robot_bounded(s.to_string())
    }

    #[test]
    fn example_back_and_forth() {
        assert!(bounded("GGLLGG"));
    }

    #[test]
    fn example_straight_line() {
        assert!(!bounded("GG"));
    }

    #[test]
    fn example_square() {
        assert!(bounded("GL"));
    }

    #[test]
    fn turning_in_place() {
        assert!(bounded("L"));
        assert!(bounded("RRRR"));
        assert!(!bounded("GLRG"));
    }

    #[test]
    fn long_instructions() {
        // 99 steps north, then a right turn: drifts in a square pattern, so bounded.
        let s = "G".repeat(99) + "R";
        assert!(bounded(&s));
        // A full turn brings the heading back north after moving: unbounded.
        let t = "G".repeat(96) + "LLLL";
        assert!(!bounded(&t));
    }
}
