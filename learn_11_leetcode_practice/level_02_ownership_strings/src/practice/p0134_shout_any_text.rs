// ==================================================================
// PRACTICE 0134: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0134_shout_any_text.md
// Run the tests:          cargo test practice::p0134
// ==================================================================

//! # Problem 0134: Shout Any Text
//!
//! - **Difficulty:** Easy
//! - **Topics:** `&str` parameters accept `&String` (deref coercion), `trim`, `to_uppercase`, `ends_with`
//! - **Source:** Original · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html) · [`str::to_uppercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase)
//!
//! ## Problem
//!
//! Implement `shout(text: &str) -> String`, which turns a message into a shout:
//!
//! 1. remove leading and trailing whitespace;
//! 2. convert every letter to upper case (full Unicode rules, so `"ß"` becomes `"SS"`);
//! 3. add one `'!'` at the end, unless the trimmed text already ends with `'!'`.
//!
//! If the text is empty after trimming, return an empty `String` (no `'!'`).
//!
//! The parameter type `&str` is the most flexible choice for read-only text: callers can pass a
//! string literal, a `&String` (it coerces to `&str` automatically) or a slice of a bigger string.
//! The result is a **new** `String`, because upper-casing changes the content.
//!
//! ## Examples
//!
//! ```text
//! Input:  text = "  hello there "
//! Output: "HELLO THERE!"
//!
//! Input:  text = "stop!"
//! Output: "STOP!"
//!
//! Input:  text = "straße"
//! Output: "STRASSE!"
//!
//! Input:  text = "   "
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. `trim()` returns a `&str` slice of the input, so trimming does not allocate.
//! 2. `to_uppercase()` returns a new `String`; then `push('!')` onto it.

pub fn shout(text: &str) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_trim_and_shout() {
        assert_eq!(shout("  hello there "), "HELLO THERE!");
    }

    #[test]
    fn example_already_exclaimed() {
        assert_eq!(shout("stop!"), "STOP!");
    }

    #[test]
    fn example_unicode_upper_case() {
        assert_eq!(shout("straße"), "STRASSE!");
    }

    #[test]
    fn example_only_whitespace() {
        assert_eq!(shout("   "), "");
        assert_eq!(shout(""), "");
    }

    #[test]
    fn accepts_string_references_and_slices() {
        let owned = String::from("rust is fun. really");
        assert_eq!(shout(&owned), "RUST IS FUN. REALLY!");
        assert_eq!(shout(&owned[..12]), "RUST IS FUN.!");
        // The caller still owns `owned`.
        assert_eq!(owned.len(), 19);
    }

    #[test]
    fn inner_whitespace_and_digits_are_kept() {
        assert_eq!(shout("\tgo  team 42\n"), "GO  TEAM 42!");
    }

    #[test]
    fn large_input() {
        let text = "ab ".repeat(3_000);
        let result = shout(&text);
        assert_eq!(result.len(), 8_999 + 1);
        assert!(result.starts_with("AB AB"));
        assert!(result.ends_with("AB!"));
    }
}
