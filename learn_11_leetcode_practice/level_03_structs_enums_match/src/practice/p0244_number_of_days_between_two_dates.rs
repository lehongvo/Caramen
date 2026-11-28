// ==================================================================
// PRACTICE 0244: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0244_number_of_days_between_two_dates.md
// Run the tests:          cargo test practice::p0244
// ==================================================================

//! # Problem 0244: Number of Days Between Two Dates
//!
//! - **Difficulty:** Easy
//! - **Topics:** a `Date` struct with a "days since epoch" method, parsing with `?`, `abs_diff`, `match` with a guard
//! - **Source:** [LeetCode 1360. Number of Days Between Two Dates](https://leetcode.com/problems/number-of-days-between-two-dates/)
//!
//! ## Problem
//!
//! Given two valid dates `date1` and `date2` as `"YYYY-MM-DD"` strings, return the number of days
//! between them: the absolute difference, so the order of the arguments does not matter. Use the
//! Gregorian leap-year rule (divisible by 4, except centuries not divisible by 400).
//!
//! Implement `Solution::days_between_dates(date1: String, date2: String) -> i32`.
//!
//! For this level, write a `Date` struct with a method `days_since_1971(&self) -> u32` (the number
//! of days from January 1st, 1971 to the date), and compute the answer from two such numbers.
//!
//! ## Examples
//!
//! ```text
//! Input:  date1 = "2019-06-29", date2 = "2019-06-30"
//! Output: 1
//!
//! Input:  date1 = "2020-01-15", date2 = "2019-12-31"
//! Output: 15
//!
//! Input:  date1 = "1971-01-01", date2 = "2100-12-31"
//! Output: 47481
//! ```
//!
//! ## Constraints
//!
//! - Both dates are valid and lie between the years `1971` and `2100` (inclusive).
//!
//! ## Hints
//!
//! 1. Count whole years from 1971 up to (not including) the date's year, then whole months of the
//!    date's year, then the day.
//! 2. Converting both dates to a single number turns "difference of dates" into a subtraction.

pub struct Solution;

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn days(a: &str, b: &str) -> i32 {
        Solution::days_between_dates(a.to_string(), b.to_string())
    }

    #[test]
    fn example_consecutive_days() {
        assert_eq!(days("2019-06-29", "2019-06-30"), 1);
    }

    #[test]
    fn example_across_new_year_reversed() {
        assert_eq!(days("2020-01-15", "2019-12-31"), 15);
    }

    #[test]
    fn whole_range() {
        assert_eq!(days("1971-01-01", "2100-12-31"), 47_481);
    }

    #[test]
    fn same_date_and_leap_days() {
        assert_eq!(days("2000-02-28", "2000-02-28"), 0);
        assert_eq!(days("2000-02-28", "2000-03-01"), 2);
        assert_eq!(days("2100-02-28", "2100-03-01"), 1);
    }

    #[test]
    fn full_years() {
        assert_eq!(days("2019-01-01", "2020-01-01"), 365);
        assert_eq!(days("2020-01-01", "2021-01-01"), 366);
    }
}
