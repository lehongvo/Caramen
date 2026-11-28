//! # Problem 0159: Number of Segments in a String
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split(' ')` vs `split_whitespace`, filtering empty pieces, `count`
//! - **Source:** [LeetCode 434. Number of Segments in a String](https://leetcode.com/problems/number-of-segments-in-a-string/)
//!
//! ## Problem
//!
//! A *segment* is a maximal run of characters that are not the space character `' '`. Count the
//! segments of `s`. The string may be empty, and may start or end with spaces.
//!
//! Implement `Solution::count_segments(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "Hello, my name is John"
//! Output: 5               (the segments are "Hello,", "my", "name", "is", "John")
//!
//! Input:  s = "Hello"
//! Output: 1
//!
//! Input:  s = "   "
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= s.len() <= 300`
//! - `s` consists of English letters, digits and the characters `"!@#$%^&*()_+-=',.:"`, plus spaces.
//! - The only whitespace character in `s` is `' '`.
//!
//! ## Hints
//!
//! 1. `s.split(' ')` produces an empty piece for every extra space; filter those out.
//! 2. Because `' '` is the only whitespace here, `split_ascii_whitespace()` gives the same pieces.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** split at single spaces and count only the non-empty pieces. Leading, trailing and
//! repeated spaces each create empty pieces, which the filter removes.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `split(' ')` on `"a  b"` yields `"a"`, `""`, `"b"`: splitting keeps empty fields, which is what
//!   you want for CSV-like data but not for words.
//! - `split_whitespace` / `split_ascii_whitespace` skip empty fields automatically.
//! - `count()` returns `usize`; `as i32` is safe for at most 300 segments.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        // @solution-start
        s.split(' ').filter(|piece| !piece.is_empty()).count() as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> i32 {
        Solution::count_segments(s.to_string())
    }

    #[test]
    fn example_sentence() {
        assert_eq!(run("Hello, my name is John"), 5);
    }

    #[test]
    fn example_single_word() {
        assert_eq!(run("Hello"), 1);
    }

    #[test]
    fn example_only_spaces() {
        assert_eq!(run("   "), 0);
    }

    #[test]
    fn empty_string() {
        assert_eq!(run(""), 0);
    }

    #[test]
    fn punctuation_counts_as_segment_content() {
        assert_eq!(run("  , . !@# a  "), 4);
    }

    #[test]
    fn longest_input() {
        assert_eq!(run(&"ab ".repeat(100)), 100);
    }
}
