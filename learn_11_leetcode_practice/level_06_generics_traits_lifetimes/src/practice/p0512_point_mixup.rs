// ==================================================================
// PRACTICE 0512: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0512_point_mixup.md
// Run the tests:          cargo test practice::p0512
// ==================================================================

//! # Problem 0512: Mixing Two Generic Points
//!
//! - **Difficulty:** Easy
//! - **Topics:** structs with several type parameters, generic methods with their own parameters, `FnOnce`
//! - **Source:** Original · [The Rust Book 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions)
//!
//! ## Problem
//!
//! Define `Point<X, Y>` with public fields `x: X` and `y: Y`: the two coordinates may have
//! **different** types. Derive `Debug`, `Clone` and `PartialEq`. Implement, in one
//! `impl<X, Y> Point<X, Y>` block:
//!
//! - `new(x: X, y: Y) -> Self`
//! - `mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2>`: a new point with `x` from `self`
//!   and `y` from `other`. Both points are consumed.
//! - `swap(self) -> Point<Y, X>`: exchanges the coordinates (and therefore their types).
//! - `map_x<X2, F>(self, f: F) -> Point<X2, Y> where F: FnOnce(X) -> X2`: applies `f` to `x` and
//!   keeps `y` unchanged.
//!
//! `mixup` and `map_x` declare type parameters of their **own** (`X2`, `Y2`, `F`), in addition to
//! the ones of the struct.
//!
//! ## Examples
//!
//! ```text
//! let p1 = Point::new(5, 10.4);
//! let p2 = Point::new("Hello", 'c');
//! p1.mixup(p2)                            => Point { x: 5, y: 'c' }
//! Point::new(1, "one").swap()             => Point { x: "one", y: 1 }
//! Point::new(3, true).map_x(|n| n * 2)    => Point { x: 6, y: true }
//! Point::new(3, true).map_x(|n| n.to_string()) => Point { x: "3", y: true }
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. The struct's parameters are declared after `impl`; a method's own parameters are declared after
//!    the method name: `fn mixup<X2, Y2>(...)`.
//! 2. Because `self` is taken by value you can move its fields out: `Point { x: self.x, y: other.y }`.

/// A point whose coordinates may have different types.
#[derive(Debug, Clone, PartialEq)]
pub struct Point<X, Y> {
    pub x: X,
    pub y: Y,
}

impl<X, Y> Point<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        todo!("Write your solution here")
    }

    /// `x` from `self`, `y` from `other`.
    pub fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X, Y2> {
        todo!("Write your solution here")
    }

    /// Exchanges the coordinates.
    pub fn swap(self) -> Point<Y, X> {
        todo!("Write your solution here")
    }

    /// Applies `f` to `x`.
    pub fn map_x<X2, F>(self, f: F) -> Point<X2, Y>
    where
        F: FnOnce(X) -> X2,
    {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixup() {
        let p1 = Point::new(5, 10.4);
        let p2 = Point::new("Hello", 'c');
        assert_eq!(p1.mixup(p2), Point { x: 5, y: 'c' });
    }

    #[test]
    fn mixup_with_owned_values() {
        let a = Point::new(String::from("kept"), vec![1, 2]);
        let b = Point::new(0_u8, String::from("taken"));
        let m = a.mixup(b);
        assert_eq!(m.x, "kept");
        assert_eq!(m.y, "taken");
    }

    #[test]
    fn swap_changes_types() {
        assert_eq!(Point::new(1, "one").swap(), Point { x: "one", y: 1 });
        let twice = Point::new('z', 2.5).swap().swap();
        assert_eq!(twice, Point::new('z', 2.5));
    }

    #[test]
    fn map_x_examples() {
        assert_eq!(
            Point::new(3, true).map_x(|n| n * 2),
            Point { x: 6, y: true }
        );
        assert_eq!(
            Point::new(3, true).map_x(|n| n.to_string()),
            Point {
                x: "3".to_string(),
                y: true
            }
        );
        let suffix = String::from("!");
        let p = Point::new(String::from("hi"), 0).map_x(move |s| s + &suffix);
        assert_eq!(p.x, "hi!");
    }
}
