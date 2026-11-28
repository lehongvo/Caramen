// ==================================================================
// PRACTICE 0119: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0119_squares_of_a_sorted_array.md
// Run the tests:          cargo test practice::p0119
// ==================================================================

//! # Problem 0119: Squares of a Sorted Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** two indices moving inwards, filling an output from the back, `iter_mut().rev()`
//! - **Source:** [LeetCode 977. Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)
//!
//! ## Problem
//!
//! You are given a vector `nums` sorted in non-decreasing order (it may contain negative numbers).
//! Implement `Solution::sorted_squares(nums: Vec<i32>) -> Vec<i32>`, which returns the square of
//! every number, also sorted in non-decreasing order.
//!
//! Squaring and then sorting works in `O(n log n)`. Aim for `O(n)`: the largest square is always at
//! one of the two **ends** of `nums`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [-4, -1, 0, 3, 10]
//! Output: [0, 1, 9, 16, 100]
//!
//! Input:  nums = [-7, -3, 2, 3, 11]
//! Output: [4, 9, 9, 49, 121]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`
//! - `-10^4 <= nums[i] <= 10^4`
//! - `nums` is sorted in non-decreasing order.
//!
//! ## Hints
//!
//! 1. Keep two indices, `lo` at the start and `hi` at the end, and compare `nums[lo].abs()` with
//!    `nums[hi].abs()`.
//! 2. The larger one gives the next square **from the back** of the result; move that index inwards.

pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_with_zero() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn example_with_duplicates() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }

    #[test]
    fn all_negative() {
        assert_eq!(Solution::sorted_squares(vec![-5, -3, -1]), vec![1, 9, 25]);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::sorted_squares(vec![-10_000]), vec![100_000_000]);
    }

    #[test]
    fn large_input_matches_sorting() {
        let nums: Vec<i32> = (-5_000..5_000).collect();
        let mut expected: Vec<i32> = nums.iter().map(|x| x * x).collect();
        expected.sort();
        assert_eq!(Solution::sorted_squares(nums), expected);
    }
}
