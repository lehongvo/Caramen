//! # Problem 0150: Fix: Indexing a String
//!
//! - **Difficulty:** Easy
//! - **Topics:** fix the compiler error, E0277, strings cannot be indexed by integers, `chars().next()`, `Chars::as_str`
//! - **Source:** Original · [E0277](https://doc.rust-lang.org/error_codes/E0277.html) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings)
//!
//! ## Problem
//!
//! This function should capitalize a word, but it does not compile:
//!
//! ```text
//! pub fn capitalize(word: &str) -> String {
//!     let first = word[0].to_uppercase();
//!     format!("{first}{}", &word[1..])
//! }
//!
//! error[E0277]: the type `str` cannot be indexed by `{integer}`
//!   |     let first = word[0].to_uppercase();
//!   |                      ^ string indices are ranges of `usize`
//! ```
//!
//! A `str` is UTF-8 bytes, and a single byte is not necessarily a character, so Rust refuses
//! `word[0]`. Even `&word[1..]` is a trap: it **panics** if the first character is longer than one
//! byte (for example `"über"`).
//!
//! Implement the corrected `capitalize(word: &str) -> String`: the first character is converted to
//! upper case (full Unicode rules: `'ß'` becomes `"SS"`), every other character is kept exactly as
//! it is. An empty word gives an empty `String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  word = "hello"
//! Output: "Hello"
//!
//! Input:  word = "über"
//! Output: "Über"
//!
//! Input:  word = "ßa"
//! Output: "SSa"
//!
//! Input:  word = "rUST"
//! Output: "RUST"            (only the first character changes)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= word.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. `let mut chars = word.chars();` then `chars.next()` gives the first `char` (or `None`).
//! 2. `char::to_uppercase()` returns an **iterator** of chars (some letters become several).
//! 3. After calling `next()`, `chars.as_str()` is the rest of the word as a `&str` slice.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** take the first `char` from a `Chars` iterator, collect its upper-case form into a new
//! `String`, then append the untouched remainder, which `Chars::as_str` exposes as a slice.
//!
//! **Complexity:** `O(n)` time and space.
//!
//! **Rust concepts:**
//! - Strings are indexed only by byte **ranges** (`&s[a..b]`), and those ranges must fall on
//!   character boundaries or the program panics.
//! - `Chars::as_str()` returns the part of the string the iterator has not consumed yet, sliced at
//!   a correct boundary.
//! - `char::to_uppercase` yields one or more `char`s; collecting it gives a `String`.
// @solution-end

pub fn capitalize(word: &str) -> String {
    // @solution-start
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => {
            let mut out: String = first.to_uppercase().collect();
            out.push_str(chars.as_str());
            out
        }
        None => String::new(),
    }
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ascii() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn example_two_byte_first_char() {
        assert_eq!(capitalize("über"), "Über");
    }

    #[test]
    fn example_first_char_expands() {
        assert_eq!(capitalize("ßa"), "SSa");
    }

    #[test]
    fn example_rest_unchanged() {
        assert_eq!(capitalize("rUST"), "RUST");
    }

    #[test]
    fn empty_and_caseless() {
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("日本"), "日本");
        assert_eq!(capitalize("42abc"), "42abc");
    }

    #[test]
    fn long_word() {
        let word = format!("ñ{}", "a".repeat(9_998));
        let result = capitalize(&word);
        assert!(result.starts_with("Ña"));
        assert_eq!(result.len(), word.len());
    }
}
