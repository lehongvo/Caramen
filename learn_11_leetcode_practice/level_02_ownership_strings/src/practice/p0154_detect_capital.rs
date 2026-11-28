// ==================================================================
// PRACTICE 0154: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0154_detect_capital.md
// Run the tests:          cargo test practice::p0154
// ==================================================================

//! # Problem 0154: Detect Capital
//!
//! - **Difficulty:** Easy
//! - **Topics:** `bytes()`, `u8::is_ascii_uppercase`, counting with `filter`, `as_bytes()[0]`
//! - **Source:** [LeetCode 520. Detect Capital](https://leetcode.com/problems/detect-capital/)
//!
//! ## Problem
//!
//! A word uses capital letters correctly if one of these holds:
//!
//! 1. all its letters are capitals, like `"USA"`;
//! 2. none of its letters are capitals, like `"leetcode"`;
//! 3. only its first letter is a capital, like `"Google"`.
//!
//! Implement `Solution::detect_capital_use(word: String) -> bool`, which returns `true` when the
//! word uses capitals correctly.
//!
//! ## Examples
//!
//! ```text
//! Input:  word = "USA"
//! Output: true
//!
//! Input:  word = "FlaG"
//! Output: false
//!
//! Input:  word = "g"
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= word.len() <= 100`
//! - `word` consists of lowercase and uppercase English letters.
//!
//! ## Hints
//!
//! 1. Count the uppercase letters.
//! 2. The word is correct if the count is `0`, equals the length, or is `1` with the capital at
//!    position 0.

pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(word: &str) -> bool {
        Solution::detect_capital_use(word.to_string())
    }

    #[test]
    fn example_all_capitals() {
        assert!(check("USA"));
    }

    #[test]
    fn example_mixed_is_wrong() {
        assert!(!check("FlaG"));
    }

    #[test]
    fn example_single_letter() {
        assert!(check("g"));
        assert!(check("G"));
    }

    #[test]
    fn lowercase_and_title_case() {
        assert!(check("leetcode"));
        assert!(check("Google"));
    }

    #[test]
    fn single_capital_not_first() {
        assert!(!check("gOogle"));
        assert!(!check("googlE"));
    }

    #[test]
    fn longest_words() {
        assert!(check(&"A".repeat(100)));
        assert!(check(&format!("A{}", "b".repeat(99))));
        assert!(!check(&format!("{}A", "b".repeat(99))));
    }
}
