// ==================================================================
// PRACTICE 0286: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0286_evaluate_reverse_polish_notation.md
// Run the tests:          cargo test practice::p0286
// ==================================================================

//! # Problem 0286: Evaluate Reverse Polish Notation
//!
//! - **Difficulty:** Medium
//! - **Topics:** parsing strings into a `Token` enum, an `Op` enum with an `apply` method, a stack, slice patterns on the stack
//! - **Source:** [LeetCode 150. Evaluate Reverse Polish Notation](https://leetcode.com/problems/evaluate-reverse-polish-notation/)
//!
//! ## Problem
//!
//! In **Reverse Polish Notation** (postfix notation) an operator comes after its two operands:
//! `3 4 +` means `3 + 4`, and `2 1 + 3 *` means `(2 + 1) * 3`. No parentheses are ever needed.
//!
//! You are given the expression as a list of tokens: integers (possibly negative, like `"-11"`) and
//! the operators `"+"`, `"-"`, `"*"`, `"/"`. Division between integers **truncates toward zero**.
//! The expression is always valid, never divides by zero, and every intermediate result fits in a
//! 32-bit integer. Return its value.
//!
//! Implement `Solution::eval_rpn(tokens: Vec<String>) -> i32`.
//!
//! For this level, parse each string into `enum Token { Num(i32), Op(Op) }` with
//! `enum Op { Add, Sub, Mul, Div }`, and give `Op` a method `apply(self, a: i32, b: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  tokens = ["2", "1", "+", "3", "*"]
//! Output: 9       ((2 + 1) * 3)
//!
//! Input:  tokens = ["4", "13", "5", "/", "+"]
//! Output: 6       (4 + 13 / 5 = 4 + 2)
//!
//! Input:  tokens = ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
//! Output: 22
//! ```
//!
//! ## Constraints
//!
//! - `1 <= tokens.len() <= 10^4`
//! - Each token is an operator or an integer in `[-200, 200]`.
//!
//! ## Hints
//!
//! 1. `"-"` alone is an operator but `"-11"` is a number: match the four operator strings first,
//!    then try `parse::<i32>()` on everything else.
//! 2. Push numbers on a stack. For an operator, pop `b` first, then `a`, and push `op.apply(a, b)`.

pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rpn(tokens: &[&str]) -> i32 {
        Solution::eval_rpn(tokens.iter().map(|t| t.to_string()).collect())
    }

    #[test]
    fn example_one() {
        assert_eq!(rpn(&["2", "1", "+", "3", "*"]), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(rpn(&["4", "13", "5", "/", "+"]), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            rpn(&[
                "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
            ]),
            22
        );
    }

    #[test]
    fn subtraction_order_and_truncation() {
        assert_eq!(rpn(&["3", "5", "-"]), -2);
        assert_eq!(rpn(&["-7", "2", "/"]), -3);
        assert_eq!(rpn(&["7", "-2", "/"]), -3);
        assert_eq!(rpn(&["42"]), 42);
        assert_eq!(rpn(&["-200"]), -200);
    }

    #[test]
    fn long_expression() {
        // 1 1 + 1 + 1 + ... : 5_000 ones summed.
        let mut tokens = vec!["1"];
        for _ in 0..4_999 {
            tokens.push("1");
            tokens.push("+");
        }
        assert_eq!(rpn(&tokens), 5_000);
    }
}
