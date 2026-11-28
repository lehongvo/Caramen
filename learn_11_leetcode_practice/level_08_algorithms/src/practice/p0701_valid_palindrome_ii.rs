// ==================================================================
// PRACTICE 0701: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0701_valid_palindrome_ii.md
// Run the tests:          cargo test practice::p0701
// ==================================================================

//! # Problem 0701: Valid Palindrome II
//!
//! - **Difficulty:** Easy
//! - **Topics:** two pointers, byte slices `&[u8]`, half-open ranges, `Iterator::eq`
//! - **Source:** [LeetCode 680. Valid Palindrome II](https://leetcode.com/problems/valid-palindrome-ii/)
//!
//! ## Problem
//!
//! You are given a string `s` made of lowercase English letters. Decide whether `s` can be turned
//! into a palindrome (a string that reads the same from left to right and from right to left) by
//! deleting **at most one** character. Deleting nothing is allowed, so a string that is already a
//! palindrome is a valid answer.
//!
//! Return `true` if it is possible and `false` otherwise.
//!
//! Implement `Solution::valid_palindrome(s: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "aba"
//! Output: true          (already a palindrome)
//!
//! Input:  s = "abca"
//! Output: true          (delete 'c' to get "aba", or 'b' to get "aca")
//!
//! Input:  s = "abc"
//! Output: false         (no single deletion gives a palindrome)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^5`
//! - `s` contains only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Compare the two ends and walk inwards while they match (two pointers).
//! 2. At the first mismatch you must delete either the left or the right character. You do not
//!    know which one, so try both.
//! 3. After that one deletion, the remaining middle part must be a palindrome with no more deletions.

pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_already_palindrome() {
        assert!(Solution::valid_palindrome("aba".to_string()));
    }

    #[test]
    fn example_one_deletion() {
        assert!(Solution::valid_palindrome("abca".to_string()));
    }

    #[test]
    fn example_impossible() {
        assert!(!Solution::valid_palindrome("abc".to_string()));
    }

    #[test]
    fn single_character() {
        assert!(Solution::valid_palindrome("a".to_string()));
    }

    #[test]
    fn only_the_right_deletion_works() {
        assert!(Solution::valid_palindrome("cbbcc".to_string()));
    }

    #[test]
    fn only_the_left_deletion_works() {
        assert!(Solution::valid_palindrome("ccbbc".to_string()));
    }

    #[test]
    fn two_deletions_needed() {
        assert!(!Solution::valid_palindrome("abcda".to_string()));
    }

    #[test]
    fn long_string_with_one_extra_letter() {
        let s = format!("{}b{}", "a".repeat(50_000), "a".repeat(49_999));
        assert!(Solution::valid_palindrome(s));
    }
}
