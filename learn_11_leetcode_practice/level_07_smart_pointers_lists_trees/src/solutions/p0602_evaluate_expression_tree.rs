//! # Problem 0602: Evaluate an Expression Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Box<T>` in enum variants, recursion, `match` through `&Box<T>`, `checked_*` arithmetic, `?` on `Option`
//! - **Source:** Original · [The Rust Book 15.1](https://doc.rust-lang.org/book/ch15-01-box.html)
//!
//! ## Problem
//!
//! An arithmetic expression can be stored as a tree: leaves are numbers and inner nodes are
//! operators whose operands are smaller expressions. Each operand lives in its own `Box`, which
//! gives the recursive enum a finite size:
//!
//! ```text
//! pub enum Expr {
//!     Num(i64),
//!     Neg(Box<Expr>),
//!     Add(Box<Expr>, Box<Expr>),
//!     Sub(Box<Expr>, Box<Expr>),
//!     Mul(Box<Expr>, Box<Expr>),
//!     Div(Box<Expr>, Box<Expr>),
//! }
//! ```
//!
//! Implement `eval(expr: &Expr) -> Option<i64>`, which computes the value of the expression.
//! `Div` is integer division that truncates toward zero (like Rust's `/`). Return `None` if any
//! step divides by zero or overflows `i64`; otherwise return `Some(value)`.
//!
//! ## Examples
//!
//! ```text
//! Add(Num(1), Mul(Num(2), Num(3)))          -> Some(7)     // 1 + 2 * 3
//! Neg(Sub(Num(4), Num(10)))                 -> Some(6)     // -(4 - 10)
//! Div(Num(7), Num(-2))                      -> Some(-3)    // truncates toward zero
//! Div(Num(1), Sub(Num(2), Num(2)))          -> None        // division by zero
//! Mul(Num(i64::MAX), Num(2))                -> None        // overflow
//! ```
//!
//! ## Constraints
//!
//! - The tree has at most `1000` nodes and depth at most `100`.
//!
//! ## Hints
//!
//! 1. One `match` arm per variant. A leaf returns its number; an operator evaluates its operands
//!    first (recursion), then combines them.
//! 2. Matching on `&Expr` gives you `&Box<Expr>` operands; passing one to `eval` works because
//!    `&Box<Expr>` coerces to `&Expr`.
//! 3. `i64::checked_add`, `checked_sub`, `checked_mul`, `checked_div` and `checked_neg` return
//!    `Option<i64>`, and `?` stops early on `None`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a post-order recursion: evaluate the children, then apply the operator with the
//! `checked_*` method that matches it. `checked_div` already returns `None` both for a zero
//! divisor and for the one overflowing case `i64::MIN / -1`.
//!
//! **Complexity:** `O(n)` time for `n` nodes, `O(depth)` stack space.
//!
//! **Rust concepts:**
//! - `Box<Expr>` makes each operand an owned heap value, so `Expr` has a fixed size.
//! - Deref coercion turns `&Box<Expr>` into `&Expr` at the call `eval(left)`.
//! - `?` on an `Option` returns `None` from the function as soon as a sub-result is `None`.
// @solution-end

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

pub fn eval(expr: &Expr) -> Option<i64> {
    // @solution-start
    match expr {
        Expr::Num(value) => Some(*value),
        Expr::Neg(inner) => eval(inner)?.checked_neg(),
        Expr::Add(left, right) => eval(left)?.checked_add(eval(right)?),
        Expr::Sub(left, right) => eval(left)?.checked_sub(eval(right)?),
        Expr::Mul(left, right) => eval(left)?.checked_mul(eval(right)?),
        Expr::Div(left, right) => eval(left)?.checked_div(eval(right)?),
    }
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(value: i64) -> Box<Expr> {
        Box::new(Expr::Num(value))
    }

    #[test]
    fn precedence_is_encoded_by_the_tree() {
        let expr = Expr::Add(num(1), Box::new(Expr::Mul(num(2), num(3))));
        assert_eq!(eval(&expr), Some(7));
        let expr = Expr::Mul(Box::new(Expr::Add(num(1), num(2))), num(3));
        assert_eq!(eval(&expr), Some(9));
    }

    #[test]
    fn negation_and_subtraction() {
        let expr = Expr::Neg(Box::new(Expr::Sub(num(4), num(10))));
        assert_eq!(eval(&expr), Some(6));
        assert_eq!(eval(&Expr::Num(-5)), Some(-5));
    }

    #[test]
    fn division_truncates_toward_zero() {
        assert_eq!(eval(&Expr::Div(num(7), num(-2))), Some(-3));
        assert_eq!(eval(&Expr::Div(num(-7), num(2))), Some(-3));
    }

    #[test]
    fn division_by_zero_is_none() {
        let zero = Box::new(Expr::Sub(num(2), num(2)));
        assert_eq!(eval(&Expr::Div(num(1), zero)), None);
    }

    #[test]
    fn overflow_is_none() {
        assert_eq!(eval(&Expr::Mul(num(i64::MAX), num(2))), None);
        assert_eq!(eval(&Expr::Div(num(i64::MIN), num(-1))), None);
        assert_eq!(eval(&Expr::Neg(num(i64::MIN))), None);
    }

    #[test]
    fn deep_left_leaning_sum() {
        // ((((0 + 1) + 2) + 3) ... + 100)
        let mut expr = Expr::Num(0);
        for i in 1..=100 {
            expr = Expr::Add(Box::new(expr), num(i));
        }
        assert_eq!(eval(&expr), Some(5050));
    }
}
