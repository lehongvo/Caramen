//! # Problem 0180: Find the Index of the First Occurrence in a String
//!
//! - **Difficulty:** Easy
//! - **Topics:** comparing byte slices, `windows(n)`, `position`, `str::find`
//! - **Source:** [LeetCode 28. Find the Index of the First Occurrence in a String](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/)
//!
//! ## Problem
//!
//! Given two strings `haystack` and `needle`, return the index of the first position where `needle`
//! occurs in `haystack`, or `-1` if it does not occur at all.
//!
//! Implement `Solution::str_str(haystack: String, needle: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  haystack = "sadbutsad", needle = "sad"
//! Output: 0                (it occurs at 0 and 6; the first one counts)
//!
//! Input:  haystack = "leetcode", needle = "leeto"
//! Output: -1
//!
//! Input:  haystack = "hello", needle = "ll"
//! Output: 2
//! ```
//!
//! ## Constraints
//!
//! - `1 <= haystack.len(), needle.len() <= 10^4`
//! - Both strings consist of lowercase English letters.
//!
//! ## Hints
//!
//! 1. `haystack.as_bytes().windows(needle.len())` yields every substring of the right length as a
//!    `&[u8]`; slices can be compared with `==`.
//! 2. `position` returns the index of the first window that matches.
//! 3. The library way is `haystack.find(needle.as_str())`, which returns a byte index.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** slide a window of `needle.len()` bytes over the haystack and return the index of the
//! first window equal to the needle. If the needle is longer than the haystack, `windows` produces
//! nothing and the answer is `-1`.
//!
//! **Complexity:** `O(n * m)` time in the worst case, `O(1)` extra space. (`str::find` uses the
//! Two-Way algorithm, which is `O(n + m)`.)
//!
//! **Rust concepts:**
//! - `windows(size)` panics for `size == 0`; the constraints guarantee a non-empty needle.
//! - `&[u8] == &[u8]` compares lengths and contents.
//! - `Option::map_or(-1, ..)` turns "not found" into LeetCode's `-1` convention. Idiomatic Rust APIs
//!   would return `Option<usize>` instead, like `str::find` does.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // @solution-start
        haystack
            .as_bytes()
            .windows(needle.len())
            .position(|window| window == needle.as_bytes())
            .map_or(-1, |i| i as i32)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(haystack: &str, needle: &str) -> i32 {
        Solution::str_str(haystack.to_string(), needle.to_string())
    }

    #[test]
    fn example_at_start() {
        assert_eq!(run("sadbutsad", "sad"), 0);
    }

    #[test]
    fn example_not_found() {
        assert_eq!(run("leetcode", "leeto"), -1);
    }

    #[test]
    fn example_in_the_middle() {
        assert_eq!(run("hello", "ll"), 2);
    }

    #[test]
    fn needle_longer_or_equal() {
        assert_eq!(run("ab", "abc"), -1);
        assert_eq!(run("abc", "abc"), 0);
    }

    #[test]
    fn match_at_the_end() {
        assert_eq!(run("aaab", "ab"), 2);
    }

    #[test]
    fn large_input_matches_find() {
        let haystack = format!("{}b", "a".repeat(9_999));
        let needle = format!("{}b", "a".repeat(99));
        assert_eq!(run(&haystack, &needle), 9_900);
        assert_eq!(haystack.find(needle.as_str()), Some(9_900));
    }
}
