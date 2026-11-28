// ==================================================================
// PRACTICE 0003: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0003_find_the_maximum_achievable_number.md
// Run the tests:          cargo test practice::p0003
// ==================================================================

//! # Problem 0003: Find the Maximum Achievable Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** arithmetic expressions, integer multiplication, reasoning before coding
//! - **Source:** [LeetCode 2769. Find the Maximum Achievable Number](https://leetcode.com/problems/find-the-maximum-achievable-number/)
//!
//! ## Problem
//!
//! You are given two integers `num` and `t`. Call an integer `x` **achievable** if you can make
//! `x` equal to `num` using at most `t` operations, where one operation does both of these things
//! at the same time:
//!
//! - change `x` by exactly 1 (either `+1` or `-1`), and
//! - change `num` by exactly 1 (either `+1` or `-1`).
//!
//! Return the largest achievable `x`. An achievable number always exists.
//!
//! Implement `Solution::the_maximum_achievable_x(num: i32, t: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 4, t = 1
//! Output: 6
//! Explanation: start with x = 6. One operation: x becomes 5 and num becomes 5. They are equal.
//!
//! Input:  num = 3, t = 2
//! Output: 7
//! Explanation: x = 7 -> (6, 4) -> (5, 5) after two operations.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num, t <= 50`
//!
//! ## Hints
//!
//! 1. To reach `num` from a large `x` as fast as possible, every operation should decrease `x` and increase `num`.
//! 2. How much does the gap `x - num` shrink in one such operation?

pub struct Solution;

impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_num_4_t_1() {
        assert_eq!(Solution::the_maximum_achievable_x(4, 1), 6);
    }

    #[test]
    fn example_num_3_t_2() {
        assert_eq!(Solution::the_maximum_achievable_x(3, 2), 7);
    }

    #[test]
    fn smallest_inputs() {
        assert_eq!(Solution::the_maximum_achievable_x(1, 1), 3);
    }

    #[test]
    fn largest_inputs() {
        assert_eq!(Solution::the_maximum_achievable_x(50, 50), 150);
    }
}
