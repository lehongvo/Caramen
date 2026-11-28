// ==================================================================
// PRACTICE 0596: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0596_blanket_impls.md
// Run the tests:          cargo test practice::p0596
// ==================================================================

//! # Problem 0596: Blanket Implementations
//!
//! - **Difficulty:** Medium
//! - **Topics:** `impl<T: Display + ?Sized> Trait for T`, blanket impls for `&T` and `Box<T>`, `?Sized` so that `str` and `dyn Trait` are covered, coherence
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)
//!
//! ## Problem
//!
//! A **blanket impl** implements a trait for every type that satisfies some bounds. The standard
//! library uses them everywhere (`impl<T: Display + ?Sized> ToString for T`). Write three:
//!
//! 1. `pub trait Shout { fn shout(&self) -> String; }` with
//!    `impl<T: fmt::Display + ?Sized> Shout for T`: the `Display` text uppercased with `"!"` appended.
//!    Thanks to `?Sized` it works on `str` itself, not only on `&str`.
//! 2. `pub trait DebugTag { fn debug_tag(&self) -> String; }` with
//!    `impl<T: fmt::Debug + ?Sized> DebugTag for T`: `"<{:?}>"`, e.g. `"<[1, 2]>"`.
//! 3. A local trait `pub trait Area { fn area(&self) -> f64; }` implemented for `Square { side }`, plus
//!    two **forwarding** blanket impls so that references and boxes are shapes too:
//!    `impl<T: Area + ?Sized> Area for &T` and `impl<T: Area + ?Sized> Area for Box<T>`.
//!    Then `total_area<A: Area>(shapes: &[A]) -> f64` accepts `&[Square]`, `&[&Square]` **and**
//!    `&[Box<dyn Area>]`.
//!
//! ## Examples
//!
//! ```text
//! "hello".shout()                    => "HELLO!"
//! 42.shout()                         => "42!"
//! vec![1, 2].debug_tag()               => "<[1, 2]>"
//! "hi".debug_tag()                     => "<\"hi\">"
//! total_area(&[Square { side: 2.0 }])                         => 4.0
//! total_area(&[&Square { side: 1.0 }, &Square { side: 3.0 }]) => 10.0
//! total_area(&[Box::new(Square { side: 1.0 }) as Box<dyn Area>]) => 1.0
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Inside the blanket impl, `self` is `&T`, so `format!("{self}")` uses `T`'s `Display`.
//! 2. The forwarding impls call through: `(**self).area()`.
//! 3. Without `?Sized`, `T` could not be `str` or `dyn Area`, and `Box<dyn Area>` would not be `Area`.

use std::fmt;

/// Loud text for anything displayable.
pub trait Shout {
    fn shout(&self) -> String;
}

/// Debug text in angle brackets for anything debuggable.
pub trait DebugTag {
    fn debug_tag(&self) -> String;
}

/// Something with an area.
pub trait Area {
    fn area(&self) -> f64;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub side: f64,
}

impl<T: fmt::Display + ?Sized> Shout for T {
    fn shout(&self) -> String {
        todo!("Write your solution here")
    }
}

impl<T: fmt::Debug + ?Sized> DebugTag for T {
    fn debug_tag(&self) -> String {
        todo!("Write your solution here")
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        todo!("Write your solution here")
    }
}

impl<T: Area + ?Sized> Area for &T {
    fn area(&self) -> f64 {
        todo!("Write your solution here")
    }
}

impl<T: Area + ?Sized> Area for Box<T> {
    fn area(&self) -> f64 {
        todo!("Write your solution here")
    }
}

/// The sum of all areas, for any `Area` element type.
pub fn total_area<A: Area>(shapes: &[A]) -> f64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shout_for_all_display_types() {
        assert_eq!("hello".shout(), "HELLO!");
        assert_eq!(42.shout(), "42!");
        assert_eq!(String::from("ok").shout(), "OK!");
        let s: &str = "unsized";
        assert_eq!(str::shout(s), "UNSIZED!");
    }

    #[test]
    fn debug_tag_for_all_debug_types() {
        assert_eq!(vec![1, 2].debug_tag(), "<[1, 2]>");
        assert_eq!("hi".debug_tag(), "<\"hi\">");
        assert_eq!(Some('x').debug_tag(), "<Some('x')>");
        assert_eq!(Square { side: 1.0 }.debug_tag(), "<Square { side: 1.0 }>");
        let slice: &[u8] = &[7, 8];
        assert_eq!(slice.debug_tag(), "<[7, 8]>");
    }

    #[test]
    fn forwarding_impls() {
        assert_eq!(total_area(&[Square { side: 2.0 }]), 4.0);
        let a = Square { side: 1.0 };
        let b = Square { side: 3.0 };
        assert_eq!(total_area(&[&a, &b]), 10.0);
        let boxed: Vec<Box<dyn Area>> = vec![Box::new(a), Box::new(b)];
        assert_eq!(total_area(&boxed), 10.0);
        let refs_to_boxes: Vec<&Box<dyn Area>> = boxed.iter().collect();
        assert_eq!(total_area(&refs_to_boxes), 10.0);
    }

    #[test]
    fn user_types_get_the_blanket_methods() {
        struct Celsius(f64);
        impl fmt::Display for Celsius {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} degrees", self.0)
            }
        }
        assert_eq!(Celsius(30.5).shout(), "30.5 DEGREES!");
        struct Circle(f64);
        impl Area for Circle {
            fn area(&self) -> f64 {
                3.0 * self.0 * self.0
            }
        }
        let mixed: Vec<Box<dyn Area>> = vec![Box::new(Circle(1.0)), Box::new(Square { side: 2.0 })];
        assert_eq!(total_area(&mixed), 7.0);
    }
}
