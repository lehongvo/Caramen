// ==================================================================
// PRACTICE 0471: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0471_cells_in_a_range_on_an_excel_sheet.md
// Run the tests:          cargo test practice::p0471
// ==================================================================

//! # Problem 0471: Cells in a Range on an Excel Sheet
//!
//! - **Difficulty:** Easy
//! - **Topics:** `char` ranges as iterators, nested `flat_map` with an inner `move` closure, `format!`
//! - **Source:** [LeetCode 2194. Cells in a Range on an Excel Sheet](https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/)
//!
//! ## Problem
//!
//! A spreadsheet cell is named by a column letter followed by a row digit, for example `"K1"`. A
//! range `"<c1><r1>:<c2><r2>"` such as `"K1:L2"` contains every cell whose column is between `c1`
//! and `c2` and whose row is between `r1` and `r2` (inclusive).
//!
//! Return the names of all cells in the range, sorted by column first and then by row.
//!
//! Implement `Solution::cells_in_range(s: String) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "K1:L2"
//! Output: ["K1", "K2", "L1", "L2"]
//!
//! Input:  s = "A1:F1"
//! Output: ["A1", "B1", "C1", "D1", "E1", "F1"]
//! ```
//!
//! ## Constraints
//!
//! - `s.len() == 5`
//! - `'A' <= s[0] <= s[3] <= 'Z'`
//! - `'1' <= s[1] <= s[4] <= '9'`
//! - `s[2] == ':'`
//!
//! ## Hints
//!
//! 1. `('A'..='C')` is an iterator over characters, just like `1..=3` is over numbers.
//! 2. For each column, produce all rows: `flat_map(|col| (r1..=r2).map(move |row| ...))`.

pub struct Solution;

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_by_two() {
        assert_eq!(
            Solution::cells_in_range("K1:L2".to_string()),
            vec!["K1", "K2", "L1", "L2"]
        );
    }

    #[test]
    fn example_one_row() {
        assert_eq!(
            Solution::cells_in_range("A1:F1".to_string()),
            vec!["A1", "B1", "C1", "D1", "E1", "F1"]
        );
    }

    #[test]
    fn single_cell_and_column() {
        assert_eq!(Solution::cells_in_range("C3:C3".to_string()), vec!["C3"]);
        assert_eq!(
            Solution::cells_in_range("Z7:Z9".to_string()),
            vec!["Z7", "Z8", "Z9"]
        );
    }

    #[test]
    fn whole_sheet() {
        let all = Solution::cells_in_range("A1:Z9".to_string());
        assert_eq!(all.len(), 26 * 9);
        assert_eq!(all[0], "A1");
        assert_eq!(all[9], "B1");
        assert_eq!(all.last().map(String::as_str), Some("Z9"));
    }
}
