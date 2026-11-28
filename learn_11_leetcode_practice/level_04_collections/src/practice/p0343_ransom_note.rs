// ==================================================================
// PRACTICE 0343: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0343_ransom_note.md
// Run the tests:          cargo test practice::p0343
// ==================================================================

//! # Problem 0343: Ransom Note
//!
//! - **Difficulty:** Easy
//! - **Topics:** count map with `entry`, consuming counts with `get_mut`, `Iterator::all`
//! - **Source:** [LeetCode 383. Ransom Note](https://leetcode.com/problems/ransom-note/)
//!
//! ## Problem
//!
//! You want to write the text `ransom_note` by cutting letters out of `magazine`. Each letter of
//! the magazine can be used **at most once**. Return `true` if the note can be written, and
//! `false` otherwise.
//!
//! Implement `Solution::can_construct(ransom_note: String, magazine: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  ransom_note = "a", magazine = "b"
//! Output: false
//!
//! Input:  ransom_note = "aa", magazine = "ab"
//! Output: false     (only one 'a' available)
//!
//! Input:  ransom_note = "aa", magazine = "aab"
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= ransom_note.len(), magazine.len() <= 10^5`
//! - Both strings contain only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Count the letters available in the magazine.
//! 2. For each letter of the note, take one from the stock; if the stock is missing or empty, the
//!    note is impossible.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
    }

    #[test]
    fn example_two_not_enough_copies() {
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
    }

    #[test]
    fn example_three() {
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }

    #[test]
    fn note_equals_magazine_rearranged() {
        assert!(Solution::can_construct(
            "abc".to_string(),
            "cba".to_string()
        ));
        assert!(!Solution::can_construct(
            "abcd".to_string(),
            "cba".to_string()
        ));
    }

    #[test]
    fn large_inputs() {
        let magazine = "abcdefghijklmnopqrstuvwxyz".repeat(3_846);
        let note = "z".repeat(3_846);
        assert!(Solution::can_construct(note.clone(), magazine.clone()));
        let too_many = format!("{note}z");
        assert!(!Solution::can_construct(too_many, magazine));
    }
}
