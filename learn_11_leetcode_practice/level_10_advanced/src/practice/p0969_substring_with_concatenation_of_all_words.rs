// ==================================================================
// PRACTICE 0969: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0969_substring_with_concatenation_of_all_words.md
// Run the tests:          cargo test practice::p0969
// ==================================================================

//! # Problem 0969: Substring with Concatenation of All Words
//!
//! - **Difficulty:** Hard
//! - **Topics:** sliding window measured in words, one pass per offset `0..word_len`, `HashMap<&str, usize>` with borrowed keys
//! - **Source:** [LeetCode 30. Substring with Concatenation of All Words](https://leetcode.com/problems/substring-with-concatenation-of-all-words/)
//!
//! ## Problem
//!
//! You are given a string `s` and an array `words` of strings that all have the **same length**. A
//! *concatenated string* is any string formed by joining all of `words` in some order, each word
//! used exactly as many times as it appears in `words`. Return the starting indices of every
//! substring of `s` that is a concatenated string, in **any order**.
//!
//! Implement `Solution::find_substring(s: String, words: Vec<String>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "barfoothefoobarman", words = ["foo", "bar"]
//! Output: [0, 9]        ("barfoo" and "foobar")
//!
//! Input:  s = "wordgoodgoodgoodbestword", words = ["word", "good", "best", "word"]
//! Output: []
//!
//! Input:  s = "barfoofoobarthefoobarman", words = ["bar", "foo", "the"]
//! Output: [6, 9, 12]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^4`, `1 <= words.len() <= 5000`, `1 <= words[i].len() <= 30`
//! - `s` and the words consist of lowercase English letters.
//!
//! ## Hints
//!
//! 1. With word length `w`, every candidate start belongs to one of `w` "lanes": `offset`,
//!    `offset + w`, `offset + 2w`, ...
//! 2. In each lane, slide a window of whole words: add the next word on the right; if it is not a
//!    word at all, reset the window; if it appears too often, drop words from the left.
//! 3. When the window holds exactly `words.len()` words, its start is an answer.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, words: &[&str]) -> Vec<i32> {
        let mut result =
            Solution::find_substring(s.to_string(), words.iter().map(|w| w.to_string()).collect());
        result.sort_unstable();
        result
    }

    fn naive(s: &str, words: &[&str]) -> Vec<i32> {
        let total: usize = words.iter().map(|w| w.len()).sum();
        let w = words[0].len();
        let mut sorted_words: Vec<&str> = words.to_vec();
        sorted_words.sort_unstable();
        (0..=s.len().saturating_sub(total))
            .filter(|&i| i + total <= s.len())
            .filter(|&i| {
                let mut pieces: Vec<&str> = (0..words.len())
                    .map(|k| &s[i + k * w..i + (k + 1) * w])
                    .collect();
                pieces.sort_unstable();
                pieces == sorted_words
            })
            .map(|i| i as i32)
            .collect()
    }

    #[test]
    fn examples() {
        assert_eq!(run("barfoothefoobarman", &["foo", "bar"]), vec![0, 9]);
        assert!(
            run(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "word"]
            )
            .is_empty()
        );
        assert_eq!(
            run("barfoofoobarthefoobarman", &["bar", "foo", "the"]),
            vec![6, 9, 12]
        );
    }

    #[test]
    fn repeated_words() {
        assert_eq!(run("aaaaaa", &["aa", "aa"]), vec![0, 1, 2]);
        assert_eq!(
            run(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "good"]
            ),
            vec![8]
        );
        assert!(run("ab", &["abc"]).is_empty());
    }

    #[test]
    fn matches_naive() {
        let s: String = (0..300)
            .map(|i: u32| if (i * 7 + i / 3) % 3 == 0 { 'a' } else { 'b' })
            .collect();
        for words in [
            vec!["ab", "ba"],
            vec!["a", "b", "a"],
            vec!["abb", "bab", "bba"],
        ] {
            assert_eq!(run(&s, &words), naive(&s, &words), "{words:?}");
        }
    }

    #[test]
    fn long_input() {
        let s = "ab".repeat(5_000);
        let words = vec!["ab"; 100];
        let result = run(&s, &words);
        assert_eq!(result.len(), (10_000 - 200) / 2 + 1);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 2);
    }
}
