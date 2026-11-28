// ==================================================================
// PRACTICE 0297: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0297_design_an_atm_machine.md
// Run the tests:          cargo test practice::p0297
// ==================================================================

//! # Problem 0297: Design an ATM Machine
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct with fixed-size arrays, greedy plan computed before mutating, `zip` over parallel arrays, `[-1]` sentinel
//! - **Source:** [LeetCode 2241. Design an ATM Machine](https://leetcode.com/problems/design-an-atm-machine/)
//!
//! ## Problem
//!
//! An ATM holds banknotes of five values: `$20`, `$50`, `$100`, `$200` and `$500`, initially none.
//! Implement the `ATM` struct:
//!
//! - `ATM::new() -> Self`: an empty machine;
//! - `deposit(&mut self, banknotes_count: Vec<i32>)`: add `banknotes_count[i]` notes of the `i`-th
//!   value (in the order 20, 50, 100, 200, 500);
//! - `withdraw(&mut self, amount: i32) -> Vec<i32>`: hand out `amount` dollars. The machine always
//!   takes as many notes of the **largest** value as possible (without exceeding the remaining
//!   amount and the notes available), then continues with the next value down. If this procedure
//!   does not reach exactly `amount`, the withdrawal is **rejected**: nothing is handed out, the
//!   stock does not change, and the result is `[-1]`. Otherwise the result is the number of notes
//!   of each value handed out, in the same order as `deposit`.
//!
//! The greedy rule is part of the specification: for example with one `$50` and three `$20` notes,
//! a request for `$60` is rejected (the machine takes the `$50` first and cannot pay `$10`), even
//! though three `$20` notes would work.
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`.
//!
//! ## Examples
//!
//! ```text
//! let mut atm = ATM::new();
//! atm.deposit(vec![0, 0, 1, 2, 1]);   // one $100, two $200, one $500
//! atm.withdraw(600)  => [0, 0, 1, 0, 1]   ($500 + $100)
//! atm.deposit(vec![0, 1, 0, 1, 1]);
//! atm.withdraw(600)  => [-1]              ($500, then $50 leaves $50 that cannot be paid)
//! atm.withdraw(550)  => [0, 1, 0, 0, 1]
//! ```
//!
//! ## Constraints
//!
//! - `banknotes_count.len() == 5`, `0 <= banknotes_count[i] <= 10^9`, `1 <= amount <= 10^9`.
//! - At most 5000 calls in total. Stocks can exceed `i32`: use `i64`.
//!
//! ## Hints
//!
//! 1. Compute the plan first in a local array, from the largest value to the smallest:
//!    `take = min(stock[i], remaining / value[i])`.
//! 2. Only if `remaining == 0` at the end, subtract the plan from the stock.

/// An ATM with five kinds of banknotes.
// The name and the argument-less `new` come from LeetCode's template.
#[allow(clippy::upper_case_acronyms)]
pub struct ATM {}

impl ATM {
    /// An empty machine.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    /// Adds banknotes, in the order 20, 50, 100, 200, 500.
    pub fn deposit(&mut self, banknotes_count: Vec<i32>) {
        todo!("Write your solution here")
    }

    /// Withdraws `amount` greedily, or returns `[-1]` without changes.
    pub fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut atm = ATM::new();
        atm.deposit(vec![0, 0, 1, 2, 1]);
        assert_eq!(atm.withdraw(600), vec![0, 0, 1, 0, 1]);
        atm.deposit(vec![0, 1, 0, 1, 1]);
        assert_eq!(atm.withdraw(600), vec![-1]);
        assert_eq!(atm.withdraw(550), vec![0, 1, 0, 0, 1]);
    }

    #[test]
    fn greedy_rejection_from_the_statement() {
        let mut atm = ATM::new();
        atm.deposit(vec![3, 1, 0, 0, 0]);
        assert_eq!(atm.withdraw(60), vec![-1]);
        // Nothing changed: 50 + 20 still works.
        assert_eq!(atm.withdraw(70), vec![1, 1, 0, 0, 0]);
        assert_eq!(atm.withdraw(40), vec![2, 0, 0, 0, 0]);
        assert_eq!(atm.withdraw(20), vec![-1]);
    }

    #[test]
    fn empty_machine() {
        let mut atm = ATM::new();
        assert_eq!(atm.withdraw(20), vec![-1]);
    }

    #[test]
    fn huge_stock() {
        let mut atm = ATM::new();
        atm.deposit(vec![1_000_000_000; 5]);
        atm.deposit(vec![1_000_000_000; 5]);
        assert_eq!(atm.withdraw(1_000_000_000), vec![0, 0, 0, 0, 2_000_000]);
        assert_eq!(atm.withdraw(170), vec![1, 1, 1, 0, 0]);
    }
}
