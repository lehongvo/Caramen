// ==================================================================
// PRACTICE 0442: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0442_running_average_closure.md
// Run the tests:          cargo test practice::p0442
// ==================================================================

//! # Problem 0442: A Running Average Closure
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning `impl FnMut`, state owned by a closure (`move`), `HashSet` inside a closure, passing an `FnMut` to `map` and `filter`
//! - **Source:** Original · [The Rust Book 13.1](https://doc.rust-lang.org/book/ch13-01-closures.html) · [std `FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html)
//!
//! ## Problem
//!
//! A closure can carry private, mutable state between calls, like a tiny object with one method.
//! The factory creates the state, moves it into the closure, and returns the closure as
//! `impl FnMut(...)`.
//!
//! Implement these factories (every call to a factory creates fresh, independent state):
//!
//! - `make_running_average() -> impl FnMut(f64) -> f64`: each call records a sample and returns the
//!   average of all samples so far.
//! - `make_accumulator(start: i64) -> impl FnMut(i64) -> i64`: each call adds its argument to the
//!   total (which begins at `start`) and returns the new total.
//! - `make_max_tracker() -> impl FnMut(i32) -> i32`: each call returns the largest value seen so far
//!   (including the current one).
//! - `make_first_seen() -> impl FnMut(&str) -> bool`: returns `true` the first time it sees a word
//!   and `false` for repeats.
//!
//! Then use them:
//!
//! - `smooth(values: &[f64]) -> Vec<f64>`: the running averages of `values`.
//! - `unique_words(text: &str) -> Vec<&str>`: the words of `text` (split on whitespace) without
//!   repeats, in order of first appearance.
//!
//! ## Examples
//!
//! ```text
//! let mut avg = make_running_average();   avg(2.0) -> 2.0,  avg(4.0) -> 3.0,  avg(9.0) -> 5.0
//! let mut acc = make_accumulator(10);     acc(5) -> 15,  acc(-20) -> -5
//! let mut best = make_max_tracker();      best(3) -> 3,  best(1) -> 3,  best(8) -> 8
//! smooth(&[1.0, 3.0, 5.0])                -> [1.0, 2.0, 3.0]
//! unique_words("a b a c b")               -> ["a", "b", "c"]
//! ```
//!
//! ## Constraints
//!
//! - At most `10^5` calls per closure.
//!
//! ## Hints
//!
//! 1. `let mut sum = 0.0; let mut count = 0; move |x| { ... }` gives the closure its own copies.
//! 2. `HashSet::insert` returns `true` when the value was not present yet.
//! 3. An `FnMut` closure can be handed to iterator adapters directly:
//!    `values.iter().copied().map(make_running_average())`.

use std::collections::HashSet;

pub fn make_running_average() -> impl FnMut(f64) -> f64 {
    |_| todo!()
}

pub fn make_accumulator(start: i64) -> impl FnMut(i64) -> i64 {
    move |_| todo!()
}

pub fn make_max_tracker() -> impl FnMut(i32) -> i32 {
    |_| todo!()
}

pub fn make_first_seen() -> impl FnMut(&str) -> bool {
    |_| todo!()
}

pub fn smooth(values: &[f64]) -> Vec<f64> {
    todo!("Write your solution here")
}

pub fn unique_words(text: &str) -> Vec<&str> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_average() {
        let mut avg = make_running_average();
        assert_eq!(avg(2.0), 2.0);
        assert_eq!(avg(4.0), 3.0);
        assert_eq!(avg(9.0), 5.0);
        let mut other = make_running_average();
        assert_eq!(other(-1.0), -1.0);
        assert_eq!(avg(5.0), 5.0);
    }

    #[test]
    fn accumulator() {
        let mut acc = make_accumulator(10);
        assert_eq!(acc(5), 15);
        assert_eq!(acc(-20), -5);
        let mut big = make_accumulator(0);
        let mut last = 0;
        for _ in 0..100_000 {
            last = big(1_000_000);
        }
        assert_eq!(last, 100_000_000_000);
    }

    #[test]
    fn max_tracker() {
        let mut best = make_max_tracker();
        assert_eq!(best(3), 3);
        assert_eq!(best(1), 3);
        assert_eq!(best(8), 8);
        let mut negative = make_max_tracker();
        assert_eq!(negative(-5), -5);
        assert_eq!(negative(-9), -5);
    }

    #[test]
    fn first_seen() {
        let mut seen = make_first_seen();
        assert!(seen("a"));
        assert!(seen("b"));
        assert!(!seen("a"));
        assert!(seen("A"));
    }

    #[test]
    fn smooth_and_unique_words() {
        assert_eq!(smooth(&[1.0, 3.0, 5.0]), vec![1.0, 2.0, 3.0]);
        assert_eq!(smooth(&[]), Vec::<f64>::new());
        assert_eq!(unique_words("a b a c b"), vec!["a", "b", "c"]);
        assert_eq!(
            unique_words("  the cat and the hat  "),
            vec!["the", "cat", "and", "hat"]
        );
        assert_eq!(unique_words(""), Vec::<&str>::new());
    }
}
