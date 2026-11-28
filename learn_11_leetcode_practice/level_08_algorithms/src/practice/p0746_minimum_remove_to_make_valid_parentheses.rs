// ==================================================================
// PRACTICE 0746: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0746_minimum_remove_to_make_valid_parentheses.md
// Run the tests:          cargo test practice::p0746
// ==================================================================

//! # Problem 0746: Minimum Remove to Make Valid Parentheses
//!
//! - **Difficulty:** Medium
//! - **Topics:** stack of indices, marking with a `Vec<bool>`, `zip` + `filter`, building a `String`
//! - **Source:** [LeetCode 1249. Minimum Remove to Make Valid Parentheses](https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/)
//!
//! ## Problem
//!
//! You are given a string `s` of `'('`, `')'` and lowercase English letters. Remove the
//! **minimum** number of parentheses (at any positions) so that the remaining string is valid, and
//! return it. A string is valid when every `')'` closes an earlier unmatched `'('` and every
//! `'('` is eventually closed. Letters are always kept. If several answers exist, return any one.
//!
//! Implement `Solution::min_remove_to_make_valid(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "lee(t(c)o)de)"
//! Output: "lee(t(c)o)de"     ("lee(t(co)de)" and "lee(t(c)ode)" are also accepted)
//!
//! Input:  s = "a)b(c)d"
//! Output: "ab(c)d"
//!
//! Input:  s = "))(("
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^5`
//! - `s` contains only `'('`, `')'` and lowercase English letters.
//!
//! ## Hints
//!
//! 1. A `')'` with no unmatched `'('` before it must be removed.
//! 2. After the scan, every `'('` that is still unmatched must be removed as well. Remember their
//!    **positions** on a stack.

pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(s: &str) -> bool {
        let mut depth = 0i32;
        for c in s.chars() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        depth == 0
    }

    fn is_subsequence(small: &str, big: &str) -> bool {
        let mut rest = big.chars();
        small.chars().all(|c| rest.any(|d| d == c))
    }

    /// Accepts any valid answer of the expected (minimal) length.
    fn check(s: &str, expected_len: usize) {
        let result = Solution::min_remove_to_make_valid(s.to_string());
        assert!(is_valid(&result), "{result:?} is not valid");
        assert_eq!(result.len(), expected_len, "{result:?}");
        assert!(
            is_subsequence(&result, s),
            "{result:?} is not obtained by removing characters"
        );
    }

    #[test]
    fn example_one_extra_closing() {
        check("lee(t(c)o)de)", 12);
    }

    #[test]
    fn example_closing_before_opening() {
        assert_eq!(
            Solution::min_remove_to_make_valid("a)b(c)d".to_string()),
            "ab(c)d"
        );
    }

    #[test]
    fn example_nothing_survives() {
        assert_eq!(Solution::min_remove_to_make_valid("))((".to_string()), "");
    }

    #[test]
    fn unclosed_opening() {
        check("(a(b(c)d)", 8);
    }

    #[test]
    fn letters_only() {
        assert_eq!(Solution::min_remove_to_make_valid("abc".to_string()), "abc");
    }

    #[test]
    fn long_input_with_one_extra() {
        let s = format!("{}{}", "(".repeat(50_000), ")".repeat(50_001));
        check(&s, 100_000);
    }
}
