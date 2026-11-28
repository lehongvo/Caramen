// ==================================================================
// PRACTICE 0567: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0567_rearrange_words_in_a_sentence.md
// Run the tests:          cargo test practice::p0567
// ==================================================================

//! # Problem 0567: Rearrange Words in a Sentence
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Vec<&str>` borrowing from a local `String`, stable `sort_by_key`, capitalising the first character
//! - **Source:** [LeetCode 1451. Rearrange Words in a Sentence](https://leetcode.com/problems/rearrange-words-in-a-sentence/)
//!
//! ## Problem
//!
//! A sentence starts with an uppercase letter; everything else is lowercase words separated by single
//! spaces. Reorder the words by **length**, shortest first. Words of the same length keep their
//! original relative order. The result must again start with an uppercase letter, with every other
//! letter lowercase.
//!
//! Implement `Solution::arrange_words(text: String) -> String`.
//!
//! Lowercase the text once into a local `String`, then work with a `Vec<&str>` of slices of that string:
//! the words themselves are never copied until the final `join`.
//!
//! ## Examples
//!
//! ```text
//! Input:  "Leetcode is cool"          Output: "Is cool leetcode"
//! Input:  "Keep calm and code on"     Output: "On and keep calm code"
//! Input:  "To be or not to be"        Output: "To be or to be not"
//! ```
//!
//! ## Constraints
//!
//! - `1 <= text.len() <= 10^5`
//! - `text` starts with an uppercase English letter, followed by lowercase letters and single spaces.
//!
//! ## Hints
//!
//! 1. `slice::sort_by_key` is **stable**, which is exactly the tie rule.
//! 2. The `Vec<&str>` borrows from the lowercase `String`, so that `String` must stay alive while the
//!    vector is used; declare it first.
//! 3. To uppercase only the first character: take `chars()`, uppercase the first, then append the rest.

pub struct Solution;

impl Solution {
    pub fn arrange_words(text: String) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arrange(text: &str) -> String {
        Solution::arrange_words(text.to_string())
    }

    #[test]
    fn example_inputs() {
        assert_eq!(arrange("Leetcode is cool"), "Is cool leetcode");
        assert_eq!(arrange("Keep calm and code on"), "On and keep calm code");
        assert_eq!(arrange("To be or not to be"), "To be or to be not");
    }

    #[test]
    fn single_word_and_already_sorted() {
        assert_eq!(arrange("Hello"), "Hello");
        assert_eq!(arrange("A bb ccc"), "A bb ccc");
        assert_eq!(arrange("Ccc bb a"), "A bb ccc");
    }

    #[test]
    fn stability_on_equal_lengths() {
        assert_eq!(arrange("Dog cat ant bee"), "Dog cat ant bee");
        assert_eq!(arrange("Elephant dog cat"), "Dog cat elephant");
    }

    #[test]
    fn long_sentence() {
        let words: Vec<String> = (0..10_000).map(|i| "x".repeat(i % 5 + 1)).collect();
        let mut text = words.join(" ");
        text.replace_range(0..1, "X");
        let out = arrange(&text);
        assert!(out.starts_with("X x x"));
        assert!(out.ends_with("xxxxx xxxxx"));
        assert_eq!(out.len(), text.len());
    }
}
