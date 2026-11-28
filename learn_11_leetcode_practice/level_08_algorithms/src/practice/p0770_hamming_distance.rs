// ==================================================================
// PRACTICE 0770: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0770_hamming_distance.md
// Run the tests:          cargo test practice::p0770
// ==================================================================

//! # Problem 0770: Hamming Distance
//!
//! - **Difficulty:** Easy
//! - **Topics:** bit manipulation, XOR `^`, `count_ones`, clearing the lowest set bit
//! - **Source:** [LeetCode 461. Hamming Distance](https://leetcode.com/problems/hamming-distance/)
//!
//! ## Problem
//!
//! The **Hamming distance** between two integers is the number of bit positions at which their
//! binary representations differ. Given two non-negative integers `x` and `y`, return their
//! Hamming distance.
//!
//! Implement `Solution::hamming_distance(x: i32, y: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  x = 1, y = 4
//! Output: 2
//!         1 = 0 0 1
//!         4 = 1 0 0
//!             ^   ^   two positions differ
//!
//! Input:  x = 3, y = 1
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `0 <= x, y <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Which bitwise operator produces a `1` exactly where two bits differ?
//! 2. Then count the `1` bits of the result.

pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_and_four() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }

    #[test]
    fn example_three_and_one() {
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(Solution::hamming_distance(0, 0), 0);
        assert_eq!(Solution::hamming_distance(i32::MAX, i32::MAX), 0);
    }

    #[test]
    fn complementary_patterns() {
        assert_eq!(Solution::hamming_distance(0b1010, 0b0101), 4);
    }

    #[test]
    fn largest_distance() {
        assert_eq!(Solution::hamming_distance(0, i32::MAX), 31);
    }
}
