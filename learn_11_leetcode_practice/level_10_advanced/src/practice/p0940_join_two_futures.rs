// ==================================================================
// PRACTICE 0940: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0940_join_two_futures.md
// Run the tests:          cargo test practice::p0940
// ==================================================================

//! # Problem 0940: Join Two Futures
//!
//! - **Difficulty:** Hard
//! - **Topics:** combinator futures, `Pin<Box<F>>`, a `MaybeDone` state enum, manual `Unpin` impls, concurrency without threads
//! - **Source:** Original · [std: `pin` module](https://doc.rust-lang.org/std/pin/index.html) · [The Rust Book 17.3: Working with Any Number of Futures](https://doc.rust-lang.org/book/ch17-03-more-futures.html)
//!
//! ## Problem
//!
//! `join(a, b)` runs two futures **concurrently** on one thread: every time the joined future is
//! polled it polls each child that is not finished yet, and it completes when both have completed,
//! with both outputs. `join_all` does the same for a `Vec` of futures.
//!
//! Use this state type for each child (its `Taken` variant marks an output already returned):
//!
//! ```text
//! enum MaybeDone<F: Future> { Running(Pin<Box<F>>), Done(F::Output), Taken }
//! ```
//!
//! Boxing the child (`Pin<Box<F>>`) means it can be polled with `as_mut()` without any unsafe pin
//! projection, for **any** `F`, even one that is not `Unpin`.
//!
//! Implement:
//!
//! - `MaybeDone::poll_child(&mut self, cx) -> bool`: polls a `Running` child; stores its output and
//!   switches to `Done` when it is ready. Returns `true` if the child is now finished.
//!   `take_output(&mut self) -> F::Output` moves a `Done` output out (it becomes `Taken`); calling
//!   it in any other state is a bug and panics.
//! - `Join<A, B>` from `join(a, b)`, with `Output = (A::Output, B::Output)`. Poll `a` before `b`.
//! - `JoinAll<F>` from `join_all(futures: Vec<F>)`, with `Output = Vec<F::Output>` in the **original
//!   order**, whatever order the children finish in. `join_all(vec![])` is ready immediately.
//! - Both combinators are `Unpin` for any children (write the `impl Unpin`s): they never pin their
//!   fields in place, the boxes do.
//!
//! `block_on` (problem 0939) and a self-waking `Steps` future are given for the tests. `Steps`
//! pushes `"<name><k>"` into a shared log on each poll and finishes after a given number of polls.
//!
//! ## Examples
//!
//! ```text
//! block_on(join(async { 1 }, async { "two" }))      => (1, "two")
//! join(steps("a", 2, &log), steps("b", 3, &log)) logs: a0 b0 a1 b1 b2   (interleaved)
//! block_on(join_all(vec![steps("x", 3, &log), steps("y", 1, &log)])) => ["x", "y"]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `poll_child`: `match self { MaybeDone::Running(f) => if let Poll::Ready(out) = f.as_mut().poll(cx) { *self = MaybeDone::Done(out) } ... }`.
//! 2. `Join::poll`: poll both children (do not short-circuit with `&&`: both must make progress), then
//!    return `Ready` only if both are finished.
//! 3. `std::mem::replace(self, MaybeDone::Taken)` moves the output out of `&mut self`.

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

pub enum MaybeDone<F: Future> {
    Running(Pin<Box<F>>),
    Done(F::Output),
    Taken,
}

impl<F: Future> MaybeDone<F> {
    pub fn new(future: F) -> Self {
        MaybeDone::Running(Box::pin(future))
    }

    pub fn poll_child(&mut self, cx: &mut Context<'_>) -> bool {
        todo!("Write your solution here")
    }

    pub fn take_output(&mut self) -> F::Output {
        todo!("Write your solution here")
    }
}

pub struct Join<A: Future, B: Future> {
    a: MaybeDone<A>,
    b: MaybeDone<B>,
}

impl<A: Future, B: Future> Unpin for Join<A, B> {}

pub fn join<A: Future, B: Future>(a: A, b: B) -> Join<A, B> {
    todo!("Write your solution here")
}

impl<A: Future, B: Future> Future for Join<A, B> {
    type Output = (A::Output, B::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Write your solution here")
    }
}

pub struct JoinAll<F: Future> {
    children: Vec<MaybeDone<F>>,
}

impl<F: Future> Unpin for JoinAll<F> {}

pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    todo!("Write your solution here")
}

impl<F: Future> Future for JoinAll<F> {
    type Output = Vec<F::Output>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!("Write your solution here")
    }
}

// ---------------------------------------------------------------- given helpers for the tests

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

/// A future that logs `"<name><k>"` on its `k`-th poll and finishes (with its name) after `polls` polls.
pub struct Steps {
    name: &'static str,
    polls: usize,
    done: usize,
    log: Rc<RefCell<Vec<String>>>,
}

pub fn steps(name: &'static str, polls: usize, log: &Rc<RefCell<Vec<String>>>) -> Steps {
    Steps {
        name,
        polls,
        done: 0,
        log: Rc::clone(log),
    }
}

impl Future for Steps {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        let entry = format!("{}{}", self.name, self.done);
        self.log.borrow_mut().push(entry);
        self.done += 1;
        if self.done >= self.polls {
            Poll::Ready(self.name)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_log() -> Rc<RefCell<Vec<String>>> {
        Rc::new(RefCell::new(Vec::new()))
    }

    #[test]
    fn join_returns_both_outputs() {
        assert_eq!(block_on(join(async { 1 }, async { "two" })), (1, "two"));
    }

    #[test]
    fn join_interleaves_the_children() {
        let log = new_log();
        let out = block_on(join(steps("a", 2, &log), steps("b", 3, &log)));
        assert_eq!(out, ("a", "b"));
        assert_eq!(*log.borrow(), ["a0", "b0", "a1", "b1", "b2"]);
    }

    #[test]
    fn join_all_keeps_the_original_order() {
        let log = new_log();
        let out = block_on(join_all(vec![
            steps("x", 3, &log),
            steps("y", 1, &log),
            steps("z", 2, &log),
        ]));
        assert_eq!(out, ["x", "y", "z"]);
        assert_eq!(*log.borrow(), ["x0", "y0", "z0", "x1", "z1", "x2"]);
        let none: Vec<u8> = block_on(join_all(Vec::<std::future::Ready<u8>>::new()));
        assert!(none.is_empty());
    }

    #[test]
    fn not_unpin_children_are_fine() {
        // `async` blocks that hold a borrow across `.await` are not `Unpin`.
        let log = new_log();
        let first = async {
            let text = String::from("borrowed");
            let view = &text;
            steps("p", 2, &log).await;
            view.len()
        };
        let second = async { steps("q", 1, &log).await };
        assert_eq!(block_on(join(first, second)), (8, "q"));
    }

    #[test]
    fn maybe_done_by_hand() {
        let mut cx = Context::from_waker(Waker::noop());
        let mut child = MaybeDone::new(std::future::ready(7));
        assert!(child.poll_child(&mut cx));
        assert!(child.poll_child(&mut cx));
        assert_eq!(child.take_output(), 7);
        assert!(matches!(child, MaybeDone::Taken));
    }

    #[test]
    fn many_children() {
        let log = new_log();
        let futures: Vec<Steps> = (0..100).map(|i| steps("s", i % 5 + 1, &log)).collect();
        let out = block_on(join_all(futures));
        assert_eq!(out.len(), 100);
        assert_eq!(
            log.borrow().len(),
            (0..100).map(|i| i % 5 + 1).sum::<usize>()
        );
    }
}
