// ==================================================================
// PRACTICE 0598: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0598_state_machine_associated_types.md
// Run the tests:          cargo test practice::p0598
// ==================================================================

//! # Problem 0598: A Generic State Machine
//!
//! - **Difficulty:** Medium
//! - **Topics:** a trait combining associated types (`State`, `Input`) with bounds and an associated constant (`START`), generic runners over `M: StateMachine`, `M: StateMachine<Input = char>`
//! - **Source:** Original · [The Rust Reference: associated items](https://doc.rust-lang.org/reference/items/associated-items.html)
//!
//! ## Problem
//!
//! Describe finite state machines with a trait and run any of them with the same generic code:
//!
//! ```text
//! pub trait StateMachine {
//!     type State: Copy + PartialEq + fmt::Debug;
//!     type Input;
//!     const START: Self::State;
//!     fn transition(state: Self::State, input: &Self::Input) -> Option<Self::State>;  // None: invalid input
//!     fn is_accepting(state: Self::State) -> bool;
//! }
//! ```
//!
//! Generic functions:
//!
//! - `run<M: StateMachine>(inputs: &[M::Input]) -> Result<M::State, (usize, M::State)>`: starts at
//!   `M::START` and applies every input. On the first invalid input it returns `Err((index, state
//!   before that input))`; otherwise `Ok(final_state)`.
//! - `accepts<M: StateMachine>(inputs: &[M::Input]) -> bool`: the run succeeds and ends in an
//!   accepting state.
//! - `accepts_text<M: StateMachine<Input = char>>(text: &str) -> bool`: the same for machines that read
//!   characters.
//!
//! Machines (all unit structs):
//!
//! 1. `Turnstile`: `State = Gate` (`Locked`, `Unlocked`), `Input = Action` (`Coin`, `Push`). A coin
//!    unlocks, a push locks again; other combinations keep the state. Starts and accepts in `Locked`.
//! 2. `EvenZeros`: `State = bool` ("even number of `'0'`s so far"), `Input = char`. `'0'` flips the
//!    state, `'1'` keeps it, any other character is invalid. Starts at `true`, accepts `true`.
//! 3. `DivisibleBy3`: `State = u8` (remainder), `Input = u8` (a decimal digit). Reading digit `d` moves
//!    from `r` to `(r * 10 + d) % 3`; a digit above 9 is invalid. Starts at `0`, accepts `0`.
//!
//! ## Examples
//!
//! ```text
//! run::<Turnstile>(&[Coin, Push, Push])     => Ok(Locked)
//! accepts::<Turnstile>(&[Coin])             => false
//! accepts_text::<EvenZeros>("1001")         => true
//! run::<EvenZeros>(&['0', 'x', '1'])        => Err((1, false))
//! accepts::<DivisibleBy3>(&[1, 2, 3])       => true     (123 = 3 * 41)
//! run::<DivisibleBy3>(&[4, 12])             => Err((1, 1))
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. In generic code the associated items are reached through the parameter: `M::START`,
//!    `M::transition(state, input)`, `M::is_accepting(state)`.
//! 2. `inputs.iter().enumerate()` gives the index needed for the error.
//! 3. `accepts_text` can collect the characters into a `Vec<char>` and call `accepts::<M>`.

use std::fmt;

/// A deterministic finite state machine.
pub trait StateMachine {
    type State: Copy + PartialEq + fmt::Debug;
    type Input;
    const START: Self::State;

    fn transition(state: Self::State, input: &Self::Input) -> Option<Self::State>;
    fn is_accepting(state: Self::State) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gate {
    Locked,
    Unlocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Coin,
    Push,
}

pub struct Turnstile;
pub struct EvenZeros;
pub struct DivisibleBy3;

impl StateMachine for Turnstile {
    type State = Gate;
    type Input = Action;
    const START: Gate = Gate::Locked;

    fn transition(state: Gate, input: &Action) -> Option<Gate> {
        todo!("Write your solution here")
    }

    fn is_accepting(state: Gate) -> bool {
        todo!("Write your solution here")
    }
}

impl StateMachine for EvenZeros {
    type State = bool;
    type Input = char;
    const START: bool = true;

    fn transition(state: bool, input: &char) -> Option<bool> {
        todo!("Write your solution here")
    }

    fn is_accepting(state: bool) -> bool {
        todo!("Write your solution here")
    }
}

impl StateMachine for DivisibleBy3 {
    type State = u8;
    type Input = u8;
    const START: u8 = 0;

    fn transition(state: u8, input: &u8) -> Option<u8> {
        todo!("Write your solution here")
    }

    fn is_accepting(state: u8) -> bool {
        todo!("Write your solution here")
    }
}

/// Runs a machine; `Err((index, state))` at the first invalid input.
pub fn run<M: StateMachine>(inputs: &[M::Input]) -> Result<M::State, (usize, M::State)> {
    todo!("Write your solution here")
}

/// Whether the machine runs successfully and ends in an accepting state.
pub fn accepts<M: StateMachine>(inputs: &[M::Input]) -> bool {
    todo!("Write your solution here")
}

/// `accepts` for machines reading characters.
pub fn accepts_text<M: StateMachine<Input = char>>(text: &str) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use Action::{Coin, Push};

    #[test]
    fn turnstile() {
        assert_eq!(run::<Turnstile>(&[Coin, Push, Push]), Ok(Gate::Locked));
        assert_eq!(run::<Turnstile>(&[Push, Coin, Coin]), Ok(Gate::Unlocked));
        assert!(!accepts::<Turnstile>(&[Coin]));
        assert!(accepts::<Turnstile>(&[]));
    }

    #[test]
    fn even_zeros() {
        assert!(accepts_text::<EvenZeros>("1001"));
        assert!(!accepts_text::<EvenZeros>("10"));
        assert!(accepts_text::<EvenZeros>(""));
        assert_eq!(run::<EvenZeros>(&['0', 'x', '1']), Err((1, false)));
        assert!(!accepts_text::<EvenZeros>("00a"));
    }

    #[test]
    fn divisible_by_three() {
        assert!(accepts::<DivisibleBy3>(&[1, 2, 3]));
        assert!(!accepts::<DivisibleBy3>(&[1, 0]));
        assert_eq!(run::<DivisibleBy3>(&[4, 12]), Err((1, 1)));
        let digits: Vec<u8> = "987654321987654321".bytes().map(|b| b - b'0').collect();
        assert!(accepts::<DivisibleBy3>(&digits)); // digit sum 90
    }

    #[test]
    fn generic_code_over_user_machines() {
        // Accepts strings of balanced parentheses up to depth 255.
        struct Parens;
        impl StateMachine for Parens {
            type State = u8;
            type Input = char;
            const START: u8 = 0;
            fn transition(depth: u8, c: &char) -> Option<u8> {
                match c {
                    '(' => depth.checked_add(1),
                    ')' => depth.checked_sub(1),
                    _ => None,
                }
            }
            fn is_accepting(depth: u8) -> bool {
                depth == 0
            }
        }
        assert!(accepts_text::<Parens>("(()(()))"));
        assert!(!accepts_text::<Parens>("(()"));
        assert_eq!(run::<Parens>(&['(', ')', ')']), Err((2, 0)));
    }
}
