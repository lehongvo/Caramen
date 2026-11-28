// ==================================================================
// PRACTICE 0716: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0716_longest_repeating_character_replacement.md
// Run the tests:          cargo test practice::p0716
// ==================================================================

//! # Problem 0716: Longest Repeating Character Replacement
//!
//! - **Difficulty:** Medium
//! - **Topics:** sliding window, count array `[usize; 26]`, the "window never shrinks" trick
//! - **Source:** [LeetCode 424. Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/)
//!
//! ## Problem
//!
//! You are given a string `s` of uppercase English letters and an integer `k`. You may perform at
//! most `k` operations; one operation replaces any single character of `s` by any other uppercase
//! letter. Return the length of the longest substring made of one repeated letter that you can
//! obtain.
//!
//! Implement `Solution::character_replacement(s: String, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "ABAB", k = 2
//! Output: 4             (replace both 'A's, or both 'B's)
//!
//! Input:  s = "AABABBA", k = 1
//! Output: 4             (replace the 'A' at index 3: "AABBBBA" contains "BBBB")
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^5`
//! - `s` contains only uppercase English letters.
//! - `0 <= k <= s.len()`
//!
//! ## Hints
//!
//! 1. A window can be made uniform when `window length - (count of its most frequent letter) <= k`.
//! 2. You only care about windows **longer** than the best one found so far. Does the window ever
//!    need to shrink?

pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, k: i32) -> i32 {
        Solution::character_replacement(s.to_string(), k)
    }

    #[test]
    fn example_replace_two() {
        assert_eq!(run("ABAB", 2), 4);
    }

    #[test]
    fn example_replace_one() {
        assert_eq!(run("AABABBA", 1), 4);
    }

    #[test]
    fn single_letter() {
        assert_eq!(run("A", 0), 1);
    }

    #[test]
    fn no_replacements() {
        assert_eq!(run("ABCDE", 0), 1);
        assert_eq!(run("ABBBCC", 0), 3);
    }

    #[test]
    fn enough_replacements_for_everything() {
        assert_eq!(run("ABCDE", 5), 5);
    }

    #[test]
    fn best_window_in_the_middle() {
        // "BBABB" (replace the 'A') is the best block.
        assert_eq!(run("ACBBABBCA", 1), 5);
    }

    #[test]
    fn long_alternating_input() {
        let s = "AB".repeat(50_000);
        assert_eq!(run(&s, 0), 1);
        assert_eq!(run(&s, 50_000), 100_000);
    }
}
