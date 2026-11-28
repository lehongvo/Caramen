// ==================================================================
// PRACTICE 0939: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0939_waker_and_block_on.md
// Run the tests:          cargo test practice::p0939
// ==================================================================

//! # Problem 0939: A `block_on` Executor
//!
//! - **Difficulty:** Hard
//! - **Topics:** `std::task::Wake`, `Waker::from(Arc<W>)`, `thread::park` / `Thread::unpark`, a minimal executor, futures woken from another thread
//! - **Source:** Original · [std: `task::Wake`](https://doc.rust-lang.org/std/task/trait.Wake.html) · [std: `thread::park`](https://doc.rust-lang.org/std/thread/fn.park.html)
//!
//! ## Problem
//!
//! Problem 0938 polled in a busy loop. A real executor **sleeps** until a future's waker is called.
//! `std::task::Wake` makes writing a waker easy: implement `wake(self: Arc<Self>)` on your type and
//! convert the `Arc` into a `Waker` with `Waker::from`.
//!
//! Implement:
//!
//! 1. `ThreadWaker`: holds the `std::thread::Thread` to wake and a wake counter (`AtomicUsize`);
//!    `wake` increments the counter and calls `unpark()` on the thread.
//! 2. `block_on<F: Future>(future: F) -> (F::Output, usize)`: runs a future to completion on the
//!    current thread. Pin it, build a `Waker` from an `Arc<ThreadWaker>` for the current thread, then
//!    loop: poll; if ready, return the output and the number of times the waker was woken; if
//!    pending, `thread::park()` until woken. (Parking may also return spuriously; just poll again.)
//! 3. `YieldNow` (from `yield_now()`): the first poll wakes the waker and returns `Pending`, the second
//!    returns `Ready(())`. It lets other work run, and it must not deadlock `block_on`.
//! 4. `Delay` (from `delay(value)`): on its first poll it spawns a thread that stores `value` in a
//!    shared slot and then calls `wake()` on a clone of the current waker; until the value is there,
//!    polls return `Pending` **without** waking. It completes with the value.
//!
//! ## Examples
//!
//! ```text
//! block_on(async { 6 * 7 })                     => (42, 0)
//! block_on(yield_now())                         => ((), 1)
//! block_on(async { yield_now().await; yield_now().await; 5 }) => (5, 2)
//! block_on(delay("from another thread")).0      => "from another thread"
//! ```
//!
//! ## Constraints
//!
//! - Tests must be deterministic: no sleeping, only real wake-ups.
//!
//! ## Hints
//!
//! 1. `thread::current()` returns a handle to the current thread; store it in the waker.
//! 2. If `unpark` runs **before** `park`, the next `park` returns immediately (a token is stored),
//!    so a wake-up can never be lost between `poll` and `park`.
//! 3. `Delay` can share an `Arc<Mutex<Option<T>>>` with the spawned thread.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

pub struct ThreadWaker {
    thread: Thread,
    wakes: AtomicUsize,
}

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        todo!("Write your solution here")
    }

    fn wake_by_ref(self: &Arc<Self>) {
        todo!("Write your solution here")
    }
}

pub fn block_on<F: Future>(future: F) -> (F::Output, usize) {
    todo!("Write your solution here")
}

pub struct YieldNow {
    yielded: bool,
}

pub fn yield_now() -> YieldNow {
    todo!("Write your solution here")
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        todo!("Write your solution here")
    }
}

pub struct Delay<T> {
    value: Option<T>,
    slot: Arc<Mutex<Option<T>>>,
}

pub fn delay<T: Send + 'static>(value: T) -> Delay<T> {
    todo!("Write your solution here")
}

impl<T> Unpin for Delay<T> {}

impl<T: Send + 'static> Future for Delay<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_futures_never_wake() {
        assert_eq!(block_on(async { 6 * 7 }), (42, 0));
        assert_eq!(block_on(std::future::ready("x")), ("x", 0));
    }

    #[test]
    fn yielding_wakes_once_per_yield() {
        assert_eq!(block_on(yield_now()), ((), 1));
        let (value, wakes) = block_on(async {
            yield_now().await;
            yield_now().await;
            5
        });
        assert_eq!((value, wakes), (5, 2));
        let (_, many) = block_on(async {
            for _ in 0..1_000 {
                yield_now().await;
            }
        });
        assert_eq!(many, 1_000);
    }

    #[test]
    fn woken_from_another_thread() {
        let (value, wakes) = block_on(delay("from another thread"));
        assert_eq!(value, "from another thread");
        assert_eq!(wakes, 1);
    }

    #[test]
    fn several_delays_in_sequence() {
        let (sum, wakes) = block_on(async {
            let mut sum = 0;
            for i in 1..=10 {
                sum += delay(i).await;
            }
            sum
        });
        assert_eq!((sum, wakes), (55, 10));
    }

    #[test]
    fn the_waker_unparks_the_right_thread() {
        let state = Arc::new(ThreadWaker {
            thread: thread::current(),
            wakes: AtomicUsize::new(0),
        });
        let waker = Waker::from(Arc::clone(&state));
        waker.wake_by_ref();
        // A second `Waker` for the same state, consumed by `wake`.
        Waker::from(Arc::clone(&state)).wake();
        assert_eq!(state.wakes.load(Ordering::SeqCst), 2);
        // The stored unpark token makes this park return immediately.
        thread::park();
        assert_eq!(block_on(yield_now()).1, 1);
    }
}
