// ==================================================================
// PRACTICE 0097: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0097_climbing_stairs.md
// Run the tests:          cargo test practice::p0097
// ==================================================================

//! # Problem 0097: Climbing Stairs
//!
//! - **Difficulty:** Easy
//! - **Topics:** recognizing a recurrence, iterative Fibonacci, tuple assignment
//! - **Source:** [LeetCode 70. Climbing Stairs](https://leetcode.com/problems/climbing-stairs/)
//!
//! ## Problem
//!
//! A staircase has `n` steps. Each move climbs either **1** or **2** steps. In how many distinct
//! ways can you reach the top? Two ways are different if the sequence of moves differs.
//!
//! Implement `Solution::climb_stairs(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 2
//! Output: 2
//! Explanation: 1 + 1, or 2.
//!
//! Input:  n = 3
//! Output: 3
//! Explanation: 1 + 1 + 1, 1 + 2, or 2 + 1.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 45`
//!
//! ## Hints
//!
//! 1. Look at the **last** move: it is either a 1-step from step `n - 1` or a 2-step from step `n - 2`.
//! 2. So `ways(n) = ways(n - 1) + ways(n - 2)`. Which sequence is that?

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn small_staircases() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
    }

    #[test]
    fn largest_input_fits_in_i32() {
        assert_eq!(Solution::climb_stairs(45), 1_836_311_903);
    }
}
