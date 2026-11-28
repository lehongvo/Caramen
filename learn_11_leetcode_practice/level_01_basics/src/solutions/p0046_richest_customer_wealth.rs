//! # Problem 0046: Richest Customer Wealth
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec<Vec<i32>>`, nested `for` loops over references, accumulators
//! - **Source:** [LeetCode 1672. Richest Customer Wealth](https://leetcode.com/problems/richest-customer-wealth/)
//!
//! ## Problem
//!
//! You are given an `m x n` grid `accounts`: `accounts[i][j]` is the amount of money customer `i`
//! keeps in bank `j`. A customer's **wealth** is the total of all their accounts (one row of the
//! grid). Return the wealth of the richest customer.
//!
//! Implement `Solution::maximum_wealth(accounts: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  accounts = [[1, 2, 3], [3, 2, 1]]
//! Output: 6
//! Explanation: both customers have 6.
//!
//! Input:  accounts = [[1, 5], [7, 3], [3, 5]]
//! Output: 10
//! Explanation: the second customer has 7 + 3 = 10.
//!
//! Input:  accounts = [[2, 8, 7], [7, 1, 3], [1, 9, 5]]
//! Output: 17
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 50` (`m` rows, `n` columns)
//! - `1 <= accounts[i][j] <= 100`
//!
//! ## Hints
//!
//! 1. The outer loop goes over customers (rows), the inner loop over banks (columns).
//! 2. `for row in &accounts` gives each row as a `&Vec<i32>` without copying it.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** sum each row in an inner loop and keep the largest sum seen so far.
//!
//! **Complexity:** `O(m * n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - A `Vec<Vec<i32>>` is a vector of rows; `accounts[i][j]` indexes the row and then the column.
//! - Iterating `&accounts` yields `&Vec<i32>`; iterating that row by `&money` copies each `i32`.
//!   Nothing is cloned, and `accounts` is still usable afterwards.
//! - `let mut wealth = 0;` inside the outer loop creates a fresh accumulator for each customer.
//! - The iterator version: `accounts.iter().map(|row| row.iter().sum::<i32>()).max().unwrap_or(0)`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let mut richest = 0;
        for row in &accounts {
            let mut wealth = 0;
            for &money in row {
                wealth += money;
            }
            richest = richest.max(wealth);
        }
        richest
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tie() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
            6
        );
    }

    #[test]
    fn example_two_banks() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
            10
        );
    }

    #[test]
    fn example_square() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17
        );
    }

    #[test]
    fn single_customer_single_bank() {
        assert_eq!(Solution::maximum_wealth(vec![vec![42]]), 42);
    }

    #[test]
    fn largest_grid() {
        let mut accounts = vec![vec![99; 50]; 50];
        accounts[37] = vec![100; 50];
        assert_eq!(Solution::maximum_wealth(accounts), 5_000);
    }
}
