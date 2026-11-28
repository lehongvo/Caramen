// ==================================================================
// PRACTICE 0953: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0953_stream_of_characters.md
// Run the tests:          cargo test practice::p0953
// ==================================================================

//! # Problem 0953: Stream of Characters
//!
//! - **Difficulty:** Hard
//! - **Topics:** a trie of **reversed** words, an arena of nodes in a `Vec` with `u32` child indices, bounded history buffer
//! - **Source:** [LeetCode 1032. Stream of Characters](https://leetcode.com/problems/stream-of-characters/)
//!
//! ## Problem
//!
//! Letters arrive one at a time. After each letter, report whether some **suffix** of the letters
//! received so far is one of the given words.
//!
//! Implement the `StreamChecker` struct:
//!
//! - `StreamChecker::new(words: Vec<String>) -> Self`.
//! - `query(&mut self, letter: char) -> bool`: appends `letter` to the stream and returns `true` if a
//!   suffix of the stream (ending with this letter) is in `words`.
//!
//! ## Examples
//!
//! ```text
//! let mut s = StreamChecker::new(vec!["abc", "xyz"]);
//! s.query('a') => false
//! s.query('x') => false
//! s.query('y') => false
//! s.query('z') => true      ("xyz" is a suffix of "axyz")
//! ```
//!
//! ## Constraints
//!
//! - `1 <= words.len() <= 2000`, `1 <= words[i].len() <= 200`
//! - Words and letters are lowercase English letters.
//! - At most `4 * 10^4` calls to `query`.
//!
//! ## Hints
//!
//! 1. Insert every word **reversed** into a trie.
//! 2. On each query, walk the trie from the newest letter backwards through the stream; stop at the
//!    first node that ends a word (return `true`) or when there is no child.
//! 3. You never need more history than the longest word: trim the buffer from time to time.

pub struct StreamChecker {}

impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        todo!("Write your solution here")
    }

    pub fn query(&mut self, letter: char) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checker(words: &[&str]) -> StreamChecker {
        StreamChecker::new(words.iter().map(|w| w.to_string()).collect())
    }

    fn feed(checker: &mut StreamChecker, letters: &str) -> Vec<bool> {
        letters.chars().map(|c| checker.query(c)).collect()
    }

    #[test]
    fn example() {
        let mut s = checker(&["abc", "xyz"]);
        assert_eq!(feed(&mut s, "axyz"), [false, false, false, true]);
    }

    #[test]
    fn longer_example() {
        let mut s = checker(&["cd", "f", "kl"]);
        assert_eq!(
            feed(&mut s, "abcdefghijkl"),
            [
                false, false, false, true, false, true, false, false, false, false, false, true
            ]
        );
    }

    #[test]
    fn words_that_are_suffixes_of_each_other() {
        let mut s = checker(&["ab", "b", "aab"]);
        assert_eq!(feed(&mut s, "aab"), [false, false, true]);
        assert_eq!(feed(&mut s, "c"), [false]);
    }

    #[test]
    fn long_stream_keeps_working_after_trimming() {
        let mut s = checker(&["needle"]);
        let haystack = "x".repeat(10_000) + "needle" + &"y".repeat(10_000) + "needle";
        let hits: Vec<usize> = feed(&mut s, &haystack)
            .into_iter()
            .enumerate()
            .filter(|&(_, hit)| hit)
            .map(|(i, _)| i)
            .collect();
        assert_eq!(hits, vec![10_005, 20_011]);
    }
}
