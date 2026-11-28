// ==================================================================
// PRACTICE 0733: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0733_find_peak_element.md
// Run the tests:          cargo test practice::p0733
// ==================================================================

//! # Problem 0733: Find Peak Element
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary search on a slope (unsorted input), invariants, checking "any valid answer"
//! - **Source:** [LeetCode 162. Find Peak Element](https://leetcode.com/problems/find-peak-element/)
//!
//! ## Problem
//!
//! A **peak** is an element strictly greater than its neighbours. Given a vector `nums` in which
//! no two adjacent values are equal, return the index of **any** peak. Imagine
//! `nums[-1] = nums[n] = -infinity`, so the first and last elements only need to beat their single
//! real neighbour.
//!
//! Your algorithm must run in `O(log n)` time.
//!
//! Implement `Solution::find_peak_element(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3, 1]
//! Output: 2             (3 is a peak)
//!
//! Input:  nums = [1, 2, 1, 3, 5, 6, 4]
//! Output: 1 or 5        (2 and 6 are both peaks; either index is accepted)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 1000`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//! - `nums[i] != nums[i + 1]` for every valid `i`.
//!
//! ## Hints
//!
//! 1. The array is not sorted, but look at `nums[mid]` and `nums[mid + 1]`. If you walk uphill,
//!    must you eventually reach a peak?
//! 2. Keep a range that is guaranteed to contain a peak, and halve it.

pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that `index` is a peak of `nums`.
    fn assert_peak(nums: &[i32], index: i32) {
        let i = usize::try_from(index).expect("index must be non-negative");
        assert!(i < nums.len(), "index {i} out of bounds");
        assert!(
            i == 0 || nums[i - 1] < nums[i],
            "left neighbour is not smaller at {i}"
        );
        assert!(
            i + 1 == nums.len() || nums[i + 1] < nums[i],
            "right neighbour is not smaller at {i}"
        );
    }

    fn check(nums: Vec<i32>) {
        let index = Solution::find_peak_element(nums.clone());
        assert_peak(&nums, index);
    }

    #[test]
    fn example_single_peak() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
    }

    #[test]
    fn example_two_peaks() {
        check(vec![1, 2, 1, 3, 5, 6, 4]);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::find_peak_element(vec![5]), 0);
    }

    #[test]
    fn monotonic_inputs() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::find_peak_element(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn extreme_values() {
        check(vec![i32::MIN, i32::MAX, i32::MIN]);
        check(vec![i32::MAX, i32::MIN]);
    }

    #[test]
    fn zigzag_input() {
        let nums: Vec<i32> = (0..1000).map(|i| if i % 2 == 0 { i } else { -i }).collect();
        check(nums);
    }
}
