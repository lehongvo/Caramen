// ==================================================================
// PRACTICE 0717: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0717_permutation_in_string.md
// Run the tests:          cargo test practice::p0717
// ==================================================================

//! # Problem 0717: Permutation in String
//!
//! - **Difficulty:** Medium
//! - **Topics:** fixed-size sliding window, letter counts `[u32; 26]`, comparing arrays with `==`
//! - **Source:** [LeetCode 567. Permutation in String](https://leetcode.com/problems/permutation-in-string/)
//!
//! ## Problem
//!
//! Given two strings `s1` and `s2` of lowercase English letters, return `true` if `s2` contains a
//! **permutation** of `s1` as a contiguous substring, i.e. some substring of `s2` uses exactly the
//! same letters as `s1`, each the same number of times, in any order. Otherwise return `false`.
//!
//! Implement `Solution::check_inclusion(s1: String, s2: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s1 = "ab", s2 = "eidbaooo"
//! Output: true          ("ba" starts at index 3)
//!
//! Input:  s1 = "ab", s2 = "eidboaoo"
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s1.len(), s2.len() <= 10^4`
//! - Both strings contain only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Two strings are permutations of each other exactly when their letter counts are equal.
//! 2. Slide a window of length `s1.len()` over `s2` and keep its letter counts up to date.

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s1: &str, s2: &str) -> bool {
        Solution::check_inclusion(s1.to_string(), s2.to_string())
    }

    #[test]
    fn example_found() {
        assert!(run("ab", "eidbaooo"));
    }

    #[test]
    fn example_not_found() {
        assert!(!run("ab", "eidboaoo"));
    }

    #[test]
    fn pattern_longer_than_text() {
        assert!(!run("abc", "ab"));
    }

    #[test]
    fn identical_strings() {
        assert!(run("a", "a"));
    }

    #[test]
    fn match_at_the_end() {
        assert!(run("adc", "dcda"));
    }

    #[test]
    fn repeated_letters_must_match_counts() {
        // Windows of "abbab" are "abb", "bba", "bab": none has two 'a's.
        assert!(!run("aab", "abbab"));
        assert!(run("aab", "abbaab"));
    }

    #[test]
    fn long_text_with_match_at_the_end() {
        let text = format!("{}xyz", "a".repeat(9_997));
        assert!(run("zyx", &text));
        assert!(!run("zyy", &text));
    }
}
