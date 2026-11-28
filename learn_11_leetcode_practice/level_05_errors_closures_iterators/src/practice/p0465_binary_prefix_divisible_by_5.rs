// ==================================================================
// PRACTICE 0465: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0465_binary_prefix_divisible_by_5.md
// Run the tests:          cargo test practice::p0465
// ==================================================================

//! # Problem 0465: Binary Prefix Divisible By 5
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Iterator::scan` with a running state, modular arithmetic, collecting `Vec<bool>`
//! - **Source:** [LeetCode 1018. Binary Prefix Divisible By 5](https://leetcode.com/problems/binary-prefix-divisible-by-5/)
//!
//! ## Problem
//!
//! `nums` is a list of bits (`0` or `1`). For every `i`, let `x_i` be the number whose binary
//! representation is `nums[0..=i]` (most significant bit first). Return a list of booleans whose
//! `i`-th element tells whether `x_i` is divisible by `5`.
//!
//! Implement `Solution::prefixes_div_by5(nums: Vec<i32>) -> Vec<bool>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [0, 1, 1]
//! Output: [true, false, false]
//! Explanation: the prefixes are 0, 1 (binary 01) and 3 (binary 011). Only 0 is divisible by 5.
//!
//! Input:  nums = [1, 1, 1]
//! Output: [false, false, false]
//!
//! Input:  nums = [1, 0, 1]
//! Output: [false, false, true]
//! Explanation: binary 101 is 5.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `nums[i]` is `0` or `1`.
//!
//! ## Hints
//!
//! 1. Appending a bit `b` to `x` gives `2 * x + b`. The numbers themselves become huge, but you only
//!    need them modulo 5: `(2 * x + b) % 5 == (2 * (x % 5) + b) % 5`.
//! 2. `scan(initial_state, |state, item| ...)` is like `map` with a mutable state that survives
//!    between items.

pub struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_zero_first() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![0, 1, 1]),
            vec![true, false, false]
        );
    }

    #[test]
    fn example_all_ones() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 1, 1]),
            vec![false, false, false]
        );
    }

    #[test]
    fn example_five() {
        assert_eq!(
            Solution::prefixes_div_by5(vec![1, 0, 1]),
            vec![false, false, true]
        );
    }

    #[test]
    fn long_input_does_not_overflow() {
        let nums: Vec<i32> = (0..100_000).map(|i| (i % 3 == 0) as i32).collect();
        let result = Solution::prefixes_div_by5(nums.clone());
        assert_eq!(result.len(), 100_000);
        // Check the first 120 prefixes against their exact values (120 bits fit in a u128).
        let mut value: u128 = 0;
        for (&bit, &divisible) in nums.iter().zip(&result).take(120) {
            value = value * 2 + bit as u128;
            assert_eq!(divisible, value % 5 == 0);
        }
    }
}
