// ==================================================================
// PRACTICE 0487: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0487_maximum_subarray.md
// Run the tests:          cargo test practice::p0487
// ==================================================================

//! # Problem 0487: Maximum Subarray
//!
//! - **Difficulty:** Medium
//! - **Topics:** Kadane's algorithm as a `scan` (best sum ending here) followed by `max`
//! - **Source:** [LeetCode 53. Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)
//!
//! ## Problem
//!
//! Given an integer array `nums`, find the **non-empty** contiguous subarray with the largest sum
//! and return that sum.
//!
//! Implement `Solution::max_sub_array(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
//! Output: 6
//! Explanation: the subarray [4, -1, 2, 1] has the largest sum.
//!
//! Input:  nums = [1]
//! Output: 1
//!
//! Input:  nums = [5, 4, -1, 7, 8]
//! Output: 23
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-10^4 <= nums[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. Let `best_here` be the largest sum of a subarray that **ends** at the current position. It is
//!    either the current number alone, or the current number appended to the best subarray ending
//!    at the previous position: `best_here = max(x, best_here + x)`.
//! 2. The answer is the maximum `best_here` over all positions. `scan` produces the `best_here`
//!    values, and `max` picks the largest.

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn example_single() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn example_almost_all_positive() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn all_negative_picks_the_largest_element() {
        assert_eq!(Solution::max_sub_array(vec![-3, -1, -2]), -1);
        assert_eq!(Solution::max_sub_array(vec![-10_000]), -10_000);
    }

    #[test]
    fn large_input() {
        let nums = vec![10_000; 100_000];
        assert_eq!(Solution::max_sub_array(nums), 1_000_000_000);
        let alternating: Vec<i32> = (0..100_000)
            .map(|i| if i % 2 == 0 { 3 } else { -2 })
            .collect();
        // 50 000 threes and the 49 999 minus-twos between them: 150 000 - 99 998.
        assert_eq!(Solution::max_sub_array(alternating), 50_002);
    }
}
