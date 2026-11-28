//! # Problem 0726: Binary Subarrays With Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** prefix sums + `HashMap` of counts ("subarray sum equals k"), `entry().or_insert`
//! - **Source:** [LeetCode 930. Binary Subarrays With Sum](https://leetcode.com/problems/binary-subarrays-with-sum/)
//!
//! ## Problem
//!
//! Given a binary vector `nums` (values `0` and `1`) and an integer `goal`, return the number of
//! non-empty **contiguous** subarrays whose sum is exactly `goal`.
//!
//! Implement `Solution::num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 0, 1, 0, 1], goal = 2
//! Output: 4             ([1, 0, 1] at 0..3, [1, 0, 1, 0] at 0..4, [0, 1, 0, 1] at 1..5, [1, 0, 1] at 2..5)
//!
//! Input:  nums = [0, 0, 0, 0, 0], goal = 0
//! Output: 15            (every one of the 5 * 6 / 2 subarrays)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 3 * 10^4`
//! - `nums[i]` is `0` or `1`.
//! - `0 <= goal <= nums.len()`
//!
//! ## Hints
//!
//! 1. The sum of `nums[a..b]` is `prefix(b) - prefix(a)`. For a fixed end `b`, how many starts `a`
//!    give exactly `goal`?
//! 2. Keep a count of how many times each prefix sum has appeared so far.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** prefix sums + hash map of counts: walking left to right, keep the running sum `s`
//! and a map `seen[v]` = how many earlier prefixes (including the empty prefix, sum `0`) had sum
//! `v`. A subarray ending here sums to `goal` exactly when it starts after a prefix with sum
//! `s - goal`, so add `seen[s - goal]` to the answer, then record `s`.
//!
//! This is the general "count subarrays whose sum equals k" technique, and it works for any
//! integers, including negative ones (unlike a sliding window, which needs non-negative values and
//! needs extra care when `goal == 0`).
//!
//! **Complexity:** `O(n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - `seen.get(&(sum - goal)).copied().unwrap_or(0)` reads a count that may be missing.
//! - `*seen.entry(sum).or_insert(0) += 1` is the usual counting idiom of the entry API.
//! - Here the prefix sums are small non-negative integers, so a `Vec` would also work; the
//!   `HashMap` version is the one that generalises.
// @solution-end

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        // @solution-start
        let mut seen: HashMap<i32, i32> = HashMap::from([(0, 1)]); // the empty prefix
        let (mut sum, mut count) = (0, 0);
        for &x in &nums {
            sum += x;
            count += seen.get(&(sum - goal)).copied().unwrap_or(0);
            *seen.entry(sum).or_insert(0) += 1;
        }
        count
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_goal_two() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
    }

    #[test]
    fn example_goal_zero() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }

    #[test]
    fn goal_too_large() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 1, 1], 4), 0);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1], 1), 1);
        assert_eq!(Solution::num_subarrays_with_sum(vec![1], 0), 0);
    }

    #[test]
    fn zeros_around_ones() {
        // Each choice of how many zeros to take on each side: 2 * 3 = 6 subarrays.
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 1, 1, 0, 0], 2), 6);
    }

    #[test]
    fn long_all_zero_input() {
        assert_eq!(
            Solution::num_subarrays_with_sum(vec![0; 30_000], 0),
            450_015_000
        );
    }
}
