// ==================================================================
// PRACTICE 0799: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0799_generate_parentheses.md
// Run the tests:          cargo test practice::p0799
// ==================================================================

//! # Problem 0799: Generate Parentheses
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking with counters, `String::push` / `pop` as the path, Catalan numbers
//! - **Source:** [LeetCode 22. Generate Parentheses](https://leetcode.com/problems/generate-parentheses/)
//!
//! ## Problem
//!
//! Given `n`, the number of pairs of parentheses, return every **well-formed** string made of `n`
//! `'('` and `n` `')'`, in any order. A string is well-formed when every `')'` closes an earlier
//! unmatched `'('`.
//!
//! Implement `Solution::generate_parenthesis(n: i32) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 3
//! Output: ["((()))", "(()())", "(())()", "()(())", "()()()"]
//!
//! Input:  n = 1
//! Output: ["()"]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 8`
//!
//! ## Hints
//!
//! 1. Generating all `2^(2n)` strings and filtering the valid ones is wasteful.
//! 2. Build the string left to right, counting the `'('` and `')'` used so far. When may you add
//!    a `'('`? When may you add a `')'`?

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_well_formed(s: &str) -> bool {
        let mut depth = 0i32;
        for c in s.chars() {
            depth += if c == '(' { 1 } else { -1 };
            if depth < 0 {
                return false;
            }
        }
        depth == 0
    }

    fn run(n: i32) -> Vec<String> {
        let mut result = Solution::generate_parenthesis(n);
        result.sort(); // any order is accepted
        result
    }

    #[test]
    fn example_three_pairs() {
        assert_eq!(
            run(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn example_one_pair() {
        assert_eq!(run(1), vec!["()"]);
    }

    #[test]
    fn two_pairs() {
        assert_eq!(run(2), vec!["(())", "()()"]);
    }

    #[test]
    fn eight_pairs_give_the_catalan_number() {
        let result = run(8);
        assert_eq!(result.len(), 1430);
        assert!(result.iter().all(|s| s.len() == 16 && is_well_formed(s)));
        let mut deduped = result.clone();
        deduped.dedup();
        assert_eq!(deduped.len(), 1430);
    }
}
