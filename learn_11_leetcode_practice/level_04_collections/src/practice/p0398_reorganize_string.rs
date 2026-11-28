// ==================================================================
// PRACTICE 0398: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0398_reorganize_string.md
// Run the tests:          cargo test practice::p0398
// ==================================================================

//! # Problem 0398: Reorganize String
//!
//! - **Difficulty:** Medium
//! - **Topics:** max-heap of `(count, byte)`, holding the previous character back with `Option::take`, impossible-case detection
//! - **Source:** [LeetCode 767. Reorganize String](https://leetcode.com/problems/reorganize-string/)
//!
//! ## Problem
//!
//! Rearrange the characters of `s` so that no two **adjacent** characters are equal. Return any
//! such rearrangement, or `""` if none exists.
//!
//! Implement `Solution::reorganize_string(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "aab"
//! Output: "aba"
//!
//! Input:  s = "aaab"
//! Output: ""        (three 'a's need at least two separators)
//!
//! Input:  s = "vvvlo"
//! Output: "vlvov"   ("vovlv" is also accepted)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 500`
//! - `s` contains lowercase English letters.
//!
//! ## Hints
//!
//! 1. Greedy: always place the character with the most remaining copies, unless it was placed just
//!    before.
//! 2. Pop the top of a max-heap of `(count, character)`, append it, and keep it **out** of the heap
//!    for one round; put the previously held-back character back in.
//! 3. If the heap runs empty while a character is still held back, the task is impossible.

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(s: &str, result: &str) -> bool {
        let mut a: Vec<u8> = s.bytes().collect();
        let mut b: Vec<u8> = result.bytes().collect();
        a.sort_unstable();
        b.sort_unstable();
        a == b && result.as_bytes().windows(2).all(|w| w[0] != w[1])
    }

    fn check_possible(s: &str) {
        let result = Solution::reorganize_string(s.to_string());
        assert!(is_valid(s, &result), "invalid answer {result:?} for {s:?}");
    }

    #[test]
    fn example_one() {
        check_possible("aab");
    }

    #[test]
    fn example_two_impossible() {
        assert_eq!(Solution::reorganize_string("aaab".to_string()), "");
    }

    #[test]
    fn example_three() {
        assert!(!is_valid("vvvlo", "vvlov"));
        check_possible("vvvlo");
    }

    #[test]
    fn single_and_borderline() {
        check_possible("z");
        check_possible("aabbcc");
        // 3 'a's out of 5: exactly (5 + 1) / 2, still possible ("ababa").
        check_possible("aaabb");
        assert_eq!(Solution::reorganize_string("aa".to_string()), "");
    }

    #[test]
    fn long_strings() {
        // 250 'a's and 250 other letters: exactly at the limit (500 + 1) / 2 = 250.
        let possible = format!("{}{}", "a".repeat(250), "bcdefghij".repeat(28));
        check_possible(&possible[..500]);
        let impossible = format!("{}{}", "q".repeat(251), "r".repeat(249));
        assert_eq!(Solution::reorganize_string(impossible), "");
    }
}
