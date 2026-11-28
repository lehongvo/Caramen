// ==================================================================
// PRACTICE 0052: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0052_left_and_right_sum_differences.md
// Run the tests:          cargo test practice::p0052
// ==================================================================

//! # Problem 0052: Left and Right Sum Differences
//!
//! - **Difficulty:** Easy
//! - **Topics:** prefix sums, total minus prefix, `abs`, building a `Vec`
//! - **Source:** [LeetCode 2574. Left and Right Sum Differences](https://leetcode.com/problems/left-and-right-sum-differences/)
//!
//! ## Problem
//!
//! Given a vector `nums` of length `n`, compute for every index `i`:
//!
//! - `left[i]`: the sum of the elements **before** index `i` (`0` for `i = 0`);
//! - `right[i]`: the sum of the elements **after** index `i` (`0` for `i = n - 1`).
//!
//! Return the vector `answer` where `answer[i] = |left[i] - right[i]|`.
//!
//! Implement `Solution::left_right_difference(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [10, 4, 8, 3]
//! Output: [15, 1, 11, 22]
//! Explanation: left = [0, 10, 14, 22], right = [15, 11, 3, 0].
//!
//! Input:  nums = [1]
//! Output: [0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 1000`
//! - `1 <= nums[i] <= 10^5`
//!
//! ## Hints
//!
//! 1. Recomputing both sums for every index is `O(n^2)`. Can you get `right[i]` from the total?
//! 2. `right[i] = total - left[i] - nums[i]`.

pub struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_elements() {
        assert_eq!(
            Solution::left_right_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
    }

    #[test]
    fn example_single_element() {
        assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
    }

    #[test]
    fn symmetric_input() {
        assert_eq!(
            Solution::left_right_difference(vec![1, 2, 3, 2, 1]),
            vec![8, 5, 0, 5, 8]
        );
    }

    #[test]
    fn largest_values() {
        let answer = Solution::left_right_difference(vec![100_000; 1000]);
        assert_eq!(answer[0], 99_900_000);
        assert_eq!(answer[999], 99_900_000);
        // Index 500 has 500 elements on the left and 499 on the right.
        assert_eq!(answer[500], 100_000);
    }
}
