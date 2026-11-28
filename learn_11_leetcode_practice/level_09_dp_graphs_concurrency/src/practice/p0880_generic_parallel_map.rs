// ==================================================================
// PRACTICE 0880: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0880_generic_parallel_map.md
// Run the tests:          cargo test practice::p0880
// ==================================================================

//! # Problem 0880: Generic Parallel Map
//!
//! - **Difficulty:** Medium
//! - **Topics:** generic functions over threads, choosing the right bounds (`T: Sync`, `U: Send`,
//!   `F: Fn(&T) -> U + Sync`), keeping the output order
//! - **Source:** Original · [The Rust Book 16.4](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
//!
//! ## Problem
//!
//! Implement
//!
//! ```text
//! pub fn parallel_map<T, U, F>(items: &[T], workers: usize, f: F) -> Vec<U>
//! where T: Sync, U: Send, F: Fn(&T) -> U + Sync
//! ```
//!
//! It returns `items.iter().map(&f).collect()`, but computed by up to `workers` scoped threads,
//! each handling one contiguous chunk of `ceil(len / workers)` items. The output must be in the
//! same order as the input, and `f` must be called exactly once per item.
//!
//! ## Examples
//!
//! ```text
//! parallel_map(&[1, 2, 3, 4], 2, |x| x * x)              -> [1, 4, 9, 16]
//! parallel_map(&["a", "bcd"], 8, |s| s.len())            -> [1, 3]
//! parallel_map(&[] as &[i32], 3, |x| *x)                 -> []
//! ```
//!
//! ## Constraints
//!
//! - `1 <= workers <= 64`; `0 <= items.len() <= 10^6`
//!
//! ## Hints
//!
//! 1. Why these bounds? Threads read the items through shared references (`&T` crosses threads, so
//!    `T: Sync`); the results are **moved** back to the caller (`U: Send`); every thread calls the
//!    same `f` through `&F` (`F: Sync`).
//! 2. Let each scoped thread return its own `Vec<U>`; join the handles in chunk order and
//!    concatenate.

use std::thread;

pub fn parallel_map<T, U, F>(items: &[T], workers: usize, f: F) -> Vec<U>
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> U + Sync,
{
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn example_squares() {
        assert_eq!(parallel_map(&[1, 2, 3, 4], 2, |x| x * x), vec![1, 4, 9, 16]);
    }

    #[test]
    fn example_string_lengths() {
        assert_eq!(parallel_map(&["a", "bcd"], 8, |s| s.len()), vec![1, 3]);
    }

    #[test]
    fn example_empty() {
        let empty: &[i32] = &[];
        assert!(parallel_map(empty, 3, |x| *x).is_empty());
    }

    #[test]
    fn non_copy_results() {
        let words = vec!["x".to_string(), "yy".to_string()];
        let got = parallel_map(&words, 2, |w| format!("{w}!"));
        assert_eq!(got, vec!["x!".to_string(), "yy!".to_string()]);
    }

    #[test]
    fn function_called_once_per_item() {
        let calls = AtomicUsize::new(0);
        let items: Vec<u64> = (0..100_000).collect();
        let got = parallel_map(&items, 7, |&x| {
            calls.fetch_add(1, Ordering::Relaxed);
            x * 3
        });
        assert_eq!(calls.load(Ordering::Relaxed), 100_000);
        assert!(got.iter().enumerate().all(|(i, &y)| y == 3 * i as u64));
    }
}
