//! # Problem 0185: Make The String Great
//!
//! - **Difficulty:** Easy
//! - **Topics:** `String` as a stack, `chars().next_back()`, match guards, `eq_ignore_ascii_case`
//! - **Source:** [LeetCode 1544. Make The String Great](https://leetcode.com/problems/make-the-string-great/)
//!
//! ## Problem
//!
//! A string is *good* if it has no two adjacent characters that are the same letter in different
//! cases (such as `"aA"` or `"Bb"`). Repeatedly remove such a pair until the string is good, and
//! return the result (it may be empty). The final string does not depend on the order of removals.
//!
//! Implement `Solution::make_good(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "leEeetcode"
//! Output: "leetcode"       ("eE" is removed)
//!
//! Input:  s = "abBAcC"
//! Output: ""               ("bB", then "aA", then "cC")
//!
//! Input:  s = "s"
//! Output: "s"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 100`
//! - `s` contains only lowercase and uppercase English letters.
//!
//! ## Hints
//!
//! 1. Same stack idea as in "Remove All Adjacent Duplicates In String": compare each character with
//!    the last character kept so far.
//! 2. A pair cancels when the two characters are different but equal ignoring case:
//!    `last != c && last.eq_ignore_ascii_case(&c)`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build the answer as a stack. Peek at its last character with `chars().next_back()`; if
//! it forms a bad pair with the incoming character, pop it, otherwise push the incoming character.
//!
//! **Complexity:** `O(n)` time and space.
//!
//! **Rust concepts:**
//! - `chars().next_back()` reads the last `char` in `O(1)` (UTF-8 can be decoded backwards); for
//!   this ASCII input `as_bytes().last()` would also work.
//! - A match guard (`Some(last) if ...`) combines destructuring the `Option` with the condition.
//! - `char::eq_ignore_ascii_case` takes `&char`, hence `&c`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        // @solution-start
        let mut stack = String::with_capacity(s.len());
        for c in s.chars() {
            match stack.chars().next_back() {
                Some(last) if last != c && last.eq_ignore_ascii_case(&c) => {
                    stack.pop();
                }
                _ => stack.push(c),
            }
        }
        stack
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::make_good(s.to_string())
    }

    #[test]
    fn example_leetcode() {
        assert_eq!(run("leEeetcode"), "leetcode");
    }

    #[test]
    fn example_everything_removed() {
        assert_eq!(run("abBAcC"), "");
    }

    #[test]
    fn example_single() {
        assert_eq!(run("s"), "s");
    }

    #[test]
    fn same_case_pairs_stay() {
        assert_eq!(run("aa"), "aa");
        assert_eq!(run("AA"), "AA");
    }

    #[test]
    fn different_letters_stay() {
        assert_eq!(run("aB"), "aB");
    }

    #[test]
    fn longest_input() {
        // "eE" meets first; each removal exposes the next pair ("dD", "cC", ...), so all cancel.
        let s = format!("{}{}", "abcde".repeat(10), "EDCBA".repeat(10));
        assert_eq!(run(&s), "");
        // Here no character ever sits next to its other-case twin.
        let s = format!("{}{}", "abcde".repeat(10), "ABCDE".repeat(10));
        assert_eq!(run(&s).len(), 100);
    }
}
