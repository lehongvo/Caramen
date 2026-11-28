// ==================================================================
// PRACTICE 0573: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0573_borrowed_tokenizer.md
// Run the tests:          cargo test practice::p0573
// ==================================================================

//! # Problem 0573: A Borrowing Tokenizer
//!
//! - **Difficulty:** Medium
//! - **Topics:** `enum Token<'a>` holding `&'a str`, `struct Tokenizer<'a>`, `impl<'a> Iterator for Tokenizer<'a>` with `type Item = Token<'a>`, byte positions and `char` boundaries
//! - **Source:** Original · [The Rust Book 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions)
//!
//! ## Problem
//!
//! Split a small source text into tokens **without allocating**: every token borrows its text from the
//! source.
//!
//! ```text
//! pub enum Token<'a> { Ident(&'a str), Number(&'a str), Str(&'a str), Unterminated(&'a str), Symbol(char) }
//! ```
//!
//! Rules, applied at the current position after skipping whitespace:
//!
//! - **Ident**: a letter or `_`, followed by letters, digits or `_` (ASCII or Unicode alphanumerics).
//! - **Number**: one or more ASCII digits, optionally followed by `.` and one or more digits
//!   (`"42"`, `"3.14"`; in `"7."` the number is `"7"` and `.` is a symbol).
//! - **Str**: a `"`, then everything up to the next `"`. The token holds the text **between** the quotes
//!   (no escapes). If there is no closing quote, the token is `Unterminated(rest)` with the text after
//!   the opening quote, and tokenizing ends.
//! - **Symbol**: any other single character (it may be multi-byte, like `'€'`).
//!
//! Implement `Tokenizer<'a>` (fields `src: &'a str` and `pos: usize`, a byte offset) with
//! `new(src: &'a str)` and `rest(&self) -> &'a str` (the unread part), and
//! `impl<'a> Iterator for Tokenizer<'a>` with `type Item = Token<'a>`. Also write
//! `tokenize(src: &str) -> Vec<Token<'_>>`.
//!
//! ## Examples
//!
//! ```text
//! tokenize("let x = 3.14;")
//!   => [Ident("let"), Ident("x"), Symbol('='), Number("3.14"), Symbol(';')]
//! tokenize("say(\"hi there\") + 7.")
//!   => [Ident("say"), Symbol('('), Str("hi there"), Symbol(')'), Symbol('+'), Number("7"), Symbol('.')]
//! tokenize("price: 5\u{20ac} \"oops")
//!   => [Ident("price"), Symbol(':'), Number("5"), Symbol('\u{20ac}'), Unterminated("oops")]
//! ```
//!
//! ## Constraints
//!
//! - Any UTF-8 input.
//!
//! ## Hints
//!
//! 1. Work on `let rest = &self.src[self.pos..]` and `rest.char_indices()`; advance `self.pos` by byte
//!    lengths (`c.len_utf8()`), never by 1 per character.
//! 2. A helper that returns the byte length of the longest prefix whose characters satisfy a predicate
//!    keeps the token rules short.
//! 3. The `Item` type mentions `'a`, so tokens stay valid after the tokenizer is dropped.

/// A token borrowed from the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Ident(&'a str),
    Number(&'a str),
    Str(&'a str),
    Unterminated(&'a str),
    Symbol(char),
}

/// Splits a source text into borrowed tokens.
#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        todo!("Write your solution here")
    }

    pub fn rest(&self) -> &'a str {
        todo!("Write your solution here")
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        todo!("Write your solution here")
    }
}

/// All tokens of `src`.
pub fn tokenize(src: &str) -> Vec<Token<'_>> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn identifiers_numbers_symbols() {
        assert_eq!(
            tokenize("let x = 3.14;"),
            vec![
                Ident("let"),
                Ident("x"),
                Symbol('='),
                Number("3.14"),
                Symbol(';')
            ]
        );
        assert_eq!(
            tokenize("_a1 22b"),
            vec![Ident("_a1"), Number("22"), Ident("b")]
        );
    }

    #[test]
    fn strings_and_trailing_dot() {
        assert_eq!(
            tokenize("say(\"hi there\") + 7."),
            vec![
                Ident("say"),
                Symbol('('),
                Str("hi there"),
                Symbol(')'),
                Symbol('+'),
                Number("7"),
                Symbol('.')
            ]
        );
        assert_eq!(tokenize("\"\""), vec![Str("")]);
    }

    #[test]
    fn unicode_and_unterminated() {
        assert_eq!(
            tokenize("price: 5\u{20ac} \"oops"),
            vec![
                Ident("price"),
                Symbol(':'),
                Number("5"),
                Symbol('\u{20ac}'),
                Unterminated("oops")
            ]
        );
        assert_eq!(
            tokenize("caf\u{e9} na\u{ef}ve"),
            vec![Ident("caf\u{e9}"), Ident("na\u{ef}ve")]
        );
        assert!(tokenize("   \n\t").is_empty());
    }

    #[test]
    fn rest_and_tokens_outlive_the_tokenizer() {
        let src = String::from("a + b");
        let first_two;
        {
            let mut t = Tokenizer::new(&src);
            first_two = (t.next(), t.next());
            assert_eq!(t.rest(), " b");
        }
        assert_eq!(first_two, (Some(Ident("a")), Some(Symbol('+'))));
    }

    #[test]
    fn long_input() {
        let src = "x1 = 10.5 * y2;\n".repeat(1000);
        let tokens = tokenize(&src);
        assert_eq!(tokens.len(), 6000);
        assert_eq!(tokens[2], Number("10.5"));
        assert_eq!(tokens[3], Symbol('*'));
        assert_eq!(tokens[5999], Symbol(';'));
    }
}
