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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** parse each integer into a `Command`, then simulate: turns change the heading; a
//! forward move takes unit steps until it is done or the next cell is an obstacle. After every
//! step, update the maximum squared distance.
//!
//! **Complexity:** `O(c * 9 + o)` time for `c` commands and `o` obstacles; `O(o)` space.
//!
//! **Rust concepts:**
//! - `Command::from_code(i32) -> Option<Command>` uses a range pattern `k @ 1..=9` to accept the
//!   move lengths and binds the value at the same time.
//! - Obstacles are converted with a slice pattern in `filter_map`:
//!   `match o.as_slice() { &[x, y] => Some((x, y)), _ => None }`.
//! - The robot's `forward(&mut self, k, obstacles: &HashSet<(i32, i32)>)` borrows the set
//!   immutably while mutating itself: two different values, so no conflict.
//! - `HashSet` is covered in the collections level; here it is only used through `contains`.
// @solution-end

pub struct Solution;

// @solution-start remove
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    Left,
    Right,
    Forward(u8),
}

impl Command {
    fn from_code(code: i32) -> Option<Command> {
        match code {
            -2 => Some(Command::Left),
            -1 => Some(Command::Right),
            // `k` is in 1..=9, so it fits in a u8.
            k @ 1..=9 => Some(Command::Forward(k as u8)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn right(self) -> Heading {
        match self {
            Heading::North => Heading::East,
            Heading::East => Heading::South,
            Heading::South => Heading::West,
            Heading::West => Heading::North,
        }
    }

    fn left(self) -> Heading {
        self.right().right().right()
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Heading::North => (0, 1),
            Heading::East => (1, 0),
            Heading::South => (0, -1),
            Heading::West => (-1, 0),
        }
    }
}

struct Robot {
    x: i32,
    y: i32,
    heading: Heading,
    best: i32,
}

impl Robot {
    fn forward(&mut self, k: u8, obstacles: &HashSet<(i32, i32)>) {
        let (dx, dy) = self.heading.delta();
        for _ in 0..k {
            let next = (self.x + dx, self.y + dy);
            if obstacles.contains(&next) {
                break;
            }
            (self.x, self.y) = next;
            self.best = self.best.max(self.x * self.x + self.y * self.y);
        }
    }
}
// @solution-end

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let blocked: HashSet<(i32, i32)> = obstacles
            .iter()
            .filter_map(|o| match o.as_slice() {
                &[x, y] => Some((x, y)),
                _ => None,
            })
            .collect();
        let mut robot = Robot {
            x: 0,
            y: 0,
            heading: Heading::North,
            best: 0,
        };
        for cmd in commands.into_iter().filter_map(Command::from_code) {
            match cmd {
                Command::Left => robot.heading = robot.heading.left(),
                Command::Right => robot.heading = robot.heading.right(),
                Command::Forward(k) => robot.forward(k, &blocked),
            }
        }
        robot.best
        // @solution-end
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
