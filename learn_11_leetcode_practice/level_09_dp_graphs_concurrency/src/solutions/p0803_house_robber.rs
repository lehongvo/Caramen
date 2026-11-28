//! # Problem 0803: House Robber
//!
//! - **Difficulty:** Medium
//! - **Topics:** 1-D dynamic programming, take/skip recurrence, `fold`
//! - **Source:** [LeetCode 198. House Robber](https://leetcode.com/problems/house-robber/)
//!
//! ## Problem
//!
//! Houses stand in a row and `nums[i]` is the amount of money in house `i`. You may take the money
//! from any set of houses, **except** that you can never take from two adjacent houses (house `i`
//! and house `i + 1`). Return the largest total you can collect.
//!
//! Implement `Solution::rob(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3, 1]
//! Output: 4
//! Explanation: take houses 0 and 2: 1 + 3 = 4.
//!
//! Input:  nums = [2, 7, 9, 3, 1]
//! Output: 12
//! Explanation: take houses 0, 2 and 4: 2 + 9 + 1 = 12.
//!
//! Input:  nums = [2, 1, 1, 2]
//! Output: 4
//! Explanation: take houses 0 and 3; skipping two houses in a row is allowed.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 100`
//! - `0 <= nums[i] <= 400`
//!
//! ## Hints
//!
//! 1. For the first `i + 1` houses, either house `i` is skipped (best of the first `i` houses), or
//!    it is taken (its money plus the best of the first `i - 1` houses).
//! 2. Keep just two numbers: the best total up to the previous house and up to the one before it.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** let `best(i)` be the best total using houses `0..=i`. Then
//! `best(i) = max(best(i - 1), best(i - 2) + nums[i])`. Fold over the houses with the pair
//! `(best(i - 2), best(i - 1))`, starting from `(0, 0)`.
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - A two-value `fold` is the idiomatic way to write a rolling DP whose state is small.
//! - `i32::max` used as a method: `a.max(b)`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // @solution-start
        let (_, best) = nums
            .iter()
            .fold((0, 0), |(prev, cur), &money| (cur, cur.max(prev + money)));
        best
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_houses() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn example_five_houses() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn skipping_two_in_a_row() {
        assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    }

    #[test]
    fn single_house_and_zeros() {
        assert_eq!(Solution::rob(vec![5]), 5);
        assert_eq!(Solution::rob(vec![0, 0, 0]), 0);
    }

    #[test]
    fn hundred_equal_houses() {
        assert_eq!(Solution::rob(vec![400; 100]), 400 * 50);
    }
}
