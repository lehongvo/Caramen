// ==================================================================
// PRACTICE 0825: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0825_edit_distance.md
// Run the tests:          cargo test practice::p0825
// ==================================================================

//! # Problem 0825: Edit Distance
//!
//! - **Difficulty:** Medium
//! - **Topics:** Levenshtein distance, 2-D DP with rolling rows, `(0..=n).collect()`
//! - **Source:** [LeetCode 72. Edit Distance](https://leetcode.com/problems/edit-distance/)
//!
//! ## Problem
//!
//! Given two strings `word1` and `word2`, return the minimum number of single-character operations
//! that turn `word1` into `word2`. The allowed operations are:
//!
//! - **insert** a character;
//! - **delete** a character;
//! - **replace** a character with another one.
//!
//! Implement `Solution::min_distance(word1: String, word2: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  word1 = "horse", word2 = "ros"
//! Output: 3
//! Explanation: horse -> rorse (replace 'h') -> rose (delete 'r') -> ros (delete 'e').
//!
//! Input:  word1 = "intention", word2 = "execution"
//! Output: 5
//!
//! Input:  word1 = "", word2 = "abc"
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `0 <= word1.len(), word2.len() <= 500`
//! - Both strings contain only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Let `D(i, j)` be the distance between `word1[..i]` and `word2[..j]`. Turning a prefix into
//!    the empty string (or back) costs its length: `D(i, 0) = i`, `D(0, j) = j`.
//! 2. If the last characters are equal, `D(i, j) = D(i - 1, j - 1)`. Otherwise it is
//!    `1 + min(replace, delete, insert)` = `1 + min(D(i-1, j-1), D(i-1, j), D(i, j-1))`.

pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dist(a: &str, b: &str) -> i32 {
        Solution::min_distance(a.to_string(), b.to_string())
    }

    #[test]
    fn example_horse_ros() {
        assert_eq!(dist("horse", "ros"), 3);
    }

    #[test]
    fn example_intention_execution() {
        assert_eq!(dist("intention", "execution"), 5);
    }

    #[test]
    fn empty_strings() {
        assert_eq!(dist("", "abc"), 3);
        assert_eq!(dist("abc", ""), 3);
        assert_eq!(dist("", ""), 0);
    }

    #[test]
    fn classic_kitten_sitting() {
        assert_eq!(dist("kitten", "sitting"), 3);
        assert_eq!(dist("same", "same"), 0);
    }

    #[test]
    fn long_completely_different() {
        assert_eq!(dist(&"a".repeat(500), &"b".repeat(500)), 500);
    }
}
