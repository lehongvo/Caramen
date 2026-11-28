//! # Problem 0403: Maximum Value of a String in an Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** `str::parse`, `Result::unwrap_or`, `Iterator::max`, `Option::unwrap_or`
//! - **Source:** [LeetCode 2496. Maximum Value of a String in an Array](https://leetcode.com/problems/maximum-value-of-a-string-in-an-array/)
//!
//! ## Problem
//!
//! Every string made of lowercase English letters and digits has a *value*:
//!
//! - if the string contains **only digits**, its value is the number it spells in base 10
//!   (leading zeros are allowed, so `"0042"` has value `42`);
//! - otherwise its value is its **length**.
//!
//! Given a list of such strings, return the largest value among them.
//!
//! Implement `Solution::maximum_value(strs: Vec<String>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  strs = ["alic3", "bob", "3", "4", "00000"]
//! Output: 5
//! Explanation: "alic3" contains a letter, so its value is its length 5. "bob" -> 3,
//!              "3" -> 3, "4" -> 4, "00000" -> 0. The maximum is 5.
//!
//! Input:  strs = ["1", "01", "001", "0001"]
//! Output: 1
//! Explanation: all four strings are digits only and all spell the number 1.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= strs.len() <= 100`
//! - `1 <= strs[i].len() <= 9`
//! - `strs[i]` contains only lowercase English letters and digits.
//!
//! ## Hints
//!
//! 1. `s.parse::<i32>()` returns `Ok(n)` exactly when `s` is a valid number. With at most 9 digits
//!    and no sign characters in the input, "digits only" and "parses successfully" mean the same.
//! 2. `Result::unwrap_or(fallback)` gives you a fallback value when parsing fails.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** map every string to its value with one expression: try to parse it, and if parsing
//! fails use its length. Then take the maximum of all values.
//!
//! **Complexity:** `O(total length of all strings)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `str::parse::<i32>()` returns `Result<i32, ParseIntError>`. Leading zeros are accepted.
//! - `unwrap_or(fallback)` ignores the error value. `s.len()` is so cheap that computing it eagerly
//!   is fine (Clippy even suggests `unwrap_or` over `unwrap_or_else(|_| s.len())` here).
//! - `Iterator::max` returns `Option<i32>` (`None` for an empty iterator). The input is never empty,
//!   but `unwrap_or(0)` keeps the code panic-free anyway.
//! - At most 9 characters means at most `999_999_999`, which fits in an `i32`, so parsing never
//!   fails because of overflow.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        // @solution-start
        strs.iter()
            .map(|s| s.parse::<i32>().unwrap_or(s.len() as i32))
            .max()
            .unwrap_or(0)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_mixed_strings() {
        assert_eq!(
            Solution::maximum_value(strings(&["alic3", "bob", "3", "4", "00000"])),
            5
        );
    }

    #[test]
    fn example_all_spell_one() {
        assert_eq!(
            Solution::maximum_value(strings(&["1", "01", "001", "0001"])),
            1
        );
    }

    #[test]
    fn a_number_can_beat_every_length() {
        assert_eq!(Solution::maximum_value(strings(&["abcdefghi", "123"])), 123);
        assert_eq!(
            Solution::maximum_value(strings(&["999999999"])),
            999_999_999
        );
    }

    #[test]
    fn letters_only_use_the_length() {
        assert_eq!(Solution::maximum_value(strings(&["a", "zz", "x9y"])), 3);
    }

    #[test]
    fn zeros_have_value_zero() {
        assert_eq!(Solution::maximum_value(strings(&["0", "000000000"])), 0);
    }
}
