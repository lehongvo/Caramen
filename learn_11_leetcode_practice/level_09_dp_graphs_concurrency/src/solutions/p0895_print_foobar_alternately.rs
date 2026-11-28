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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `foo_turn: Mutex<bool>` starts as `true`. In each of its `n` iterations, `foo` waits
//! while `foo_turn` is `false`, prints, sets it to `false` and notifies; `bar` does the mirror
//! image. A private helper `take_turns(mine, print)` implements both.
//!
//! **Complexity:** `O(n)` time: `2n` callbacks, and every turn change wakes the other thread once.
//!
//! **Rust concepts:**
//! - `impl Fn()` (not `FnOnce`) because the callback is called `n` times.
//! - With only two threads waiting on the condvar, `notify_one` would also work, but
//!   `notify_all` stays correct if more waiters are ever added.
// @solution-end

use std::sync::{Condvar, Mutex, PoisonError};

pub struct FooBar {
    // @solution-start remove
    n: usize,
    foo_turn: Mutex<bool>,
    changed: Condvar,
    // @solution-end
}

impl FooBar {
    pub fn new(n: usize) -> Self {
        // @solution-start
        Self {
            n,
            foo_turn: Mutex::new(true),
            changed: Condvar::new(),
        }
        // @solution-end
    }

    // @solution-start remove
    /// Runs `print` `n` times, each time when `foo_turn == is_foo`, then passes the turn.
    fn take_turns(&self, is_foo: bool, print: impl Fn()) {
        for _ in 0..self.n {
            let guard = self.foo_turn.lock().unwrap_or_else(PoisonError::into_inner);
            let mut foo_turn = self
                .changed
                .wait_while(guard, |foo_turn| *foo_turn != is_foo)
                .unwrap_or_else(PoisonError::into_inner);
            print();
            *foo_turn = !is_foo;
            self.changed.notify_all();
        }
    }
    // @solution-end

    pub fn foo(&self, print_foo: impl Fn()) {
        // @solution-start
        self.take_turns(true, print_foo);
        // @solution-end
    }

    pub fn bar(&self, print_bar: impl Fn()) {
        // @solution-start
        self.take_turns(false, print_bar);
        // @solution-end
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
