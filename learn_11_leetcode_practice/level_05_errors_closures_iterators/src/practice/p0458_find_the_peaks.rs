// ==================================================================
// PRACTICE 0458: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0458_find_the_peaks.md
// Run the tests:          cargo test practice::p0458
// ==================================================================

//! # Problem 0458: Find the Peaks
//!
//! - **Difficulty:** Easy
//! - **Topics:** `slice::windows(3)`, `enumerate`, `filter_map`, index arithmetic on windows
//! - **Source:** [LeetCode 2951. Find the Peaks](https://leetcode.com/problems/find-the-peaks/)
//!
//! ## Problem
//!
//! `mountain[i]` is the height at position `i`. A *peak* is a position that is **strictly** higher
//! than both of its neighbours. The first and the last positions are never peaks (they have only
//! one neighbour).
//!
//! Return the indexes of all peaks in increasing order.
//!
//! Implement `Solution::find_peaks(mountain: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mountain = [2, 4, 4]
//! Output: []
//! Explanation: index 1 is not strictly higher than index 2.
//!
//! Input:  mountain = [1, 4, 3, 8, 5]
//! Output: [1, 3]
//! ```
//!
//! ## Constraints
//!
//! - `3 <= mountain.len() <= 100`
//! - `1 <= mountain[i] <= 100`
//!
//! ## Hints
//!
//! 1. `mountain.windows(3)` yields every group `[left, middle, right]` of three neighbours.
//! 2. The window with index `i` (from `enumerate`) has its middle element at index `i + 1`.

pub struct Solution;

impl Solution {
    pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_plateau() {
        assert_eq!(Solution::find_peaks(vec![2, 4, 4]), Vec::<i32>::new());
    }

    #[test]
    fn example_two_peaks() {
        assert_eq!(Solution::find_peaks(vec![1, 4, 3, 8, 5]), vec![1, 3]);
    }

    #[test]
    fn edges_are_not_peaks() {
        assert_eq!(Solution::find_peaks(vec![9, 1, 9]), Vec::<i32>::new());
        assert_eq!(Solution::find_peaks(vec![1, 2, 1]), vec![1]);
    }

    #[test]
    fn zigzag() {
        let mountain: Vec<i32> = (0..100).map(|i| if i % 2 == 0 { 1 } else { 100 }).collect();
        let expected: Vec<i32> = (1..99).step_by(2).collect();
        assert_eq!(Solution::find_peaks(mountain), expected);
    }
}
