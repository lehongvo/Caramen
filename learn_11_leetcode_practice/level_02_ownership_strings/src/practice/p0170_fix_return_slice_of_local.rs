// ==================================================================
// PRACTICE 0170: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0170_fix_return_slice_of_local.md
// Run the tests:          cargo test practice::p0170
// ==================================================================

//! # Problem 0170: Fix: Returning a Slice of a Local String
//!
//! - **Difficulty:** Easy
//! - **Topics:** fix the borrow checker, error E0515, slices of temporaries, choosing `String` as the return type
//! - **Source:** Original · [E0515](https://doc.rust-lang.org/error_codes/E0515.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references)
//!
//! ## Problem
//!
//! This function should return the first word of a text in upper case, but it does not compile:
//!
//! ```text
//! pub fn shout_first_word(text: &str) -> &str {
//!     let upper = text.to_uppercase();
//!     upper.split_whitespace().next().unwrap_or("")
//! }
//!
//! error[E0515]: cannot return value referencing local variable `upper`
//!   |     upper.split_whitespace().next().unwrap_or("")
//!   |     -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//!   |     |
//!   |     returns a value referencing data owned by the current function
//!   |     `upper` is borrowed here
//! ```
//!
//! `upper` is a new `String` owned by the function and dropped when it returns, so no slice of it
//! can be returned. The upper-case text does not exist in `text` either. The result is new text, and
//! new text must be returned as an owned `String`.
//!
//! Implement the corrected `shout_first_word(text: &str) -> String`: it returns the first
//! whitespace-separated word of `text` converted to upper case (Unicode rules), or an empty string
//! if `text` has no word. Upper-case only that word, not the whole text.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "hello big world"
//! Output: "HELLO"
//!
//! Input:  text = "  straße links"
//! Output: "STRASSE"
//!
//! Input:  text = " \n "
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. Find the first word in `text` itself (`split_whitespace().next()`): that slice borrows from the
//!    caller's data, which outlives the function.
//! 2. Then convert just that word with `to_uppercase()` and return the resulting `String`.

pub fn shout_first_word(text: &str) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ascii() {
        assert_eq!(shout_first_word("hello big world"), "HELLO");
    }

    #[test]
    fn example_unicode_expansion() {
        assert_eq!(shout_first_word("  straße links"), "STRASSE");
    }

    #[test]
    fn example_no_word() {
        assert_eq!(shout_first_word(" \n "), "");
        assert_eq!(shout_first_word(""), "");
    }

    #[test]
    fn result_outlives_the_input() {
        let result = {
            let text = String::from("temporary text");
            shout_first_word(&text)
        };
        assert_eq!(result, "TEMPORARY");
    }

    #[test]
    fn only_the_first_word_is_converted() {
        let text = format!("go {}", "x".repeat(99_000));
        assert_eq!(shout_first_word(&text), "GO");
    }
}
