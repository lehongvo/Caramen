//! # Problem 0735: Search in Rotated Sorted Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary search, deciding which half is sorted, half-open bounds
//! - **Source:** [LeetCode 33. Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)
//!
//! ## Problem
//!
//! A vector of **distinct** values sorted in ascending order may have been rotated at an unknown
//! pivot: for example `[0, 1, 2, 4, 5, 6, 7]` can become `[4, 5, 6, 7, 0, 1, 2]`. Given the
//! (possibly rotated) vector `nums` and a `target`, return the index of `target`, or `-1` if it is
//! not present. The algorithm must run in `O(log n)` time.
//!
//! Implement `Solution::search(nums: Vec<i32>, target: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [4, 5, 6, 7, 0, 1, 2], target = 0
//! Output: 4
//!
//! Input:  nums = [4, 5, 6, 7, 0, 1, 2], target = 3
//! Output: -1
//!
//! Input:  nums = [1], target = 0
//! Output: -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 5000`
//! - `-10^4 <= nums[i], target <= 10^4`, all values of `nums` distinct
//! - `nums` is an ascending vector, possibly rotated.
//!
//! ## Hints
//!
//! 1. Cut the range at `mid`. At least one of the two halves is sorted. How can you tell which?
//! 2. In the sorted half, a simple range check tells whether the target can be there.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** binary search, picking the sorted half: in the half-open range `lo..hi`, look at
//! `mid`. If `nums[lo] <= nums[mid]`, the left part `lo..=mid` is sorted (the rotation point is
//! not inside it); then the target is in that part exactly when
//! `nums[lo] <= target < nums[mid]`, otherwise it can only be in the right part. If instead
//! `nums[lo] > nums[mid]`, the rotation point is in the left part, so the right part `mid..hi` is
//! sorted, and the target is there exactly when `nums[mid] < target <= nums[hi - 1]`. Either way,
//! half of the range is discarded.
//!
//! **Complexity:** `O(log n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - With half-open bounds the last element of the range is `nums[hi - 1]`, which is valid because
//!   `hi > lo >= 0` inside the loop.
//! - The exhaustive test rotates a sorted vector with `rotate_left` and searches for every present
//!   and absent value.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // @solution-start
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[lo] <= nums[mid] {
                // lo..=mid is sorted.
                if nums[lo] <= target && target < nums[mid] {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            } else if nums[mid] < target && target <= nums[hi - 1] {
                // mid..hi is sorted and contains the target.
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        -1
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_found() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn example_missing() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn example_single_element() {
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![1], 1), 0);
    }

    #[test]
    fn two_and_three_elements() {
        assert_eq!(Solution::search(vec![3, 1], 1), 1);
        assert_eq!(Solution::search(vec![3, 1], 3), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 5), 0);
        assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
    }

    #[test]
    fn every_rotation_and_every_target() {
        // Even values only, so the odd values are guaranteed to be absent.
        let sorted: Vec<i32> = (0..60).map(|i| i * 2).collect();
        for k in 0..sorted.len() {
            let mut nums = sorted.clone();
            nums.rotate_left(k);
            for target in -1..=120 {
                let expected = nums
                    .iter()
                    .position(|&x| x == target)
                    .map_or(-1, |i| i as i32);
                assert_eq!(
                    Solution::search(nums.clone(), target),
                    expected,
                    "k = {k}, target = {target}"
                );
            }
        }
    }
}
