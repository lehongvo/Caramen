// ==================================================================
// PRACTICE 0370: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0370_reorder_data_in_log_files.md
// Run the tests:          cargo test practice::p0370
// ==================================================================

//! # Problem 0370: Reorder Data in Log Files
//!
//! - **Difficulty:** Medium
//! - **Topics:** a **stable** `sort_by` with a custom `Ordering`, `split_once`, matching on a pair of `bool`s
//! - **Source:** [LeetCode 937. Reorder Data in Log Files](https://leetcode.com/problems/reorder-data-in-log-files/)
//!
//! ## Problem
//!
//! Each log is a string of words separated by single spaces. The first word is an
//! **identifier**; the remaining words form the **content**. There are two kinds of logs:
//!
//! - **letter-logs**: all words of the content are lowercase letters;
//! - **digit-logs**: all words of the content are digits.
//!
//! Reorder the logs so that:
//!
//! 1. every letter-log comes before every digit-log;
//! 2. letter-logs are sorted by content; when two contents are equal, by identifier;
//! 3. digit-logs keep their **original relative order**.
//!
//! Return the reordered logs.
//!
//! Implement `Solution::reorder_log_files(logs: Vec<String>) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  ["dig1 8 1 5 1", "let1 art can", "dig2 3 6", "let2 own kit dig", "let3 art zero"]
//! Output: ["let1 art can", "let3 art zero", "let2 own kit dig", "dig1 8 1 5 1", "dig2 3 6"]
//!
//! Input:  ["a1 9 2 3 1", "g1 act car", "zo4 4 7", "ab1 off key dog", "a8 act zoo"]
//! Output: ["g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= logs.len() <= 100`, `3 <= logs[i].len() <= 100`
//! - Every log has an identifier and at least one content word, separated by single spaces.
//!
//! ## Hints
//!
//! 1. A log is a digit-log when its content starts with a digit.
//! 2. Write one comparator: letter vs digit decides by kind; two letter-logs compare
//!    `(content, identifier)`; two digit-logs are `Equal`.
//! 3. Rule 3 then comes for free from a **stable** sort, which never reorders equal elements.

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
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
        let logs = strings(&[
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero",
        ]);
        assert_eq!(
            Solution::reorder_log_files(logs),
            strings(&[
                "let1 art can",
                "let3 art zero",
                "let2 own kit dig",
                "dig1 8 1 5 1",
                "dig2 3 6"
            ])
        );
    }

    #[test]
    fn example_two() {
        let logs = strings(&[
            "a1 9 2 3 1",
            "g1 act car",
            "zo4 4 7",
            "ab1 off key dog",
            "a8 act zoo",
        ]);
        assert_eq!(
            Solution::reorder_log_files(logs),
            strings(&[
                "g1 act car",
                "a8 act zoo",
                "ab1 off key dog",
                "a1 9 2 3 1",
                "zo4 4 7"
            ])
        );
    }

    #[test]
    fn equal_contents_sorted_by_identifier() {
        let logs = strings(&["b art", "a art", "c 1", "a 0"]);
        assert_eq!(
            Solution::reorder_log_files(logs),
            strings(&["a art", "b art", "c 1", "a 0"])
        );
    }

    #[test]
    fn many_digit_logs_keep_their_order() {
        let logs: Vec<String> = (0..100)
            .map(|i| {
                if i % 2 == 0 {
                    format!("d{i} {}", 99 - i)
                } else {
                    format!("l{i} word")
                }
            })
            .collect();
        let result = Solution::reorder_log_files(logs);
        assert_eq!(result[0], "l1 word");
        assert_eq!(result[1], "l11 word");
        assert_eq!(result[50], "d0 99");
        assert_eq!(result[99], "d98 1");
    }
}
