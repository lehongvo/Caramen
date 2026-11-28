// ==================================================================
// PRACTICE 0261: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0261_utf_8_validation.md
// Run the tests:          cargo test practice::p0261
// ==================================================================

//! # Problem 0261: UTF-8 Validation
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary literal range patterns, an `enum` state machine, `match` on `(state, byte)`, `fold`-like loop
//! - **Source:** [LeetCode 393. UTF-8 Validation](https://leetcode.com/problems/utf-8-validation/)
//!
//! ## Problem
//!
//! In UTF-8, one character is encoded as 1 to 4 bytes, recognised by their leading bits:
//!
//! ```text
//! 1 byte:  0xxxxxxx
//! 2 bytes: 110xxxxx 10xxxxxx
//! 3 bytes: 1110xxxx 10xxxxxx 10xxxxxx
//! 4 bytes: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
//! ```
//!
//! Every byte after the first one of a character (a *continuation byte*) starts with `10`. You are
//! given `data`, a list of integers where only the lowest 8 bits of each element are used (each
//! element is one byte). Return `true` if `data` is a sequence of correctly formed characters by the
//! table above: every leading byte is followed by exactly the right number of continuation bytes,
//! and nothing is left unfinished at the end. (Only this bit structure is checked.)
//!
//! Implement `Solution::valid_utf8(data: Vec<i32>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  data = [197, 130, 1]
//! Output: true      (11000101 10000010 = one 2-byte character, then 00000001)
//!
//! Input:  data = [235, 140, 4]
//! Output: false     (11101011 starts a 3-byte character, but 00000100 is not a continuation byte)
//!
//! Input:  data = [240, 162, 138, 147]
//! Output: true      (one 4-byte character)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= data.len() <= 2 * 10^4`
//! - `0 <= data[i] <= 255`
//!
//! ## Hints
//!
//! 1. Range patterns work with binary literals: `0b1100_0000..=0b1101_1111` are exactly the bytes
//!    starting with `110`.
//! 2. Keep a state: "expecting a new character" or "expecting `n` more continuation bytes".

pub struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_byte_then_ascii() {
        assert!(Solution::valid_utf8(vec![197, 130, 1]));
    }

    #[test]
    fn example_broken_three_byte() {
        assert!(!Solution::valid_utf8(vec![235, 140, 4]));
    }

    #[test]
    fn example_four_byte() {
        assert!(Solution::valid_utf8(vec![240, 162, 138, 147]));
    }

    #[test]
    fn stray_continuation_and_bad_leaders() {
        assert!(!Solution::valid_utf8(vec![145]));
        assert!(!Solution::valid_utf8(vec![0b1111_1000, 128, 128, 128, 128]));
        assert!(!Solution::valid_utf8(vec![0b1110_0000, 128]));
    }

    #[test]
    fn real_utf8_strings() {
        let text = "Grüße, мир, 世界, 🦀";
        let data: Vec<i32> = text.bytes().map(i32::from).collect();
        assert!(Solution::valid_utf8(data.clone()));
        let mut cut = data;
        cut.pop();
        assert!(!Solution::valid_utf8(cut));
    }

    #[test]
    fn long_ascii() {
        assert!(Solution::valid_utf8(vec![65; 20_000]));
    }
}
