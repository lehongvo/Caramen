// ==================================================================
// PRACTICE 0816: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0816_longest_string_chain.md
// Run the tests:          cargo test practice::p0816
// ==================================================================

//! # Problem 0816: Longest String Chain
//!
//! - **Difficulty:** Medium
//! - **Topics:** DP over words sorted by length, `HashMap<&str, i32>`, `sort_unstable_by_key`
//! - **Source:** [LeetCode 1048. Longest String Chain](https://leetcode.com/problems/longest-string-chain/)
//!
//! ## Problem
//!
//! Word `a` is a **predecessor** of word `b` if you can insert exactly one letter anywhere in `a`
//! (without reordering the other letters) to obtain `b`. For example `"abc"` is a predecessor of
//! `"abac"`, but `"cba"` is not a predecessor of `"bcad"`.
//!
//! A **word chain** is a sequence `w1, w2, ..., wk` (with `k >= 1`) in which each word is a
//! predecessor of the next one. A single word is a chain of length 1. Given a list of words,
//! return the length of the longest chain that uses only words from the list.
//!
//! Implement `Solution::longest_str_chain(words: Vec<String>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  words = ["a", "b", "ba", "bca", "bda", "bdca"]
//! Output: 4
//! Explanation: "a" -> "ba" -> "bda" -> "bdca".
//!
//! Input:  words = ["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]
//! Output: 5
//! Explanation: "xb" -> "xbc" -> "cxbc" -> "pcxbc" -> "pcxbcf".
//!
//! Input:  words = ["abcd", "dbqca"]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= words.len() <= 1000`
//! - `1 <= words[i].len() <= 16`
//! - Words contain only lowercase English letters.
//!
//! ## Hints
//!
//! 1. A predecessor is always exactly one letter shorter, so process words from shortest to
//!    longest.
//! 2. For a word of length `L`, there are only `L` candidate predecessors: delete each letter once.
//! 3. Remember the best chain ending at each word in a map keyed by the word.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(words: &[&str]) -> i32 {
        Solution::longest_str_chain(words.iter().map(|w| w.to_string()).collect())
    }

    #[test]
    fn example_four() {
        assert_eq!(run(&["a", "b", "ba", "bca", "bda", "bdca"]), 4);
    }

    #[test]
    fn example_five() {
        assert_eq!(run(&["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]), 5);
    }

    #[test]
    fn example_no_chain() {
        assert_eq!(run(&["abcd", "dbqca"]), 1);
    }

    #[test]
    fn order_of_letters_matters() {
        // "cba" is not a predecessor of "bcad".
        assert_eq!(run(&["cba", "bcad"]), 1);
    }

    #[test]
    fn full_ladder_of_sixteen() {
        let words: Vec<String> = (1..=16).map(|k| "a".repeat(k)).collect();
        assert_eq!(Solution::longest_str_chain(words), 16);
    }
}
