// ==================================================================
// PRACTICE 0607: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0607_drop_order_logger.md
// Run the tests:          cargo test practice::p0607
// ==================================================================

//! # Problem 0607: Drop Order Logger
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Drop` trait, drop order of locals, fields and `Vec` items, `std::mem::drop`, `Option::take`
//! - **Source:** Original · [The Rust Book 15.3](https://doc.rust-lang.org/book/ch15-03-drop.html) · [Reference: destructors](https://doc.rust-lang.org/reference/destructors.html)
//!
//! ## Problem
//!
//! Rust runs a value's destructor at a predictable moment. The rules you will observe:
//!
//! - local variables are dropped at the end of their scope, in **reverse** order of declaration;
//! - the fields of a struct are dropped in **declaration** order, after the struct itself;
//! - the items of a `Vec` are dropped from index `0` upward;
//! - `std::mem::drop(value)` (in the prelude as `drop`) ends a value's life early;
//! - `let _ = expr;` does not bind anything, so the value is dropped immediately.
//!
//! A `Noisy` value writes a line into a shared log when it is dropped:
//!
//! ```text
//! pub type Log = Rc<RefCell<Vec<String>>>;
//!
//! pub struct Noisy {
//!     name: String,
//!     log: Log,
//! }
//! ```
//!
//! Implement:
//!
//! 1. `Noisy::new(name: &str, log: &Log) -> Noisy`, keeping its own `Rc` handle to the log;
//! 2. `impl Drop for Noisy`, which pushes exactly `"drop <name>"` (for example `"drop a"`) onto the
//!    log. A destructor must never panic;
//! 3. `drop_in_order(items: Vec<Noisy>, order: &[usize])`: drop `items[order[0]]`, then
//!    `items[order[1]]`, and so on. When the function returns, every item not listed in `order`
//!    has been dropped too, in increasing index order. `order` holds distinct valid indices.
//!
//! ## Examples
//!
//! ```text
//! {
//!     let _a = Noisy::new("a", &log);
//!     let _b = Noisy::new("b", &log);
//! }                                   // log: ["drop b", "drop a"]
//!
//! drop_in_order(vec![a, b, c, d], &[2, 0])
//!                                     // log: ["drop c", "drop a", "drop b", "drop d"]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= items.len() <= 100`
//!
//! ## Hints
//!
//! 1. `Drop::drop` takes `&mut self`, so you can read `self.name` and borrow the log mutably with
//!    `self.log.borrow_mut()`.
//! 2. You cannot move a single item out of a `Vec` by index and leave a hole. Turn the vector into
//!    a `Vec<Option<Noisy>>` first; then `slots[i].take()` moves the item out and leaves `None`.

use std::cell::RefCell;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct Noisy {
    name: String,
    log: Log,
}

impl Noisy {
    pub fn new(name: &str, log: &Log) -> Noisy {
        todo!("Write your solution here")
    }
}

impl Drop for Noisy {
    fn drop(&mut self) {
        // TODO: push "drop <name>" onto the log (never panic inside `drop`).
    }
}

pub fn drop_in_order(items: Vec<Noisy>, order: &[usize]) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locals_drop_in_reverse_order() {
        let log = Log::default();
        {
            let _a = Noisy::new("a", &log);
            let _b = Noisy::new("b", &log);
            assert!(log.borrow().is_empty());
        }
        assert_eq!(*log.borrow(), ["drop b", "drop a"]);
    }

    #[test]
    fn mem_drop_ends_a_life_early() {
        let log = Log::default();
        {
            let a = Noisy::new("a", &log);
            let _b = Noisy::new("b", &log);
            drop(a);
            assert_eq!(*log.borrow(), ["drop a"]);
        }
        assert_eq!(*log.borrow(), ["drop a", "drop b"]);
    }

    #[test]
    fn struct_fields_drop_in_declaration_order() {
        struct Pair {
            _first: Noisy,
            _second: Noisy,
        }
        let log = Log::default();
        let pair = Pair {
            _second: Noisy::new("second", &log),
            _first: Noisy::new("first", &log),
        };
        drop(pair);
        assert_eq!(*log.borrow(), ["drop first", "drop second"]);
    }

    #[test]
    fn vec_items_drop_from_the_front() {
        let log = Log::default();
        let items: Vec<Noisy> = ["x", "y", "z"]
            .iter()
            .map(|n| Noisy::new(n, &log))
            .collect();
        drop(items);
        assert_eq!(*log.borrow(), ["drop x", "drop y", "drop z"]);
    }

    #[test]
    fn underscore_pattern_drops_immediately() {
        let log = Log::default();
        let _ = Noisy::new("temp", &log);
        assert_eq!(*log.borrow(), ["drop temp"]);
        let mut slot = Noisy::new("old", &log);
        assert_eq!(slot.name, "old");
        // Assigning drops the value that is overwritten.
        slot = Noisy::new("new", &log);
        assert_eq!(*log.borrow(), ["drop temp", "drop old"]);
        drop(slot);
        assert_eq!(*log.borrow(), ["drop temp", "drop old", "drop new"]);
    }

    #[test]
    fn drop_in_order_example() {
        let log = Log::default();
        let items: Vec<Noisy> = ["a", "b", "c", "d"]
            .iter()
            .map(|n| Noisy::new(n, &log))
            .collect();
        drop_in_order(items, &[2, 0]);
        assert_eq!(*log.borrow(), ["drop c", "drop a", "drop b", "drop d"]);
    }

    #[test]
    fn drop_in_order_with_full_and_empty_orders() {
        let log = Log::default();
        let items: Vec<Noisy> = ["p", "q", "r"]
            .iter()
            .map(|n| Noisy::new(n, &log))
            .collect();
        drop_in_order(items, &[1, 2, 0]);
        assert_eq!(*log.borrow(), ["drop q", "drop r", "drop p"]);

        let log = Log::default();
        let items: Vec<Noisy> = ["p", "q"].iter().map(|n| Noisy::new(n, &log)).collect();
        drop_in_order(items, &[]);
        assert_eq!(*log.borrow(), ["drop p", "drop q"]);
    }
}
