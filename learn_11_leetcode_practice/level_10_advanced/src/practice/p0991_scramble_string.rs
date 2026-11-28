// ==================================================================
// PRACTICE 0991: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0991_scramble_string.md
// Run the tests:          cargo test practice::p0991
// ==================================================================

//! # Problem 0991: Scramble String
//!
//! - **Difficulty:** Hard
//! - **Topics:** interval DP over (start1, start2, length), memoisation with a flat `Vec<Option<bool>>`, letter-count pruning
//! - **Source:** [LeetCode 87. Scramble String](https://leetcode.com/problems/scramble-string/)
//!
//! ## Problem
//!
//! A string can be **scrambled** recursively: a string of length 1 stays as it is; a longer string is
//! split at any position into two non-empty parts `x + y`, the parts are optionally swapped (giving
//! `x + y` or `y + x`), and then each part is scrambled recursively. Given two strings of the same
//! length, decide whether `s2` is a scrambled version of `s1`.
//!
//! Implement `Solution::is_scramble(s1: String, s2: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! s1 = "great", s2 = "rgeat"   => true    ("gr|eat" -> swap "g|r" -> "rg" + "eat")
//! s1 = "abcde", s2 = "caebd"   => false
//! s1 = "a",     s2 = "a"       => true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s1.len() == s2.len() <= 30`
//! - Lowercase English letters only.
//!
//! ## Hints
//!
//! 1. `scramble(i, j, len)`: is `s2[j..j + len]` a scramble of `s1[i..i + len]`? Try every split `k`:
//!    without swap, `(i, j, k)` and `(i + k, j + k, len - k)`; with swap, `(i, j + len - k, k)` and
//!    `(i + k, j, len - k)`.
//! 2. Memoise the `n^3` states.
//! 3. Prune: two substrings with different letter counts can never match.

pub struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(a: &str, b: &str) -> bool {
        Solution::is_scramble(a.to_string(), b.to_string())
    }

    #[test]
    fn examples() {
        assert!(run("great", "rgeat"));
        assert!(!run("abcde", "caebd"));
        assert!(run("a", "a"));
    }

    #[test]
    fn swaps_at_several_levels() {
        assert!(run("abcd", "badc"));
        assert!(run("abcd", "dcba"));
        assert!(run("abc", "bca"));
        assert!(!run("abcd", "bdac"));
        assert!(!run("ab", "cd"));
    }

    #[test]
    fn every_scramble_of_a_short_string_is_found() {
        // Generate all scrambles of "abcde" and check them; also check a non-scramble.
        fn scrambles(s: &str) -> Vec<String> {
            if s.len() == 1 {
                return vec![s.to_string()];
            }
            let mut out = Vec::new();
            for k in 1..s.len() {
                for x in scrambles(&s[..k]) {
                    for y in scrambles(&s[k..]) {
                        out.push(format!("{x}{y}"));
                        out.push(format!("{y}{x}"));
                    }
                }
            }
            out.sort();
            out.dedup();
            out
        }
        let all = scrambles("abcde");
        assert!(all.iter().all(|t| run("abcde", t)));
        assert!(!all.contains(&"caebd".to_string()));
    }

    #[test]
    fn longest_inputs() {
        let a = "abcdefghijklmnopqrstuvwxyzabcd";
        let b: String = a.chars().rev().collect();
        assert!(run(a, &b));
        assert!(!run(
            "eebaacbcbcadaaedceaaacadccd",
            "eadcaacabaddaceacbceaabeccd"
        ));
    }
}
