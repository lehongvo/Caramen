// ==================================================================
// PRACTICE 0445: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0445_memoize_closure.md
// Run the tests:          cargo test practice::p0445
// ==================================================================

//! # Problem 0445: Memoize an Expensive Function
//!
//! - **Difficulty:** Medium
//! - **Topics:** storing a closure in a generic struct, calling a closure field `(self.func)(n)`, `HashMap::entry` + `or_insert_with`, disjoint field capture, returning a memoizing `impl FnMut`
//! - **Source:** Original · [The Rust Book 13.1: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html) · [std `HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
//!
//! ## Problem
//!
//! *Memoization* remembers the results of an expensive function so that asking again for the same
//! input is instant. The cache and the function can live together in a struct that is generic over
//! the closure type.
//!
//! Implement `Memo<F>` where `F: Fn(u64) -> u64`:
//!
//! - `Memo::new(func: F) -> Memo<F>`: an empty cache.
//! - `get(&mut self, n: u64) -> u64`: `func(n)`, computed at most once per distinct `n`.
//! - `calls(&self) -> usize`: how many times `func` has actually been run.
//! - `cached(&self) -> usize`: how many distinct inputs are stored.
//!
//! And the closure version, which hides the cache inside the returned closure:
//!
//! - `memoize<F: Fn(u64) -> u64>(func: F) -> impl FnMut(u64) -> u64`.
//!
//! ## Examples
//!
//! ```text
//! let mut slow_square = Memo::new(|n| n * n);
//! slow_square.get(4) -> 16    (computed)
//! slow_square.get(4) -> 16    (from the cache)
//! slow_square.get(5) -> 25    (computed)
//! slow_square.calls() -> 2,   slow_square.cached() -> 2
//!
//! let mut digits = memoize(|n| n.to_string().len() as u64);
//! digits(12345) -> 5
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. The struct keeps the closure in a field `func: F`; call it with `(self.func)(n)`. Without the
//!    parentheses, `self.func(n)` would look for a *method* named `func`.
//! 2. `self.cache.entry(n).or_insert_with(|| ...)` computes the value only when it is missing.
//! 3. Inside that closure you may use `self.calls` and `self.func`: closures capture individual
//!    fields, so they do not conflict with the borrow of `self.cache`.

use std::collections::HashMap;

pub struct Memo<F: Fn(u64) -> u64> {
    func: F,
}

impl<F: Fn(u64) -> u64> Memo<F> {
    pub fn new(func: F) -> Memo<F> {
        todo!("Write your solution here")
    }

    pub fn get(&mut self, n: u64) -> u64 {
        todo!("Write your solution here")
    }

    pub fn calls(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn cached(&self) -> usize {
        todo!("Write your solution here")
    }
}

pub fn memoize<F: Fn(u64) -> u64>(func: F) -> impl FnMut(u64) -> u64 {
    move |_| todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn memo_computes_each_input_once() {
        let mut slow_square = Memo::new(|n| n * n);
        assert_eq!(slow_square.get(4), 16);
        assert_eq!(slow_square.get(4), 16);
        assert_eq!(slow_square.get(5), 25);
        assert_eq!(slow_square.calls(), 2);
        assert_eq!(slow_square.cached(), 2);
    }

    #[test]
    fn memo_starts_empty() {
        let memo = Memo::new(|n| n + 1);
        assert_eq!(memo.calls(), 0);
        assert_eq!(memo.cached(), 0);
    }

    #[test]
    fn memo_with_a_captured_value() {
        let base = 1_000;
        let mut shifted = Memo::new(move |n| n + base);
        let total: u64 = (0..100).map(|i| shifted.get(i % 10)).sum();
        assert_eq!(total, 10 * (0..10).map(|i| i + 1_000).sum::<u64>());
        assert_eq!(shifted.calls(), 10);
    }

    #[test]
    fn memoize_closure_runs_the_function_once_per_input() {
        // `Cell` lets an `Fn` closure count its own calls (interior mutability, see level 7).
        let runs = Cell::new(0);
        let mut digits = memoize(|n| {
            runs.set(runs.get() + 1);
            n.to_string().len() as u64
        });
        assert_eq!(digits(12_345), 5);
        assert_eq!(digits(12_345), 5);
        assert_eq!(digits(7), 1);
        assert_eq!(digits(7), 1);
        assert_eq!(runs.get(), 2);
    }

    #[test]
    fn two_memoized_closures_have_separate_caches() {
        let mut a = memoize(|n| n * 2);
        let mut b = memoize(|n| n * 3);
        assert_eq!(a(10), 20);
        assert_eq!(b(10), 30);
        assert_eq!(a(10), 20);
    }
}
