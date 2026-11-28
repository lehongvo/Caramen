//! # Problem 0048: Max Consecutive Ones
//!
//! - **Difficulty:** Easy
//! - **Topics:** counters that reset, `if` / `else`, `max`
//! - **Source:** [LeetCode 485. Max Consecutive Ones](https://leetcode.com/problems/max-consecutive-ones/)
//!
//! ## Problem
//!
//! Given a vector `nums` that contains only `0`s and `1`s, return the length of the longest run of
//! consecutive `1`s.
//!
//! Implement `Solution::find_max_consecutive_ones(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 0, 1, 1, 1]
//! Output: 3
//!
//! Input:  nums = [1, 0, 1, 1, 0, 1]
//! Output: 2
//!
//! Input:  nums = [0, 0]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `nums[i]` is `0` or `1`
//!
//! ## Hints
//!
//! 1. Keep the length of the **current** run and the length of the **best** run.
//! 2. A `0` ends the current run: reset its counter.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** scan once. On a `1`, extend the current run and update the best; on a `0`, reset the
//! current run to zero.
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - Two `mut` counters carry state across iterations; the `if`/`else` decides how to update them.
//! - Updating `best` inside the `1` branch (instead of only at a `0`) handles a run that reaches
//!   the end of the vector.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        // @solution-start
        let mut current = 0;
        let mut best = 0;
        for &x in &nums {
            if x == 1 {
                current += 1;
                best = best.max(current);
            } else {
                current = 0;
            }
        }
        best
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_run_at_the_end() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
    }

    #[test]
    fn example_run_in_the_middle() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }

    #[test]
    fn no_ones() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![0, 0]), 0);
        assert_eq!(Solution::find_max_consecutive_ones(vec![0]), 0);
    }

    #[test]
    fn all_ones() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1; 100_000]),
            100_000
        );
    }

    #[test]
    fn alternating() {
        let nums: Vec<i32> = (0..100_000).map(|i| i % 2).collect();
        assert_eq!(Solution::find_max_consecutive_ones(nums), 1);
    }
}
