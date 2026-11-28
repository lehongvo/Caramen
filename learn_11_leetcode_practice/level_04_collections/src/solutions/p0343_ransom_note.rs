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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build a stock of magazine letters in a `HashMap<char, usize>`. Then spend one unit of
//! stock per letter of the note. `get_mut` returns a mutable reference to the count, so the check
//! and the decrement touch the map only once per letter.
//!
//! **Complexity:** `O(n + m)` average time, `O(1)` space (at most 26 keys).
//!
//! **Rust concepts:**
//! - `match stock.get_mut(&c)` distinguishes three cases: missing key (`None`), empty stock
//!   (`Some(0)`), and available stock (`Some(count)`, which we decrement through `*count -= 1`).
//! - `all` stops at the first letter that cannot be supplied.
//! - The match arm `Some(count) if *count > 0` uses a guard on the dereferenced value.
// @solution-end

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // @solution-start
        let mut stock: HashMap<char, usize> = HashMap::new();
        for c in magazine.chars() {
            *stock.entry(c).or_insert(0) += 1;
        }
        ransom_note.chars().all(|c| match stock.get_mut(&c) {
            Some(count) if *count > 0 => {
                *count -= 1;
                true
            }
            _ => false,
        })
        // @solution-end
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
