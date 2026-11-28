//! # Problem 0182: Largest Odd Number in String
//!
//! - **Difficulty:** Easy
//! - **Topics:** `rfind` with a `[char; N]` pattern, `String::truncate` on an owned `String`, prefixes
//! - **Source:** [LeetCode 1903. Largest Odd Number in String](https://leetcode.com/problems/largest-odd-number-in-string/)
//!
//! ## Problem
//!
//! `num` is a large non-negative integer written as a string of decimal digits. Among all its
//! non-empty **substrings** (contiguous parts) that represent an odd number, return the one with the
//! largest value, or an empty string if there is none.
//!
//! Implement `Solution::largest_odd_number(num: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = "52"
//! Output: "5"
//!
//! Input:  num = "4206"
//! Output: ""               (every digit is even, so no odd substring exists)
//!
//! Input:  num = "35427"
//! Output: "35427"          (the number itself is odd)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num.len() <= 10^5`
//! - `num` consists only of digits and has no leading zeros.
//!
//! ## Hints
//!
//! 1. A number is odd exactly when its **last** digit is odd.
//! 2. Longer substrings without leading zeros are larger, and `num` has no leading zero, so the best
//!    candidate is the longest **prefix** of `num` that ends with an odd digit.
//! 3. `num.rfind(['1', '3', '5', '7', '9'])` finds the last odd digit.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** find the last odd digit. The prefix that ends there is the longest odd substring starting
//! at the first digit, hence the largest. Truncating the owned `String` produces it without copying.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - An array of `char`s is a valid string pattern: it matches any of the listed characters.
//! - `rfind` searches from the end and returns the byte index of the match.
//! - `String::truncate(len)` shortens the string in place; `len = i + 1` keeps the odd digit (ASCII,
//!   so it is one byte).
// @solution-end

pub struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        // @solution-start
        match num.rfind(['1', '3', '5', '7', '9']) {
            Some(i) => {
                let mut num = num;
                num.truncate(i + 1);
                num
            }
            None => String::new(),
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(num: &str) -> String {
        Solution::largest_odd_number(num.to_string())
    }

    #[test]
    fn example_first_digit() {
        assert_eq!(run("52"), "5");
    }

    #[test]
    fn example_all_even() {
        assert_eq!(run("4206"), "");
    }

    #[test]
    fn example_whole_number() {
        assert_eq!(run("35427"), "35427");
    }

    #[test]
    fn odd_digit_in_the_middle() {
        assert_eq!(run("2468135024"), "2468135");
        assert_eq!(run("7"), "7");
        assert_eq!(run("0"), "");
    }

    #[test]
    fn large_input() {
        let num = format!("1{}", "2".repeat(99_999));
        assert_eq!(run(&num), "1");
        let num = format!("{}3", "8".repeat(99_999));
        assert_eq!(run(&num).len(), 100_000);
    }
}
