// ==================================================================
// PRACTICE 0895: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0895_print_foobar_alternately.md
// Run the tests:          cargo test practice::p0895
// ==================================================================

//! # Problem 0895: Print FooBar Alternately
//!
//! - **Difficulty:** Medium
//! - **Topics:** strict alternation between two threads with a shared boolean turn and a `Condvar`,
//!   callbacks passed as `impl Fn()` and called many times
//! - **Source:** Adapted from [LeetCode 1115. Print FooBar Alternately](https://leetcode.com/problems/print-foobar-alternately/)
//!
//! ## Problem
//!
//! LeetCode has no Rust template for this problem, so the API is defined here.
//!
//! A `FooBar` is created with a number `n` and shared by two threads: thread A calls `foo` once and
//! thread B calls `bar` once. `foo` must call `print_foo` exactly `n` times and `bar` must call
//! `print_bar` exactly `n` times, interleaved so that the output is `"foobar"` repeated `n` times:
//! `foo` always goes first, and the two strictly alternate.
//!
//! ```text
//! pub struct FooBar { /* your fields */ }
//! impl FooBar {
//!     pub fn new(n: usize) -> Self;
//!     pub fn foo(&self, print_foo: impl Fn());
//!     pub fn bar(&self, print_bar: impl Fn());
//! }
//! ```
//!
//! ## Examples
//!
//! ```text
//! n = 1  -> "foobar"
//! n = 2  -> "foobarfoobar"        (whichever thread starts first)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`
//!
//! ## Hints
//!
//! 1. A single `Mutex<bool>` saying "it is foo's turn" is all the state you need.
//! 2. Each method loops `n` times: wait for its turn, print, hand the turn to the other method,
//!    notify.

use std::sync::{Condvar, Mutex, PoisonError};

pub struct FooBar {}

impl FooBar {
    pub fn new(n: usize) -> Self {
        todo!("Write your solution here")
    }

    pub fn foo(&self, print_foo: impl Fn()) {
        todo!("Write your solution here")
    }

    pub fn bar(&self, print_bar: impl Fn()) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn run(n: usize, bar_starts_first: bool) -> String {
        let fb = FooBar::new(n);
        let out = Mutex::new(String::new());
        let print = |s: &str| out.lock().unwrap().push_str(s);
        thread::scope(|s| {
            if bar_starts_first {
                s.spawn(|| fb.bar(|| print("bar")));
                s.spawn(|| fb.foo(|| print("foo")));
            } else {
                s.spawn(|| fb.foo(|| print("foo")));
                s.spawn(|| fb.bar(|| print("bar")));
            }
        });
        out.into_inner().unwrap()
    }

    #[test]
    fn example_n_1() {
        assert_eq!(run(1, false), "foobar");
    }

    #[test]
    fn example_n_2() {
        assert_eq!(run(2, false), "foobarfoobar");
    }

    #[test]
    fn bar_thread_started_first() {
        assert_eq!(run(3, true), "foobarfoobarfoobar");
    }

    #[test]
    fn long_alternation() {
        assert_eq!(run(1000, true), "foobar".repeat(1000));
        for _ in 0..50 {
            assert_eq!(run(10, false), "foobar".repeat(10));
        }
    }
}
