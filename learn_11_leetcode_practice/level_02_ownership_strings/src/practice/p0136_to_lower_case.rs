// ==================================================================
// PRACTICE 0136: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0136_to_lower_case.md
// Run the tests:          cargo test practice::p0136
// ==================================================================

//! # Problem 0136: To Lower Case
//!
//! - **Difficulty:** Easy
//! - **Topics:** case conversion, `make_ascii_lowercase` in place, owned `String` parameter
//! - **Source:** [LeetCode 709. To Lower Case](https://leetcode.com/problems/to-lower-case/)
//!
//! ## Problem
//!
//! Implement `Solution::to_lower_case(s: String) -> String`, which returns `s` with every uppercase
//! ASCII letter (`'A'` to `'Z'`) replaced by the matching lowercase letter. Every other character
//! stays the same.
//!
//! The function receives the `String` by value. Because you own it, you can convert it **in place**
//! and return it, without allocating a new string.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "Hello"
//! Output: "hello"
//!
//! Input:  s = "here"
//! Output: "here"
//!
//! Input:  s = "LOVELY 42!"
//! Output: "lovely 42!"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= s.len() <= 100`
//! - `s` consists of printable ASCII characters.
//!
//! ## Hints
//!
//! 1. `String` (through `str`) has `make_ascii_lowercase(&mut self)`, which changes the bytes in place.
//! 2. To do it by hand: for an uppercase byte `b`, the lowercase letter is `b + 32` (`b'a' - b'A'`).

pub struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_capitalized() {
        assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello");
    }

    #[test]
    fn example_already_lower() {
        assert_eq!(Solution::to_lower_case("here".to_string()), "here");
    }

    #[test]
    fn example_with_digits_and_symbols() {
        assert_eq!(
            Solution::to_lower_case("LOVELY 42!".to_string()),
            "lovely 42!"
        );
    }

    #[test]
    fn boundary_characters() {
        // '@' is just before 'A' and '[' just after 'Z'.
        assert_eq!(Solution::to_lower_case("@AZ[".to_string()), "@az[");
    }

    #[test]
    fn longest_input() {
        let s = "AbC".repeat(33);
        assert_eq!(Solution::to_lower_case(s), "abc".repeat(33));
    }
}
