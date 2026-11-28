// ==================================================================
// PRACTICE 0499: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0499_basic_calculator.md
// Run the tests:          cargo test practice::p0499
// ==================================================================

//! # Problem 0499: Basic Calculator
//!
//! - **Difficulty:** Hard
//! - **Topics:** tokenising with `Peekable` and `next_if`, an explicit stack instead of recursion, unary minus, `i64` accumulation
//! - **Source:** [LeetCode 224. Basic Calculator](https://leetcode.com/problems/basic-calculator/)
//!
//! ## Problem
//!
//! Evaluate an arithmetic expression given as a string. It contains non-negative integers, `'+'`,
//! `'-'`, parentheses `'('` and `')'`, and spaces. `'-'` may be **unary** (as in `"-(2 + 3)"` or
//! `"-1 + 2"`), but `'+'` is never unary. Do not use any built-in expression evaluator.
//!
//! Implement `Solution::calculate(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "1 + 1"                  Output: 2
//! Input:  s = " 2-1 + 2 "              Output: 3
//! Input:  s = "(1+(4+5+2)-3)+(6+8)"    Output: 23
//! Input:  s = "- (3 + (4 + 5))"        Output: -12
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 3 * 10^5`
//! - `s` is a valid expression made of digits, `'+'`, `'-'`, `'('`, `')'` and `' '`.
//! - No two operators are adjacent; every number and every intermediate result fits in an `i32`.
//!
//! ## Hints
//!
//! 1. With only `+` and `-`, an expression is a sum of signed terms. Keep a running `total` and the
//!    `sign` (`+1` or `-1`) of the next term.
//! 2. A `'('` starts a new sub-total: push the current `(total, sign)` on a stack and restart. A
//!    `')'` finishes it: pop `(previous, sign)` and set `total = previous + sign * total`.
//! 3. Read multi-digit numbers with `chars.next_if(|c| c.is_ascii_digit())`.

pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> i32 {
        Solution::calculate(s.to_string())
    }

    #[test]
    fn examples_without_parentheses() {
        assert_eq!(run("1 + 1"), 2);
        assert_eq!(run(" 2-1 + 2 "), 3);
    }

    #[test]
    fn example_parentheses() {
        assert_eq!(run("(1+(4+5+2)-3)+(6+8)"), 23);
    }

    #[test]
    fn unary_minus() {
        assert_eq!(run("- (3 + (4 + 5))"), -12);
        assert_eq!(run("-2+ 1"), -1);
        assert_eq!(run("1-(     -2)"), 3);
        assert_eq!(run("-(-(-7))"), -7);
    }

    #[test]
    fn large_numbers() {
        assert_eq!(run("2147483647"), i32::MAX);
        assert_eq!(run("2147483647 - 2147483647 + 1"), 1);
        assert_eq!(run("0-2147483647-1"), i32::MIN);
    }

    #[test]
    fn deep_nesting_does_not_overflow_the_stack() {
        // 3 * 99_999 + 1 characters: within the 3 * 10^5 limit.
        let depth = 99_999;
        let s = format!("{}1{}", "(".repeat(depth), ")".repeat(depth));
        assert_eq!(run(&s), 1);
        let s = format!("{}5{}", "-(".repeat(depth), ")".repeat(depth));
        assert_eq!(run(&s), -5);
    }
}
