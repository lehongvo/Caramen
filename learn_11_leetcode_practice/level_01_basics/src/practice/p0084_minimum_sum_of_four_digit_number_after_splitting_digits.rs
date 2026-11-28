// ==================================================================
// PRACTICE 0084: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0084_minimum_sum_of_four_digit_number_after_splitting_digits.md
// Run the tests:          cargo test practice::p0084
// ==================================================================

//! # Problem 0084: Minimum Sum of Four Digit Number After Splitting Digits
//!
//! - **Difficulty:** Easy
//! - **Topics:** fixed-size array `[i32; 4]`, sorting an array, place values, greedy reasoning
//! - **Source:** [LeetCode 2160. Minimum Sum of Four Digit Number After Splitting Digits](https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/)
//!
//! ## Problem
//!
//! You are given a four-digit positive integer `num`. Use **all four** of its digits (each exactly
//! once) to form two new integers `new1` and `new2`. Leading zeros are allowed in them, and either
//! may have any number of the digits. Return the minimum possible value of `new1 + new2`.
//!
//! Implement `Solution::minimum_sum(num: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 2932
//! Output: 52
//! Explanation: digits 2, 9, 3, 2. Form 29 and 23: 29 + 23 = 52.
//!
//! Input:  num = 4009
//! Output: 13
//! Explanation: form 04 and 09: 4 + 9 = 13.
//! ```
//!
//! ## Constraints
//!
//! - `1000 <= num <= 9999`
//!
//! ## Hints
//!
//! 1. Using two 2-digit numbers is always best: a 3-digit number would put some digit in the hundreds place.
//! 2. The two smallest digits should be the tens digits, and the two largest the units digits.
//! 3. Put the digits into an array `[i32; 4]` and call `.sort()` on it.

pub struct Solution;

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2932() {
        assert_eq!(Solution::minimum_sum(2932), 52);
    }

    #[test]
    fn example_4009() {
        assert_eq!(Solution::minimum_sum(4009), 13);
    }

    #[test]
    fn boundaries() {
        assert_eq!(Solution::minimum_sum(1000), 1);
        assert_eq!(Solution::minimum_sum(9999), 198);
    }

    #[test]
    fn all_different_digits() {
        // digits 1, 2, 3, 4: 13 + 24 = 37
        assert_eq!(Solution::minimum_sum(4321), 37);
        assert_eq!(Solution::minimum_sum(1234), 37);
    }
}
