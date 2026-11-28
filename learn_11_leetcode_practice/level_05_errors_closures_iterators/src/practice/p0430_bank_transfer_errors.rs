// ==================================================================
// PRACTICE 0430: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0430_bank_transfer_errors.md
// Run the tests:          cargo test practice::p0430
// ==================================================================

//! # Problem 0430: Bank Transfers with Typed Errors
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Result<T, BankError>` from `&mut self` methods, error variants with data, validate-then-mutate, `iter().position`
//! - **Source:** Original · [The Rust Book 9.3: Creating custom types for validation](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation)
//!
//! ## Problem
//!
//! A tiny bank keeps a list of accounts. Every operation can fail for a business reason, so every
//! method returns a `Result` whose error says exactly what went wrong. A failed operation must
//! **not change anything** (no money may disappear halfway through a transfer).
//!
//! Implement `Bank` with:
//!
//! - `Bank::new(accounts: Vec<Account>) -> Bank`.
//! - `balance(&self, id: u32) -> Result<u64, BankError>`.
//! - `deposit(&mut self, id: u32, amount: u64) -> Result<u64, BankError>`: returns the new balance.
//!   Checks: `amount > 0` (`ZeroAmount`), the account exists (`AccountNotFound(id)`), it is not
//!   frozen (`Frozen(id)`).
//! - `withdraw(&mut self, id: u32, amount: u64) -> Result<u64, BankError>`: same checks, then the
//!   balance must be at least `amount`, otherwise
//!   `InsufficientFunds { account, balance, needed }`.
//! - `transfer(&mut self, from: u32, to: u32, amount: u64) -> Result<(), BankError>` with checks in
//!   this order: `ZeroAmount`; `SameAccount(from)` when `from == to`; `AccountNotFound(from)`;
//!   `AccountNotFound(to)`; `Frozen(from)`; `Frozen(to)`; `InsufficientFunds` for `from`.
//!
//! And `Display` for `BankError`:
//!
//! ```text
//! AccountNotFound(7)                                         -> "account 7 does not exist"
//! InsufficientFunds { account: 1, balance: 50, needed: 80 }  -> "account 1 has 50, needs 80"
//! SameAccount(3)                                             -> "cannot transfer from account 3 to itself"
//! ZeroAmount                                                 -> "amount must be positive"
//! Frozen(2)                                                  -> "account 2 is frozen"
//! ```
//!
//! ## Examples
//!
//! ```text
//! bank = [Account { id: 1, balance: 100, frozen: false }, Account { id: 2, balance: 0, frozen: false }]
//! bank.transfer(1, 2, 30)  -> Ok(())          balances: 1 -> 70, 2 -> 30
//! bank.transfer(2, 1, 50)  -> Err(InsufficientFunds { account: 2, balance: 30, needed: 50 })
//! bank.withdraw(9, 5)      -> Err(AccountNotFound(9))
//! bank.deposit(1, 0)       -> Err(ZeroAmount)
//! ```
//!
//! ## Constraints
//!
//! - Balances and amounts stay below `10^12`, so additions never overflow a `u64`.
//!
//! ## Hints
//!
//! 1. A private helper `fn index_of(&self, id: u32) -> Result<usize, BankError>` removes a lot of
//!    repetition.
//! 2. Do **all** checks first, and only then change the balances.

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BankError {
    AccountNotFound(u32),
    InsufficientFunds {
        account: u32,
        balance: u64,
        needed: u64,
    },
    SameAccount(u32),
    ZeroAmount,
    Frozen(u32),
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for BankError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub id: u32,
    pub balance: u64,
    pub frozen: bool,
}

pub struct Bank {}

impl Bank {
    pub fn new(accounts: Vec<Account>) -> Bank {
        todo!("Write your solution here")
    }

    pub fn balance(&self, id: u32) -> Result<u64, BankError> {
        todo!("Write your solution here")
    }

    pub fn deposit(&mut self, id: u32, amount: u64) -> Result<u64, BankError> {
        todo!("Write your solution here")
    }

    pub fn withdraw(&mut self, id: u32, amount: u64) -> Result<u64, BankError> {
        todo!("Write your solution here")
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: u64) -> Result<(), BankError> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bank() -> Bank {
        Bank::new(vec![
            Account {
                id: 1,
                balance: 100,
                frozen: false,
            },
            Account {
                id: 2,
                balance: 0,
                frozen: false,
            },
            Account {
                id: 3,
                balance: 500,
                frozen: true,
            },
        ])
    }

    #[test]
    fn deposit_and_withdraw() {
        let mut b = bank();
        assert_eq!(b.deposit(2, 40), Ok(40));
        assert_eq!(b.withdraw(1, 100), Ok(0));
        assert_eq!(b.balance(1), Ok(0));
        assert_eq!(b.deposit(1, 0), Err(BankError::ZeroAmount));
        assert_eq!(b.withdraw(9, 5), Err(BankError::AccountNotFound(9)));
        assert_eq!(b.deposit(3, 5), Err(BankError::Frozen(3)));
        assert_eq!(
            b.withdraw(2, 41),
            Err(BankError::InsufficientFunds {
                account: 2,
                balance: 40,
                needed: 41
            })
        );
        assert_eq!(b.balance(2), Ok(40));
    }

    #[test]
    fn successful_transfer() {
        let mut b = bank();
        assert_eq!(b.transfer(1, 2, 30), Ok(()));
        assert_eq!(b.balance(1), Ok(70));
        assert_eq!(b.balance(2), Ok(30));
        assert_eq!(b.transfer(1, 2, 70), Ok(()));
        assert_eq!(b.balance(1), Ok(0));
    }

    #[test]
    fn failed_transfers_change_nothing() {
        let mut b = bank();
        b.transfer(1, 2, 30).unwrap();
        assert_eq!(
            b.transfer(2, 1, 50),
            Err(BankError::InsufficientFunds {
                account: 2,
                balance: 30,
                needed: 50
            })
        );
        assert_eq!(b.transfer(1, 3, 10), Err(BankError::Frozen(3)));
        assert_eq!(b.transfer(3, 1, 10), Err(BankError::Frozen(3)));
        assert_eq!(b.balance(1), Ok(70));
        assert_eq!(b.balance(2), Ok(30));
        assert_eq!(b.balance(3), Ok(500));
    }

    #[test]
    fn transfer_check_order() {
        let mut b = bank();
        assert_eq!(b.transfer(8, 8, 0), Err(BankError::ZeroAmount));
        assert_eq!(b.transfer(8, 8, 1), Err(BankError::SameAccount(8)));
        assert_eq!(b.transfer(8, 9, 1), Err(BankError::AccountNotFound(8)));
        assert_eq!(b.transfer(1, 9, 1), Err(BankError::AccountNotFound(9)));
        assert_eq!(b.transfer(3, 9, 1), Err(BankError::AccountNotFound(9)));
        assert_eq!(b.transfer(3, 2, 1_000), Err(BankError::Frozen(3)));
    }

    #[test]
    fn display_messages() {
        assert_eq!(
            BankError::AccountNotFound(7).to_string(),
            "account 7 does not exist"
        );
        assert_eq!(
            BankError::InsufficientFunds {
                account: 1,
                balance: 50,
                needed: 80
            }
            .to_string(),
            "account 1 has 50, needs 80"
        );
        assert_eq!(
            BankError::SameAccount(3).to_string(),
            "cannot transfer from account 3 to itself"
        );
        assert_eq!(BankError::ZeroAmount.to_string(), "amount must be positive");
        assert_eq!(BankError::Frozen(2).to_string(), "account 2 is frozen");
    }
}
