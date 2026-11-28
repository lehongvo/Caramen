// ==================================================================
// PRACTICE 0121: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0121_reverse_string.md
// Run the tests:          cargo test practice::p0121
// ==================================================================

//! # Problem 0121: Reverse String
//!
//! - **Difficulty:** Easy
//! - **Topics:** in-place modification through `&mut Vec<char>`, two indices, `slice::swap`
//! - **Source:** [LeetCode 344. Reverse String](https://leetcode.com/problems/reverse-string/)
//!
//! ## Problem
//!
//! A string is given as a vector of characters `s`. Reverse the order of the characters **in place**:
//! the function receives a mutable reference to the caller's vector, changes it, and returns
//! nothing. Use only `O(1)` extra memory (do not build a second vector).
//!
//! Implement `Solution::reverse_string(s: &mut Vec<char>)`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = ['h', 'e', 'l', 'l', 'o']
//! Output: s = ['o', 'l', 'l', 'e', 'h']
//!
//! Input:  s = ['H', 'a', 'n', 'n', 'a', 'h']
//! Output: s = ['h', 'a', 'n', 'n', 'a', 'H']
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 10^5`
//! - Every `s[i]` is a printable ASCII character.
//!
//! ## Hints
//!
//! 1. Put one index at the start and one at the end, swap the two characters, then move both
//!    indices towards the middle.
//! 2. `s.swap(i, j)` exchanges two elements; stop when the indices meet.

pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<char>`; `&mut [char]` would be the idiomatic choice.
    #[allow(clippy::ptr_arg)]
    pub fn reverse_string(s: &mut Vec<char>) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reversed(text: &str) -> String {
        let mut chars: Vec<char> = text.chars().collect();
        Solution::reverse_string(&mut chars);
        chars.into_iter().collect()
    }

    #[test]
    fn example_hello() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn example_hannah() {
        assert_eq!(reversed("Hannah"), "hannaH");
    }

    #[test]
    fn single_and_two_characters() {
        assert_eq!(reversed("a"), "a");
        assert_eq!(reversed("ab"), "ba");
    }

    #[test]
    fn odd_length_keeps_the_middle() {
        assert_eq!(reversed("abcde"), "edcba");
    }

    #[test]
    fn large_input() {
        let text: String = (0..100_000)
            .map(|i| (b'a' + (i % 26) as u8) as char)
            .collect();
        let expected: String = text.chars().rev().collect();
        assert_eq!(reversed(&text), expected);
    }
}
