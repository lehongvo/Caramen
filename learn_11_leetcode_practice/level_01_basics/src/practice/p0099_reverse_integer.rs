// ==================================================================
// PRACTICE 0099: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0099_reverse_integer.md
// Run the tests:          cargo test practice::p0099
// ==================================================================

//! # Problem 0099: Reverse Integer
//!
//! - **Difficulty:** Medium
//! - **Topics:** `checked_mul` and `checked_add` on `i32`, `%` with negative numbers, `match` on `Option`
//! - **Source:** [LeetCode 7. Reverse Integer](https://leetcode.com/problems/reverse-integer/)
//!
//! ## Problem
//!
//! Given a signed 32-bit integer `x`, return `x` with its decimal digits reversed (the sign stays in
//! front). If the reversed value does not fit in an `i32` (outside `-2^31 ..= 2^31 - 1`), return `0`.
//!
//! Pretend that 64-bit integers do not exist: do all the arithmetic in `i32` and detect overflow
//! instead of avoiding it with a wider type.
//!
//! Implement `Solution::reverse(x: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  x = 123
//! Output: 321
//!
//! Input:  x = -123
//! Output: -321
//!
//! Input:  x = 120
//! Output: 21
//!
//! Input:  x = 1534236469
//! Output: 0
//! Explanation: 9646324351 is larger than i32::MAX.
//! ```
//!
//! ## Constraints
//!
//! - `-2^31 <= x <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. The digit loop `rev = rev * 10 + x % 10; x /= 10;` works for negative numbers too: in Rust,
//!    `-123 % 10 == -3` and `-123 / 10 == -12`.
//! 2. Either step (`* 10` or `+ digit`) can overflow. Use `checked_mul(10)` and `checked_add(digit)`.
//! 3. Do not take `x.abs()` first: `i32::MIN.abs()` itself overflows.

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_positive() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn example_negative() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn example_trailing_zero() {
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn example_overflow() {
        assert_eq!(Solution::reverse(1_534_236_469), 0);
    }

    #[test]
    fn boundaries() {
        assert_eq!(Solution::reverse(i32::MAX), 0);
        assert_eq!(Solution::reverse(i32::MIN), 0);
        // 1463847412 reversed is 2147483641, which still fits.
        assert_eq!(Solution::reverse(1_463_847_412), 2_147_483_641);
        // -1463847412 reversed is -2147483641; -2147483648 itself is not reachable.
        assert_eq!(Solution::reverse(-1_463_847_412), -2_147_483_641);
        // 1563847412 reversed would be 2147483651 > i32::MAX.
        assert_eq!(Solution::reverse(1_563_847_412), 0);
    }
}
