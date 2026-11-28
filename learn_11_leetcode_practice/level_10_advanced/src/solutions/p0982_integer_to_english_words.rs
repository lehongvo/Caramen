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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** two word tables (`0..20` and the tens) cover everything below 100. `below_thousand`
//! appends the words for `1..1000` to a vector. The main function walks the scales from billion down,
//! appending each non-zero group followed by its scale word, and joins everything with spaces.
//!
//! **Complexity:** `O(1)`: the number has at most 10 digits.
//!
//! **Rust concepts:**
//! - `const` arrays of `&'static str` are perfect lookup tables.
//! - Collecting into `Vec<&'static str>` and calling `join(" ")` once avoids juggling spaces.
// @solution-end

pub struct Solution;

// @solution-start remove
const SMALL: [&str; 20] = [
    "Zero",
    "One",
    "Two",
    "Three",
    "Four",
    "Five",
    "Six",
    "Seven",
    "Eight",
    "Nine",
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];
const TENS: [&str; 10] = [
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

/// Appends the words for `n` in `1..1000`.
fn below_thousand(n: usize, out: &mut Vec<&'static str>) {
    if n >= 100 {
        out.push(SMALL[n / 100]);
        out.push("Hundred");
    }
    let rest = n % 100;
    if rest >= 20 {
        out.push(TENS[rest / 10]);
        if rest % 10 != 0 {
            out.push(SMALL[rest % 10]);
        }
    } else if rest > 0 {
        out.push(SMALL[rest]);
    }
}
// @solution-end

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        // @solution-start
        if num == 0 {
            return "Zero".to_string();
        }
        let mut n = num as usize;
        let mut words: Vec<&'static str> = Vec::new();
        for (scale, name) in [
            (1_000_000_000, "Billion"),
            (1_000_000, "Million"),
            (1_000, "Thousand"),
        ] {
            if n >= scale {
                below_thousand(n / scale, &mut words);
                words.push(name);
                n %= scale;
            }
        }
        if n > 0 {
            below_thousand(n, &mut words);
        }
        words.join(" ")
        // @solution-end
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
