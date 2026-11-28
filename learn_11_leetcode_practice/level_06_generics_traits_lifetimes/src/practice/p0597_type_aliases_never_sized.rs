// ==================================================================
// PRACTICE 0597: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0597_type_aliases_never_sized.md
// Run the tests:          cargo test practice::p0597
// ==================================================================

//! # Problem 0597: Type Aliases, `!` and `?Sized`
//!
//! - **Difficulty:** Medium
//! - **Topics:** type aliases (plain and generic), the never type `!` (`continue`, `panic!`, diverging functions), dynamically sized types and `T: ?Sized`, `size_of_val`
//! - **Source:** Original · [The Rust Book 20.3](https://doc.rust-lang.org/book/ch20-03-advanced-types.html)
//!
//! ## Problem
//!
//! Practise the three topics of the Book's "Advanced Types" section.
//!
//! **Type aliases.** `pub type Kilometers = i32;` is only a synonym (a `Kilometers` *is* an `i32`, unlike
//! a newtype), and `pub type Result<T> = std::result::Result<T, ParseError>;` shortens a common
//! signature. `pub type Handler = Box<dyn Fn(i32) -> i32>;` names a long type.
//!
//! - `parse_distance(text: &str) -> Result<Kilometers>`: trims the text; empty gives
//!   `ParseError::Empty`, a non-number gives `ParseError::NotANumber(trimmed)`, a negative number gives
//!   `ParseError::Negative(value)`.
//! - `total_distance(inputs: &[&str]) -> Kilometers`: the sum of the valid distances. Inside the loop,
//!   write `let d = match parse_distance(s) { Ok(d) => d, Err(_) => continue };`: `continue` has type
//!   `!`, which coerces to `Kilometers`.
//! - `handlers() -> Vec<Handler>`: three handlers: add one, double, negate (in that order).
//! - `apply_all(handlers: &[Handler], x: i32) -> i32`: applies them in order.
//!
//! **The never type.** `fail(message: &str) -> !` panics with `"fatal: {message}"`. A function returning
//! `!` never returns, so `let n: i32 = if ok { 1 } else { fail("x") };` type-checks.
//!
//! **Dynamically sized types.** `describe_size<T: ?Sized + fmt::Debug>(value: &T) -> String` returns
//! `"{value:?} uses {n} bytes"` where `n` is `std::mem::size_of_val(value)`. Because of `?Sized` it
//! accepts `&str`, `&[T]` and `&dyn Debug`, not only sized types.
//!
//! ## Examples
//!
//! ```text
//! parse_distance(" 42 ")                  => Ok(42)
//! parse_distance("-3")                    => Err(Negative(-3))
//! total_distance(&["10", "x", "", "5"])   => 15
//! apply_all(&handlers(), 4)               => -10      (((4 + 1) * 2) negated)
//! describe_size("abc")                    => "\"abc\" uses 3 bytes"
//! describe_size(&[1_u16, 2, 3][..])       => "[1, 2, 3] uses 6 bytes"
//! fail("boom")                            panics with "fatal: boom"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. A generic alias takes parameters like a struct: `type Result<T> = std::result::Result<T, E>;`.
//! 2. `panic!` already has type `!`, so `fail` can be a one-line body.
//! 3. Every generic parameter is `Sized` by default; `T: ?Sized` relaxes that, and the value must then be
//!    used behind a reference (`&T`).

use std::fmt;

/// Distances in kilometers (a synonym for `i32`, not a new type).
pub type Kilometers = i32;

/// A boxed integer transformation.
pub type Handler = Box<dyn Fn(i32) -> i32>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    NotANumber(String),
    Negative(i64),
}

/// `Result` specialised to this module's error type.
pub type Result<T> = std::result::Result<T, ParseError>;

/// Parses a non-negative distance.
pub fn parse_distance(text: &str) -> Result<Kilometers> {
    todo!("Write your solution here")
}

/// The sum of the valid distances.
pub fn total_distance(inputs: &[&str]) -> Kilometers {
    todo!("Write your solution here")
}

/// Add one, double, negate.
pub fn handlers() -> Vec<Handler> {
    todo!("Write your solution here")
}

/// Applies the handlers in order.
pub fn apply_all(handlers: &[Handler], x: i32) -> i32 {
    todo!("Write your solution here")
}

/// Never returns.
pub fn fail(message: &str) -> ! {
    todo!("Write your solution here")
}

/// `"{value:?} uses {n} bytes"`, also for unsized values.
pub fn describe_size<T: ?Sized + fmt::Debug>(value: &T) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_distances() {
        assert_eq!(parse_distance(" 42 "), Ok(42));
        assert_eq!(parse_distance("-3"), Err(ParseError::Negative(-3)));
        assert_eq!(parse_distance("   "), Err(ParseError::Empty));
        assert_eq!(
            parse_distance("ten"),
            Err(ParseError::NotANumber("ten".to_string()))
        );
        let k: Kilometers = 5;
        let plain: i32 = k + 1; // an alias mixes freely with i32
        assert_eq!(plain, 6);
        assert_eq!(total_distance(&["10", "x", "", "5"]), 15);
    }

    #[test]
    fn boxed_handlers() {
        assert_eq!(apply_all(&handlers(), 4), -10);
        assert_eq!(apply_all(&handlers()[..1], 4), 5);
        assert_eq!(apply_all(&[], 9), 9);
    }

    #[test]
    fn sizes_of_unsized_values() {
        assert_eq!(describe_size("abc"), "\"abc\" uses 3 bytes");
        assert_eq!(describe_size(&[1_u16, 2, 3][..]), "[1, 2, 3] uses 6 bytes");
        assert_eq!(describe_size(&7_u64), "7 uses 8 bytes");
        let d: &dyn fmt::Debug = &'z';
        assert_eq!(describe_size(d), "'z' uses 4 bytes");
    }

    #[test]
    fn never_type_in_expressions() {
        let ok = true;
        let n: i32 = if ok { 1 } else { fail("unreachable") };
        assert_eq!(n, 1);
        assert_eq!(total_distance(&["1", "2", "-5", "3"]), 6);
    }

    #[test]
    #[should_panic(expected = "fatal: boom")]
    fn fail_panics() {
        fail("boom");
    }
}
