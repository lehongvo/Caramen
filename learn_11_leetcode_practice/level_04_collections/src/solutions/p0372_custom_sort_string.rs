//! # Problem 0372: Custom Sort String
//!
//! - **Difficulty:** Medium
//! - **Topics:** a rank table `[usize; 26]`, `sort_by_key` on `Vec<u8>`, `String::from_utf8`, checking answers that allow many outputs
//! - **Source:** [LeetCode 791. Custom Sort String](https://leetcode.com/problems/custom-sort-string/)
//!
//! ## Problem
//!
//! You are given a string `order` whose characters are all **distinct**, and a string `s`. Both
//! contain lowercase English letters. `order` defines a custom alphabet: if `x` comes before `y` in
//! `order`, then every `x` must come before every `y` in the answer.
//!
//! Return any permutation of `s` that respects this rule. Characters of `s` that do not appear in
//! `order` may be placed anywhere.
//!
//! Implement `Solution::custom_sort_string(order: String, s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  order = "cba", s = "abcd"
//! Output: "cbad"      ("dcba", "cdba", "cbda" are also accepted)
//!
//! Input:  order = "bcafg", s = "abcd"
//! Output: "bcad"      (any position for 'd' is fine)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= order.len() <= 26`, `1 <= s.len() <= 200`
//! - `order` and `s` contain lowercase English letters; the characters of `order` are distinct.
//!
//! ## Hints
//!
//! 1. Give every letter a rank: its position in `order`, or `26` if it is not in `order`.
//! 2. Sort the bytes of `s` by rank.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build `rank[letter] = position in order` (default `26`, after every ordered letter),
//! then sort the bytes of `s` by that rank and turn them back into a `String`.
//!
//! **Complexity:** `O(|order| + |s| log |s|)` time, `O(|s|)` space. (Counting the letters and
//! emitting them in `order` order gives `O(|order| + |s|)`.)
//!
//! **Rust concepts:**
//! - `s.into_bytes()` takes the string's buffer as a `Vec<u8>` without copying.
//! - `(b - b'a') as usize` turns a lowercase ASCII byte into an index `0..26`.
//! - `String::from_utf8(bytes)` returns a `Result` because arbitrary bytes might not be UTF-8;
//!   rearranged ASCII bytes always are, so `unwrap_or_default()` never hits its fallback.
//! - The tests accept every valid answer: they check that the output is a permutation of `s` and
//!   that ranks never decrease among the letters that appear in `order`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        // @solution-start
        let mut rank = [26usize; 26];
        for (i, b) in order.bytes().enumerate() {
            rank[(b - b'a') as usize] = i;
        }
        let mut bytes = s.into_bytes();
        bytes.sort_by_key(|&b| rank[(b - b'a') as usize]);
        String::from_utf8(bytes).unwrap_or_default()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that `result` is a valid answer for `(order, s)`.
    fn is_valid(order: &str, s: &str, result: &str) -> bool {
        let mut a: Vec<char> = s.chars().collect();
        let mut b: Vec<char> = result.chars().collect();
        a.sort_unstable();
        b.sort_unstable();
        let ranks: Vec<usize> = result.chars().filter_map(|c| order.find(c)).collect();
        a == b && ranks.windows(2).all(|w| w[0] <= w[1])
    }

    fn check(order: &str, s: &str) {
        let result = Solution::custom_sort_string(order.to_string(), s.to_string());
        assert!(is_valid(order, s, &result), "invalid answer {result:?}");
    }

    #[test]
    fn example_one() {
        // The checker itself: wrong order, missing letter, and another valid answer.
        assert!(!is_valid("cba", "abcd", "abcd"));
        assert!(!is_valid("cba", "abcd", "cba"));
        assert!(is_valid("cba", "abcd", "dcba"));
        check("cba", "abcd");
    }

    #[test]
    fn example_two() {
        check("bcafg", "abcd");
    }

    #[test]
    fn no_letter_of_s_is_ordered() {
        check("xyz", "hello");
    }

    #[test]
    fn repeated_letters_and_full_alphabet() {
        check("zyxwvutsrqponmlkjihgfedcba", &"abcxyz".repeat(33));
        check("q", "aqbqcq");
        let result = Solution::custom_sort_string("ba".to_string(), "aabb".to_string());
        assert_eq!(result, "bbaa");
    }
}
