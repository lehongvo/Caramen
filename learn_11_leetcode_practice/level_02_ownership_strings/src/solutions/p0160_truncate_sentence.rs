//! # Problem 0160: Truncate Sentence
//!
//! - **Difficulty:** Easy
//! - **Topics:** finding a byte index with `match_indices`, `String::truncate` on an owned `String`
//! - **Source:** [LeetCode 1816. Truncate Sentence](https://leetcode.com/problems/truncate-sentence/)
//!
//! ## Problem
//!
//! A sentence is a list of words separated by single spaces, with no leading or trailing space; every
//! word consists of English letters. Keep only the first `k` words and return the shortened sentence.
//!
//! Implement `Solution::truncate_sentence(s: String, k: i32) -> String`.
//!
//! The function owns `s`, so the cheapest solution cuts `s` itself instead of building a new string.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "Hello how are you Contestant", k = 4
//! Output: "Hello how are you"
//!
//! Input:  s = "What is the solution to this problem", k = 4
//! Output: "What is the solution"
//!
//! Input:  s = "chopper is not a tanuki", k = 5
//! Output: "chopper is not a tanuki"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 500`
//! - `1 <= k <= number of words in s`
//! - `s` consists of English letters and single spaces, with no leading or trailing space.
//!
//! ## Hints
//!
//! 1. The first `k` words end right before the `k`-th space (if there is one).
//! 2. `s.match_indices(' ')` yields `(byte_index, " ")` for every space; `.nth(k - 1)` is the `k`-th.
//! 3. `String::truncate(len)` shortens a `String` in place.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** look for the `k`-th space. If it exists, everything before it is the answer, so
//! truncate the owned string at that byte index; otherwise the sentence has exactly `k` words and is
//! returned as it is.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space (the input buffer is reused).
//!
//! **Rust concepts:**
//! - The search borrows `s` immutably and produces a plain `usize`; that borrow ends before the
//!   mutable `truncate` call, so the two do not conflict.
//! - `String::truncate` panics if the index is not a character boundary; a space's byte index always
//!   is.
//! - A word-based alternative is `s.split(' ').take(k).collect::<Vec<_>>().join(" ")`, which
//!   allocates a new `String`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        // @solution-start
        let mut s = s;
        if let Some((cut, _)) = s.match_indices(' ').nth(k as usize - 1) {
            s.truncate(cut);
        }
        s
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, k: i32) -> String {
        Solution::truncate_sentence(s.to_string(), k)
    }

    #[test]
    fn example_drop_last_word() {
        assert_eq!(run("Hello how are you Contestant", 4), "Hello how are you");
    }

    #[test]
    fn example_drop_several_words() {
        assert_eq!(run("What is the solution to this problem", 4), "What is the solution");
    }

    #[test]
    fn example_keep_everything() {
        assert_eq!(run("chopper is not a tanuki", 5), "chopper is not a tanuki");
    }

    #[test]
    fn keep_one_word() {
        assert_eq!(run("one two", 1), "one");
        assert_eq!(run("one", 1), "one");
    }

    #[test]
    fn long_sentence() {
        let s = vec!["ab"; 166].join(" ");
        assert_eq!(run(&s, 100), vec!["ab"; 100].join(" "));
    }
}
