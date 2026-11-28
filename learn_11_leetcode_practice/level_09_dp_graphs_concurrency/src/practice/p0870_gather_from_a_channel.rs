// ==================================================================
// PRACTICE 0870: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0870_gather_from_a_channel.md
// Run the tests:          cargo test practice::p0870
// ==================================================================

//! # Problem 0870: Gather from a Channel
//!
//! - **Difficulty:** Easy
//! - **Topics:** `std::sync::mpsc::channel`, cloning a `Sender` for several producers, ending the
//!   receive loop by dropping every `Sender`
//! - **Source:** Original · [The Rust Book 16.2](https://doc.rust-lang.org/book/ch16-02-message-passing.html) ·
//!   [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html)
//!
//! ## Problem
//!
//! Implement `gather(workers: usize, per_worker: usize) -> Vec<(usize, usize)>`.
//!
//! Create one channel and spawn `workers` producer threads. Producer `w` sends the messages
//! `(w, 0), (w, 1), ..., (w, per_worker - 1)` in that order. The calling thread receives **every**
//! message and returns them in the order they were received. The function must return once all
//! producers are done; it must not hang waiting for more messages.
//!
//! Messages of different producers may interleave in any way, but a channel is first-in first-out:
//! the messages of one producer arrive in the order that producer sent them.
//!
//! ## Examples
//!
//! ```text
//! gather(1, 3) -> [(0, 0), (0, 1), (0, 2)]
//! gather(2, 2) -> for example [(0, 0), (1, 0), (0, 1), (1, 1)]
//!                 (any interleaving that keeps each worker's own order)
//! gather(0, 5) -> []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= workers <= 32`, `0 <= per_worker <= 10^4`
//!
//! ## Hints
//!
//! 1. `let (tx, rx) = mpsc::channel();` then give each producer its own `tx.clone()` (moved into
//!    its thread).
//! 2. `for msg in rx` (or `rx.iter()`) keeps receiving until **all** senders are dropped.
//! 3. The original `tx` is a sender too: drop it in the receiving thread, or the loop never ends.

use std::sync::mpsc;
use std::thread;

pub fn gather(workers: usize, per_worker: usize) -> Vec<(usize, usize)> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that `got` has every expected message once and keeps each worker's order.
    fn assert_valid(got: &[(usize, usize)], workers: usize, per_worker: usize) {
        assert_eq!(got.len(), workers * per_worker);
        let mut next = vec![0; workers];
        for &(w, i) in got {
            assert_eq!(i, next[w], "worker {w} messages arrived out of order");
            next[w] += 1;
        }
        assert!(next.iter().all(|&n| n == per_worker));
    }

    #[test]
    fn example_single_worker() {
        assert_eq!(gather(1, 3), vec![(0, 0), (0, 1), (0, 2)]);
    }

    #[test]
    fn example_two_workers() {
        assert_valid(&gather(2, 2), 2, 2);
    }

    #[test]
    fn example_no_workers() {
        assert!(gather(0, 5).is_empty());
    }

    #[test]
    fn workers_with_nothing_to_send() {
        assert!(gather(4, 0).is_empty());
    }

    #[test]
    fn many_workers_many_messages() {
        assert_valid(&gather(32, 10_000), 32, 10_000);
    }
}
