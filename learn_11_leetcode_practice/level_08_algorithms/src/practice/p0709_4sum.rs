// ==================================================================
// PRACTICE 0709: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0709_4sum.md
// Run the tests:          cargo test practice::p0709
// ==================================================================

//! # Problem 0709: 4Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** sorting + nested two pointers, skipping duplicates, `i64` to avoid overflow
//! - **Source:** [LeetCode 18. 4Sum](https://leetcode.com/problems/4sum/)
//!
//! ## Problem
//!
//! Given a vector `nums` and an integer `target`, return every **distinct** quadruplet
//! `[a, b, c, d]` of values taken from four different positions such that
//! `a + b + c + d == target`.
//!
//! Quadruplets with the same values (in any order) count once. The quadruplets, and the values
//! inside each one, may be returned in any order.
//!
//! Implement `Solution::four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 0, -1, 0, -2, 2], target = 0
//! Output: [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
//!
//! Input:  nums = [2, 2, 2, 2, 2], target = 8
//! Output: [[2, 2, 2, 2]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 200`
//! - `-10^9 <= nums[i] <= 10^9`
//! - `-10^9 <= target <= 10^9`
//!
//! ## Hints
//!
//! 1. Fix the first **two** values with two nested loops, then run the two-pointer scan from 3Sum.
//! 2. Four values of `10^9` add up to `4 * 10^9`, which does not fit in `i32`.

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized(mut quads: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for q in &mut quads {
            q.sort_unstable();
        }
        quads.sort();
        quads
    }

    #[test]
    fn example_three_quadruplets() {
        assert_eq!(
            normalized(Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
    }

    #[test]
    fn example_all_equal() {
        assert_eq!(
            normalized(Solution::four_sum(vec![2, 2, 2, 2, 2], 8)),
            vec![vec![2, 2, 2, 2]]
        );
    }

    #[test]
    fn fewer_than_four_values() {
        assert!(Solution::four_sum(vec![1, 2, 3], 6).is_empty());
    }

    #[test]
    fn huge_values_do_not_overflow() {
        // In i32 the sum 4 * 10^9 would wrap around to exactly -294_967_296.
        let nums = vec![1_000_000_000; 4];
        assert!(Solution::four_sum(nums, -294_967_296).is_empty());
    }

    #[test]
    fn huge_values_matching_target() {
        let nums = vec![
            1_000_000_000,
            1_000_000_000,
            -1_000_000_000,
            -1_000_000_000,
            0,
        ];
        assert_eq!(
            normalized(Solution::four_sum(nums, 0)),
            vec![vec![
                -1_000_000_000,
                -1_000_000_000,
                1_000_000_000,
                1_000_000_000
            ]]
        );
    }
}
