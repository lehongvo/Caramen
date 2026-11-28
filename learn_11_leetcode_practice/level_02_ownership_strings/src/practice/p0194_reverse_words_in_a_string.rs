// ==================================================================
// PRACTICE 0194: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0194_reverse_words_in_a_string.md
// Run the tests:          cargo test practice::p0194
// ==================================================================

//! # Problem 0194: Reverse Words in a String
//!
//! - **Difficulty:** Medium
//! - **Topics:** in-place word reversal on the string's own bytes, compacting spaces, `split_whitespace().rev()`
//! - **Source:** [LeetCode 151. Reverse Words in a String](https://leetcode.com/problems/reverse-words-in-a-string/)
//!
//! ## Problem
//!
//! Given a string `s`, return its words in reverse order, separated by exactly one space. A word is a
//! maximal run of non-space characters. `s` may have leading or trailing spaces and several spaces
//! between words; the result must not.
//!
//! Implement `Solution::reverse_words(s: String) -> String`.
//!
//! Follow-up (the interesting part in Rust): do it in place on the string's own buffer, with `O(1)`
//! extra space.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "the sky is blue"
//! Output: "blue is sky the"
//!
//! Input:  s = "  hello world  "
//! Output: "world hello"
//!
//! Input:  s = "a good   example"
//! Output: "example good a"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^4`
//! - `s` contains English letters, digits and spaces `' '`, and at least one word.
//!
//! ## Hints
//!
//! 1. One-liner: `s.split_whitespace().rev().collect::<Vec<_>>().join(" ")`.
//! 2. In place: reverse the whole byte buffer, so the words are in the right order but each word is
//!    backwards; then reverse every word.
//! 3. While doing that, copy the bytes towards the front with a `write` index so that only single
//!    spaces remain, and `truncate` at the end.

pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::reverse_words(s.to_string())
    }

    #[test]
    fn example_sky() {
        assert_eq!(run("the sky is blue"), "blue is sky the");
    }

    #[test]
    fn example_outer_spaces() {
        assert_eq!(run("  hello world  "), "world hello");
    }

    #[test]
    fn example_inner_spaces() {
        assert_eq!(run("a good   example"), "example good a");
    }

    #[test]
    fn single_word() {
        assert_eq!(run("   rust   "), "rust");
        assert_eq!(run("x"), "x");
    }

    #[test]
    fn matches_split_whitespace_version() {
        let s = "  12 apples  and 7   Oranges ";
        let expected = s.split_whitespace().rev().collect::<Vec<_>>().join(" ");
        assert_eq!(run(s), expected);
    }

    #[test]
    fn large_input() {
        let words: Vec<String> = (0..2_000).map(|i| i.to_string()).collect();
        let s = format!("  {}  ", words.join("   "));
        let expected: Vec<&str> = words.iter().rev().map(|w| w.as_str()).collect();
        assert_eq!(run(&s), expected.join(" "));
    }
}
