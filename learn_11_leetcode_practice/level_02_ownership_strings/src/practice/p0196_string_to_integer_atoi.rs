// ==================================================================
// PRACTICE 0196: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0196_string_to_integer_atoi.md
// Run the tests:          cargo test practice::p0196
// ==================================================================

//! # Problem 0196: String to Integer (atoi)
//!
//! - **Difficulty:** Medium
//! - **Topics:** `trim_start_matches`, matching on `as_bytes().first()`, `take_while`, overflow handling, `clamp`
//! - **Source:** [LeetCode 8. String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)
//!
//! ## Problem
//!
//! Convert a string to a 32-bit signed integer, the way C's `atoi` does:
//!
//! 1. Skip leading space characters `' '` (only spaces).
//! 2. Read an optional sign: `'-'` makes the number negative, `'+'` (or no sign) positive.
//! 3. Read the digits that follow, until the first non-digit character or the end of the string.
//!    Leading zeros are allowed. If no digit is read, the result is `0`.
//! 4. If the value is outside the `i32` range `[-2^31, 2^31 - 1]`, clamp it to that range.
//!
//! Everything after the digits is ignored. Implement `Solution::my_atoi(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "42"
//! Output: 42
//!
//! Input:  s = "   -042"
//! Output: -42
//!
//! Input:  s = "1337c0d3"
//! Output: 1337
//!
//! Input:  s = "0-1"
//! Output: 0
//!
//! Input:  s = "words and 987"
//! Output: 0
//!
//! Input:  s = "-91283472332"
//! Output: -2147483648      (clamped)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= s.len() <= 200`
//! - `s` consists of English letters, digits, `' '`, `'+'`, `'-'` and `'.'`.
//!
//! ## Hints
//!
//! 1. `s.trim_start_matches(' ')` removes only spaces (`trim_start` would also remove tabs).
//! 2. Look at the first byte to detect a sign, then slice it off with `&rest[1..]`.
//! 3. Accumulate in an `i64` and stop as soon as the value exceeds `2^31`; then clamp.

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atoi(s: &str) -> i32 {
        Solution::my_atoi(s.to_string())
    }

    #[test]
    fn example_plain_number() {
        assert_eq!(atoi("42"), 42);
    }

    #[test]
    fn example_spaces_sign_and_zero() {
        assert_eq!(atoi("   -042"), -42);
    }

    #[test]
    fn example_stops_at_letter() {
        assert_eq!(atoi("1337c0d3"), 1337);
    }

    #[test]
    fn example_stops_at_second_sign() {
        assert_eq!(atoi("0-1"), 0);
    }

    #[test]
    fn example_no_leading_digits() {
        assert_eq!(atoi("words and 987"), 0);
    }

    #[test]
    fn example_clamped_negative() {
        assert_eq!(atoi("-91283472332"), i32::MIN);
    }

    #[test]
    fn clamping_boundaries() {
        assert_eq!(atoi("2147483647"), i32::MAX);
        assert_eq!(atoi("2147483648"), i32::MAX);
        assert_eq!(atoi("-2147483648"), i32::MIN);
        assert_eq!(atoi("-2147483649"), i32::MIN);
    }

    #[test]
    fn edge_inputs() {
        assert_eq!(atoi(""), 0);
        assert_eq!(atoi("   "), 0);
        assert_eq!(atoi("+"), 0);
        assert_eq!(atoi("+-12"), 0);
        assert_eq!(atoi(" +0 123"), 0);
        assert_eq!(atoi("3.14159"), 3);
    }

    #[test]
    fn very_long_number() {
        let s = format!("   +{}", "9".repeat(190));
        assert_eq!(atoi(&s), i32::MAX);
        let s = format!("-{}7", "0".repeat(190));
        assert_eq!(atoi(&s), -7);
    }
}
