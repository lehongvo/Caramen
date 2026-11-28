// ==================================================================
// PRACTICE 0174: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0174_reverse_vowels_of_a_string.md
// Run the tests:          cargo test practice::p0174
// ==================================================================

//! # Problem 0174: Reverse Vowels of a String
//!
//! - **Difficulty:** Easy
//! - **Topics:** `into_bytes`, two indices with `swap`, `matches!`, `String::from_utf8`
//! - **Source:** [LeetCode 345. Reverse Vowels of a String](https://leetcode.com/problems/reverse-vowels-of-a-string/)
//!
//! ## Problem
//!
//! Reverse only the vowels of a string: the first vowel trades places with the last vowel, the
//! second with the second to last, and so on. All other characters stay where they are. The vowels
//! are `a`, `e`, `i`, `o`, `u` in lowercase and uppercase.
//!
//! Implement `Solution::reverse_vowels(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "IceCreAm"
//! Output: "AceCreIm"        (the vowels I, e, e, A become A, e, e, I)
//!
//! Input:  s = "leetcode"
//! Output: "leotcede"
//!
//! Input:  s = "xyz"
//! Output: "xyz"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 3 * 10^5`
//! - `s` consists of printable ASCII characters.
//!
//! ## Hints
//!
//! 1. Turn the owned `String` into its bytes with `into_bytes()` so you can swap positions.
//! 2. Move `i` right until it is on a vowel and `j` left until it is on a vowel, swap, repeat.
//! 3. `matches!(b, b'a' | b'e' | ...)` is a compact vowel test.

pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::reverse_vowels(s.to_string())
    }

    #[test]
    fn example_ice_cream() {
        assert_eq!(run("IceCreAm"), "AceCreIm");
    }

    #[test]
    fn example_leetcode() {
        assert_eq!(run("leetcode"), "leotcede");
    }

    #[test]
    fn example_no_vowels() {
        assert_eq!(run("xyz"), "xyz");
    }

    #[test]
    fn single_vowel_and_punctuation() {
        assert_eq!(run("a"), "a");
        assert_eq!(run("a.b,O!"), "O.b,a!");
    }

    #[test]
    fn reversing_twice_restores() {
        let s = "Programming in Rust is fun";
        assert_eq!(run(&run(s)), s);
    }

    #[test]
    fn large_input() {
        let s = "aE".repeat(100_000);
        assert_eq!(run(&s), "Ea".repeat(100_000));
    }
}
