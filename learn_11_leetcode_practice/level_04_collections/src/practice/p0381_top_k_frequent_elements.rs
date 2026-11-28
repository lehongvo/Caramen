// ==================================================================
// PRACTICE 0381: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0381_top_k_frequent_elements.md
// Run the tests:          cargo test practice::p0381
// ==================================================================

//! # Problem 0381: Top K Frequent Elements
//!
//! - **Difficulty:** Medium
//! - **Topics:** counting with a `HashMap`, a min-heap of `Reverse((count, value))` bounded to `k`, top-k pattern
//! - **Source:** [LeetCode 347. Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/)
//!
//! ## Problem
//!
//! Given a vector of integers `nums` and an integer `k`, return the `k` values that occur most
//! often. The answer is guaranteed to be **unique** (there is no tie at the `k`-th place), and it
//! may be returned in **any order**.
//!
//! LeetCode's follow-up asks for something better than `O(n log n)`.
//!
//! Implement `Solution::top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 1, 2, 2, 3], k = 2
//! Output: [1, 2]
//!
//! Input:  nums = [1], k = 1
//! Output: [1]
//!
//! Input:  nums = [1, 2, 1, 2, 1, 2, 3, 1, 3, 2], k = 2
//! Output: [1, 2]      ([2, 1] is also accepted)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-10^4 <= nums[i] <= 10^4`
//! - `1 <= k <=` number of distinct values; the answer is unique.
//!
//! ## Hints
//!
//! 1. Count the values first.
//! 2. Keep the `k` best `(count, value)` pairs in a **min**-heap: push each pair and, if the heap
//!    holds more than `k`, pop the smallest. That is `O(d log k)` for `d` distinct values.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        assert_eq!(
            sorted(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2)),
            vec![1, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            sorted(Solution::top_k_frequent(
                vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2],
                2
            )),
            vec![1, 2]
        );
    }

    #[test]
    fn all_distinct_values_requested() {
        assert_eq!(
            sorted(Solution::top_k_frequent(vec![-4, 9, -4, 0], 3)),
            vec![-4, 0, 9]
        );
    }

    #[test]
    fn large_input() {
        // Value v occurs v + 1 times for v in 0..400 (80_200 elements): the top 3 are 397..=399.
        let nums: Vec<i32> = (0..400)
            .flat_map(|v| std::iter::repeat_n(v, v as usize + 1))
            .collect();
        assert_eq!(
            sorted(Solution::top_k_frequent(nums, 3)),
            vec![397, 398, 399]
        );
    }
}
