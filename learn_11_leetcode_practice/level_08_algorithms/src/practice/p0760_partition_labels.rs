// ==================================================================
// PRACTICE 0760: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0760_partition_labels.md
// Run the tests:          cargo test practice::p0760
// ==================================================================

//! # Problem 0760: Partition Labels
//!
//! - **Difficulty:** Medium
//! - **Topics:** last occurrence table, greedy cut, interval-style merging on a string
//! - **Source:** [LeetCode 763. Partition Labels](https://leetcode.com/problems/partition-labels/)
//!
//! ## Problem
//!
//! Given a string `s` of lowercase English letters, split it into as many consecutive parts as
//! possible so that every letter appears in **at most one** part. Concatenating the parts in order
//! must give back `s`. Return the sizes of the parts, in order.
//!
//! Implement `Solution::partition_labels(s: String) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "ababcbacadefegdehijhklij"
//! Output: [9, 7, 8]     ("ababcbaca", "defegde", "hijhklij")
//!
//! Input:  s = "eccbbbbdec"
//! Output: [10]          ('e' appears at both ends, so nothing can be cut)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 500`
//! - `s` contains only lowercase English letters.
//!
//! ## Hints
//!
//! 1. A part that contains a letter must extend at least to that letter's **last** occurrence.
//! 2. Scan left to right, extending the current part's end; cut as soon as you reach it.

pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> Vec<i32> {
        Solution::partition_labels(s.to_string())
    }

    #[test]
    fn example_three_parts() {
        assert_eq!(run("ababcbacadefegdehijhklij"), vec![9, 7, 8]);
    }

    #[test]
    fn example_one_part() {
        assert_eq!(run("eccbbbbdec"), vec![10]);
    }

    #[test]
    fn all_distinct_letters() {
        assert_eq!(run("abc"), vec![1, 1, 1]);
        assert_eq!(run("a"), vec![1]);
    }

    #[test]
    fn repeated_letter_then_block() {
        assert_eq!(run("abacc"), vec![3, 2]);
    }

    #[test]
    fn long_input() {
        // 500 letters: the alphabet once (26 parts), then 474 'z's join the last part.
        let s = format!("{}{}", ('a'..='z').collect::<String>(), "z".repeat(474));
        let mut expected = vec![1; 25];
        expected.push(475);
        assert_eq!(run(&s), expected);
    }
}
