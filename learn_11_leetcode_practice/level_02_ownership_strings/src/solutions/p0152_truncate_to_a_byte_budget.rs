//! # Problem 0152: Truncate to a Byte Budget
//!
//! - **Difficulty:** Easy
//! - **Topics:** `str::is_char_boundary`, returning a `&str` prefix, UTF-8 byte lengths
//! - **Source:** Original · [`str::is_char_boundary`](https://doc.rust-lang.org/std/primitive.str.html#method.is_char_boundary) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html#slicing-strings)
//!
//! ## Problem
//!
//! A database column stores at most `max_bytes` bytes of UTF-8 text. Implement
//! `truncate_to_bytes(text: &str, max_bytes: usize) -> &str`, which returns the **longest prefix**
//! of `text` that:
//!
//! - is at most `max_bytes` bytes long, and
//! - does not cut a character in half (it ends on a character boundary).
//!
//! If `text` already fits, return it unchanged. Return a slice of `text`; do not allocate.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "hello", max_bytes = 3
//! Output: "hel"
//!
//! Input:  text = "Zürich", max_bytes = 2
//! Output: "Z"           ('ü' occupies bytes 1 and 2, so "Zü" needs 3 bytes)
//!
//! Input:  text = "日本語", max_bytes = 7
//! Output: "日本"         (6 bytes; the third character would need bytes 6, 7 and 8)
//!
//! Input:  text = "🦀", max_bytes = 3
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//! - `0 <= max_bytes <= 10^6`
//!
//! ## Hints
//!
//! 1. `&text[..max_bytes]` panics if `max_bytes` is inside a character.
//! 2. `text.is_char_boundary(i)` tells you whether byte index `i` may be used to slice; index `0`
//!    is always a boundary.
//! 3. Starting at `max_bytes`, step backwards until you reach a boundary; a UTF-8 character is at
//!    most 4 bytes, so this takes at most 3 steps.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** if the text fits, return it. Otherwise start at `max_bytes` (which is then a valid
//! index, smaller than `text.len()`) and move left until the index is a character boundary; the
//! slice up to that point is the answer.
//!
//! **Complexity:** `O(1)` time (at most 4 boundary checks), `O(1)` extra space.
//!
//! **Rust concepts:**
//! - In UTF-8, continuation bytes have the bit pattern `10xxxxxx`; `is_char_boundary` checks that the
//!   byte at the index is not one of them (or that the index equals `len()`).
//! - The loop always terminates because `0` is a boundary, so `end -= 1` never underflows.
//! - Nightly Rust has `str::floor_char_boundary` for this job, but it is not stable in Rust 1.87.
// @solution-end

pub fn truncate_to_bytes(text: &str, max_bytes: usize) -> &str {
    // @solution-start
    if text.len() <= max_bytes {
        return text;
    }
    let mut end = max_bytes;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ascii() {
        assert_eq!(truncate_to_bytes("hello", 3), "hel");
    }

    #[test]
    fn example_inside_two_byte_char() {
        assert_eq!(truncate_to_bytes("Zürich", 2), "Z");
        assert_eq!(truncate_to_bytes("Zürich", 3), "Zü");
    }

    #[test]
    fn example_inside_three_byte_char() {
        assert_eq!(truncate_to_bytes("日本語", 7), "日本");
        assert_eq!(truncate_to_bytes("日本語", 8), "日本");
        assert_eq!(truncate_to_bytes("日本語", 9), "日本語");
    }

    #[test]
    fn example_budget_smaller_than_one_char() {
        assert_eq!(truncate_to_bytes("🦀", 3), "");
    }

    #[test]
    fn fits_or_zero_budget() {
        assert_eq!(truncate_to_bytes("abc", 1_000_000), "abc");
        assert_eq!(truncate_to_bytes("abc", 0), "");
        assert_eq!(truncate_to_bytes("", 5), "");
    }

    #[test]
    fn every_budget_gives_a_valid_prefix() {
        let text = "a🦀ü日b";
        for budget in 0..=text.len() {
            let prefix = truncate_to_bytes(text, budget);
            assert!(prefix.len() <= budget);
            assert!(text.starts_with(prefix));
            // Adding the next character would exceed the budget.
            if let Some(next) = text[prefix.len()..].chars().next() {
                assert!(prefix.len() + next.len_utf8() > budget);
            }
        }
    }

    #[test]
    fn large_input() {
        let text = "🦀".repeat(25_000);
        assert_eq!(truncate_to_bytes(&text, 99_999).len(), 99_996);
    }
}
