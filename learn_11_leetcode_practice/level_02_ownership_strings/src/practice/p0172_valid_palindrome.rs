// ==================================================================
// PRACTICE 0172: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0172_valid_palindrome.md
// Run the tests:          cargo test practice::p0172
// ==================================================================

//! # Problem 0172: Valid Palindrome
//!
//! - **Difficulty:** Easy
//! - **Topics:** two indices over `as_bytes()`, `is_ascii_alphanumeric`, `eq_ignore_ascii_case`
//! - **Source:** [LeetCode 125. Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)
//!
//! ## Problem
//!
//! A phrase is a palindrome if, after converting all uppercase letters to lowercase and removing
//! every character that is not a letter or a digit, it reads the same forwards and backwards.
//!
//! Implement `Solution::is_palindrome(s: String) -> bool`. An empty phrase (or one with no letters
//! or digits) counts as a palindrome.
//!
//! Try to do it without building a cleaned-up copy of the string.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "A man, a plan, a canal: Panama"
//! Output: true            ("amanaplanacanalpanama")
//!
//! Input:  s = "race a car"
//! Output: false           ("raceacar")
//!
//! Input:  s = " "
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 2 * 10^5`
//! - `s` consists only of printable ASCII characters.
//!
//! ## Hints
//!
//! 1. Use an index `i` from the left and an index `j` from the right; skip characters that are not
//!    alphanumeric on either side.
//! 2. Compare the two bytes with `u8::eq_ignore_ascii_case`.
//! 3. Keeping `j` exclusive (`b[j - 1]` is the right character) avoids `usize` underflow.

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str) -> bool {
        Solution::is_palindrome(s.to_string())
    }

    #[test]
    fn example_panama() {
        assert!(check("A man, a plan, a canal: Panama"));
    }

    #[test]
    fn example_race_a_car() {
        assert!(!check("race a car"));
    }

    #[test]
    fn example_only_space() {
        assert!(check(" "));
    }

    #[test]
    fn digits_matter() {
        assert!(!check("0P"));
        assert!(check("1a2, 2A1"));
    }

    #[test]
    fn only_punctuation() {
        assert!(check(".,!?"));
    }

    #[test]
    fn large_input() {
        let half = "Ab1, ".repeat(19_999);
        let mirrored: String = half.chars().rev().collect();
        assert!(check(&format!("{half}{mirrored}")));
        assert!(!check(&format!("{half}x{mirrored}y")));
    }
}
