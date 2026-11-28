// ==================================================================
// PRACTICE 0513: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0513_pair_conditional_methods.md
// Run the tests:          cargo test practice::p0513
// ==================================================================

//! # Problem 0513: Pair With Conditional Methods
//!
//! - **Difficulty:** Easy
//! - **Topics:** trait bounds on impl blocks, conditional methods, `PartialOrd`, `Display`, `Add`
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)
//!
//! ## Problem
//!
//! Define `Pair<T>` with public fields `x: T` and `y: T`. Its methods are spread over several impl
//! blocks, each with different bounds, so a method only exists when `T` supports it:
//!
//! | Impl block | Methods |
//! | --- | --- |
//! | `impl<T> Pair<T>` | `new(x: T, y: T) -> Self` |
//! | `impl<T: PartialOrd> Pair<T>` | `largest(&self) -> &T`, `into_ordered(self) -> (T, T)` |
//! | `impl<T: PartialOrd + Display> Pair<T>` | `cmp_display(&self) -> String` |
//! | `impl<T: Copy + Add<Output = T>> Pair<T>` | `sum(&self) -> T` |
//!
//! Rules:
//!
//! - `largest` returns a reference to `x` when `x >= y`, otherwise to `y`.
//! - `into_ordered` returns `(smaller, larger)`; when they are equal, `(x, y)`.
//! - `cmp_display` returns `"The largest member is x = 7"` or `"The largest member is y = 7"`,
//!   choosing the same member as `largest`.
//! - `sum` returns `x + y`.
//!
//! ## Examples
//!
//! ```text
//! Pair::new(3, 7).largest()          => &7
//! Pair::new(3, 7).cmp_display()      => "The largest member is y = 7"
//! Pair::new("b", "a").into_ordered() => ("a", "b")
//! Pair::new(1.5, 2.0).sum()          => 3.5
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Write `impl<T: PartialOrd> Pair<T> { ... }`: every method inside can use `>=` on `T`.
//! 2. `Vec<i32>` is `PartialOrd` but not `Display` or `Copy`, so a `Pair<Vec<i32>>` has `largest`
//!    but not `cmp_display` or `sum`.

use std::fmt::Display;
use std::ops::Add;

/// Two values of the same type.
#[derive(Debug, Clone, PartialEq)]
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        todo!("Write your solution here")
    }
}

impl<T: PartialOrd> Pair<T> {
    /// `&x` when `x >= y`, otherwise `&y`.
    pub fn largest(&self) -> &T {
        todo!("Write your solution here")
    }

    /// `(smaller, larger)`, or `(x, y)` when equal.
    pub fn into_ordered(self) -> (T, T) {
        todo!("Write your solution here")
    }
}

impl<T: PartialOrd + Display> Pair<T> {
    /// `"The largest member is x = .."` or `"The largest member is y = .."`.
    pub fn cmp_display(&self) -> String {
        todo!("Write your solution here")
    }
}

impl<T: Copy + Add<Output = T>> Pair<T> {
    /// `x + y`.
    pub fn sum(&self) -> T {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_and_display() {
        let p = Pair::new(3, 7);
        assert_eq!(*p.largest(), 7);
        assert_eq!(p.cmp_display(), "The largest member is y = 7");
        let q = Pair::new(9, 9);
        assert_eq!(q.cmp_display(), "The largest member is x = 9");
        assert!(std::ptr::eq(q.largest(), &q.x));
    }

    #[test]
    fn ordering_strings() {
        assert_eq!(Pair::new("b", "a").into_ordered(), ("a", "b"));
        assert_eq!(Pair::new("a", "b").into_ordered(), ("a", "b"));
        let owned = Pair::new(String::from("pear"), String::from("fig"));
        assert_eq!(owned.cmp_display(), "The largest member is x = pear");
    }

    #[test]
    fn sums() {
        assert_eq!(Pair::new(1.5, 2.0).sum(), 3.5);
        assert_eq!(Pair::new(-4_i64, 10).sum(), 6);
    }

    #[test]
    fn partial_ord_only_types() {
        // Vec<i32> is PartialOrd (lexicographic) but neither Display nor Copy.
        let p = Pair::new(vec![1, 2, 3], vec![1, 3]);
        assert_eq!(p.largest(), &vec![1, 3]);
        assert_eq!(p.into_ordered(), (vec![1, 2, 3], vec![1, 3]));
    }
}
