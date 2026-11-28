// ==================================================================
// PRACTICE 0554: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0554_fraction_type.md
// Run the tests:          cargo test practice::p0554
// ==================================================================

//! # Problem 0554: A Fraction Type
//!
//! - **Difficulty:** Medium
//! - **Topics:** a normalised value type, `Add`/`Sub`/`Mul`/`Div`/`Neg`, manual `Ord` by cross-multiplication, `From<i64>`, `Display`
//! - **Source:** Original · [std::ops](https://doc.rust-lang.org/std/ops/index.html)
//!
//! ## Problem
//!
//! Implement an exact rational number `Fraction` with private fields `num: i64` and `den: i64`. Every
//! `Fraction` is kept in **normal form**:
//!
//! - `den > 0` (the sign lives in `num`);
//! - `gcd(|num|, den) == 1`;
//! - zero is `0/1`.
//!
//! Because of the normal form, the derived `PartialEq`/`Eq`/`Hash` are correct: equal values have equal
//! fields.
//!
//! Implement:
//!
//! - `Fraction::new(num: i64, den: i64) -> Fraction`, normalising; it panics with the message
//!   `"denominator must not be zero"` when `den == 0`. Getters `numer(self)` and `denom(self)`.
//! - `From<i64> for Fraction` (`n` becomes `n/1`).
//! - `Add`, `Sub`, `Mul`, `Div`, `Neg`. Dividing by a zero fraction panics with `"division by zero"`.
//! - `Ord` and `PartialOrd`: compare `a/b` and `c/d` through `a * d` and `c * b`, computed in `i128`
//!   so the products cannot overflow.
//! - `Display`: `"3/4"`, `"-1/2"`, and just `"5"` when the denominator is `1`.
//!
//! ## Examples
//!
//! ```text
//! Fraction::new(6, -8)                          => -3/4
//! Fraction::new(1, 2) + Fraction::new(1, 3)     => 5/6
//! Fraction::new(1, 2) - Fraction::from(1)       => -1/2
//! Fraction::new(2, 3) * Fraction::new(9, 4)     => 3/2
//! Fraction::new(1, 2) / Fraction::new(1, 4)     => 2
//! Fraction::new(1, 3) < Fraction::new(1, 2)     => true
//! ```
//!
//! ## Constraints
//!
//! - Intermediate results fit in `i64` in the tests (comparison must still use `i128`).
//!
//! ## Hints
//!
//! 1. Write a private `fn gcd(a: i64, b: i64) -> i64` (Euclid) and do all normalisation in `new`; every
//!    operator then just calls `Fraction::new` with the textbook formula.
//! 2. `a/b + c/d = (a*d + c*b) / (b*d)`, `a/b / (c/d) = (a*d) / (b*c)`.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// An exact rational number in normal form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fraction {}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Fraction {
        todo!("Write your solution here")
    }

    pub fn numer(self) -> i64 {
        todo!("Write your solution here")
    }

    pub fn denom(self) -> i64 {
        todo!("Write your solution here")
    }
}

impl From<i64> for Fraction {
    fn from(n: i64) -> Self {
        todo!("Write your solution here")
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Fraction) -> Fraction {
        todo!("Write your solution here")
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Fraction {
        todo!("Write your solution here")
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Fraction {
        todo!("Write your solution here")
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Fraction {
        todo!("Write your solution here")
    }
}

impl Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Fraction {
        todo!("Write your solution here")
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!("Write your solution here")
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fr(n: i64, d: i64) -> Fraction {
        Fraction::new(n, d)
    }

    #[test]
    fn normal_form() {
        let f = fr(6, -8);
        assert_eq!((f.numer(), f.denom()), (-3, 4));
        assert_eq!(fr(0, -5), fr(0, 1));
        assert_eq!(fr(10, 5), Fraction::from(2));
        assert_eq!(fr(-4, -6).to_string(), "2/3");
    }

    #[test]
    fn arithmetic() {
        assert_eq!(fr(1, 2) + fr(1, 3), fr(5, 6));
        assert_eq!(fr(1, 2) - Fraction::from(1), fr(-1, 2));
        assert_eq!(fr(2, 3) * fr(9, 4), fr(3, 2));
        assert_eq!(fr(1, 2) / fr(1, 4), Fraction::from(2));
        assert_eq!(-fr(3, 7), fr(-3, 7));
        assert_eq!((fr(1, 2) / fr(-1, 4)).to_string(), "-2");
    }

    #[test]
    fn ordering() {
        assert!(fr(1, 3) < fr(1, 2));
        assert!(fr(-1, 2) < fr(-1, 3));
        assert_eq!(fr(2, 4).cmp(&fr(1, 2)), Ordering::Equal);
        let big = fr(i64::MAX - 1, i64::MAX);
        let bigger = fr(i64::MAX, i64::MAX - 1);
        assert!(big < bigger);
        let mut v = [fr(3, 4), fr(-1, 2), fr(1, 3), Fraction::from(1)];
        v.sort();
        let shown: Vec<String> = v.iter().map(Fraction::to_string).collect();
        assert_eq!(shown, vec!["-1/2", "1/3", "3/4", "1"]);
    }

    #[test]
    #[should_panic(expected = "denominator must not be zero")]
    fn zero_denominator_panics() {
        let _ = fr(1, 0);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn division_by_zero_panics() {
        let _ = fr(1, 2) / fr(0, 3);
    }

    #[test]
    fn harmonic_sum() {
        // 1/1 + 1/2 + ... + 1/10 = 7381/2520
        let h = (1..=10).fold(Fraction::from(0), |acc, k| acc + fr(1, k));
        assert_eq!(h, fr(7381, 2520));
    }
}
