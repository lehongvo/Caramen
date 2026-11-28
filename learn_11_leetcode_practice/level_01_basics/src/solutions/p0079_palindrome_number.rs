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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** reject negative numbers and numbers ending in 0 (except 0). Then move the last digit of
//! `x` onto `rev` repeatedly while `x > rev`. At the end, `x` holds the left half and `rev` the
//! reversed right half. For an even number of digits they are equal; for an odd number, `rev` has
//! the middle digit too, so compare `x == rev / 10`.
//!
//! **Complexity:** `O(d)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `rev` never exceeds about `10^5`, so `rev * 10` cannot overflow. Choosing an algorithm that
//!   avoids overflow is often simpler than detecting it.
//! - `if ... { return false; }` guards at the top keep the main loop free of special cases.
//! - `||` combines the two final comparisons into one `bool` expression.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // @solution-start
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut left = x;
        let mut rev = 0;
        while left > rev {
            rev = rev * 10 + left % 10;
            left /= 10;
        }
        left == rev || left == rev / 10
        // @solution-end
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
