// ==================================================================
// PRACTICE 0342: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0342_valid_anagram.md
// Run the tests:          cargo test practice::p0342
// ==================================================================

//! # Problem 0342: Valid Anagram
//!
//! - **Difficulty:** Easy
//! - **Topics:** counting with `entry().or_insert(0)`, comparing two `HashMap`s with `==`
//! - **Source:** [LeetCode 242. Valid Anagram](https://leetcode.com/problems/valid-anagram/)
//!
//! ## Problem
//!
//! Two strings are **anagrams** when one can be obtained by rearranging the characters of the
//! other, using every character exactly as many times as it appears (for example `"listen"` and
//! `"silent"`). Given strings `s` and `t`, return `true` if `t` is an anagram of `s`.
//!
//! LeetCode's follow-up asks what changes if the inputs may contain any Unicode characters; aim for
//! a solution that already handles them.
//!
//! Implement `Solution::is_anagram(s: String, t: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "anagram", t = "nagaram"
//! Output: true
//!
//! Input:  s = "rat", t = "car"
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len(), t.len() <= 5 * 10^4`
//! - On LeetCode the strings contain lowercase English letters; the tests here also use other
//!   characters.
//!
//! ## Hints
//!
//! 1. Two strings are anagrams exactly when every character occurs the same number of times in
//!    both.
//! 2. Build a `HashMap<char, usize>` of counts for each string and compare the two maps.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn different_lengths_and_counts() {
        assert!(!Solution::is_anagram("ab".to_string(), "a".to_string()));
        assert!(!Solution::is_anagram("aab".to_string(), "abb".to_string()));
        assert!(Solution::is_anagram("a".to_string(), "a".to_string()));
    }

    #[test]
    fn unicode_follow_up() {
        // Cyrillic: "мир" (peace) and "рим" (Rome) are anagrams; every letter is 2 bytes in UTF-8.
        assert!(Solution::is_anagram("мир".to_string(), "рим".to_string()));
        assert!(!Solution::is_anagram("мир".to_string(), "мор".to_string()));
        // Same byte length (6), different characters.
        assert!(!Solution::is_anagram(
            "мир".to_string(),
            "abcdef".to_string()
        ));
    }

    #[test]
    fn large_strings() {
        let s = "abcde".repeat(10_000);
        let t: String = s.chars().rev().collect();
        assert!(Solution::is_anagram(s.clone(), t));
        let mut u = s.clone();
        u.pop();
        u.push('z');
        assert!(!Solution::is_anagram(s, u));
    }
}
