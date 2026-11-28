// ==================================================================
// PRACTICE 0952: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0952_longest_increasing_subsequence_ii.md
// Run the tests:          cargo test practice::p0952
// ==================================================================

//! # Problem 0952: Longest Increasing Subsequence II
//!
//! - **Difficulty:** Hard
//! - **Topics:** DP over values, iterative (bottom-up) segment tree for range maximum, point updates, half-open query ranges
//! - **Source:** [LeetCode 2407. Longest Increasing Subsequence II](https://leetcode.com/problems/longest-increasing-subsequence-ii/)
//!
//! ## Problem
//!
//! Given an integer array `nums` and an integer `k`, find the length of the longest subsequence of
//! `nums` that is **strictly increasing** and in which every two adjacent elements differ by **at
//! most** `k`.
//!
//! Implement `Solution::length_of_lis(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [4, 2, 1, 4, 3, 4, 5, 8, 15], k = 3
//! Output: 5        ([1, 3, 4, 5, 8]: each step is at most 3)
//!
//! Input:  nums = [7, 4, 5, 1, 8, 12, 4, 7], k = 5
//! Output: 4        ([4, 5, 8, 12])
//!
//! Input:  nums = [1, 5], k = 1
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `1 <= nums[i], k <= 10^5`
//!
//! ## Hints
//!
//! 1. Let `best[v]` be the longest valid subsequence (seen so far) that ends with the value `v`.
//! 2. Processing `x`, the previous element must be in `[x - k, x - 1]`:
//!    `best[x] = 1 + max(best[x - k ..= x - 1])`.
//! 3. A segment tree indexed by value answers that range maximum and the update in `O(log V)`.

pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(nums: &[i32], k: i32) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] && nums[i] - nums[j] <= k {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp.into_iter().max().unwrap_or(0)
    }

    #[test]
    fn examples() {
        assert_eq!(
            Solution::length_of_lis(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3),
            5
        );
        assert_eq!(Solution::length_of_lis(vec![7, 4, 5, 1, 8, 12, 4, 7], 5), 4);
        assert_eq!(Solution::length_of_lis(vec![1, 5], 1), 1);
    }

    #[test]
    fn equal_values_do_not_chain() {
        assert_eq!(Solution::length_of_lis(vec![3, 3, 3], 5), 1);
        assert_eq!(Solution::length_of_lis(vec![1, 2, 2, 3], 1), 3);
    }

    #[test]
    fn matches_naive() {
        let nums: Vec<i32> = (0..400).map(|i: i32| (i * 7_919) % 97 + 1).collect();
        for k in [1, 2, 5, 30, 100] {
            assert_eq!(
                Solution::length_of_lis(nums.clone(), k),
                naive(&nums, k),
                "k = {k}"
            );
        }
    }

    #[test]
    fn large_increasing_input() {
        let nums: Vec<i32> = (1..=100_000).collect();
        assert_eq!(Solution::length_of_lis(nums.clone(), 1), 100_000);
        let evens: Vec<i32> = (1..=50_000).map(|i| 2 * i).collect();
        assert_eq!(Solution::length_of_lis(evens.clone(), 1), 1);
        assert_eq!(Solution::length_of_lis(evens, 2), 50_000);
    }
}
