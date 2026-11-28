//! # Problem 0826: Interleaving String
//!
//! - **Difficulty:** Medium
//! - **Topics:** 2-D boolean DP compressed to one row, byte slices, early length check
//! - **Source:** [LeetCode 97. Interleaving String](https://leetcode.com/problems/interleaving-string/)
//!
//! ## Problem
//!
//! Given strings `s1`, `s2` and `s3`, return `true` if `s3` can be formed by **interleaving** `s1`
//! and `s2`: every character of `s3` is taken, in order, from the front of what is left of either
//! `s1` or `s2`, and in the end both `s1` and `s2` are used up completely. The relative order of
//! characters inside `s1` (and inside `s2`) is preserved.
//!
//! Implement `Solution::is_interleave(s1: String, s2: String, s3: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
//! Output: true
//! Explanation: aa (s1) + dbbc (s2) + bc (s1) + a (s2) + c (s1).
//!
//! Input:  s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
//! Output: false
//!
//! Input:  s1 = "", s2 = "", s3 = ""
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `0 <= s1.len(), s2.len() <= 100`
//! - `0 <= s3.len() <= 200`
//! - All strings contain only lowercase English letters.
//!
//! ## Hints
//!
//! 1. If `s1.len() + s2.len() != s3.len()`, the answer is `false` immediately.
//! 2. Let `ok(i, j)` mean "the first `i` characters of `s1` and the first `j` of `s2` interleave
//!    into the first `i + j` characters of `s3`".
//! 3. `ok(i, j)` is true if (`ok(i - 1, j)` and `s1[i - 1] == s3[i + j - 1]`) or
//!    (`ok(i, j - 1)` and `s2[j - 1] == s3[i + j - 1]`).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** greedy matching fails when both strings offer the same next character, so use the DP
//! from the hints. Store one row `ok[j]` for the current `i`: before the update, `ok[j]` still
//! holds `ok(i - 1, j)` ("came from `s1`") and `ok[j - 1]` already holds `ok(i, j - 1)` ("came
//! from `s2`").
//!
//! **Complexity:** `O(len1 * len2)` time, `O(len2)` space.
//!
//! **Rust concepts:**
//! - Byte slices (`as_bytes`) give `O(1)` indexing.
//! - Boolean DP cells combine with `&&` and `||`, which short-circuit.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // @solution-start
        let (a, b, c) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        if a.len() + b.len() != c.len() {
            return false;
        }
        let mut ok = vec![false; b.len() + 1];
        for i in 0..=a.len() {
            for j in 0..=b.len() {
                ok[j] = if i == 0 && j == 0 {
                    true
                } else {
                    let from_a = i > 0 && ok[j] && a[i - 1] == c[i + j - 1];
                    let from_b = j > 0 && ok[j - 1] && b[j - 1] == c[i + j - 1];
                    from_a || from_b
                };
            }
        }
        ok[b.len()]
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(a: &str, b: &str, c: &str) -> bool {
        Solution::is_interleave(a.to_string(), b.to_string(), c.to_string())
    }

    #[test]
    fn example_true() {
        assert!(run("aabcc", "dbbca", "aadbbcbcac"));
    }

    #[test]
    fn example_false() {
        assert!(!run("aabcc", "dbbca", "aadbbbaccc"));
    }

    #[test]
    fn example_all_empty() {
        assert!(run("", "", ""));
    }

    #[test]
    fn one_side_empty_and_length_mismatch() {
        assert!(run("abc", "", "abc"));
        assert!(!run("abc", "", "acb"));
        assert!(!run("a", "b", "a"));
    }

    #[test]
    fn greedy_choice_would_fail() {
        // Both strings start with 'a'; taking it from s1 first is the wrong choice.
        assert!(run("ab", "ac", "acab"));
    }

    #[test]
    fn long_alternating_strings() {
        let a = "ab".repeat(50);
        let b = "ba".repeat(50);
        let c: String = a.chars().zip(b.chars()).flat_map(|(x, y)| [x, y]).collect();
        assert!(run(&a, &b, &c));
        let mut wrong = c.clone();
        wrong.replace_range(0..1, "c");
        assert!(!run(&a, &b, &wrong));
    }
}
