//! # Problem 0435: Apply a Closure n Times
//!
//! - **Difficulty:** Easy
//! - **Topics:** closure syntax, generic parameter `F: Fn(..) -> ..`, `impl Fn` in argument position, closures capturing variables, passing functions as values
//! - **Source:** Original · [The Rust Book 13.1: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html) · [Rust By Example: Closures as input parameters](https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html)
//!
//! ## Problem
//!
//! A closure is an anonymous function that can capture variables from the place where it is
//! written: `let offset = 10; let add = |x| x + offset;`. Functions can accept closures through a
//! generic parameter bounded by one of the closure traits. `Fn` is the most restrictive for the
//! closure (it may only *read* what it captured), which also means the function may call it as
//! many times as it wants.
//!
//! Implement:
//!
//! - `apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32`: `f(x)`.
//! - `apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32`: `f(f(x))`.
//! - `apply_n<F: Fn(i64) -> i64>(f: F, n: usize, x: i64) -> i64`: `f` applied `n` times (`n = 0`
//!   returns `x`).
//! - `map_all(values: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32>`: `f` applied to every value.
//! - `count_where(values: &[i32], pred: impl Fn(i32) -> bool) -> usize`: how many values satisfy
//!   `pred`.
//!
//! ## Examples
//!
//! ```text
//! apply(|x| x * x, 7)                   -> 49
//! let offset = 10;
//! apply_twice(|x| x + offset, 1)        -> 21
//! apply_n(|x| x * 2, 10, 1)             -> 1024
//! apply_n(|x| x + 1, 0, 5)              -> 5
//! map_all(&[1, 2, 3], |x| x - 1)        -> [0, 1, 2]
//! count_where(&[3, 8, 1, 9], |x| x > 2) -> 3
//! ```
//!
//! ## Constraints
//!
//! - `n <= 10^5`. The tests never overflow.
//!
//! ## Hints
//!
//! 1. Calling a closure stored in a parameter is ordinary call syntax: `f(x)`.
//! 2. `apply_n` is a `fold` over `0..n`: the accumulator starts at `x` and becomes `f(acc)`.
//! 3. `impl Fn(i32) -> i32` in argument position is short for an anonymous generic parameter.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the functions only call `f`; everything interesting happens at the call site, where
//! the caller decides what the closure does and what it captures.
//!
//! **Rust concepts:**
//! - Each closure has its own unique, unnameable type, so functions take closures through generics
//!   (`F: Fn(i32) -> i32`) or `impl Fn(i32) -> i32`; both are compiled once per closure type
//!   (static dispatch, no allocation).
//! - `Fn` closures can be called many times through a shared reference; a regular `fn` item such as
//!   `i32::abs` also implements `Fn` and can be passed the same way.
//! - `(0..n).fold(x, |acc, _| f(acc))` repeats an action `n` times without a mutable variable.
//! - Inside `map_all`, `|&v| f(v)` destructures the `&i32` items of `iter()`.
// @solution-end

pub fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    // @solution-start
    f(x)
    // @solution-end
}

pub fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    // @solution-start
    f(f(x))
    // @solution-end
}

pub fn apply_n<F: Fn(i64) -> i64>(f: F, n: usize, x: i64) -> i64 {
    // @solution-start
    (0..n).fold(x, |acc, _| f(acc))
    // @solution-end
}

pub fn map_all(values: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    // @solution-start
    values.iter().map(|&v| f(v)).collect()
    // @solution-end
}

pub fn count_where(values: &[i32], pred: impl Fn(i32) -> bool) -> usize {
    // @solution-start
    values.iter().filter(|&&v| pred(v)).count()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(x: i32) -> i32 {
        x * x
    }

    #[test]
    fn apply_closures_and_functions() {
        assert_eq!(apply(|x| x * x, 7), 49);
        assert_eq!(apply(square, -3), 9);
        assert_eq!(apply(i32::abs, -12), 12);
    }

    #[test]
    fn apply_twice_with_a_captured_variable() {
        let offset = 10;
        assert_eq!(apply_twice(|x| x + offset, 1), 21);
        assert_eq!(apply_twice(square, 3), 81);
    }

    #[test]
    fn apply_n_times() {
        assert_eq!(apply_n(|x| x * 2, 10, 1), 1024);
        assert_eq!(apply_n(|x| x + 1, 0, 5), 5);
        assert_eq!(apply_n(|x| x + 3, 100_000, 0), 300_000);
    }

    #[test]
    fn map_all_values() {
        assert_eq!(map_all(&[1, 2, 3], |x| x - 1), vec![0, 1, 2]);
        let factor = 3;
        assert_eq!(map_all(&[4, -5], |x| x * factor), vec![12, -15]);
        assert_eq!(map_all(&[], square), Vec::<i32>::new());
    }

    #[test]
    fn count_matching_values() {
        assert_eq!(count_where(&[3, 8, 1, 9], |x| x > 2), 3);
        let limit = 100;
        assert_eq!(count_where(&[100, 101, 99], |x| x >= limit), 2);
        assert_eq!(count_where(&[], |_| true), 0);
    }
}
