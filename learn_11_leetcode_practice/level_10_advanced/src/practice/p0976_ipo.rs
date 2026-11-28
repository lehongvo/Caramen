// ==================================================================
// PRACTICE 0976: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0976_ipo.md
// Run the tests:          cargo test practice::p0976
// ==================================================================

//! # Problem 0976: IPO
//!
//! - **Difficulty:** Hard
//! - **Topics:** greedy with a max-heap of affordable profits, sorting pairs by capital, `zip`, `BinaryHeap`
//! - **Source:** [LeetCode 502. IPO](https://leetcode.com/problems/ipo/)
//!
//! ## Problem
//!
//! You start with capital `w` and may complete at most `k` distinct projects. Project `i` requires a
//! capital of at least `capital[i]` to start (it is not consumed) and adds `profits[i]` to your
//! capital when finished. Projects are done one after another. Return the maximum capital you can
//! end up with.
//!
//! Implement `Solution::find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  k = 2, w = 0, profits = [1, 2, 3], capital = [0, 1, 1]
//! Output: 4      (do project 0 -> capital 1, then project 2 -> capital 4)
//!
//! Input:  k = 3, w = 0, profits = [1, 2, 3], capital = [0, 1, 2]
//! Output: 6
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= 10^5`, `0 <= w <= 10^9`
//! - `1 <= profits.len() == capital.len() <= 10^5`
//! - `0 <= profits[i] <= 10^4`, `0 <= capital[i] <= 10^9`
//! - The answer fits in an `i32`.
//!
//! ## Hints
//!
//! 1. At every step, among the projects you can afford, the most profitable one is always a safe
//!    choice: profits are never negative, so more capital never hurts.
//! 2. Sort projects by required capital; as your capital grows, push newly affordable profits into a
//!    max-heap.
//! 3. Stop early when no project is affordable.

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
        assert_eq!(
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }

    #[test]
    fn nothing_affordable() {
        assert_eq!(Solution::find_maximized_capital(5, 0, vec![10], vec![1]), 0);
        assert_eq!(
            Solution::find_maximized_capital(1, 2, vec![1, 2, 3], vec![1, 1, 2]),
            5
        );
    }

    #[test]
    fn k_larger_than_projects() {
        assert_eq!(
            Solution::find_maximized_capital(10, 1, vec![1, 1, 1], vec![0, 1, 3]),
            4
        );
    }

    #[test]
    fn many_projects() {
        let n = 100_000;
        let profits = vec![1; n];
        let capital: Vec<i32> = (0..n as i32).collect();
        // Each project unlocks the next one.
        assert_eq!(
            Solution::find_maximized_capital(100_000, 0, profits, capital),
            100_000
        );
    }
}
