// ==================================================================
// PRACTICE 0970: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0970_subarrays_with_k_different_integers.md
// Run the tests:          cargo test practice::p0970
// ==================================================================

//! # Problem 0970: Subarrays with K Different Integers
//!
//! - **Difficulty:** Hard
//! - **Topics:** "exactly k = at most k - at most (k - 1)", sliding window with a count array, counting subarrays by their right end
//! - **Source:** [LeetCode 992. Subarrays with K Different Integers](https://leetcode.com/problems/subarrays-with-k-different-integers/)
//!
//! ## Problem
//!
//! Given an integer array `nums` and an integer `k`, return the number of **good** subarrays: non-empty
//! contiguous subarrays that contain **exactly** `k` different integers.
//!
//! Implement `Solution::subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 1, 2, 3], k = 2
//! Output: 7     ([1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2])
//!
//! Input:  nums = [1, 2, 1, 3, 4], k = 3
//! Output: 3     ([1,2,1,3], [2,1,3], [1,3,4])
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 2 * 10^4`
//! - `1 <= nums[i], k <= nums.len()`
//!
//! ## Hints
//!
//! 1. "Exactly `k`" is awkward for a sliding window, but "at most `k`" is easy: for each right end,
//!    the valid left ends form a suffix `[left, right]`, contributing `right - left + 1` subarrays.
//! 2. `exactly(k) = at_most(k) - at_most(k - 1)`.
//! 3. Values are at most `n`, so a `Vec<usize>` of counts replaces a hash map.

pub struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn naive(nums: &[i32], k: usize) -> i32 {
        let mut total = 0;
        for i in 0..nums.len() {
            let mut seen = HashSet::new();
            for &x in &nums[i..] {
                seen.insert(x);
                if seen.len() == k {
                    total += 1;
                }
            }
        }
        total
    }

    #[test]
    fn examples() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
            7
        );
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
            3
        );
    }

    #[test]
    fn edge_cases() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1], 1), 1);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 1, 1], 1), 6);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2], 3), 0);
    }

    #[test]
    fn matches_naive() {
        let nums: Vec<i32> = (0..300).map(|i: i32| (i * 7_919) % 6 + 1).collect();
        for k in 1..=6 {
            assert_eq!(
                Solution::subarrays_with_k_distinct(nums.clone(), k as i32),
                naive(&nums, k),
                "k = {k}"
            );
        }
    }

    #[test]
    fn all_distinct_large() {
        let n = 20_000;
        let nums: Vec<i32> = (1..=n).collect();
        // Exactly k distinct means exactly length k: n - k + 1 subarrays.
        assert_eq!(Solution::subarrays_with_k_distinct(nums.clone(), 1), n);
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 500), n - 499);
    }
}
