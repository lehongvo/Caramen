// ==================================================================
// PRACTICE 0748: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0748_car_fleet.md
// Run the tests:          cargo test practice::p0748
// ==================================================================

//! # Problem 0748: Car Fleet
//!
//! - **Difficulty:** Medium
//! - **Topics:** sorting by position, monotonic stack of arrival times, exact comparison of fractions
//! - **Source:** [LeetCode 853. Car Fleet](https://leetcode.com/problems/car-fleet/)
//!
//! ## Problem
//!
//! `n` cars drive along a one-lane road towards a destination at mile `target`. Car `i` starts at
//! mile `position[i]` and drives at `speed[i]` miles per hour. A car can never pass the car in
//! front of it: when it catches up, it slows down and the two drive together at the slower speed.
//!
//! A **fleet** is a group of cars driving together (a single car is also a fleet). Cars that catch
//! up exactly at the destination form one fleet. Return the number of fleets that arrive at the
//! destination.
//!
//! Implement `Solution::car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  target = 12, position = [10, 8, 0, 5, 3], speed = [2, 4, 1, 1, 3]
//! Output: 3
//!         (cars at 10 and 8 both arrive at hour 1 together; the car at 0 is alone;
//!          the car at 3 catches the car at 5 at mile 6, and they arrive together)
//!
//! Input:  target = 10, position = [3], speed = [3]
//! Output: 1
//!
//! Input:  target = 100, position = [0, 2, 4], speed = [4, 2, 1]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^5`, `position.len() == speed.len() == n`
//! - `0 < target <= 10^6`
//! - `0 <= position[i] < target`, all positions distinct
//! - `0 < speed[i] <= 10^6`
//!
//! ## Hints
//!
//! 1. Only the car directly in front matters. Process cars from the one closest to the destination
//!    backwards.
//! 2. Compute when each car would arrive if it were alone. A car behind that would arrive
//!    **no later** than the fleet in front joins that fleet.
//! 3. Comparing `f64` times can suffer from rounding. Compare the fractions
//!    `distance / speed` with integer cross-multiplication instead.

use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_fleets() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        );
    }

    #[test]
    fn example_single_car() {
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    }

    #[test]
    fn example_everyone_merges() {
        assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
    }

    #[test]
    fn faster_car_ahead_stays_ahead() {
        assert_eq!(Solution::car_fleet(10, vec![6, 8], vec![3, 2]), 2);
    }

    #[test]
    fn catching_up_exactly_at_the_destination() {
        // Both cars need 5 hours: they meet at the target and count as one fleet.
        assert_eq!(Solution::car_fleet(10, vec![0, 5], vec![2, 1]), 1);
    }

    #[test]
    fn nearly_equal_times_are_compared_exactly() {
        // 1_000_000 / 999_999 < 999_999 / 999_998: the car behind is (barely) faster overall.
        assert_eq!(
            Solution::car_fleet(1_000_000, vec![0, 1], vec![999_999, 999_998]),
            1
        );
    }

    #[test]
    fn equal_speeds_never_merge() {
        let n = 100_000;
        let position: Vec<i32> = (0..n).collect();
        assert_eq!(
            Solution::car_fleet(1_000_000, position, vec![7; n as usize]),
            n
        );
    }
}
