// ==================================================================
// PRACTICE 0185: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0185_make_the_string_great.md
// Run the tests:          cargo test practice::p0185
// ==================================================================

//! # Problem 0185: Make The String Great
//!
//! - **Difficulty:** Easy
//! - **Topics:** `String` as a stack, `chars().next_back()`, match guards, `eq_ignore_ascii_case`
//! - **Source:** [LeetCode 1544. Make The String Great](https://leetcode.com/problems/make-the-string-great/)
//!
//! ## Problem
//!
//! A string is *good* if it has no two adjacent characters that are the same letter in different
//! cases (such as `"aA"` or `"Bb"`). Repeatedly remove such a pair until the string is good, and
//! return the result (it may be empty). The final string does not depend on the order of removals.
//!
//! Implement `Solution::make_good(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "leEeetcode"
//! Output: "leetcode"       ("eE" is removed)
//!
//! Input:  s = "abBAcC"
//! Output: ""               ("bB", then "aA", then "cC")
//!
//! Input:  s = "s"
//! Output: "s"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 100`
//! - `s` contains only lowercase and uppercase English letters.
//!
//! ## Hints
//!
//! 1. Same stack idea as in "Remove All Adjacent Duplicates In String": compare each character with
//!    the last character kept so far.
//! 2. A pair cancels when the two characters are different but equal ignoring case:
//!    `last != c && last.eq_ignore_ascii_case(&c)`.

pub struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::make_good(s.to_string())
    }

    #[test]
    fn example_leetcode() {
        assert_eq!(run("leEeetcode"), "leetcode");
    }

    #[test]
    fn example_everything_removed() {
        assert_eq!(run("abBAcC"), "");
    }

    #[test]
    fn example_single() {
        assert_eq!(run("s"), "s");
    }

    #[test]
    fn same_case_pairs_stay() {
        assert_eq!(run("aa"), "aa");
        assert_eq!(run("AA"), "AA");
    }

    #[test]
    fn different_letters_stay() {
        assert_eq!(run("aB"), "aB");
    }

    #[test]
    fn longest_input() {
        // "eE" meets first; each removal exposes the next pair ("dD", "cC", ...), so all cancel.
        let s = format!("{}{}", "abcde".repeat(10), "EDCBA".repeat(10));
        assert_eq!(run(&s), "");
        // Here no character ever sits next to its other-case twin.
        let s = format!("{}{}", "abcde".repeat(10), "ABCDE".repeat(10));
        assert_eq!(run(&s).len(), 100);
    }
}
