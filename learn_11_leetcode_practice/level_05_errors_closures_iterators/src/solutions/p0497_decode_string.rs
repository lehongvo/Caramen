//! # Problem 0497: Decode String
//!
//! - **Difficulty:** Medium
//! - **Topics:** recursive descent over `Peekable<Chars>`, `peek` / `next_if` / `next`, `str::repeat`
//! - **Source:** [LeetCode 394. Decode String](https://leetcode.com/problems/decode-string/)
//!
//! ## Problem
//!
//! An encoded string uses the rule `k[text]`, meaning "`text` repeated `k` times". Rules can be
//! nested, and plain letters can appear anywhere. Return the decoded string.
//!
//! The input is always valid: brackets are balanced, every `[` is preceded by a positive number,
//! digits appear only in those numbers, and there are no spaces.
//!
//! Implement `Solution::decode_string(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "3[a]2[bc]"
//! Output: "aaabcbc"
//!
//! Input:  s = "3[a2[c]]"
//! Output: "accaccacc"
//!
//! Input:  s = "2[abc]3[cd]ef"
//! Output: "abcabccdcdcdef"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 30`
//! - `s` contains lowercase English letters, digits and `'['`, `']'`.
//! - All repeat counts are in `1..=300`; the output has at most `10^5` characters.
//!
//! ## Hints
//!
//! 1. Write `fn decode(chars: &mut Peekable<Chars>) -> String` that decodes until it sees a `']'`
//!    (without consuming it) or the end of the input.
//! 2. On a digit: read the whole number with `next_if`, skip the `'['`, call `decode` recursively for
//!    the inside, skip the `']'`, and append the inside repeated `k` times.
//! 3. `peek()` lets you look at the next character before deciding what to do with it.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the grammar is recursive (`k[...]` contains a whole encoded string), so the parser is
//! too. One shared `Peekable` iterator walks the input; each recursive call decodes one bracketed
//! level and returns at the matching `']'`.
//!
//! **Complexity:** `O(output length)` time and space.
//!
//! **Rust concepts:**
//! - Passing `&mut Peekable<Chars>` lets nested calls consume the same input in order.
//! - `peek()` returns `Option<&char>`; matching on `Some(&c)` copies the character out.
//! - `next_if(|c| c.is_ascii_digit())` reads a multi-digit number one digit at a time.
//! - `str::repeat(k)` builds the repeated text in one allocation.
// @solution-end

use std::iter::Peekable;
use std::str::Chars;

pub struct Solution;

// @solution-start remove
fn decode(chars: &mut Peekable<Chars<'_>>) -> String {
    let mut out = String::new();
    while let Some(&c) = chars.peek() {
        match c {
            ']' => break,
            '0'..='9' => {
                let mut k = 0usize;
                while let Some(d) = chars.next_if(|c| c.is_ascii_digit()) {
                    // `d` is an ASCII digit, so this subtraction gives its value.
                    k = k * 10 + usize::from(d as u8 - b'0');
                }
                chars.next(); // the '['
                let inner = decode(chars);
                chars.next(); // the ']'
                out.push_str(&inner.repeat(k));
            }
            letter => {
                out.push(letter);
                chars.next();
            }
        }
    }
    out
}
// @solution-end

impl Solution {
    pub fn decode_string(s: String) -> String {
        // @solution-start
        decode(&mut s.chars().peekable())
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::decode_string(s.to_string())
    }

    #[test]
    fn example_two_groups() {
        assert_eq!(run("3[a]2[bc]"), "aaabcbc");
    }

    #[test]
    fn example_nested() {
        assert_eq!(run("3[a2[c]]"), "accaccacc");
    }

    #[test]
    fn example_trailing_letters() {
        assert_eq!(run("2[abc]3[cd]ef"), "abcabccdcdcdef");
    }

    #[test]
    fn plain_text_and_multi_digit_counts() {
        assert_eq!(run("abc"), "abc");
        assert_eq!(run("10[a]"), "a".repeat(10));
        assert_eq!(run("a2[b3[c]]d"), "abcccbcccd");
        assert_eq!(run("300[a]").len(), 300);
    }
}
