// ==================================================================
// PRACTICE 0968: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0968_minimum_window_substring.md
// Run the tests:          cargo test practice::p0968
// ==================================================================

//! # Problem 0968: Minimum Window Substring
//!
//! - **Difficulty:** Hard
//! - **Topics:** variable-size sliding window, byte-indexed count arrays `[i32; 128]`, a "missing" counter, slicing the answer
//! - **Source:** [LeetCode 76. Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/)
//!
//! ## Problem
//!
//! Given strings `s` and `t`, return the **shortest substring** of `s` that contains every character
//! of `t` **with multiplicity** (if `t` has two `'a'`s, the window needs at least two). Return `""`
//! if there is no such substring. When several shortest windows exist, return the leftmost one (the
//! tests are built so that this choice is the one LeetCode expects).
//!
//! Implement `Solution::min_window(s: String, t: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "ADOBECODEBANC", t = "ABC"
//! Output: "BANC"
//!
//! Input:  s = "a", t = "aa"
//! Output: ""          (only one 'a' is available)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len(), t.len() <= 10^5`
//! - `s` and `t` consist of English letters (ASCII).
//!
//! ## Hints
//!
//! 1. `need[c]` = how many more copies of `c` the window still needs (can go negative for surplus);
//!    `missing` = the total number of required characters not yet covered.
//! 2. Extend the right end; when `missing == 0`, shrink from the left as long as the window stays
//!    valid, recording the best window.
//! 3. A character's arrival decreases `missing` only if `need[c]` was positive before.

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str, t: &str) -> String {
        Solution::min_window(s.to_string(), t.to_string())
    }

    #[test]
    fn examples() {
        assert_eq!(run("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(run("a", "a"), "a");
        assert_eq!(run("a", "aa"), "");
    }

    #[test]
    fn multiplicity_and_case() {
        assert_eq!(run("aaflslflsldkalskaaa", "aaa"), "aaa");
        assert_eq!(run("abcAbc", "Ab"), "Ab");
        assert_eq!(run("ab", "b"), "b");
        assert_eq!(run("bba", "ab"), "ba");
        assert_eq!(run("xyz", "q"), "");
    }

    #[test]
    fn window_is_the_whole_string() {
        assert_eq!(run("cabwefgewcwaefgcf", "cae"), "cwae");
        assert_eq!(run("abc", "cba"), "abc");
    }

    #[test]
    fn long_input() {
        let s = "x".repeat(50_000) + "abc" + &"y".repeat(50_000);
        assert_eq!(run(&s, "cba"), "abc");
        let t = "z".repeat(10);
        assert_eq!(run(&s, &t), "");
    }
}
