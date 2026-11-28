// ==================================================================
// PRACTICE 0241: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0241_day_of_the_year.md
// Run the tests:          cargo test practice::p0241
// ==================================================================

//! # Problem 0241: Day of the Year
//!
//! - **Difficulty:** Easy
//! - **Topics:** parsing into a `Date` struct, `let ... else`, methods on a struct, leap-year rule, `match` on month numbers
//! - **Source:** [LeetCode 1154. Day of the Year](https://leetcode.com/problems/day-of-the-year/)
//!
//! ## Problem
//!
//! You are given a Gregorian calendar date as a string `date` in the format `"YYYY-MM-DD"`. Return
//! its day number within its year: January 1st is day `1`, February 1st is day `32`, and so on.
//!
//! A year is a **leap year** (February has 29 days) if it is divisible by 4, except years divisible
//! by 100, which are leap years only if they are also divisible by 400 (2000 is a leap year, 1900 is
//! not).
//!
//! Implement `Solution::day_of_year(date: String) -> i32`.
//!
//! For this level, parse the string into a `struct Date { year, month, day }` first, and give the
//! struct methods such as `is_leap_year(&self)` and `day_of_year(&self)`.
//!
//! ## Examples
//!
//! ```text
//! Input:  date = "2019-01-09"
//! Output: 9
//!
//! Input:  date = "2019-02-10"
//! Output: 41     (31 days of January + 10)
//!
//! Input:  date = "2004-03-01"
//! Output: 61     (2004 is a leap year: 31 + 29 + 1)
//! ```
//!
//! ## Constraints
//!
//! - `date.len() == 10`, `date[4] == date[7] == '-'`, all other characters are digits.
//! - `date` is a valid date between January 1st, 1900 and December 31st, 2019.
//!
//! ## Hints
//!
//! 1. `date.split('-')` gives the three parts; `let (Some(y), Some(m), Some(d)) = (...) else { ... }`
//!    binds them all.
//! 2. A helper `days_in_month(year, month)` with a `match` on the month (`1 | 3 | 5 | ... => 31`)
//!    keeps the leap-year rule in one place.

pub struct Solution;

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_january() {
        assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
    }

    #[test]
    fn example_february() {
        assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
    }

    #[test]
    fn example_leap_year_march() {
        assert_eq!(Solution::day_of_year("2004-03-01".to_string()), 61);
    }

    #[test]
    fn century_rules() {
        // 1900 is not a leap year, 2000 is.
        assert_eq!(Solution::day_of_year("1900-03-01".to_string()), 60);
        assert_eq!(Solution::day_of_year("2000-03-01".to_string()), 61);
    }

    #[test]
    fn last_days_of_years() {
        assert_eq!(Solution::day_of_year("2019-12-31".to_string()), 365);
        assert_eq!(Solution::day_of_year("2016-12-31".to_string()), 366);
        assert_eq!(Solution::day_of_year("1900-01-01".to_string()), 1);
    }
}
