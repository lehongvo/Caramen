//! # Problem 0743: Daily Temperatures
//!
//! - **Difficulty:** Medium
//! - **Topics:** monotonic stack of indices, distances between indices, `pop_if`
//! - **Source:** [LeetCode 739. Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)
//!
//! ## Problem
//!
//! `temperatures[i]` is the temperature on day `i`. Return a vector `answer` where `answer[i]` is
//! the number of days you have to wait after day `i` to get a **strictly warmer** day. If no
//! later day is warmer, `answer[i]` is `0`.
//!
//! Implement `Solution::daily_temperatures(temperatures: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  temperatures = [73, 74, 75, 71, 69, 72, 76, 73]
//! Output: [1, 1, 4, 2, 1, 1, 0, 0]
//!
//! Input:  temperatures = [30, 40, 50, 60]
//! Output: [1, 1, 1, 0]
//!
//! Input:  temperatures = [30, 60, 90]
//! Output: [1, 1, 0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= temperatures.len() <= 10^5`
//! - `30 <= temperatures[i] <= 100`
//!
//! ## Hints
//!
//! 1. With `10^5` days, checking every later day for every day is too slow.
//! 2. Keep the days that are still waiting for a warmer day on a stack. Store **indices**, so you
//!    can compute the distance when the warmer day arrives.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** monotonic stack of indices: scan the days. Day `j` is the answer for every waiting
//! day with a strictly lower temperature, and those are exactly the days on top of the stack
//! (waiting temperatures are non-increasing from bottom to top). Pop each such day `i` and store
//! `j - i`. Then push `j`. Days still on the stack at the end keep the default `0`.
//!
//! **Complexity:** `O(n)` time (each index is pushed and popped at most once), `O(n)` space.
//!
//! **Rust concepts:**
//! - `vec![0; n]` pre-fills the "no warmer day" answer, so the leftover stack needs no clean-up.
//! - `pop_if` with a closure that reads `temperatures` by shared reference while `answer` is
//!   mutated in the loop body: the borrows do not overlap.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // @solution-start
        let mut answer = vec![0; temperatures.len()];
        let mut waiting: Vec<usize> = Vec::new();
        for (j, &t) in temperatures.iter().enumerate() {
            while let Some(i) = waiting.pop_if(|i| temperatures[*i] < t) {
                answer[i] = (j - i) as i32;
            }
            waiting.push(j);
        }
        answer
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn example_increasing() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn example_short() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }

    #[test]
    fn equal_temperatures_are_not_warmer() {
        assert_eq!(
            Solution::daily_temperatures(vec![50, 50, 50]),
            vec![0, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![50, 50, 51]),
            vec![2, 1, 0]
        );
    }

    #[test]
    fn long_cold_spell_then_heat() {
        let mut temperatures = vec![30; 99_999];
        temperatures.push(100);
        let answer = Solution::daily_temperatures(temperatures);
        assert_eq!(answer[0], 99_999);
        assert_eq!(answer[99_998], 1);
        assert_eq!(answer[99_999], 0);
    }
}
