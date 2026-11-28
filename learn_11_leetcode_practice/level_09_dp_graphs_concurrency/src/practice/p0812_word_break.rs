// ==================================================================
// PRACTICE 0812: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0812_word_break.md
// Run the tests:          cargo test practice::p0812
// ==================================================================

//! # Problem 0812: Word Break
//!
//! - **Difficulty:** Medium
//! - **Topics:** prefix DP over a string, `HashSet<&str>` borrowed from a `Vec<String>`, `any`
//! - **Source:** [LeetCode 139. Word Break](https://leetcode.com/problems/word-break/)
//!
//! ## Problem
//!
//! Given a string `s` and a dictionary `word_dict` of distinct words, return `true` if `s` can be
//! cut into a sequence of one or more dictionary words (each word may be used any number of times),
//! and `false` otherwise.
//!
//! Implement `Solution::word_break(s: String, word_dict: Vec<String>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "leetcode", word_dict = ["leet", "code"]
//! Output: true
//!
//! Input:  s = "applepenapple", word_dict = ["apple", "pen"]
//! Output: true
//! Explanation: "apple" is used twice.
//!
//! Input:  s = "catsandog", word_dict = ["cats", "dog", "sand", "and", "cat"]
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 300`
//! - `1 <= word_dict.len() <= 1000`, `1 <= word_dict[i].len() <= 20`
//! - `s` and the words contain only lowercase English letters; the words are distinct.
//!
//! ## Hints
//!
//! 1. Let `ok[i]` mean "the first `i` characters can be segmented". `ok[0]` is `true`.
//! 2. `ok[i]` is true if some `j < i` has `ok[j]` and `s[j..i]` is a word.
//! 3. Words are at most 20 letters long, so only `j >= i - 20` needs to be checked.

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, dict: &[&str]) -> bool {
        Solution::word_break(s.to_string(), dict.iter().map(|w| w.to_string()).collect())
    }

    #[test]
    fn example_leetcode() {
        assert!(run("leetcode", &["leet", "code"]));
    }

    #[test]
    fn example_reused_word() {
        assert!(run("applepenapple", &["apple", "pen"]));
    }

    #[test]
    fn example_impossible() {
        assert!(!run("catsandog", &["cats", "dog", "sand", "and", "cat"]));
    }

    #[test]
    fn greedy_longest_prefix_would_fail() {
        // Taking "car" first leaves "s"; the right split is "ca" + "rs".
        assert!(run("cars", &["car", "ca", "rs"]));
    }

    #[test]
    fn long_string_that_cannot_be_split() {
        let s = format!("{}b", "a".repeat(299));
        let dict = ["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa"];
        assert!(!run(&s, &dict));
        assert!(run(&"a".repeat(300), &dict));
    }
}
