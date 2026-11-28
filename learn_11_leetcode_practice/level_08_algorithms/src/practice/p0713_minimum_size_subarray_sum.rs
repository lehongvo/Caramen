// ==================================================================
// PRACTICE 0713: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0713_minimum_size_subarray_sum.md
// Run the tests:          cargo test practice::p0713
// ==================================================================

//! # Problem 0713: Minimum Size Subarray Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** shrinking sliding window, positive values, `usize::MAX` as "not found"
//! - **Source:** [LeetCode 209. Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/)
//!
//! ## Problem
//!
//! You are given a vector `nums` of **positive** integers and a positive integer `target`. Return
//! the length of the shortest contiguous subarray whose sum is **greater than or equal to**
//! `target`. If no subarray reaches `target`, return `0`.
//!
//! Implement `Solution::min_sub_array_len(target: i32, nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  target = 7, nums = [2, 3, 1, 2, 4, 3]
//! Output: 2             ([4, 3])
//!
//! Input:  target = 4, nums = [1, 4, 4]
//! Output: 1             ([4])
//!
//! Input:  target = 11, nums = [1, 1, 1, 1, 1, 1, 1, 1]
//! Output: 0             (the whole array only sums to 8)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= target <= 10^9`
//! - `1 <= nums.len() <= 10^5`
//! - `1 <= nums[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. Because all values are positive, adding an element can only increase a window's sum and
//!    removing one can only decrease it.
//! 2. Grow the window on the right; as long as its sum reaches `target`, record its length and
//!    shrink it from the left.

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_elements() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn example_single_element() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn example_impossible() {
        assert_eq!(Solution::min_sub_array_len(11, vec![1; 8]), 0);
    }

    #[test]
    fn whole_array_needed() {
        assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);
    }

    #[test]
    fn short_window_after_long_one() {
        assert_eq!(
            Solution::min_sub_array_len(100, vec![1, 1, 1, 1, 50, 60]),
            2
        );
    }

    #[test]
    fn largest_target() {
        assert_eq!(
            Solution::min_sub_array_len(1_000_000_000, vec![10_000; 100_000]),
            100_000
        );
    }
}
