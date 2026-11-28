// ==================================================================
// PRACTICE 0750: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0750_merge_intervals.md
// Run the tests:          cargo test practice::p0750
// ==================================================================

//! # Problem 0750: Merge Intervals
//!
//! - **Difficulty:** Medium
//! - **Topics:** sort by start, sweep with `last_mut`, `match` with a guard
//! - **Source:** [LeetCode 56. Merge Intervals](https://leetcode.com/problems/merge-intervals/)
//!
//! ## Problem
//!
//! You are given a list of closed intervals `intervals[i] = [start, end]`. Merge every group of
//! overlapping intervals and return the resulting non-overlapping intervals, which together cover
//! exactly the same points as the input. Two intervals that touch (one ends where the other
//! starts, like `[1, 4]` and `[4, 5]`) overlap. The intervals may be returned in any order.
//!
//! Implement `Solution::merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  intervals = [[1, 3], [2, 6], [8, 10], [15, 18]]
//! Output: [[1, 6], [8, 10], [15, 18]]
//!
//! Input:  intervals = [[1, 4], [4, 5]]
//! Output: [[1, 5]]
//!
//! Input:  intervals = [[4, 7], [1, 4]]
//! Output: [[1, 7]]      (the input is not sorted)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= intervals.len() <= 10^4`
//! - `intervals[i].len() == 2` and `0 <= start <= end <= 10^4`
//!
//! ## Hints
//!
//! 1. After sorting by start, overlapping intervals are next to each other.
//! 2. Compare each interval only with the **last** merged interval.

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Solution::merge(intervals);
        result.sort(); // any order is accepted
        result
    }

    #[test]
    fn example_one_overlap() {
        assert_eq!(
            run(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn example_touching_intervals() {
        assert_eq!(run(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
    }

    #[test]
    fn example_unsorted_input() {
        assert_eq!(run(vec![vec![4, 7], vec![1, 4]]), vec![vec![1, 7]]);
    }

    #[test]
    fn interval_inside_another() {
        assert_eq!(
            run(vec![vec![1, 10], vec![2, 3], vec![4, 5]]),
            vec![vec![1, 10]]
        );
    }

    #[test]
    fn single_and_point_intervals() {
        assert_eq!(run(vec![vec![5, 5]]), vec![vec![5, 5]]);
        assert_eq!(
            run(vec![vec![1, 1], vec![2, 2]]),
            vec![vec![1, 1], vec![2, 2]]
        );
    }

    #[test]
    fn long_chain_merges_into_one() {
        let intervals: Vec<Vec<i32>> = (0..10_000).rev().map(|i| vec![i, i + 1]).collect();
        assert_eq!(run(intervals), vec![vec![0, 10_000]]);
    }
}
