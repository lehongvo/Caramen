// ==================================================================
// PRACTICE 0832: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0832_partition_to_k_equal_sum_subsets.md
// Run the tests:          cargo test practice::p0832
// ==================================================================

//! # Problem 0832: Partition to K Equal Sum Subsets
//!
//! - **Difficulty:** Medium
//! - **Topics:** bitmask DP over subsets, `1 << n`, `Option<i32>` table, bit tests with `&`
//! - **Source:** [LeetCode 698. Partition to K Equal Sum Subsets](https://leetcode.com/problems/partition-to-k-equal-sum-subsets/)
//!
//! ## Problem
//!
//! Given an integer array `nums` and an integer `k`, return `true` if the elements can be divided
//! into `k` non-empty groups (every element in exactly one group) that all have the same sum.
//!
//! Implement `Solution::can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [4, 3, 2, 3, 5, 2, 1], k = 4
//! Output: true
//! Explanation: (5), (1, 4), (2, 3), (2, 3): every group sums to 5.
//!
//! Input:  nums = [1, 2, 3, 4], k = 3
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= nums.len() <= 16`
//! - `1 <= nums[i] <= 10^4`
//! - Each value appears between 1 and 4 times.
//!
//! ## Hints
//!
//! 1. The common sum must be `target = total / k`; if `total % k != 0`, or some element is larger
//!    than `target`, the answer is `false`.
//! 2. Fill the groups one after another. The state is the **set** of elements already used,
//!    which fits in a 16-bit mask. Given the mask, the sum of the group being filled is
//!    `sum(mask) % target`.
//! 3. From a reachable mask, you may add an unused element if it still fits in the current group.

pub struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_groups() {
        assert!(Solution::can_partition_k_subsets(
            vec![4, 3, 2, 3, 5, 2, 1],
            4
        ));
    }

    #[test]
    fn example_impossible() {
        assert!(!Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3));
    }

    #[test]
    fn divisible_total_but_no_partition() {
        // Total 20 and target 5, but 4 + 1 is impossible without a 1.
        assert!(!Solution::can_partition_k_subsets(
            vec![2, 2, 2, 2, 3, 4, 5],
            4
        ));
    }

    #[test]
    fn k_equal_one_or_len() {
        assert!(Solution::can_partition_k_subsets(vec![5, 1, 9], 1));
        assert!(Solution::can_partition_k_subsets(vec![3, 3, 3], 3));
        assert!(!Solution::can_partition_k_subsets(vec![3, 3, 4], 3));
    }

    #[test]
    fn larger_inputs_up_to_sixteen_elements() {
        let nums = vec![10, 10, 10, 7, 7, 7, 7, 7, 7, 6, 6, 6];
        assert!(Solution::can_partition_k_subsets(nums, 3));
        let nums = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
        assert!(Solution::can_partition_k_subsets(nums, 8));
    }
}
