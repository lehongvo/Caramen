//! # Problem 0716: Longest Repeating Character Replacement
//!
//! - **Difficulty:** Medium
//! - **Topics:** sliding window, count array `[usize; 26]`, the "window never shrinks" trick
//! - **Source:** [LeetCode 424. Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/)
//!
//! ## Problem
//!
//! You are given a string `s` of uppercase English letters and an integer `k`. You may perform at
//! most `k` operations; one operation replaces any single character of `s` by any other uppercase
//! letter. Return the length of the longest substring made of one repeated letter that you can
//! obtain.
//!
//! Implement `Solution::character_replacement(s: String, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "ABAB", k = 2
//! Output: 4             (replace both 'A's, or both 'B's)
//!
//! Input:  s = "AABABBA", k = 1
//! Output: 4             (replace the 'A' at index 3: "AABBBBA" contains "BBBB")
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^5`
//! - `s` contains only uppercase English letters.
//! - `0 <= k <= s.len()`
//!
//! ## Hints
//!
//! 1. A window can be made uniform when `window length - (count of its most frequent letter) <= k`.
//! 2. You only care about windows **longer** than the best one found so far. Does the window ever
//!    need to shrink?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** sliding window: a window is fixable when its length minus the count of its most
//! common letter is at most `k` (replace every other letter). Grow the window one letter at a
//! time and update `max_freq`, the highest count seen. If the window becomes unfixable, slide it:
//! drop one letter on the left so its length stays the same.
//!
//! The trick is that `max_freq` is never decreased, even when it becomes stale. That is fine:
//! the window only **grows** when some letter reaches a new highest count `max_freq`, and at that
//! moment the window really has length `max_freq + (at most k)`, so it is a genuine answer. A stale
//! `max_freq` can only keep the window at its current size, never make it grow wrongly. The final
//! window length is therefore the answer.
//!
//! **Complexity:** `O(n)` time, `O(1)` space (26 counters).
//!
//! **Rust concepts:**
//! - `usize::from(b - b'A')` maps `'A'..='Z'` to `0..26`; the subtraction cannot underflow because
//!   the input is guaranteed uppercase.
//! - `let count = &mut counts[i]; *count += 1;` updates a counter through a mutable reference.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        // @solution-start
        let bytes = s.as_bytes();
        let k = k as usize;
        let mut counts = [0usize; 26];
        let (mut left, mut max_freq) = (0, 0);
        for (right, &b) in bytes.iter().enumerate() {
            let count = &mut counts[usize::from(b - b'A')];
            *count += 1;
            max_freq = max_freq.max(*count);
            if right + 1 - left - max_freq > k {
                // Not fixable: slide the window instead of growing it.
                counts[usize::from(bytes[left] - b'A')] -= 1;
                left += 1;
            }
        }
        (bytes.len() - left) as i32 // the final window length
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, k: i32) -> i32 {
        Solution::character_replacement(s.to_string(), k)
    }

    #[test]
    fn example_replace_two() {
        assert_eq!(run("ABAB", 2), 4);
    }

    #[test]
    fn example_replace_one() {
        assert_eq!(run("AABABBA", 1), 4);
    }

    #[test]
    fn single_letter() {
        assert_eq!(run("A", 0), 1);
    }

    #[test]
    fn no_replacements() {
        assert_eq!(run("ABCDE", 0), 1);
        assert_eq!(run("ABBBCC", 0), 3);
    }

    #[test]
    fn enough_replacements_for_everything() {
        assert_eq!(run("ABCDE", 5), 5);
    }

    #[test]
    fn best_window_in_the_middle() {
        // "BBABB" (replace the 'A') is the best block.
        assert_eq!(run("ACBBABBCA", 1), 5);
    }

    #[test]
    fn long_alternating_input() {
        let s = "AB".repeat(50_000);
        assert_eq!(run(&s, 0), 1);
        assert_eq!(run(&s, 50_000), 100_000);
    }
}
