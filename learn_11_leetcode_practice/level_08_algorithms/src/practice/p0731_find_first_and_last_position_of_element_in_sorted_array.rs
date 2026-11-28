// ==================================================================
// PRACTICE 0731: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0731_find_first_and_last_position_of_element_in_sorted_array.md
// Run the tests:          cargo test practice::p0731
// ==================================================================

//! # Problem 0731: Find First and Last Position of Element in Sorted Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** two boundary binary searches, `partition_point`, `slice::get`
//! - **Source:** [LeetCode 34. Find First and Last Position of Element in Sorted Array](https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/)
//!
//! ## Problem
//!
//! Given a vector `nums` sorted in non-decreasing order (duplicates allowed) and an integer
//! `target`, return `[first, last]`: the first and the last index where `target` appears. If
//! `target` does not appear, return `[-1, -1]`. The algorithm must run in `O(log n)` time.
//!
//! Implement `Solution::search_range(nums: Vec<i32>, target: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [5, 7, 7, 8, 8, 10], target = 8
//! Output: [3, 4]
//!
//! Input:  nums = [5, 7, 7, 8, 8, 10], target = 6
//! Output: [-1, -1]
//!
//! Input:  nums = [], target = 0
//! Output: [-1, -1]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= nums.len() <= 10^5`
//! - `-10^9 <= nums[i], target <= 10^9`
//! - `nums` is sorted in non-decreasing order.
//!
//! ## Hints
//!
//! 1. Finding one occurrence and then walking left and right is `O(n)` when everything is equal.
//! 2. Search for two boundaries instead: where "`x < target`" stops being true, and where
//!    "`x <= target`" stops being true.

pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_found() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }

    #[test]
    fn example_missing() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }

    #[test]
    fn example_empty() {
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
        assert_eq!(Solution::search_range(vec![1], 2), vec![-1, -1]);
    }

    #[test]
    fn target_at_the_edges() {
        assert_eq!(Solution::search_range(vec![2, 2, 3, 4, 4], 2), vec![0, 1]);
        assert_eq!(Solution::search_range(vec![2, 2, 3, 4, 4], 4), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![2, 2, 3, 4, 4], 5), vec![-1, -1]);
    }

    #[test]
    fn all_equal_long_input() {
        assert_eq!(Solution::search_range(vec![7; 100_000], 7), vec![0, 99_999]);
    }
}
