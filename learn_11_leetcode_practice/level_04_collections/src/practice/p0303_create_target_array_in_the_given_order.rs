// ==================================================================
// PRACTICE 0303: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0303_create_target_array_in_the_given_order.md
// Run the tests:          cargo test practice::p0303
// ==================================================================

//! # Problem 0303: Create Target Array in the Given Order
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec::insert`, `zip`, `as usize` casts
//! - **Source:** [LeetCode 1389. Create Target Array in the Given Order](https://leetcode.com/problems/create-target-array-in-the-given-order/)
//!
//! ## Problem
//!
//! You receive two vectors of the same length, `nums` and `index`. Start with an empty vector
//! `target` and, for `i = 0, 1, 2, ...` in order, insert the value `nums[i]` at position
//! `index[i]` of `target`. Inserting at a position moves the element already there (and every
//! element after it) one place to the right. Return `target` after all insertions.
//!
//! Every insertion is guaranteed to be valid: at step `i`, `index[i]` is at most the current
//! length of `target`.
//!
//! Implement `Solution::create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [0, 1, 2, 3, 4], index = [0, 1, 2, 2, 1]
//! Output: [0, 4, 1, 3, 2]
//! Steps:  [0] -> [0, 1] -> [0, 1, 2] -> [0, 1, 3, 2] -> [0, 4, 1, 3, 2]
//!
//! Input:  nums = [1, 2, 3, 4, 0], index = [0, 1, 2, 3, 0]
//! Output: [0, 1, 2, 3, 4]
//!
//! Input:  nums = [1], index = [0]
//! Output: [1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() == index.len() <= 100`
//! - `0 <= nums[i] <= 100`
//! - `0 <= index[i] <= i`
//!
//! ## Hints
//!
//! 1. `Vec::insert(position, value)` does exactly what one step describes.
//! 2. Walk both vectors together with `nums.into_iter().zip(index)`.

pub struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::create_target_array(vec![1], vec![0]), vec![1]);
    }

    #[test]
    fn always_insert_at_front_reverses() {
        let nums: Vec<i32> = (0..100).collect();
        let expected: Vec<i32> = (0..100).rev().collect();
        assert_eq!(Solution::create_target_array(nums, vec![0; 100]), expected);
    }
}
