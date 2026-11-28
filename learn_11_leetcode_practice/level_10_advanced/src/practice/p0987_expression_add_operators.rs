// ==================================================================
// PRACTICE 0987: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0987_expression_add_operators.md
// Run the tests:          cargo test practice::p0987
// ==================================================================

//! # Problem 0987: Expression Add Operators
//!
//! - **Difficulty:** Hard
//! - **Topics:** backtracking over digit splits, evaluating `*` precedence on the fly with "last operand", `i64`, a shared `String` buffer with `truncate`
//! - **Source:** [LeetCode 282. Expression Add Operators](https://leetcode.com/problems/expression-add-operators/)
//!
//! ## Problem
//!
//! Given a string `num` of digits and an integer `target`, insert the binary operators `+`, `-` or
//! `*` between the digits (or nothing, which joins digits into a longer number) so that the resulting
//! expression evaluates to `target` with normal precedence (`*` before `+` and `-`). Return **all**
//! such expressions, in any order. Operands must not have leading zeros (`"05"` is not allowed, `"0"`
//! is).
//!
//! Implement `Solution::add_operators(num: String, target: i32) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = "123", target = 6          Output: ["1*2*3", "1+2+3"]
//! Input:  num = "232", target = 8          Output: ["2*3+2", "2+3*2"]
//! Input:  num = "3456237490", target = 9191 Output: []
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num.len() <= 10`, `num` consists of digits.
//! - `-2^31 <= target <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Recurse over the position: choose the next operand `num[pos..end]` and an operator before it.
//! 2. Track the value so far and the **last operand** (with its sign). For `*`, undo the last operand
//!    and add `last * operand` instead: `value - last + last * operand`.
//! 3. Values can exceed `i32` (`9999999999`): use `i64`.

pub struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(num: &str, target: i32) -> Vec<String> {
        let mut out = Solution::add_operators(num.to_string(), target);
        out.sort();
        out
    }

    /// Evaluates an expression with `+ - *` and normal precedence.
    fn eval(expr: &str) -> i64 {
        let mut total = 0;
        let mut term = 1;
        let mut sign = 1;
        let mut number = 0;
        for ch in expr.chars().chain(std::iter::once('+')) {
            match ch {
                '0'..='9' => number = number * 10 + i64::from(ch as u8 - b'0'),
                '*' => {
                    term *= number;
                    number = 0;
                }
                _ => {
                    total += sign * term * number;
                    term = 1;
                    number = 0;
                    sign = if ch == '-' { -1 } else { 1 };
                }
            }
        }
        total
    }

    #[test]
    fn examples() {
        assert_eq!(run("123", 6), ["1*2*3", "1+2+3"]);
        assert_eq!(run("232", 8), ["2*3+2", "2+3*2"]);
        assert!(run("3456237490", 9191).is_empty());
    }

    #[test]
    fn zeros() {
        assert_eq!(run("105", 5), ["1*0+5", "10-5"]);
        assert_eq!(run("00", 0), ["0*0", "0+0", "0-0"]);
        assert_eq!(run("0", 0), ["0"]);
    }

    #[test]
    fn large_numbers_do_not_overflow() {
        assert!(run("2147483648", -2_147_483_648).is_empty());
        assert_eq!(run("2147483647", 2_147_483_647), ["2147483647"]);
    }

    #[test]
    fn every_result_evaluates_to_the_target() {
        let results = run("123456789", 45);
        assert!(results.contains(&"1+2+3+4+5+6+7+8+9".to_string()));
        assert!(results.iter().all(|e| eval(e) == 45));
        let digits: Vec<String> = results
            .iter()
            .map(|e| e.chars().filter(char::is_ascii_digit).collect())
            .collect();
        assert!(digits.iter().all(|d| d == "123456789"));
        assert_eq!(results.len(), 121);
    }
}
