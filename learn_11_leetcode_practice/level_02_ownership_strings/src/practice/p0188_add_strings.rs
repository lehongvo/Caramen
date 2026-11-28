// ==================================================================
// PRACTICE 0188: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0188_add_strings.md
// Run the tests:          cargo test practice::p0188
// ==================================================================

//! # Problem 0188: Add Strings
//!
//! - **Difficulty:** Easy
//! - **Topics:** two `bytes().rev()` iterators, `Option::map_or`, carry, collecting digits in reverse
//! - **Source:** [LeetCode 415. Add Strings](https://leetcode.com/problems/add-strings/)
//!
//! ## Problem
//!
//! Given two non-negative integers `num1` and `num2` written as decimal strings, return their sum as
//! a decimal string. Do not convert the inputs to integers (they can have thousands of digits) and do
//! not use a big-integer library.
//!
//! Implement `Solution::add_strings(num1: String, num2: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num1 = "11", num2 = "123"
//! Output: "134"
//!
//! Input:  num1 = "456", num2 = "77"
//! Output: "533"
//!
//! Input:  num1 = "0", num2 = "0"
//! Output: "0"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num1.len(), num2.len() <= 10^4`
//! - Both contain only digits and have no leading zeros except the number `"0"`.
//!
//! ## Hints
//!
//! 1. `num1.bytes().rev()` walks the digits from least to most significant.
//! 2. Advance both reversed iterators together; when one runs out, treat its digit as `0`
//!    (`next().map_or(0, |d| d - b'0')`).
//! 3. Stop when both are exhausted **and** the carry is `0`.

pub struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add(a: &str, b: &str) -> String {
        Solution::add_strings(a.to_string(), b.to_string())
    }

    #[test]
    fn example_different_lengths() {
        assert_eq!(add("11", "123"), "134");
    }

    #[test]
    fn example_carries() {
        assert_eq!(add("456", "77"), "533");
    }

    #[test]
    fn example_zeros() {
        assert_eq!(add("0", "0"), "0");
    }

    #[test]
    fn final_carry_adds_a_digit() {
        assert_eq!(add("999", "1"), "1000");
        assert_eq!(add("1", "9"), "10");
    }

    #[test]
    fn matches_integer_addition() {
        for (x, y) in [
            (0u64, 12_345u64),
            (98_765, 43_210),
            (u32::MAX as u64, u32::MAX as u64),
        ] {
            assert_eq!(add(&x.to_string(), &y.to_string()), (x + y).to_string());
        }
    }

    #[test]
    fn longest_inputs() {
        let result = add(&"9".repeat(10_000), &"9".repeat(10_000));
        assert_eq!(result.len(), 10_001);
        assert_eq!(&result[..2], "19");
        assert!(result.ends_with("998"));
    }
}
