//! # Problem 0717: Permutation in String
//!
//! - **Difficulty:** Medium
//! - **Topics:** fixed-size sliding window, letter counts `[u32; 26]`, comparing arrays with `==`
//! - **Source:** [LeetCode 567. Permutation in String](https://leetcode.com/problems/permutation-in-string/)
//!
//! ## Problem
//!
//! Given two strings `s1` and `s2` of lowercase English letters, return `true` if `s2` contains a
//! **permutation** of `s1` as a contiguous substring, i.e. some substring of `s2` uses exactly the
//! same letters as `s1`, each the same number of times, in any order. Otherwise return `false`.
//!
//! Implement `Solution::check_inclusion(s1: String, s2: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s1 = "ab", s2 = "eidbaooo"
//! Output: true          ("ba" starts at index 3)
//!
//! Input:  s1 = "ab", s2 = "eidboaoo"
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s1.len(), s2.len() <= 10^4`
//! - Both strings contain only lowercase English letters.
//!
//! ## Hints
//!
//! 1. Two strings are permutations of each other exactly when their letter counts are equal.
//! 2. Slide a window of length `s1.len()` over `s2` and keep its letter counts up to date.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** fixed-size sliding window: count the letters of `s1` in `need`. Slide a window of
//! the same length over `s2`, adding the entering letter and removing the letter that falls out
//! (once the window is full). Whenever `window == need`, that window is a permutation of `s1`.
//!
//! **Complexity:** `O(26 * n)` time, which is `O(n)` (each comparison checks 26 counters),
//! `O(1)` space.
//!
//! **Rust concepts:**
//! - Arrays implement `PartialEq`, so `window == need` compares all 26 counters at once.
//! - A small helper `fn letter(b: u8) -> usize` keeps the `b - b'a'` conversion in one place.
// @solution-end

pub struct Solution;

// @solution-start remove
fn letter(b: u8) -> usize {
    usize::from(b - b'a')
}
// @solution-end

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // @solution-start
        let (pattern, text) = (s1.as_bytes(), s2.as_bytes());
        let m = pattern.len();
        if m > text.len() {
            return false;
        }
        let mut need = [0u32; 26];
        for &b in pattern {
            need[letter(b)] += 1;
        }
        let mut window = [0u32; 26];
        for (i, &b) in text.iter().enumerate() {
            window[letter(b)] += 1;
            if i >= m {
                window[letter(text[i - m])] -= 1; // the letter that left the window
            }
            if window == need {
                return true;
            }
        }
        false
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s1: &str, s2: &str) -> bool {
        Solution::check_inclusion(s1.to_string(), s2.to_string())
    }

    #[test]
    fn example_found() {
        assert!(run("ab", "eidbaooo"));
    }

    #[test]
    fn example_not_found() {
        assert!(!run("ab", "eidboaoo"));
    }

    #[test]
    fn pattern_longer_than_text() {
        assert!(!run("abc", "ab"));
    }

    #[test]
    fn identical_strings() {
        assert!(run("a", "a"));
    }

    #[test]
    fn match_at_the_end() {
        assert!(run("adc", "dcda"));
    }

    #[test]
    fn repeated_letters_must_match_counts() {
        // Windows of "abbab" are "abb", "bba", "bab": none has two 'a's.
        assert!(!run("aab", "abbab"));
        assert!(run("aab", "abbaab"));
    }

    #[test]
    fn long_text_with_match_at_the_end() {
        let text = format!("{}xyz", "a".repeat(9_997));
        assert!(run("zyx", &text));
        assert!(!run("zyy", &text));
    }
}
