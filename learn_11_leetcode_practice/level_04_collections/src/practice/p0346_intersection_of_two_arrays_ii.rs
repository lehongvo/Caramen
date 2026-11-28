// ==================================================================
// PRACTICE 0346: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0346_intersection_of_two_arrays_ii.md
// Run the tests:          cargo test practice::p0346
// ==================================================================

//! # Problem 0346: Intersection of Two Arrays II
//!
//! - **Difficulty:** Easy
//! - **Topics:** count map, decrementing through `get_mut`, a match guard on `Option<&mut usize>`
//! - **Source:** [LeetCode 350. Intersection of Two Arrays II](https://leetcode.com/problems/intersection-of-two-arrays-ii/)
//!
//! ## Problem
//!
//! Given two vectors `nums1` and `nums2`, return their intersection **with multiplicity**: a value
//! appears in the result as many times as it appears in **both** inputs (that is, the minimum of
//! its two counts). The result may be in any order.
//!
//! Implement `Solution::intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums1 = [1, 2, 2, 1], nums2 = [2, 2]
//! Output: [2, 2]
//!
//! Input:  nums1 = [4, 9, 5], nums2 = [9, 4, 9, 8, 4]
//! Output: [4, 9]       ([9, 4] is also accepted)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums1.len(), nums2.len() <= 1000`
//! - `0 <= nums1[i], nums2[i] <= 1000`
//!
//! ## Hints
//!
//! 1. Count the values of one vector in a `HashMap`.
//! 2. Walk the other vector; each value with a positive remaining count goes into the result and
//!    uses up one unit of that count.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
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
        assert_eq!(
            sorted(Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])),
            vec![2, 2]
        );
    }

    #[test]
    fn example_two_any_order() {
        assert_eq!(
            sorted(Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4])),
            vec![4, 9]
        );
    }

    #[test]
    fn multiplicity_is_the_minimum() {
        assert_eq!(
            sorted(Solution::intersect(vec![1, 1, 1, 2], vec![1, 1, 2, 2, 2])),
            vec![1, 1, 2]
        );
        assert_eq!(Solution::intersect(vec![3], vec![4]), Vec::<i32>::new());
    }

    #[test]
    fn large_inputs() {
        let nums1: Vec<i32> = (0..1000).map(|i| i % 10).collect();
        let nums2: Vec<i32> = (0..1000).map(|i| i % 20).collect();
        let result = sorted(Solution::intersect(nums1, nums2));
        // Values 0..10 occur 100 times in nums1 and 50 times in nums2.
        assert_eq!(result.len(), 500);
        assert_eq!(result.iter().filter(|&&x| x == 7).count(), 50);
    }
}
