// ==================================================================
// PRACTICE 0805: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0805_delete_and_earn.md
// Run the tests:          cargo test practice::p0805
// ==================================================================

//! # Problem 0805: Delete and Earn
//!
//! - **Difficulty:** Medium
//! - **Topics:** dynamic programming, reduction to House Robber, value buckets in a `Vec`
//! - **Source:** [LeetCode 740. Delete and Earn](https://leetcode.com/problems/delete-and-earn/)
//!
//! ## Problem
//!
//! You are given an array `nums`. You may repeat the following operation as often as you like:
//! pick any element `x`, earn `x` points and delete it; then **every** element equal to `x - 1` or
//! `x + 1` is deleted as well (without earning anything for them). Return the maximum number of
//! points you can earn.
//!
//! Implement `Solution::delete_and_earn(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 4, 2]
//! Output: 6
//! Explanation: take 4 (deletes 3), then take 2. Total 4 + 2 = 6.
//!
//! Input:  nums = [2, 2, 3, 3, 3, 4]
//! Output: 9
//! Explanation: take a 3 (deletes both 2s and the 4), then the two remaining 3s: 3 + 3 + 3 = 9.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 2 * 10^4`
//! - `1 <= nums[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. If you take one copy of `x`, you might as well take all copies of `x`: they cost nothing more.
//! 2. Add up the points per value: `points[v] = v * (number of copies of v)`.
//! 3. Now "taking value `v` forbids values `v - 1` and `v + 1`" is exactly House Robber over
//!    `points`.

pub struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_values() {
        assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    }

    #[test]
    fn example_with_duplicates() {
        assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::delete_and_earn(vec![1]), 1);
    }

    #[test]
    fn gaps_between_values() {
        // Values 1 (x3), 2, 4, 5 (x3), 6: best is all 1s plus all 5s = 3 + 15.
        assert_eq!(
            Solution::delete_and_earn(vec![1, 1, 1, 2, 4, 5, 5, 5, 6]),
            18
        );
    }

    #[test]
    fn large_input() {
        // 20000 copies of 10000: take them all.
        assert_eq!(Solution::delete_and_earn(vec![10_000; 20_000]), 200_000_000);
    }
}
