//! # Problem 0186: Backspace String Compare
//!
//! - **Difficulty:** Easy
//! - **Topics:** a helper taking `&str`, `String` as a stack, comparing owned results
//! - **Source:** [LeetCode 844. Backspace String Compare](https://leetcode.com/problems/backspace-string-compare/)
//!
//! ## Problem
//!
//! Two strings `s` and `t` are typed into empty text editors, one character at a time. The
//! character `'#'` means *backspace*: it deletes the previous character, if there is one (a backspace
//! on an empty editor does nothing). Return `true` if both editors end up with the same text.
//!
//! Implement `Solution::backspace_compare(s: String, t: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "ab#c", t = "ad#c"
//! Output: true             (both become "ac")
//!
//! Input:  s = "ab##", t = "c#d#"
//! Output: true             (both become "")
//!
//! Input:  s = "a#c", t = "b"
//! Output: false            ("c" vs "b")
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len(), t.len() <= 200`
//! - `s` and `t` contain only lowercase letters and `'#'`.
//!
//! ## Hints
//!
//! 1. Write a helper `fn typed(keys: &str) -> String` that simulates one editor: push letters, `pop()`
//!    on `'#'`.
//! 2. The helper only reads its input, so it should take `&str`; call it as `typed(&s)`.
//! 3. Follow-up: walking both strings from the end while counting pending backspaces needs only
//!    `O(1)` extra space.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** simulate both editors with a small helper and compare the two final texts. `String::pop`
//! on an empty string simply returns `None`, which matches "backspace on an empty editor".
//!
//! **Complexity:** `O(n + m)` time and space.
//!
//! **Rust concepts:**
//! - A helper that reads text takes `&str`; the caller passes `&s` (a `&String` that coerces to
//!   `&str`) and keeps ownership.
//! - The helper creates new text, so it returns an owned `String`.
//! - `String == String` compares contents.
// @solution-end

// @solution-start remove
/// Simulates an editor: letters are typed, `'#'` deletes the previous character.
fn typed(keys: &str) -> String {
    let mut text = String::with_capacity(keys.len());
    for c in keys.chars() {
        if c == '#' {
            text.pop();
        } else {
            text.push(c);
        }
    }
    text
}
// @solution-end

pub struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        // @solution-start
        typed(&s) == typed(&t)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(s: &str, t: &str) -> bool {
        Solution::backspace_compare(s.to_string(), t.to_string())
    }

    #[test]
    fn example_both_ac() {
        assert!(check("ab#c", "ad#c"));
    }

    #[test]
    fn example_both_empty() {
        assert!(check("ab##", "c#d#"));
    }

    #[test]
    fn example_different() {
        assert!(!check("a#c", "b"));
    }

    #[test]
    fn backspace_on_empty_editor() {
        assert!(check("###a", "a"));
        assert!(check("#", "##"));
    }

    #[test]
    fn same_letters_different_order() {
        assert!(!check("ab", "ba"));
    }

    #[test]
    fn longest_inputs() {
        let s = "ab#".repeat(66);
        let t = "a".repeat(66);
        assert!(check(&s, &t));
        assert!(!check(&s, &"a".repeat(65)));
    }
}
