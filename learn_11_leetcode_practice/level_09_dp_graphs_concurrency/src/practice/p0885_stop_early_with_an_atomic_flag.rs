// ==================================================================
// PRACTICE 0885: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0885_stop_early_with_an_atomic_flag.md
// Run the tests:          cargo test practice::p0885
// ==================================================================

//! # Problem 0885: Stop Early with an Atomic Flag
//!
//! - **Difficulty:** Medium
//! - **Topics:** cooperative cancellation with `AtomicBool`, `AtomicUsize::fetch_min` to keep the
//!   best answer, generic predicates shared by scoped threads
//! - **Source:** Original · [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/index.html)
//!
//! ## Problem
//!
//! Threads cannot be killed from the outside in Rust; to stop early they must **cooperate** by
//! checking a shared flag. Implement two parallel searches over a slice, each using up to
//! `workers` scoped threads on contiguous chunks of `ceil(len / workers)` items:
//!
//! 1. `parallel_any(items, workers, pred) -> bool`: `true` if some item satisfies `pred`. As soon
//!    as one worker finds a match it raises an `AtomicBool`, and every worker checks the flag
//!    before testing its next item and stops if it is set.
//! 2. `parallel_position(items, workers, pred) -> Option<usize>`: the **smallest** index whose item
//!    satisfies `pred`. Keep the best index found so far in an `AtomicUsize` (start with
//!    `usize::MAX`) updated with `fetch_min`. A worker stops at its first match, and also as soon as
//!    the index it would test next is not smaller than the best index found so far.
//!
//! Signatures (`T: Sync`, `F: Fn(&T) -> bool + Sync`):
//!
//! ```text
//! pub fn parallel_any<T, F>(items: &[T], workers: usize, pred: F) -> bool
//! pub fn parallel_position<T, F>(items: &[T], workers: usize, pred: F) -> Option<usize>
//! ```
//!
//! ## Examples
//!
//! ```text
//! parallel_any(&[1, 3, 5, 6, 7], 2, |x| x % 2 == 0)      -> true
//! parallel_any(&[1, 3, 5], 3, |x| x % 2 == 0)            -> false
//! parallel_position(&[4, 1, 8, 1, 9], 3, |&x| x == 1)    -> Some(1)
//! parallel_position(&[] as &[i32], 3, |_| true)          -> None
//! ```
//!
//! ## Constraints
//!
//! - `1 <= workers <= 32`; `0 <= items.len() <= 10^6`
//!
//! ## Hints
//!
//! 1. The flag only says "stop now", it does not protect other data, so `Ordering::Relaxed` loads
//!    and stores are enough; joining the scope makes the final value visible.
//! 2. `fetch_min(i)` stores `min(old, i)` atomically, so two workers racing to report a match
//!    cannot overwrite a smaller index with a larger one.
//! 3. With a single worker, the search is sequential: it tests items in order and stops at the
//!    first match.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

pub fn parallel_any<T, F>(items: &[T], workers: usize, pred: F) -> bool
where
    T: Sync,
    F: Fn(&T) -> bool + Sync,
{
    todo!("Write your solution here")
}

pub fn parallel_position<T, F>(items: &[T], workers: usize, pred: F) -> Option<usize>
where
    T: Sync,
    F: Fn(&T) -> bool + Sync,
{
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_any_true() {
        assert!(parallel_any(&[1, 3, 5, 6, 7], 2, |x| x % 2 == 0));
    }

    #[test]
    fn example_any_false() {
        assert!(!parallel_any(&[1, 3, 5], 3, |x| x % 2 == 0));
    }

    #[test]
    fn example_position() {
        assert_eq!(parallel_position(&[4, 1, 8, 1, 9], 3, |&x| x == 1), Some(1));
    }

    #[test]
    fn example_empty() {
        let empty: &[i32] = &[];
        assert_eq!(parallel_position(empty, 3, |_| true), None);
        assert!(!parallel_any(empty, 3, |_| true));
    }

    #[test]
    fn smallest_index_wins_across_chunks() {
        // Matches at 999_999 (last chunk) and 250_001 (second chunk); the smaller one is returned.
        let mut items = vec![0_u8; 1_000_000];
        items[999_999] = 1;
        items[250_001] = 1;
        for workers in [1, 2, 4, 32] {
            assert_eq!(
                parallel_position(&items, workers, |&x| x == 1),
                Some(250_001)
            );
            assert!(parallel_any(&items, workers, |&x| x == 1));
        }
        assert_eq!(parallel_position(&items, 4, |&x| x == 2), None);
    }

    #[test]
    fn single_worker_stops_at_the_first_match() {
        let calls = AtomicUsize::new(0);
        let items: Vec<u32> = (0..10_000).collect();
        let pos = parallel_position(&items, 1, |&x| {
            calls.fetch_add(1, Ordering::Relaxed);
            x == 41
        });
        assert_eq!(pos, Some(41));
        assert_eq!(calls.load(Ordering::Relaxed), 42);
        calls.store(0, Ordering::Relaxed);
        assert!(parallel_any(&items, 1, |&x| {
            calls.fetch_add(1, Ordering::Relaxed);
            x == 9
        }));
        assert_eq!(calls.load(Ordering::Relaxed), 10);
    }
}
