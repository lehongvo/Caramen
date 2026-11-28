// ==================================================================
// PRACTICE 0041: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0041_concatenation_of_array.md
// Run the tests:          cargo test practice::p0041
// ==================================================================

//! # Problem 0041: Concatenation of Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** building a `Vec`, `Vec::with_capacity`, `extend_from_slice`, borrowing a `Vec` as a slice
//! - **Source:** [LeetCode 1929. Concatenation of Array](https://leetcode.com/problems/concatenation-of-array/)
//!
//! ## Problem
//!
//! Given a vector `nums` of length `n`, return a vector `ans` of length `2n` such that
//! `ans[i] == nums[i]` and `ans[i + n] == nums[i]` for every `0 <= i < n`. In other words, `ans` is
//! `nums` followed by another copy of `nums`.
//!
//! Implement `Solution::get_concatenation(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 1]
//! Output: [1, 2, 1, 1, 2, 1]
//!
//! Input:  nums = [1, 3, 2, 1]
//! Output: [1, 3, 2, 1, 1, 3, 2, 1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 1000`
//! - `1 <= nums[i] <= 1000`
//!
//! ## Hints
//!
//! 1. Create an empty vector with room for `2 * n` elements, then append `nums` twice.
//! 2. `v.extend_from_slice(&nums)` appends a copy of every element of `nums` to `v`.

pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_elements() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        );
    }

    #[test]
    fn example_four_elements() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::get_concatenation(vec![7]), vec![7, 7]);
    }

    #[test]
    fn largest_input_keeps_the_rule() {
        let nums: Vec<i32> = (1..=1000).collect();
        let ans = Solution::get_concatenation(nums.clone());
        assert_eq!(ans.len(), 2000);
        for i in 0..1000 {
            assert_eq!(ans[i], nums[i]);
            assert_eq!(ans[i + 1000], nums[i]);
        }
    }
}
