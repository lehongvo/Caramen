//! # Problem 0902: Invariants Behind Private Fields
//!
//! - **Difficulty:** Medium
//! - **Topics:** private fields, validated constructors, getters, `pub(crate)`, all-or-nothing updates
//! - **Source:** Original · [The Rust Book 7.3](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#making-structs-and-enums-public)
//!
//! ## Problem
//!
//! Privacy is not only about hiding code: it lets a module **guarantee invariants**. Code outside the
//! `bank` module below can never build an `Account` by hand or change a balance directly, so every
//! rule enforced by `Bank`'s methods holds for every account that exists.
//!
//! Implement the `bank` module (decide the private fields yourself):
//!
//! - `Account` has getters `id(&self) -> u32`, `owner(&self) -> &str`, `balance(&self) -> u64`.
//! - `Bank::new()` creates an empty bank. Account ids start at `1` and increase by one.
//! - `open(&mut self, owner: &str, initial: u64) -> Result<u32, BankError>`: the owner name is
//!   trimmed; an empty name gives `BankError::InvalidOwner`. Returns the new account id.
//! - `deposit(&mut self, id, amount) -> Result<u64, BankError>`: returns the new balance.
//! - `withdraw(&mut self, id, amount) -> Result<u64, BankError>`: fails with
//!   `BankError::InsufficientFunds { balance, requested }` if `amount > balance`.
//! - `transfer(&mut self, from, to, amount) -> Result<(), BankError>`: `from == to` gives
//!   `BankError::SameAccount`. The transfer is **all or nothing**: on any error no balance changes.
//! - Every method that takes an id returns `BankError::UnknownAccount(id)` for a missing account
//!   (check `from` before `to`).
//! - `account(&self, id) -> Option<&Account>` and `total_balance(&self) -> u64`.
//! - `audit_log(&self) -> &[String]` is `pub(crate)`: one entry per **successful** operation, in
//!   order: `"open 1 Alice 100"`, `"deposit 1 50"`, `"withdraw 1 30"`, `"transfer 1 2 20"`.
//! - `last_operation(&self) -> Option<&str>` (public): the newest log entry, built on `audit_log`.
//!
//! ## Examples
//!
//! ```text
//! let mut bank = Bank::new();
//! bank.open("  Alice ", 100)   => Ok(1)        (owner stored as "Alice")
//! bank.open("Bob", 0)          => Ok(2)
//! bank.open("   ", 5)          => Err(InvalidOwner)
//! bank.withdraw(2, 1)          => Err(InsufficientFunds { balance: 0, requested: 1 })
//! bank.transfer(1, 2, 40)      => Ok(())       balances: 60 and 40
//! bank.transfer(1, 9, 1)       => Err(UnknownAccount(9))   (nothing changed)
//! bank.audit_log()             => ["open 1 Alice 100", "open 2 Bob 0", "transfer 1 2 40"]
//! ```
//!
//! ## Constraints
//!
//! - Amounts are `u64`; a deposit never overflows in the tests.
//! - At most `10^4` accounts.
//!
//! ## Hints
//!
//! 1. Store the accounts in a `Vec<Account>` (or a map) plus a `next_id` counter and the log.
//! 2. A private helper `fn index_of(&self, id: u32) -> Result<usize, BankError>` keeps every method
//!    short. Indices avoid holding two `&mut` borrows of the vector at once.
//! 3. For `transfer`, do **all** the checks before changing anything.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** accounts live in a `Vec` in id order, so the account with id `k` is at index `k - 1`.
//! Each method validates its input, then mutates, then appends to the log. `transfer` looks up both
//! indices and checks the funds first; only then does it subtract and add, so an error leaves the
//! bank untouched.
//!
//! **Rust concepts:**
//! - Fields without `pub` are private to the module, so `Account { id: 1, .. }` cannot be written
//!   outside `bank`: the only way to get an `Account` is through `Bank::open`.
//! - Getters returning `&str` and `u64` expose the data read-only.
//! - `pub(crate)` on `audit_log` keeps it usable by the rest of this crate (and its tests) without
//!   making it part of the public API.
//! - `impl Default` next to `new()` satisfies Clippy's `new_without_default` lint.
// @solution-end

pub mod bank {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum BankError {
        InvalidOwner,
        UnknownAccount(u32),
        SameAccount,
        InsufficientFunds { balance: u64, requested: u64 },
    }

    #[derive(Debug)]
    pub struct Account {
        // @solution-start remove
        id: u32,
        owner: String,
        balance: u64,
        // @solution-end
    }

    impl Account {
        pub fn id(&self) -> u32 {
            // @solution-start
            self.id
            // @solution-end
        }

        pub fn owner(&self) -> &str {
            // @solution-start
            &self.owner
            // @solution-end
        }

        pub fn balance(&self) -> u64 {
            // @solution-start
            self.balance
            // @solution-end
        }
    }

    #[derive(Debug)]
    pub struct Bank {
        // @solution-start remove
        accounts: Vec<Account>,
        log: Vec<String>,
        // @solution-end
    }

    impl Default for Bank {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Bank {
        pub fn new() -> Self {
            // @solution-start
            Bank {
                accounts: Vec::new(),
                log: Vec::new(),
            }
            // @solution-end
        }

        pub fn open(&mut self, owner: &str, initial: u64) -> Result<u32, BankError> {
            // @solution-start
            let owner = owner.trim();
            if owner.is_empty() {
                return Err(BankError::InvalidOwner);
            }
            // Ids are 1-based and accounts are never removed, so the next id is len + 1.
            let id = self.accounts.len() as u32 + 1;
            self.accounts.push(Account {
                id,
                owner: owner.to_string(),
                balance: initial,
            });
            self.log.push(format!("open {id} {owner} {initial}"));
            Ok(id)
            // @solution-end
        }

        pub fn deposit(&mut self, id: u32, amount: u64) -> Result<u64, BankError> {
            // @solution-start
            let index = self.index_of(id)?;
            let account = &mut self.accounts[index];
            account.balance += amount;
            let balance = account.balance;
            self.log.push(format!("deposit {id} {amount}"));
            Ok(balance)
            // @solution-end
        }

        pub fn withdraw(&mut self, id: u32, amount: u64) -> Result<u64, BankError> {
            // @solution-start
            let index = self.index_of(id)?;
            let account = &mut self.accounts[index];
            if amount > account.balance {
                return Err(BankError::InsufficientFunds {
                    balance: account.balance,
                    requested: amount,
                });
            }
            account.balance -= amount;
            let balance = account.balance;
            self.log.push(format!("withdraw {id} {amount}"));
            Ok(balance)
            // @solution-end
        }

        pub fn transfer(&mut self, from: u32, to: u32, amount: u64) -> Result<(), BankError> {
            // @solution-start
            let source = self.index_of(from)?;
            let target = self.index_of(to)?;
            if source == target {
                return Err(BankError::SameAccount);
            }
            let balance = self.accounts[source].balance;
            if amount > balance {
                return Err(BankError::InsufficientFunds {
                    balance,
                    requested: amount,
                });
            }
            self.accounts[source].balance -= amount;
            self.accounts[target].balance += amount;
            self.log.push(format!("transfer {from} {to} {amount}"));
            Ok(())
            // @solution-end
        }

        pub fn account(&self, id: u32) -> Option<&Account> {
            // @solution-start
            self.index_of(id).ok().map(|index| &self.accounts[index])
            // @solution-end
        }

        pub fn total_balance(&self) -> u64 {
            // @solution-start
            self.accounts.iter().map(|account| account.balance).sum()
            // @solution-end
        }

        pub(crate) fn audit_log(&self) -> &[String] {
            // @solution-start
            &self.log
            // @solution-end
        }

        pub fn last_operation(&self) -> Option<&str> {
            // @solution-start
            self.audit_log().last().map(String::as_str)
            // @solution-end
        }

        // @solution-start remove
        fn index_of(&self, id: u32) -> Result<usize, BankError> {
            let index = (id as usize).checked_sub(1);
            match index {
                Some(index) if index < self.accounts.len() => Ok(index),
                _ => Err(BankError::UnknownAccount(id)),
            }
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::bank::{Bank, BankError};

    #[test]
    fn open_trims_and_validates_owner() {
        let mut bank = Bank::new();
        assert_eq!(bank.open("  Alice ", 100), Ok(1));
        assert_eq!(bank.open("Bob", 0), Ok(2));
        assert_eq!(bank.open("   ", 5), Err(BankError::InvalidOwner));
        let alice = bank.account(1).unwrap();
        assert_eq!(
            (alice.id(), alice.owner(), alice.balance()),
            (1, "Alice", 100)
        );
        assert!(bank.account(3).is_none());
        assert!(bank.account(0).is_none());
    }

    #[test]
    fn deposit_and_withdraw() {
        let mut bank = Bank::default();
        bank.open("Alice", 100).unwrap();
        assert_eq!(bank.deposit(1, 50), Ok(150));
        assert_eq!(bank.withdraw(1, 30), Ok(120));
        assert_eq!(
            bank.withdraw(1, 121),
            Err(BankError::InsufficientFunds {
                balance: 120,
                requested: 121
            })
        );
        assert_eq!(bank.deposit(7, 1), Err(BankError::UnknownAccount(7)));
        assert_eq!(bank.account(1).unwrap().balance(), 120);
    }

    #[test]
    fn transfer_is_all_or_nothing() {
        let mut bank = Bank::new();
        bank.open("Alice", 100).unwrap();
        bank.open("Bob", 0).unwrap();
        assert_eq!(bank.transfer(1, 2, 40), Ok(()));
        assert_eq!(bank.transfer(1, 9, 1), Err(BankError::UnknownAccount(9)));
        assert_eq!(bank.transfer(9, 1, 1), Err(BankError::UnknownAccount(9)));
        assert_eq!(bank.transfer(2, 2, 1), Err(BankError::SameAccount));
        assert_eq!(
            bank.transfer(2, 1, 41),
            Err(BankError::InsufficientFunds {
                balance: 40,
                requested: 41
            })
        );
        assert_eq!(bank.account(1).unwrap().balance(), 60);
        assert_eq!(bank.account(2).unwrap().balance(), 40);
        assert_eq!(bank.total_balance(), 100);
    }

    #[test]
    fn audit_log_records_only_successes() {
        let mut bank = Bank::new();
        bank.open("  Alice ", 100).unwrap();
        bank.open("Bob", 0).unwrap();
        let _ = bank.open("", 1);
        let _ = bank.withdraw(2, 1);
        bank.transfer(1, 2, 40).unwrap();
        bank.deposit(2, 5).unwrap();
        bank.withdraw(1, 10).unwrap();
        assert_eq!(
            bank.audit_log(),
            [
                "open 1 Alice 100",
                "open 2 Bob 0",
                "transfer 1 2 40",
                "deposit 2 5",
                "withdraw 1 10"
            ]
        );
        assert_eq!(bank.last_operation(), Some("withdraw 1 10"));
        assert_eq!(Bank::new().last_operation(), None);
    }

    #[test]
    fn many_accounts_keep_the_total() {
        let mut bank = Bank::new();
        for i in 0..1_000 {
            assert_eq!(bank.open(&format!("user{i}"), 10), Ok(i + 1));
        }
        for id in 1..1_000 {
            bank.transfer(id, id + 1, 10).unwrap();
        }
        assert_eq!(bank.total_balance(), 10_000);
        assert_eq!(bank.account(1_000).unwrap().balance(), 20);
    }
}
