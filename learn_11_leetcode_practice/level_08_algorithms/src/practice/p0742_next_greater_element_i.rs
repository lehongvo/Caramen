// ==================================================================
// PRACTICE 0742: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0742_next_greater_element_i.md
// Run the tests:          cargo test practice::p0742
// ==================================================================

//! # Problem 0742: Next Greater Element I
//!
//! - **Difficulty:** Easy
//! - **Topics:** monotonic decreasing stack, `HashMap` for lookups, `pop_if`
//! - **Source:** [LeetCode 496. Next Greater Element I](https://leetcode.com/problems/next-greater-element-i/)
//!
//! ## Problem
//!
//! The **next greater element** of a value `x` in a vector is the first value to the right of `x`
//! that is greater than `x`.
//!
//! You are given two vectors of **distinct** integers, `nums1` and `nums2`, where every value of
//! `nums1` also appears in `nums2`. For each `nums1[i]`, find its position in `nums2` and return
//! the next greater element **in `nums2`**, or `-1` if there is none. Return the answers in the
//! order of `nums1`.
//!
//! Implement `Solution::next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums1 = [4, 1, 2], nums2 = [1, 3, 4, 2]
//! Output: [-1, 3, -1]   (nothing after 4 is bigger; after 1 comes 3; nothing after 2)
//!
//! Input:  nums1 = [2, 4], nums2 = [1, 2, 3, 4]
//! Output: [3, -1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums1.len() <= nums2.len() <= 1000`
//! - `0 <= nums1[i], nums2[i] <= 10^4`
//! - All values within each vector are distinct, and every value of `nums1` is in `nums2`.
//!
//! ## Hints
//!
//! 1. Compute the next greater element of **every** value of `nums2` once, then answer `nums1`
//!    with lookups.
//! 2. Values still waiting for a greater element form a decreasing stack.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn example_increasing() {
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }

    #[test]
    fn single_value() {
        assert_eq!(Solution::next_greater_element(vec![7], vec![7]), vec![-1]);
    }

    #[test]
    fn answer_is_not_the_maximum() {
        // The next greater element of 1 is 5 (the first bigger value), not 9.
        assert_eq!(
            Solution::next_greater_element(vec![1, 5, 3], vec![1, 5, 3, 9]),
            vec![5, 9, 9]
        );
    }

    #[test]
    fn long_decreasing_then_peak() {
        let mut nums2: Vec<i32> = (0..999).rev().collect();
        nums2.push(5_000);
        let nums1: Vec<i32> = (0..999).step_by(10).collect();
        let expected = vec![5_000; nums1.len()];
        assert_eq!(Solution::next_greater_element(nums1, nums2), expected);
    }
}
