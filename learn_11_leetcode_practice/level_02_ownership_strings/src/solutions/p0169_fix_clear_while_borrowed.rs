//! # Problem 0169: Fix: Clearing a String While Borrowed
//!
//! - **Difficulty:** Easy
//! - **Topics:** fix the borrow checker, error E0502, a slice keeps its `String` borrowed, `to_string` before `clear`
//! - **Source:** Original · [E0502](https://doc.rust-lang.org/error_codes/E0502.html) · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! An input box keeps the user's text in a `String`. When the user presses Enter, the program takes
//! the first word as a command and clears the box. This attempt does not compile:
//!
//! ```text
//! pub fn take_first_word(text: &mut String) -> String {
//!     let word = text.split_whitespace().next().unwrap_or("");
//!     text.clear();
//!     word.to_string()
//! }
//!
//! error[E0502]: cannot borrow `*text` as mutable because it is also borrowed as immutable
//!   |     let word = text.split_whitespace().next().unwrap_or("");
//!   |                ---- immutable borrow occurs here
//!   |     text.clear();
//!   |     ^^^^^^^^^^^^ mutable borrow occurs here
//!   |     word.to_string()
//!   |     ---- immutable borrow later used here
//! ```
//!
//! `word` is a slice **into** `text`. Clearing `text` while `word` is still needed would leave `word`
//! pointing at removed data, which is exactly the bug the borrow checker prevents (The Rust Book
//! shows the same situation with its `first_word` example).
//!
//! Implement the corrected `take_first_word(text: &mut String) -> String`: it returns the first
//! whitespace-separated word of `text` as an owned `String` (or `""` if there is none) and leaves
//! `text` empty in every case.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "open file.txt now"
//! Output: "open", text = ""
//!
//! Input:  text = "   quit"
//! Output: "quit", text = ""
//!
//! Input:  text = "   "
//! Output: "",     text = ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. Make the owned copy of the word **before** clearing: once `word.to_string()` has run, the slice
//!    is no longer used and the borrow ends.
//! 2. Order matters more than syntax here: read, copy, then mutate.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** turn the borrowed first word into an owned `String` first. From then on nothing borrows
//! `text`, so `clear()` is allowed, and the owned word is returned.
//!
//! **Complexity:** `O(n)` time to find the word; `O(w)` space for the returned word.
//!
//! **Rust concepts:**
//! - A `&str` obtained from a `String` keeps the `String` borrowed for as long as the slice is used.
//! - Non-lexical lifetimes: a borrow ends at its **last use**, so reordering the statements is
//!   enough to satisfy the borrow checker.
//! - `clear()` keeps the allocated capacity, so the input box can be reused without reallocating.
// @solution-end

pub fn take_first_word(text: &mut String) -> String {
    // @solution-start
    let word = text.split_whitespace().next().unwrap_or("").to_string();
    // `word` is owned now, so `text` is no longer borrowed.
    text.clear();
    word
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_command_with_arguments() {
        let mut text = String::from("open file.txt now");
        assert_eq!(take_first_word(&mut text), "open");
        assert_eq!(text, "");
    }

    #[test]
    fn example_leading_spaces() {
        let mut text = String::from("   quit");
        assert_eq!(take_first_word(&mut text), "quit");
        assert!(text.is_empty());
    }

    #[test]
    fn example_only_spaces() {
        let mut text = String::from("   ");
        assert_eq!(take_first_word(&mut text), "");
        assert!(text.is_empty());
    }

    #[test]
    fn capacity_is_kept_for_reuse() {
        let mut text = String::with_capacity(64);
        text.push_str("save now");
        assert_eq!(take_first_word(&mut text), "save");
        assert!(text.capacity() >= 64);
        text.push_str("next");
        assert_eq!(take_first_word(&mut text), "next");
    }

    #[test]
    fn unicode_word() {
        let mut text = String::from("\tüber alles");
        assert_eq!(take_first_word(&mut text), "über");
    }

    #[test]
    fn large_input() {
        let mut text = format!("{} tail", "w".repeat(99_000));
        assert_eq!(take_first_word(&mut text).len(), 99_000);
        assert!(text.is_empty());
    }
}
