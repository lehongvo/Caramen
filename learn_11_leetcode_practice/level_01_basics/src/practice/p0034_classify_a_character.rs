// ==================================================================
// PRACTICE 0034: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0034_classify_a_character.md
// Run the tests:          cargo test practice::p0034
// ==================================================================

//! # Problem 0034: Classify a Character
//!
//! - **Difficulty:** Easy
//! - **Topics:** the `char` type, `match` on `char` ranges, `|` patterns, `&'static str`
//! - **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type)
//!
//! ## Problem
//!
//! A Rust `char` is a single Unicode scalar value: it can be `'a'`, `'7'`, `' '`, but also `'Ω'`
//! or `'中'`. Write `classify(c: char) -> &'static str` that puts a character into exactly one
//! category:
//!
//! | Characters                                   | Result           |
//! | -------------------------------------------- | ---------------- |
//! | `'a'` to `'z'`                               | `"lowercase"`    |
//! | `'A'` to `'Z'`                               | `"uppercase"`    |
//! | `'0'` to `'9'`                               | `"digit"`        |
//! | space `' '`, tab `'\t'`, newline `'\n'`, carriage return `'\r'` | `"whitespace"` |
//! | anything else (punctuation, non-ASCII, ...)  | `"other"`        |
//!
//! Only the ASCII letters and digits listed above count; for example `'Ω'` is `"other"`.
//!
//! ## Examples
//!
//! ```text
//! classify('q')  -> "lowercase"
//! classify('Q')  -> "uppercase"
//! classify('7')  -> "digit"
//! classify('\t') -> "whitespace"
//! classify('?')  -> "other"
//! classify('Ω')  -> "other"
//! ```
//!
//! ## Constraints
//!
//! - Any `char`.
//!
//! ## Hints
//!
//! 1. Range patterns work with `char`: `'a'..='z' => ...`.
//! 2. Several characters can share an arm: `' ' | '\t' | '\n' | '\r' => ...`.

pub fn classify(c: char) -> &'static str {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_and_digits() {
        assert_eq!(classify('q'), "lowercase");
        assert_eq!(classify('a'), "lowercase");
        assert_eq!(classify('z'), "lowercase");
        assert_eq!(classify('Q'), "uppercase");
        assert_eq!(classify('A'), "uppercase");
        assert_eq!(classify('Z'), "uppercase");
        assert_eq!(classify('0'), "digit");
        assert_eq!(classify('9'), "digit");
    }

    #[test]
    fn whitespace() {
        for c in [' ', '\t', '\n', '\r'] {
            assert_eq!(classify(c), "whitespace", "{c:?}");
        }
    }

    #[test]
    fn other_characters() {
        for c in ['?', '_', '@', '[', '`', '{', '/', ':', 'Ω', '中', '\u{0}'] {
            assert_eq!(classify(c), "other", "{c:?}");
        }
    }

    #[test]
    fn counts_over_all_ascii() {
        let mut lower = 0;
        let mut upper = 0;
        let mut digits = 0;
        for code in 0u8..=127 {
            match classify(code as char) {
                "lowercase" => lower += 1,
                "uppercase" => upper += 1,
                "digit" => digits += 1,
                _ => {}
            }
        }
        assert_eq!((lower, upper, digits), (26, 26, 10));
    }
}
