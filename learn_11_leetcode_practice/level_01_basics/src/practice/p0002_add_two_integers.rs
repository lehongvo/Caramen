// ==================================================================
// PRACTICE 0002: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0002_add_two_integers.md
// Run the tests:          cargo test practice::p0002
// ==================================================================

//! # Problem 0002: Add Two Integers
//!
//! - **Difficulty:** Easy
//! - **Topics:** functions, parameters, return values, expressions vs statements
//! - **Source:** [LeetCode 2235. Add Two Integers](https://leetcode.com/problems/add-two-integers/)
//!
//! ## Problem
//!
//! Write a function that receives two integers, `num1` and `num2`, and returns their sum.
//!
//! This is the smallest possible Rust function with parameters and a return value. Pay attention
//! to how the result is produced: in Rust the **last expression** of a function body (written
//! without a trailing semicolon) is the value the function returns.
//!
//! Implement `Solution::sum(num1: i32, num2: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num1 = 12, num2 = 5
//! Output: 17
//!
//! Input:  num1 = -10, num2 = 4
//! Output: -6
//! Explanation: -10 + 4 = -6.
//! ```
//!
//! ## Constraints
//!
//! - `-100 <= num1, num2 <= 100`
//!
//! ## Hints
//!
//! 1. Every parameter in a Rust function must have a type annotation, and so must the return type (`-> i32`).
//! 2. `num1 + num2` is an expression. If you write `num1 + num2;` (with a semicolon) it becomes a
//!    statement that produces `()`, and the compiler reports "mismatched types".

pub struct Solution;

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_positive_numbers() {
        assert_eq!(Solution::sum(12, 5), 17);
    }

    #[test]
    fn example_negative_and_positive() {
        assert_eq!(Solution::sum(-10, 4), -6);
    }

    #[test]
    fn zeros() {
        assert_eq!(Solution::sum(0, 0), 0);
    }

    #[test]
    fn extreme_values() {
        assert_eq!(Solution::sum(100, 100), 200);
        assert_eq!(Solution::sum(-100, -100), -200);
        assert_eq!(Solution::sum(-100, 100), 0);
    }
}
