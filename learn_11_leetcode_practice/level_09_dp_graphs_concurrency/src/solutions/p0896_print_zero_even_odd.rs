//! # Problem 0896: Print Zero Even Odd
//!
//! - **Difficulty:** Medium
//! - **Topics:** coordinating three threads with a small state struct behind one `Mutex`, a
//!   `Condvar` predicate that depends on two fields, `step_by`
//! - **Source:** Adapted from [LeetCode 1116. Print Zero Even Odd](https://leetcode.com/problems/print-zero-even-odd/)
//!
//! ## Problem
//!
//! LeetCode has no Rust template for this problem, so the API is defined here.
//!
//! A `ZeroEvenOdd` is created with a number `n` and shared by three threads: A calls `zero`, B calls
//! `even` and C calls `odd` (each exactly once). Together they must produce the sequence
//! `0 1 0 2 0 3 ... 0 n` through `print_number`:
//!
//! - `zero` prints `0` exactly `n` times, once before each number;
//! - `odd` prints the odd numbers `1, 3, 5, ...` up to `n`;
//! - `even` prints the even numbers `2, 4, 6, ...` up to `n`.
//!
//! ```text
//! pub struct ZeroEvenOdd { /* your fields */ }
//! impl ZeroEvenOdd {
//!     pub fn new(n: u32) -> Self;
//!     pub fn zero(&self, print_number: impl Fn(u32));
//!     pub fn even(&self, print_number: impl Fn(u32));
//!     pub fn odd(&self, print_number: impl Fn(u32));
//! }
//! ```
//!
//! ## Examples
//!
//! ```text
//! n = 2  -> printed numbers 0 1 0 2      (as a string: "0102")
//! n = 5  -> "0102030405"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`
//!
//! ## Hints
//!
//! 1. Shared state: `next` (the next non-zero number to print) and `zero_turn` (whether a `0` comes
//!    next). Put both in one struct inside one `Mutex`.
//! 2. `zero` waits for `zero_turn`; `odd` waits for `!zero_turn && next` odd; `even` waits for
//!    `!zero_turn && next` even. After printing, each updates the state and calls `notify_all`.
//! 3. Each thread knows in advance how many times it prints: `odd` loops over
//!    `(1..=n).step_by(2)`, `even` over `(2..=n).step_by(2)`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** state `{ next: 1, zero_turn: true }`. `zero` repeats `n` times: wait for
//! `zero_turn`, print `0`, clear `zero_turn`. `odd` and `even` share a helper that, for each of its
//! numbers `k`, waits until `!zero_turn && next == k`, prints `k`, increments `next` and sets
//! `zero_turn` again. Waiting for the exact `k` makes the predicate obviously correct.
//!
//! **Complexity:** `O(n)` prints; each of the `2n` state changes wakes all three threads, so every thread
//! is woken `O(n)` times. `O(1)` extra space.
//!
//! **Rust concepts:**
//! - A private `struct State` keeps related fields under a single lock, so they always change
//!   together.
//! - `(start..=n).step_by(2)` yields each thread's numbers without any arithmetic in the loop.
// @solution-end

use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

pub struct ZeroEvenOdd {
    // @solution-start remove
    n: u32,
    state: Mutex<State>,
    changed: Condvar,
    // @solution-end
}

// @solution-start remove
struct State {
    next: u32,
    zero_turn: bool,
}
// @solution-end

impl ZeroEvenOdd {
    pub fn new(n: u32) -> Self {
        // @solution-start
        Self {
            n,
            state: Mutex::new(State {
                next: 1,
                zero_turn: true,
            }),
            changed: Condvar::new(),
        }
        // @solution-end
    }

    // @solution-start remove
    fn wait_for(&self, ready: impl Fn(&State) -> bool) -> MutexGuard<'_, State> {
        let guard = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        self.changed
            .wait_while(guard, |s| !ready(s))
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn print_numbers(&self, first: u32, print_number: impl Fn(u32)) {
        for k in (first..=self.n).step_by(2) {
            let mut state = self.wait_for(|s| !s.zero_turn && s.next == k);
            print_number(k);
            state.next += 1;
            state.zero_turn = true;
            self.changed.notify_all();
        }
    }
    // @solution-end

    pub fn zero(&self, print_number: impl Fn(u32)) {
        // @solution-start
        for _ in 0..self.n {
            let mut state = self.wait_for(|s| s.zero_turn);
            print_number(0);
            state.zero_turn = false;
            self.changed.notify_all();
        }
        // @solution-end
    }

    pub fn even(&self, print_number: impl Fn(u32)) {
        // @solution-start
        self.print_numbers(2, print_number);
        // @solution-end
    }

    pub fn odd(&self, print_number: impl Fn(u32)) {
        // @solution-start
        self.print_numbers(1, print_number);
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn run(n: u32) -> String {
        let zeo = ZeroEvenOdd::new(n);
        let out = Mutex::new(String::new());
        let print = |x: u32| out.lock().unwrap().push_str(&x.to_string());
        thread::scope(|s| {
            s.spawn(|| zeo.even(print));
            s.spawn(|| zeo.odd(print));
            s.spawn(|| zeo.zero(print));
        });
        out.into_inner().unwrap()
    }

    fn expected(n: u32) -> String {
        (1..=n).map(|k| format!("0{k}")).collect()
    }

    #[test]
    fn example_n_2() {
        assert_eq!(run(2), "0102");
    }

    #[test]
    fn example_n_5() {
        assert_eq!(run(5), "0102030405");
    }

    #[test]
    fn single_number() {
        assert_eq!(run(1), "01");
    }

    #[test]
    fn multi_digit_numbers() {
        assert_eq!(run(12), expected(12));
        assert_eq!(run(1000), expected(1000));
        for _ in 0..30 {
            assert_eq!(run(7), expected(7));
        }
    }
}
