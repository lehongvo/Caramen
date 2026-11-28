// ==================================================================
// PRACTICE 0292: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0292_simple_bank_system.md
// Run the tests:          cargo test practice::p0292
// ==================================================================

//! # Problem 0292: Simple Bank System
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, validating 1-based account numbers into `Option<usize>`, `get_mut`, checking before mutating
//! - **Source:** [LeetCode 2043. Simple Bank System](https://leetcode.com/problems/simple-bank-system/)
//!
//! ## Problem
//!
//! A bank has `n` accounts numbered `1` to `n`; `balance[i]` is the initial balance of account
//! `i + 1`. Implement the `Bank` struct:
//!
//! - `Bank::new(balance: Vec<i64>) -> Self`;
//! - `transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool`: move `money` from
//!   `account1` to `account2`;
//! - `deposit(&mut self, account: i32, money: i64) -> bool`: add `money` to `account`;
//! - `withdraw(&mut self, account: i32, money: i64) -> bool`: take `money` from `account`.
//!
//! A transaction is **valid** when every account number involved is between `1` and `n` and, for
//! `transfer` and `withdraw`, the paying account has at least `money`. A valid transaction is
//! performed and returns `true`; an invalid one changes nothing and returns `false`. A transfer
//! from an account to itself is valid if the account exists and has enough money (and changes
//! nothing).
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`.
//!
//! ## Examples
//!
//! ```text
//! let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
//! bank.withdraw(3, 10)     => true     (account 3: 20 -> 10)
//! bank.transfer(5, 1, 20)  => true     (account 5: 30 -> 10, account 1: 10 -> 30)
//! bank.deposit(5, 20)      => true     (account 5: 10 -> 30)
//! bank.transfer(3, 4, 15)  => false    (account 3 has only 10)
//! bank.withdraw(10, 50)    => false    (account 10 does not exist)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^5`; balances and `money` are in `[0, 10^12]`; at most `10^4` calls.
//!
//! ## Hints
//!
//! 1. Write `fn index(&self, account: i32) -> Option<usize>` that turns a 1-based account number
//!    into a valid vector index, or `None`.
//! 2. In `transfer`, validate **both** accounts and the balance before changing anything.

/// A bank with numbered accounts.
pub struct Bank {}

impl Bank {
    /// A bank whose account `i + 1` starts with `balance[i]`.
    pub fn new(balance: Vec<i64>) -> Self {
        todo!("Write your solution here")
    }

    /// Moves `money` from `account1` to `account2`.
    pub fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        todo!("Write your solution here")
    }

    /// Adds `money` to `account`.
    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        todo!("Write your solution here")
    }

    /// Takes `money` from `account`.
    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
        assert!(bank.withdraw(3, 10));
        assert!(bank.transfer(5, 1, 20));
        assert!(bank.deposit(5, 20));
        assert!(!bank.transfer(3, 4, 15));
        assert!(!bank.withdraw(10, 50));
    }

    #[test]
    fn invalid_accounts_change_nothing() {
        let mut bank = Bank::new(vec![5]);
        assert!(!bank.deposit(0, 1));
        assert!(!bank.deposit(2, 1));
        assert!(!bank.deposit(-1, 1));
        assert!(!bank.transfer(1, 2, 1));
        assert!(!bank.transfer(2, 1, 0));
        // Account 1 still has exactly 5.
        assert!(bank.withdraw(1, 5));
        assert!(!bank.withdraw(1, 1));
    }

    #[test]
    fn self_transfer_and_zero_amounts() {
        let mut bank = Bank::new(vec![7, 0]);
        assert!(bank.transfer(1, 1, 7));
        assert!(!bank.transfer(1, 1, 8));
        assert!(bank.withdraw(2, 0));
        assert!(bank.transfer(2, 1, 0));
        assert!(bank.withdraw(1, 7));
    }

    #[test]
    fn large_amounts_and_many_accounts() {
        let mut bank = Bank::new(vec![1_000_000_000_000; 100_000]);
        assert!(bank.transfer(1, 100_000, 1_000_000_000_000));
        assert!(bank.withdraw(100_000, 2_000_000_000_000));
        assert!(!bank.withdraw(1, 1));
        assert!(!bank.deposit(100_001, 1));
    }
}
