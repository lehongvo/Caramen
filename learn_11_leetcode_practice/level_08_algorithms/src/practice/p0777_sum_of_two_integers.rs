// ==================================================================
// PRACTICE 0777: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0777_sum_of_two_integers.md
// Run the tests:          cargo test practice::p0777
// ==================================================================

//! # Problem 0777: Sum of Two Integers
//!
//! - **Difficulty:** Medium
//! - **Topics:** addition with XOR and carries, `<<` on signed integers, two's complement
//! - **Source:** [LeetCode 371. Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/)
//!
//! ## Problem
//!
//! Given two integers `a` and `b`, return their sum **without** using the operators `+` and `-`
//! (and without methods such as `wrapping_add` that do the addition for you). Use bitwise
//! operators.
//!
//! Implement `Solution::get_sum(a: i32, b: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  a = 1, b = 2
//! Output: 3
//!
//! Input:  a = 2, b = 3
//! Output: 5
//!
//! Input:  a = -1, b = 1
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `-1000 <= a, b <= 1000`
//!
//! ## Hints
//!
//! 1. Adding two bits without the carry is XOR. The carry is AND, moved one position to the left.
//! 2. Adding the carry may create a new carry. Repeat until there is no carry left.
//! 3. Negative numbers are stored in two's complement, so the same bit rules apply to them.

pub struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_plus_two() {
        assert_eq!(Solution::get_sum(1, 2), 3);
    }

    #[test]
    fn example_two_plus_three() {
        assert_eq!(Solution::get_sum(2, 3), 5);
    }

    #[test]
    fn example_opposites() {
        assert_eq!(Solution::get_sum(-1, 1), 0);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::get_sum(-1000, -1000), -2000);
        assert_eq!(Solution::get_sum(1000, -1), 999);
        assert_eq!(Solution::get_sum(-7, 3), -4);
    }

    #[test]
    fn every_small_pair() {
        for a in -60..=60 {
            for b in -60..=60 {
                assert_eq!(Solution::get_sum(a, b), a + b, "{a} + {b}");
            }
        }
    }
}
