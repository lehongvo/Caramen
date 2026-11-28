//! # Problem 0189: Rotate Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** in-place rotation with three `reverse` calls on sub-slices, `k % n`, `rotate_right`
//! - **Source:** [LeetCode 189. Rotate Array](https://leetcode.com/problems/rotate-array/)
//!
//! ## Problem
//!
//! Rotate the vector `nums` to the **right** by `k` steps: every element moves `k` positions to the
//! right, and elements that fall off the end come back at the front. `k` may be larger than the
//! length of the vector.
//!
//! Implement `Solution::rotate(nums: &mut Vec<i32>, k: i32)`, modifying `nums` in place with `O(1)`
//! extra space.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3, 4, 5, 6, 7], k = 3
//! Output: nums = [5, 6, 7, 1, 2, 3, 4]
//!
//! Input:  nums = [-1, -100, 3, 99], k = 2
//! Output: nums = [3, 99, -1, -100]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//! - `0 <= k <= 10^5`
//!
//! ## Hints
//!
//! 1. Rotating by `n` changes nothing, so only `k % n` matters.
//! 2. Reversing the whole vector puts the last `k` elements at the front, but each part is now
//!    backwards. Reverse the two parts again.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** with `k = k % n`, the rotation is "reverse all, then reverse the first `k`, then reverse
//! the remaining `n - k`". Example: `[1..7]`, `k = 3`: reverse all gives `[7,6,5,4,3,2,1]`; reversing
//! `[7,6,5]` and `[4,3,2,1]` gives `[5,6,7,1,2,3,4]`.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `nums[..k].reverse()` and `nums[k..].reverse()` operate on mutable sub-slices of the same
//!   vector, one after the other, so there is never more than one mutable borrow at a time.
//! - The standard library has this built in: `nums.rotate_right(k)`.
//! - `k as usize % n` is computed before slicing, so `k` larger than `n` is fine.
// @solution-end

pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // @solution-start
        let n = nums.len();
        if n == 0 {
            return;
        }
        let k = k as usize % n;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotated(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
        Solution::rotate(&mut nums, k);
        nums
    }

    #[test]
    fn example_by_three() {
        assert_eq!(
            rotated(vec![1, 2, 3, 4, 5, 6, 7], 3),
            vec![5, 6, 7, 1, 2, 3, 4]
        );
    }

    #[test]
    fn example_by_two() {
        assert_eq!(rotated(vec![-1, -100, 3, 99], 2), vec![3, 99, -1, -100]);
    }

    #[test]
    fn zero_and_full_rotations() {
        assert_eq!(rotated(vec![1, 2, 3], 0), vec![1, 2, 3]);
        assert_eq!(rotated(vec![1, 2, 3], 3), vec![1, 2, 3]);
    }

    #[test]
    fn k_larger_than_length() {
        assert_eq!(rotated(vec![1, 2], 5), vec![2, 1]);
        assert_eq!(rotated(vec![7], 100_000), vec![7]);
    }

    #[test]
    fn large_input_matches_rotate_right() {
        let nums: Vec<i32> = (0..100_000).collect();
        let mut expected = nums.clone();
        expected.rotate_right(12_345);
        assert_eq!(rotated(nums, 12_345), expected);
    }
}
