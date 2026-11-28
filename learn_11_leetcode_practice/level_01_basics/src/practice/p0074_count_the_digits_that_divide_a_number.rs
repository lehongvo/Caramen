// ==================================================================
// PRACTICE 0074: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0074_count_the_digits_that_divide_a_number.md
// Run the tests:          cargo test practice::p0074
// ==================================================================

//! # Problem 0074: Count the Digits That Divide a Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** digit loop on a **copy** of the input, `%` for divisibility, counting
//! - **Source:** [LeetCode 2520. Count the Digits That Divide a Number](https://leetcode.com/problems/count-the-digits-that-divide-a-number/)
//!
//! ## Problem
//!
//! Given an integer `num` that contains no digit `0`, count how many of its digits divide `num`
//! evenly (`num % digit == 0`). Repeated digits are counted every time they appear.
//!
//! Implement `Solution::count_digits(num: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 7
//! Output: 1
//!
//! Input:  num = 121
//! Output: 2
//! Explanation: 121 is divisible by 1 (twice) but not by 2.
//!
//! Input:  num = 1248
//! Output: 4
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num <= 10^9`
//! - `num` has no digit `0`.
//!
//! ## Hints
//!
//! 1. You need two numbers at once: the original `num` (to test divisibility) and a shrinking copy
//!    (to read the digits).

pub struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_single_digit() {
        assert_eq!(Solution::count_digits(7), 1);
    }

    #[test]
    fn example_repeated_digits() {
        assert_eq!(Solution::count_digits(121), 2);
    }

    #[test]
    fn example_all_divide() {
        assert_eq!(Solution::count_digits(1248), 4);
    }

    #[test]
    fn none_but_one_divide() {
        // 13 % 3 != 0, 13 % 1 == 0.
        assert_eq!(Solution::count_digits(13), 1);
        assert_eq!(Solution::count_digits(23), 0);
    }

    #[test]
    fn large_numbers() {
        assert_eq!(Solution::count_digits(111_111_111), 9);
        assert_eq!(Solution::count_digits(999_999_999), 9);
        assert_eq!(Solution::count_digits(987_654_321), 3);
    }
}
