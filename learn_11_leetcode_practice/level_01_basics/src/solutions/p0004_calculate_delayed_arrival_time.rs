//! # Problem 0004: Calculate Delayed Arrival Time
//!
//! - **Difficulty:** Easy
//! - **Topics:** the remainder operator `%`, clock arithmetic
//! - **Source:** [LeetCode 2651. Calculate Delayed Arrival Time](https://leetcode.com/problems/calculate-delayed-arrival-time/)
//!
//! ## Problem
//!
//! A train is scheduled to arrive at hour `arrival_time` of a 24-hour clock (hours are numbered
//! `0` to `23`). It is late by `delayed_time` hours. Return the hour at which it actually arrives,
//! also on the 24-hour clock. After hour 23 the clock goes back to hour 0.
//!
//! Implement `Solution::find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arrival_time = 15, delayed_time = 5
//! Output: 20
//!
//! Input:  arrival_time = 13, delayed_time = 11
//! Output: 0
//! Explanation: 13 + 11 = 24, which is hour 0 of the next day.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= arrival_time < 24`
//! - `1 <= delayed_time <= 24`
//!
//! ## Hints
//!
//! 1. First add the two numbers, then bring the result back into the range `0..24`.
//! 2. The remainder of a division by 24 is always between 0 and 23 for non-negative numbers.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the actual arrival hour is `(arrival_time + delayed_time) % 24`. The sum is at most
//! `23 + 24 = 47`, and `% 24` maps it back onto the clock.
//!
//! **Complexity:** `O(1)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `%` is the remainder operator. For non-negative operands it behaves like the "modulo" you know
//!   from school; with negative operands it keeps the sign of the left operand (see problem 0009).
//! - Parentheses control evaluation order: `%` binds tighter than `+`, so `a + b % 24` would be
//!   wrong here.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        // @solution-start
        (arrival_time + delayed_time) % 24
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_same_day() {
        assert_eq!(Solution::find_delayed_arrival_time(15, 5), 20);
    }

    #[test]
    fn example_exactly_midnight() {
        assert_eq!(Solution::find_delayed_arrival_time(13, 11), 0);
    }

    #[test]
    fn delay_of_a_full_day() {
        assert_eq!(Solution::find_delayed_arrival_time(7, 24), 7);
    }

    #[test]
    fn largest_sum_wraps() {
        assert_eq!(Solution::find_delayed_arrival_time(23, 24), 23);
        assert_eq!(Solution::find_delayed_arrival_time(23, 2), 1);
    }
}
