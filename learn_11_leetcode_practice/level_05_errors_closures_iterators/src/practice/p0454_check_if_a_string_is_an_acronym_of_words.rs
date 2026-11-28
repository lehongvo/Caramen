// ==================================================================
// PRACTICE 0454: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0454_check_if_a_string_is_an_acronym_of_words.md
// Run the tests:          cargo test practice::p0454
// ==================================================================

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

pub struct Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        todo!("Write your solution here")
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
