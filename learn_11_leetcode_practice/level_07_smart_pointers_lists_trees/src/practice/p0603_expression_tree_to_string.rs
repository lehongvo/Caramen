// ==================================================================
// PRACTICE 0603: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0603_expression_tree_to_string.md
// Run the tests:          cargo test practice::p0603
// ==================================================================

//! # Problem 0603: Expression Tree to String
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Box<T>` recursive enum, `impl fmt::Display`, `write!`, operator precedence
//! - **Source:** Original · [The Rust Book 15.1](https://doc.rust-lang.org/book/ch15-01-box.html) · [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//!
//! ## Problem
//!
//! An expression tree over integers uses only addition and multiplication:
//!
//! ```text
//! pub enum Expr {
//!     Num(i64),
//!     Add(Box<Expr>, Box<Expr>),
//!     Mul(Box<Expr>, Box<Expr>),
//! }
//! ```
//!
//! Implement `std::fmt::Display` for `Expr` so that `expr.to_string()` prints the expression in
//! ordinary infix notation with the **fewest parentheses** that keep its meaning:
//!
//! - numbers print as themselves (negative numbers keep their sign, e.g. `-2`);
//! - operators are written with one space on each side: `1 + 2`, `3 * 4`;
//! - `*` binds tighter than `+`, so an `Add` operand of a `Mul` must be wrapped in parentheses;
//! - nothing else gets parentheses (both operators are associative, so `Add(1, Add(2, 3))` prints
//!   as `1 + 2 + 3`).
//!
//! ## Examples
//!
//! ```text
//! Add(1, Mul(2, 3))           -> "1 + 2 * 3"
//! Mul(Add(1, 2), 3)           -> "(1 + 2) * 3"
//! Mul(Add(1, 2), Add(3, 4))   -> "(1 + 2) * (3 + 4)"
//! Add(Add(1, 2), Add(3, 4))   -> "1 + 2 + 3 + 4"
//! Mul(Num(3), Num(-2))        -> "3 * -2"
//! ```
//!
//! ## Constraints
//!
//! - The tree has at most `1000` nodes and depth at most `100`.
//!
//! ## Hints
//!
//! 1. Inside `fmt`, `write!(f, "{} + {}", left, right)` formats each operand with *its own*
//!    `Display` implementation, so the recursion happens for free.
//! 2. Only `Mul` needs extra care: write a small helper that formats one operand of a `Mul`,
//!    adding `(` and `)` when that operand is an `Add`.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(value: i64) -> Box<Expr> {
        Box::new(Expr::Num(value))
    }

    fn add(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Add(left, right))
    }

    fn mul(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Mul(left, right))
    }

    #[test]
    fn multiplication_binds_tighter() {
        assert_eq!(add(num(1), mul(num(2), num(3))).to_string(), "1 + 2 * 3");
    }

    #[test]
    fn additions_inside_multiplication_get_parentheses() {
        assert_eq!(mul(add(num(1), num(2)), num(3)).to_string(), "(1 + 2) * 3");
        assert_eq!(
            mul(add(num(1), num(2)), add(num(3), num(4))).to_string(),
            "(1 + 2) * (3 + 4)"
        );
    }

    #[test]
    fn associative_chains_need_no_parentheses() {
        assert_eq!(
            add(add(num(1), num(2)), add(num(3), num(4))).to_string(),
            "1 + 2 + 3 + 4"
        );
        assert_eq!(mul(num(2), mul(num(3), num(4))).to_string(), "2 * 3 * 4");
    }

    #[test]
    fn single_and_negative_numbers() {
        assert_eq!(Expr::Num(42).to_string(), "42");
        assert_eq!(mul(num(3), num(-2)).to_string(), "3 * -2");
    }

    #[test]
    fn nested_mix() {
        // (1 + 2 * (3 + 4)) * 5
        let inner = add(num(1), mul(num(2), add(num(3), num(4))));
        assert_eq!(mul(inner, num(5)).to_string(), "(1 + 2 * (3 + 4)) * 5");
    }
}
