// ==================================================================
// PRACTICE 0941: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0941_select_two_futures.md
// Run the tests:          cargo test practice::p0941
// ==================================================================

//! # Problem 0941: Race Two Futures
//!
//! - **Difficulty:** Hard
//! - **Topics:** `select`-style combinators, an `Either` output, cancellation by `Drop`, `async fn` built on a hand-written combinator
//! - **Source:** Original · [std: `Future`](https://doc.rust-lang.org/std/future/trait.Future.html) · [The Rust Book 17.3: Racing Futures](https://doc.rust-lang.org/book/ch17-03-more-futures.html)
//!
//! ## Problem
//!
//! `select(a, b)` polls two futures and completes with the output of **whichever finishes first**.
//! The other future is then simply dropped: in async Rust, dropping a future **cancels** it, and its
//! destructor runs at that moment.
//!
//! ```text
//! pub enum Either<L, R> { Left(L), Right(R) }
//! pub struct Select<A: Future, B: Future> { a: Option<Pin<Box<A>>>, b: Option<Pin<Box<B>>> }
//! ```
//!
//! Implement:
//!
//! 1. `select(a, b) -> Select<A, B>` with `Output = Either<A::Output, B::Output>`. Each poll polls
//!    `a` first, then `b`; if `a` is ready, `b` is not polled again. When one side finishes, drop the
//!    loser **immediately** (set its `Option` to `None`), then the finished winner, before returning.
//!    Polling a finished `Select` panics with `"Select polled after completion"`.
//! 2. `async fn with_poll_budget<F: Future>(future: F, budget: usize) -> Option<F::Output>`:
//!    `Some(output)` if `future` completes within `budget` polls, `None` otherwise. Build it by
//!    awaiting `select` of `future` and the given `Budget` future.
//!
//! Given for the tests: `block_on`, `Budget` (pending, and waking itself, for `n` polls, then ready),
//! and `Job` (logs its polls and its drop into a shared log, and finishes after a number of polls).
//!
//! ## Examples
//!
//! ```text
//! block_on(select(job("fast", 1, &log), job("slow", 5, &log)))   => Left("fast")
//!   log: ["fast polled", "slow dropped", "fast dropped"]
//! block_on(with_poll_budget(job("j", 3, &log), 5))               => Some("j")
//! block_on(with_poll_budget(job("j", 10, &log), 2))              => None
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `self.a.as_mut()` gives `Option<&mut Pin<Box<A>>>`; poll it with `future.as_mut().poll(cx)`.
//! 2. Assigning `self.b = None;` runs `B`'s destructor right there.
//! 3. `match select(future, Budget::new(budget)).await { Either::Left(..) => ..., Either::Right(()) => ... }`.

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct Select<A: Future, B: Future> {
    a: Option<Pin<Box<A>>>,
    b: Option<Pin<Box<B>>>,
}

pub fn select<A: Future, B: Future>(a: A, b: B) -> Select<A, B> {
    todo!("Write your solution here")
}

impl<A: Future, B: Future> Future for Select<A, B> {
    type Output = Either<A::Output, B::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Write your solution here")
    }
}

pub async fn with_poll_budget<F: Future>(future: F, budget: usize) -> Option<F::Output> {
    todo!("Write your solution here")
}

// ---------------------------------------------------------------- given helpers

struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Runs a future to completion on the current thread (see problem 0939).
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
            return output;
        }
        thread::park();
    }
}

/// Pending (and waking itself) for `n` polls, then ready.
pub struct Budget {
    left: usize,
}

impl Budget {
    pub fn new(n: usize) -> Self {
        Budget { left: n }
    }
}

impl Future for Budget {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.left == 0 {
            return Poll::Ready(());
        }
        self.left -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

/// Logs `"<name> polled"` on every poll and `"<name> dropped"` when dropped; ready after `polls` polls.
pub struct Job {
    name: &'static str,
    polls_left: usize,
    log: Rc<RefCell<Vec<String>>>,
}

pub fn job(name: &'static str, polls: usize, log: &Rc<RefCell<Vec<String>>>) -> Job {
    Job {
        name,
        polls_left: polls,
        log: Rc::clone(log),
    }
}

impl Future for Job {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        let entry = format!("{} polled", self.name);
        self.log.borrow_mut().push(entry);
        self.polls_left = self.polls_left.saturating_sub(1);
        if self.polls_left == 0 {
            Poll::Ready(self.name)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        let entry = format!("{} dropped", self.name);
        self.log.borrow_mut().push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_log() -> Rc<RefCell<Vec<String>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[test]
    fn the_faster_future_wins_and_the_loser_is_dropped_at_once() {
        let log = new_log();
        let winner = block_on(select(job("fast", 1, &log), job("slow", 5, &log)));
        assert_eq!(winner, Either::Left("fast"));
        assert_eq!(
            *log.borrow(),
            ["fast polled", "slow dropped", "fast dropped"]
        );
    }

    #[test]
    fn right_side_can_win() {
        let log = new_log();
        let winner = block_on(select(job("a", 3, &log), job("b", 2, &log)));
        assert_eq!(winner, Either::Right("b"));
        assert_eq!(
            *log.borrow(),
            [
                "a polled",
                "b polled",
                "a polled",
                "b polled",
                "a dropped",
                "b dropped"
            ]
        );
    }

    #[test]
    fn left_wins_ties() {
        let log = new_log();
        let winner = block_on(select(job("l", 2, &log), job("r", 2, &log)));
        assert_eq!(winner, Either::Left("l"));
        // `r` was not polled a second time.
        assert_eq!(
            *log.borrow(),
            ["l polled", "r polled", "l polled", "r dropped", "l dropped"]
        );
    }

    #[test]
    #[should_panic(expected = "Select polled after completion")]
    fn polling_after_completion_panics() {
        let mut cx = Context::from_waker(Waker::noop());
        let mut race = select(async { 1 }, async { 2 });
        let mut race = Pin::new(&mut race);
        let _ = race.as_mut().poll(&mut cx);
        let _ = race.as_mut().poll(&mut cx);
    }

    #[test]
    fn poll_budgets() {
        let log = new_log();
        assert_eq!(block_on(with_poll_budget(job("j", 3, &log), 5)), Some("j"));
        assert_eq!(block_on(with_poll_budget(job("k", 10, &log), 2)), None);
        assert_eq!(
            log.borrow()
                .iter()
                .filter(|e| e.starts_with("k polled"))
                .count(),
            3
        );
        assert_eq!(block_on(with_poll_budget(async { 7 }, 0)), Some(7));
    }
}
