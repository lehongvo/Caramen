// ==================================================================
// PRACTICE 0827: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0827_palindromic_substrings.md
// Run the tests:          cargo test practice::p0827
// ==================================================================

//! # Problem 0827: Palindromic Substrings
//!
//! - **Difficulty:** Medium
//! - **Topics:** expand around centers (a space-free alternative to the `O(n^2)` DP table), `sum`
//! - **Source:** [LeetCode 647. Palindromic Substrings](https://leetcode.com/problems/palindromic-substrings/)
//!
//! ## Problem
//!
//! Given a string `s`, return how many of its substrings are palindromes (read the same forwards
//! and backwards). Substrings at different positions count separately, even if they are equal
//! strings. Every single character is a palindrome.
//!
//! Implement `Solution::count_substrings(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "abc"
//! Output: 3
//! Explanation: "a", "b", "c".
//!
//! Input:  s = "aaa"
//! Output: 6
//! Explanation: "a" three times, "aa" twice, "aaa" once.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 1000`
//! - `s` contains only lowercase English letters.
//!
//! ## Hints
//!
//! 1. The DP view: `s[i..=j]` is a palindrome if `s[i] == s[j]` and `s[i + 1..j]` is one.
//! 2. Equivalent and lighter: every palindrome has a center, either a character (odd length) or the
//!    gap between two characters (even length). There are `2n - 1` centers.
//! 3. From each center, grow outwards while the two ends match; each step is one more palindrome.

pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(s: &str) -> i32 {
        Solution::count_substrings(s.to_string())
    }

    #[test]
    fn example_distinct_letters() {
        assert_eq!(count("abc"), 3);
    }

    #[test]
    fn example_repeated_letter() {
        assert_eq!(count("aaa"), 6);
    }

    #[test]
    fn single_character() {
        assert_eq!(count("z"), 1);
    }

    #[test]
    fn even_length_palindromes() {
        // a, b, b, a, bb, abba
        assert_eq!(count("abba"), 6);
    }

    #[test]
    fn longest_input_all_equal() {
        // Every one of the 1000 * 1001 / 2 substrings is a palindrome.
        assert_eq!(count(&"a".repeat(1000)), 500_500);
    }
}
