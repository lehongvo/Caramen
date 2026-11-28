// ==================================================================
// PRACTICE 0336: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0336_path_crossing.md
// Run the tests:          cargo test practice::p0336
// ==================================================================

//! # Problem 0336: Path Crossing
//!
//! - **Difficulty:** Easy
//! - **Topics:** `HashSet<(i32, i32)>` of tuples, `insert` returns `bool`, `match` on `char`
//! - **Source:** [LeetCode 1496. Path Crossing](https://leetcode.com/problems/path-crossing/)
//!
//! ## Problem
//!
//! A walker starts at the origin `(0, 0)` of a grid and follows the string `path`. Each character
//! moves one unit: `'N'` north (y + 1), `'S'` south (y - 1), `'E'` east (x + 1), `'W'` west
//! (x - 1). Return `true` if the walker ever stands on a point it has already visited (including
//! the origin), and `false` otherwise.
//!
//! Implement `Solution::is_path_crossing(path: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  path = "NES"
//! Output: false     (visits (0,0), (0,1), (1,1), (1,0): all different)
//!
//! Input:  path = "NESWW"
//! Output: true      (the 4th move returns to (0,0))
//! ```
//!
//! ## Constraints
//!
//! - `1 <= path.len() <= 10^4`
//! - `path` contains only `'N'`, `'S'`, `'E'` and `'W'`.
//!
//! ## Hints
//!
//! 1. Tuples of hashable types are hashable, so a `HashSet<(i32, i32)>` can store points.
//! 2. Insert the origin first; then a failed `insert` after a move means a repeat visit.

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_no_crossing() {
        assert!(!Solution::is_path_crossing("NES".to_string()));
    }

    #[test]
    fn example_crossing_at_origin() {
        assert!(Solution::is_path_crossing("NESWW".to_string()));
    }

    #[test]
    fn going_back_immediately() {
        assert!(Solution::is_path_crossing("NS".to_string()));
        assert!(!Solution::is_path_crossing("W".to_string()));
    }

    #[test]
    fn long_straight_line_and_late_crossing() {
        assert!(!Solution::is_path_crossing("E".repeat(10_000)));
        let path = format!("{}N{}S", "E".repeat(4_999), "W".repeat(4_999));
        // Returns to the start column one row up, then steps down onto the origin.
        assert!(Solution::is_path_crossing(path));
    }
}
