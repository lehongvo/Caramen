// ==================================================================
// PRACTICE 0265: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0265_lemonade_change.md
// Run the tests:          cargo test practice::p0265
// ==================================================================

//! # Problem 0265: Lemonade Change
//!
//! - **Difficulty:** Easy
//! - **Topics:** a `Bill` enum parsed from `i32`, a `Till` struct with a `&mut self` method, greedy choice in `match` arms
//! - **Source:** [LeetCode 860. Lemonade Change](https://leetcode.com/problems/lemonade-change/)
//!
//! ## Problem
//!
//! A lemonade costs `$5`. Customers arrive in the order given by `bills`, each buys exactly one
//! lemonade and pays with a single `$5`, `$10` or `$20` bill. You start with **no** money and must
//! give every customer the exact change (`$0`, `$5` or `$15`) using only the bills received so far.
//!
//! Return `true` if you can give every customer correct change.
//!
//! Implement `Solution::lemonade_change(bills: Vec<i32>) -> bool`.
//!
//! For this level, convert each number into an `enum Bill { Five, Ten, Twenty }` and keep the cash
//! in a `struct Till { fives, tens }` with a method `accept(&mut self, bill: Bill) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  bills = [5, 5, 5, 10, 20]
//! Output: true     (the last customer gets a $10 and a $5)
//!
//! Input:  bills = [5, 5, 10, 10, 20]
//! Output: false    (after two $10 customers, only $10 bills are left: no way to give $15)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= bills.len() <= 10^5`
//! - `bills[i]` is `5`, `10` or `20`.
//!
//! ## Hints
//!
//! 1. Twenty-dollar bills are never useful as change, so the till only needs to count fives and tens.
//! 2. For a `$20`, prefer giving `$10 + $5` over `$5 + $5 + $5`: fives are more flexible.

pub struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_can_serve_everyone() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]));
    }

    #[test]
    fn example_runs_out_of_fives() {
        assert!(!Solution::lemonade_change(vec![5, 5, 10, 10, 20]));
    }

    #[test]
    fn first_customer_needs_change() {
        assert!(!Solution::lemonade_change(vec![10]));
        assert!(!Solution::lemonade_change(vec![20]));
        assert!(Solution::lemonade_change(vec![5]));
    }

    #[test]
    fn prefers_ten_plus_five() {
        // Giving 5+5+5 for the first 20 would leave no five for the final 10.
        assert!(Solution::lemonade_change(vec![5, 5, 5, 5, 10, 20, 10]));
    }

    #[test]
    fn three_fives_when_no_ten() {
        assert!(Solution::lemonade_change(vec![5, 5, 5, 20]));
        assert!(!Solution::lemonade_change(vec![5, 5, 20]));
    }

    #[test]
    fn long_queue() {
        // Every ten uses up one five, so the final twenty finds no five left.
        let mut bills = vec![5; 49_999];
        bills.extend(vec![10; 49_999]);
        assert!(Solution::lemonade_change(bills.clone()));
        bills.push(20);
        assert!(!Solution::lemonade_change(bills));
    }
}
