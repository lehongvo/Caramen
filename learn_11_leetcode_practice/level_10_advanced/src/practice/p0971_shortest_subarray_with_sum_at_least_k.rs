// ==================================================================
// PRACTICE 0971: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0971_shortest_subarray_with_sum_at_least_k.md
// Run the tests:          cargo test practice::p0971
// ==================================================================

//! # Problem 0971: Shortest Subarray with Sum at Least K
//!
//! - **Difficulty:** Hard
//! - **Topics:** prefix sums in `i64`, monotonic deque of prefix indices, why a plain sliding window fails with negative numbers
//! - **Source:** [LeetCode 862. Shortest Subarray with Sum at Least K](https://leetcode.com/problems/shortest-subarray-with-sum-at-least-k/)
//!
//! ## Problem
//!
//! Given an integer array `nums` (which may contain **negative** numbers) and an integer `k`, return
//! the length of the shortest non-empty contiguous subarray whose sum is at least `k`, or `-1` if
//! there is none.
//!
//! Implement `Solution::shortest_subarray(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1], k = 1          Output: 1
//! Input:  nums = [1, 2], k = 4       Output: -1
//! Input:  nums = [2, -1, 2], k = 3   Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-10^5 <= nums[i] <= 10^5`
//! - `1 <= k <= 10^9`
//!
//! ## Hints
//!
//! 1. With prefix sums `P`, we want the smallest `j - i` with `P[j] - P[i] >= k` and `i < j`.
//! 2. Keep a deque of indices with **increasing** prefix sums. For a new `j`: while `P[j] - P[front] >= k`,
//!    `front` is answered (no later `j` can do better for it), so record and pop it.
//! 3. Then pop from the back every index whose prefix sum is `>= P[j]`: `j` is a better start (later
//!    and not larger).

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(nums: &[i32], k: i32) -> i32 {
        let mut best = i32::MAX;
        for i in 0..nums.len() {
            let mut sum = 0_i64;
            for (j, &x) in nums.iter().enumerate().skip(i) {
                sum += i64::from(x);
                if sum >= i64::from(k) {
                    best = best.min((j - i + 1) as i32);
                    break;
                }
            }
        }
        if best == i32::MAX { -1 } else { best }
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
        assert_eq!(Solution::shortest_subarray(vec![1, 2], 4), -1);
        assert_eq!(Solution::shortest_subarray(vec![2, -1, 2], 3), 3);
    }

    #[test]
    fn negatives_in_the_middle() {
        assert_eq!(
            Solution::shortest_subarray(vec![84, -37, 32, 40, 95], 167),
            3
        );
        assert_eq!(
            Solution::shortest_subarray(vec![-28, 81, -20, 28, -29], 89),
            3
        );
        assert_eq!(Solution::shortest_subarray(vec![-5, -5], 1), -1);
    }

    #[test]
    fn matches_naive() {
        let nums: Vec<i32> = (0..400).map(|i: i32| (i * 7_919) % 41 - 15).collect();
        for k in [1, 10, 50, 120, 400, 10_000] {
            assert_eq!(
                Solution::shortest_subarray(nums.clone(), k),
                naive(&nums, k),
                "k = {k}"
            );
        }
    }

    #[test]
    fn large_values() {
        let nums = vec![100_000; 100_000];
        assert_eq!(Solution::shortest_subarray(nums, 1_000_000_000), 10_000);
    }
}
