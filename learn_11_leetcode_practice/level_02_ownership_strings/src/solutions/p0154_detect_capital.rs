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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** one pass counts the capitals; the three rules then become three simple comparisons.
//! Because the word is ASCII, `len()` is also the number of letters and `as_bytes()[0]` is the first
//! letter.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `bytes()` iterates the raw `u8`s of a string, which is exact for ASCII-only input.
//! - `u8::is_ascii_uppercase` works on bytes; `char::is_ascii_uppercase` is the `char` version.
//! - Indexing `as_bytes()[0]` is safe here: the constraints guarantee a non-empty word.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        // @solution-start
        let capitals = word.bytes().filter(|b| b.is_ascii_uppercase()).count();
        capitals == 0
            || capitals == word.len()
            || (capitals == 1 && word.as_bytes()[0].is_ascii_uppercase())
        // @solution-end
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
