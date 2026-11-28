// ==================================================================
// PRACTICE 0972: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0972_median_of_two_sorted_arrays.md
// Run the tests:          cargo test practice::p0972
// ==================================================================

//! # Problem 0972: Median of Two Sorted Arrays
//!
//! - **Difficulty:** Hard
//! - **Topics:** binary search on a partition, sentinels with `i32::MIN`/`i32::MAX` via `Option` defaults, `std::mem::swap` of `Vec`s
//! - **Source:** [LeetCode 4. Median of Two Sorted Arrays](https://leetcode.com/problems/median-of-two-sorted-arrays/)
//!
//! ## Problem
//!
//! Given two sorted arrays `nums1` and `nums2` (sizes `m` and `n`, not both empty), return the median
//! of the combined `m + n` numbers. The overall run time must be `O(log(m + n))`, so merging is not
//! allowed.
//!
//! Implement `Solution::find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums1 = [1, 3], nums2 = [2]       Output: 2.0
//! Input:  nums1 = [1, 2], nums2 = [3, 4]    Output: 2.5
//! ```
//!
//! ## Constraints
//!
//! - `0 <= m, n <= 1000`, `1 <= m + n <= 2000`
//! - `-10^6 <= nums1[i], nums2[i] <= 10^6`
//!
//! ## Hints
//!
//! 1. Cut `nums1` after `i` elements and `nums2` after `j = (m + n + 1) / 2 - i` elements: the two left
//!    parts together hold the smaller half.
//! 2. The cut is correct when `left1 <= right2` and `left2 <= right1` (missing neighbours count as
//!    minus or plus infinity). Binary search `i` over the **shorter** array.
//! 3. Odd total: the median is `max(left1, left2)`. Even: the mean of that and `min(right1, right2)`.

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(a: &[i32], b: &[i32]) -> f64 {
        let mut all: Vec<i32> = a.iter().chain(b).copied().collect();
        all.sort_unstable();
        let n = all.len();
        if n % 2 == 1 {
            f64::from(all[n / 2])
        } else {
            (f64::from(all[n / 2 - 1]) + f64::from(all[n / 2])) / 2.0
        }
    }

    #[test]
    fn examples() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn one_empty_array() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2, 3], vec![]), 2.5);
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![-1, 0, 7]),
            0.0
        );
    }

    #[test]
    fn duplicates_and_extremes() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 1, 1], vec![1, 1]),
            1.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![-1_000_000], vec![1_000_000]),
            0.0
        );
    }

    #[test]
    fn matches_naive() {
        for m in 0..12 {
            for n in 0..12 {
                if m + n == 0 {
                    continue;
                }
                let a: Vec<i32> = (0..m).map(|i| i * 3 - 7).collect();
                let b: Vec<i32> = (0..n).map(|i| i * 2 - 4).collect();
                assert_eq!(
                    Solution::find_median_sorted_arrays(a.clone(), b.clone()),
                    naive(&a, &b),
                    "m = {m}, n = {n}"
                );
            }
        }
    }
}
