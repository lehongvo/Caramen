// ==================================================================
// PRACTICE 0466: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0466_running_balance_scan.md
// Run the tests:          cargo test practice::p0466
// ==================================================================

//! # Problem 0466: Running Balance with scan
//!
//! - **Difficulty:** Easy
//! - **Topics:** `scan` for running totals, stopping `scan` early with `None`, `position`, `iter::once` + `chain` + `max`
//! - **Source:** Original · [std `Iterator::scan`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan)
//!
//! ## Problem
//!
//! A bank account starts with `start` and then receives a list of transactions (positive deposits,
//! negative withdrawals). `scan` turns this into the list of balances after each transaction,
//! carrying the current balance as its state. Its closure returns an `Option`: returning `None`
//! **ends** the iteration, which is how you stop at the first problem.
//!
//! Implement:
//!
//! - `balances(start: i64, transactions: &[i64]) -> Vec<i64>`: the balance after each transaction.
//! - `balances_until_overdraft(start: i64, transactions: &[i64]) -> Vec<i64>`: the same, but stop
//!   **before** the first transaction that would make the balance negative (that transaction and
//!   all later ones are ignored).
//! - `first_overdraft(start: i64, transactions: &[i64]) -> Option<usize>`: the index of the first
//!   transaction after which the balance is negative.
//! - `peak_balance(start: i64, transactions: &[i64]) -> i64`: the largest balance ever held,
//!   including the starting balance.
//!
//! ## Examples
//!
//! ```text
//! balances(100, &[-30, 50, -200, 10])                   -> [70, 120, -80, -70]
//! balances_until_overdraft(100, &[-30, 50, -200, 10])   -> [70, 120]
//! first_overdraft(100, &[-30, 50, -200, 10])            -> Some(2)
//! first_overdraft(10, &[5, -15])                        -> None   (the balance reaches 0, not below)
//! peak_balance(100, &[-30, 50, -200, 10])               -> 120
//! peak_balance(100, &[-1])                              -> 100
//! ```
//!
//! ## Constraints
//!
//! - `start >= 0`; at most `10^5` transactions with `|t| <= 10^9`, so no sum overflows an `i64`.
//!
//! ## Hints
//!
//! 1. `transactions.iter().scan(start, |balance, &t| { *balance += t; Some(*balance) })`.
//! 2. To stop early, compute the next balance and return `None` when it is negative, **without**
//!    yielding it.
//! 3. `iter::once(start).chain(balances...)` adds the starting balance in front.

use std::iter;

pub fn balances(start: i64, transactions: &[i64]) -> Vec<i64> {
    todo!("Write your solution here")
}

pub fn balances_until_overdraft(start: i64, transactions: &[i64]) -> Vec<i64> {
    todo!("Write your solution here")
}

pub fn first_overdraft(start: i64, transactions: &[i64]) -> Option<usize> {
    todo!("Write your solution here")
}

pub fn peak_balance(start: i64, transactions: &[i64]) -> i64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TXS: [i64; 4] = [-30, 50, -200, 10];

    #[test]
    fn running_balances() {
        assert_eq!(balances(100, &TXS), vec![70, 120, -80, -70]);
        assert_eq!(balances(5, &[]), Vec::<i64>::new());
    }

    #[test]
    fn stop_before_overdraft() {
        assert_eq!(balances_until_overdraft(100, &TXS), vec![70, 120]);
        assert_eq!(balances_until_overdraft(0, &[-1, 100]), Vec::<i64>::new());
        assert_eq!(balances_until_overdraft(10, &[5, -15, 3]), vec![15, 0, 3]);
    }

    #[test]
    fn first_negative_balance() {
        assert_eq!(first_overdraft(100, &TXS), Some(2));
        assert_eq!(first_overdraft(10, &[5, -15]), None);
        assert_eq!(first_overdraft(0, &[-1]), Some(0));
    }

    #[test]
    fn peaks() {
        assert_eq!(peak_balance(100, &TXS), 120);
        assert_eq!(peak_balance(100, &[-1]), 100);
        assert_eq!(peak_balance(7, &[]), 7);
    }

    #[test]
    fn large_input() {
        let txs: Vec<i64> = (0..100_000)
            .map(|i| {
                if i % 2 == 0 {
                    1_000_000_000
                } else {
                    -999_999_999
                }
            })
            .collect();
        let all = balances(0, &txs);
        assert_eq!(all.len(), 100_000);
        assert_eq!(*all.last().unwrap(), 50_000);
        assert_eq!(peak_balance(0, &txs), 1_000_049_999);
        assert_eq!(first_overdraft(0, &txs), None);
    }
}
