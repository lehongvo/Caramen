//! # Problem 0398: Reorganize String
//!
//! - **Difficulty:** Medium
//! - **Topics:** max-heap of `(count, byte)`, holding the previous character back with `Option::take`, impossible-case detection
//! - **Source:** [LeetCode 767. Reorganize String](https://leetcode.com/problems/reorganize-string/)
//!
//! ## Problem
//!
//! Rearrange the characters of `s` so that no two **adjacent** characters are equal. Return any
//! such rearrangement, or `""` if none exists.
//!
//! Implement `Solution::reorganize_string(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "aab"
//! Output: "aba"
//!
//! Input:  s = "aaab"
//! Output: ""        (three 'a's need at least two separators)
//!
//! Input:  s = "vvvlo"
//! Output: "vlvov"   ("vovlv" is also accepted)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 500`
//! - `s` contains lowercase English letters.
//!
//! ## Hints
//!
//! 1. Greedy: always place the character with the most remaining copies, unless it was placed just
//!    before.
//! 2. Pop the top of a max-heap of `(count, character)`, append it, and keep it **out** of the heap
//!    for one round; put the previously held-back character back in.
//! 3. If the heap runs empty while a character is still held back, the task is impossible.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each step appends the most frequent character that differs from the last one. The
//! character placed in the previous step is held in `prev` (outside the heap), so it cannot be
//! picked twice in a row; after placing a new character, `prev` goes back into the heap and the new
//! one (if copies remain) becomes `prev`. If the loop ends before every character is placed, the
//! leftover in `prev` proves that no arrangement exists (it happens exactly when one character
//! occurs more than `(len + 1) / 2` times).
//!
//! **Complexity:** `O(n log 26)` time, `O(n)` space for the output.
//!
//! **Rust concepts:**
//! - `prev.take()` moves the value out of the `Option` and leaves `None` behind, perfect for
//!   "give it back to the heap".
//! - Working on bytes (`u8`) is fine for ASCII input; `String::from_utf8(out)` converts back, and
//!   `unwrap_or_default()` never triggers because the bytes come from a valid `String`.
//! - The tests check any valid answer with a small checker (same letters, no equal neighbours).
// @solution-end

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        // @solution-start
        let mut counts = [0usize; 26];
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        let mut heap: BinaryHeap<(usize, u8)> = (0..26u8)
            .filter(|&i| counts[i as usize] > 0)
            .map(|i| (counts[i as usize], b'a' + i))
            .collect();
        let mut out: Vec<u8> = Vec::with_capacity(s.len());
        let mut prev: Option<(usize, u8)> = None;
        while let Some((count, b)) = heap.pop() {
            out.push(b);
            if let Some(held) = prev.take() {
                heap.push(held);
            }
            if count > 1 {
                prev = Some((count - 1, b));
            }
        }
        if out.len() == s.len() {
            String::from_utf8(out).unwrap_or_default()
        } else {
            String::new()
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(s: &str, result: &str) -> bool {
        let mut a: Vec<u8> = s.bytes().collect();
        let mut b: Vec<u8> = result.bytes().collect();
        a.sort_unstable();
        b.sort_unstable();
        a == b && result.as_bytes().windows(2).all(|w| w[0] != w[1])
    }

    fn check_possible(s: &str) {
        let result = Solution::reorganize_string(s.to_string());
        assert!(is_valid(s, &result), "invalid answer {result:?} for {s:?}");
    }

    #[test]
    fn example_one() {
        check_possible("aab");
    }

    #[test]
    fn example_two_impossible() {
        assert_eq!(Solution::reorganize_string("aaab".to_string()), "");
    }

    #[test]
    fn example_three() {
        assert!(!is_valid("vvvlo", "vvlov"));
        check_possible("vvvlo");
    }

    #[test]
    fn single_and_borderline() {
        check_possible("z");
        check_possible("aabbcc");
        // 3 'a's out of 5: exactly (5 + 1) / 2, still possible ("ababa").
        check_possible("aaabb");
        assert_eq!(Solution::reorganize_string("aa".to_string()), "");
    }

    #[test]
    fn long_strings() {
        // 250 'a's and 250 other letters: exactly at the limit (500 + 1) / 2 = 250.
        let possible = format!("{}{}", "a".repeat(250), "bcdefghij".repeat(28));
        check_possible(&possible[..500]);
        let impossible = format!("{}{}", "q".repeat(251), "r".repeat(249));
        assert_eq!(Solution::reorganize_string(impossible), "");
    }
}
