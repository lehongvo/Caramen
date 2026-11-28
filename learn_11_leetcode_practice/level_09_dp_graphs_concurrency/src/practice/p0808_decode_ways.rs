// ==================================================================
// PRACTICE 0808: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0808_decode_ways.md
// Run the tests:          cargo test practice::p0808
// ==================================================================

//! # Problem 0808: Decode Ways
//!
//! - **Difficulty:** Medium
//! - **Topics:** DP over bytes, rolling variables, `as_bytes`, `windows`-style lookback
//! - **Source:** [LeetCode 91. Decode Ways](https://leetcode.com/problems/decode-ways/)
//!
//! ## Problem
//!
//! A message of capital letters was encoded with `'A' -> "1"`, `'B' -> "2"`, ..., `'Z' -> "26"`
//! and the codes were written next to each other with no separator. Given the digit string `s`,
//! return the number of different letter messages that encode to exactly `s`.
//!
//! A code is valid only if it is one of `"1"` to `"26"`: codes with a leading zero such as `"06"`
//! are **not** valid, and `"0"` alone is not valid. If `s` cannot be decoded at all, return `0`.
//!
//! Implement `Solution::num_decodings(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "12"
//! Output: 2
//! Explanation: "AB" (1 2) or "L" (12).
//!
//! Input:  s = "226"
//! Output: 3
//! Explanation: "BZ" (2 26), "VF" (22 6) or "BBF" (2 2 6).
//!
//! Input:  s = "06"
//! Output: 0
//! Explanation: "06" is not a valid code and "0" cannot be decoded.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 100`
//! - `s` contains only digits and may contain leading zeros.
//! - The answer fits in an `i32`.
//!
//! ## Hints
//!
//! 1. Let `ways(i)` be the number of decodings of the first `i` characters. The last code is either
//!    one digit or two digits long.
//! 2. A one-digit last code is valid when that digit is not `'0'`; a two-digit code is valid when it
//!    is between `10` and `26`.
//! 3. You only need `ways(i - 1)` and `ways(i - 2)`.

pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(s: &str) -> i32 {
        Solution::num_decodings(s.to_string())
    }

    #[test]
    fn example_12() {
        assert_eq!(decode("12"), 2);
    }

    #[test]
    fn example_226() {
        assert_eq!(decode("226"), 3);
    }

    #[test]
    fn example_leading_zero() {
        assert_eq!(decode("06"), 0);
    }

    #[test]
    fn zeros_must_pair_with_a_previous_digit() {
        assert_eq!(decode("10"), 1);
        assert_eq!(decode("2101"), 1);
        assert_eq!(decode("11106"), 2);
        assert_eq!(decode("30"), 0);
        assert_eq!(decode("100"), 0);
    }

    #[test]
    fn ones_give_fibonacci_counts() {
        // A run of k ones decodes in Fibonacci(k + 1) ways.
        assert_eq!(decode("1111111111"), 89);
    }
}
