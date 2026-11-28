// ==================================================================
// PRACTICE 0289: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0289_basic_calculator_ii.md
// Run the tests:          cargo test practice::p0289
// ==================================================================

//! # Problem 0289: Basic Calculator II
//!
//! - **Difficulty:** Medium
//! - **Topics:** tokenizing into an enum, operator precedence with a "pending operator" state, `match` on `(Op, value)`, a stack of terms
//! - **Source:** [LeetCode 227. Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/)
//!
//! ## Problem
//!
//! Evaluate an expression string `s` made of non-negative integers, the operators `+ - * /` and
//! spaces. There are no parentheses. The usual precedence applies: `*` and `/` bind tighter than
//! `+` and `-`, and operators of the same precedence are evaluated from left to right. Integer
//! division truncates toward zero. You may **not** use any built-in "eval" facility.
//!
//! Implement `Solution::calculate(s: String) -> i32`.
//!
//! For this level, first turn the string into tokens (`enum Token { Num(i32), Op(Op) }`), then
//! evaluate the tokens.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "3+2*2"
//! Output: 7
//!
//! Input:  s = " 3/2 "
//! Output: 1
//!
//! Input:  s = " 3+5 / 2 "
//! Output: 5        (5 / 2 = 2 is computed first)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 3 * 10^5`; the expression is valid.
//! - All numbers are in `[0, 2^31 - 1]`, and the answer and every intermediate value fit in an `i32`.
//!
//! ## Hints
//!
//! 1. Keep a stack of **terms** that will be added at the end. Remember the operator that came
//!    before the current number (start with `+`).
//! 2. When a number arrives: after `+` push it, after `-` push its negation, after `*` or `/`
//!    replace the top of the stack with `top * n` or `top / n`.

pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calc(s: &str) -> i32 {
        Solution::calculate(s.to_string())
    }

    #[test]
    fn example_precedence() {
        assert_eq!(calc("3+2*2"), 7);
    }

    #[test]
    fn example_division() {
        assert_eq!(calc(" 3/2 "), 1);
    }

    #[test]
    fn example_mixed() {
        assert_eq!(calc(" 3+5 / 2 "), 5);
    }

    #[test]
    fn left_to_right_and_negative_terms() {
        assert_eq!(calc("14-3/2"), 13);
        assert_eq!(calc("2*3*4/5"), 4);
        assert_eq!(calc("1-1-1"), -1);
        assert_eq!(calc("1-3*2"), -5);
        assert_eq!(calc("0-7/2"), -3);
    }

    #[test]
    fn large_numbers_and_long_input() {
        assert_eq!(calc("2147483647"), i32::MAX);
        assert_eq!(calc("2147483647 - 2147483647 * 1"), 0);
        let long = "1 + ".repeat(50_000) + "1";
        assert_eq!(calc(&long), 50_001);
    }
}
