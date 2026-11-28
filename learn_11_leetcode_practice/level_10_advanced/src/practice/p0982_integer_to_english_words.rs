// ==================================================================
// PRACTICE 0982: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0982_integer_to_english_words.md
// Run the tests:          cargo test practice::p0982
// ==================================================================

//! # Problem 0982: Integer to English Words
//!
//! - **Difficulty:** Hard
//! - **Topics:** decomposition into groups of three digits, `const` word tables, a recursive helper below 1000, `Vec<&str>` + `join`
//! - **Source:** [LeetCode 273. Integer to English Words](https://leetcode.com/problems/integer-to-english-words/)
//!
//! ## Problem
//!
//! Convert a non-negative integer to its English words representation, with words capitalised and
//! separated by single spaces, using the short scale: `Thousand`, `Million`, `Billion`. No "and" and
//! no hyphens are used (`21` is `"Twenty One"`). Zero is `"Zero"`.
//!
//! Implement `Solution::number_to_words(num: i32) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 123        Output: "One Hundred Twenty Three"
//! Input:  num = 12345      Output: "Twelve Thousand Three Hundred Forty Five"
//! Input:  num = 1234567    Output: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= num <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Split the number into groups of three digits from the right: billions, millions, thousands,
//!    units.
//! 2. Write one function for `1..1000`: hundreds, then the rest below 100 (`< 20` has its own names,
//!    otherwise a tens word plus an optional ones word).
//! 3. Skip groups that are zero (`1_000_000` is `"One Million"`, not `"One Million Zero Thousand"`).

pub struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::number_to_words(123), "One Hundred Twenty Three");
        assert_eq!(
            Solution::number_to_words(12_345),
            "Twelve Thousand Three Hundred Forty Five"
        );
        assert_eq!(
            Solution::number_to_words(1_234_567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
    }

    #[test]
    fn small_numbers() {
        assert_eq!(Solution::number_to_words(0), "Zero");
        assert_eq!(Solution::number_to_words(7), "Seven");
        assert_eq!(Solution::number_to_words(13), "Thirteen");
        assert_eq!(Solution::number_to_words(20), "Twenty");
        assert_eq!(Solution::number_to_words(99), "Ninety Nine");
        assert_eq!(Solution::number_to_words(100), "One Hundred");
        assert_eq!(Solution::number_to_words(101), "One Hundred One");
    }

    #[test]
    fn zero_groups_are_skipped() {
        assert_eq!(Solution::number_to_words(1_000_000), "One Million");
        assert_eq!(Solution::number_to_words(1_000_010), "One Million Ten");
        assert_eq!(Solution::number_to_words(2_000_000_000), "Two Billion");
        assert_eq!(
            Solution::number_to_words(50_000_868),
            "Fifty Million Eight Hundred Sixty Eight"
        );
    }

    #[test]
    fn largest_value() {
        assert_eq!(
            Solution::number_to_words(i32::MAX),
            "Two Billion One Hundred Forty Seven Million Four Hundred Eighty Three Thousand Six Hundred Forty Seven"
        );
    }
}
