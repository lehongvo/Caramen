//! # Problem 0125: Remove Duplicates from Sorted Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** in-place compaction through `&mut Vec<i32>`, write index, comparing with the last kept value
//! - **Source:** [LeetCode 26. Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)
//!
//! ## Problem
//!
//! `nums` is sorted in non-decreasing order. Remove the duplicates **in place** so that each distinct
//! value appears only once, keeping the values in sorted order, and return `k`, the number of
//! distinct values.
//!
//! After the call, `nums[..k]` must hold the distinct values in increasing order. The content of the
//! vector after position `k` does not matter.
//!
//! Implement `Solution::remove_duplicates(nums: &mut Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 2]
//! Output: 2, nums starts with [1, 2]
//!
//! Input:  nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]
//! Output: 5, nums starts with [0, 1, 2, 3, 4]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 3 * 10^4`
//! - `-100 <= nums[i] <= 100`
//! - `nums` is sorted in non-decreasing order.
//!
//! ## Hints
//!
//! 1. Because the input is sorted, equal values are next to each other.
//! 2. Keep a `write` index; copy `nums[read]` to `nums[write]` only when it differs from the last
//!    value you kept, `nums[write - 1]`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the first element is always kept, so `write` starts at 1. For every later element,
//! compare it with the last kept value `nums[write - 1]`; if it is new, store it at `write` and
//! advance. `write` is the answer.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - Comparing with the last *kept* value (not with `nums[read - 1]`) keeps the logic correct even
//!   after earlier positions have been overwritten.
//! - The empty-vector guard avoids reporting one distinct value for an empty input.
//! - `Vec::dedup()` does this and shrinks the vector; LeetCode only checks the first `k` slots.
// @solution-end

pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // @solution-start
        if nums.is_empty() {
            return 0;
        }
        let mut write = 1;
        for read in 1..nums.len() {
            if nums[read] != nums[write - 1] {
                nums[write] = nums[read];
                write += 1;
            }
        }
        write as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(mut nums: Vec<i32>) -> Vec<i32> {
        let k = Solution::remove_duplicates(&mut nums) as usize;
        nums.truncate(k);
        nums
    }

    #[test]
    fn example_short() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
        assert_eq!(&nums[..2], &[1, 2]);
    }

    #[test]
    fn example_long() {
        assert_eq!(run(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn single_element() {
        assert_eq!(run(vec![-100]), vec![-100]);
    }

    #[test]
    fn all_equal_and_all_distinct() {
        assert_eq!(run(vec![5, 5, 5, 5]), vec![5]);
        assert_eq!(run(vec![-1, 0, 1]), vec![-1, 0, 1]);
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..30_000).map(|i| i / 150 - 100).collect();
        let expected: Vec<i32> = (-100..100).collect();
        assert_eq!(run(nums), expected);
    }
}
