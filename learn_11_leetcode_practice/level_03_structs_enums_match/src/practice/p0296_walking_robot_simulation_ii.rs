// ==================================================================
// PRACTICE 0296: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0296_walking_robot_simulation_ii.md
// Run the tests:          cargo test practice::p0296
// ==================================================================

//! # Problem 0296: Walking Robot Simulation II
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, a `Facing` enum with `name()`, reducing a long walk with `%`, mapping a perimeter index to coordinates with `match` guards
//! - **Source:** [LeetCode 2069. Walking Robot Simulation II](https://leetcode.com/problems/walking-robot-simulation-ii/)
//!
//! ## Problem
//!
//! A robot lives on a `width x height` grid whose bottom-left cell is `(0, 0)` and top-right cell is
//! `(width - 1, height - 1)`. It starts at `(0, 0)` facing **East**. To take one step it tries to
//! move one cell forward; if that cell is outside the grid, it first turns 90 degrees
//! **counter-clockwise** and tries again. As a result it only ever walks around the border.
//!
//! Implement the `Robot` struct:
//!
//! - `Robot::new(width: i32, height: i32) -> Self`;
//! - `step(&mut self, num: i32)`: take `num` steps;
//! - `get_pos(&self) -> Vec<i32>`: the current cell as `[x, y]`;
//! - `get_dir(&self) -> String`: `"East"`, `"North"`, `"West"` or `"South"`.
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`.
//!
//! ## Examples
//!
//! ```text
//! let mut r = Robot::new(6, 3);
//! r.step(2);  r.step(2);          // (4, 0) facing East
//! r.get_pos() => [4, 0]
//! r.get_dir() => "East"
//! r.step(2);                      // (5, 0), then turn North, (5, 1)
//! r.step(1);                      // (5, 2)
//! r.step(4);                      // turn West, (4, 2) ... (1, 2)
//! r.get_pos() => [1, 2]
//! r.get_dir() => "West"
//! ```
//!
//! ## Constraints
//!
//! - `2 <= width, height <= 100`, `1 <= num <= 10^5`; at most `10^4` calls in total.
//!
//! ## Hints
//!
//! 1. The border is a loop of `p = 2 * (width - 1) + 2 * (height - 1)` cells. Track the robot by its
//!    index along that loop, `0` being `(0, 0)`; stepping is `(index + num) % p`.
//! 2. Each side of the loop maps to coordinates and a facing with simple arithmetic.
//! 3. Careful: back at index `0` **after having moved**, the robot arrived from above, so it faces
//!    **South**, not East.

/// A robot walking around the border of a grid.
pub struct Robot {}

impl Robot {
    /// A robot at `(0, 0)` facing East on a `width x height` grid.
    pub fn new(width: i32, height: i32) -> Self {
        todo!("Write your solution here")
    }

    /// Takes `num` steps.
    pub fn step(&mut self, num: i32) {
        todo!("Write your solution here")
    }

    /// The current cell `[x, y]`.
    pub fn get_pos(&self) -> Vec<i32> {
        todo!("Write your solution here")
    }

    /// The current facing as text.
    pub fn get_dir(&self) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut r = Robot::new(6, 3);
        r.step(2);
        r.step(2);
        assert_eq!(r.get_pos(), vec![4, 0]);
        assert_eq!(r.get_dir(), "East");
        r.step(2);
        r.step(1);
        r.step(4);
        assert_eq!(r.get_pos(), vec![1, 2]);
        assert_eq!(r.get_dir(), "West");
    }

    #[test]
    fn starts_east_and_ends_south_after_full_loop() {
        let mut r = Robot::new(3, 3);
        assert_eq!(r.get_pos(), vec![0, 0]);
        assert_eq!(r.get_dir(), "East");
        r.step(8);
        assert_eq!(r.get_pos(), vec![0, 0]);
        assert_eq!(r.get_dir(), "South");
    }

    #[test]
    fn corners_keep_the_incoming_direction() {
        let mut r = Robot::new(3, 3);
        r.step(2);
        assert_eq!((r.get_pos(), r.get_dir()), (vec![2, 0], "East".to_string()));
        r.step(2);
        assert_eq!(
            (r.get_pos(), r.get_dir()),
            (vec![2, 2], "North".to_string())
        );
        r.step(2);
        assert_eq!((r.get_pos(), r.get_dir()), (vec![0, 2], "West".to_string()));
        r.step(1);
        assert_eq!(
            (r.get_pos(), r.get_dir()),
            (vec![0, 1], "South".to_string())
        );
    }

    #[test]
    fn huge_steps() {
        let mut r = Robot::new(100, 100);
        // The perimeter has 396 cells; 100_000 % 396 = 208.
        r.step(100_000);
        // 208 = 99 (bottom) + 99 (right) + 10 along the top, moving West.
        assert_eq!(r.get_pos(), vec![89, 99]);
        assert_eq!(r.get_dir(), "West");
        for _ in 0..9_999 {
            r.step(100_000);
        }
        // 10_000 * 100_000 % 396 = 208 again: 9_999 more calls add a multiple of 396.
        assert_eq!(r.get_pos(), vec![89, 99]);
        assert_eq!(r.get_dir(), "West");
        r.step(396 - 208 + 64);
        assert_eq!(r.get_pos(), vec![64, 0]);
        assert_eq!(r.get_dir(), "East");
    }
}
