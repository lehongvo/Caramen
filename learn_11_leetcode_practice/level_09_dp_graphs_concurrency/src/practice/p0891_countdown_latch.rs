// ==================================================================
// PRACTICE 0891: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0891_countdown_latch.md
// Run the tests:          cargo test practice::p0891
// ==================================================================

//! # Problem 0891: Countdown Latch
//!
//! - **Difficulty:** Medium
//! - **Topics:** `std::sync::Condvar` paired with a `Mutex`, `Condvar::wait_while`, `notify_all`,
//!   blocking until a condition holds
//! - **Source:** Original · [`Condvar`](https://doc.rust-lang.org/std/sync/struct.Condvar.html)
//!
//! ## Problem
//!
//! A **condition variable** lets a thread sleep until another thread signals that something it
//! waits for may have changed. It is always used with a `Mutex` that protects the condition.
//!
//! Implement `CountDownLatch`, a one-shot gate that opens after a fixed number of events:
//!
//! - `new(count: usize) -> CountDownLatch`;
//! - `count_down(&self)`: decrease the count by one (it never goes below `0`); when it reaches `0`,
//!   wake **every** waiting thread;
//! - `count(&self) -> usize`: the current count;
//! - `wait(&self)`: block until the count is `0` (returns immediately if it already is).
//!
//! A typical use: start `n` workers, let each one publish its result and call `count_down`, and
//! let the coordinator `wait` before reading the results, without joining the threads.
//!
//! ## Examples
//!
//! ```text
//! let latch = CountDownLatch::new(2);
//! latch.count()      -> 2
//! latch.count_down(); latch.count() -> 1
//! latch.count_down(); latch.count() -> 0
//! latch.count_down(); latch.count() -> 0   (never negative)
//! latch.wait()       returns at once
//! ```
//!
//! ## Constraints
//!
//! - `0 <= count <= 10^4`; up to 32 threads.
//!
//! ## Hints
//!
//! 1. Fields: `count: Mutex<usize>` and `zero: Condvar`.
//! 2. `self.zero.wait_while(guard, |c| *c > 0)` releases the lock while sleeping, re-locks when
//!    woken, and loops until the closure returns `false`. It also handles *spurious* wake-ups,
//!    which `Condvar::wait` alone does not.
//! 3. Several threads may be waiting, so use `notify_all`, not `notify_one`.

use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

/// A gate that opens once `count_down` has been called `count` times.
pub struct CountDownLatch {}

impl CountDownLatch {
    pub fn new(count: usize) -> Self {
        todo!("Write your solution here")
    }

    pub fn count_down(&self) {
        todo!("Write your solution here")
    }

    pub fn count(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn wait(&self) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn example_counting_down() {
        let latch = CountDownLatch::new(2);
        assert_eq!(latch.count(), 2);
        latch.count_down();
        assert_eq!(latch.count(), 1);
        latch.count_down();
        assert_eq!(latch.count(), 0);
        latch.count_down();
        assert_eq!(latch.count(), 0);
        latch.wait();
    }

    #[test]
    fn zero_latch_is_open() {
        CountDownLatch::new(0).wait();
    }

    #[test]
    fn coordinator_waits_for_workers_without_joining() {
        let latch = Arc::new(CountDownLatch::new(16));
        let results = Arc::new(Mutex::new(Vec::new()));
        for w in 0..16 {
            let (latch, results) = (Arc::clone(&latch), Arc::clone(&results));
            thread::spawn(move || {
                results.lock().unwrap().push(w * w);
                latch.count_down();
            });
        }
        latch.wait();
        let mut got = results.lock().unwrap().clone();
        got.sort_unstable();
        assert_eq!(got, (0..16).map(|w| w * w).collect::<Vec<_>>());
    }

    #[test]
    fn every_waiter_is_released() {
        let latch = CountDownLatch::new(3);
        thread::scope(|s| {
            let waiters: Vec<_> = (0..8)
                .map(|_| {
                    s.spawn(|| {
                        latch.wait();
                        latch.count()
                    })
                })
                .collect();
            for _ in 0..3 {
                latch.count_down();
            }
            for w in waiters {
                assert_eq!(w.join().unwrap(), 0);
            }
        });
    }
}
