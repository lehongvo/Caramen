// ==================================================================
// PRACTICE 0133: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0133_the_first_word.md
// Run the tests:          cargo test practice::p0133
// ==================================================================

//! # Problem 0133: The First Word
//!
//! - **Difficulty:** Easy
//! - **Topics:** string slices, returning `&str` that borrows from the input, `as_bytes`, `enumerate`
//! - **Source:** Original · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! This is the example that motivates slices in The Rust Book. Implement
//! `first_word(text: &str) -> &str`, which returns the part of `text` before the first space
//! character `' '`. If `text` contains no space, the whole `text` is the first word. If `text` starts
//! with a space, the first word is the empty string `""`.
//!
//! Return a **slice of the input**, not a new `String`. Then the result cannot get out of sync with
//! the text: while the returned `&str` is alive, the borrow checker forbids the caller from
//! modifying (for example clearing) the original `String`.
//!
//! Only the space character `' '` separates words here (tabs and newlines are ordinary characters).
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "hello world"
//! Output: "hello"
//!
//! Input:  text = "single"
//! Output: "single"
//!
//! Input:  text = " leading space"
//! Output: ""
//!
//! Input:  text = "naïve idea"
//! Output: "naïve"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `text.as_bytes()` gives a `&[u8]`; `.iter().enumerate()` yields `(index, &byte)` pairs.
//! 2. When you find `b' '` at index `i`, return `&text[..i]`.
//! 3. A space byte is never part of a multi-byte UTF-8 character, so slicing right before it is
//!    always on a character boundary.

pub fn first_word(text: &str) -> &str {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_words() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn example_single_word() {
        assert_eq!(first_word("single"), "single");
    }

    #[test]
    fn example_leading_space() {
        assert_eq!(first_word(" leading space"), "");
    }

    #[test]
    fn example_non_ascii_word() {
        assert_eq!(first_word("naïve idea"), "naïve");
    }

    #[test]
    fn empty_text_and_tabs() {
        assert_eq!(first_word(""), "");
        assert_eq!(first_word("tab\tseparated text"), "tab\tseparated");
    }

    #[test]
    fn result_points_into_the_original_string() {
        let text = String::from("borrow checker");
        let word = first_word(&text);
        assert_eq!(word.as_ptr(), text.as_ptr());
        assert_eq!(word.len(), 6);
    }

    #[test]
    fn large_input() {
        let text = format!("{} end", "a".repeat(100_000));
        assert_eq!(first_word(&text).len(), 100_000);
    }
}
