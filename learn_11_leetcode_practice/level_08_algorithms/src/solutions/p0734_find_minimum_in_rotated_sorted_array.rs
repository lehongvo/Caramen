//! # Problem 0734: Find Minimum in Rotated Sorted Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary search on a rotated array, comparing with the right end, `rotate_left` in tests
//! - **Source:** [LeetCode 153. Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)
//!
//! ## Problem
//!
//! A vector of **distinct** values sorted in ascending order was rotated between `1` and `n`
//! times. Rotating once moves the last element to the front: `[0, 1, 2, 4, 5, 6, 7]` rotated four
//! times is `[4, 5, 6, 7, 0, 1, 2]`, and rotated `n` times it is unchanged.
//!
//! Given the rotated vector `nums`, return its minimum value in `O(log n)` time.
//!
//! Implement `Solution::find_min(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 4, 5, 1, 2]
//! Output: 1
//!
//! Input:  nums = [4, 5, 6, 7, 0, 1, 2]
//! Output: 0
//!
//! Input:  nums = [11, 13, 15, 17]
//! Output: 11            (rotated n times: still sorted)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 5000`
//! - `-5000 <= nums[i] <= 5000`, all distinct
//! - `nums` is a sorted vector rotated between `1` and `n` times.
//!
//! ## Hints
//!
//! 1. The rotated array is two sorted runs; the minimum is the first element of the second run.
//! 2. Compare `nums[mid]` with the **last** element of the current range. What does
//!    `nums[mid] > nums[hi]` tell you about where the minimum is?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** binary search against the right end: keep `lo..=hi` containing the minimum. If
//! `nums[mid] > nums[hi]`, the range is not sorted between `mid` and `hi`, so the drop (and the
//! minimum) is strictly to the right of `mid`: `lo = mid + 1`. Otherwise `nums[mid..=hi]` is
//! sorted, so `nums[mid]` is the smallest value of that part and the minimum is at `mid` or to its
//! left: `hi = mid`. When `lo == hi`, it points at the minimum.
//!
//! Comparing with the left end instead does not work as simply: for a range that is already
//! sorted, `nums[lo] < nums[mid]` does not tell which side the minimum is on.
//!
//! **Complexity:** `O(log n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - The tests build every rotation with
//!   [`slice::rotate_right`](https://doc.rust-lang.org/std/primitive.slice.html#method.rotate_right),
//!   which rotates in place in `O(n)`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // @solution-start
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] > nums[hi] {
                lo = mid + 1; // the drop is to the right of mid
            } else {
                hi = mid; // nums[mid..=hi] is sorted
            }
        }
        nums[lo]
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_rotated_three_times() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn example_rotated_four_times() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn example_not_rotated() {
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn tiny_inputs() {
        assert_eq!(Solution::find_min(vec![1]), 1);
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
        assert_eq!(Solution::find_min(vec![1, 2]), 1);
    }

    #[test]
    fn every_rotation() {
        let sorted: Vec<i32> = (-2500..2500).step_by(7).collect();
        for k in 0..sorted.len() {
            let mut nums = sorted.clone();
            nums.rotate_right(k);
            assert_eq!(Solution::find_min(nums), -2500, "rotation {k}");
        }
    }
}
