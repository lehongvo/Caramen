// ==================================================================
// PRACTICE 0063: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0063_calculate_money_in_leetcode_bank.md
// Run the tests:          cargo test practice::p0063
// ==================================================================

//! # Problem 0063: Calculate Money in Leetcode Bank
//!
//! - **Difficulty:** Easy
//! - **Topics:** `for` over a range, `/` and `%` to split days into weeks and weekdays
//! - **Source:** [LeetCode 1716. Calculate Money in Leetcode Bank](https://leetcode.com/problems/calculate-money-in-leetcode-bank/)
//!
//! ## Problem
//!
//! Hercy saves money every day. On the very first day, a Monday, he puts in `$1`. On each following
//! day of the same week (Tuesday to Sunday) he puts in `$1` more than the day before. Every new
//! Monday he puts in `$1` more than on the **previous Monday**, and the week continues the same way.
//!
//! Return the total amount saved after `n` days.
//!
//! Implement `Solution::total_money(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4
//! Output: 10
//! Explanation: 1 + 2 + 3 + 4 = 10.
//!
//! Input:  n = 10
//! Output: 37
//! Explanation: week 1: 1+2+3+4+5+6+7 = 28; week 2 starts at 2: 2+3+4 = 9. Total 37.
//!
//! Input:  n = 20
//! Output: 96
//! Explanation: 28 + 35 + (3+4+5+6+7+8) = 96.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`
//!
//! ## Hints
//!
//! 1. Number the days from 0. Day `d` belongs to week `d / 7` and is weekday `d % 7` (0 is Monday).
//! 2. What is deposited on day `d` in terms of those two numbers?

pub struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_days() {
        assert_eq!(Solution::total_money(4), 10);
    }

    #[test]
    fn example_ten_days() {
        assert_eq!(Solution::total_money(10), 37);
    }

    #[test]
    fn example_twenty_days() {
        assert_eq!(Solution::total_money(20), 96);
    }

    #[test]
    fn whole_weeks() {
        assert_eq!(Solution::total_money(1), 1);
        assert_eq!(Solution::total_money(7), 28);
        assert_eq!(Solution::total_money(14), 63);
    }

    #[test]
    fn largest_input() {
        // 142 full weeks: sum of (28 + 7k) for k = 0..142 = 28*142 + 7*142*141/2 = 73_773.
        // Then 6 more days starting at 143: 143 + ... + 148 = 873.
        assert_eq!(Solution::total_money(1000), 74_926);
    }
}
