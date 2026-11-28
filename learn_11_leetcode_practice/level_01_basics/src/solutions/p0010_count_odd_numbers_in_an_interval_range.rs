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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the number of odd values in `[0, x]` is `(x + 1) / 2` (integer division rounds down):
//! for `x = 7` that is 4 (1, 3, 5, 7). Odd numbers in `[low, high]` = odd numbers in `[0, high]`
//! minus odd numbers in `[0, low - 1]`, which is `(high + 1) / 2 - low / 2`.
//!
//! **Complexity:** `O(1)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - Integer `/` discards the fractional part, which makes "count multiples up to x" formulas
//!   (`x / k`) natural to write.
//! - `high + 1` is at most `10^9 + 1`, far below `i32::MAX` (about `2.1 * 10^9`), so no overflow.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        // @solution-start
        (high + 1) / 2 - low / 2
        // @solution-end
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
