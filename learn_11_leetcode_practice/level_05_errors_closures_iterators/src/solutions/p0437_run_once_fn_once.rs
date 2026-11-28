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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `FnOnce` is the bound to use when you call a closure at most once, because it accepts
//! every closure. To keep a one-shot closure inside a struct, box it (its type is unnameable) and
//! wrap it in an `Option`: `take()` is the "use it up" operation, and `None` records that it is gone.
//!
//! **Rust concepts:**
//! - `greeting + " world"` moves the captured `String`, so that closure is `FnOnce` only.
//! - `move` transfers ownership of `names` into the returned closure; `into_iter()` inside the body
//!   consumes it, making the closure `FnOnce` (with `names.join(", ")` it would only borrow and be
//!   `Fn`). `Iterator::reduce` folds the owned strings into the first one.
//! - `Box<dyn FnOnce() -> String>` erases the closure type; the `'static` bound means the closure
//!   may not borrow short-lived local variables.
//! - `Option::take` plus `map(|task| task())` runs the task exactly once without any flag variable.
// @solution-end

pub fn run_once<F: FnOnce() -> String>(f: F) -> String {
    // @solution-start
    f()
    // @solution-end
}

pub fn make_report(names: Vec<String>) -> impl FnOnce() -> String {
    // @solution-start replace: move || todo!()
    move || {
        let count = names.len();
        // `into_iter` moves the strings out of `names`; `reduce` reuses the first one's buffer.
        let joined = names
            .into_iter()
            .reduce(|mut acc, name| {
                acc.push_str(", ");
                acc.push_str(&name);
                acc
            })
            .unwrap_or_default();
        format!("{count} names: {joined}")
    }
    // @solution-end
}

pub struct OnceTask {
    // @solution-start remove
    task: Option<Box<dyn FnOnce() -> String>>,
    // @solution-end
}

impl OnceTask {
    pub fn new(task: impl FnOnce() -> String + 'static) -> OnceTask {
        // @solution-start
        OnceTask {
            task: Some(Box::new(task)),
        }
        // @solution-end
    }

    pub fn run(&mut self) -> Option<String> {
        // @solution-start
        self.task.take().map(|task| task())
        // @solution-end
    }

    pub fn is_done(&self) -> bool {
        // @solution-start
        self.task.is_none()
        // @solution-end
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
