// ==================================================================
// PRACTICE 0046: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0046_richest_customer_wealth.md
// Run the tests:          cargo test practice::p0046
// ==================================================================

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

pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
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
