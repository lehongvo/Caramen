//! # Problem 0712: Longest Substring Without Repeating Characters
//!
//! - **Difficulty:** Medium
//! - **Topics:** variable-size sliding window, last-seen table `[Option<usize>; 256]`, `if let`
//! - **Source:** [LeetCode 3. Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)
//!
//! ## Problem
//!
//! Given a string `s`, return the length of the longest **substring** (a contiguous block of
//! characters) in which no character appears twice.
//!
//! Implement `Solution::length_of_longest_substring(s: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "abcabcbb"
//! Output: 3             ("abc")
//!
//! Input:  s = "bbbbb"
//! Output: 1             ("b")
//!
//! Input:  s = "pwwkew"
//! Output: 3             ("wke"; "pwke" is not contiguous, so it does not count)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= s.len() <= 5 * 10^4`
//! - `s` consists of English letters, digits, symbols and spaces (all ASCII).
//!
//! ## Hints
//!
//! 1. Keep a window `start..=i` that never contains a repeated character, and grow it one character
//!    at a time.
//! 2. When the new character already appears inside the window, where must the window start now?
//! 3. Remember the last index where each byte was seen.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** variable-size sliding window: the window `start..=i` holds distinct characters. When
//! the byte at `i` was last seen at index `prev` and `prev` is inside the window, the window must
//! start right after it: `start = prev + 1`. If `prev` is older than `start`, it is outside the
//! window and changes nothing, which `start = start.max(prev + 1)` expresses in one line (without
//! the `max`, `"abba"` would move `start` backwards). After that, the window is valid again and
//! `i + 1 - start` is a candidate length.
//!
//! **Complexity:** `O(n)` time, `O(1)` space (a fixed table of 256 entries).
//!
//! **Rust concepts:**
//! - `[None; 256]` creates an array of `Option<usize>`; arrays can be built this way because
//!   `Option<usize>` is `Copy`.
//! - `if let Some(prev) = last_seen[b as usize]` handles "seen before" without a sentinel value.
//! - Indexing with `b as usize` is always in bounds for a `u8` and a table of 256 entries.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // @solution-start
        let mut last_seen: [Option<usize>; 256] = [None; 256];
        let mut start = 0;
        let mut best = 0;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            if let Some(prev) = last_seen[b as usize] {
                start = start.max(prev + 1); // only move forward
            }
            last_seen[b as usize] = Some(i);
            best = best.max(i + 1 - start);
        }
        best as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> i32 {
        Solution::length_of_longest_substring(s.to_string())
    }

    #[test]
    fn example_abc() {
        assert_eq!(run("abcabcbb"), 3);
    }

    #[test]
    fn example_all_same() {
        assert_eq!(run("bbbbb"), 1);
    }

    #[test]
    fn example_pwwkew() {
        assert_eq!(run("pwwkew"), 3);
    }

    #[test]
    fn empty_and_space() {
        assert_eq!(run(""), 0);
        assert_eq!(run(" "), 1);
    }

    #[test]
    fn stale_index_outside_window() {
        // When the second 'a' arrives, its previous index 0 is already outside the window "b".
        assert_eq!(run("abba"), 2);
        assert_eq!(run("dvdf"), 3);
    }

    #[test]
    fn long_repeating_cycle() {
        // The 95 printable ASCII characters repeated: the longest window holds each one once.
        let s: String = (0..50_000)
            .map(|i| char::from(b' ' + (i % 95) as u8))
            .collect();
        assert_eq!(run(&s), 95);
    }
}
