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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** run the digit loop on a mutable copy `x` of `num`; for each digit `d`, count it when
//! `num % d == 0`. The constraints guarantee `d != 0`, so the remainder never divides by zero.
//!
//! **Complexity:** `O(d)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `let mut x = num;` copies the `i32` (integers are `Copy`), so `num` keeps its original value
//!   for the divisibility tests.
//! - `num % 0` would panic with "attempt to calculate the remainder with a divisor of zero"; the
//!   problem's guarantee is what makes the plain `%` safe here.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        // @solution-start
        let mut x = num;
        let mut count = 0;
        while x > 0 {
            let d = x % 10;
            if num % d == 0 {
                count += 1;
            }
            x /= 10;
        }
        count
        // @solution-end
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
