//! # Problem 0319: Search Insert Position
//!
//! - **Difficulty:** Easy
//! - **Topics:** `slice::binary_search`, `Result<usize, usize>`, or-patterns in `match`
//! - **Source:** [LeetCode 35. Search Insert Position](https://leetcode.com/problems/search-insert-position/)
//!
//! ## Problem
//!
//! You are given a vector `nums` of **distinct** integers sorted in ascending order, and an
//! integer `target`. If `target` is in `nums`, return its index. Otherwise return the index where
//! it would have to be inserted to keep `nums` sorted. The algorithm must run in `O(log n)` time.
//!
//! Implement `Solution::search_insert(nums: Vec<i32>, target: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 3, 5, 6], target = 5
//! Output: 2      (found at index 2)
//!
//! Input:  nums = [1, 3, 5, 6], target = 2
//! Output: 1      (2 belongs between 1 and 3)
//!
//! Input:  nums = [1, 3, 5, 6], target = 7
//! Output: 4      (after the last element)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`
//! - `-10^4 <= nums[i], target <= 10^4`
//! - `nums` is sorted in ascending order and has no duplicates.
//!
//! ## Hints
//!
//! 1. The standard library already implements binary search on sorted slices.
//! 2. `binary_search` returns `Ok(index)` when the value is found and `Err(index)` with the
//!    insertion point when it is not.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `binary_search` answers both questions at once: `Ok(i)` is "found at `i`" and
//! `Err(i)` is "not found; inserting at `i` keeps the order". Here both cases return `i`.
//!
//! **Complexity:** `O(log n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `binary_search` returns `Result<usize, usize>`: an `Err` here is not a failure but extra
//!   information (the insertion point).
//! - The or-pattern `Ok(i) | Err(i)` binds `i` in both alternatives. An equivalent one-liner is
//!   `nums.binary_search(&target).unwrap_or_else(|i| i)`.
//! - With duplicates, `binary_search` may return any of the matching indices; use
//!   `partition_point` when you need the first or the last one (see the next problem).
// @solution-end

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // @solution-start
        match nums.binary_search(&target) {
            Ok(i) | Err(i) => i as i32,
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_found() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn example_insert_in_middle() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn example_insert_at_end() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn insert_at_front_and_single_element() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1], 1), 0);
        assert_eq!(Solution::search_insert(vec![1], 2), 1);
    }

    #[test]
    fn large_odd_numbers() {
        let nums: Vec<i32> = (0..10_000).map(|i| -9_999 + 2 * i).collect();
        assert_eq!(Solution::search_insert(nums.clone(), -9_999), 0);
        assert_eq!(Solution::search_insert(nums.clone(), 0), 5_000);
        assert_eq!(Solution::search_insert(nums, 10_000), 10_000);
    }
}
