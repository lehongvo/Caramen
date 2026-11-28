//! # Problem 0349: Word Pattern
//!
//! - **Difficulty:** Easy
//! - **Topics:** `HashMap<char, &str>` + `HashMap<&str, char>`, borrowing keys from the input, `split_whitespace`
//! - **Source:** [LeetCode 290. Word Pattern](https://leetcode.com/problems/word-pattern/)
//!
//! ## Problem
//!
//! You are given a `pattern` of lowercase letters and a string `s` of words separated by single
//! spaces. `s` **follows** the pattern when there is a one-to-one correspondence between letters
//! and words:
//!
//! - the number of words equals the number of letters;
//! - every occurrence of a letter corresponds to the same word;
//! - two different letters never correspond to the same word.
//!
//! Return `true` if `s` follows `pattern`.
//!
//! Implement `Solution::word_pattern(pattern: String, s: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  pattern = "abba", s = "dog cat cat dog"
//! Output: true
//!
//! Input:  pattern = "abba", s = "dog cat cat fish"
//! Output: false     ('a' would be both "dog" and "fish")
//!
//! Input:  pattern = "aaaa", s = "dog cat cat dog"
//! Output: false
//!
//! Input:  pattern = "abba", s = "dog dog dog dog"
//! Output: false     ('a' and 'b' would both be "dog")
//! ```
//!
//! ## Constraints
//!
//! - `1 <= pattern.len() <= 300`, lowercase English letters
//! - `1 <= s.len() <= 3000`, lowercase letters and single spaces, no leading or trailing space
//!
//! ## Hints
//!
//! 1. Collect the words into a `Vec<&str>` first and compare its length with the pattern's.
//! 2. As in "Isomorphic Strings", keep one map per direction.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** split `s` into words and reject a length mismatch. Then walk letters and words together
//! and maintain `letter -> word` and `word -> letter`; each pair must agree with both maps.
//!
//! **Complexity:** `O(n + |s|)` average time, `O(n)` space (hashing a word costs its length).
//!
//! **Rust concepts:**
//! - The maps store `&str` slices that borrow from `s`, so no word is copied. This works because
//!   `s` lives until the end of the function, longer than both maps.
//! - `entry(letter).or_insert(word)` returns `&mut &str`; comparing `*... == word` checks the
//!   stored word.
//! - Without the length check, `zip` would silently stop at the shorter input and accept
//!   `"aaaa"` / `"dog"`.
// @solution-end

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        // @solution-start
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() != pattern.chars().count() {
            return false;
        }
        let mut letter_to_word: HashMap<char, &str> = HashMap::new();
        let mut word_to_letter: HashMap<&str, char> = HashMap::new();
        pattern.chars().zip(words).all(|(letter, word)| {
            *letter_to_word.entry(letter).or_insert(word) == word
                && *word_to_letter.entry(word).or_insert(letter) == letter
        })
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn follows(pattern: &str, s: &str) -> bool {
        Solution::word_pattern(pattern.to_string(), s.to_string())
    }

    #[test]
    fn example_one() {
        assert!(follows("abba", "dog cat cat dog"));
    }

    #[test]
    fn example_two() {
        assert!(!follows("abba", "dog cat cat fish"));
    }

    #[test]
    fn example_three() {
        assert!(!follows("aaaa", "dog cat cat dog"));
    }

    #[test]
    fn example_four_two_letters_one_word() {
        assert!(!follows("abba", "dog dog dog dog"));
    }

    #[test]
    fn length_mismatch() {
        assert!(!follows("aaaa", "dog"));
        assert!(!follows("a", "dog dog"));
        assert!(follows("a", "dog"));
    }

    #[test]
    fn long_pattern() {
        let pattern: String = (0..300).map(|i| (b'a' + (i % 26) as u8) as char).collect();
        let s: Vec<String> = (0..300).map(|i| format!("w{}", i % 26)).collect();
        assert!(Solution::word_pattern(pattern.clone(), s.join(" ")));
        let mut wrong = s.clone();
        wrong[299] = "w0".to_string();
        assert!(!Solution::word_pattern(pattern, wrong.join(" ")));
    }
}
