// ==================================================================
// PRACTICE 0104: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0104_empty_the_buffer.md
// Run the tests:          cargo test practice::p0104
// ==================================================================

//! # Problem 0104: Empty the Buffer
//!
//! - **Difficulty:** Easy
//! - **Topics:** `&mut String`, `std::mem::take`, moving out of a mutable reference, `Option`
//! - **Source:** Original · [`std::mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
//!
//! ## Problem
//!
//! A logger collects text in a `String` buffer that it owns. From time to time it calls a helper
//! that receives the buffer through a **mutable reference** and decides whether to flush it.
//!
//! Implement `flush_if_full(buffer: &mut String, limit: usize) -> Option<String>`:
//!
//! - if `buffer.len() >= limit`, return `Some(content)` where `content` is everything the buffer
//!   held, and leave the buffer **empty** (so the logger can keep writing into it);
//! - otherwise return `None` and leave the buffer untouched.
//!
//! You only have `&mut String`, not the `String` itself, so `return Some(*buffer)` does not
//! compile: you cannot move out of a reference without putting something back. Do not clone the
//! content either: hand over the existing text.
//!
//! ## Examples
//!
//! ```text
//! Input:  buffer = "abcdef", limit = 4
//! Output: Some("abcdef"), buffer is now ""
//!
//! Input:  buffer = "abc", limit = 4
//! Output: None, buffer is still "abc"
//!
//! Input:  buffer = "", limit = 0
//! Output: Some(""), buffer is still ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= buffer.len() <= 10^5`
//! - `0 <= limit <= 10^5`
//!
//! ## Hints
//!
//! 1. `std::mem::take(buffer)` moves the current `String` out and leaves `String::new()` (the
//!    `Default` value) in its place.
//! 2. `std::mem::replace(buffer, String::new())` does the same with an explicit replacement.

pub fn flush_if_full(buffer: &mut String, limit: usize) -> Option<String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_full_buffer_is_flushed() {
        let mut buffer = String::from("abcdef");
        assert_eq!(flush_if_full(&mut buffer, 4), Some(String::from("abcdef")));
        assert_eq!(buffer, "");
    }

    #[test]
    fn example_short_buffer_is_kept() {
        let mut buffer = String::from("abc");
        assert_eq!(flush_if_full(&mut buffer, 4), None);
        assert_eq!(buffer, "abc");
    }

    #[test]
    fn example_empty_buffer_with_zero_limit() {
        let mut buffer = String::new();
        assert_eq!(flush_if_full(&mut buffer, 0), Some(String::new()));
        assert_eq!(buffer, "");
    }

    #[test]
    fn exact_limit_flushes() {
        let mut buffer = String::from("1234");
        assert_eq!(flush_if_full(&mut buffer, 4), Some(String::from("1234")));
        assert!(buffer.is_empty());
    }

    #[test]
    fn buffer_can_be_reused_after_flush() {
        let mut buffer = String::new();
        let mut flushed = Vec::new();
        for word in ["ab", "cd", "ef", "g"] {
            buffer.push_str(word);
            if let Some(chunk) = flush_if_full(&mut buffer, 4) {
                flushed.push(chunk);
            }
        }
        assert_eq!(flushed, vec!["abcd"]);
        assert_eq!(buffer, "efg");
    }

    #[test]
    fn large_buffer() {
        let mut buffer = "x".repeat(100_000);
        let chunk = flush_if_full(&mut buffer, 100_000).expect("buffer is full");
        assert_eq!(chunk.len(), 100_000);
        assert!(buffer.is_empty());
    }
}
