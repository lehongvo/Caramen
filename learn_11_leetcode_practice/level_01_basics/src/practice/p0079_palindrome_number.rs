// ==================================================================
// PRACTICE 0079: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0079_palindrome_number.md
// Run the tests:          cargo test practice::p0079
// ==================================================================

//! # Problem 0079: Palindrome Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** reversing digits, avoiding overflow by reversing only half, early `return`
//! - **Source:** [LeetCode 9. Palindrome Number](https://leetcode.com/problems/palindrome-number/)
//!
//! ## Problem
//!
//! Given an integer `x`, return `true` if its decimal representation reads the same from left to
//! right and from right to left, and `false` otherwise. Negative numbers are never palindromes
//! (`-121` reversed is `121-`). Do not convert the number to a string.
//!
//! Implement `Solution::is_palindrome(x: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  x = 121
//! Output: true
//!
//! Input:  x = -121
//! Output: false
//!
//! Input:  x = 10
//! Output: false
//! Explanation: reversed it reads "01".
//! ```
//!
//! ## Constraints
//!
//! - `-2^31 <= x <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Reversing the whole number can overflow an `i32` (think of `2147483647`, whose reverse is `7463847412`).
//! 2. Reverse only the right half: move digits from `x` to `rev` until `rev >= x`.
//! 3. A number ending in `0` can only be a palindrome if it is `0` itself.

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_121() {
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn example_negative() {
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(i32::MIN));
    }

    #[test]
    fn example_trailing_zero() {
        assert!(!Solution::is_palindrome(10));
        assert!(!Solution::is_palindrome(1_000_000));
        assert!(Solution::is_palindrome(0));
    }

    #[test]
    fn even_and_odd_lengths() {
        assert!(Solution::is_palindrome(1221));
        assert!(Solution::is_palindrome(12_321));
        assert!(!Solution::is_palindrome(1231));
        assert!(Solution::is_palindrome(7));
    }

    #[test]
    fn large_values_do_not_overflow() {
        assert!(!Solution::is_palindrome(i32::MAX));
        assert!(Solution::is_palindrome(2_147_447_412));
        assert!(!Solution::is_palindrome(2_147_483_642));
    }
}
