//! # Problem 0207: A Bank Account with `&mut self`
//!
//! - **Difficulty:** Easy
//! - **Topics:** methods with `&mut self`, private fields, getters, keeping an invariant
//! - **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
//!
//! ## Problem
//!
//! Model a simple bank account. Its fields are **private**: code outside the module can only change
//! the balance through your methods, so the balance can never become negative. Choose the private
//! fields yourself (for example the owner's name, the balance in cents and a transaction counter).
//!
//! Implement on `Account`:
//!
//! - `Account::new(owner: &str) -> Self`: a new account with balance `0`.
//! - `owner(&self) -> &str` and `balance(&self) -> u64` (getters).
//! - `deposit(&mut self, amount: u64)`: adds `amount` to the balance.
//! - `withdraw(&mut self, amount: u64) -> bool`: if the balance is at least `amount`, subtracts it
//!   and returns `true`; otherwise changes nothing and returns `false`.
//! - `transaction_count(&self) -> usize`: the number of **successful** deposits and withdrawals so
//!   far (a deposit of `0` still counts; a refused withdrawal does not).
//!
//! And one free function that needs two mutable borrows at the same time:
//!
//! - `transfer(from: &mut Account, to: &mut Account, amount: u64) -> bool`: moves `amount` from
//!   `from` to `to` if `from` has enough money (counts as one withdrawal on `from` and one deposit
//!   on `to`); otherwise changes nothing and returns `false`.
//!
//! ## Examples
//!
//! ```text
//! let mut a = Account::new("Ana");
//! a.deposit(100);          // balance 100
//! a.withdraw(30)  => true  // balance 70
//! a.withdraw(500) => false // balance still 70
//! a.transaction_count() => 2
//!
//! let mut b = Account::new("Bo");
//! transfer(&mut a, &mut b, 50) => true   // a: 20, b: 50
//! transfer(&mut a, &mut b, 50) => false  // nothing changes
//! ```
//!
//! ## Constraints
//!
//! - Amounts are in cents, `0 <= amount <= 10^12`; balances never exceed `u64::MAX`.
//!
//! ## Hints
//!
//! 1. A method that changes the struct must take `&mut self`; the caller then needs a `let mut`
//!    binding (or a `&mut Account`).
//! 2. `transfer` can reuse your methods: try `from.withdraw(amount)` first and only deposit when it
//!    succeeded.
//! 3. Getters that return `&str` borrow from `self`: `&self.owner` coerces from `&String` to `&str`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** store the owner, the balance and a counter. Every successful operation updates the
//! counter. `withdraw` checks before subtracting, which keeps the invariant `balance >= 0` (it is a
//! `u64`, so going below zero would even panic in debug builds). `transfer` composes the two methods.
//!
//! **Rust concepts:**
//! - Private fields plus public methods are Rust's way to protect invariants: other modules cannot
//!   write `account.balance = 1_000_000`.
//! - `&mut self` gives exclusive access for the duration of the call.
//! - `transfer(&mut a, &mut b, ..)` compiles because `a` and `b` are different variables; the
//!   borrow checker would reject `transfer(&mut a, &mut a, ..)` (error E0499).
//! - `if !from.withdraw(amount) { return false; }` uses the `bool` result as a guard.
// @solution-end

/// A bank account whose balance can never go below zero.
#[derive(Debug)]
pub struct Account {
    // @solution-start remove
    owner: String,
    balance: u64,
    transactions: usize,
    // @solution-end
}

impl Account {
    /// Opens an empty account.
    pub fn new(owner: &str) -> Self {
        // @solution-start
        Self {
            owner: owner.to_string(),
            balance: 0,
            transactions: 0,
        }
        // @solution-end
    }

    /// The owner's name.
    pub fn owner(&self) -> &str {
        // @solution-start
        &self.owner
        // @solution-end
    }

    /// The current balance in cents.
    pub fn balance(&self) -> u64 {
        // @solution-start
        self.balance
        // @solution-end
    }

    /// Adds money.
    pub fn deposit(&mut self, amount: u64) {
        // @solution-start
        self.balance += amount;
        self.transactions += 1;
        // @solution-end
    }

    /// Removes money if there is enough; returns whether it happened.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        // @solution-start
        if self.balance < amount {
            return false;
        }
        self.balance -= amount;
        self.transactions += 1;
        true
        // @solution-end
    }

    /// Number of successful deposits and withdrawals.
    pub fn transaction_count(&self) -> usize {
        // @solution-start
        self.transactions
        // @solution-end
    }
}

/// Moves `amount` from one account to another if possible.
pub fn transfer(from: &mut Account, to: &mut Account, amount: u64) -> bool {
    // @solution-start
    if !from.withdraw(amount) {
        return false;
    }
    to.deposit(amount);
    true
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_deposit_and_withdraw() {
        let mut a = Account::new("Ana");
        a.deposit(100);
        assert!(a.withdraw(30));
        assert!(!a.withdraw(500));
        assert_eq!(a.balance(), 70);
        assert_eq!(a.transaction_count(), 2);
    }

    #[test]
    fn example_transfer() {
        let mut a = Account::new("Ana");
        let mut b = Account::new("Bo");
        a.deposit(70);
        assert!(transfer(&mut a, &mut b, 50));
        assert_eq!((a.balance(), b.balance()), (20, 50));
        assert!(!transfer(&mut a, &mut b, 50));
        assert_eq!((a.balance(), b.balance()), (20, 50));
    }

    #[test]
    fn new_account_is_empty() {
        let a = Account::new("Cy");
        assert_eq!(a.owner(), "Cy");
        assert_eq!(a.balance(), 0);
        assert_eq!(a.transaction_count(), 0);
    }

    #[test]
    fn withdraw_exact_balance() {
        let mut a = Account::new("Di");
        a.deposit(42);
        assert!(a.withdraw(42));
        assert_eq!(a.balance(), 0);
        assert!(!a.withdraw(1));
        assert_eq!(a.transaction_count(), 2);
    }

    #[test]
    fn transfer_counts_on_both_sides() {
        let mut a = Account::new("Ed");
        let mut b = Account::new("Flo");
        a.deposit(10);
        assert!(transfer(&mut a, &mut b, 10));
        assert_eq!(a.transaction_count(), 2);
        assert_eq!(b.transaction_count(), 1);
        assert!(!transfer(&mut a, &mut b, 1));
        assert_eq!(b.transaction_count(), 1);
    }

    #[test]
    fn large_amounts() {
        let mut a = Account::new("Gus");
        for _ in 0..1000 {
            a.deposit(1_000_000_000_000);
        }
        assert_eq!(a.balance(), 1_000_000_000_000_000);
        assert!(a.withdraw(999_999_999_999_999));
        assert_eq!(a.balance(), 1);
    }
}
