// ==================================================================
// PRACTICE 0246: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0246_char_classifier.md
// Run the tests:          cargo test practice::p0246
// ==================================================================

//! # Problem 0246: Classify Characters
//!
//! - **Difficulty:** Easy
//! - **Topics:** `char` range patterns, `|` alternatives, arm order, counting into a struct with `#[derive(Default)]`
//! - **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)
//!
//! ## Problem
//!
//! Classify a character into one of six kinds:
//!
//! - `Vowel`: `a e i o u`, lowercase or uppercase;
//! - `Consonant`: any other ASCII letter `a..=z` or `A..=Z`;
//! - `Digit`: `0..=9`;
//! - `Whitespace`: space, `'\t'`, `'\n'` or `'\r'`;
//! - `Punctuation`: one of `. , ! ? ; :`;
//! - `Other`: everything else (including non-ASCII letters such as `'ß'`).
//!
//! Implement `classify(c: char) -> CharKind` with one `match` that uses range patterns and `|`,
//! then `count_kinds(text: &str) -> KindCounts`, where `KindCounts` has one public `usize` field
//! per kind (`vowels`, `consonants`, `digits`, `whitespace`, `punctuation`, `other`).
//!
//! ## Examples
//!
//! ```text
//! classify('E')   => Vowel
//! classify('z')   => Consonant
//! classify('7')   => Digit
//! classify('\t')  => Whitespace
//! classify('?')   => Punctuation
//! classify('ß')   => Other
//! count_kinds("Hi, 2 you!") => KindCounts { vowels: 3, consonants: 2, digits: 1, whitespace: 2,
//!                                           punctuation: 2, other: 0 }
//! ```
//!
//! ## Constraints
//!
//! - `text` has at most `10^5` characters.
//!
//! ## Hints
//!
//! 1. Put the vowel arm **before** the letter-range arm: arms are tried from top to bottom.
//! 2. `'a'..='z' | 'A'..='Z'` combines two ranges in one arm.

/// The kind of a character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharKind {
    Vowel,
    Consonant,
    Digit,
    Whitespace,
    Punctuation,
    Other,
}

/// How many characters of each kind a text contains.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KindCounts {
    pub vowels: usize,
    pub consonants: usize,
    pub digits: usize,
    pub whitespace: usize,
    pub punctuation: usize,
    pub other: usize,
}

/// Classifies one character.
pub fn classify(c: char) -> CharKind {
    todo!("Write your solution here")
}

/// Counts the kinds of all characters in `text`.
pub fn count_kinds(text: &str) -> KindCounts {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_characters() {
        assert_eq!(classify('E'), CharKind::Vowel);
        assert_eq!(classify('a'), CharKind::Vowel);
        assert_eq!(classify('z'), CharKind::Consonant);
        assert_eq!(classify('Y'), CharKind::Consonant);
        assert_eq!(classify('7'), CharKind::Digit);
        assert_eq!(classify('\t'), CharKind::Whitespace);
        assert_eq!(classify('?'), CharKind::Punctuation);
        assert_eq!(classify('ß'), CharKind::Other);
        assert_eq!(classify('-'), CharKind::Other);
    }

    #[test]
    fn counting_example() {
        assert_eq!(
            count_kinds("Hi, 2 you!"),
            KindCounts {
                vowels: 3,
                consonants: 2,
                digits: 1,
                whitespace: 2,
                punctuation: 2,
                other: 0,
            }
        );
    }

    #[test]
    fn empty_and_unicode() {
        assert_eq!(count_kinds(""), KindCounts::default());
        let c = count_kinds("Fuß\n");
        assert_eq!(c.vowels, 1);
        assert_eq!(c.consonants, 1);
        assert_eq!(c.other, 1);
        assert_eq!(c.whitespace, 1);
    }

    #[test]
    fn long_text() {
        let text = "ab1. ".repeat(20_000);
        let c = count_kinds(&text);
        assert_eq!(
            (
                c.vowels,
                c.consonants,
                c.digits,
                c.punctuation,
                c.whitespace
            ),
            (20_000, 20_000, 20_000, 20_000, 20_000)
        );
    }
}
