// ==================================================================
// PRACTICE 0833: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0833_distinct_subsequences.md
// Run the tests:          cargo test practice::p0833
// ==================================================================

//! # Problem 0833: Distinct Subsequences
//!
//! - **Difficulty:** Hard
//! - **Topics:** counting DP over two strings, one row updated right to left, `wrapping_add`
//! - **Source:** [LeetCode 115. Distinct Subsequences](https://leetcode.com/problems/distinct-subsequences/)
//!
//! ## Problem
//!
//! Given two strings `s` and `t`, return the number of distinct ways to pick a subsequence of `s`
//! that equals `t`. Two ways are different if they use a different set of positions of `s`.
//!
//! Implement `Solution::num_distinct(s: String, t: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "rabbbit", t = "rabbit"
//! Output: 3
//! Explanation: any one of the three 'b's of s can be left out.
//!
//! Input:  s = "babgbag", t = "bag"
//! Output: 5
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len(), t.len() <= 1000`
//! - `s` and `t` contain only English letters.
//! - The answer fits in an `i32` (counts for prefixes of `t` may be much larger).
//!
//! ## Hints
//!
//! 1. Let `W(i, j)` be the number of ways to form `t[..j]` from `s[..i]`. `W(i, 0) = 1`: the empty
//!    target is formed once.
//! 2. `W(i, j) = W(i - 1, j)` (do not use `s[i - 1]`) plus `W(i - 1, j - 1)` if
//!    `s[i - 1] == t[j - 1]` (use it as the last character of the match).
//! 3. With a single row indexed by `j`, iterate `j` from right to left so `W(i - 1, j - 1)` is
//!    still the old value.

pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn count(s: &str, t: &str) -> i32 {
        Solution::num_distinct(s.to_string(), t.to_string())
    }

    #[test]
    fn example_rabbit() {
        assert_eq!(count("rabbbit", "rabbit"), 3);
    }

    #[test]
    fn example_bag() {
        assert_eq!(count("babgbag", "bag"), 5);
    }

    #[test]
    fn target_longer_or_missing() {
        assert_eq!(count("abc", "abcd"), 0);
        assert_eq!(count("abc", "x"), 0);
        assert_eq!(count("abc", "abc"), 1);
    }

    #[test]
    fn repeated_pattern() {
        assert_eq!(count(&"ab".repeat(20), &"ab".repeat(3)), 100_947);
    }

    #[test]
    fn intermediate_overflow_is_harmless() {
        let s = "a".repeat(1000);
        let t = format!("{}b", "a".repeat(500));
        assert_eq!(count(&s, &t), 0);
    }
}
