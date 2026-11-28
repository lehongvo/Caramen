// ==================================================================
// PRACTICE 0256: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0256_day_of_the_week.md
// Run the tests:          cargo test practice::p0256
// ==================================================================

//! # Problem 0256: Day of the Week
//!
//! - **Difficulty:** Easy
//! - **Topics:** a `Weekday` enum with `name()`, converting a count into an enum, `rem_euclid`, leap years
//! - **Source:** [LeetCode 1185. Day of the Week](https://leetcode.com/problems/day-of-the-week/)
//!
//! ## Problem
//!
//! Given a valid date as three integers `day`, `month` and `year`, return the day of the week it
//! falls on, as one of `"Sunday"`, `"Monday"`, `"Tuesday"`, `"Wednesday"`, `"Thursday"`,
//! `"Friday"`, `"Saturday"`.
//!
//! Implement `Solution::day_of_the_week(day: i32, month: i32, year: i32) -> String`.
//!
//! Useful fact: January 1st, 1971 was a **Friday**. Leap years follow the Gregorian rule.
//!
//! For this level, compute a `Weekday` enum value first and convert it to text with a
//! `name(self) -> &'static str` method.
//!
//! ## Examples
//!
//! ```text
//! Input:  day = 31, month = 8, year = 2019
//! Output: "Saturday"
//!
//! Input:  day = 18, month = 7, year = 1999
//! Output: "Sunday"
//!
//! Input:  day = 15, month = 8, year = 1993
//! Output: "Sunday"
//! ```
//!
//! ## Constraints
//!
//! - The date is valid and between the years `1971` and `2100`.
//!
//! ## Hints
//!
//! 1. Count the days from 1971-01-01 to the date; that count modulo 7 tells you how many weekdays
//!    to move forward from Friday.
//! 2. `Weekday::from_offset(n)` can map `0..7` to the variants with a `match`.

pub struct Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday");
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday");
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday");
    }

    #[test]
    fn range_ends() {
        assert_eq!(Solution::day_of_the_week(1, 1, 1971), "Friday");
        assert_eq!(Solution::day_of_the_week(31, 12, 2100), "Friday");
    }

    #[test]
    fn leap_days() {
        assert_eq!(Solution::day_of_the_week(29, 2, 2000), "Tuesday");
        assert_eq!(Solution::day_of_the_week(1, 3, 2000), "Wednesday");
        assert_eq!(Solution::day_of_the_week(29, 2, 2024), "Thursday");
    }
}
