// ==================================================================
// PRACTICE 0142: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0142_merge_strings_alternately.md
// Run the tests:          cargo test practice::p0142
// ==================================================================

//! # Problem 0142: Merge Strings Alternately
//!
//! - **Difficulty:** Easy
//! - **Topics:** two `Chars` iterators, `String::with_capacity`, `String: Extend<char>`, `Option` as an iterator
//! - **Source:** [LeetCode 1768. Merge Strings Alternately](https://leetcode.com/problems/merge-strings-alternately/)
//!
//! ## Problem
//!
//! Merge two words by taking characters alternately: first character of `word1`, first character of
//! `word2`, second character of `word1`, second character of `word2`, and so on. When one word runs
//! out of characters, append the rest of the other word unchanged.
//!
//! Implement `Solution::merge_alternately(word1: String, word2: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  word1 = "abc", word2 = "pqr"
//! Output: "apbqcr"
//!
//! Input:  word1 = "ab", word2 = "pqrs"
//! Output: "apbqrs"
//!
//! Input:  word1 = "abcd", word2 = "pq"
//! Output: "apbqcd"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= word1.len(), word2.len() <= 100`
//! - Both words consist of lowercase English letters.
//!
//! ## Hints
//!
//! 1. Create two iterators, `word1.chars()` and `word2.chars()`, and call `next()` on each in turn.
//! 2. Stop when **both** return `None`.
//! 3. `String` implements `Extend<char>`, and an `Option<char>` is an iterator of zero or one `char`,
//!    so `out.extend(a.next())` pushes the character only if there is one.

pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn merge(a: &str, b: &str) -> String {
        Solution::merge_alternately(a.to_string(), b.to_string())
    }

    #[test]
    fn example_equal_lengths() {
        assert_eq!(merge("abc", "pqr"), "apbqcr");
    }

    #[test]
    fn example_second_longer() {
        assert_eq!(merge("ab", "pqrs"), "apbqrs");
    }

    #[test]
    fn example_first_longer() {
        assert_eq!(merge("abcd", "pq"), "apbqcd");
    }

    #[test]
    fn single_letters() {
        assert_eq!(merge("a", "z"), "az");
        assert_eq!(merge("a", "xyz"), "axyz");
    }

    #[test]
    fn longest_inputs() {
        let result = merge(&"a".repeat(100), &"b".repeat(50));
        assert_eq!(result.len(), 150);
        assert_eq!(&result[..4], "abab");
        assert_eq!(&result[100..], "a".repeat(50));
    }
}
