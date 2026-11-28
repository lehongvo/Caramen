// ==================================================================
// PRACTICE 0081: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0081_maximum_69_number.md
// Run the tests:          cargo test practice::p0081
// ==================================================================

//! # Problem 0081: Maximum 69 Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** place values (`1, 10, 100, ...`), digit loop that remembers a position, greedy choice
//! - **Source:** [LeetCode 1323. Maximum 69 Number](https://leetcode.com/problems/maximum-69-number/)
//!
//! ## Problem
//!
//! You are given a positive integer `num` written only with the digits `6` and `9`. You may change
//! **at most one** digit: a `6` into a `9`, or a `9` into a `6`. Return the largest number you can
//! obtain.
//!
//! Implement `Solution::maximum69_number(num: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 9669
//! Output: 9969
//! Explanation: changing the first 6 gives the biggest increase.
//!
//! Input:  num = 9996
//! Output: 9999
//!
//! Input:  num = 9999
//! Output: 9999
//! Explanation: any change would make it smaller, so change nothing.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num <= 10^4`
//! - `num` consists only of the digits `6` and `9`.
//!
//! ## Hints
//!
//! 1. Turning a `9` into a `6` never helps. Turning a `6` into a `9` adds `3 * place_value`.
//! 2. So change the **leftmost** `6`. While reading digits from the right, remember the place value of
//!    the last `6` you saw.

pub struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_first_six_in_the_middle() {
        assert_eq!(Solution::maximum69_number(9669), 9969);
    }

    #[test]
    fn example_last_digit() {
        assert_eq!(Solution::maximum69_number(9996), 9999);
    }

    #[test]
    fn example_no_six() {
        assert_eq!(Solution::maximum69_number(9999), 9999);
        assert_eq!(Solution::maximum69_number(9), 9);
    }

    #[test]
    fn all_sixes() {
        assert_eq!(Solution::maximum69_number(6), 9);
        assert_eq!(Solution::maximum69_number(6666), 9666);
    }

    #[test]
    fn leading_six() {
        assert_eq!(Solution::maximum69_number(6999), 9999);
        assert_eq!(Solution::maximum69_number(696), 996);
    }
}
