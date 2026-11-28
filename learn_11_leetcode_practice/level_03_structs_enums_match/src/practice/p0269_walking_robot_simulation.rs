// ==================================================================
// PRACTICE 0269: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0269_walking_robot_simulation.md
// Run the tests:          cargo test practice::p0269
// ==================================================================

//! # Problem 0269: Walking Robot Simulation
//!
//! - **Difficulty:** Medium
//! - **Topics:** command `enum` parsed from integers, robot `struct` with heading `enum`, `HashSet` of obstacles, step-by-step simulation
//! - **Source:** [LeetCode 874. Walking Robot Simulation](https://leetcode.com/problems/walking-robot-simulation/)
//!
//! ## Problem
//!
//! A robot starts at `(0, 0)` on an infinite grid, facing north (`+y`). It receives a list of
//! integer `commands`:
//!
//! - `-2`: turn left 90 degrees;
//! - `-1`: turn right 90 degrees;
//! - `1 <= k <= 9`: move forward `k` units, **one unit at a time**.
//!
//! Some grid points are obstacles, given as `obstacles[i] = [x, y]`. If the next unit step would
//! land on an obstacle, the robot stays where it is and ignores the rest of that move command
//! (it still processes the following commands). An obstacle at `(0, 0)` does not stop the robot
//! from leaving the origin, but it cannot come back to it.
//!
//! Return the **maximum squared Euclidean distance** `x² + y²` from the origin that the robot
//! reaches at any moment.
//!
//! Implement `Solution::robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  commands = [4, -1, 3], obstacles = []
//! Output: 25        (north to (0, 4), turn right, east to (3, 4): 9 + 16 = 25)
//!
//! Input:  commands = [4, -1, 4, -2, 4], obstacles = [[2, 4]]
//! Output: 65        ((0, 4), then blocked at (1, 4), then north to (1, 8): 1 + 64 = 65)
//!
//! Input:  commands = [6, -1, -1, 6], obstacles = [[0, 0]]
//! Output: 36        (back down from (0, 6), it stops at (0, 1) because of the obstacle)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= commands.len() <= 10^4`; each command is `-2`, `-1` or in `1..=9`.
//! - `0 <= obstacles.len() <= 10^4`; coordinates are in `[-3 * 10^4, 3 * 10^4]`.
//! - The answer fits in an `i32` (it is below `2^31`).
//!
//! ## Hints
//!
//! 1. Store obstacles in a `HashSet<(i32, i32)>` for `O(1)` lookups.
//! 2. Model commands as `enum Command { Left, Right, Forward(u8) }` and the robot as a struct
//!    with a `heading` field.

pub struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_no_obstacles() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
    }

    #[test]
    fn example_blocked() {
        assert_eq!(
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]),
            65
        );
    }

    #[test]
    fn example_obstacle_at_origin() {
        assert_eq!(
            Solution::robot_sim(vec![6, -1, -1, 6], vec![vec![0, 0]]),
            36
        );
    }

    #[test]
    fn walled_in() {
        let walls = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        assert_eq!(Solution::robot_sim(vec![9, -1, 9, -1, 9, -1, 9], walls), 0);
    }

    #[test]
    fn many_commands() {
        // Repeating "9 north, turn right" walks a 9x9 square forever.
        let mut commands = Vec::new();
        for _ in 0..5_000 {
            commands.extend([9, -1]);
        }
        assert_eq!(Solution::robot_sim(commands, vec![]), 81 + 81);
    }
}
