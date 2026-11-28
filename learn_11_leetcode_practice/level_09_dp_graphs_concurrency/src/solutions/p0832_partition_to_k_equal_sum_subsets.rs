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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `fill[mask]` is `Some(r)` if the elements in `mask` can be arranged into complete
//! groups followed by a partial group of sum `r < target`, else `None`. Start with
//! `fill[0] = Some(0)`. Masks only grow, so visiting them in increasing numeric order processes
//! every mask after all of its subsets. From a reachable mask, add any unused `nums[i]` with
//! `r + nums[i] <= target`; the new remainder is `(r + nums[i]) % target`. The answer is whether
//! the full mask is reachable.
//!
//! **Complexity:** `O(2^n * n)` time, `O(2^n)` space.
//!
//! **Rust concepts:**
//! - `mask & (1 << i) == 0` tests whether element `i` is unused; `mask | (1 << i)` adds it.
//! - `let Some(r) = fill[mask] else { continue };` skips unreachable states in one line.
//! - For a given mask the remainder is the same whatever order was used, so storing a single
//!   value per mask is enough.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        // @solution-start
        let total: i32 = nums.iter().sum();
        if total % k != 0 {
            return false;
        }
        let target = total / k;
        if nums.iter().any(|&x| x > target) {
            return false;
        }
        let n = nums.len();
        let full = (1_usize << n) - 1;
        let mut fill: Vec<Option<i32>> = vec![None; full + 1];
        fill[0] = Some(0);
        for mask in 0..=full {
            let Some(r) = fill[mask] else { continue };
            for (i, &x) in nums.iter().enumerate() {
                let next = mask | (1 << i);
                if mask & (1 << i) == 0 && r + x <= target && fill[next].is_none() {
                    fill[next] = Some((r + x) % target);
                }
            }
        }
        fill[full].is_some()
        // @solution-end
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
