//! # Problem 0147: Caesar Cipher
//!
//! - **Difficulty:** Easy
//! - **Topics:** `chars().map(..).collect()`, `is_ascii_lowercase`, `u8` arithmetic, `as char`
//! - **Source:** Original · [`char::is_ascii_lowercase`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_lowercase) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! The Caesar cipher replaces every letter by the letter `shift` positions later in the alphabet,
//! wrapping around from `z` back to `a`.
//!
//! Implement `caesar(text: &str, shift: u8) -> String`:
//!
//! - ASCII lowercase letters `a`-`z` are shifted and stay lowercase;
//! - ASCII uppercase letters `A`-`Z` are shifted and stay uppercase;
//! - every other character (digits, spaces, punctuation, non-ASCII letters such as `'ü'`) is copied
//!   unchanged.
//!
//! `shift` can be any `u8`; shifting by 26 is the same as not shifting at all.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "abc xyz", shift = 3
//! Output: "def abc"
//!
//! Input:  text = "Hello, World!", shift = 13
//! Output: "Uryyb, Jbeyq!"
//!
//! Input:  text = "Grüße 2024", shift = 1
//! Output: "Hsüßf 2024"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//! - `0 <= shift <= 255`
//!
//! ## Hints
//!
//! 1. Reduce the shift first: `shift % 26`.
//! 2. For an ASCII letter `c`, its offset in the alphabet is `c as u8 - b'a'` (or `- b'A'`); add the
//!    shift, take `% 26`, add the base back and convert with `as char`.
//! 3. Work with `chars()`, not `bytes()`, so non-ASCII characters are copied as whole characters.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** map every character independently. For a letter, compute its alphabet offset, rotate
//! it by `shift % 26` and rebuild the letter from the right base (`b'a'` or `b'A'`); any other
//! character is returned as is. Collect the mapped characters into a new `String`.
//!
//! **Complexity:** `O(n)` time and space.
//!
//! **Rust concepts:**
//! - `c as u8` is only meaningful for ASCII characters; the `is_ascii_lowercase` /
//!   `is_ascii_uppercase` checks guarantee that.
//! - The arithmetic stays below 256: offset `< 26` plus shift `< 26` is `< 52`, so `u8` cannot
//!   overflow.
//! - `u8 as char` converts a byte in `0..=127` to the same ASCII character.
// @solution-end

// @solution-start remove
/// Rotates an ASCII letter `c` whose alphabet starts at `base` (`b'a'` or `b'A'`).
fn rotate(c: char, base: u8, shift: u8) -> char {
    let offset = c as u8 - base;
    (base + (offset + shift) % 26) as char
}
// @solution-end

pub fn caesar(text: &str, shift: u8) -> String {
    // @solution-start
    let shift = shift % 26;
    text.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                rotate(c, b'a', shift)
            } else if c.is_ascii_uppercase() {
                rotate(c, b'A', shift)
            } else {
                c
            }
        })
        .collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_wraps_around() {
        assert_eq!(caesar("abc xyz", 3), "def abc");
    }

    #[test]
    fn example_rot13_keeps_case_and_punctuation() {
        assert_eq!(caesar("Hello, World!", 13), "Uryyb, Jbeyq!");
    }

    #[test]
    fn example_non_ascii_unchanged() {
        assert_eq!(caesar("Grüße 2024", 1), "Hsüßf 2024");
    }

    #[test]
    fn shift_of_26_and_255() {
        assert_eq!(caesar("Rust", 26), "Rust");
        // 255 % 26 = 21
        assert_eq!(caesar("az", 255), "vu");
    }

    #[test]
    fn rot13_twice_is_identity() {
        let text = "The Quick Brown Fox 🦀";
        assert_eq!(caesar(&caesar(text, 13), 13), text);
    }

    #[test]
    fn large_input() {
        let text = "abcdefghijklmnopqrstuvwxyz".repeat(4_000);
        let shifted = caesar(&text, 1);
        assert_eq!(shifted.len(), 104_000);
        assert!(shifted.starts_with("bcd"));
        assert!(shifted.ends_with("yza"));
    }
}
