// ==================================================================
// PRACTICE 0828: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0828_longest_palindromic_subsequence.md
// Run the tests:          cargo test practice::p0828
// ==================================================================

//! # Problem 0828: Longest Palindromic Subsequence
//!
//! - **Difficulty:** Medium
//! - **Topics:** interval DP (`dp[i][j]` over substrings), filling order, one-row compression
//! - **Source:** [LeetCode 516. Longest Palindromic Subsequence](https://leetcode.com/problems/longest-palindromic-subsequence/)
//!
//! ## Problem
//!
//! Given a string `s`, return the length of its longest **subsequence** that is a palindrome. (A
//! subsequence may skip characters; it does not have to be contiguous.)
//!
//! Implement `Solution::longest_palindrome_subseq(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "bbbab"
//! Output: 4
//! Explanation: "bbbb".
//!
//! Input:  s = "cbbd"
//! Output: 2
//! Explanation: "bb".
//!
//! Input:  s = "character"
//! Output: 5
//! Explanation: "carac".
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 1000`
//! - `s` contains only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Let `P(i, j)` be the answer for `s[i..=j]`. A single character gives `1`.
//! 2. If `s[i] == s[j]`, both ends can wrap a palindrome of `s[i + 1..j]`: `P(i + 1, j - 1) + 2`.
//!    Otherwise drop one end: `max(P(i + 1, j), P(i, j - 1))`.
//! 3. `P(i, ..)` depends on row `i + 1`, so fill `i` from `n - 1` down to `0`.

pub struct Solution;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lps(s: &str) -> i32 {
        Solution::longest_palindrome_subseq(s.to_string())
    }

    #[test]
    fn example_bbbab() {
        assert_eq!(lps("bbbab"), 4);
    }

    #[test]
    fn example_cbbd() {
        assert_eq!(lps("cbbd"), 2);
    }

    #[test]
    fn example_character() {
        assert_eq!(lps("character"), 5);
    }

    #[test]
    fn single_and_distinct_letters() {
        assert_eq!(lps("a"), 1);
        assert_eq!(lps("abcde"), 1);
    }

    #[test]
    fn long_inputs() {
        assert_eq!(lps(&"a".repeat(1000)), 1000);
        // "ab" * 500: drop one end to get an odd palindrome of length 999.
        assert_eq!(lps(&"ab".repeat(500)), 999);
    }
}
