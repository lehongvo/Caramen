// ==================================================================
// PRACTICE 0889: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0889_deadlock_free_bank_transfers.md
// Run the tests:          cargo test practice::p0889
// ==================================================================

//! # Problem 0889: Deadlock-Free Bank Transfers
//!
//! - **Difficulty:** Medium
//! - **Topics:** one `Mutex` per account (`Vec<Mutex<i64>>`), deadlock avoidance by a global lock
//!   order, holding several guards at once, an error `enum`
//! - **Source:** Original · [The Rust Book 16.3](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
//!
//! ## Problem
//!
//! A bank stores each balance in its own `Mutex`, so transfers between unrelated accounts can run
//! in parallel. A transfer must lock **both** accounts. If thread A locks account 1 then waits for
//! account 2 while thread B locks account 2 then waits for account 1, both wait forever: a
//! **deadlock**. The classic fix is to always lock accounts in the same global order (for example,
//! smaller index first), whatever the direction of the transfer.
//!
//! Implement `Bank`:
//!
//! - `new(balances: Vec<i64>) -> Bank`: account `i` starts with `balances[i]`;
//! - `balance(&self, id: usize) -> Option<i64>`: `None` for an unknown account;
//! - `transfer(&self, from: usize, to: usize, amount: i64) -> Result<(), TransferError>`: move
//!   `amount` (always `> 0`) from `from` to `to`. Errors, checked in this order:
//!   `NoSuchAccount` if either index is out of range, `SameAccount` if `from == to`,
//!   `InsufficientFunds` if `from` holds less than `amount` (then nothing changes);
//! - `total(&self) -> i64`: the sum of all balances as one **consistent snapshot**: while
//!   transfers run concurrently, the result must always equal the true total (money is never seen
//!   "in flight").
//!
//! ## Examples
//!
//! ```text
//! let bank = Bank::new(vec![100, 50]);
//! bank.transfer(0, 1, 30) -> Ok(())       balances [70, 80]
//! bank.transfer(1, 0, 500) -> Err(InsufficientFunds)
//! bank.transfer(0, 0, 1)  -> Err(SameAccount)
//! bank.transfer(0, 9, 1)  -> Err(NoSuchAccount)
//! bank.total()            -> 150
//! ```
//!
//! ## Constraints
//!
//! - `1 <= balances.len() <= 100`; up to 16 threads transferring concurrently.
//!
//! ## Hints
//!
//! 1. Let `(first, second) = (from.min(to), from.max(to))`; lock `first`, then `second`; then work
//!    out which guard is the sender.
//! 2. For `total`, lock **every** account in increasing index order and keep all guards alive
//!    (collect them into a `Vec`) while summing. The same order as `transfer` means no deadlock.

use std::sync::{Mutex, MutexGuard, PoisonError};

#[derive(Debug, PartialEq, Eq)]
pub enum TransferError {
    NoSuchAccount,
    SameAccount,
    InsufficientFunds,
}

/// A bank with one lock per account.
pub struct Bank {}

impl Bank {
    pub fn new(balances: Vec<i64>) -> Self {
        todo!("Write your solution here")
    }

    pub fn balance(&self, id: usize) -> Option<i64> {
        todo!("Write your solution here")
    }

    pub fn transfer(&self, from: usize, to: usize, amount: i64) -> Result<(), TransferError> {
        todo!("Write your solution here")
    }

    pub fn total(&self) -> i64 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;

    #[test]
    fn example_transfers_and_errors() {
        let bank = Bank::new(vec![100, 50]);
        assert_eq!(bank.transfer(0, 1, 30), Ok(()));
        assert_eq!((bank.balance(0), bank.balance(1)), (Some(70), Some(80)));
        assert_eq!(
            bank.transfer(1, 0, 500),
            Err(TransferError::InsufficientFunds)
        );
        assert_eq!(bank.transfer(0, 0, 1), Err(TransferError::SameAccount));
        assert_eq!(bank.transfer(0, 9, 1), Err(TransferError::NoSuchAccount));
        assert_eq!(bank.total(), 150);
    }

    #[test]
    fn failed_transfer_changes_nothing() {
        let bank = Bank::new(vec![10, 0, 5]);
        assert_eq!(
            bank.transfer(1, 2, 1),
            Err(TransferError::InsufficientFunds)
        );
        assert_eq!(bank.balance(1), Some(0));
        assert_eq!(bank.balance(3), None);
        assert_eq!(bank.transfer(2, 0, 5), Ok(()));
        assert_eq!(bank.balance(0), Some(15));
    }

    #[test]
    fn opposite_directions_do_not_deadlock() {
        let bank = Bank::new(vec![1_000_000, 1_000_000]);
        thread::scope(|s| {
            for t in 0..16 {
                let bank = &bank;
                s.spawn(move || {
                    let (from, to) = if t % 2 == 0 { (0, 1) } else { (1, 0) };
                    for _ in 0..10_000 {
                        bank.transfer(from, to, 1).unwrap();
                    }
                });
            }
        });
        assert_eq!(
            (bank.balance(0), bank.balance(1)),
            (Some(1_000_000), Some(1_000_000))
        );
    }

    #[test]
    fn total_is_a_consistent_snapshot() {
        let bank = Bank::new(vec![1000; 10]);
        let done = AtomicBool::new(false);
        thread::scope(|s| {
            for t in 0..8_usize {
                let bank = &bank;
                s.spawn(move || {
                    for i in 0..5000 {
                        let from = (t + i) % 10;
                        let to = (t * 3 + i * 7 + 1) % 10;
                        let _ = bank.transfer(from, to, 1 + (i % 50) as i64);
                    }
                });
            }
            s.spawn(|| {
                while !done.load(Ordering::Relaxed) {
                    assert_eq!(bank.total(), 10_000);
                }
            });
            // Let the auditor run while the transfers are going on, then stop it.
            for _ in 0..200 {
                assert_eq!(bank.total(), 10_000);
            }
            done.store(true, Ordering::Relaxed);
        });
        assert_eq!(bank.total(), 10_000);
    }
}
