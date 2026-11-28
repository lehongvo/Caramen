// ==================================================================
// PRACTICE 0383: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0383_top_k_frequent_words.md
// Run the tests:          cargo test practice::p0383
// ==================================================================

//! # Problem 0383: Top K Frequent Words
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashMap<&str, usize>` borrowing words, a two-level comparator (count descending, word ascending), `truncate`
//! - **Source:** [LeetCode 692. Top K Frequent Words](https://leetcode.com/problems/top-k-frequent-words/)
//!
//! ## Problem
//!
//! Given a list of words and an integer `k`, return the `k` most frequent words, ordered by
//! frequency from highest to lowest. Words with the same frequency are ordered
//! **lexicographically** (alphabetically). Unlike "Top K Frequent Elements", the output order is
//! fully specified.
//!
//! LeetCode's follow-up asks for `O(n log k)` time.
//!
//! Implement `Solution::top_k_frequent(words: Vec<String>, k: i32) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  words = ["i", "love", "leetcode", "i", "love", "coding"], k = 2
//! Output: ["i", "love"]      (both occur twice; "i" < "love")
//!
//! Input:  words = ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], k = 4
//! Output: ["the", "is", "sunny", "day"]     (4, 3, 2 and 1 occurrences)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= words.len() <= 500`, `1 <= words[i].len() <= 10`, lowercase letters
//! - `1 <= k <=` number of distinct words
//!
//! ## Hints
//!
//! 1. Count with a `HashMap<&str, usize>` that borrows the words instead of cloning them.
//! 2. Sort the `(word, count)` pairs with `b.count.cmp(&a.count).then_with(|| a.word.cmp(b.word))`.
//! 3. For the follow-up, keep a heap of size `k` whose top is the **worst** of the current best
//!    `k`.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn example_one() {
        let words = strings(&["i", "love", "leetcode", "i", "love", "coding"]);
        assert_eq!(Solution::top_k_frequent(words, 2), strings(&["i", "love"]));
    }

    #[test]
    fn example_two() {
        let words = strings(&[
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ]);
        assert_eq!(
            Solution::top_k_frequent(words, 4),
            strings(&["the", "is", "sunny", "day"])
        );
    }

    #[test]
    fn ties_are_alphabetical() {
        let words = strings(&["b", "a", "c", "b", "a", "c"]);
        assert_eq!(
            Solution::top_k_frequent(words.clone(), 3),
            strings(&["a", "b", "c"])
        );
        assert_eq!(Solution::top_k_frequent(words, 1), strings(&["a"]));
    }

    #[test]
    fn five_hundred_words() {
        // "w00" .. "w24": word i occurs i + 1 times, plus filler words that occur once.
        let mut words: Vec<String> = (0..25)
            .flat_map(|i| std::iter::repeat_n(format!("w{i:02}"), i + 1))
            .collect();
        words.extend((0..175).map(|i| format!("z{i:03}")));
        assert_eq!(words.len(), 500);
        assert_eq!(
            Solution::top_k_frequent(words.clone(), 3),
            strings(&["w24", "w23", "w22"])
        );
        // Count 1: "w00" sorts before every "z..." filler.
        let all = Solution::top_k_frequent(words, 26);
        assert_eq!(all[24], "w00");
        assert_eq!(all[25], "z000");
    }
}
