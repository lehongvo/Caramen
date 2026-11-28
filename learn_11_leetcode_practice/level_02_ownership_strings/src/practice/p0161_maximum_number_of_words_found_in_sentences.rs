// ==================================================================
// PRACTICE 0161: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0161_maximum_number_of_words_found_in_sentences.md
// Run the tests:          cargo test practice::p0161
// ==================================================================

//! # Problem 0161: Maximum Number of Words Found in Sentences
//!
//! - **Difficulty:** Easy
//! - **Topics:** iterating a `Vec<String>` by reference, counting bytes, `max`
//! - **Source:** [LeetCode 2114. Maximum Number of Words Found in Sentences](https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/)
//!
//! ## Problem
//!
//! Every sentence in `sentences` is a list of lowercase words separated by single spaces, with no
//! leading or trailing space. Return the largest number of words found in a single sentence.
//!
//! Implement `Solution::most_words_found(sentences: Vec<String>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  sentences = ["alice and bob love leetcode", "i think so too", "this is great thanks very much"]
//! Output: 6
//! Explanation: the sentences have 5, 4 and 6 words.
//!
//! Input:  sentences = ["please wait", "continue to fight", "continue to win"]
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= sentences.len() <= 100`
//! - `1 <= sentences[i].len() <= 100`
//! - Words are separated by single spaces; no leading or trailing spaces.
//!
//! ## Hints
//!
//! 1. With single spaces and no spaces at the ends, a sentence has (number of spaces + 1) words.
//! 2. `sentences.iter()` borrows each `String`; `s.bytes().filter(|&b| b == b' ').count()` counts
//!    its spaces.

pub struct Solution;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(sentences: &[&str]) -> i32 {
        Solution::most_words_found(sentences.iter().map(|s| s.to_string()).collect())
    }

    #[test]
    fn example_three_sentences() {
        assert_eq!(
            run(&[
                "alice and bob love leetcode",
                "i think so too",
                "this is great thanks very much"
            ]),
            6
        );
    }

    #[test]
    fn example_tie() {
        assert_eq!(
            run(&["please wait", "continue to fight", "continue to win"]),
            3
        );
    }

    #[test]
    fn single_word_sentences() {
        assert_eq!(run(&["a", "b"]), 1);
    }

    #[test]
    fn largest_input() {
        let long = vec!["a"; 50].join(" ");
        let mut sentences = vec!["x y"; 99];
        sentences.push(&long);
        assert_eq!(run(&sentences), 50);
    }
}
