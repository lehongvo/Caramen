//! # Problem 0076: Add Digits
//!
//! - **Difficulty:** Easy
//! - **Topics:** nested `while` loops, the digital root, `%` by 9
//! - **Source:** [LeetCode 258. Add Digits](https://leetcode.com/problems/add-digits/)
//!
//! ## Problem
//!
//! Given a non-negative integer `num`, replace it by the sum of its digits, and repeat until the
//! result has a single digit. Return that digit.
//!
//! Implement `Solution::add_digits(num: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 38
//! Output: 2
//! Explanation: 38 -> 3 + 8 = 11 -> 1 + 1 = 2.
//!
//! Input:  num = 0
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= num <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. The outer loop repeats while the number has two or more digits (`x >= 10`); the inner loop sums the digits.
//! 2. Follow-up: can you do it without any loop? Look at the answers for `1, 2, ..., 20`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** simulate with two nested `while` loops. The follow-up has a closed form, the
//! **digital root**: a number and its digit sum leave the same remainder modulo 9 (because
//! `10 % 9 == 1`), so the answer is `0` for `0` and `1 + (num - 1) % 9` otherwise.
//!
//! **Complexity:** the loops take `O(log num)` time overall; the formula is `O(1)`.
//!
//! **Rust concepts:**
//! - A `while` inside a `while`: the inner loop uses its own `mut` variable that is reset each time.
//! - After the inner loop, `x = sum;` feeds the result back to the outer loop.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        // @solution-start
        let mut x = num;
        while x >= 10 {
            let mut sum = 0;
            let mut rest = x;
            while rest > 0 {
                sum += rest % 10;
                rest /= 10;
            }
            x = sum;
        }
        x
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_38() {
        assert_eq!(Solution::add_digits(38), 2);
    }

    #[test]
    fn example_zero() {
        assert_eq!(Solution::add_digits(0), 0);
    }

    #[test]
    fn single_digits_are_unchanged() {
        for d in 1..=9 {
            assert_eq!(Solution::add_digits(d), d);
        }
    }

    #[test]
    fn matches_digital_root_formula() {
        for num in 1..=100_000 {
            assert_eq!(Solution::add_digits(num), 1 + (num - 1) % 9, "num = {num}");
        }
    }

    #[test]
    fn largest_input() {
        // 2147483647: digit sum 46 -> 10 -> 1.
        assert_eq!(Solution::add_digits(i32::MAX), 1);
    }
}
