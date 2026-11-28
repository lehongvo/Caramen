// ==================================================================
// PRACTICE 0177: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0177_lexicographically_smallest_palindrome.md
// Run the tests:          cargo test practice::p0177
// ==================================================================

//! # Problem 0177: Lexicographically Smallest Palindrome
//!
//! - **Difficulty:** Easy
//! - **Topics:** mirrored indices `i` and `n - 1 - i`, writing two positions of one `Vec<u8>`, `min`
//! - **Source:** [LeetCode 2697. Lexicographically Smallest Palindrome](https://leetcode.com/problems/lexicographically-smallest-palindrome/)
//!
//! ## Problem
//!
//! You may replace any character of the lowercase string `s` with another lowercase letter; each
//! replacement counts as one operation. Turn `s` into a palindrome using the **minimum** number of
//! operations. If several palindromes need that minimum, return the lexicographically smallest one
//! (the one that comes first in dictionary order).
//!
//! Implement `Solution::make_smallest_palindrome(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "egcfe"
//! Output: "efcfe"          (one operation: 'g' -> 'f')
//!
//! Input:  s = "abcd"
//! Output: "abba"           (two operations)
//!
//! Input:  s = "seven"
//! Output: "neven"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 1000`
//! - `s` consists of lowercase English letters.
//!
//! ## Hints
//!
//! 1. Each pair of mirrored positions `(i, n - 1 - i)` must end up equal; if they differ, one
//!    operation is needed and enough.
//! 2. To make the result as small as possible, set both to the **smaller** of the two letters.

pub struct Solution;

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::make_smallest_palindrome(s.to_string())
    }

    #[test]
    fn example_one_change() {
        assert_eq!(run("egcfe"), "efcfe");
    }

    #[test]
    fn example_two_changes() {
        assert_eq!(run("abcd"), "abba");
    }

    #[test]
    fn example_seven() {
        assert_eq!(run("seven"), "neven");
    }

    #[test]
    fn already_palindrome_and_single_letter() {
        assert_eq!(run("racecar"), "racecar");
        assert_eq!(run("q"), "q");
    }

    #[test]
    fn longest_input() {
        let s = format!("{}{}", "z".repeat(500), "a".repeat(500));
        assert_eq!(run(&s), "a".repeat(1000));
    }
}
