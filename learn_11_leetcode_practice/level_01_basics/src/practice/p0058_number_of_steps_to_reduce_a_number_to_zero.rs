// ==================================================================
// PRACTICE 0058: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0058_number_of_steps_to_reduce_a_number_to_zero.md
// Run the tests:          cargo test practice::p0058
// ==================================================================

//! # Problem 0058: Number of Steps to Reduce a Number to Zero
//!
//! - **Difficulty:** Easy
//! - **Topics:** `while` loops, `if` / `else`, compound assignment `/=` and `-=`
//! - **Source:** [LeetCode 1342. Number of Steps to Reduce a Number to Zero](https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/)
//!
//! ## Problem
//!
//! Given a non-negative integer `num`, repeat the following step until it becomes `0`:
//!
//! - if the current number is even, divide it by 2;
//! - otherwise, subtract 1.
//!
//! Return how many steps were needed.
//!
//! Implement `Solution::number_of_steps(num: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 14
//! Output: 6
//! Explanation: 14 -> 7 -> 6 -> 3 -> 2 -> 1 -> 0.
//!
//! Input:  num = 8
//! Output: 4
//! Explanation: 8 -> 4 -> 2 -> 1 -> 0.
//!
//! Input:  num = 123
//! Output: 12
//! ```
//!
//! ## Constraints
//!
//! - `0 <= num <= 10^6`
//!
//! ## Hints
//!
//! 1. A `while` loop runs as long as its condition is `true`: `while n != 0 { ... }`.
//! 2. Parameters are immutable; copy `num` into a `let mut` variable first.

pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_fourteen() {
        assert_eq!(Solution::number_of_steps(14), 6);
    }

    #[test]
    fn example_eight() {
        assert_eq!(Solution::number_of_steps(8), 4);
    }

    #[test]
    fn example_123() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }

    #[test]
    fn zero_and_one() {
        assert_eq!(Solution::number_of_steps(0), 0);
        assert_eq!(Solution::number_of_steps(1), 1);
    }

    #[test]
    fn matches_bit_formula() {
        for num in 1..=100_000i32 {
            let bits = 32 - (num as u32).leading_zeros() as i32;
            let ones = num.count_ones() as i32;
            assert_eq!(
                Solution::number_of_steps(num),
                bits + ones - 1,
                "num = {num}"
            );
        }
        assert_eq!(Solution::number_of_steps(1_000_000), 26);
    }
}
