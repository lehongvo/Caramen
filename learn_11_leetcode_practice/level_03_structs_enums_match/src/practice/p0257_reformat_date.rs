// ==================================================================
// PRACTICE 0257: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0257_reformat_date.md
// Run the tests:          cargo test practice::p0257
// ==================================================================

//! # Problem 0257: Reformat Date
//!
//! - **Difficulty:** Easy
//! - **Topics:** `match` on string slices, slice patterns on `split_whitespace` results, `trim_end_matches`, zero-padded `format!`
//! - **Source:** [LeetCode 1507. Reformat Date](https://leetcode.com/problems/reformat-date/)
//!
//! ## Problem
//!
//! A date is written as `"Day Month Year"`, for example `"20th Oct 2052"`, where:
//!
//! - `Day` is one of `"1st"`, `"2nd"`, `"3rd"`, `"4th"`, ..., `"30th"`, `"31st"`;
//! - `Month` is a three-letter English abbreviation: `"Jan"`, `"Feb"`, `"Mar"`, `"Apr"`, `"May"`,
//!   `"Jun"`, `"Jul"`, `"Aug"`, `"Sep"`, `"Oct"`, `"Nov"`, `"Dec"`;
//! - `Year` is a four-digit number between 1900 and 2100.
//!
//! Return the same date in the ISO format `"YYYY-MM-DD"`, with two-digit month and day.
//!
//! Implement `Solution::reformat_date(date: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  date = "20th Oct 2052"
//! Output: "2052-10-20"
//!
//! Input:  date = "6th Jun 1933"
//! Output: "1933-06-06"
//!
//! Input:  date = "26th May 1960"
//! Output: "1960-05-26"
//! ```
//!
//! ## Constraints
//!
//! - The date is valid.
//!
//! ## Hints
//!
//! 1. Collect the three words into a `Vec<&str>` and match it against the slice pattern
//!    `[day, month, year]`.
//! 2. A `match month { "Jan" => 1, "Feb" => 2, ... }` turns the name into a number.
//! 3. Strip the two-letter suffix from the day: every suffix is two ASCII letters.

pub struct Solution;

impl Solution {
    pub fn reformat_date(date: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn iso(d: &str) -> String {
        Solution::reformat_date(d.to_string())
    }

    #[test]
    fn example_one() {
        assert_eq!(iso("20th Oct 2052"), "2052-10-20");
    }

    #[test]
    fn example_two() {
        assert_eq!(iso("6th Jun 1933"), "1933-06-06");
    }

    #[test]
    fn example_three() {
        assert_eq!(iso("26th May 1960"), "1960-05-26");
    }

    #[test]
    fn all_suffixes() {
        assert_eq!(iso("1st Jan 1900"), "1900-01-01");
        assert_eq!(iso("2nd Feb 2000"), "2000-02-02");
        assert_eq!(iso("3rd Mar 2001"), "2001-03-03");
        assert_eq!(iso("31st Dec 2100"), "2100-12-31");
        assert_eq!(iso("22nd Sep 1999"), "1999-09-22");
    }

    #[test]
    fn every_month() {
        let names = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        for (i, name) in names.iter().enumerate() {
            assert_eq!(
                iso(&format!("10th {name} 2020")),
                format!("2020-{:02}-10", i + 1)
            );
        }
    }
}
