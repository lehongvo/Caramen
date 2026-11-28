// ==================================================================
// PRACTICE 0437: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0437_run_once_fn_once.md
// Run the tests:          cargo test practice::p0437
// ==================================================================

//! # Problem 0437: Run a Task Exactly Once with FnOnce
//!
//! - **Difficulty:** Easy
//! - **Topics:** `FnOnce`, closures that move captured values out, `Box<dyn FnOnce>`, `Option::take`, returning `impl FnOnce`
//! - **Source:** Original · [std `FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) · [The Rust Book 13.1](https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits)
//!
//! ## Problem
//!
//! Some closures can only run **once** because running them gives away something they own: a
//! closure that returns a captured `String`, or that calls `into_iter()` on a captured `Vec`, has
//! nothing left afterwards. Such closures implement only `FnOnce`, and calling one consumes it.
//!
//! Implement:
//!
//! - `run_once<F: FnOnce() -> String>(f: F) -> String`: call `f` and return its result.
//! - `make_report(names: Vec<String>) -> impl FnOnce() -> String`: return a closure that takes
//!   ownership of `names` and, when called, consumes them to produce `"N names: A, B, C"`
//!   (`"0 names: "` for an empty list).
//! - `OnceTask`, a slot that holds a boxed `FnOnce() -> String`:
//!   - `OnceTask::new(task: impl FnOnce() -> String + 'static) -> OnceTask`;
//!   - `run(&mut self) -> Option<String>`: the first call runs the task and returns `Some(result)`;
//!     every later call returns `None`;
//!   - `is_done(&self) -> bool`: whether the task has already run.
//!
//! ## Examples
//!
//! ```text
//! let greeting = String::from("hello");
//! run_once(move || greeting + " world")                 -> "hello world"
//! make_report(vec!["ann", "bob"])()                     -> "2 names: ann, bob"
//! let mut task = OnceTask::new(|| "done".to_string());
//! task.is_done() -> false;  task.run() -> Some("done");  task.run() -> None;  task.is_done() -> true
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. An `FnOnce` value is called with ordinary syntax, `f()`, which moves it.
//! 2. Store the task as `Option<Box<dyn FnOnce() -> String>>`. `self.task.take()` moves the box out
//!    and leaves `None`, so the task cannot run twice.
//! 3. A `Box<dyn FnOnce() -> String>` can be called directly: `boxed()`.

pub fn run_once<F: FnOnce() -> String>(f: F) -> String {
    todo!("Write your solution here")
}

pub fn make_report(names: Vec<String>) -> impl FnOnce() -> String {
    move || todo!()
}

pub struct OnceTask {}

impl OnceTask {
    pub fn new(task: impl FnOnce() -> String + 'static) -> OnceTask {
        todo!("Write your solution here")
    }

    pub fn run(&mut self) -> Option<String> {
        todo!("Write your solution here")
    }

    pub fn is_done(&self) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_once_consumes_a_moving_closure() {
        let greeting = String::from("hello");
        assert_eq!(run_once(move || greeting + " world"), "hello world");
        assert_eq!(run_once(|| String::from("plain")), "plain");
    }

    #[test]
    fn report_closure_owns_the_names() {
        let names = vec!["ann".to_string(), "bob".to_string()];
        let report = make_report(names);
        assert_eq!(report(), "2 names: ann, bob");
        assert_eq!(make_report(Vec::new())(), "0 names: ");
        assert_eq!(make_report(vec!["cy".to_string()])(), "1 names: cy");
    }

    #[test]
    fn once_task_runs_only_once() {
        let mut task = OnceTask::new(|| "done".to_string());
        assert!(!task.is_done());
        assert_eq!(task.run(), Some("done".to_string()));
        assert!(task.is_done());
        assert_eq!(task.run(), None);
        assert_eq!(task.run(), None);
    }

    #[test]
    fn once_task_can_move_captured_data() {
        let message = String::from("bye");
        let mut task = OnceTask::new(move || message + "!");
        assert_eq!(task.run().as_deref(), Some("bye!"));
        assert_eq!(task.run(), None);
    }
}
