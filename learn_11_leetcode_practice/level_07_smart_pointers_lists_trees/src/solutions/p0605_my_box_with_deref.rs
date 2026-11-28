//! # Problem 0605: MyBox with Deref
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Deref` trait, `*` operator on custom types, deref coercion, generic tuple structs
//! - **Source:** Original · [The Rust Book 15.2](https://doc.rust-lang.org/book/ch15-02-deref.html) · [`std::ops::Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
//!
//! ## Problem
//!
//! Build a tiny smart pointer, `MyBox<T>`, that wraps one value (unlike `Box`, it keeps the value
//! inline instead of on the heap; the point is the trait, not the allocation):
//!
//! ```text
//! pub struct MyBox<T>(T);
//! ```
//!
//! 1. `MyBox::new(value: T) -> MyBox<T>` wraps a value.
//! 2. Implement `std::ops::Deref` for `MyBox<T>` with `Target = T`, so that `*my_box` reads the
//!    inner value and methods of `T` can be called directly on a `MyBox<T>`.
//! 3. Implement `greet_boxed(name: &MyBox<String>) -> String`, which must return exactly what the
//!    given function `greet` returns for the inner string. Call `greet` with `name` itself (no
//!    `.0`, no `&**`, no `.as_str()`): deref coercion must do the conversion.
//!
//! The function `greet(name: &str) -> String` is given and returns `"Hello, <name>!"`.
//!
//! ## Examples
//!
//! ```text
//! let x = MyBox::new(5);
//! *x == 5
//!
//! greet_boxed(&MyBox::new(String::from("Rust"))) == "Hello, Rust!"
//!
//! let v = MyBox::new(vec![1, 2, 3]);
//! v.len() == 3          // Vec::len, reached through Deref
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `impl<T> Deref for MyBox<T> { type Target = T; fn deref(&self) -> &T { ... } }`: return a
//!    reference to the first (and only) field, `self.0`.
//! 2. `*x` on a type that is not a reference is rewritten by the compiler as `*(x.deref())`.
//! 3. In `greet_boxed`, the compiler applies `deref` as many times as needed:
//!    `&MyBox<String>` becomes `&String`, which becomes `&str` (because `String: Deref<Target = str>`).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `deref` returns `&self.0`. Everything else follows from the language rules: the `*`
//! operator calls `deref` for us, method calls auto-deref, and function arguments are coerced
//! through any chain of `Deref` implementations.
//!
//! **Complexity:** `O(1)` for every operation.
//!
//! **Rust concepts:**
//! - `Deref::deref` returns a *reference*; `*x` becomes `*(x.deref())`, so the value is not moved
//!   out of the wrapper.
//! - Deref coercion happens at compile time, so it costs nothing at run time.
//! - Implementing `Deref` is meant for smart pointers and wrappers, not as a general way to
//!   "inherit" methods.
// @solution-end

use std::ops::Deref;

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(value: T) -> MyBox<T> {
        // @solution-start
        MyBox(value)
        // @solution-end
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // @solution-start
        &self.0
        // @solution-end
    }
}

/// Given: greets a name.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn greet_boxed(name: &MyBox<String>) -> String {
    // @solution-start
    greet(name)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_reads_the_inner_value() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_coercion_reaches_str() {
        let name = MyBox::new(String::from("Rust"));
        assert_eq!(greet_boxed(&name), "Hello, Rust!");
        assert_eq!(greet_boxed(&MyBox::new(String::new())), "Hello, !");
    }

    #[test]
    fn methods_of_the_inner_type_are_available() {
        let v = MyBox::new(vec![1, 2, 3]);
        assert_eq!(v.len(), 3);
        assert_eq!(v.iter().sum::<i32>(), 6);
        let s = MyBox::new(String::from("hello"));
        assert!(s.starts_with("he"));
        assert_eq!(s.to_uppercase(), "HELLO");
    }

    #[test]
    fn nested_boxes_deref_twice() {
        let nested = MyBox::new(MyBox::new(7));
        assert_eq!(**nested, 7);
    }
}
