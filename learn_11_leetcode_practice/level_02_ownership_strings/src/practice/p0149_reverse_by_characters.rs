// ==================================================================
// PRACTICE 0149: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0149_reverse_by_characters.md
// Run the tests:          cargo test practice::p0149
// ==================================================================

//! # Problem 0149: Reverse by Characters
//!
//! - **Difficulty:** Easy
//! - **Topics:** `chars().rev()`, `collect::<String>()`, `Iterator::eq`, why reversing bytes breaks UTF-8
//! - **Source:** Original · [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! Implement two functions that treat a string as a sequence of `char`s:
//!
//! 1. `reverse_chars(text: &str) -> String` returns the characters of `text` in reverse order.
//! 2. `is_char_palindrome(text: &str) -> bool` returns `true` if `text` reads the same forwards and
//!    backwards, comparing `char` by `char` (case and spaces matter; the empty string is a palindrome).
//!
//! Reversing the **bytes** of a UTF-8 string would scramble every multi-byte character (the result
//! would not even be valid UTF-8), so work with `chars()`.
//!
//! ## Examples
//!
//! ```text
//! Input:  reverse_chars("stressed")
//! Output: "desserts"
//!
//! Input:  reverse_chars("ab日本")
//! Output: "本日ba"
//!
//! Input:  is_char_palindrome("über rebü")
//! Output: true
//!
//! Input:  is_char_palindrome("Abba")
//! Output: false            ('A' and 'a' are different chars)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `chars()` is a double-ended iterator, so `.rev()` walks it from the end.
//! 2. `Iterator::eq` compares two iterators element by element without building any `String`.

pub fn reverse_chars(text: &str) -> String {
    todo!("Write your solution here")
}

pub fn is_char_palindrome(text: &str) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_reverse_ascii() {
        assert_eq!(reverse_chars("stressed"), "desserts");
    }

    #[test]
    fn example_reverse_multibyte() {
        assert_eq!(reverse_chars("ab日本"), "本日ba");
    }

    #[test]
    fn example_unicode_palindrome() {
        assert!(is_char_palindrome("über rebü"));
    }

    #[test]
    fn example_case_matters() {
        assert!(!is_char_palindrome("Abba"));
    }

    #[test]
    fn empty_and_single_char() {
        assert_eq!(reverse_chars(""), "");
        assert_eq!(reverse_chars("🦀"), "🦀");
        assert!(is_char_palindrome(""));
        assert!(is_char_palindrome("ß"));
    }

    #[test]
    fn reversed_text_is_valid_and_same_length() {
        let text = "naïve 🦀 straße";
        let reversed = reverse_chars(text);
        assert_eq!(reversed.len(), text.len());
        assert_eq!(reverse_chars(&reversed), text);
    }

    #[test]
    fn large_input() {
        let half = "aü日🦀".repeat(5_000);
        let text = format!("{half}{}", reverse_chars(&half));
        assert!(is_char_palindrome(&text));
        assert!(!is_char_palindrome(&format!("{text}x")));
    }
}
