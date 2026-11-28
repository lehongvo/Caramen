//! # Problem 0167: Longest Word
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split` with a closure pattern, `char::is_alphanumeric`, returning the best `&str`
//! - **Source:** Original · [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split) · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! Implement `longest_word(text: &str) -> &str`, which returns the longest word of `text`.
//!
//! - A *word* is a maximal run of alphanumeric characters (`char::is_alphanumeric`, so letters and
//!   digits of any script). Everything else (spaces, punctuation, symbols) separates words.
//! - Length is measured in **characters** (`chars().count()`), not bytes.
//! - If several words have the maximum length, return the **first** one.
//! - If `text` has no word, return `""`.
//!
//! The result must be a slice of `text`.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "Hello, wonderful world!"
//! Output: "wonderful"
//!
//! Input:  text = "I was 1000% sure"
//! Output: "1000"          ("1000" and "sure" both have 4 characters; the first wins)
//!
//! Input:  text = "日本語 is fun"
//! Output: "日本語"          (3 characters, although 9 bytes; "fun" also has 3 and comes later)
//!
//! Input:  text = "?!"
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `text.split(|c: char| !c.is_alphanumeric())` splits at every non-alphanumeric character; it
//!    yields empty pieces between consecutive separators, so filter them out.
//! 2. Keep the best word as a `&str` and its character count; replace it only when a word is
//!    strictly longer.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** split the text into candidate words with a closure pattern, skip empty pieces, and keep
//! the first word with the highest character count. All candidates are slices of `text`, so the
//! answer is too.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - String patterns can be a `char`, a `&str`, a slice of `char`s or a closure `FnMut(char) -> bool`.
//! - Comparing `chars().count()` instead of `len()` makes `"日本語"` (9 bytes) as long as `"fun"`.
//! - `best` starts as `""`, a `&'static str`, which coerces to the shorter lifetime of `text`.
// @solution-end

pub fn longest_word(text: &str) -> &str {
    // @solution-start
    let mut best = "";
    let mut best_len = 0;
    for word in text.split(|c: char| !c.is_alphanumeric()) {
        let len = word.chars().count();
        if len > best_len {
            best = word;
            best_len = len;
        }
    }
    best
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_punctuation() {
        assert_eq!(longest_word("Hello, wonderful world!"), "wonderful");
    }

    #[test]
    fn example_tie_with_digits() {
        assert_eq!(longest_word("I was 1000% sure"), "1000");
    }

    #[test]
    fn example_counts_characters_not_bytes() {
        assert_eq!(longest_word("日本語 is fun"), "日本語");
        assert_eq!(longest_word("fun is 日本語"), "fun");
    }

    #[test]
    fn example_no_words() {
        assert_eq!(longest_word("?!"), "");
        assert_eq!(longest_word(""), "");
    }

    #[test]
    fn apostrophes_and_hyphens_split_words() {
        assert_eq!(longest_word("don't over-think"), "think");
    }

    #[test]
    fn result_borrows_from_the_text() {
        let text = String::from("a bbb cc");
        let word = longest_word(&text);
        assert_eq!(word, "bbb");
        assert_eq!(word.as_ptr(), text[2..].as_ptr());
    }

    #[test]
    fn large_input() {
        let text = format!("{} {} {}", "ab ".repeat(20_000), "z".repeat(50), "y".repeat(50));
        assert_eq!(longest_word(&text), "z".repeat(50));
    }
}
