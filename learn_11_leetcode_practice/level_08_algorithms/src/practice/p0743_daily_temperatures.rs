// ==================================================================
// PRACTICE 0743: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0743_daily_temperatures.md
// Run the tests:          cargo test practice::p0743
// ==================================================================

//! # Problem 0743: Daily Temperatures
//!
//! - **Difficulty:** Medium
//! - **Topics:** monotonic stack of indices, distances between indices, `pop_if`
//! - **Source:** [LeetCode 739. Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)
//!
//! ## Problem
//!
//! `temperatures[i]` is the temperature on day `i`. Return a vector `answer` where `answer[i]` is
//! the number of days you have to wait after day `i` to get a **strictly warmer** day. If no
//! later day is warmer, `answer[i]` is `0`.
//!
//! Implement `Solution::daily_temperatures(temperatures: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  temperatures = [73, 74, 75, 71, 69, 72, 76, 73]
//! Output: [1, 1, 4, 2, 1, 1, 0, 0]
//!
//! Input:  temperatures = [30, 40, 50, 60]
//! Output: [1, 1, 1, 0]
//!
//! Input:  temperatures = [30, 60, 90]
//! Output: [1, 1, 0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= temperatures.len() <= 10^5`
//! - `30 <= temperatures[i] <= 100`
//!
//! ## Hints
//!
//! 1. With `10^5` days, checking every later day for every day is too slow.
//! 2. Keep the days that are still waiting for a warmer day on a stack. Store **indices**, so you
//!    can compute the distance when the warmer day arrives.

pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn example_increasing() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn example_short() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }

    #[test]
    fn equal_temperatures_are_not_warmer() {
        assert_eq!(
            Solution::daily_temperatures(vec![50, 50, 50]),
            vec![0, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![50, 50, 51]),
            vec![2, 1, 0]
        );
    }

    #[test]
    fn long_cold_spell_then_heat() {
        let mut temperatures = vec![30; 99_999];
        temperatures.push(100);
        let answer = Solution::daily_temperatures(temperatures);
        assert_eq!(answer[0], 99_999);
        assert_eq!(answer[99_998], 1);
        assert_eq!(answer[99_999], 0);
    }
}
