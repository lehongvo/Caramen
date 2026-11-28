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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** scan the characters once. Numbers are added to the running total with the current
//! sign; parentheses save and restore the outer state on a stack. A unary minus is just a `'-'` that
//! sets the sign of the next term (a number or a parenthesised group).
//!
//! **Complexity:** `O(n)` time, `O(depth)` space for the stack.
//!
//! **Rust concepts:**
//! - `Peekable::next_if` consumes the following characters only while they are digits.
//! - An explicit `Vec` stack instead of a recursive parser: the input can contain about `1.5 * 10^5`
//!   nested parentheses, which would overflow the call stack with recursion.
//! - The `match` on `char` uses a range pattern `'0'..='9'` and ignores spaces with `_`.
//! - Accumulating in `i64` keeps the intermediate arithmetic safe; the result fits in an `i32`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        // @solution-start
        let mut chars = s.chars().peekable();
        let mut stack: Vec<(i64, i64)> = Vec::new();
        let mut total: i64 = 0;
        let mut sign: i64 = 1;
        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    let mut number = i64::from(c as u8 - b'0');
                    while let Some(d) = chars.next_if(|c| c.is_ascii_digit()) {
                        number = number * 10 + i64::from(d as u8 - b'0');
                    }
                    total += sign * number;
                    sign = 1;
                }
                '+' => sign = 1,
                '-' => sign = -1,
                '(' => {
                    stack.push((total, sign));
                    total = 0;
                    sign = 1;
                }
                ')' => {
                    let (previous, outer_sign) = stack.pop().unwrap_or((0, 1));
                    total = previous + outer_sign * total;
                }
                _ => {} // spaces
            }
        }
        // The constraints guarantee that the result fits in an i32.
        total as i32
        // @solution-end
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
