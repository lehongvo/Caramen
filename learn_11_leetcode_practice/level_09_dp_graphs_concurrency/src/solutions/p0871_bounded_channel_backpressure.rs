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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `fill_until_full` counts successful `try_send`s while `rx` is kept alive; the buffer
//! accepts exactly `capacity` values. `send_after_hang_up` drops the receiver and matches the
//! error. `bounded_pipeline` moves the `SyncSender` into a producer thread (its `send` blocks
//! whenever the consumer is `capacity` items behind) and collects `rx` on the calling thread; the
//! loop ends when the producer finishes and drops its sender.
//!
//! **Rust concepts:**
//! - `SyncSender<T>` is cloneable and `Send`, like `Sender<T>`, but its buffer is bounded.
//! - `TrySendError<T>` gives the unsent value back inside `Full(v)` / `Disconnected(v)`, so nothing
//!   is lost when a send fails.
//! - A rendezvous channel (`capacity = 0`) still works in the pipeline because the producer and
//!   the consumer run on different threads.
// @solution-end

use std::sync::mpsc::{self, TrySendError};
use std::thread;

pub fn fill_until_full(capacity: usize) -> usize {
    // @solution-start
    let (tx, _rx) = mpsc::sync_channel::<usize>(capacity);
    let mut sent = 0;
    loop {
        match tx.try_send(sent) {
            Ok(()) => sent += 1,
            Err(TrySendError::Full(_)) | Err(TrySendError::Disconnected(_)) => return sent,
        }
    }
    // @solution-end
}

pub fn send_after_hang_up() -> bool {
    // @solution-start
    let (tx, rx) = mpsc::sync_channel::<i32>(1);
    drop(rx);
    matches!(tx.try_send(1), Err(TrySendError::Disconnected(1)))
    // @solution-end
}

pub fn bounded_pipeline(items: Vec<i32>, capacity: usize) -> Vec<i32> {
    // @solution-start
    let (tx, rx) = mpsc::sync_channel::<i32>(capacity);
    let producer = thread::spawn(move || {
        for item in items {
            if tx.send(item).is_err() {
                break; // the consumer hung up
            }
        }
    });
    let out: Vec<i32> = rx.iter().map(|x| x * 2).collect();
    producer.join().expect("the producer cannot panic");
    out
    // @solution-end
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
