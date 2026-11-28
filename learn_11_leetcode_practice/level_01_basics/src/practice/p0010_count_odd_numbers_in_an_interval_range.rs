// ==================================================================
// PRACTICE 0010: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0010_count_odd_numbers_in_an_interval_range.md
// Run the tests:          cargo test practice::p0010
// ==================================================================

//! # Problem 0010: Count Odd Numbers in an Interval Range
//!
//! - **Difficulty:** Easy
//! - **Topics:** integer division rounding, counting with a formula instead of a loop
//! - **Source:** [LeetCode 1523. Count Odd Numbers in an Interval Range](https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/)
//!
//! ## Problem
//!
//! Given two non-negative integers `low` and `high` (with `low <= high`), return how many odd
//! numbers lie in the closed interval `[low, high]`, that is, including both ends.
//!
//! Implement `Solution::count_odds(low: i32, high: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  low = 3, high = 7
//! Output: 3
//! Explanation: the odd numbers are 3, 5 and 7.
//!
//! Input:  low = 8, high = 10
//! Output: 1
//! Explanation: only 9.
//! ```
//!
//! ## Constraints
//!
//! - `0 <= low <= high <= 10^9`
//!
//! ## Hints
//!
//! 1. A loop over up to a billion numbers is too slow. Find a formula.
//! 2. How many odd numbers are in `[0, x]`? Try small values of `x` and look for a pattern with `x / 2`.

pub struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_odd_bounds() {
        assert_eq!(Solution::count_odds(3, 7), 3);
    }

    #[test]
    fn example_even_bounds() {
        assert_eq!(Solution::count_odds(8, 10), 1);
    }

    #[test]
    fn single_value_ranges() {
        assert_eq!(Solution::count_odds(0, 0), 0);
        assert_eq!(Solution::count_odds(1, 1), 1);
        assert_eq!(Solution::count_odds(2, 2), 0);
    }

    #[test]
    fn matches_brute_force() {
        for low in 0..30 {
            for high in low..30 {
                let expected = (low..=high).filter(|x| x % 2 == 1).count() as i32;
                assert_eq!(
                    Solution::count_odds(low, high),
                    expected,
                    "low={low}, high={high}"
                );
            }
        }
    }

    #[test]
    fn largest_range() {
        assert_eq!(Solution::count_odds(0, 1_000_000_000), 500_000_000);
    }
}
