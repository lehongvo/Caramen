// ==================================================================
// PRACTICE 0893: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0893_bounded_blocking_queue.md
// Run the tests:          cargo test practice::p0893
// ==================================================================

//! # Problem 0893: Bounded Blocking Queue
//!
//! - **Difficulty:** Medium
//! - **Topics:** a generic producer-consumer queue from `Mutex<VecDeque<T>>` and **two**
//!   `Condvar`s (`not_empty`, `not_full`), blocking `push`/`pop`, a non-blocking `try_pop`
//! - **Source:** Original · [`Condvar`](https://doc.rust-lang.org/std/sync/struct.Condvar.html)
//!
//! ## Problem
//!
//! Build the data structure behind `sync_channel` yourself: a FIFO queue with a fixed capacity that
//! many threads can use through `&self`.
//!
//! ```text
//! pub struct BoundedQueue<T> { /* your fields */ }
//! impl<T> BoundedQueue<T> {
//!     pub fn new(capacity: usize) -> Self;      // capacity >= 1
//!     pub fn push(&self, item: T);              // blocks while the queue is full
//!     pub fn pop(&self) -> T;                   // blocks while the queue is empty
//!     pub fn try_pop(&self) -> Option<T>;       // never blocks
//!     pub fn len(&self) -> usize;
//!     pub fn is_empty(&self) -> bool;
//!     pub fn peak(&self) -> usize;              // the largest len() ever reached
//! }
//! ```
//!
//! Items come out in the order they went in. `peak` lets the tests verify that the queue never held
//! more than `capacity` items, even with many producers and consumers.
//!
//! ## Examples
//!
//! ```text
//! let q = BoundedQueue::new(2);
//! q.push('a'); q.push('b');        // a third push would block until someone pops
//! q.len()     -> 2
//! q.pop()     -> 'a'
//! q.try_pop() -> Some('b')
//! q.try_pop() -> None
//! q.peak()    -> 2
//! ```
//!
//! ## Constraints
//!
//! - `1 <= capacity <= 1000`; up to 8 producers and 8 consumers.
//!
//! ## Hints
//!
//! 1. One mutex protects the deque (and the peak). `push` waits on `not_full` while
//!    `len == capacity`; `pop` waits on `not_empty` while the deque is empty.
//! 2. After pushing, `not_empty.notify_one()`; after popping, `not_full.notify_one()`. Each event
//!    frees exactly one slot or one item, so waking one waiter is enough.
//! 3. Always wait with a predicate (`wait_while`): another thread may have taken the item first.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

/// A fixed-capacity FIFO queue whose `push` and `pop` block when needed.
pub struct BoundedQueue<T> {
    _todo: std::marker::PhantomData<T>,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&self, item: T) {
        todo!("Write your solution here")
    }

    pub fn pop(&self) -> T {
        todo!("Write your solution here")
    }

    pub fn try_pop(&self) -> Option<T> {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn peak(&self) -> usize {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn example_fifo() {
        let q = BoundedQueue::new(2);
        q.push('a');
        q.push('b');
        assert_eq!(q.len(), 2);
        assert_eq!(q.pop(), 'a');
        assert_eq!(q.try_pop(), Some('b'));
        assert_eq!(q.try_pop(), None);
        assert!(q.is_empty());
        assert_eq!(q.peak(), 2);
    }

    #[test]
    fn push_blocks_until_there_is_room() {
        let q = BoundedQueue::new(1);
        q.push(1);
        thread::scope(|s| {
            s.spawn(|| q.push(2)); // blocks until the main thread pops 1
            assert_eq!(q.pop(), 1);
            assert_eq!(q.pop(), 2); // blocks until the push above has happened
        });
        assert_eq!(q.peak(), 1);
    }

    #[test]
    fn pop_blocks_until_an_item_arrives() {
        let q = BoundedQueue::new(3);
        thread::scope(|s| {
            let consumer = s.spawn(|| q.pop() + q.pop());
            q.push(20);
            q.push(22);
            assert_eq!(consumer.join().unwrap(), 42);
        });
    }

    #[test]
    fn many_producers_and_consumers() {
        let q = BoundedQueue::new(4);
        let mut received: Vec<u32> = thread::scope(|s| {
            for p in 0..8 {
                let q = &q;
                s.spawn(move || {
                    for i in 0..1000 {
                        q.push(p * 1000 + i);
                    }
                });
            }
            let consumers: Vec<_> = (0..8)
                .map(|_| s.spawn(|| (0..1000).map(|_| q.pop()).collect::<Vec<u32>>()))
                .collect();
            consumers
                .into_iter()
                .flat_map(|c| c.join().unwrap())
                .collect()
        });
        received.sort_unstable();
        assert_eq!(received, (0..8000).collect::<Vec<u32>>());
        assert!(q.peak() <= 4 && q.peak() >= 1);
        assert!(q.is_empty());
    }
}
