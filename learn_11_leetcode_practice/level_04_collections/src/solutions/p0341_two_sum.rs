//! # Problem 0341: Two Sum
//!
//! - **Difficulty:** Easy
//! - **Topics:** `HashMap<i32, usize>` value -> index, `get` before `insert`, one pass
//! - **Source:** [LeetCode 1. Two Sum](https://leetcode.com/problems/two-sum/)
//!
//! ## Problem
//!
//! Given a vector of integers `nums` and an integer `target`, return the indices of the two
//! **different** positions whose values add up to `target`. Exactly one such pair exists. The two
//! indices may be returned in any order.
//!
//! Implement `Solution::two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 7, 11, 15], target = 9
//! Output: [0, 1]       (2 + 7 = 9)
//!
//! Input:  nums = [3, 2, 4], target = 6
//! Output: [1, 2]       (2 + 4; index 0 cannot be used twice)
//!
//! Input:  nums = [3, 3], target = 6
//! Output: [0, 1]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= nums.len() <= 10^4`
//! - `-10^9 <= nums[i], target <= 10^9`
//! - Exactly one valid answer exists.
//!
//! ## Hints
//!
//! 1. For the value `x` at index `i`, you are looking for an earlier index holding `target - x`.
//! 2. A `HashMap` from value to index answers "where did I see this value?" in `O(1)`.
//! 3. Look up before inserting the current value, so a value cannot pair with itself.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** walk the vector once. For each value `x` at index `i`, check whether the complement
//! `target - x` was seen before; if so, the answer is `[that index, i]`. Otherwise remember `x -> i`.
//! Looking up before inserting handles duplicates such as `[3, 3]` correctly.
//!
//! **Complexity:** `O(n)` average time, `O(n)` space.
//!
//! **Rust concepts:**
//! - `HashMap::with_capacity(n)` pre-allocates for `n` entries.
//! - `if let Some(&j) = seen.get(&(target - x))` destructures the `Option<&usize>` and copies the
//!   index out.
//! - `target - x` cannot overflow: both values are within `±10^9`, so the difference is within
//!   `±2 * 10^9`, inside `i32`'s range.
//! - The function must return something even though an answer always exists; returning an empty
//!   vector after the loop is a panic-free default.
// @solution-end

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // @solution-start
        let mut seen: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, &x) in nums.iter().enumerate() {
            if let Some(&j) = seen.get(&(target - x)) {
                return vec![j as i32, i as i32];
            }
            seen.insert(x, i);
        }
        Vec::new()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        assert_eq!(sorted(Solution::two_sum(vec![2, 7, 11, 15], 9)), vec![0, 1]);
    }

    #[test]
    fn example_two_no_self_pairing() {
        assert_eq!(sorted(Solution::two_sum(vec![3, 2, 4], 6)), vec![1, 2]);
    }

    #[test]
    fn example_three_duplicates() {
        assert_eq!(sorted(Solution::two_sum(vec![3, 3], 6)), vec![0, 1]);
    }

    #[test]
    fn negative_and_extreme_values() {
        assert_eq!(
            sorted(Solution::two_sum(vec![-1_000_000_000, 5, 1_000_000_000], 0)),
            vec![0, 2]
        );
        assert_eq!(sorted(Solution::two_sum(vec![-3, 4, 3, 90], 0)), vec![0, 2]);
    }

    #[test]
    fn large_input_pair_at_the_ends() {
        // Even numbers 2, 4, 6, ... except two odd values at the ends.
        let mut nums: Vec<i32> = (0..10_000).map(|i| i * 2).collect();
        nums[0] = 1;
        nums[9_999] = 3;
        // 1 + 3 is the only pair making 4: 0 + 4 is impossible (0 was replaced), 2 occurs once.
        assert_eq!(sorted(Solution::two_sum(nums, 4)), vec![0, 9_999]);
    }
}
