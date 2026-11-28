//! # Problem 0792: Letter Case Permutation
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking over positions, binary choices, ASCII case methods on `u8`
//! - **Source:** [LeetCode 784. Letter Case Permutation](https://leetcode.com/problems/letter-case-permutation/)
//!
//! ## Problem
//!
//! Given a string `s` of English letters and digits, you may change each letter independently to
//! lowercase or uppercase (digits stay as they are). Return every string that can be produced
//! this way, in any order.
//!
//! Implement `Solution::letter_case_permutation(s: String) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "a1b2"
//! Output: ["a1b2", "a1B2", "A1b2", "A1B2"]
//!
//! Input:  s = "3z4"
//! Output: ["3z4", "3Z4"]
//!
//! Input:  s = "12345"
//! Output: ["12345"]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 12`
//! - `s` contains only English letters and digits.
//!
//! ## Hints
//!
//! 1. Walk through the positions. A digit has one option; a letter has two.
//! 2. Recursion with a mutable buffer: keep the character, recurse; then switch its case, recurse,
//!    and switch it back.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** backtracking over positions: `backtrack(i)` decides position `i`. It first recurses
//! with the character unchanged. If the character is a letter, it switches its case, recurses again,
//! and restores it (the un-choose step). When `i` reaches the end, the buffer is one complete
//! string. With `L` letters this produces exactly `2^L` strings, each once.
//!
//! **Complexity:** `O(n * 2^L)` time (`2^L` strings of length `n`), `O(n)` recursion depth.
//!
//! **Rust concepts:**
//! - The buffer is a `Vec<u8>` of ASCII bytes passed as `&mut [u8]`; `u8::is_ascii_alphabetic`,
//!   `to_ascii_uppercase` and `to_ascii_lowercase` work directly on bytes.
//! - `bytes.iter().map(|&b| char::from(b)).collect::<String>()` turns the ASCII buffer back into a
//!   `String` without any fallible UTF-8 check.
// @solution-end

pub struct Solution;

// @solution-start remove
fn backtrack(bytes: &mut [u8], i: usize, result: &mut Vec<String>) {
    if i == bytes.len() {
        result.push(bytes.iter().map(|&b| char::from(b)).collect());
        return;
    }
    backtrack(bytes, i + 1, result); // keep this character
    let b = bytes[i];
    if b.is_ascii_alphabetic() {
        bytes[i] = if b.is_ascii_lowercase() {
            b.to_ascii_uppercase()
        } else {
            b.to_ascii_lowercase()
        };
        backtrack(bytes, i + 1, result); // switch its case
        bytes[i] = b; // restore
    }
}
// @solution-end

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        // @solution-start
        let mut bytes = s.into_bytes();
        let mut result = Vec::new();
        backtrack(&mut bytes, 0, &mut result);
        result
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> Vec<String> {
        let mut result = Solution::letter_case_permutation(s.to_string());
        result.sort(); // any order is accepted
        result
    }

    #[test]
    fn example_two_letters() {
        assert_eq!(run("a1b2"), vec!["A1B2", "A1b2", "a1B2", "a1b2"]);
    }

    #[test]
    fn example_one_letter() {
        assert_eq!(run("3z4"), vec!["3Z4", "3z4"]);
    }

    #[test]
    fn example_digits_only() {
        assert_eq!(run("12345"), vec!["12345"]);
    }

    #[test]
    fn uppercase_input() {
        assert_eq!(run("C"), vec!["C", "c"]);
    }

    #[test]
    fn twelve_letters() {
        let result = run("abcdefghijkl");
        assert_eq!(result.len(), 4096);
        let mut deduped = result.clone();
        deduped.dedup();
        assert_eq!(deduped.len(), 4096);
        assert!(result.contains(&"aBcDeFgHiJkL".to_string()));
    }
}
