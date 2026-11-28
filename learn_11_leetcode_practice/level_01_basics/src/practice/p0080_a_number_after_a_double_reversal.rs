// ==================================================================
// PRACTICE 0080: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0080_a_number_after_a_double_reversal.md
// Run the tests:          cargo test practice::p0080
// ==================================================================

//! # Problem 0080: A Number After a Double Reversal
//!
//! - **Difficulty:** Easy
//! - **Topics:** reasoning about trailing zeros, `||`, checking a shortcut against a simulation
//! - **Source:** [LeetCode 2119. A Number After a Double Reversal](https://leetcode.com/problems/a-number-after-a-double-reversal/)
//!
//! ## Problem
//!
//! **Reversing** an integer means writing its digits in the opposite order and dropping any leading
//! zeros: reversing `2021` gives `1202`, and reversing `12300` gives `321`.
//!
//! Given `num`, reverse it to get `reversed1`, then reverse `reversed1` to get `reversed2`. Return
//! `true` if `reversed2 == num`.
//!
//! Implement `Solution::is_same_after_reversals(num: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 526
//! Output: true
//! Explanation: 526 -> 625 -> 526.
//!
//! Input:  num = 1800
//! Output: false
//! Explanation: 1800 -> 81 -> 18.
//!
//! Input:  num = 0
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `0 <= num <= 10^6`
//!
//! ## Hints
//!
//! 1. Which digits get lost during a reversal?
//! 2. Only trailing zeros disappear, and only if the number is not 0 itself.

pub struct Solution;

impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reverse(mut x: i32) -> i32 {
        let mut rev = 0;
        while x > 0 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        rev
    }

    #[test]
    fn example_526() {
        assert!(Solution::is_same_after_reversals(526));
    }

    #[test]
    fn example_1800() {
        assert!(!Solution::is_same_after_reversals(1800));
    }

    #[test]
    fn example_zero() {
        assert!(Solution::is_same_after_reversals(0));
    }

    #[test]
    fn matches_simulation_for_every_input() {
        for num in 0..=1_000_000 {
            let expected = reverse(reverse(num)) == num;
            assert_eq!(
                Solution::is_same_after_reversals(num),
                expected,
                "num = {num}"
            );
        }
    }
}
