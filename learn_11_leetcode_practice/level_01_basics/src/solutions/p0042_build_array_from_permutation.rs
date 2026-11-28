//! # Problem 0042: Build Array from Permutation
//!
//! - **Difficulty:** Easy
//! - **Topics:** using values as indices, `as usize`, iterating a `Vec` by reference, `push`
//! - **Source:** [LeetCode 1920. Build Array from Permutation](https://leetcode.com/problems/build-array-from-permutation/)
//!
//! ## Problem
//!
//! You are given a vector `nums` that is a **zero-based permutation**: it contains every integer
//! from `0` to `nums.len() - 1` exactly once, in some order. Build and return a vector `ans` of the
//! same length where `ans[i] = nums[nums[i]]`.
//!
//! Implement `Solution::build_array(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [0, 2, 1, 5, 3, 4]
//! Output: [0, 1, 2, 4, 5, 3]
//! Explanation: ans[1] = nums[nums[1]] = nums[2] = 1, ans[3] = nums[5] = 4, ...
//!
//! Input:  nums = [5, 0, 1, 2, 3, 4]
//! Output: [4, 5, 0, 1, 2, 3]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 1000`
//! - `0 <= nums[i] < nums.len()`
//! - The values in `nums` are distinct.
//!
//! ## Hints
//!
//! 1. Vectors are indexed with `usize`, not `i32`. Convert with `x as usize` (safe here because the
//!    values are never negative).
//! 2. You need to **read** `nums` while building a **new** vector, so do not modify `nums` in place.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for each value `x` in `nums` (in order), push `nums[x]` onto a new vector.
//!
//! **Complexity:** `O(n)` time, `O(n)` space for the output.
//!
//! **Rust concepts:**
//! - Indices must be `usize`, the pointer-sized unsigned type. Rust does not convert `i32` to
//!   `usize` automatically, so the cast `x as usize` is required. A negative `i32` would become a
//!   huge `usize` and the indexing would panic, which the constraints rule out.
//! - `for &x in &nums` borrows the vector immutably, and the body can still index `nums`, because
//!   any number of shared borrows may exist at the same time.
//! - The iterator version is `nums.iter().map(|&x| nums[x as usize]).collect()`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        // @solution-start
        let mut ans = Vec::with_capacity(nums.len());
        for &x in &nums {
            ans.push(nums[x as usize]);
        }
        ans
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::build_array(vec![0, 2, 1, 5, 3, 4]),
            vec![0, 1, 2, 4, 5, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::build_array(vec![5, 0, 1, 2, 3, 4]),
            vec![4, 5, 0, 1, 2, 3]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::build_array(vec![0]), vec![0]);
    }

    #[test]
    fn identity_and_swap() {
        assert_eq!(Solution::build_array(vec![0, 1, 2, 3]), vec![0, 1, 2, 3]);
        // A permutation made of swaps is its own inverse, so applying it twice gives the identity.
        assert_eq!(Solution::build_array(vec![1, 0, 3, 2]), vec![0, 1, 2, 3]);
    }

    #[test]
    fn large_rotation() {
        // nums[i] = (i + 1) % n, so nums[nums[i]] = (i + 2) % n.
        let n = 1000;
        let nums: Vec<i32> = (0..n).map(|i| (i + 1) % n).collect();
        let expected: Vec<i32> = (0..n).map(|i| (i + 2) % n).collect();
        assert_eq!(Solution::build_array(nums), expected);
    }
}
