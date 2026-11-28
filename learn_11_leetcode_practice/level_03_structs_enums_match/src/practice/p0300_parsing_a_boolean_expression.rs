// ==================================================================
// PRACTICE 0300: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0300_parsing_a_boolean_expression.md
// Run the tests:          cargo test practice::p0300
// ==================================================================

//! # Problem 0300: Parsing A Boolean Expression
//!
//! - **Difficulty:** Hard
//! - **Topics:** an operator `enum` with an `eval` method, a stack of `Frame` structs, `match` on `char`, `Option` for the pending operator
//! - **Source:** [LeetCode 1106. Parsing A Boolean Expression](https://leetcode.com/problems/parsing-a-boolean-expression/)
//!
//! ## Problem
//!
//! A boolean expression is one of:
//!
//! - `'t'` (true) or `'f'` (false);
//! - `'!(expr)'`: the logical NOT of one inner expression;
//! - `'&(expr1,expr2,...)'`: the logical AND of one or more inner expressions;
//! - `'|(expr1,expr2,...)'`: the logical OR of one or more inner expressions.
//!
//! Given a valid `expression`, return its value.
//!
//! Implement `Solution::parse_bool_expr(expression: String) -> bool`.
//!
//! For this level, evaluate it with an explicit stack instead of recursion: each open parenthesis
//! pushes a `Frame { op: Op, values: Vec<bool> }` with `enum Op { Not, And, Or }`, and each closing
//! parenthesis pops a frame, evaluates it, and passes the result to the frame below.
//!
//! ## Examples
//!
//! ```text
//! Input:  expression = "&(|(f))"
//! Output: false        (|(f) is false, &(false) is false)
//!
//! Input:  expression = "|(f,f,f,t)"
//! Output: true
//!
//! Input:  expression = "!(&(f,t))"
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= expression.len() <= 2 * 10^4`; `expression` is valid and uses only `(` `)` `&` `|` `!`
//!   `t` `f` `,`.
//!
//! ## Hints
//!
//! 1. An operator character is always followed by `(`; remember it as `pending: Option<Op>` until
//!    the `(` arrives.
//! 2. `Op::eval(self, values: &[bool]) -> bool`: `Not` negates the single value, `And` is
//!    `values.iter().all(..)`, `Or` is `values.iter().any(..)`.
//! 3. A value (from `t`, `f` or a closed frame) goes into the top frame, or becomes the final result
//!    when the stack is empty.

pub struct Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(e: &str) -> bool {
        Solution::parse_bool_expr(e.to_string())
    }

    #[test]
    fn example_nested_and_or() {
        assert!(!eval("&(|(f))"));
    }

    #[test]
    fn example_or() {
        assert!(eval("|(f,f,f,t)"));
    }

    #[test]
    fn example_not_and() {
        assert!(eval("!(&(f,t))"));
    }

    #[test]
    fn literals_and_single_operands() {
        assert!(eval("t"));
        assert!(!eval("f"));
        assert!(!eval("!(t)"));
        assert!(eval("&(t)"));
        assert!(!eval("|(f)"));
    }

    #[test]
    fn deeper_expressions() {
        assert!(eval("|(&(t,f,t),!(f))"));
        assert!(!eval("&(|(f,t),!(|(t,f)))"));
        assert!(eval("!(!(!(f)))"));
    }

    #[test]
    fn long_expression() {
        // 5_000 nested NOTs around t: an even count gives t.
        let e = "!(".repeat(5_000) + "t" + &")".repeat(5_000);
        assert!(eval(&e));
        let wide = format!("&({})", vec!["t"; 9_000].join(","));
        assert!(eval(&wide));
        let wide_with_f = format!("&({},f)", vec!["t"; 9_000].join(","));
        assert!(!eval(&wide_with_f));
    }
}
