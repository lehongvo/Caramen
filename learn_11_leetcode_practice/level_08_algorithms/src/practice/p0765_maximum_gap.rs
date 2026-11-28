// ==================================================================
// PRACTICE 0765: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0765_maximum_gap.md
// Run the tests:          cargo test practice::p0765
// ==================================================================

//! # Problem 0765: Maximum Gap
//!
//! - **Difficulty:** Medium
//! - **Topics:** radix sort (LSD, base 256), counting with prefix positions, `std::mem::swap`, `windows`
//! - **Source:** [LeetCode 164. Maximum Gap](https://leetcode.com/problems/maximum-gap/)
//!
//! ## Problem
//!
//! Given a vector `nums` of non-negative integers, return the largest difference between two
//! successive elements **in sorted order**. If the vector has fewer than two elements, return `0`.
//!
//! The algorithm must run in linear time and use linear extra space, so a comparison sort
//! (`O(n log n)`) is not allowed. Implement a radix sort.
//!
//! Implement `Solution::maximum_gap(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 6, 9, 1]
//! Output: 3             (sorted: 1, 3, 6, 9; every gap is 3 or less)
//!
//! Input:  nums = [10]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `0 <= nums[i] <= 10^9`
//!
//! ## Hints
//!
//! 1. A radix sort sorts by one digit at a time, starting with the **least** significant one, using
//!    a stable counting sort for each digit.
//! 2. With base 256, a `u32` has 4 digits (bytes): 4 passes of counting sort.
//! 3. For a stable pass: count each digit, turn the counts into starting positions, then place every
//!    element at its digit's next position.

pub struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_values() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    }

    #[test]
    fn example_single_value() {
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }

    #[test]
    fn duplicates() {
        assert_eq!(Solution::maximum_gap(vec![1, 1, 1, 1]), 0);
    }

    #[test]
    fn large_gaps() {
        assert_eq!(Solution::maximum_gap(vec![1, 10_000_000]), 9_999_999);
        assert_eq!(Solution::maximum_gap(vec![1_000_000_000, 0]), 1_000_000_000);
    }

    #[test]
    fn values_differing_in_high_bytes() {
        // 0x0100_0000 and 0x00FF_FFFF differ only after the most significant pass.
        assert_eq!(
            Solution::maximum_gap(vec![0x0100_0000, 0x00FF_FFFF, 0]),
            0x00FF_FFFF
        );
    }

    #[test]
    fn large_input_matches_sorting() {
        let mut state = 7u64;
        let nums: Vec<i32> = (0..100_000)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1);
                ((state >> 33) % 1_000_000_001) as i32
            })
            .collect();
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        let expected = sorted.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0);
        assert_eq!(Solution::maximum_gap(nums), expected);
    }
}
