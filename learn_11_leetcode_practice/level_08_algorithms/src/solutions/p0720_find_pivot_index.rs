//! # Problem 0720: Find Pivot Index
//!
//! - **Difficulty:** Easy
//! - **Topics:** prefix sums, total minus prefix, `iter().sum()`, `enumerate`
//! - **Source:** [LeetCode 724. Find Pivot Index](https://leetcode.com/problems/find-pivot-index/)
//!
//! ## Problem
//!
//! Given a vector `nums`, a **pivot index** `i` is one where the sum of the elements strictly to
//! its left equals the sum of the elements strictly to its right. At the edges, the missing side
//! counts as `0` (so index `0` is a pivot when the sum of `nums[1..]` is `0`).
//!
//! Return the **leftmost** pivot index, or `-1` if there is none.
//!
//! Implement `Solution::pivot_index(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 7, 3, 6, 5, 6]
//! Output: 3             (1 + 7 + 3 = 11 = 5 + 6)
//!
//! Input:  nums = [1, 2, 3]
//! Output: -1
//!
//! Input:  nums = [2, 1, -1]
//! Output: 0             (left side is empty: 0 = 1 + (-1))
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`
//! - `-1000 <= nums[i] <= 1000`
//!
//! ## Hints
//!
//! 1. If you know the total sum and the sum to the left of `i`, you know the sum to the right.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** prefix sums: compute `total` once. Walk left to right keeping `left`, the sum of the
//! elements before `i`. The sum after `i` is `total - left - nums[i]`, so `i` is a pivot exactly
//! when `left == total - left - nums[i]`. The first index that satisfies it is the leftmost pivot.
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `nums.iter().sum::<i32>()` adds everything; the turbofish (or a type annotation on the
//!   variable) tells `sum` which type to produce.
//! - Returning from inside a `for` loop gives "find the first" without a flag variable.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        // @solution-start
        let total: i32 = nums.iter().sum();
        let mut left = 0;
        for (i, &x) in nums.iter().enumerate() {
            if left == total - left - x {
                return i as i32;
            }
            left += x;
        }
        -1
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_middle_pivot() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn example_no_pivot() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn example_first_index() {
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }

    #[test]
    fn single_element_is_a_pivot() {
        assert_eq!(Solution::pivot_index(vec![0]), 0);
        assert_eq!(Solution::pivot_index(vec![7]), 0);
    }

    #[test]
    fn last_index_and_leftmost_choice() {
        assert_eq!(Solution::pivot_index(vec![-1, -1, 0, 1, 1, 0]), 5);
        assert_eq!(Solution::pivot_index(vec![0, 0, 0]), 0);
    }

    #[test]
    fn long_symmetric_input() {
        let mut nums = vec![1000; 5000];
        nums.push(-3);
        nums.extend(vec![1000; 5000]);
        assert_eq!(Solution::pivot_index(nums), 5000);
    }
}
