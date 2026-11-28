// ==================================================================
// PRACTICE 0412: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0412_check_if_numbers_are_ascending_in_a_sentence.md
// Run the tests:          cargo test practice::p0412
// ==================================================================

//! # Problem 0412: Check if Numbers Are Ascending in a Sentence
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split`, `filter_map`, `Result::ok`, `windows`, `Iterator::all`
//! - **Source:** [LeetCode 2042. Check if Numbers Are Ascending in a Sentence](https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/)
//!
//! ## Problem
//!
//! A sentence is a list of *tokens* separated by single spaces. Each token is either a positive
//! number written with digits only (no leading zeros) or a word made of lowercase English letters.
//!
//! Return `true` if all the numbers in the sentence are **strictly increasing** from left to right
//! (each number is smaller than the next number), and `false` otherwise. The words are ignored.
//!
//! Implement `Solution::are_numbers_ascending(s: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "1 box has 3 blue 4 red 6 green and 12 yellow marbles"
//! Output: true
//! Explanation: the numbers are 1, 3, 4, 6, 12.
//!
//! Input:  s = "hello world 5 x 5"
//! Output: false
//! Explanation: 5 is not strictly smaller than 5.
//!
//! Input:  s = "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
//! Output: false
//! Explanation: 51 is followed by 50.
//! ```
//!
//! ## Constraints
//!
//! - `3 <= s.len() <= 200`
//! - `s` has lowercase English letters, digits and single spaces; no leading or trailing spaces.
//! - Numbers are positive, below `100`, without leading zeros. There are at least two numbers.
//!
//! ## Hints
//!
//! 1. A token is a number exactly when `token.parse::<u32>()` succeeds.
//! 2. `filter_map(|t| t.parse::<u32>().ok())` keeps only the numbers, already converted.
//! 3. Compare neighbours with `windows(2)` on a `Vec`, or remember the previous number while
//!    iterating.

pub struct Solution;

impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> bool {
        Solution::are_numbers_ascending(s.to_string())
    }

    #[test]
    fn example_marbles() {
        assert!(run("1 box has 3 blue 4 red 6 green and 12 yellow marbles"));
    }

    #[test]
    fn example_equal_numbers() {
        assert!(!run("hello world 5 x 5"));
    }

    #[test]
    fn example_decrease_later() {
        assert!(!run(
            "sunset is at 7 51 pm overnight lows will be in the low 50 and 60 s"
        ));
    }

    #[test]
    fn only_numbers() {
        assert!(run("1 2 3 99"));
        assert!(!run("2 1"));
        assert!(run("4 5"));
    }
}
