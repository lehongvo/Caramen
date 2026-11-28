//! # Problem 0151: Take the First N Characters
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning `&str` from the input, `char_indices().nth`, slicing on a character boundary
//! - **Source:** Original · [`str::char_indices`](https://doc.rust-lang.org/std/primitive.str.html#method.char_indices) · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! A preview widget shows only the first `n` characters of a message. Implement
//! `take_chars(text: &str, n: usize) -> &str`, which returns the prefix of `text` made of its first
//! `n` characters (`char`s). If `text` has `n` characters or fewer, return all of `text`.
//!
//! The result must be a slice of `text` (no allocation). Note that `&text[..n]` is wrong: it takes
//! `n` **bytes**, and it panics when byte `n` falls inside a multi-byte character.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "hello world", n = 5
//! Output: "hello"
//!
//! Input:  text = "Zürich", n = 2
//! Output: "Zü"             (3 bytes)
//!
//! Input:  text = "日本語", n = 10
//! Output: "日本語"
//!
//! Input:  text = "abc", n = 0
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//! - `0 <= n <= 10^5`
//!
//! ## Hints
//!
//! 1. The prefix of `n` characters ends where character number `n` (counting from 0) **starts**.
//! 2. `text.char_indices().nth(n)` returns `Some((byte_offset, char))` for that character, or `None`
//!    if the text is shorter.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** find the byte offset at which the `(n + 1)`-th character begins; the answer is
//! everything before it. If there is no such character, the whole text fits.
//!
//! **Complexity:** `O(n)` time (it walks at most `n + 1` characters), `O(1)` extra space.
//!
//! **Rust concepts:**
//! - Byte offsets produced by `char_indices` are always character boundaries, so slicing with them
//!   never panics.
//! - `Iterator::nth(n)` skips `n` items and returns the next one.
//! - Returning `&str` tied to `text` (lifetime elision) avoids allocating a new `String`.
// @solution-end

pub fn take_chars(text: &str, n: usize) -> &str {
    // @solution-start
    match text.char_indices().nth(n) {
        Some((end, _)) => &text[..end],
        None => text,
    }
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ascii_prefix() {
        assert_eq!(take_chars("hello world", 5), "hello");
    }

    #[test]
    fn example_multibyte_prefix() {
        let prefix = take_chars("Zürich", 2);
        assert_eq!(prefix, "Zü");
        assert_eq!(prefix.len(), 3);
    }

    #[test]
    fn example_n_larger_than_text() {
        assert_eq!(take_chars("日本語", 10), "日本語");
    }

    #[test]
    fn example_zero_chars() {
        assert_eq!(take_chars("abc", 0), "");
    }

    #[test]
    fn exact_length_and_empty_text() {
        assert_eq!(take_chars("🦀🦀", 2), "🦀🦀");
        assert_eq!(take_chars("", 3), "");
    }

    #[test]
    fn result_borrows_from_the_input() {
        let text = String::from("straße");
        let prefix = take_chars(&text, 5);
        assert_eq!(prefix, "straß");
        assert_eq!(prefix.as_ptr(), text.as_ptr());
    }

    #[test]
    fn large_input() {
        let text = "日".repeat(30_000);
        let prefix = take_chars(&text, 25_000);
        assert_eq!(prefix.len(), 75_000);
        assert_eq!(prefix.chars().count(), 25_000);
    }
}
