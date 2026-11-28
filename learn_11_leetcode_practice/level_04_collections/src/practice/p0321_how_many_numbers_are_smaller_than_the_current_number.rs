// ==================================================================
// PRACTICE 0321: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0321_how_many_numbers_are_smaller_than_the_current_number.md
// Run the tests:          cargo test practice::p0321
// ==================================================================

//! # Problem 0321: How Many Numbers Are Smaller Than the Current Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** sorted copy with `clone` + `sort_unstable`, `partition_point`, `map` + `collect`
//! - **Source:** [LeetCode 1365. How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/)
//!
//! ## Problem
//!
//! Given a vector `nums`, build a vector `answer` of the same length where `answer[i]` is the
//! number of elements of `nums` that are **strictly smaller** than `nums[i]`.
//!
//! Implement `Solution::smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [8, 1, 2, 2, 3]
//! Output: [4, 0, 1, 1, 3]
//! Explanation: four numbers are smaller than 8; none is smaller than 1; only 1 is smaller
//!              than 2 (twice); 1, 2, 2 are smaller than 3.
//!
//! Input:  nums = [6, 5, 4, 8]
//! Output: [2, 1, 0, 3]
//!
//! Input:  nums = [7, 7, 7, 7]
//! Output: [0, 0, 0, 0]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= nums.len() <= 500`
//! - `0 <= nums[i] <= 100`
//!
//! ## Hints
//!
//! 1. In a **sorted** copy, the number of elements smaller than `x` is the index of the first
//!    element that is `>= x`.
//! 2. `sorted.partition_point(|&y| y < x)` returns exactly that index.

pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
    }

    #[test]
    fn example_three_all_equal() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }

    #[test]
    fn extremes() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![100, 0]),
            vec![1, 0]
        );
    }

    #[test]
    fn large_descending() {
        let nums: Vec<i32> = (0..500).rev().map(|i| i / 5).collect();
        let result = Solution::smaller_numbers_than_current(nums);
        assert_eq!(result[0], 495);
        assert_eq!(result[499], 0);
    }
}
