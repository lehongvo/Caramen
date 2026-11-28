// ==================================================================
// PRACTICE 0753: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0753_non_overlapping_intervals.md
// Run the tests:          cargo test practice::p0753
// ==================================================================

//! # Problem 0753: Non-overlapping Intervals
//!
//! - **Difficulty:** Medium
//! - **Topics:** greedy by earliest end (interval scheduling), `sort_unstable_by_key`
//! - **Source:** [LeetCode 435. Non-overlapping Intervals](https://leetcode.com/problems/non-overlapping-intervals/)
//!
//! ## Problem
//!
//! Given a list of intervals `[start, end]`, return the **minimum** number of intervals to remove
//! so that the remaining intervals do not overlap. Here intervals that only touch, such as
//! `[1, 2]` and `[2, 3]`, do **not** overlap.
//!
//! Implement `Solution::erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  intervals = [[1, 2], [2, 3], [3, 4], [1, 3]]
//! Output: 1             (remove [1, 3])
//!
//! Input:  intervals = [[1, 2], [1, 2], [1, 2]]
//! Output: 2
//!
//! Input:  intervals = [[1, 2], [2, 3]]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= intervals.len() <= 10^5`
//! - `intervals[i].len() == 2` and `-5 * 10^4 <= start < end <= 5 * 10^4`
//!
//! ## Hints
//!
//! 1. Removing as few as possible is the same as **keeping** as many as possible.
//! 2. Among all intervals, which one is the safest to keep first? Think about how much room it
//!    leaves for the others.

pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(intervals: &[[i32; 2]]) -> i32 {
        Solution::erase_overlap_intervals(intervals.iter().map(|iv| iv.to_vec()).collect())
    }

    #[test]
    fn example_remove_one() {
        assert_eq!(run(&[[1, 2], [2, 3], [3, 4], [1, 3]]), 1);
    }

    #[test]
    fn example_duplicates() {
        assert_eq!(run(&[[1, 2], [1, 2], [1, 2]]), 2);
    }

    #[test]
    fn example_touching_is_fine() {
        assert_eq!(run(&[[1, 2], [2, 3]]), 0);
    }

    #[test]
    fn long_interval_blocks_short_ones() {
        assert_eq!(run(&[[1, 100], [11, 22], [1, 11], [2, 12]]), 2);
    }

    #[test]
    fn negative_coordinates() {
        assert_eq!(run(&[[-50_000, 50_000], [-3, -1], [-1, 4], [0, 2]]), 2);
    }

    #[test]
    fn many_nested_intervals() {
        // [i - 50_000, 50_000 - i] for i in 0..50_000: all of them contain [-1, 1], keep one.
        let intervals: Vec<Vec<i32>> = (0..50_000).map(|i| vec![i - 50_000, 50_000 - i]).collect();
        assert_eq!(Solution::erase_overlap_intervals(intervals), 49_999);
    }
}
