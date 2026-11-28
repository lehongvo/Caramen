// ==================================================================
// PRACTICE 0285: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0285_expression_tokenizer.md
// Run the tests:          cargo test practice::p0285
// ==================================================================

//! # Problem 0285: Expression Tokenizer
//!
//! - **Difficulty:** Medium
//! - **Topics:** a `Token` enum with data, scanning `char`s with `peekable` and `while let`, `match` with ranges and guards, `Option<Vec<Token>>`
//! - **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) · [`Peekable`](https://doc.rust-lang.org/std/iter/struct.Peekable.html)
//!
//! ## Problem
//!
//! Turn an arithmetic expression into a list of tokens, the first step of any calculator or
//! compiler:
//!
//! ```text
//! enum Op    { Add, Sub, Mul, Div }
//! enum Token { Num(u64), Ident(String), Op(Op), LParen, RParen }
//! ```
//!
//! Rules for `tokenize(input: &str) -> Option<Vec<Token>>`:
//!
//! - spaces and tabs between tokens are skipped;
//! - a run of ASCII digits is one `Num` (it must fit in a `u64`, otherwise the input is rejected);
//! - an identifier starts with an ASCII letter or `_` and continues with ASCII letters, digits or
//!   `_` (`x1`, `_tmp`, `rate`);
//! - `+ - * /` are operators and `(` `)` are parentheses;
//! - a digit run directly followed by a letter or `_` (like `12abc`) is invalid;
//! - any other character makes the whole input invalid: return `None`.
//!
//! Then write `count_ops(tokens: &[Token]) -> [usize; 4]`: how many `Add`, `Sub`, `Mul` and `Div`
//! operators appear, in that order.
//!
//! ## Examples
//!
//! ```text
//! tokenize("3 + 4*x1")    => Some([Num(3), Op(Add), Num(4), Op(Mul), Ident("x1")])
//! tokenize("(a-_b)/20")   => Some([LParen, Ident("a"), Op(Sub), Ident("_b"), RParen, Op(Div), Num(20)])
//! tokenize("")            => Some([])
//! tokenize("2 ^ 3")       => None      ('^' is not allowed)
//! tokenize("12abc")       => None
//! tokenize("99999999999999999999") => None   (does not fit in a u64)
//! ```
//!
//! ## Constraints
//!
//! - `input.len() <= 10^5`.
//!
//! ## Hints
//!
//! 1. `let mut chars = input.chars().peekable();` and `while let Some(&c) = chars.peek() { ... }`.
//! 2. A helper `take_while_into(chars, String, predicate)` collects a run of characters.
//! 3. `match c { '0'..='9' => ..., c if c.is_ascii_alphabetic() || c == '_' => ..., '+' => ..., _ => return None }`.

/// An arithmetic operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// One token of an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Num(u64),
    Ident(String),
    Op(Op),
    LParen,
    RParen,
}

/// Splits `input` into tokens, or `None` if it contains an invalid character or number.
pub fn tokenize(input: &str) -> Option<Vec<Token>> {
    todo!("Write your solution here")
}

/// Counts of `Add`, `Sub`, `Mul` and `Div` tokens.
pub fn count_ops(tokens: &[Token]) -> [usize; 4] {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ident(s: &str) -> Token {
        Token::Ident(s.to_string())
    }

    #[test]
    fn simple_expression() {
        assert_eq!(
            tokenize("3 + 4*x1"),
            Some(vec![
                Token::Num(3),
                Token::Op(Op::Add),
                Token::Num(4),
                Token::Op(Op::Mul),
                ident("x1")
            ])
        );
    }

    #[test]
    fn parentheses_and_underscores() {
        assert_eq!(
            tokenize("(a-_b)/20"),
            Some(vec![
                Token::LParen,
                ident("a"),
                Token::Op(Op::Sub),
                ident("_b"),
                Token::RParen,
                Token::Op(Op::Div),
                Token::Num(20)
            ])
        );
        assert_eq!(tokenize("\t rate_2 "), Some(vec![ident("rate_2")]));
    }

    #[test]
    fn empty_and_invalid() {
        assert_eq!(tokenize(""), Some(vec![]));
        assert_eq!(tokenize("   "), Some(vec![]));
        assert_eq!(tokenize("2 ^ 3"), None);
        assert_eq!(tokenize("12abc"), None);
        assert_eq!(tokenize("12_"), None);
        assert_eq!(tokenize("a.b"), None);
        assert_eq!(tokenize("x = 1"), None);
    }

    #[test]
    fn number_limits() {
        assert_eq!(
            tokenize("18446744073709551615"),
            Some(vec![Token::Num(u64::MAX)])
        );
        assert_eq!(tokenize("18446744073709551616"), None);
        assert_eq!(tokenize("007"), Some(vec![Token::Num(7)]));
    }

    #[test]
    fn counting_operators() {
        let tokens = tokenize("a+b-c*d/e+f").expect("valid");
        assert_eq!(count_ops(&tokens), [2, 1, 1, 1]);
        assert_eq!(count_ops(&[]), [0; 4]);
    }

    #[test]
    fn long_input() {
        let input = "x1 + 22 * ".repeat(10_000) + "y";
        let tokens = tokenize(&input).expect("valid");
        assert_eq!(tokens.len(), 40_001);
        assert_eq!(count_ops(&tokens), [10_000, 0, 10_000, 0]);
    }
}
