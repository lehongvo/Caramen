// ==================================================================
// PRACTICE 0139: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0139_fix_adding_two_strings.md
// Run the tests:          cargo test practice::p0139
// ==================================================================

//! # Problem 0139: Fix: Adding Two Strings
//!
//! - **Difficulty:** Easy
//! - **Topics:** fix the compiler error, E0308, the `+` operator on `String` (`String + &str`), `format!`
//! - **Source:** Original · [E0308](https://doc.rust-lang.org/error_codes/E0308.html) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! This function should join three words with single spaces, but it does not compile:
//!
//! ```text
//! pub fn join_words(first: String, second: String, third: String) -> String {
//!     first + " " + second + " " + third
//! }
//!
//! error[E0308]: mismatched types
//!   |     first + " " + second + " " + third
//!   |                   ^^^^^^ expected `&str`, found `String`
//!   help: consider borrowing here: `&second`
//! ```
//!
//! The `+` operator on strings is defined as `String + &str -> String`: it **takes ownership** of the
//! left `String`, appends the borrowed right-hand side to it and returns it. The right-hand side must
//! be a `&str`, never a `String`.
//!
//! Implement the corrected `join_words(first: String, second: String, third: String) -> String`,
//! which returns `first`, `second` and `third` separated by one space each (`"a b c"`). Keep using
//! `+` (and keep the signature).
//!
//! ## Examples
//!
//! ```text
//! Input:  first = "tic", second = "tac", third = "toe"
//! Output: "tic tac toe"
//!
//! Input:  first = "", second = "x", third = ""
//! Output: " x "
//! ```
//!
//! ## Constraints
//!
//! - `0 <= first.len(), second.len(), third.len() <= 1000`
//!
//! ## Hints
//!
//! 1. Borrow the right-hand operands: `&second` is a `&String`, which coerces to `&str`.
//! 2. `first` is moved into the expression and its buffer is reused for the result.

pub fn join_words(first: String, second: String, third: String) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(text: &str) -> String {
        text.to_string()
    }

    #[test]
    fn example_tic_tac_toe() {
        assert_eq!(join_words(s("tic"), s("tac"), s("toe")), "tic tac toe");
    }

    #[test]
    fn example_empty_parts_keep_the_spaces() {
        assert_eq!(join_words(s(""), s("x"), s("")), " x ");
    }

    #[test]
    fn unicode_words() {
        assert_eq!(
            join_words(s("über"), s("straße"), s("日本")),
            "über straße 日本"
        );
    }

    #[test]
    fn long_words() {
        let result = join_words("a".repeat(1000), "b".repeat(1000), "c".repeat(1000));
        assert_eq!(result.len(), 3002);
        assert_eq!(&result[999..1002], "a b");
    }
}
