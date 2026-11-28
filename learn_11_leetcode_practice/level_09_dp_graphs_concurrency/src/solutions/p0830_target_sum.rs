//! # Problem 0830: Target Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** reducing a +/- assignment to subset counting, 0/1 knapsack (counting), `checked`
//!   arithmetic for impossible targets
//! - **Source:** [LeetCode 494. Target Sum](https://leetcode.com/problems/target-sum/)
//!
//! ## Problem
//!
//! You are given an array of non-negative integers `nums` and an integer `target`. Put a `+` or a
//! `-` sign in front of **every** element and evaluate the resulting expression. Return the number
//! of different sign assignments whose value equals `target`.
//!
//! Implement `Solution::find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 1, 1, 1], target = 3
//! Output: 5
//! Explanation: -1+1+1+1+1, +1-1+1+1+1, +1+1-1+1+1, +1+1+1-1+1, +1+1+1+1-1.
//!
//! Input:  nums = [1], target = 1
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 20`
//! - `0 <= nums[i] <= 1000`
//! - `0 <= sum(nums) <= 1000`
//! - `-1000 <= target <= 1000`
//!
//! ## Hints
//!
//! 1. Let `P` be the sum of the elements with `+` and `N` the sum of those with `-`. Then
//!    `P - N = target` and `P + N = total`.
//! 2. So `P = (total + target) / 2`: count the subsets with that sum. If `total + target` is
//!    negative or odd, there is no way at all.
//! 3. Zeros are fine: a `0` can take either sign, and the counting DP doubles the count naturally.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** compute `P = (total + target) / 2` when it is a non-negative even split, then run the
//! 0/1 counting knapsack: `ways[0] = 1`, and for each element `x`, for `s` from `P` down to `x`,
//! `ways[s] += ways[s - x]`. The answer is `ways[P]`.
//!
//! **Complexity:** `O(n * total)` time, `O(total)` space (versus `O(2^n)` for trying all signs).
//!
//! **Rust concepts:**
//! - `usize::try_from(total + target)` rejects a negative sum without a separate `if`.
//! - The backward loop over `s` makes every element contribute at most once per subset; with
//!   `x = 0` the update `ways[s] += ways[s]` doubles the count, matching the two sign choices.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // @solution-start
        let total: i32 = nums.iter().sum();
        let Ok(doubled) = usize::try_from(total + target) else {
            return 0; // target < -total
        };
        if doubled % 2 != 0 || doubled / 2 > total as usize {
            return 0;
        }
        let goal = doubled / 2;
        let mut ways = vec![0_i32; goal + 1];
        ways[0] = 1;
        for x in nums.into_iter().map(|x| x as usize) {
            for s in (x..=goal).rev() {
                ways[s] += ways[s - x];
            }
        }
        ways[goal]
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_five_ones() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn example_single() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }

    #[test]
    fn unreachable_targets() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 2), 0);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 2], -4), 0);
        assert_eq!(Solution::find_target_sum_ways(vec![1, 2], 1000), 0);
    }

    #[test]
    fn zeros_double_the_count() {
        let nums = vec![0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(Solution::find_target_sum_ways(nums, 1), 256);
    }

    #[test]
    fn negative_target() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 2, 3], -6), 1);
    }

    #[test]
    fn mixed_values() {
        let nums = vec![2, 7, 4, 1, 8, 1, 9, 3, 5, 6, 2, 4];
        assert_eq!(Solution::find_target_sum_ways(nums, 10), 156);
    }

    #[test]
    fn twenty_zeros() {
        assert_eq!(Solution::find_target_sum_ways(vec![0; 20], 0), 1 << 20);
    }
}
