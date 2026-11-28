// ==================================================================
// PRACTICE 0494: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0494_restore_ip_addresses.md
// Run the tests:          cargo test practice::p0494
// ==================================================================

//! # Problem 0494: Restore IP Addresses
//!
//! - **Difficulty:** Medium
//! - **Topics:** nested `flat_map` over segment lengths, a validation closure, `filter`, `join`, arrays of slices
//! - **Source:** [LeetCode 93. Restore IP Addresses](https://leetcode.com/problems/restore-ip-addresses/)
//!
//! ## Problem
//!
//! A valid IPv4 address is four integers separated by single dots, each between `0` and `255`
//! inclusive and written **without leading zeros** (`"0"` is fine, `"01"` is not). For example
//! `"192.168.1.1"` is valid while `"192.168.01.1"` and `"256.1.1.1"` are not.
//!
//! Given a string `s` of digits, return every valid IP address that can be made by inserting exactly
//! three dots into `s` (without reordering or removing digits), in **any order**.
//!
//! Implement `Solution::restore_ip_addresses(s: String) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "25525511135"
//! Output: ["255.255.11.135", "255.255.111.35"]
//!
//! Input:  s = "0000"
//! Output: ["0.0.0.0"]
//!
//! Input:  s = "101023"
//! Output: ["1.0.10.23", "1.0.102.3", "10.1.0.23", "10.10.2.3", "101.0.2.3"]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 20`
//! - `s` contains only digits.
//!
//! ## Hints
//!
//! 1. Each of the four segments has 1, 2 or 3 digits. Choosing the lengths `(a, b, c)` of the first
//!    three fixes the fourth: it is whatever is left, and it must have 1 to 3 digits too.
//! 2. Generate all `(a, b, c)` with nested `flat_map`s over `1..=3`, cut `s` into four slices, and
//!    keep the cuts where every slice is a valid segment.
//! 3. A segment is valid when it is `"0"` or does not start with `'0'`, and parses as a `u8`.

pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> Vec<String> {
        let mut out = Solution::restore_ip_addresses(s.to_string());
        out.sort();
        out
    }

    #[test]
    fn example_two_answers() {
        assert_eq!(run("25525511135"), vec!["255.255.11.135", "255.255.111.35"]);
    }

    #[test]
    fn example_zeros() {
        assert_eq!(run("0000"), vec!["0.0.0.0"]);
    }

    #[test]
    fn example_five_answers() {
        assert_eq!(
            run("101023"),
            vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }

    #[test]
    fn impossible_lengths_and_values() {
        assert_eq!(run("123"), Vec::<String>::new());
        assert_eq!(run("1234567890123"), Vec::<String>::new());
        assert_eq!(run("256256256256"), Vec::<String>::new());
        assert_eq!(run("010010"), vec!["0.10.0.10", "0.100.1.0"]);
        assert_eq!(run("255255255255"), vec!["255.255.255.255"]);
    }
}
