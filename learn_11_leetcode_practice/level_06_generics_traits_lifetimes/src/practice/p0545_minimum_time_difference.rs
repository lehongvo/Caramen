// ==================================================================
// PRACTICE 0545: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0545_minimum_time_difference.md
// Run the tests:          cargo test practice::p0545
// ==================================================================

//! # Problem 0545: Minimum Time Difference
//!
//! - **Difficulty:** Medium
//! - **Topics:** `FromStr` for a clock-time newtype, derived `Ord` on a newtype, sorting, circular differences
//! - **Source:** [LeetCode 539. Minimum Time Difference](https://leetcode.com/problems/minimum-time-difference/)
//!
//! ## Problem
//!
//! You are given at least two times of day in 24-hour `"HH:MM"` format. Return the smallest difference,
//! in minutes, between any two of them. The clock is **circular**: `"23:59"` and `"00:00"` are 1 minute
//! apart.
//!
//! Implement `Solution::find_min_difference(time_points: Vec<String>) -> i32`.
//!
//! Parse the strings into a newtype `TimeOfDay(pub u16)` that stores minutes since midnight. Implement
//! `FromStr for TimeOfDay` with `type Err = String`: accept exactly two digits, a colon and two digits,
//! with hours in `0..=23` and minutes in `0..=59`; anything else returns an error message (any text).
//! `TimeOfDay` derives `PartialOrd`/`Ord`, so a `Vec<TimeOfDay>` can be sorted directly.
//!
//! ## Examples
//!
//! ```text
//! Input:  ["23:59", "00:00"]            Output: 1
//! Input:  ["00:00", "23:59", "00:00"]   Output: 0       (a duplicate)
//! Input:  ["01:00", "13:00", "19:30"]   Output: 330     (19:30 -> 01:00 across midnight)
//!
//! "07:05".parse::<TimeOfDay>()          => Ok(TimeOfDay(425))
//! "24:00".parse::<TimeOfDay>()          => Err(..)
//! ```
//!
//! ## Constraints
//!
//! - `2 <= time_points.len() <= 2 * 10^4`
//! - Every element of `time_points` is a valid `"HH:MM"` time.
//!
//! ## Hints
//!
//! 1. After sorting, the minimum difference is between two neighbours, or between the last and the
//!    first time going across midnight (`first + 1440 - last`).
//! 2. There are only 1440 distinct times, so more than 1440 inputs must contain a duplicate.
//! 3. `s.split_once(':')` splits `"HH:MM"` into `("HH", "MM")`.

use std::str::FromStr;

pub struct Solution;

/// Minutes since midnight, `0..1440`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeOfDay(pub u16);

impl FromStr for TimeOfDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Write your solution here")
    }
}

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_inputs() {
        assert_eq!(
            Solution::find_min_difference(strings(&["23:59", "00:00"])),
            1
        );
        assert_eq!(
            Solution::find_min_difference(strings(&["00:00", "23:59", "00:00"])),
            0
        );
        assert_eq!(
            Solution::find_min_difference(strings(&["01:00", "13:00", "19:30"])),
            330
        );
    }

    #[test]
    fn parsing_times() {
        assert_eq!("07:05".parse::<TimeOfDay>(), Ok(TimeOfDay(425)));
        assert_eq!("00:00".parse::<TimeOfDay>(), Ok(TimeOfDay(0)));
        assert_eq!("23:59".parse::<TimeOfDay>(), Ok(TimeOfDay(1439)));
        for bad in [
            "24:00", "12:60", "7:05", "07-05", "ab:cd", "07:5", "+1:00", "",
        ] {
            assert!(
                bad.parse::<TimeOfDay>().is_err(),
                "{bad:?} should be rejected"
            );
        }
    }

    #[test]
    fn noon_opposites() {
        assert_eq!(
            Solution::find_min_difference(strings(&["00:00", "12:00"])),
            720
        );
        assert_eq!(
            Solution::find_min_difference(strings(&["05:31", "22:08", "00:35"])),
            147
        );
    }

    #[test]
    fn many_times() {
        let all: Vec<String> = (0..1440)
            .map(|m| format!("{:02}:{:02}", m / 60, m % 60))
            .collect();
        let every_seventh: Vec<String> = all.iter().step_by(7).cloned().collect();
        // 1440 = 7 * 205 + 5, so the wrap-around gap is 5 minutes.
        assert_eq!(Solution::find_min_difference(every_seventh), 5);
        let mut with_duplicates = all.clone();
        with_duplicates.extend(all);
        assert_eq!(Solution::find_min_difference(with_duplicates), 0);
    }
}
