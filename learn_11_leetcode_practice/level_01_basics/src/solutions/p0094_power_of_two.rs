//! # Problem 0094: Power of Two
//!
//! - **Difficulty:** Easy
//! - **Topics:** bitwise `&`, `count_ones`, binary representation, guarding against non-positive input
//! - **Source:** [LeetCode 231. Power of Two](https://leetcode.com/problems/power-of-two/)
//!
//! ## Problem
//!
//! Given an integer `n`, return `true` if `n` is a power of two, meaning `n == 2^x` for some
//! integer `x >= 0` (so `1, 2, 4, 8, ...`). Try to solve it without loops or recursion.
//!
//! Implement `Solution::is_power_of_two(n: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 1
//! Output: true
//! Explanation: 2^0 = 1.
//!
//! Input:  n = 16
//! Output: true
//!
//! Input:  n = 3
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `-2^31 <= n <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. In binary, a power of two has exactly one `1` bit: `16` is `10000`.
//! 2. Subtracting 1 flips that bit and turns all lower bits on: `16 - 1` is `01111`. What is `16 & 15`?
//! 3. Zero and negative numbers are never powers of two.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for `n > 0`, `n & (n - 1)` clears the lowest `1` bit. A power of two has only one
//! bit set, so the result is `0` exactly for powers of two. Equivalently, `n.count_ones() == 1`.
//!
//! **Complexity:** `O(1)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `&`, `|`, `^`, `!`, `<<` and `>>` are the bitwise operators on integers (`!` is bitwise NOT for
//!   integers and logical NOT for `bool`).
//! - `n > 0 && ...` short-circuits, so `n - 1` is never computed for `n = i32::MIN` (where it would overflow).
//! - Integers have bit-counting methods: `count_ones`, `leading_zeros`, `trailing_zeros`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // @solution-start
        n > 0 && n & (n - 1) == 0
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(Solution::is_power_of_two(1));
    }

    #[test]
    fn example_sixteen() {
        assert!(Solution::is_power_of_two(16));
    }

    #[test]
    fn example_three() {
        assert!(!Solution::is_power_of_two(3));
    }

    #[test]
    fn non_positive_values() {
        assert!(!Solution::is_power_of_two(0));
        assert!(!Solution::is_power_of_two(-16));
        assert!(!Solution::is_power_of_two(i32::MIN));
    }

    #[test]
    fn every_power_and_neighbors() {
        for k in 0..31 {
            let p = 1 << k;
            assert!(Solution::is_power_of_two(p), "2^{k}");
            if p > 2 {
                assert!(!Solution::is_power_of_two(p - 1));
                assert!(!Solution::is_power_of_two(p + 1));
            }
        }
        assert!(!Solution::is_power_of_two(i32::MAX));
    }
}
