// ==================================================================
// PRACTICE 0077: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0077_harshad_number.md
// Run the tests:          cargo test practice::p0077
// ==================================================================

//! # Problem 0077: Harshad Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** helper functions, digit sums, `if` as the return expression
//! - **Source:** [LeetCode 3099. Harshad Number](https://leetcode.com/problems/harshad-number/)
//!
//! ## Problem
//!
//! A positive integer is a **Harshad number** if it is divisible by the sum of its digits. Given an
//! integer `x`, return the sum of its digits if `x` is a Harshad number, and `-1` otherwise.
//!
//! Implement `Solution::sum_of_the_digits_of_harshad_number(x: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  x = 18
//! Output: 9
//! Explanation: 1 + 8 = 9 and 18 % 9 == 0.
//!
//! Input:  x = 23
//! Output: -1
//! Explanation: 2 + 3 = 5 and 23 is not divisible by 5.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= x <= 100`
//!
//! ## Hints
//!
//! 1. Write a small helper `digit_sum(n: i32) -> i32` and call it.
//! 2. The digit sum of a positive number is at least 1, so `x % sum` is always safe.

pub struct Solution;

impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_harshad() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
    }

    #[test]
    fn example_not_harshad() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(23), -1);
    }

    #[test]
    fn single_digits_are_always_harshad() {
        for d in 1..=9 {
            assert_eq!(Solution::sum_of_the_digits_of_harshad_number(d), d);
        }
    }

    #[test]
    fn boundaries() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(100), 1);
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(99), -1);
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(10), 1);
    }

    #[test]
    fn count_up_to_100() {
        let count = (1..=100)
            .filter(|&x| Solution::sum_of_the_digits_of_harshad_number(x) != -1)
            .count();
        assert_eq!(count, 33);
    }
}
