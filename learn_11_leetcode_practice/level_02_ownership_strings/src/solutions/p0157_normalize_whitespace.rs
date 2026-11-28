//! # Problem 0157: Normalize Whitespace
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split_whitespace`, `Vec<&str>` borrowing from the input, `join`
//! - **Source:** Original · [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! Text pasted from different sources often contains runs of spaces, tabs and newlines. Implement
//! `normalize_whitespace(text: &str) -> String`, which returns the words of `text` separated by
//! exactly **one** ASCII space `' '`, with no whitespace at the beginning or the end.
//!
//! A *word* is a maximal run of non-whitespace characters. "Whitespace" uses the Unicode
//! definition (`char::is_whitespace`), so tabs, newlines and characters such as the ideographic
//! space `'\u{3000}'` all separate words. If `text` has no words, return an empty string.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "  the   quick\tbrown \n fox  "
//! Output: "the quick brown fox"
//!
//! Input:  text = "single"
//! Output: "single"
//!
//! Input:  text = " \t\n "
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `text.split_whitespace()` yields the words as `&str` slices and skips every kind and amount of
//!    whitespace.
//! 2. Collect them into a `Vec<&str>` and call `.join(" ")`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** let `split_whitespace` find the words, then join them with single spaces. The
//! intermediate `Vec<&str>` only stores slices pointing into `text`; the text itself is copied once,
//! into the final `String`.
//!
//! **Complexity:** `O(n)` time and space.
//!
//! **Rust concepts:**
//! - `split_whitespace` never yields empty pieces, unlike `split(' ')`, which yields `""` between two
//!   consecutive spaces.
//! - A `Vec<&str>` borrows from `text`; it must not outlive it, which the compiler checks.
//! - `[&str]::join(sep)` computes the final length first and allocates exactly once.
// @solution-end

pub fn normalize_whitespace(text: &str) -> String {
    // @solution-start
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed_whitespace() {
        assert_eq!(
            normalize_whitespace("  the   quick\tbrown \n fox  "),
            "the quick brown fox"
        );
    }

    #[test]
    fn example_single_word() {
        assert_eq!(normalize_whitespace("single"), "single");
    }

    #[test]
    fn example_only_whitespace() {
        assert_eq!(normalize_whitespace(" \t\n "), "");
        assert_eq!(normalize_whitespace(""), "");
    }

    #[test]
    fn unicode_whitespace_and_words() {
        assert_eq!(normalize_whitespace("日本\u{3000}語  über"), "日本 語 über");
    }

    #[test]
    fn already_normalized_is_unchanged() {
        assert_eq!(normalize_whitespace("a b c"), "a b c");
    }

    #[test]
    fn large_input() {
        let text = "word \t\n  ".repeat(10_000);
        let result = normalize_whitespace(&text);
        assert_eq!(result.len(), 10_000 * 5 - 1);
        assert!(!result.contains("  "));
    }
}
