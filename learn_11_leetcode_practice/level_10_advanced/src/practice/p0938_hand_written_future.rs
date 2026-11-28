// ==================================================================
// PRACTICE 0938: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0938_hand_written_future.md
// Run the tests:          cargo test practice::p0938
// ==================================================================

//! # Problem 0938: Implement `Future` by Hand
//!
//! - **Difficulty:** Hard
//! - **Topics:** the `Future` trait, `Poll`, `Pin<&mut Self>`, `Context` and `Waker`, `Unpin`, `std::pin::pin!`, `Waker::noop`
//! - **Source:** Original · [std: `Future`](https://doc.rust-lang.org/std/future/trait.Future.html) · [The Rust Book 17.5: Traits for Async](https://doc.rust-lang.org/book/ch17-05-traits-for-async.html)
//!
//! ## Problem
//!
//! An `async` block is compiled into a state machine implementing `Future`. Write a few futures by
//! hand to see what that means:
//!
//! ```text
//! pub trait Future { type Output; fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>; }
//! ```
//!
//! 1. `Countdown` (created by `countdown(n)`): each poll increments a poll counter; while the
//!    remaining count is positive it decrements it, **wakes its own waker** (so an executor knows
//!    to poll again) and returns `Poll::Pending`; once it reaches zero it returns
//!    `Poll::Ready(total number of polls)`. So `countdown(3)` completes on its 4th poll with `4`.
//! 2. `Ready<T>` (created by `ready(value)`): completes immediately with `value`. Polling it again
//!    after completion panics with `"Ready polled after completion"`. It must be `Unpin` for **every**
//!    `T` (write `impl<T> Unpin for Ready<T> {}`).
//! 3. `Map<F, G>` (created by `map(future, f)`), for `F: Future + Unpin` and `G: FnOnce(F::Output) -> U`:
//!    polls the inner future and applies `f` to its output. Polling after completion panics with
//!    `"Map polled after completion"`. `Map` is `Unpin` whenever `F` is (the closure is never
//!    pinned): `impl<F: Unpin, G> Unpin for Map<F, G> {}` is given.
//! 4. `poll_until_ready<F: Future>(future: F) -> (F::Output, usize)`: pins the future on the stack
//!    with `std::pin::pin!` and polls it in a loop with a context built from `Waker::noop()`, until it
//!    is ready. Returns the output and the number of polls.
//!
//! ## Examples
//!
//! ```text
//! poll_until_ready(countdown(3))                    => (4, 4)
//! poll_until_ready(ready("hi"))                     => ("hi", 1)
//! poll_until_ready(map(countdown(1), |n| n * 100))  => (200, 2)
//! poll_until_ready(async { 1 + 2 })                 => (3, 1)
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. For an `Unpin` type, `self.get_mut()` (or `&mut *self`) turns `Pin<&mut Self>` into `&mut Self`.
//! 2. `Countdown` has only integer fields, so it is automatically `Unpin`.
//! 3. In `Map`, keep the function in an `Option<G>` and `take()` it when the inner future is ready;
//!    poll the inner future with `Pin::new(&mut this.future).poll(cx)`.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct Countdown {
    remaining: u32,
    polls: usize,
}

pub fn countdown(n: u32) -> Countdown {
    todo!("Write your solution here")
}

impl Future for Countdown {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<usize> {
        todo!("Write your solution here")
    }
}

pub struct Ready<T> {
    value: Option<T>,
}

impl<T> Unpin for Ready<T> {}

pub fn ready<T>(value: T) -> Ready<T> {
    todo!("Write your solution here")
}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        todo!("Write your solution here")
    }
}

pub struct Map<F, G> {
    future: F,
    f: Option<G>,
}

// `Map` never pins its closure, so it only needs the inner future to be `Unpin`.
impl<F: Unpin, G> Unpin for Map<F, G> {}

pub fn map<F, G, U>(future: F, f: G) -> Map<F, G>
where
    F: Future + Unpin,
    G: FnOnce(F::Output) -> U,
{
    todo!("Write your solution here")
}

impl<F, G, U> Future for Map<F, G>
where
    F: Future + Unpin,
    G: FnOnce(F::Output) -> U,
{
    type Output = U;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<U> {
        todo!("Write your solution here")
    }
}

pub fn poll_until_ready<F: Future>(future: F) -> (F::Output, usize) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::Wake;

    struct CountWakes(AtomicUsize);

    impl Wake for CountWakes {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn countdown_polls() {
        assert_eq!(poll_until_ready(countdown(3)), (4, 4));
        assert_eq!(poll_until_ready(countdown(0)), (1, 1));
        assert_eq!(poll_until_ready(countdown(1_000)), (1_001, 1_001));
    }

    #[test]
    fn countdown_wakes_itself_before_pending() {
        let counter = Arc::new(CountWakes(AtomicUsize::new(0)));
        let waker = Waker::from(Arc::clone(&counter));
        let mut cx = Context::from_waker(&waker);
        let mut future = countdown(2);
        assert_eq!(Pin::new(&mut future).poll(&mut cx), Poll::Pending);
        assert_eq!(Pin::new(&mut future).poll(&mut cx), Poll::Pending);
        assert_eq!(Pin::new(&mut future).poll(&mut cx), Poll::Ready(3));
        assert_eq!(counter.0.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn ready_and_map() {
        assert_eq!(poll_until_ready(ready("hi")), ("hi", 1));
        assert_eq!(poll_until_ready(map(countdown(1), |n| n * 100)), (200, 2));
        let nested = map(map(ready(2), |x: i32| x + 1), |x| x.to_string());
        assert_eq!(poll_until_ready(nested), ("3".to_string(), 1));
    }

    #[test]
    #[should_panic(expected = "Ready polled after completion")]
    fn ready_panics_when_polled_twice() {
        let mut cx = Context::from_waker(Waker::noop());
        let mut future = ready(1);
        let _ = Pin::new(&mut future).poll(&mut cx);
        let _ = Pin::new(&mut future).poll(&mut cx);
    }

    #[test]
    fn async_blocks_are_futures_too() {
        assert_eq!(poll_until_ready(async { 1 + 2 }), (3, 1));
        let (value, polls) = poll_until_ready(async {
            let a = countdown(2).await;
            let b = ready(10).await;
            a + b
        });
        assert_eq!((value, polls), (13, 3));
    }
}
