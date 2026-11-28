//! # Problem 0044: Find the Highest Altitude
//!
//! - **Difficulty:** Easy
//! - **Topics:** running totals with `mut`, `max`, iterating a `Vec` by reference
//! - **Source:** [LeetCode 1732. Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/)
//!
//! ## Problem
//!
//! A cyclist rides through `n + 1` points at different altitudes. The ride starts at point 0 at
//! altitude `0`. You are given a vector `gain` of length `n`, where `gain[i]` is the change in
//! altitude between point `i` and point `i + 1` (negative means downhill). Return the **highest**
//! altitude reached at any point, including the starting point.
//!
//! Implement `Solution::largest_altitude(gain: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  gain = [-5, 1, 5, 0, -7]
//! Output: 1
//! Explanation: the altitudes are [0, -5, -4, 1, 1, -6]; the highest is 1.
//!
//! Input:  gain = [-4, -3, -2, -1, 4, 3, 2]
//! Output: 0
//! Explanation: the altitudes are [0, -4, -7, -9, -10, -6, -3, -1]; the start is the highest point.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= gain.len() <= 100`
//! - `-100 <= gain[i] <= 100`
//!
//! ## Hints
//!
//! 1. Keep the current altitude in a `mut` variable and add each gain to it.
//! 2. Keep a second variable with the best altitude so far. What should it start at?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** start with `altitude = 0` and `highest = 0` (the starting point counts). After each
//! gain, update `highest = highest.max(altitude)`.
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - Two `mut` bindings carry state across loop iterations; everything else is immutable.
//! - `a.max(b)` is a method on every integer type (from the `Ord` trait).
//! - Starting `highest` at `0` (not `i32::MIN`) encodes the rule that the start point counts.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        // @solution-start
        let mut altitude = 0;
        let mut highest = 0;
        for &g in &gain {
            altitude += g;
            highest = highest.max(altitude);
        }
        highest
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_peak_in_the_middle() {
        assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    }

    #[test]
    fn example_start_is_highest() {
        assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }

    #[test]
    fn single_gain() {
        assert_eq!(Solution::largest_altitude(vec![7]), 7);
        assert_eq!(Solution::largest_altitude(vec![-7]), 0);
    }

    #[test]
    fn always_climbing() {
        assert_eq!(Solution::largest_altitude(vec![100; 100]), 10_000);
    }

    #[test]
    fn peak_then_valley() {
        let mut gain = vec![100; 50];
        gain.extend(vec![-100; 50]);
        assert_eq!(Solution::largest_altitude(gain), 5_000);
    }
}
