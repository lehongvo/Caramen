//! # Problem 0619: Shared Bank Account
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Rc<RefCell<T>>`, several owners mutating one value, `Rc::strong_count`
//! - **Source:** Original · [The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
//!
//! ## Problem
//!
//! `Rc<T>` gives a value several owners but only shared (read-only) access. `RefCell<T>` gives
//! mutable access through a shared reference. Together, `Rc<RefCell<T>>` is a value that several
//! owners can all **change**.
//!
//! A family shares one bank account; each member has a card:
//!
//! ```text
//! pub struct Account {
//!     pub balance: i64,
//!     pub history: Vec<String>,
//! }
//! pub type SharedAccount = Rc<RefCell<Account>>;
//! ```
//!
//! Implement:
//!
//! - `open_account(initial: i64) -> SharedAccount`: a new account with an empty history;
//! - `Card::new(account: &SharedAccount, holder: &str) -> Card`: a card that co-owns the account;
//! - `deposit(&self, amount: i64)`: add `amount` and record `"<holder> +<amount>"`;
//! - `withdraw(&self, amount: i64) -> bool`: if the balance is at least `amount`, subtract it,
//!   record `"<holder> -<amount>"` and return `true`; otherwise change nothing and return `false`;
//! - `balance(&self) -> i64`.
//!
//! All amounts are positive.
//!
//! ## Examples
//!
//! ```text
//! let account = open_account(100);
//! let ana = Card::new(&account, "ana");
//! let bo = Card::new(&account, "bo");
//! ana.deposit(50);             // 150
//! bo.withdraw(120)  -> true    // 30
//! ana.withdraw(40)  -> false   // still 30
//! bo.balance()      -> 30
//! account.borrow().history == ["ana +50", "bo -120"]
//! Rc::strong_count(&account) == 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= amount <= 10^9`; at most `10^4` operations.
//!
//! ## Hints
//!
//! 1. `Card` stores `Rc::clone(account)` and the holder's name.
//! 2. Take **one** `borrow_mut()` guard at the start of a method and use it for both the balance
//!    and the history: `let mut account = self.account.borrow_mut();`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each card holds an `Rc` handle to the same `RefCell<Account>`. Every method borrows
//! the account for the duration of the method, mutably when it changes something.
//!
//! **Complexity:** `O(1)` per operation (amortized for the history push).
//!
//! **Rust concepts:**
//! - `Rc<RefCell<T>>`: `Rc` answers "who owns it?", `RefCell` answers "who may change it now?".
//! - A `RefMut` guard derefs to `&mut Account`, so `account.balance -= amount` works directly.
//! - Keeping each borrow inside one method makes a runtime borrow conflict impossible here.
// @solution-end

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Account {
    pub balance: i64,
    pub history: Vec<String>,
}

pub type SharedAccount = Rc<RefCell<Account>>;

pub fn open_account(initial: i64) -> SharedAccount {
    // @solution-start
    Rc::new(RefCell::new(Account {
        balance: initial,
        history: Vec::new(),
    }))
    // @solution-end
}

pub struct Card {
    // @solution-start remove
    account: SharedAccount,
    holder: String,
    // @solution-end
}

impl Card {
    pub fn new(account: &SharedAccount, holder: &str) -> Card {
        // @solution-start
        Card {
            account: Rc::clone(account),
            holder: holder.to_string(),
        }
        // @solution-end
    }

    pub fn deposit(&self, amount: i64) {
        // @solution-start
        let mut account = self.account.borrow_mut();
        account.balance += amount;
        account.history.push(format!("{} +{amount}", self.holder));
        // @solution-end
    }

    pub fn withdraw(&self, amount: i64) -> bool {
        // @solution-start
        let mut account = self.account.borrow_mut();
        if account.balance < amount {
            return false;
        }
        account.balance -= amount;
        account.history.push(format!("{} -{amount}", self.holder));
        true
        // @solution-end
    }

    pub fn balance(&self) -> i64 {
        // @solution-start
        self.account.borrow().balance
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_example() {
        let account = open_account(100);
        let ana = Card::new(&account, "ana");
        let bo = Card::new(&account, "bo");
        ana.deposit(50);
        assert!(bo.withdraw(120));
        assert!(!ana.withdraw(40));
        assert_eq!(bo.balance(), 30);
        assert_eq!(ana.balance(), 30);
        assert_eq!(account.borrow().history, vec!["ana +50", "bo -120"]);
        assert_eq!(Rc::strong_count(&account), 3);
    }

    #[test]
    fn dropping_cards_releases_ownership() {
        let account = open_account(0);
        let cards: Vec<Card> = (0..5)
            .map(|i| Card::new(&account, &i.to_string()))
            .collect();
        assert_eq!(Rc::strong_count(&account), 6);
        cards[3].deposit(7);
        drop(cards);
        assert_eq!(Rc::strong_count(&account), 1);
        assert_eq!(account.borrow().balance, 7);
        assert_eq!(account.borrow().history, vec!["3 +7"]);
    }

    #[test]
    fn exact_withdrawal_empties_the_account() {
        let account = open_account(10);
        let card = Card::new(&account, "cy");
        assert!(card.withdraw(10));
        assert_eq!(card.balance(), 0);
        assert!(!card.withdraw(1));
        assert_eq!(account.borrow().history, vec!["cy -10"]);
    }

    #[test]
    fn many_operations() {
        let account = open_account(0);
        let a = Card::new(&account, "a");
        let b = Card::new(&account, "b");
        for _ in 0..5_000 {
            a.deposit(2);
            assert!(b.withdraw(1));
        }
        assert_eq!(a.balance(), 5_000);
        assert_eq!(account.borrow().history.len(), 10_000);
    }
}
