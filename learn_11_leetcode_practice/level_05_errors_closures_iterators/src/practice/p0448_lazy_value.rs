// ==================================================================
// PRACTICE 0448: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0448_lazy_value.md
// Run the tests:          cargo test practice::p0448
// ==================================================================

//! # Problem 0448: Lazy Initialization with FnOnce
//!
//! - **Difficulty:** Medium
//! - **Topics:** struct generic over `F: FnOnce() -> T`, `Option::take`, `Option::get_or_insert_with`, disjoint field capture, consuming `self`
//! - **Source:** Original · [std `Option::get_or_insert_with`](https://doc.rust-lang.org/std/option/enum.Option.html#method.get_or_insert_with) · [std `FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)
//!
//! ## Problem
//!
//! Some values are expensive to build and not always needed: a parsed configuration, a big lookup
//! table... *Lazy initialization* stores **how** to build the value (a closure) and runs it only the
//! first time the value is requested. Because the builder runs at most once, it only needs to be
//! `FnOnce`.
//!
//! `Lazy<T, F>` holds two fields: `init: Option<F>` (the builder, until it is used) and
//! `value: Option<T>` (the value, once built). Implement:
//!
//! - `Lazy::new(init: F) -> Lazy<T, F>`: nothing is computed yet.
//! - `is_initialized(&self) -> bool`.
//! - `get(&mut self) -> &T`: build the value on the first call, then return a reference to it.
//! - `into_value(self) -> T`: build the value if needed and return it by value.
//!
//! ## Examples
//!
//! ```text
//! let mut config = Lazy::new(|| "port=8080".to_string());
//! config.is_initialized()  -> false
//! config.get()             -> "port=8080"   (the closure runs now)
//! config.get()             -> "port=8080"   (no second run)
//! config.is_initialized()  -> true
//! Lazy::new(|| vec![1, 2, 3]).into_value() -> [1, 2, 3]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Invariant: exactly one of `init` and `value` is `Some`.
//! 2. `self.value.get_or_insert_with(|| ...)` runs its closure only when `value` is `None`. Inside,
//!    `self.init.take()` gives you the builder; borrowing `self.init` and `self.value` at the same
//!    time is fine because closures capture individual fields.
//! 3. `into_value` can call `get` first, then move the value out of `self.value`.

pub struct Lazy<T, F: FnOnce() -> T> {
    init: Option<F>,
    value: Option<T>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Lazy<T, F> {
        todo!("Write your solution here")
    }

    pub fn is_initialized(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn get(&mut self) -> &T {
        todo!("Write your solution here")
    }

    pub fn into_value(mut self) -> T {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_built_on_first_get() {
        let mut config = Lazy::new(|| "port=8080".to_string());
        assert!(!config.is_initialized());
        assert_eq!(config.get(), "port=8080");
        assert!(config.is_initialized());
        assert_eq!(config.get(), "port=8080");
    }

    #[test]
    fn builder_runs_at_most_once() {
        let mut runs = 0;
        {
            let mut lazy = Lazy::new(|| {
                runs += 1;
                42
            });
            assert_eq!(*lazy.get(), 42);
            assert_eq!(*lazy.get(), 42);
            assert_eq!(*lazy.get(), 42);
        }
        assert_eq!(runs, 1);
    }

    #[test]
    fn builder_never_runs_if_unused() {
        let mut runs = 0;
        {
            let lazy = Lazy::new(|| {
                runs += 1;
                0
            });
            assert!(!lazy.is_initialized());
        }
        assert_eq!(runs, 0);
    }

    #[test]
    fn into_value_builds_or_reuses() {
        assert_eq!(Lazy::new(|| vec![1, 2, 3]).into_value(), vec![1, 2, 3]);
        let mut lazy = Lazy::new(|| String::from("once"));
        lazy.get();
        assert_eq!(lazy.into_value(), "once");
    }

    #[test]
    fn builder_can_consume_captured_data() {
        let words = vec!["a".to_string(), "b".to_string()];
        let mut joined = Lazy::new(move || words.into_iter().collect::<String>());
        assert_eq!(joined.get(), "ab");
    }
}
