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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** prefix DP. Put the words in a `HashSet<&str>` for `O(1)` lookups, then for every end
//! position `i` look back at most `max_len` characters for a reachable start `j` whose slice
//! `s[j..i]` is in the set.
//!
//! **Complexity:** `O(n * L)` set lookups (each hashing up to `L` bytes), where `L <= 20` is the
//! longest word; `O(n + total dictionary size)` space.
//!
//! **Rust concepts:**
//! - `word_dict.iter().map(String::as_str).collect::<HashSet<&str>>()` builds a set of borrowed
//!   slices: no string is copied, and the set cannot outlive `word_dict`.
//! - Slicing `&s[j..i]` is safe because the input is ASCII (every byte is a char boundary).
//! - `(lo..i).any(|j| ...)` stops at the first successful split.
// @solution-end

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // @solution-start
        let words: HashSet<&str> = word_dict.iter().map(String::as_str).collect();
        let max_len = word_dict.iter().map(String::len).max().unwrap_or(0);
        let n = s.len();
        let mut ok = vec![false; n + 1];
        ok[0] = true;
        for i in 1..=n {
            let lo = i.saturating_sub(max_len);
            ok[i] = (lo..i).any(|j| ok[j] && words.contains(&s[j..i]));
        }
        ok[n]
        // @solution-end
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
