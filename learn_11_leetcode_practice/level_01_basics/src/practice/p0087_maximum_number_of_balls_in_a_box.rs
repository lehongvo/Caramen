// ==================================================================
// PRACTICE 0087: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0087_maximum_number_of_balls_in_a_box.md
// Run the tests:          cargo test practice::p0087
// ==================================================================

//! # Problem 0087: Maximum Number of Balls in a Box
//!
//! - **Difficulty:** Easy
//! - **Topics:** fixed-size array as a counter `[i32; 46]`, digit sums, `as usize`, finding a maximum
//! - **Source:** [LeetCode 1742. Maximum Number of Balls in a Box](https://leetcode.com/problems/maximum-number-of-balls-in-a-box/)
//!
//! ## Problem
//!
//! A factory has one ball for every number from `low_limit` to `high_limit` (inclusive). Each ball
//! is placed into the box whose number equals the **sum of the digits** of the ball's number: ball
//! `321` goes into box `3 + 2 + 1 = 6`, and ball `10` into box `1`.
//!
//! Return the number of balls in the box that receives the most balls.
//!
//! Implement `Solution::count_balls(low_limit: i32, high_limit: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  low_limit = 1, high_limit = 10
//! Output: 2
//! Explanation: box 1 gets balls 1 and 10; every other box gets at most one ball.
//!
//! Input:  low_limit = 5, high_limit = 15
//! Output: 2
//! Explanation: boxes 5 and 6 get two balls each (5, 14 and 6, 15).
//!
//! Input:  low_limit = 19, high_limit = 28
//! Output: 2
//! Explanation: box 10 gets balls 19 and 28.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= low_limit <= high_limit <= 10^5`
//!
//! ## Hints
//!
//! 1. What is the largest possible digit sum for numbers up to `10^5`? (Think of `99999`.)
//! 2. That makes the number of boxes small and fixed: an array works as a counter.

pub struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_to_ten() {
        assert_eq!(Solution::count_balls(1, 10), 2);
    }

    #[test]
    fn example_five_to_fifteen() {
        assert_eq!(Solution::count_balls(5, 15), 2);
    }

    #[test]
    fn example_nineteen_to_twenty_eight() {
        assert_eq!(Solution::count_balls(19, 28), 2);
    }

    #[test]
    fn single_ball() {
        assert_eq!(Solution::count_balls(99_999, 99_999), 1);
        assert_eq!(Solution::count_balls(100_000, 100_000), 1);
    }

    #[test]
    fn full_range() {
        assert_eq!(Solution::count_balls(1, 100_000), 6000);
    }
}
