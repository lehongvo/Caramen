//! # Problem 0894: Counting Semaphore
//!
//! - **Difficulty:** Medium
//! - **Topics:** a semaphore from `Mutex<usize>` + `Condvar`, RAII permits released in `Drop`,
//!   a guard type that borrows its semaphore (`SemaphorePermit<'a>`)
//! - **Source:** Original · [`Condvar`](https://doc.rust-lang.org/std/sync/struct.Condvar.html) ·
//!   [The Rust Book 15.3](https://doc.rust-lang.org/book/ch15-03-drop.html)
//!
//! ## Problem
//!
//! A **counting semaphore** limits how many threads may be inside some section at once (for
//! example, at most 3 simultaneous downloads). It holds a number of *permits*: acquiring takes one
//! (waiting if none is left) and releasing gives it back.
//!
//! Implement it in RAII style, like `MutexGuard`: acquiring returns a `SemaphorePermit`, and
//! dropping the permit releases it automatically, even if the holder panics or returns early.
//!
//! ```text
//! pub struct Semaphore { /* your fields */ }
//! pub struct SemaphorePermit<'a> { semaphore: &'a Semaphore }
//! impl Semaphore {
//!     pub fn new(permits: usize) -> Self;
//!     pub fn acquire(&self) -> SemaphorePermit<'_>;              // waits for a free permit
//!     pub fn try_acquire(&self) -> Option<SemaphorePermit<'_>>;  // never waits
//!     pub fn available_permits(&self) -> usize;
//! }
//! impl Drop for SemaphorePermit<'_> { /* give the permit back and wake one waiter */ }
//! ```
//!
//! ## Examples
//!
//! ```text
//! let sem = Semaphore::new(2);
//! let a = sem.acquire();
//! let b = sem.try_acquire();     // Some(permit)
//! sem.try_acquire()              -> None (both permits taken)
//! drop(a);
//! sem.available_permits()        -> 1
//! ```
//!
//! ## Constraints
//!
//! - `0 <= permits <= 64`; up to 16 threads.
//!
//! ## Hints
//!
//! 1. Fields: `permits: Mutex<usize>` and `available: Condvar`.
//! 2. `acquire`: `wait_while(guard, |p| *p == 0)`, then decrement. `Drop`: lock, increment,
//!    `notify_one`.
//! 3. The permit stores `&'a Semaphore`, so the borrow checker guarantees a permit never outlives
//!    its semaphore.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the number of free permits lives in a mutex. `acquire` waits until it is positive and
//! decrements it; `try_acquire` decrements only if positive. Both wrap the result in a
//! `SemaphorePermit` whose `Drop` calls a private `release`, which increments the count and wakes
//! one waiting thread.
//!
//! **Rust concepts:**
//! - RAII: tying "release" to `Drop` makes it impossible to forget, the same way `MutexGuard`
//!   unlocks a mutex.
//! - A guard type with a lifetime parameter (`SemaphorePermit<'a>`) borrows the semaphore, so the
//!   compiler rejects code that would keep a permit after the semaphore is dropped.
//! - `std::sync` has no semaphore type; this is how one is usually built.
// @solution-end

use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

/// A counting semaphore with RAII permits.
pub struct Semaphore {
    // @solution-start remove
    permits: Mutex<usize>,
    available: Condvar,
    // @solution-end
}

/// One acquired permit; dropping it gives the permit back.
pub struct SemaphorePermit<'a> {
    semaphore: &'a Semaphore,
}

impl Semaphore {
    pub fn new(permits: usize) -> Self {
        // @solution-start
        Self {
            permits: Mutex::new(permits),
            available: Condvar::new(),
        }
        // @solution-end
    }

    // @solution-start remove
    fn lock(&self) -> MutexGuard<'_, usize> {
        self.permits.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn release(&self) {
        *self.lock() += 1;
        self.available.notify_one();
    }
    // @solution-end

    pub fn acquire(&self) -> SemaphorePermit<'_> {
        // @solution-start
        let guard = self.lock();
        let mut permits = self
            .available
            .wait_while(guard, |p| *p == 0)
            .unwrap_or_else(PoisonError::into_inner);
        *permits -= 1;
        SemaphorePermit { semaphore: self }
        // @solution-end
    }

    pub fn try_acquire(&self) -> Option<SemaphorePermit<'_>> {
        // @solution-start
        let mut permits = self.lock();
        if *permits == 0 {
            return None;
        }
        *permits -= 1;
        Some(SemaphorePermit { semaphore: self })
        // @solution-end
    }

    pub fn available_permits(&self) -> usize {
        // @solution-start
        *self.lock()
        // @solution-end
    }
}

impl Drop for SemaphorePermit<'_> {
    fn drop(&mut self) {
        // @solution-start
        self.semaphore.release();
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn example_two_permits() {
        let sem = Semaphore::new(2);
        let a = sem.acquire();
        let b = sem.try_acquire();
        assert!(b.is_some());
        assert!(sem.try_acquire().is_none());
        drop(a);
        assert_eq!(sem.available_permits(), 1);
        drop(b);
        assert_eq!(sem.available_permits(), 2);
    }

    #[test]
    fn zero_permits_until_one_is_returned() {
        let sem = Semaphore::new(0);
        assert!(sem.try_acquire().is_none());
        assert_eq!(sem.available_permits(), 0);
    }

    #[test]
    fn waiting_thread_gets_the_released_permit() {
        let sem = Semaphore::new(1);
        let held = sem.acquire();
        thread::scope(|s| {
            let waiter = s.spawn(|| {
                let _p = sem.acquire(); // blocks until `held` is dropped
                7
            });
            drop(held);
            assert_eq!(waiter.join().unwrap(), 7);
        });
        assert_eq!(sem.available_permits(), 1);
    }

    #[test]
    fn never_more_holders_than_permits() {
        let sem = Semaphore::new(3);
        let inside = AtomicUsize::new(0);
        let most = AtomicUsize::new(0);
        thread::scope(|s| {
            for _ in 0..16 {
                s.spawn(|| {
                    for _ in 0..200 {
                        let _permit = sem.acquire();
                        let now = inside.fetch_add(1, Ordering::SeqCst) + 1;
                        most.fetch_max(now, Ordering::SeqCst);
                        thread::yield_now();
                        inside.fetch_sub(1, Ordering::SeqCst);
                    }
                });
            }
        });
        let most = most.load(Ordering::SeqCst);
        assert!(
            (1..=3).contains(&most),
            "{most} threads were inside at once"
        );
        assert_eq!(sem.available_permits(), 3);
    }
}
