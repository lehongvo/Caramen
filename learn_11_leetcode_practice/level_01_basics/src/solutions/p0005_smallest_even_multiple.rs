//! # Problem 0005: Smallest Even Multiple
//!
//! - **Difficulty:** Easy
//! - **Topics:** `if` as an expression, `%` to test divisibility
//! - **Source:** [LeetCode 2413. Smallest Even Multiple](https://leetcode.com/problems/smallest-even-multiple/)
//!
//! ## Problem
//!
//! Given a positive integer `n`, return the smallest positive integer that is a multiple of both
//! `2` and `n`.
//!
//! Implement `Solution::smallest_even_multiple(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 5
//! Output: 10
//! Explanation: 10 is the smallest number divisible by both 5 and 2.
//!
//! Input:  n = 6
//! Output: 6
//! Explanation: 6 is already even, so it is a multiple of 2 and of itself.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 150`
//!
//! ## Hints
//!
//! 1. If `n` is even, `n` itself works. If `n` is odd, what is the next multiple of `n`?
//! 2. In Rust, `if` is an expression: both branches can produce a value and the whole `if` is that value.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** an even `n` is already a multiple of 2. For an odd `n`, the multiples are `n, 2n, 3n, ...`
//! and the first even one is `2n`.
//!
//! **Complexity:** `O(1)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `if cond { a } else { b }` is an expression, so it can be the last expression of the
//!   function. Both branches must have the same type (`i32` here). See
//!   [The Rust Book 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#using-if-in-a-let-statement).
//! - Rust has no ternary operator `?:`; `if`/`else` fills that role.
//! - The condition must be a `bool`. `if n % 2 { ... }` does not compile: write `n % 2 == 0`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        // @solution-start
        if n % 2 == 0 { n } else { n * 2 }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_odd() {
        assert_eq!(Solution::smallest_even_multiple(5), 10);
    }

    #[test]
    fn example_even() {
        assert_eq!(Solution::smallest_even_multiple(6), 6);
    }

    #[test]
    fn one_and_two() {
        assert_eq!(Solution::smallest_even_multiple(1), 2);
        assert_eq!(Solution::smallest_even_multiple(2), 2);
    }

    #[test]
    fn largest_values() {
        assert_eq!(Solution::smallest_even_multiple(149), 298);
        assert_eq!(Solution::smallest_even_multiple(150), 150);
    }
}
