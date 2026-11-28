// ==================================================================
// PRACTICE 0124: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0124_remove_element.md
// Run the tests:          cargo test practice::p0124
// ==================================================================

//! # Problem 0124: Remove Element
//!
//! - **Difficulty:** Easy
//! - **Topics:** in-place compaction through `&mut Vec<i32>`, read/write indices, returning a length
//! - **Source:** [LeetCode 27. Remove Element](https://leetcode.com/problems/remove-element/)
//!
//! ## Problem
//!
//! Given a vector `nums` and a value `val`, remove every occurrence of `val` **in place** and return
//! `k`, the number of elements that are not equal to `val`.
//!
//! After the call, the first `k` elements of `nums` must be exactly the elements different from
//! `val` (in any order). What is stored at positions `k` and later does not matter.
//!
//! Implement `Solution::remove_element(nums: &mut Vec<i32>, val: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 2, 2, 3], val = 3
//! Output: 2, nums starts with [2, 2]
//!
//! Input:  nums = [0, 1, 2, 2, 3, 0, 4, 2], val = 2
//! Output: 5, nums starts with [0, 1, 3, 0, 4] (any order of these five values is accepted)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= nums.len() <= 100`
//! - `0 <= nums[i] <= 50`
//! - `0 <= val <= 100`
//!
//! ## Hints
//!
//! 1. Keep a `write` index. Every element that should stay is copied to `nums[write]`.
//! 2. `i32` is `Copy`, so `nums[write] = nums[read]` simply copies the value.

pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the function and returns `k` and the sorted first `k` elements.
    fn run(mut nums: Vec<i32>, val: i32) -> (i32, Vec<i32>) {
        let k = Solution::remove_element(&mut nums, val);
        let mut kept = nums[..k as usize].to_vec();
        kept.sort();
        (k, kept)
    }

    #[test]
    fn example_remove_threes() {
        assert_eq!(run(vec![3, 2, 2, 3], 3), (2, vec![2, 2]));
    }

    #[test]
    fn example_remove_twos() {
        assert_eq!(
            run(vec![0, 1, 2, 2, 3, 0, 4, 2], 2),
            (5, vec![0, 0, 1, 3, 4])
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(run(vec![], 1), (0, vec![]));
    }

    #[test]
    fn value_not_present_and_all_removed() {
        assert_eq!(run(vec![1, 2, 3], 9), (3, vec![1, 2, 3]));
        assert_eq!(run(vec![7, 7, 7], 7), (0, vec![]));
    }

    #[test]
    fn largest_input() {
        let nums: Vec<i32> = (0..100).map(|i| i % 5).collect();
        let (k, kept) = run(nums, 0);
        assert_eq!(k, 80);
        assert!(kept.iter().all(|&x| x != 0));
    }
}
