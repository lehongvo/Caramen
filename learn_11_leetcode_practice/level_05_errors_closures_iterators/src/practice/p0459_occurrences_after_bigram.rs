// ==================================================================
// PRACTICE 0459: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0459_occurrences_after_bigram.md
// Run the tests:          cargo test practice::p0459
// ==================================================================

//! # Problem 0459: Occurrences After Bigram
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split` into `Vec<&str>`, `windows(3)` over words, `filter` + `map`, borrowing vs owning
//! - **Source:** [LeetCode 1078. Occurrences After Bigram](https://leetcode.com/problems/occurrences-after-bigram/)
//!
//! ## Problem
//!
//! `text` is a list of words separated by single spaces. Whenever the word `first` is immediately
//! followed by the word `second`, the word right after them (if any) is called a *third*. Return all
//! thirds, in the order they appear. Occurrences may overlap.
//!
//! Implement `Solution::find_ocurrences(text: String, first: String, second: String) -> Vec<String>`
//! (the name is spelled this way in LeetCode's template).
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "alice is a good girl she is a good student", first = "a", second = "good"
//! Output: ["girl", "student"]
//!
//! Input:  text = "we will we will rock you", first = "we", second = "will"
//! Output: ["we", "rock"]
//! Explanation: the occurrences "we will we" and "we will rock" overlap.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= text.len() <= 1000`
//! - `text` contains lowercase English letters and single spaces, without leading or trailing spaces.
//! - `1 <= first.len(), second.len() <= 10`, lowercase English letters.
//!
//! ## Hints
//!
//! 1. Collect the words into a `Vec<&str>` so you can use `windows(3)` on it.
//! 2. Keep the windows `[a, b, c]` with `a == first` and `b == second`, and output `c`.

pub struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(text: &str, first: &str, second: &str) -> Vec<String> {
        Solution::find_ocurrences(text.to_string(), first.to_string(), second.to_string())
    }

    #[test]
    fn example_alice() {
        assert_eq!(
            run("alice is a good girl she is a good student", "a", "good"),
            vec!["girl", "student"]
        );
    }

    #[test]
    fn example_overlapping() {
        assert_eq!(
            run("we will we will rock you", "we", "will"),
            vec!["we", "rock"]
        );
    }

    #[test]
    fn bigram_at_the_end_has_no_third() {
        assert_eq!(run("a b", "a", "b"), Vec::<String>::new());
        assert_eq!(run("x a b", "a", "b"), Vec::<String>::new());
        assert_eq!(run("a", "a", "a"), Vec::<String>::new());
    }

    #[test]
    fn repeated_word() {
        assert_eq!(run("a a a a", "a", "a"), vec!["a", "a"]);
        assert_eq!(run("ab abc ab abc", "ab", "abc"), vec!["ab"]);
    }
}
