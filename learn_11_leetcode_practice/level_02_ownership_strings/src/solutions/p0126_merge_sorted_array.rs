//! # Problem 0126: Merge Sorted Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** two `&mut Vec<i32>` parameters, filling from the back, `usize` index arithmetic
//! - **Source:** [LeetCode 88. Merge Sorted Array](https://leetcode.com/problems/merge-sorted-array/)
//!
//! ## Problem
//!
//! You get two vectors sorted in non-decreasing order, `nums1` and `nums2`, and two counts `m` and
//! `n`. Only the first `m` elements of `nums1` are real values; `nums1` has length `m + n` and its
//! last `n` slots are placeholders (zeros). `nums2` has exactly `n` elements.
//!
//! Merge `nums2` into `nums1` **in place** so that `nums1` holds all `m + n` values in non-decreasing
//! order. The function returns nothing; the result lives in `nums1`.
//!
//! Implement `Solution::merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32)`.
//! (`nums2` is only read; LeetCode's signature happens to pass it as `&mut`.)
//!
//! ## Examples
//!
//! ```text
//! Input:  nums1 = [1, 2, 3, 0, 0, 0], m = 3, nums2 = [2, 5, 6], n = 3
//! Output: nums1 = [1, 2, 2, 3, 5, 6]
//!
//! Input:  nums1 = [1], m = 1, nums2 = [], n = 0
//! Output: nums1 = [1]
//!
//! Input:  nums1 = [0], m = 0, nums2 = [1], n = 1
//! Output: nums1 = [1]
//! ```
//!
//! ## Constraints
//!
//! - `nums1.len() == m + n`, `nums2.len() == n`
//! - `0 <= m, n <= 200`, `1 <= m + n <= 200`
//! - `-10^9 <= nums1[i], nums2[j] <= 10^9`
//!
//! ## Hints
//!
//! 1. Merging from the front would overwrite values of `nums1` you have not read yet.
//! 2. The free space is at the **end** of `nums1`: compare the largest remaining values of both
//!    vectors and write the larger one into the last free slot.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** keep `i` = number of unread `nums1` values and `j` = number of unread `nums2` values.
//! The next slot to fill is `i + j - 1`. Write the larger of `nums1[i - 1]` and `nums2[j - 1]` there.
//! Once `nums2` is exhausted, the remaining `nums1` values are already in place.
//!
//! **Complexity:** `O(m + n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - Counting "remaining" elements instead of pointing at the last one keeps every index a valid
//!   `usize`: we only compute `i - 1` after checking `i > 0`, so nothing underflows.
//! - `&&` short-circuits, so `nums1[i - 1]` is never evaluated when `i == 0`.
//! - Both parameters are `&mut Vec<i32>` because of LeetCode; Clippy's `ptr_arg` is allowed.
// @solution-end

pub struct Solution;

impl Solution {
    // LeetCode's signature: both vectors are `&mut Vec<i32>`, although slices would be enough.
    #[allow(clippy::ptr_arg)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // @solution-start
        let (mut i, mut j) = (m as usize, n as usize);
        while j > 0 {
            let slot = i + j - 1;
            if i > 0 && nums1[i - 1] > nums2[j - 1] {
                nums1[slot] = nums1[i - 1];
                i -= 1;
            } else {
                nums1[slot] = nums2[j - 1];
                j -= 1;
            }
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merged(mut nums1: Vec<i32>, m: i32, mut nums2: Vec<i32>, n: i32) -> Vec<i32> {
        Solution::merge(&mut nums1, m, &mut nums2, n);
        nums1
    }

    #[test]
    fn example_interleaved() {
        assert_eq!(merged(vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3), vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn example_second_empty() {
        assert_eq!(merged(vec![1], 1, vec![], 0), vec![1]);
    }

    #[test]
    fn example_first_empty() {
        assert_eq!(merged(vec![0], 0, vec![1], 1), vec![1]);
    }

    #[test]
    fn all_of_nums2_is_smaller() {
        assert_eq!(merged(vec![4, 5, 0, 0], 2, vec![-3, -1], 2), vec![-3, -1, 4, 5]);
    }

    #[test]
    fn largest_input() {
        let mut nums1: Vec<i32> = (0..100).map(|i| i * 2).collect();
        nums1.extend(std::iter::repeat_n(0, 100));
        let nums2: Vec<i32> = (0..100).map(|i| i * 2 + 1).collect();
        assert_eq!(merged(nums1, 100, nums2, 100), (0..200).collect::<Vec<i32>>());
    }
}
