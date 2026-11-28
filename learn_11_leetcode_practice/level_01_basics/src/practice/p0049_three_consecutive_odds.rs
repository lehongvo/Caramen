// ==================================================================
// PRACTICE 0049: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0049_three_consecutive_odds.md
// Run the tests:          cargo test practice::p0049
// ==================================================================

//! # Problem 0049: Three Consecutive Odds
//!
//! - **Difficulty:** Easy
//! - **Topics:** index windows `arr[i - 2]`, `arr[i - 1]`, `arr[i]`, ranges that avoid underflow, `&&`
//! - **Source:** [LeetCode 1550. Three Consecutive Odds](https://leetcode.com/problems/three-consecutive-odds/)
//!
//! ## Problem
//!
//! Given a vector of integers `arr`, return `true` if it contains **three odd numbers in a row**
//! (at three consecutive positions), and `false` otherwise.
//!
//! Implement `Solution::three_consecutive_odds(arr: Vec<i32>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr = [2, 6, 4, 1]
//! Output: false
//!
//! Input:  arr = [1, 2, 34, 3, 4, 5, 7, 23, 12]
//! Output: true
//! Explanation: 5, 7 and 23 are consecutive and all odd.
//!
//! Input:  arr = [1, 3]
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= arr.len() <= 1000`
//! - `1 <= arr[i] <= 1000`
//!
//! ## Hints
//!
//! 1. Look at every index `i` that has two elements before it, and test `arr[i - 2]`, `arr[i - 1]`, `arr[i]`.
//! 2. Indices are `usize`: `i - 2` for `i = 0` would underflow and panic. Start the range at 2.
//! 3. A vector with fewer than three elements needs no special case if the range is empty.

pub struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_no_odds_in_a_row() {
        assert!(!Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
    }

    #[test]
    fn example_run_in_the_middle() {
        assert!(Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7, 23, 12
        ]));
    }

    #[test]
    fn short_inputs() {
        assert!(!Solution::three_consecutive_odds(vec![1, 3]));
        assert!(!Solution::three_consecutive_odds(vec![1]));
        assert!(Solution::three_consecutive_odds(vec![1, 3, 5]));
    }

    #[test]
    fn run_at_the_edges() {
        assert!(Solution::three_consecutive_odds(vec![1, 1, 1, 2, 2]));
        assert!(Solution::three_consecutive_odds(vec![2, 2, 9, 9, 9]));
    }

    #[test]
    fn many_pairs_but_no_triple() {
        let arr: Vec<i32> = (0..1000).map(|i| if i % 3 == 2 { 2 } else { 1 }).collect();
        assert!(!Solution::three_consecutive_odds(arr));
    }
}
