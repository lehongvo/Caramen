//! # Problem 0162: Initials
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split_whitespace`, `chars().next()`, `char::to_uppercase`, building a `String`
//! - **Source:** Original · [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! Implement `initials(full_name: &str) -> String`, which returns the initials of a name: for every
//! word (a run of non-whitespace characters), take its **first character**, convert it to upper case
//! and write it followed by a period `'.'`. The initials are written next to each other, with no
//! spaces. A name without words gives an empty string.
//!
//! Punctuation inside a word belongs to the word (`"jean-luc"` is one word with initial `J`).
//! Upper-casing follows Unicode rules (`'ö'` becomes `'Ö'`).
//!
//! ## Examples
//!
//! ```text
//! Input:  full_name = "ada lovelace"
//! Output: "A.L."
//!
//! Input:  full_name = "  Grace   brewster  Hopper "
//! Output: "G.B.H."
//!
//! Input:  full_name = "jean-luc picard"
//! Output: "J.P."
//!
//! Input:  full_name = "ölaf über"
//! Output: "Ö.Ü."
//! ```
//!
//! ## Constraints
//!
//! - `0 <= full_name.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. `split_whitespace()` takes care of extra spaces at the ends and between the words.
//! 2. Each word is non-empty, so `word.chars().next()` is always `Some`.
//! 3. `c.to_uppercase()` is an iterator; `String::extend` appends all its characters.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for every word, decode only its first character, append its upper-case form and a
//! period. The rest of each word is never looked at.
//!
//! **Complexity:** `O(n)` time for the scan, `O(w)` space for `w` words.
//!
//! **Rust concepts:**
//! - `chars().next()` reads one character, correctly decoded even when it is several bytes long;
//!   `word.as_bytes()[0]` would only be right for ASCII.
//! - `if let Some(c) = ...` handles the `Option` without `unwrap`.
//! - `char::to_uppercase` may produce several characters (`'ß'` gives `"SS"`), hence `extend`.
// @solution-end

pub fn initials(full_name: &str) -> String {
    // @solution-start
    let mut out = String::new();
    for word in full_name.split_whitespace() {
        if let Some(first) = word.chars().next() {
            out.extend(first.to_uppercase());
            out.push('.');
        }
    }
    out
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_words() {
        assert_eq!(initials("ada lovelace"), "A.L.");
    }

    #[test]
    fn example_extra_spaces() {
        assert_eq!(initials("  Grace   brewster  Hopper "), "G.B.H.");
    }

    #[test]
    fn example_hyphenated_word() {
        assert_eq!(initials("jean-luc picard"), "J.P.");
    }

    #[test]
    fn example_non_ascii_initials() {
        assert_eq!(initials("ölaf über"), "Ö.Ü.");
    }

    #[test]
    fn empty_and_single_word() {
        assert_eq!(initials(""), "");
        assert_eq!(initials("   "), "");
        assert_eq!(initials("plato"), "P.");
    }

    #[test]
    fn many_words() {
        let name = "x ".repeat(5_000);
        assert_eq!(initials(&name), "X.".repeat(5_000));
    }
}
