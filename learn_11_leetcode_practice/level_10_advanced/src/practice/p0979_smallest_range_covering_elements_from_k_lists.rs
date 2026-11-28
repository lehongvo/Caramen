// ==================================================================
// PRACTICE 0979: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0979_smallest_range_covering_elements_from_k_lists.md
// Run the tests:          cargo test practice::p0979
// ==================================================================

//! # Problem 0979: Smallest Range Covering Elements from K Lists
//!
//! - **Difficulty:** Hard
//! - **Topics:** k-way merge with a min-heap of `Reverse((value, list, index))`, tracking the current maximum, tie-breaking ranges
//! - **Source:** [LeetCode 632. Smallest Range Covering Elements from K Lists](https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/)
//!
//! ## Problem
//!
//! You have `k` lists of integers, each sorted in non-decreasing order. Find the smallest range
//! `[a, b]` that contains **at least one number from each list**. Range `[a, b]` is smaller than
//! `[c, d]` if `b - a < d - c`, or if `b - a == d - c` and `a < c`.
//!
//! Implement `Solution::smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32>` returning `[a, b]`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]]
//! Output: [20, 24]       (24 from list 1, 20 from list 2, 22 from list 3)
//!
//! Input:  nums = [[1, 2, 3], [1, 2, 3], [1, 2, 3]]
//! Output: [1, 1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= 3500`, `1 <= nums[i].len() <= 50`
//! - `-10^5 <= nums[i][j] <= 10^5`, each list sorted.
//!
//! ## Hints
//!
//! 1. Keep one "current" element per list. The range they span is `[min, max]`.
//! 2. To possibly shrink it, the only useful move is to advance the list holding the minimum.
//! 3. A min-heap gives the minimum; keep the maximum in a variable (it only grows). Stop when a list
//!    runs out.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(nums: &[Vec<i32>]) -> Vec<i32> {
        let mut candidates: Vec<i32> = nums.iter().flatten().copied().collect();
        candidates.sort_unstable();
        candidates.dedup();
        let mut best: Option<(i32, i32)> = None;
        for &a in &candidates {
            for &b in &candidates {
                if b < a {
                    continue;
                }
                let covers = nums
                    .iter()
                    .all(|list| list.iter().any(|&x| a <= x && x <= b));
                let better = best.is_none_or(|(c, d)| (b - a, a) < (d - c, c));
                if covers && better {
                    best = Some((a, b));
                }
            }
        }
        let (a, b) = best.unwrap();
        vec![a, b]
    }

    #[test]
    fn examples() {
        let nums = vec![
            vec![4, 10, 15, 24, 26],
            vec![0, 9, 12, 20],
            vec![5, 18, 22, 30],
        ];
        assert_eq!(Solution::smallest_range(nums), vec![20, 24]);
        assert_eq!(Solution::smallest_range(vec![vec![1, 2, 3]; 3]), vec![1, 1]);
    }

    #[test]
    fn single_list_and_ties() {
        assert_eq!(Solution::smallest_range(vec![vec![5, 8, 100]]), vec![5, 5]);
        // [1, 3] and [7, 9] both have width 2: the one with the smaller start wins.
        assert_eq!(
            Solution::smallest_range(vec![vec![1, 7], vec![3, 9]]),
            vec![1, 3]
        );
        assert_eq!(
            Solution::smallest_range(vec![vec![-100_000], vec![100_000]]),
            vec![-100_000, 100_000]
        );
    }

    #[test]
    fn matches_naive() {
        let nums: Vec<Vec<i32>> = (0..5)
            .map(|k: i32| {
                let mut list: Vec<i32> = (0..8).map(|i| (i * 37 + k * 101) % 97 - 40).collect();
                list.sort_unstable();
                list
            })
            .collect();
        assert_eq!(Solution::smallest_range(nums.clone()), naive(&nums));
    }

    #[test]
    fn many_lists() {
        let nums: Vec<Vec<i32>> = (0..3_500)
            .map(|k| (0..50).map(|i| i * 3_500 + k).collect())
            .collect();
        // Every window of 3_500 consecutive integers covers all lists; the first is [0, 3_499].
        assert_eq!(Solution::smallest_range(nums), vec![0, 3_499]);
    }
}
