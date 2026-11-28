// ==================================================================
// PRACTICE 0775: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0775_single_number_ii.md
// Run the tests:          cargo test practice::p0775
// ==================================================================

//! # Problem 0775: Single Number II
//!
//! - **Difficulty:** Medium
//! - **Topics:** counting bits modulo 3, arithmetic shift of negative numbers, `fold`
//! - **Source:** [LeetCode 137. Single Number II](https://leetcode.com/problems/single-number-ii/)
//!
//! ## Problem
//!
//! In the vector `nums`, every value appears **exactly three times**, except one value that
//! appears exactly once. Return that single value. Use linear time and constant extra memory (no
//! hash map, no sorting).
//!
//! Implement `Solution::single_number(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 2, 3, 2]
//! Output: 3
//!
//! Input:  nums = [0, 1, 0, 1, 0, 1, 99]
//! Output: 99
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 3 * 10^4`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//! - Every value appears three times except one, which appears once.
//!
//! ## Hints
//!
//! 1. XOR alone does not help here: three equal values XOR to the value itself.
//! 2. Look at one bit position. How many numbers have that bit set? What is that count modulo 3?

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_small() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn example_with_zeros() {
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::single_number(vec![7]), 7);
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            Solution::single_number(vec![-2, -2, 1, 1, 4, 1, 4, 4, -4, -2]),
            -4
        );
    }

    #[test]
    fn extreme_values() {
        assert_eq!(
            Solution::single_number(vec![i32::MAX, i32::MIN, i32::MAX, i32::MAX]),
            i32::MIN
        );
        assert_eq!(
            Solution::single_number(vec![i32::MIN, i32::MAX, i32::MIN, i32::MIN]),
            i32::MAX
        );
    }

    #[test]
    fn long_input() {
        let mut nums: Vec<i32> = (0..9_999).flat_map(|i| [i * 3 - 7_000; 3]).collect();
        nums.push(123_456_789);
        assert_eq!(Solution::single_number(nums), 123_456_789);
    }
}
