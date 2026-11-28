// ==================================================================
// PRACTICE 0132: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0132_count_bytes_and_chars.md
// Run the tests:          cargo test practice::p0132
// ==================================================================

//! # Problem 0132: Count Bytes and Characters
//!
//! - **Difficulty:** Easy
//! - **Topics:** `&str`, UTF-8 encoding, `len()` vs `chars().count()`, `char::len_utf8`
//! - **Source:** Original · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html) · [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len)
//!
//! ## Problem
//!
//! Rust strings are UTF-8: ASCII characters take 1 byte, but other characters take 2, 3 or 4 bytes.
//! `str::len()` returns the number of **bytes**, not the number of characters.
//!
//! Implement `byte_and_char_len(text: &str) -> (usize, usize)`, which returns the pair
//! `(number of bytes, number of chars)`, where a "char" is a Unicode scalar value (one Rust `char`).
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "hello"
//! Output: (5, 5)
//!
//! Input:  text = "Zürich"
//! Output: (7, 6)        ('ü' takes 2 bytes)
//!
//! Input:  text = "日本"
//! Output: (6, 2)        (each of these characters takes 3 bytes)
//!
//! Input:  text = "🦀"
//! Output: (4, 1)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `text.len()` is already the byte count, and it is `O(1)`.
//! 2. `text.chars()` decodes the UTF-8 bytes into `char`s; `.count()` walks the whole string.

pub fn byte_and_char_len(text: &str) -> (usize, usize) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ascii() {
        assert_eq!(byte_and_char_len("hello"), (5, 5));
    }

    #[test]
    fn example_two_byte_char() {
        assert_eq!(byte_and_char_len("Zürich"), (7, 6));
    }

    #[test]
    fn example_three_byte_chars() {
        assert_eq!(byte_and_char_len("日本"), (6, 2));
    }

    #[test]
    fn example_four_byte_char() {
        assert_eq!(byte_and_char_len("🦀"), (4, 1));
    }

    #[test]
    fn empty_string() {
        assert_eq!(byte_and_char_len(""), (0, 0));
    }

    #[test]
    fn works_with_a_borrowed_string() {
        let owned = String::from("straße");
        assert_eq!(byte_and_char_len(&owned), (7, 6));
    }

    #[test]
    fn large_mixed_input() {
        let text = "aü日🦀".repeat(10_000);
        assert_eq!(byte_and_char_len(&text), (100_000, 40_000));
    }
}
