// ==================================================================
// PRACTICE 0198: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0198_zigzag_conversion.md
// Run the tests:          cargo test practice::p0198
// ==================================================================

//! # Problem 0198: Zigzag Conversion
//!
//! - **Difficulty:** Medium
//! - **Topics:** a `Vec<String>` of rows, `vec![String::new(); n]`, direction flag, `concat`
//! - **Source:** [LeetCode 6. Zigzag Conversion](https://leetcode.com/problems/zigzag-conversion/)
//!
//! ## Problem
//!
//! Write the characters of `s` in a zigzag over `num_rows` rows: go down the rows one character at a
//! time, then diagonally back up to the first row, then down again, and so on. Then read the rows
//! from top to bottom, each from left to right, and return the resulting string.
//!
//! For `s = "PAYPALISHIRING"` and `num_rows = 3` the zigzag is:
//!
//! ```text
//! P   A   H   N
//! A P L S I I G
//! Y   I   R
//! ```
//!
//! and reading the rows gives `"PAHNAPLSIIGYIR"`.
//!
//! Implement `Solution::convert(s: String, num_rows: i32) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "PAYPALISHIRING", num_rows = 3
//! Output: "PAHNAPLSIIGYIR"
//!
//! Input:  s = "PAYPALISHIRING", num_rows = 4
//! Output: "PINALSIGYAHRPI"
//!
//! Input:  s = "A", num_rows = 1
//! Output: "A"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 1000`
//! - `s` consists of English letters, `','` and `'.'`.
//! - `1 <= num_rows <= 1000`
//!
//! ## Hints
//!
//! 1. You do not need the 2D picture: keep one `String` per row and remember the current row.
//! 2. The direction flips when you reach the first or the last row.
//! 3. With a single row (or at least as many rows as characters), the answer is `s` itself.

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, rows: i32) -> String {
        Solution::convert(s.to_string(), rows)
    }

    #[test]
    fn example_three_rows() {
        assert_eq!(run("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn example_four_rows() {
        assert_eq!(run("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
    }

    #[test]
    fn example_single_row() {
        assert_eq!(run("A", 1), "A");
        assert_eq!(run("hello", 1), "hello");
    }

    #[test]
    fn two_rows_and_too_many_rows() {
        assert_eq!(run("ABCDEF", 2), "ACEBDF");
        assert_eq!(run("AB,C.", 1000), "AB,C.");
    }

    #[test]
    fn longest_input() {
        let s: String = (0..1000).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let result = run(&s, 7);
        assert_eq!(result.len(), 1000);
        // Row 0 holds every 12th character (cycle length 2 * (7 - 1)).
        let row0: String = s.chars().step_by(12).collect();
        assert!(result.starts_with(&row0));
    }
}
