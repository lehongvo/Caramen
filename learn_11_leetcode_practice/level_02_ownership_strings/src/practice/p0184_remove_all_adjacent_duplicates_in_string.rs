// ==================================================================
// PRACTICE 0184: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0184_remove_all_adjacent_duplicates_in_string.md
// Run the tests:          cargo test practice::p0184
// ==================================================================

//! # Problem 0184: Remove All Adjacent Duplicates In String
//!
//! - **Difficulty:** Easy
//! - **Topics:** a `String` used as a stack (`push`, `pop`, `ends_with`)
//! - **Source:** [LeetCode 1047. Remove All Adjacent Duplicates In String](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/)
//!
//! ## Problem
//!
//! A *duplicate removal* deletes two adjacent, equal letters. Starting from the lowercase string `s`,
//! keep applying duplicate removals until none is possible, and return the final string. The answer
//! does not depend on the order of the removals.
//!
//! Implement `Solution::remove_duplicates(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "abbaca"
//! Output: "ca"
//! Explanation: remove "bb" to get "aaca", then remove "aa" to get "ca".
//!
//! Input:  s = "azxxzy"
//! Output: "ay"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^5`
//! - `s` consists of lowercase English letters.
//!
//! ## Hints
//!
//! 1. Process the letters from left to right and keep the result built so far.
//! 2. If the next letter equals the last letter of the result, the two cancel out: `pop()` it.
//!    Otherwise `push()` the letter.

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::remove_duplicates(s.to_string())
    }

    #[test]
    fn example_abbaca() {
        assert_eq!(run("abbaca"), "ca");
    }

    #[test]
    fn example_azxxzy() {
        assert_eq!(run("azxxzy"), "ay");
    }

    #[test]
    fn everything_cancels() {
        assert_eq!(run("aa"), "");
        assert_eq!(run("abccba"), "");
    }

    #[test]
    fn odd_run_leaves_one() {
        assert_eq!(run("aaa"), "a");
        assert_eq!(run("x"), "x");
    }

    #[test]
    fn large_input() {
        let s = format!("{}{}", "ab".repeat(25_000), "ba".repeat(25_000));
        assert_eq!(run(&s), "");
        assert_eq!(run(&"abc".repeat(30_000)).len(), 90_000);
    }
}
