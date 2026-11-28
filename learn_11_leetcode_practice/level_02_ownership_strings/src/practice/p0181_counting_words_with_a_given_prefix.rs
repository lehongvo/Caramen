// ==================================================================
// PRACTICE 0181: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0181_counting_words_with_a_given_prefix.md
// Run the tests:          cargo test practice::p0181
// ==================================================================

//! # Problem 0181: Counting Words With a Given Prefix
//!
//! - **Difficulty:** Easy
//! - **Topics:** `starts_with` with a `&String` pattern, borrowing in `filter`, `count`
//! - **Source:** [LeetCode 2185. Counting Words With a Given Prefix](https://leetcode.com/problems/counting-words-with-a-given-prefix/)
//!
//! ## Problem
//!
//! Count how many strings in `words` start with the string `pref`.
//!
//! Implement `Solution::prefix_count(words: Vec<String>, pref: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  words = ["pay", "attention", "practice", "attend"], pref = "at"
//! Output: 2                ("attention" and "attend")
//!
//! Input:  words = ["leetcode", "win", "loops", "success"], pref = "code"
//! Output: 0                ("leetcode" contains "code" but does not start with it)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= words.len() <= 100`
//! - `1 <= words[i].len(), pref.len() <= 100`
//! - All strings consist of lowercase English letters.
//!
//! ## Hints
//!
//! 1. `word.starts_with(pattern)` accepts a `&str`, a `char`, or a `&String`.
//! 2. `words.iter().filter(..).count()` counts without moving any word.

pub struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(words: &[&str], pref: &str) -> i32 {
        Solution::prefix_count(
            words.iter().map(|w| w.to_string()).collect(),
            pref.to_string(),
        )
    }

    #[test]
    fn example_two_matches() {
        assert_eq!(run(&["pay", "attention", "practice", "attend"], "at"), 2);
    }

    #[test]
    fn example_contains_is_not_enough() {
        assert_eq!(run(&["leetcode", "win", "loops", "success"], "code"), 0);
    }

    #[test]
    fn prefix_equal_to_word_and_longer_than_word() {
        assert_eq!(run(&["at", "a"], "at"), 1);
    }

    #[test]
    fn all_match() {
        assert_eq!(run(&["abc", "abd", "ab"], "ab"), 3);
    }

    #[test]
    fn largest_input() {
        let words: Vec<String> = (0..100)
            .map(|i| format!("{}{}", "p".repeat(i % 3 + 1), "x".repeat(97)))
            .collect();
        assert_eq!(Solution::prefix_count(words, "ppp".to_string()), 33);
    }
}
