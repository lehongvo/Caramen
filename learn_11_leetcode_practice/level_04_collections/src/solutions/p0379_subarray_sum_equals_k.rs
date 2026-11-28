//! # Problem 0379: Subarray Sum Equals K
//!
//! - **Difficulty:** Medium
//! - **Topics:** running prefix sums, `HashMap<i32, i32>` counting earlier prefixes, `get` before `entry`
//! - **Source:** [LeetCode 560. Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/)
//!
//! ## Problem
//!
//! Given a vector of integers `nums` (which may contain negative numbers) and an integer `k`,
//! return the number of **subarrays** whose elements add up to `k`. A subarray is a contiguous,
//! non-empty part of the vector.
//!
//! Implement `Solution::subarray_sum(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 1], k = 2
//! Output: 2        ([1, 1] at positions 0..2 and 1..3)
//!
//! Input:  nums = [1, 2, 3], k = 3
//! Output: 2        ([1, 2] and [3])
//!
//! Input:  nums = [1, -1, 0], k = 0
//! Output: 3        ([1, -1], [0], [1, -1, 0])
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 2 * 10^4`
//! - `-1000 <= nums[i] <= 1000`
//! - `-10^7 <= k <= 10^7`
//!
//! ## Hints
//!
//! 1. With `prefix[j]` = sum of the first `j` elements, the subarray `i..j` sums to
//!    `prefix[j] - prefix[i]`.
//! 2. So, at position `j`, the number of good subarrays ending here is the number of earlier
//!    prefixes equal to `prefix[j] - k`.
//! 3. Count earlier prefixes in a `HashMap`. Do not forget the empty prefix (sum `0`).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** walk the vector keeping the running sum. Before recording the current prefix sum, add
//! the number of earlier prefixes equal to `sum - k`; each of them starts a subarray that ends here
//! and sums to `k`. The map starts with `{0: 1}` for the empty prefix. Negative numbers are why a
//! sliding window does not work here, while prefix counting does.
//!
//! **Complexity:** `O(n)` average time, `O(n)` space.
//!
//! **Rust concepts:**
//! - `HashMap::from([(0, 1)])` builds a map from an array of pairs.
//! - Lookup first (`get(&(sum - k))`), then record (`entry(sum).or_insert(0)`): doing it in the
//!   other order would count the empty subarray when `k == 0`.
//! - Sums stay within `±2 * 10^7`, so `i32` is enough.
// @solution-end

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        // @solution-start
        let mut seen: HashMap<i32, i32> = HashMap::from([(0, 1)]);
        let mut sum = 0;
        let mut count = 0;
        for x in nums {
            sum += x;
            count += seen.get(&(sum - k)).copied().unwrap_or(0);
            *seen.entry(sum).or_insert(0) += 1;
        }
        count
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }

    #[test]
    fn example_three_zero_target() {
        assert_eq!(Solution::subarray_sum(vec![1, -1, 0], 0), 3);
    }

    #[test]
    fn single_element_and_no_match() {
        assert_eq!(Solution::subarray_sum(vec![5], 5), 1);
        assert_eq!(Solution::subarray_sum(vec![5], -5), 0);
        assert_eq!(Solution::subarray_sum(vec![-1, -1, 1], 0), 1);
    }

    #[test]
    fn many_zeros() {
        // Every one of the n * (n + 1) / 2 subarrays of zeros sums to 0.
        assert_eq!(Solution::subarray_sum(vec![0; 20_000], 0), 200_010_000);
    }
}
