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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** count days since 1971-01-01 (whole years, whole months, then the day), add that count
//! to Friday's position in the week, reduce modulo 7 and name the result.
//!
//! **Complexity:** `O(Y)` time where `Y <= 130` is the number of years since 1971; `O(1)` space.
//!
//! **Rust concepts:**
//! - An enum of the seven days with a `name` method keeps the strings in one place and makes the
//!   arithmetic produce a typed value first.
//! - `Weekday::from_offset` uses a `match` on `0..7`; the final `_` arm is unreachable in practice
//!   because the argument is always reduced modulo 7, so it returns the last variant.
//! - `String::from(weekday.name())` converts the `&'static str` into the owned `String` LeetCode wants.
// @solution-end

pub struct Solution;

// @solution-start remove
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Weekday {
    /// `0` is Sunday, `6` is Saturday; callers pass a value already reduced modulo 7.
    fn from_offset(n: u32) -> Weekday {
        match n % 7 {
            0 => Weekday::Sunday,
            1 => Weekday::Monday,
            2 => Weekday::Tuesday,
            3 => Weekday::Wednesday,
            4 => Weekday::Thursday,
            5 => Weekday::Friday,
            _ => Weekday::Saturday,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Weekday::Sunday => "Sunday",
            Weekday::Monday => "Monday",
            Weekday::Tuesday => "Tuesday",
            Weekday::Wednesday => "Wednesday",
            Weekday::Thursday => "Thursday",
            Weekday::Friday => "Friday",
            Weekday::Saturday => "Saturday",
        }
    }
}

fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_since_1971(day: u32, month: u32, year: u32) -> u32 {
    const MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let years: u32 = (1971..year)
        .map(|y| if is_leap(y) { 366 } else { 365 })
        .sum();
    let months: u32 = MONTH_DAYS.iter().take(month as usize - 1).sum();
    let leap_day = u32::from(month > 2 && is_leap(year));
    years + months + leap_day + day - 1
}
// @solution-end

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        // @solution-start
        // All inputs are positive by the constraints.
        let offset = days_since_1971(day as u32, month as u32, year as u32);
        // 1971-01-01 was a Friday, which is offset 5 from Sunday.
        let weekday = Weekday::from_offset((offset + 5) % 7);
        String::from(weekday.name())
        // @solution-end
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
