// ==================================================================
// PRACTICE 0096: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0096_fibonacci_number.md
// Run the tests:          cargo test practice::p0096
// ==================================================================

//! # Problem 0096: Fibonacci Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** iteration instead of recursion, tuple assignment `(a, b) = (b, a + b)`, `for _ in`
//! - **Source:** [LeetCode 509. Fibonacci Number](https://leetcode.com/problems/fibonacci-number/)
//!
//! ## Problem
//!
//! The Fibonacci numbers are defined by `F(0) = 0`, `F(1) = 1`, and `F(n) = F(n - 1) + F(n - 2)`
//! for `n > 1`. Given `n`, return `F(n)`.
//!
//! Implement `Solution::fib(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 2
//! Output: 1
//! Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
//!
//! Input:  n = 3
//! Output: 2
//!
//! Input:  n = 4
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 30`
//!
//! ## Hints
//!
//! 1. A direct recursive function works but recomputes the same values exponentially many times.
//! 2. Keep only the last two values and move forward `n` times.

pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn base_cases() {
        assert_eq!(Solution::fib(0), 0);
        assert_eq!(Solution::fib(1), 1);
    }

    #[test]
    fn first_ten() {
        let values: Vec<i32> = (0..10).map(Solution::fib).collect();
        assert_eq!(values, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn largest_input() {
        assert_eq!(Solution::fib(30), 832_040);
    }
}
