// ==================================================================
// PRACTICE 0990: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0990_wildcard_matching.md
// Run the tests:          cargo test practice::p0990
// ==================================================================

//! # Problem 0990: Wildcard Matching
//!
//! - **Difficulty:** Hard
//! - **Topics:** DP with a rolling 1D row, `'?'` and `'*'` semantics, `std::mem::swap` of rows, large inputs
//! - **Source:** [LeetCode 44. Wildcard Matching](https://leetcode.com/problems/wildcard-matching/)
//!
//! ## Problem
//!
//! Match the whole string `s` against a wildcard pattern `p` where:
//!
//! - `'?'` matches any single character;
//! - `'*'` matches any sequence of characters, including the empty one.
//!
//! Unlike problem 0989, `'*'` is a stand-alone wildcard here, not a modifier of the previous element.
//! Implement `Solution::is_match(s: String, p: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! s = "aa",  p = "a"     => false
//! s = "aa",  p = "*"     => true
//! s = "cb",  p = "?a"    => false
//! s = "adceb", p = "*a*b" => true
//! ```
//!
//! ## Constraints
//!
//! - `0 <= s.len(), p.len() <= 2000`
//! - `s` has lowercase letters; `p` has lowercase letters, `'?'` and `'*'`.
//!
//! ## Hints
//!
//! 1. Let `dp[i][j]` mean "`s[..i]` matches `p[..j]`"; `dp[0][0]` is true, and `dp[0][j]` is true only
//!    while `p[..j]` is all stars.
//! 2. A `'*'` at `p[j - 1]` gives `dp[i][j] = dp[i][j - 1]` (matches nothing) `|| dp[i - 1][j]`
//!    (matches one more character). A letter or `'?'` needs `dp[i - 1][j - 1]` and a matching char.
//! 3. Row `i` only depends on row `i - 1`: keep two rows of length `m + 1`.

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn m(s: &str, p: &str) -> bool {
        Solution::is_match(s.to_string(), p.to_string())
    }

    fn naive(s: &[u8], p: &[u8]) -> bool {
        match p.first() {
            None => s.is_empty(),
            Some(b'*') => naive(s, &p[1..]) || (!s.is_empty() && naive(&s[1..], p)),
            Some(&c) => !s.is_empty() && (c == b'?' || c == s[0]) && naive(&s[1..], &p[1..]),
        }
    }

    #[test]
    fn examples() {
        assert!(!m("aa", "a"));
        assert!(m("aa", "*"));
        assert!(!m("cb", "?a"));
        assert!(m("adceb", "*a*b"));
        assert!(!m("acdcb", "a*c?b"));
    }

    #[test]
    fn empty_strings_and_stars() {
        assert!(m("", ""));
        assert!(m("", "***"));
        assert!(!m("", "?"));
        assert!(!m("a", ""));
        assert!(m("abc", "a**?"));
    }

    #[test]
    fn matches_naive() {
        let strings = ["", "a", "ab", "abc", "aab", "bba", "abab"];
        let patterns = [
            "*", "?", "a*", "*b", "?*?", "a?b", "*a*b*", "a*a", "??*", "b*",
        ];
        for s in strings {
            for p in patterns {
                assert_eq!(
                    m(s, p),
                    naive(s.as_bytes(), p.as_bytes()),
                    "s = {s:?}, p = {p:?}"
                );
            }
        }
    }

    #[test]
    fn large_inputs() {
        let s = "a".repeat(2_000);
        let p = "*".repeat(1_000) + "b";
        assert!(!m(&s, &p));
        let p = "a*".repeat(999) + "a";
        assert!(m(&s, &p));
    }
}
