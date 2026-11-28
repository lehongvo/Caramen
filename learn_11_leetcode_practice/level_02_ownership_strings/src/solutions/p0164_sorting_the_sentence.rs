//! # Problem 0164: Sorting the Sentence
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split_at` to separate a suffix, `Vec<&str>` slots borrowing from the input, `join`
//! - **Source:** [LeetCode 1859. Sorting the Sentence](https://leetcode.com/problems/sorting-the-sentence/)
//!
//! ## Problem
//!
//! A sentence of at most 9 words was *shuffled*: each word got its 1-based position appended as a
//! single digit, and then the words were put in an arbitrary order. For example, `"This is a
//! sentence"` can become `"sentence4 a3 is2 This1"`.
//!
//! Given the shuffled sentence `s` (words separated by single spaces), return the original sentence
//! without the digits.
//!
//! Implement `Solution::sort_sentence(s: String) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  s = "is2 sentence4 This1 a3"
//! Output: "This is a sentence"
//!
//! Input:  s = "Myself2 Me1 I4 and3"
//! Output: "Me Myself and I"
//! ```
//!
//! ## Constraints
//!
//! - `2 <= s.len() <= 200`
//! - `s` contains English letters, digits `1`-`9` and single spaces; 1 to 9 words.
//! - Every word ends with its position digit, and each position appears exactly once.
//!
//! ## Hints
//!
//! 1. `word.split_at(word.len() - 1)` separates the letters from the final digit (both are ASCII).
//! 2. Prepare `vec![""; number_of_words]` and store each word's letters at index `digit - 1`; the
//!    slots are `&str` slices of `s`, so nothing is copied yet.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every word tells us where it belongs. Put the letter part of each word into its slot of
//! a `Vec<&str>`, then join the slots with spaces. Only the final `join` allocates.
//!
//! **Complexity:** `O(n)` time and space.
//!
//! **Rust concepts:**
//! - A `Vec<&str>` of slices into `s` is a cheap, temporary view; it borrows `s`, which therefore
//!   stays alive until the `join` is done.
//! - `split_at(i)` returns two `&str`s; `i` must be a character boundary, which it is for ASCII.
//! - `(digit - b'1') as usize` turns the ASCII digit byte into a 0-based index.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        // @solution-start
        let count = s.split(' ').count();
        let mut slots: Vec<&str> = vec![""; count];
        for word in s.split(' ') {
            let (letters, digit) = word.split_at(word.len() - 1);
            let index = (digit.as_bytes()[0] - b'1') as usize;
            slots[index] = letters;
        }
        slots.join(" ")
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &str) -> String {
        Solution::sort_sentence(s.to_string())
    }

    #[test]
    fn example_four_words() {
        assert_eq!(run("is2 sentence4 This1 a3"), "This is a sentence");
    }

    #[test]
    fn example_myself() {
        assert_eq!(run("Myself2 Me1 I4 and3"), "Me Myself and I");
    }

    #[test]
    fn single_word() {
        assert_eq!(run("Hello1"), "Hello");
    }

    #[test]
    fn nine_words_reversed() {
        let words = ["a", "bb", "ccc", "Dd", "e", "ff", "GGG", "hh", "i"];
        let shuffled: Vec<String> = (0..9)
            .rev()
            .map(|i| format!("{}{}", words[i], i + 1))
            .collect();
        assert_eq!(run(&shuffled.join(" ")), words.join(" "));
    }
}
