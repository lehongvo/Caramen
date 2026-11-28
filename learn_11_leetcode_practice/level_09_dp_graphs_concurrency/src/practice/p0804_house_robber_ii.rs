// ==================================================================
// PRACTICE 0804: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0804_house_robber_ii.md
// Run the tests:          cargo test practice::p0804
// ==================================================================

//! # Problem 0804: House Robber II
//!
//! - **Difficulty:** Medium
//! - **Topics:** 1-D dynamic programming, sub-slices `&nums[1..]`, private helper on `&[i32]`
//! - **Source:** [LeetCode 213. House Robber II](https://leetcode.com/problems/house-robber-ii/)
//!
//! ## Problem
//!
//! Same rules as House Robber (problem 0803): `nums[i]` is the money in house `i`, and you may not
//! take from two adjacent houses. This time the houses stand in a **circle**, so the first house
//! and the last house are also neighbours. Return the largest total you can collect.
//!
//! Implement `Solution::rob(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [2, 3, 2]
//! Output: 3
//! Explanation: houses 0 and 2 are neighbours in the circle, so take house 1 only.
//!
//! Input:  nums = [1, 2, 3, 1]
//! Output: 4
//! Explanation: take houses 0 and 2.
//!
//! Input:  nums = [1, 2, 3]
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 100`
//! - `0 <= nums[i] <= 1000`
//!
//! ## Hints
//!
//! 1. In any valid choice, the first house or the last house (or both) is skipped.
//! 2. So the answer is the better of two straight-line problems: houses `1..n` and houses `0..n-1`.
//! 3. A single house is a special case: it is not its own neighbour.

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_houses() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn example_four_houses() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn example_increasing() {
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }

    #[test]
    fn single_and_two_houses() {
        assert_eq!(Solution::rob(vec![5]), 5);
        assert_eq!(Solution::rob(vec![4, 9]), 9);
    }

    #[test]
    fn big_value_next_to_the_first_house() {
        // Taking 100 (last) forbids house 0, which is fine: 3 + 100 = 103.
        assert_eq!(Solution::rob(vec![1, 3, 1, 3, 100]), 103);
    }
}
