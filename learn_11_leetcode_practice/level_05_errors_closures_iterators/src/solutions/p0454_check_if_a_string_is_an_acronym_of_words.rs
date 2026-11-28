//! # Problem 0454: Check if a String Is an Acronym of Words
//!
//! - **Difficulty:** Easy
//! - **Topics:** `filter_map` with `chars().next()`, `Iterator::eq`, comparing an iterator with a string's characters
//! - **Source:** [LeetCode 2828. Check if a String Is an Acronym of Words](https://leetcode.com/problems/check-if-a-string-is-an-acronym-of-words/)
//!
//! ## Problem
//!
//! The *acronym* of a list of words is the string made of the first character of each word, in
//! order: the acronym of `["apple", "banana"]` is `"ab"`.
//!
//! Given `words` and a string `s`, return `true` if `s` is exactly the acronym of `words`.
//!
//! Implement `Solution::is_acronym(words: Vec<String>, s: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  words = ["alice", "bob", "charlie"], s = "abc"
//! Output: true
//!
//! Input:  words = ["an", "apple"], s = "a"
//! Output: false
//! Explanation: the acronym is "aa", which is longer than "a".
//!
//! Input:  words = ["never", "gonna", "give", "up", "on", "you"], s = "ngguoy"
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= words.len() <= 100`
//! - `1 <= words[i].len() <= 10`
//! - `1 <= s.len() <= 100`
//! - `words[i]` and `s` contain lowercase English letters.
//!
//! ## Hints
//!
//! 1. `w.chars().next()` is the first character of `w` as an `Option<char>`.
//! 2. Build the acronym lazily with `filter_map` and compare it with `s.chars()` using `eq`, which
//!    also checks that the lengths match.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the acronym is the iterator of first characters; compare it with the characters of
//! `s`. `Iterator::eq` returns `false` as soon as one character differs or one side runs out.
//!
//! **Complexity:** `O(n)` time for `n` words, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `filter_map(|w| w.chars().next())` keeps the first character of every non-empty word (the
//!   constraints guarantee non-empty words, so nothing is dropped).
//! - Comparing iterators avoids building the acronym `String`.
//! - An equivalent eager version is `words.iter().map(|w| &w[..1]).collect::<String>() == s`, which
//!   allocates and relies on ASCII input for the slicing.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        // @solution-start
        words.iter().filter_map(|w| w.chars().next()).eq(s.chars())
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(words: &[&str], s: &str) -> bool {
        Solution::is_acronym(words.iter().map(|w| w.to_string()).collect(), s.to_string())
    }

    #[test]
    fn example_alice_bob_charlie() {
        assert!(run(&["alice", "bob", "charlie"], "abc"));
    }

    #[test]
    fn example_too_short() {
        assert!(!run(&["an", "apple"], "a"));
    }

    #[test]
    fn example_never_gonna() {
        assert!(run(
            &["never", "gonna", "give", "up", "on", "you"],
            "ngguoy"
        ));
    }

    #[test]
    fn too_long_or_wrong_letter() {
        assert!(!run(&["a"], "ab"));
        assert!(!run(&["a", "b"], "ac"));
        assert!(run(&["z"], "z"));
    }
}
