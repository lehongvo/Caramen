// ==================================================================
// PRACTICE 0264: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0264_vending_machine.md
// Run the tests:          cargo test practice::p0264
// ==================================================================

//! # Problem 0264: Vending Machine
//!
//! - **Difficulty:** Medium
//! - **Topics:** input and output enums carrying data, a struct holding machine state, `&mut self` event handler, `get_mut` + `let else`
//! - **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
//!
//! ## Problem
//!
//! Simulate a vending machine with numbered slots. Each slot has a price (in cents) and a stock
//! count. The machine reacts to one `Input` at a time and answers with one `Output`:
//!
//! ```text
//! enum Coin   { Nickel, Dime, Quarter }                 // 5, 10 and 25 cents
//! enum Input  { Insert(Coin), Select(usize), Cancel }
//! enum Output { Credit(u32), Dispensed { slot: usize, change: u32 }, NeedMore { missing: u32 },
//!               SoldOut(usize), InvalidSlot(usize), Refund(u32), Nothing }
//! ```
//!
//! Implement `VendingMachine`:
//!
//! - `new(slots: Vec<(u32, u32)>) -> VendingMachine`: slot `i` has price `slots[i].0` and stock
//!   `slots[i].1`; the credit starts at `0`;
//! - `credit(&self) -> u32` and `stock(&self, slot: usize) -> Option<u32>`;
//! - `handle(&mut self, input: Input) -> Output` with these rules:
//!   - `Insert(coin)`: add the coin's value to the credit, answer `Credit(new_credit)`;
//!   - `Select(slot)`: if the slot does not exist, `InvalidSlot(slot)`; if its stock is `0`,
//!     `SoldOut(slot)`; if the credit is below the price, `NeedMore { missing }`; otherwise take one
//!     item, return the rest of the credit as change, reset the credit to `0` and answer
//!     `Dispensed { slot, change }`. The credit is kept in the three failure cases;
//!   - `Cancel`: `Refund(credit)` and reset the credit, or `Nothing` if the credit is `0`.
//!
//! ## Examples
//!
//! ```text
//! let mut vm = VendingMachine::new(vec![(65, 2), (100, 0)]);
//! vm.handle(Insert(Quarter))  => Credit(25)
//! vm.handle(Select(0))        => NeedMore { missing: 40 }
//! vm.handle(Insert(Quarter))  => Credit(50)
//! vm.handle(Insert(Quarter))  => Credit(75)
//! vm.handle(Select(0))        => Dispensed { slot: 0, change: 10 }
//! vm.handle(Select(1))        => SoldOut(1)
//! vm.handle(Select(7))        => InvalidSlot(7)
//! vm.handle(Insert(Dime))     => Credit(10)
//! vm.handle(Cancel)           => Refund(10)
//! vm.handle(Cancel)           => Nothing
//! ```
//!
//! ## Constraints
//!
//! - At most `100` slots and `10^4` inputs; credits fit in a `u32`.
//!
//! ## Hints
//!
//! 1. Give `Coin` a `cents(self) -> u32` method.
//! 2. In the `Select` arm, `let Some((price, stock)) = self.slots.get_mut(slot) else { return ... };`
//!    gives mutable access to one slot or exits with `InvalidSlot`.

/// A coin the machine accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    /// The coin's value in cents.
    pub fn cents(self) -> u32 {
        todo!("Write your solution here")
    }
}

/// Something the customer does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Insert(Coin),
    Select(usize),
    Cancel,
}

/// The machine's answer to one input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Output {
    Credit(u32),
    Dispensed { slot: usize, change: u32 },
    NeedMore { missing: u32 },
    SoldOut(usize),
    InvalidSlot(usize),
    Refund(u32),
    Nothing,
}

/// A vending machine with priced, stocked slots.
#[derive(Debug, Clone)]
pub struct VendingMachine {}

impl VendingMachine {
    /// A machine with `(price, stock)` per slot and no credit.
    pub fn new(slots: Vec<(u32, u32)>) -> VendingMachine {
        todo!("Write your solution here")
    }

    /// The current credit in cents.
    pub fn credit(&self) -> u32 {
        todo!("Write your solution here")
    }

    /// The stock of a slot, or `None` if it does not exist.
    pub fn stock(&self, slot: usize) -> Option<u32> {
        todo!("Write your solution here")
    }

    /// Reacts to one input.
    pub fn handle(&mut self, input: Input) -> Output {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::Coin::*;
    use super::Input::*;
    use super::*;

    #[test]
    fn example_session() {
        let mut vm = VendingMachine::new(vec![(65, 2), (100, 0)]);
        assert_eq!(vm.handle(Insert(Quarter)), Output::Credit(25));
        assert_eq!(vm.handle(Select(0)), Output::NeedMore { missing: 40 });
        assert_eq!(vm.handle(Insert(Quarter)), Output::Credit(50));
        assert_eq!(vm.handle(Insert(Quarter)), Output::Credit(75));
        assert_eq!(
            vm.handle(Select(0)),
            Output::Dispensed {
                slot: 0,
                change: 10
            }
        );
        assert_eq!(vm.handle(Select(1)), Output::SoldOut(1));
        assert_eq!(vm.handle(Select(7)), Output::InvalidSlot(7));
        assert_eq!(vm.handle(Insert(Dime)), Output::Credit(10));
        assert_eq!(vm.handle(Cancel), Output::Refund(10));
        assert_eq!(vm.handle(Cancel), Output::Nothing);
    }

    #[test]
    fn credit_is_kept_on_failures() {
        let mut vm = VendingMachine::new(vec![(30, 0), (10, 1)]);
        vm.handle(Insert(Nickel));
        vm.handle(Select(0));
        vm.handle(Select(9));
        assert_eq!(vm.credit(), 5);
        assert_eq!(vm.handle(Select(1)), Output::NeedMore { missing: 5 });
        vm.handle(Insert(Nickel));
        assert_eq!(
            vm.handle(Select(1)),
            Output::Dispensed { slot: 1, change: 0 }
        );
        assert_eq!(vm.credit(), 0);
    }

    #[test]
    fn stock_goes_down_until_sold_out() {
        let mut vm = VendingMachine::new(vec![(5, 2)]);
        for _ in 0..2 {
            vm.handle(Insert(Nickel));
            assert!(matches!(vm.handle(Select(0)), Output::Dispensed { .. }));
        }
        assert_eq!(vm.stock(0), Some(0));
        assert_eq!(vm.stock(1), None);
        vm.handle(Insert(Nickel));
        assert_eq!(vm.handle(Select(0)), Output::SoldOut(0));
    }

    #[test]
    fn many_coins() {
        let mut vm = VendingMachine::new(vec![(100_000, 1)]);
        for _ in 0..9_999 {
            vm.handle(Insert(Quarter));
        }
        assert_eq!(vm.credit(), 249_975);
        assert_eq!(
            vm.handle(Select(0)),
            Output::Dispensed {
                slot: 0,
                change: 149_975
            }
        );
    }
}
