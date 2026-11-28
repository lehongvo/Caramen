// ==================================================================
// PRACTICE 0317: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0317_sort_array_by_increasing_frequency.md
// Run the tests:          cargo test practice::p0317
// ==================================================================

//! # Problem 0317: Sort Array by Increasing Frequency
//!
//! - **Difficulty:** Easy
//! - **Topics:** counting with a fixed-size array, `sort_by` with a two-level `Ordering`, `then_with`
//! - **Source:** [LeetCode 1636. Sort Array by Increasing Frequency](https://leetcode.com/problems/sort-array-by-increasing-frequency/)
//!
//! ## Problem
//!
//! Given a vector of integers `nums`, sort it so that values that occur **less often** come
//! first. When two different values occur the same number of times, the **larger** value comes
//! first. Every copy of a value stays in the output (the output has the same length as the input).
//!
//! Implement `Solution::frequency_sort(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 2, 2, 2, 3]
//! Output: [3, 1, 1, 2, 2, 2]
//! Explanation: 3 occurs once, 1 twice, 2 three times.
//!
//! Input:  nums = [2, 3, 1, 3, 2]
//! Output: [1, 3, 3, 2, 2]
//! Explanation: 2 and 3 both occur twice, so the larger value 3 goes first.
//!
//! Input:  nums = [-1, 1, -6, 4, 5, -6, 1, 4, 1]
//! Output: [5, -1, 4, 4, -6, -6, 1, 1, 1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 100`
//! - `-100 <= nums[i] <= 100`
//!
//! ## Hints
//!
//! 1. Values lie in `-100..=100`, so an array `[0; 201]` indexed by `value + 100` can count them.
//! 2. Compare two values by their counts first and, on a tie, compare them in reverse.

pub struct Solution;

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
    }

    #[test]
    fn example_two_tie_goes_to_larger() {
        assert_eq!(
            Solution::frequency_sort(vec![2, 3, 1, 3, 2]),
            vec![1, 3, 3, 2, 2]
        );
    }

    #[test]
    fn example_three_negatives() {
        assert_eq!(
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }

    #[test]
    fn extremes_and_single() {
        assert_eq!(Solution::frequency_sort(vec![7]), vec![7]);
        assert_eq!(
            Solution::frequency_sort(vec![-100, 100, -100]),
            vec![100, -100, -100]
        );
    }

    #[test]
    fn all_distinct_is_descending() {
        let nums: Vec<i32> = (-50..50).collect();
        let expected: Vec<i32> = (-50..50).rev().collect();
        assert_eq!(Solution::frequency_sort(nums), expected);
    }
}
