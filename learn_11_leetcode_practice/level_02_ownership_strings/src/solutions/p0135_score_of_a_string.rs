//! # Problem 0135: Score of a String
//!
//! - **Difficulty:** Easy
//! - **Topics:** `as_bytes`, `windows(2)`, `u8::abs_diff`, `sum`
//! - **Source:** [LeetCode 3110. Score of a String](https://leetcode.com/problems/score-of-a-string/)
//!
//! ## Problem
//!
//! The *score* of a string is the sum of the absolute differences between the ASCII codes of every
//! pair of **adjacent** characters.
//!
//! Implement `Solution::score_of_string(s: String) -> i32`, which returns the score of `s`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "hello"
//! Output: 13
//! Explanation: the codes are h=104, e=101, l=108, l=108, o=111, so the score is
//!              |104-101| + |101-108| + |108-108| + |108-111| = 3 + 7 + 0 + 3 = 13.
//!
//! Input:  s = "zaz"
//! Output: 50
//! Explanation: |122-97| + |97-122| = 25 + 25 = 50.
//! ```
//!
//! ## Constraints
//!
//! - `2 <= s.len() <= 100`
//! - `s` consists only of lowercase English letters.
//!
//! ## Hints
//!
//! 1. For ASCII text, each character is exactly one byte: `s.as_bytes()` gives the codes as `&[u8]`.
//! 2. `windows(2)` yields adjacent pairs; `a.abs_diff(b)` computes `|a - b|` without overflow.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** look at the bytes, take every adjacent pair with `windows(2)`, compute the absolute
//! difference and add everything up.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `as_bytes()` borrows the string's UTF-8 bytes as `&[u8]` without copying; for ASCII input the
//!   byte value is the ASCII code.
//! - `u8` subtraction that would go below 0 panics in debug builds; `abs_diff` returns the distance
//!   as a `u8` safely.
//! - `i32::from(d)` widens a `u8` to `i32` losslessly before summing.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        // @solution-start
        s.as_bytes()
            .windows(2)
            .map(|w| i32::from(w[0].abs_diff(w[1])))
            .sum()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_hello() {
        assert_eq!(Solution::score_of_string("hello".to_string()), 13);
    }

    #[test]
    fn example_zaz() {
        assert_eq!(Solution::score_of_string("zaz".to_string()), 50);
    }

    #[test]
    fn equal_letters_score_zero() {
        assert_eq!(Solution::score_of_string("aa".to_string()), 0);
    }

    #[test]
    fn alphabet_scores_one_per_step() {
        let alphabet: String = ('a'..='z').collect();
        assert_eq!(Solution::score_of_string(alphabet), 25);
    }

    #[test]
    fn longest_alternating_input() {
        assert_eq!(Solution::score_of_string("az".repeat(50)), 99 * 25);
    }
}
