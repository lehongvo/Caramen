// ==================================================================
// PRACTICE 0128: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0128_apply_operations_to_an_array.md
// Run the tests:          cargo test practice::p0128
// ==================================================================

//! # Problem 0128: Apply Operations to an Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** owned `mut` vector, sequential in-place updates, moving zeros with a write index
//! - **Source:** [LeetCode 2460. Apply Operations to an Array](https://leetcode.com/problems/apply-operations-to-an-array/)
//!
//! ## Problem
//!
//! You are given a vector `nums` of `n` non-negative integers. Perform these two steps and return
//! the resulting vector:
//!
//! 1. For `i = 0, 1, ..., n - 2`, **in this order**: if `nums[i] == nums[i + 1]`, double `nums[i]`
//!    and set `nums[i + 1]` to `0`. Each operation sees the changes made by the previous ones.
//! 2. Move all zeros to the end, keeping the order of the non-zero values.
//!
//! Implement `Solution::apply_operations(nums: Vec<i32>) -> Vec<i32>`, reusing the owned vector.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 2, 1, 1, 0]
//! Output: [1, 4, 2, 0, 0, 0]
//! Explanation: i = 1 gives [1, 4, 0, 1, 1, 0]; i = 3 gives [1, 4, 0, 2, 0, 0];
//!              i = 4 compares 0 with 0 (doubling 0 changes nothing). Then zeros move to the end.
//!
//! Input:  nums = [0, 1]
//! Output: [1, 0]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= nums.len() <= 2000`
//! - `0 <= nums[i] <= 1000`
//!
//! ## Hints
//!
//! 1. Step 1 is a plain index loop over `0..n - 1` that reads and writes `nums[i]` and `nums[i + 1]`.
//! 2. Step 2 is "Move Zeroes": a write index plus `swap`.

pub struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed() {
        assert_eq!(
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
    }

    #[test]
    fn example_two_elements() {
        assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
    }

    #[test]
    fn chain_of_equal_values_is_processed_in_order() {
        // [2,2,2,2] -> i=0: [4,0,2,2] -> i=2: [4,0,4,0]
        assert_eq!(
            Solution::apply_operations(vec![2, 2, 2, 2]),
            vec![4, 4, 0, 0]
        );
        // [3,3,3] -> i=0: [6,0,3] -> nothing else
        assert_eq!(Solution::apply_operations(vec![3, 3, 3]), vec![6, 3, 0]);
    }

    #[test]
    fn no_equal_neighbours() {
        assert_eq!(Solution::apply_operations(vec![5, 0, 7]), vec![5, 7, 0]);
    }

    #[test]
    fn largest_input() {
        let result = Solution::apply_operations(vec![1000; 2000]);
        assert!(result[..1000].iter().all(|&x| x == 2000));
        assert!(result[1000..].iter().all(|&x| x == 0));
    }
}
