//! # Problem 0455: Split Strings by Separator
//!
//! - **Difficulty:** Easy
//! - **Topics:** `flat_map` with `str::split`, `filter`, `map(String::from)`, collecting into `Vec<String>`
//! - **Source:** [LeetCode 2788. Split Strings by Separator](https://leetcode.com/problems/split-strings-by-separator/)
//!
//! ## Problem
//!
//! Given a list of strings `words` and a character `separator`, split every string at each
//! occurrence of `separator` and return all the resulting pieces **in order**, skipping the empty
//! pieces. The separator itself never appears in the output.
//!
//! Implement `Solution::split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  words = ["one.two.three", "four.five", "six"], separator = '.'
//! Output: ["one", "two", "three", "four", "five", "six"]
//!
//! Input:  words = ["$easy$", "$problem$"], separator = '$'
//! Output: ["easy", "problem"]
//!
//! Input:  words = ["|||"], separator = '|'
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `1 <= words.len() <= 100`
//! - `1 <= words[i].len() <= 20`
//! - Characters of `words[i]` are lowercase English letters or one of `. , | $ # @`.
//! - `separator` is one of `. , | $ # @`.
//!
//! ## Hints
//!
//! 1. `"a..b".split('.')` yields `"a"`, `""`, `"b"`: empty pieces appear between adjacent
//!    separators and at the ends.
//! 2. `flat_map(|w| w.split(separator))` turns the list of words into one iterator of pieces.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** split every word, flatten all the pieces into one stream, drop the empty ones and turn
//! the remaining `&str` pieces into owned `String`s.
//!
//! **Complexity:** `O(L)` time and space, where `L` is the total length of the words.
//!
//! **Rust concepts:**
//! - `flat_map` is `map` followed by `flatten`: each word becomes an iterator of pieces.
//! - The pieces borrow from `words`, so they must be converted (`String::from`) before `words` is
//!   dropped at the end of the function.
//! - `filter(|p| !p.is_empty())` receives `&&str`; auto-deref makes `p.is_empty()` work anyway.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        // @solution-start
        words
            .iter()
            .flat_map(|w| w.split(separator))
            .filter(|piece| !piece.is_empty())
            .map(String::from)
            .collect()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(words: &[&str], separator: char) -> Vec<String> {
        Solution::split_words_by_separator(words.iter().map(|w| w.to_string()).collect(), separator)
    }

    #[test]
    fn example_dots() {
        assert_eq!(
            run(&["one.two.three", "four.five", "six"], '.'),
            vec!["one", "two", "three", "four", "five", "six"]
        );
    }

    #[test]
    fn example_dollars() {
        assert_eq!(run(&["$easy$", "$problem$"], '$'), vec!["easy", "problem"]);
    }

    #[test]
    fn example_only_separators() {
        assert_eq!(run(&["|||"], '|'), Vec::<String>::new());
    }

    #[test]
    fn other_characters_are_kept() {
        assert_eq!(run(&["a.b#c", "#"], '#'), vec!["a.b", "c"]);
        assert_eq!(run(&["no-separator"], '@'), vec!["no-separator"]);
    }
}
