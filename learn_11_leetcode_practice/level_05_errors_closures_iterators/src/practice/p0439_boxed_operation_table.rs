// ==================================================================
// PRACTICE 0439: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0439_boxed_operation_table.md
// Run the tests:          cargo test practice::p0439
// ==================================================================

//! # Problem 0439: A Table of Boxed Operations
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Box<dyn Fn>`, choosing a closure at run time, `Vec<Box<dyn Fn>>`, `try_fold` with `?`, calling boxed closures
//! - **Source:** Original · [The Rust Book 20.4: Returning closures](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html#returning-closures) · [std `Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)
//!
//! ## Problem
//!
//! `impl Fn` can only stand for **one** concrete closure type. When a function must return
//! *different* closures depending on its input, or when closures of different types must live in
//! the same `Vec`, put each closure in a box: `Box<dyn Fn(i64, i64) -> i64>`. The box has one known
//! type, and calls go through a pointer (dynamic dispatch).
//!
//! Implement:
//!
//! - `operation(name: &str) -> Option<Box<dyn Fn(i64, i64) -> i64>>` for the names `"add"`,
//!   `"sub"` (left minus right), `"mul"`, `"max"` and `"min"`; any other name gives `None`.
//! - `evaluate(start: i64, steps: &[(&str, i64)]) -> Option<i64>`: start with `start` and apply each
//!   step `(name, operand)` in order: `acc = op(acc, operand)`. An unknown name gives `None`.
//! - `scalers(factors: &[i64]) -> Vec<Box<dyn Fn(i64) -> i64>>`: one closure per factor that
//!   multiplies its argument by that factor.
//! - `apply_all(fs: &[Box<dyn Fn(i64) -> i64>], x: i64) -> Vec<i64>`: every closure applied to `x`.
//!
//! ## Examples
//!
//! ```text
//! operation("sub").map(|f| f(10, 3))                  -> Some(7)
//! operation("pow")                                    -> None (so `.is_none()` is true)
//! evaluate(2, &[("add", 3), ("mul", 4), ("max", 7)])  -> Some(20)    ((2 + 3) * 4 = 20, max(20, 7) = 20)
//! evaluate(2, &[("add", 3), ("pow", 2)])              -> None
//! apply_all(&scalers(&[1, 2, 10]), 5)                 -> [5, 10, 50]
//! ```
//!
//! ## Constraints
//!
//! - The tests never overflow an `i64`.
//!
//! ## Hints
//!
//! 1. Each `match` arm can return `Some(Box::new(|a, b| ...))`.
//! 2. `steps.iter().try_fold(start, |acc, &(name, x)| Some(operation(name)?(acc, x)))`.
//! 3. In `scalers`, `map(|&k| Box::new(move |x| x * k) as Box<dyn Fn(i64) -> i64>)`: the closure must
//!    `move` its factor, and the cast tells the compiler which box type you want.

pub fn operation(name: &str) -> Option<Box<dyn Fn(i64, i64) -> i64>> {
    todo!("Write your solution here")
}

pub fn evaluate(start: i64, steps: &[(&str, i64)]) -> Option<i64> {
    todo!("Write your solution here")
}

pub fn scalers(factors: &[i64]) -> Vec<Box<dyn Fn(i64) -> i64>> {
    todo!("Write your solution here")
}

pub fn apply_all(fs: &[Box<dyn Fn(i64) -> i64>], x: i64) -> Vec<i64> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_operations() {
        let call = |name: &str, a: i64, b: i64| operation(name).map(|f| f(a, b));
        assert_eq!(call("add", 10, 3), Some(13));
        assert_eq!(call("sub", 10, 3), Some(7));
        assert_eq!(call("mul", 10, 3), Some(30));
        assert_eq!(call("max", -10, 3), Some(3));
        assert_eq!(call("min", -10, 3), Some(-10));
    }

    #[test]
    fn unknown_operation() {
        assert!(operation("pow").is_none());
        assert!(operation("ADD").is_none());
        assert!(operation("").is_none());
    }

    #[test]
    fn evaluate_steps() {
        assert_eq!(evaluate(2, &[("add", 3), ("mul", 4), ("max", 7)]), Some(20));
        assert_eq!(evaluate(2, &[("add", 3), ("pow", 2)]), None);
        assert_eq!(evaluate(42, &[]), Some(42));
        assert_eq!(
            evaluate(0, &[("sub", 5), ("min", -10), ("mul", -1)]),
            Some(10)
        );
    }

    #[test]
    fn scalers_capture_their_factor() {
        let fs = scalers(&[1, 2, 10]);
        assert_eq!(fs.len(), 3);
        assert_eq!(apply_all(&fs, 5), vec![5, 10, 50]);
        assert_eq!(apply_all(&fs, -1), vec![-1, -2, -10]);
        assert_eq!(apply_all(&scalers(&[]), 5), Vec::<i64>::new());
    }

    #[test]
    fn apply_all_accepts_mixed_closures() {
        let offset = 100;
        let mixed: Vec<Box<dyn Fn(i64) -> i64>> = vec![
            Box::new(|x| x + 1),
            Box::new(move |x| x + offset),
            Box::new(i64::abs),
        ];
        assert_eq!(apply_all(&mixed, -7), vec![-6, 93, 7]);
    }
}
