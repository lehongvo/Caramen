// ==================================================================
// PRACTICE 0178: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0178_is_subsequence.md
// Run the tests:          cargo test practice::p0178
// ==================================================================

//! # Problem 0178: Is Subsequence
//!
//! - **Difficulty:** Easy
//! - **Topics:** one iterator shared across a loop, `Iterator::any` consuming items, `all`
//! - **Source:** [LeetCode 392. Is Subsequence](https://leetcode.com/problems/is-subsequence/)
//!
//! ## Problem
//!
//! A string `s` is a *subsequence* of `t` if you can obtain `s` by deleting some (possibly zero)
//! characters of `t` without changing the order of the remaining ones. For example, `"ace"` is a
//! subsequence of `"abcde"`, but `"aec"` is not.
//!
//! Implement `Solution::is_subsequence(s: String, t: String) -> bool`. The empty string is a
//! subsequence of every string.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "abc", t = "ahbgdc"
//! Output: true
//!
//! Input:  s = "axc", t = "ahbgdc"
//! Output: false
//!
//! Input:  s = "", t = "anything"
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `0 <= s.len() <= 100`
//! - `0 <= t.len() <= 10^4`
//! - Both strings consist of lowercase English letters.
//!
//! ## Hints
//!
//! 1. Greedy works: match each character of `s` with the **earliest** possible character of `t`
//!    after the previous match.
//! 2. Keep one iterator over `t` for the whole function. `t_chars.any(|c| c == wanted)` advances it
//!    up to and including the first match, so the next search continues after it.

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str, t: &str) -> bool {
        Solution::is_subsequence(s.to_string(), t.to_string())
    }

    #[test]
    fn example_found() {
        assert!(check("abc", "ahbgdc"));
    }

    #[test]
    fn example_not_found() {
        assert!(!check("axc", "ahbgdc"));
    }

    #[test]
    fn example_empty_s() {
        assert!(check("", "anything"));
        assert!(check("", ""));
    }

    #[test]
    fn order_matters_and_repeats_need_repeats() {
        assert!(!check("ba", "ab"));
        assert!(!check("aa", "a"));
        assert!(check("aa", "baba"));
    }

    #[test]
    fn s_longer_than_t() {
        assert!(!check("abc", "ab"));
    }

    #[test]
    fn largest_input() {
        let t = format!("{}{}", "x".repeat(9_900), "abcdefghij".repeat(10));
        // `t` contains ten "a...j" blocks, so ten ("a", "j") pairs fit but not eleven.
        let s = "aj".repeat(10);
        assert!(check(&s, &t));
        assert!(!check(&format!("{s}a"), &t));
    }
}
