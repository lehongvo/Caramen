// ==================================================================
// PRACTICE 0193: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0193_string_compression.md
// Run the tests:          cargo test practice::p0193
// ==================================================================

//! # Problem 0193: String Compression
//!
//! - **Difficulty:** Medium
//! - **Topics:** in-place rewrite of a `&mut Vec<char>` with read/write indices, `to_string().chars()`, `truncate`
//! - **Source:** [LeetCode 443. String Compression](https://leetcode.com/problems/string-compression/)
//!
//! ## Problem
//!
//! Compress the character vector `chars` **in place**. Every group of consecutive equal characters is
//! replaced by the character followed by the group length written in decimal, except that groups of
//! length 1 are written as just the character. A length of 10 or more takes several characters
//! (`12` becomes `'1', '2'`).
//!
//! Return the new length `k`; the compressed form must be stored in `chars[..k]`. Use only `O(1)`
//! extra space (besides the few characters of a group length).
//!
//! Implement `Solution::compress(chars: &mut Vec<char>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  chars = ['a', 'a', 'b', 'b', 'c', 'c', 'c']
//! Output: 6, chars starts with ['a', '2', 'b', '2', 'c', '3']
//!
//! Input:  chars = ['a']
//! Output: 1, chars starts with ['a']
//!
//! Input:  chars = ['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b']
//! Output: 4, chars starts with ['a', 'b', '1', '2']
//! ```
//!
//! ## Constraints
//!
//! - `1 <= chars.len() <= 2000`
//! - `chars[i]` is an English letter, a digit or a symbol.
//!
//! ## Hints
//!
//! 1. Use a `read` index to find the end of each group and a `write` index for the output.
//! 2. The compressed form of a group is never longer than the group itself, so `write` never passes
//!    `read`: you only overwrite characters you have already read.
//! 3. `count.to_string()` gives the digits of a group length.

pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(text: &str) -> (i32, String) {
        let mut chars: Vec<char> = text.chars().collect();
        let k = Solution::compress(&mut chars);
        (k, chars[..k as usize].iter().collect())
    }

    #[test]
    fn example_three_groups() {
        assert_eq!(run("aabbccc"), (6, "a2b2c3".to_string()));
    }

    #[test]
    fn example_single_char() {
        assert_eq!(run("a"), (1, "a".to_string()));
    }

    #[test]
    fn example_two_digit_count() {
        assert_eq!(run("abbbbbbbbbbbb"), (4, "ab12".to_string()));
    }

    #[test]
    fn no_repeats_and_symbols() {
        assert_eq!(run("ab#!"), (4, "ab#!".to_string()));
        assert_eq!(run("##"), (2, "#2".to_string()));
    }

    #[test]
    fn digits_as_characters() {
        assert_eq!(run("1111222"), (4, "1423".to_string()));
    }

    #[test]
    fn longest_input() {
        let text = format!("{}{}", "x".repeat(1_000), "y".repeat(1_000));
        assert_eq!(run(&text), (10, "x1000y1000".to_string()));
    }
}
