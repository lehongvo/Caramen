// ==================================================================
// PRACTICE 0140: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0140_repeat_with_a_separator.md
// Run the tests:          cargo test practice::p0140
// ==================================================================

//! # Problem 0140: Repeat With a Separator
//!
//! - **Difficulty:** Easy
//! - **Topics:** `String::with_capacity`, `push_str`, `join`, `str::repeat`
//! - **Source:** Original · [`String::push_str`](https://doc.rust-lang.org/std/string/struct.String.html#method.push_str) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! Implement `repeat_with_separator(word: &str, times: usize, separator: &str) -> String`, which
//! returns `word` repeated `times` times with `separator` **between** consecutive copies (not before
//! the first one and not after the last one).
//!
//! If `times` is `0`, return an empty string. If `times` is `1`, return just `word`.
//!
//! ## Examples
//!
//! ```text
//! Input:  word = "ab", times = 3, separator = "-"
//! Output: "ab-ab-ab"
//!
//! Input:  word = "go", times = 2, separator = ", "
//! Output: "go, go"
//!
//! Input:  word = "x", times = 0, separator = "+"
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= times <= 10^4`
//! - `0 <= word.len(), separator.len() <= 100`
//!
//! ## Hints
//!
//! 1. Push the separator before every copy **except the first** one.
//! 2. The final length is known in advance: `word.len() * times + separator.len() * (times - 1)`
//!    (when `times > 0`), so `String::with_capacity` avoids reallocations.
//! 3. Shorter alternative: `vec![word; times].join(separator)`.

pub fn repeat_with_separator(word: &str, times: usize, separator: &str) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_dashes() {
        assert_eq!(repeat_with_separator("ab", 3, "-"), "ab-ab-ab");
    }

    #[test]
    fn example_comma_space() {
        assert_eq!(repeat_with_separator("go", 2, ", "), "go, go");
    }

    #[test]
    fn example_zero_times() {
        assert_eq!(repeat_with_separator("x", 0, "+"), "");
    }

    #[test]
    fn one_time_has_no_separator() {
        assert_eq!(repeat_with_separator("solo", 1, "|"), "solo");
    }

    #[test]
    fn empty_word_or_separator() {
        assert_eq!(repeat_with_separator("", 3, "-"), "--");
        assert_eq!(repeat_with_separator("ha", 3, ""), "hahaha");
    }

    #[test]
    fn large_input_matches_join() {
        let result = repeat_with_separator("word", 10_000, " / ");
        assert_eq!(result, vec!["word"; 10_000].join(" / "));
    }
}
