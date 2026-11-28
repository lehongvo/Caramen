// ==================================================================
// PRACTICE 0473: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0473_excel_sheet_column_title.md
// Run the tests:          cargo test practice::p0473
// ==================================================================

//! # Problem 0473: Excel Sheet Column Title
//!
//! - **Difficulty:** Easy
//! - **Topics:** `iter::successors` for digit extraction, bijective base 26, `rev`, collecting `char`s into a `String`
//! - **Source:** [LeetCode 168. Excel Sheet Column Title](https://leetcode.com/problems/excel-sheet-column-title/)
//!
//! ## Problem
//!
//! Spreadsheet columns are named `A, B, ..., Z, AA, AB, ..., AZ, BA, ..., ZZ, AAA, ...`. Column
//! number `1` is `"A"`, `26` is `"Z"`, `27` is `"AA"` and `28` is `"AB"`.
//!
//! Given a column number, return its title.
//!
//! Implement `Solution::convert_to_title(column_number: i32) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  column_number = 1
//! Output: "A"
//!
//! Input:  column_number = 28
//! Output: "AB"
//!
//! Input:  column_number = 701
//! Output: "ZY"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= column_number <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. This is "base 26" with digits `A..=Z` meaning `1..=26` (there is no zero digit). Subtracting
//!    `1` first turns it into ordinary base 26: the last letter is `(n - 1) % 26`, and the rest of
//!    the title is the title of `(n - 1) / 26`.
//! 2. `iter::successors(Some(n), |&n| ...)` can produce `n`, then `(n - 1) / 26`, ... until `0`.
//! 3. The letters come out last-to-first; reverse them.

pub struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::convert_to_title(1), "A");
    }

    #[test]
    fn example_twenty_eight() {
        assert_eq!(Solution::convert_to_title(28), "AB");
    }

    #[test]
    fn example_seven_hundred_one() {
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }

    #[test]
    fn boundaries_between_lengths() {
        assert_eq!(Solution::convert_to_title(26), "Z");
        assert_eq!(Solution::convert_to_title(27), "AA");
        assert_eq!(Solution::convert_to_title(52), "AZ");
        assert_eq!(Solution::convert_to_title(702), "ZZ");
        assert_eq!(Solution::convert_to_title(703), "AAA");
        assert_eq!(Solution::convert_to_title(i32::MAX), "FXSHRXW");
    }
}
