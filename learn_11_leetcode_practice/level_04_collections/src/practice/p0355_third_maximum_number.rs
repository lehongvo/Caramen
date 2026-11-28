// ==================================================================
// PRACTICE 0355: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0355_third_maximum_number.md
// Run the tests:          cargo test practice::p0355
// ==================================================================

//! # Problem 0355: Third Maximum Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** `BTreeSet` as a small sorted window, `pop_first`, `first`, `last`
//! - **Source:** [LeetCode 414. Third Maximum Number](https://leetcode.com/problems/third-maximum-number/)
//!
//! ## Problem
//!
//! Given a vector `nums`, return its **third largest distinct** value. If there are fewer than
//! three distinct values, return the largest value instead.
//!
//! LeetCode's follow-up asks for an `O(n)` solution.
//!
//! Implement `Solution::third_max(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 2, 1]
//! Output: 1
//!
//! Input:  nums = [1, 2]
//! Output: 2          (only two distinct values: return the maximum)
//!
//! Input:  nums = [2, 2, 3, 1]
//! Output: 1          (distinct values 3, 2, 1; the two 2s count once)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. A `BTreeSet` keeps distinct values in sorted order.
//! 2. You only care about the three largest values. After each insert, if the set holds four
//!    values, drop the smallest one with `pop_first()`.

use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    }

    #[test]
    fn example_two_fewer_than_three() {
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
    }

    #[test]
    fn example_three_duplicates() {
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }

    #[test]
    fn extreme_values() {
        assert_eq!(Solution::third_max(vec![1, 2, i32::MIN]), i32::MIN);
        assert_eq!(Solution::third_max(vec![i32::MAX, i32::MAX]), i32::MAX);
        assert_eq!(
            Solution::third_max(vec![i32::MIN, i32::MIN, i32::MIN]),
            i32::MIN
        );
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..10_000).map(|i| (i * 37) % 10_000).collect();
        assert_eq!(Solution::third_max(nums), 9_997);
    }
}
