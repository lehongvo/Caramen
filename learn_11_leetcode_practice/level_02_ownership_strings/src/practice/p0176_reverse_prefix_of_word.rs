// ==================================================================
// PRACTICE 0176: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0176_reverse_prefix_of_word.md
// Run the tests:          cargo test practice::p0176
// ==================================================================

//! # Problem 0176: Reverse Prefix of Word
//!
//! - **Difficulty:** Easy
//! - **Topics:** `str::find`, `split_at`, `chars().rev().chain(..)`, returning the input unchanged
//! - **Source:** [LeetCode 2000. Reverse Prefix of Word](https://leetcode.com/problems/reverse-prefix-of-word/)
//!
//! ## Problem
//!
//! Given a word and a character `ch`, reverse the part of the word that goes from its start up to
//! and **including** the first occurrence of `ch`. If `ch` does not occur in the word, return the
//! word unchanged.
//!
//! Implement `Solution::reverse_prefix(word: String, ch: char) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  word = "abcdefd", ch = 'd'
//! Output: "dcbaefd"        (the first 'd' is at index 3: reverse "abcd")
//!
//! Input:  word = "xyxzxe", ch = 'z'
//! Output: "zxyxxe"
//!
//! Input:  word = "abcd", ch = 'z'
//! Output: "abcd"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= word.len() <= 250`
//! - `word` consists of lowercase English letters; `ch` is a lowercase English letter.
//!
//! ## Hints
//!
//! 1. `word.find(ch)` returns the byte index of the first `ch`, or `None`.
//! 2. `word.split_at(i + ch.len_utf8())` separates the prefix (including `ch`) from the rest.
//! 3. Build the answer from `prefix.chars().rev()` followed by `rest.chars()`.

pub struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(word: &str, ch: char) -> String {
        Solution::reverse_prefix(word.to_string(), ch)
    }

    #[test]
    fn example_first_d() {
        assert_eq!(run("abcdefd", 'd'), "dcbaefd");
    }

    #[test]
    fn example_z_in_the_middle() {
        assert_eq!(run("xyxzxe", 'z'), "zxyxxe");
    }

    #[test]
    fn example_not_found() {
        assert_eq!(run("abcd", 'z'), "abcd");
    }

    #[test]
    fn first_or_last_character() {
        assert_eq!(run("abc", 'a'), "abc");
        assert_eq!(run("abc", 'c'), "cba");
    }

    #[test]
    fn longest_word() {
        let word = format!("{}z{}", "a".repeat(200), "b".repeat(49));
        let expected = format!("z{}{}", "a".repeat(200), "b".repeat(49));
        assert_eq!(run(&word, 'z'), expected);
    }
}
