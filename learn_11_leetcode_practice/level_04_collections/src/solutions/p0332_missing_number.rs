//! # Problem 0332: Missing Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** building a `HashSet` with `collect`, `contains`, `find` over a range, the sum formula
//! - **Source:** [LeetCode 268. Missing Number](https://leetcode.com/problems/missing-number/)
//!
//! ## Problem
//!
//! The vector `nums` contains `n` **distinct** numbers taken from the range `0..=n` (which has
//! `n + 1` numbers). Exactly one number of that range is therefore missing. Return it.
//!
//! Implement `Solution::missing_number(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 0, 1]
//! Output: 2          (n = 3, the range is 0..=3)
//!
//! Input:  nums = [0, 1]
//! Output: 2          (n = 2, the range is 0..=2)
//!
//! Input:  nums = [9, 6, 4, 2, 3, 5, 7, 0, 1]
//! Output: 8
//! ```
//!
//! ## Constraints
//!
//! - `n == nums.len()`, `1 <= n <= 10^4`
//! - `0 <= nums[i] <= n`, all values distinct
//!
//! ## Hints
//!
//! 1. Put the numbers in a `HashSet`, then look for the first value of `0..=n` that is not in it.
//! 2. Follow-up: the numbers `0..=n` add up to `n * (n + 1) / 2`. What does that tell you?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** collect the numbers into a `HashSet` for `O(1)` membership tests, then scan the range
//! `0..=n` and return the first number that is not present.
//!
//! **Complexity:** `O(n)` average time, `O(n)` space.
//!
//! The constant-space alternative: `n * (n + 1) / 2 - nums.iter().sum::<i32>()` (it fits in `i32`
//! for `n <= 10^4`), or XOR all indices and values together.
//!
//! **Rust concepts:**
//! - `nums.iter().copied().collect::<HashSet<i32>>()` builds a set straight from an iterator.
//! - `contains(&x)` takes a reference to the value.
//! - `(0..=n).find(...)` returns an `Option<i32>`; exactly one number is missing, so `unwrap_or(n)`
//!   is only there to avoid an `unwrap`.
// @solution-end

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // @solution-start
        let n = nums.len() as i32;
        let present: HashSet<i32> = nums.iter().copied().collect();
        (0..=n).find(|x| !present.contains(x)).unwrap_or(n)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn missing_zero_and_single() {
        assert_eq!(Solution::missing_number(vec![1]), 0);
        assert_eq!(Solution::missing_number(vec![0]), 1);
        assert_eq!(Solution::missing_number(vec![2, 1]), 0);
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..=10_000).rev().filter(|&x| x != 4_321).collect();
        assert_eq!(nums.len(), 10_000);
        assert_eq!(Solution::missing_number(nums), 4_321);
    }
}
