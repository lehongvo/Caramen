// ==================================================================
// PRACTICE 0108: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0108_fix_use_after_consuming_loop.md
// Run the tests:          cargo test practice::p0108
// ==================================================================

//! # Problem 0108: Fix: Use After a Consuming Loop
//!
//! - **Difficulty:** Easy
//! - **Topics:** fix the borrow checker, error E0382, `for x in v` vs `for x in &v`
//! - **Source:** Original · [E0382](https://doc.rust-lang.org/error_codes/E0382.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
//!
//! ## Problem
//!
//! This function should return the total length of all words and the longest word, but it does not
//! compile:
//!
//! ```text
//! pub fn total_and_longest(words: Vec<String>) -> (usize, String) {
//!     let mut total = 0;
//!     for word in words {
//!         total += word.len();
//!     }
//!     let mut longest = String::new();
//!     for word in words {
//!         if word.len() > longest.len() {
//!             longest = word;
//!         }
//!     }
//!     (total, longest)
//! }
//!
//! error[E0382]: use of moved value: `words`
//!   |     for word in words {
//!   |                 ----- `words` moved due to this implicit call to `.into_iter()`
//!   ...
//!   |     for word in words {
//!   |                 ^^^^^ value used here after move
//! ```
//!
//! Implement the corrected `total_and_longest(words: Vec<String>) -> (usize, String)`:
//!
//! - the first value is the sum of `word.len()` over all words;
//! - the second value is the longest word (the **first** one if several have the maximum length),
//!   or an empty `String` if `words` is empty.
//!
//! Keep the signature, and do not clone any word: the longest word can be moved out of `words`.
//!
//! ## Examples
//!
//! ```text
//! Input:  words = ["red", "green", "blue"]
//! Output: (12, "green")
//!
//! Input:  words = ["ab", "cd"]
//! Output: (4, "ab")       (tie: the first longest word wins)
//!
//! Input:  words = []
//! Output: (0, "")
//! ```
//!
//! ## Constraints
//!
//! - `0 <= words.len() <= 10^4`
//! - Words are ASCII.
//!
//! ## Hints
//!
//! 1. `for word in words` calls `into_iter()` and consumes the vector. Which of the two loops really
//!    needs to own the words?
//! 2. `for word in &words` borrows the vector and yields `&String`, leaving `words` usable afterwards.

pub fn total_and_longest(words: Vec<String>) -> (usize, String) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| w.to_string()).collect()
    }

    #[test]
    fn example_colors() {
        assert_eq!(
            total_and_longest(owned(&["red", "green", "blue"])),
            (12, "green".to_string())
        );
    }

    #[test]
    fn example_tie_keeps_first() {
        assert_eq!(
            total_and_longest(owned(&["ab", "cd"])),
            (4, "ab".to_string())
        );
    }

    #[test]
    fn example_empty() {
        assert_eq!(total_and_longest(Vec::new()), (0, String::new()));
    }

    #[test]
    fn only_empty_words() {
        assert_eq!(total_and_longest(owned(&["", ""])), (0, String::new()));
    }

    #[test]
    fn longest_at_the_end() {
        assert_eq!(
            total_and_longest(owned(&["a", "bb", "ccc"])),
            (6, "ccc".to_string())
        );
    }

    #[test]
    fn large_input() {
        let mut words: Vec<String> = (0..10_000).map(|_| "abc".to_string()).collect();
        words[7_000] = "abcdef".to_string();
        assert_eq!(total_and_longest(words), (30_003, "abcdef".to_string()));
    }
}
