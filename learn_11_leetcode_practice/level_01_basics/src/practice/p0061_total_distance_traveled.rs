// ==================================================================
// PRACTICE 0061: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0061_total_distance_traveled.md
// Run the tests:          cargo test practice::p0061
// ==================================================================

//! # Problem 0061: Total Distance Traveled
//!
//! - **Difficulty:** Easy
//! - **Topics:** `while` simulation with a compound condition `&&`, several `mut` variables
//! - **Source:** [LeetCode 2739. Total Distance Traveled](https://leetcode.com/problems/total-distance-traveled/)
//!
//! ## Problem
//!
//! A truck has two fuel tanks: the main tank holds `main_tank` liters and the additional tank holds
//! `additional_tank` liters. The truck drives 10 km per liter and only burns fuel from the main tank.
//!
//! Every time **5 liters** have been used from the main tank, if the additional tank still has at
//! least 1 liter, exactly 1 liter is moved from the additional tank into the main tank right away.
//! The truck stops when the main tank is empty. Return the total distance driven, in km.
//!
//! Implement `Solution::distance_traveled(main_tank: i32, additional_tank: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  main_tank = 5, additional_tank = 10
//! Output: 60
//! Explanation: after 5 liters (50 km) one liter is transferred; it gives 10 more km. Total 60 km.
//!
//! Input:  main_tank = 1, additional_tank = 2
//! Output: 10
//! Explanation: the main tank runs dry before 5 liters are used, so no transfer happens.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= main_tank, additional_tank <= 100`
//!
//! ## Hints
//!
//! 1. Simulate in chunks of 5 liters while the main tank has at least 5 liters **and** the
//!    additional tank is not empty.
//! 2. When the loop ends, the remaining fuel in the main tank is burned without further transfers.

pub struct Solution;

impl Solution {
    pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_transfer() {
        assert_eq!(Solution::distance_traveled(5, 10), 60);
    }

    #[test]
    fn example_no_transfer() {
        assert_eq!(Solution::distance_traveled(1, 2), 10);
    }

    #[test]
    fn chained_transfers() {
        // 9 liters: burn 5 (+1) -> 5 left, burn 5 (+1) -> 1 left, burn 1. 11 liters in total.
        assert_eq!(Solution::distance_traveled(9, 2), 110);
        assert_eq!(Solution::distance_traveled(9, 1), 100);
    }

    #[test]
    fn largest_inputs() {
        // 100 liters lead to 24 transfers (the main tank burns 4 net liters per transfer).
        assert_eq!(Solution::distance_traveled(100, 100), 1240);
        assert_eq!(Solution::distance_traveled(100, 1), 1010);
    }
}
