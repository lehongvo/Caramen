// ==================================================================
// PRACTICE 0146: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0146_run_length_decode.md
// Run the tests:          cargo test practice::p0146
// ==================================================================

//! # Problem 0146: Run-Length Decode
//!
//! - **Difficulty:** Easy
//! - **Topics:** `chars()`, `char::to_digit`, accumulating a number, `Option::take`, `push` in a loop
//! - **Source:** Original · [`char::to_digit`](https://doc.rust-lang.org/std/primitive.char.html#method.to_digit) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! Run-length encoding writes repeated characters as a count followed by the character. Implement
//! the decoder `run_length_decode(encoded: &str) -> String`.
//!
//! The input is a sequence of groups. Each group is an **optional** count, made of one or more ASCII
//! digits (`0`-`9`, possibly several, such as `12`), followed by exactly one character that is not
//! an ASCII digit. The group stands for that character repeated `count` times; a group without a
//! count stands for the character once. A count of `0` produces nothing.
//!
//! Any non-digit character can be encoded, including spaces and non-ASCII characters.
//!
//! ## Examples
//!
//! ```text
//! Input:  encoded = "3a2bc"
//! Output: "aaabbc"
//!
//! Input:  encoded = "12x"
//! Output: "xxxxxxxxxxxx"
//!
//! Input:  encoded = "a2ü3 0q!"
//! Output: "aüü   !"        (three spaces; "0q" produces nothing)
//!
//! Input:  encoded = ""
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= encoded.len() <= 10^4`
//! - `encoded` is valid: it never ends with a digit, and the decoded text has at most `10^5` chars.
//!
//! ## Hints
//!
//! 1. Iterate with `chars()` (not bytes), so that `'ü'` is one character.
//! 2. `c.to_digit(10)` returns `Some(d)` for `'0'`-`'9'`; build multi-digit counts with
//!    `count = count * 10 + d`.
//! 3. Store the pending count as an `Option<usize>`: `None` means "no count written, use 1".

pub fn run_length_decode(encoded: &str) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_simple_groups() {
        assert_eq!(run_length_decode("3a2bc"), "aaabbc");
    }

    #[test]
    fn example_multi_digit_count() {
        assert_eq!(run_length_decode("12x"), "x".repeat(12));
    }

    #[test]
    fn example_unicode_spaces_and_zero() {
        assert_eq!(run_length_decode("a2ü3 0q!"), "aüü   !");
    }

    #[test]
    fn example_empty() {
        assert_eq!(run_length_decode(""), "");
    }

    #[test]
    fn count_resets_after_each_group() {
        assert_eq!(run_length_decode("2ab"), "aab");
        assert_eq!(run_length_decode("10a1b"), "aaaaaaaaaab");
        assert_eq!(run_length_decode("2🦀"), "🦀🦀");
    }

    #[test]
    fn large_output() {
        let result = run_length_decode("50000a50000b");
        assert_eq!(result.len(), 100_000);
        assert!(result[..50_000].bytes().all(|b| b == b'a'));
        assert!(result[50_000..].bytes().all(|b| b == b'b'));
    }
}
