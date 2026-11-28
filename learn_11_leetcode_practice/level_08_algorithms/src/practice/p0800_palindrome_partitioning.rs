// ==================================================================
// PRACTICE 0800: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0800_palindrome_partitioning.md
// Run the tests:          cargo test practice::p0800
// ==================================================================

//! # Problem 0800: Palindrome Partitioning
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking over cut positions, precomputed palindrome table, `&str` slicing, `Vec<Vec<String>>`
//! - **Source:** [LeetCode 131. Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/)
//!
//! ## Problem
//!
//! Given a string `s` of lowercase English letters, split it into consecutive non-empty parts so
//! that **every part is a palindrome**. Return all possible such partitions, in any order; each
//! partition is a vector of the parts in their original left-to-right order.
//!
//! Implement `Solution::partition(s: String) -> Vec<Vec<String>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "aab"
//! Output: [["a", "a", "b"], ["aa", "b"]]
//!
//! Input:  s = "a"
//! Output: [["a"]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 16`
//! - `s` contains only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Choose the first part: any palindromic prefix. Then partition the rest of the string the same
//!    way (backtracking).
//! 2. The same "is `s[i..=j]` a palindrome?" question is asked many times. Precompute a table:
//!    `s[i..=j]` is a palindrome when `s[i] == s[j]` and `s[i + 1..j]` is a palindrome.

pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> Vec<Vec<String>> {
        let mut result = Solution::partition(s.to_string());
        result.sort(); // any order is accepted
        result
    }

    fn parts(list: &[&[&str]]) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = list
            .iter()
            .map(|p| p.iter().map(|s| s.to_string()).collect())
            .collect();
        result.sort();
        result
    }

    #[test]
    fn example_aab() {
        assert_eq!(run("aab"), parts(&[&["a", "a", "b"], &["aa", "b"]]));
    }

    #[test]
    fn example_single_letter() {
        assert_eq!(run("a"), parts(&[&["a"]]));
    }

    #[test]
    fn odd_palindrome() {
        assert_eq!(run("aba"), parts(&[&["a", "b", "a"], &["aba"]]));
    }

    #[test]
    fn repeated_letter_gives_every_split() {
        // Every one of the 3 gaps of "aaaa" can be cut or not: 2^3 = 8 partitions.
        assert_eq!(run("aaaa").len(), 8);
    }

    #[test]
    fn longest_inputs() {
        assert_eq!(run("abcdefghijklmnop").len(), 1);
        let many = run(&"a".repeat(16));
        assert_eq!(many.len(), 1 << 15);
        assert!(many.iter().all(|p| p.concat() == "a".repeat(16)));
    }
}
