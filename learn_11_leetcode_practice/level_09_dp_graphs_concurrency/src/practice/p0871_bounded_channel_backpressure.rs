// ==================================================================
// PRACTICE 0871: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0871_bounded_channel_backpressure.md
// Run the tests:          cargo test practice::p0871
// ==================================================================

//! # Problem 0871: Bounded Channel Backpressure
//!
//! - **Difficulty:** Medium
//! - **Topics:** `mpsc::sync_channel` with a fixed capacity, blocking `send` versus `try_send`,
//!   `TrySendError::{Full, Disconnected}`
//! - **Source:** Original · [`mpsc::sync_channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html)
//!
//! ## Problem
//!
//! `mpsc::channel()` has an unbounded buffer: a fast producer can fill memory if the consumer is
//! slow. `mpsc::sync_channel(capacity)` buffers at most `capacity` messages; when the buffer is
//! full, `send` **blocks** until the consumer takes a message (this is called *backpressure*), and
//! `try_send` returns an error instead of blocking. With `capacity = 0` the channel is a
//! *rendezvous* channel: every `send` waits until a receiver takes the value directly.
//!
//! Implement three functions:
//!
//! 1. `fill_until_full(capacity: usize) -> usize`: create a `sync_channel(capacity)` and, **from
//!    the calling thread only**, call `try_send(0)`, `try_send(1)`, ... until it fails with
//!    `TrySendError::Full`. Return how many sends succeeded.
//! 2. `send_after_hang_up() -> bool`: create a `sync_channel(1)`, drop the receiver, then return
//!    `true` if `try_send` reports `TrySendError::Disconnected` (and `false` otherwise).
//! 3. `bounded_pipeline(items: Vec<i32>, capacity: usize) -> Vec<i32>`: spawn a producer thread
//!    that sends every item through a `sync_channel(capacity)` with the **blocking** `send`. The
//!    calling thread receives the items and returns them doubled, in the original order. It must
//!    work for every capacity, including `0`.
//!
//! ## Examples
//!
//! ```text
//! fill_until_full(3)               -> 3
//! fill_until_full(0)               -> 0      (nobody is waiting to receive)
//! send_after_hang_up()             -> true
//! bounded_pipeline(vec![1, 2, 3], 1) -> [2, 4, 6]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= capacity <= 1000`; `0 <= items.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `let (tx, rx) = mpsc::sync_channel::<i32>(capacity);` returns a `SyncSender` and a
//!    `Receiver`.
//! 2. In `fill_until_full` nobody receives, so a blocking `send` on a full channel would wait
//!    forever. Only `try_send` is safe there; keep `rx` alive until the end so the channel is not
//!    disconnected.
//! 3. `match tx.try_send(v) { Ok(()) => .., Err(TrySendError::Full(_)) => .., Err(TrySendError::Disconnected(_)) => .. }`

use std::sync::mpsc::{self, TrySendError};
use std::thread;

pub fn fill_until_full(capacity: usize) -> usize {
    todo!("Write your solution here")
}

pub fn send_after_hang_up() -> bool {
    todo!("Write your solution here")
}

pub fn bounded_pipeline(items: Vec<i32>, capacity: usize) -> Vec<i32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_fill_three() {
        assert_eq!(fill_until_full(3), 3);
    }

    #[test]
    fn example_rendezvous_has_no_buffer() {
        assert_eq!(fill_until_full(0), 0);
    }

    #[test]
    fn example_hang_up() {
        assert!(send_after_hang_up());
    }

    #[test]
    fn example_pipeline() {
        assert_eq!(bounded_pipeline(vec![1, 2, 3], 1), vec![2, 4, 6]);
    }

    #[test]
    fn larger_buffer() {
        assert_eq!(fill_until_full(1000), 1000);
    }

    #[test]
    fn pipeline_with_every_capacity() {
        let items: Vec<i32> = (0..100_000).collect();
        let want: Vec<i32> = items.iter().map(|x| x * 2).collect();
        for capacity in [0, 1, 16, 1000] {
            assert_eq!(bounded_pipeline(items.clone(), capacity), want);
        }
        assert!(bounded_pipeline(vec![], 0).is_empty());
    }
}
