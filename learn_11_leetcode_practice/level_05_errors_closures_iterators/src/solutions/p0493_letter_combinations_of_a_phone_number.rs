//! # Problem 0493: Letter Combinations of a Phone Number
//!
//! - **Difficulty:** Medium
//! - **Topics:** `fold` over the digits, `flat_map` to extend every prefix, inner `move` closure, a `const` lookup table
//! - **Source:** [LeetCode 17. Letter Combinations of a Phone Number](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)
//!
//! ## Problem
//!
//! On a phone keypad, the digits `2` to `9` carry letters:
//!
//! ```text
//! 2: abc   3: def   4: ghi   5: jkl   6: mno   7: pqrs   8: tuv   9: wxyz
//! ```
//!
//! Given a string of digits from `'2'` to `'9'`, return every string of letters that the digits can
//! spell (one letter per digit), in **any order**. An empty input gives an empty list.
//!
//! Implement `Solution::letter_combinations(digits: String) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  digits = "23"
//! Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
//!
//! Input:  digits = ""
//! Output: []
//!
//! Input:  digits = "2"
//! Output: ["a", "b", "c"]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= digits.len() <= 4`
//! - `digits[i]` is a digit in `'2'..='9'`.
//!
//! ## Hints
//!
//! 1. Start with the list `[""]` (one empty prefix). For each digit, replace the list by "every old
//!    prefix followed by every letter of the digit".
//! 2. That step is a `fold` whose body is a `flat_map`:
//!    `acc.iter().flat_map(|prefix| letters.chars().map(move |c| format!("{prefix}{c}")))`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build the combinations digit by digit. After `k` digits the accumulator holds all
//! combinations of the first `k` digits; `flat_map` extends each of them with every possible letter
//! of the next digit.
//!
//! **Complexity:** `O(4^n * n)` time and space for `n` digits (there are at most `4^n` results of
//! length `n`).
//!
//! **Rust concepts:**
//! - `fold(vec![String::new()], |acc, digit| ...)` replaces the accumulator at every step.
//! - Nested closures: the inner `map` closure uses `prefix` from the outer one, so it is `move` (it
//!   copies the `&String` reference into the inner closure).
//! - A `const` array of `&str` is a zero-cost lookup table; `(d - b'2') as usize` indexes it.
//! - Without the empty-input check, the fold would return `[""]`, but the problem wants `[]`.
// @solution-end

pub struct Solution;

// @solution-start remove
const KEYPAD: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
// @solution-end

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // @solution-start
        if digits.is_empty() {
            return Vec::new();
        }
        digits.bytes().fold(vec![String::new()], |acc, d| {
            let letters = KEYPAD[usize::from(d - b'2')];
            acc.iter()
                .flat_map(|prefix| letters.chars().map(move |c| format!("{prefix}{c}")))
                .collect()
        })
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn example_two_digits() {
        assert_eq!(
            sorted(Solution::letter_combinations("23".to_string())),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn example_empty() {
        assert_eq!(
            Solution::letter_combinations(String::new()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn example_one_digit() {
        assert_eq!(
            sorted(Solution::letter_combinations("2".to_string())),
            vec!["a", "b", "c"]
        );
        assert_eq!(
            sorted(Solution::letter_combinations("7".to_string())),
            vec!["p", "q", "r", "s"]
        );
    }

    #[test]
    fn four_digits() {
        let all = Solution::letter_combinations("7979".to_string());
        assert_eq!(all.len(), 256);
        assert!(all.iter().all(|s| s.len() == 4));
        let mut unique = all.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), 256);
        assert!(all.contains(&"swsw".to_string()));
    }
}
