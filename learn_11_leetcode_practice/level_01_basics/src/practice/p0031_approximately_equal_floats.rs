// ==================================================================
// PRACTICE 0031: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0031_approximately_equal_floats.md
// Run the tests:          cargo test practice::p0031
// ==================================================================

//! # Problem 0031: Approximately Equal Floats
//!
//! - **Difficulty:** Easy
//! - **Topics:** floating-point precision, `f64::abs`, `f64::max`, NaN and infinity, `||`
//! - **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types)
//!
//! ## Problem
//!
//! In binary floating point, `0.1 + 0.2 == 0.3` is **false** (the left side is
//! `0.30000000000000004`). Code that compares measured or computed floats therefore uses a
//! tolerance. Write two functions:
//!
//! 1. `approx_eq_abs(a: f64, b: f64, tolerance: f64) -> bool` returns `true` when
//!    `|a - b| <= tolerance`.
//! 2. `approx_eq_rel(a: f64, b: f64, rel_tolerance: f64) -> bool` returns `true` when
//!    `|a - b| <= rel_tolerance * max(|a|, |b|)`, a tolerance that scales with the size of the numbers.
//!
//! Both functions must also follow these rules:
//!
//! - if `a == b` exactly (this includes two equal infinities), the result is `true`;
//! - if `a` or `b` is NaN, the result is `false` (NaN is not equal to anything, not even itself).
//!
//! ## Examples
//!
//! ```text
//! approx_eq_abs(0.1 + 0.2, 0.3, 1e-9)        -> true
//! approx_eq_abs(1.0, 1.1, 1e-9)              -> false
//! approx_eq_abs(f64::INFINITY, f64::INFINITY, 1e-9) -> true
//! approx_eq_abs(f64::NAN, f64::NAN, 1.0)     -> false
//!
//! approx_eq_rel(1e20, 1e20 + 1e5, 1e-9)      -> true     the difference is tiny compared to 1e20
//! approx_eq_rel(1e-20, 2e-20, 1e-9)          -> false    a 100% difference, even though both are tiny
//! ```
//!
//! ## Constraints
//!
//! - `tolerance >= 0.0` and `rel_tolerance >= 0.0` (both finite).
//!
//! ## Hints
//!
//! 1. Check `a == b` first; `f64::INFINITY - f64::INFINITY` is NaN, which would make the tolerance test fail.
//! 2. Every comparison involving NaN is `false`, so a NaN input automatically fails `<=`.
//! 3. `x.abs()` and `x.max(y)` are methods on `f64`.

pub fn approx_eq_abs(a: f64, b: f64, tolerance: f64) -> bool {
    todo!("Write your solution here")
}

pub fn approx_eq_rel(a: f64, b: f64, rel_tolerance: f64) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_classic_example() {
        let sum = 0.1 + 0.2;
        assert!(sum != 0.3);
        assert!(approx_eq_abs(sum, 0.3, 1e-9));
        assert!(approx_eq_rel(sum, 0.3, 1e-9));
    }

    #[test]
    fn clearly_different_values() {
        assert!(!approx_eq_abs(1.0, 1.1, 1e-9));
        assert!(!approx_eq_rel(1.0, 1.1, 1e-9));
        assert!(approx_eq_abs(1.0, 1.1, 0.2));
    }

    #[test]
    fn infinities() {
        assert!(approx_eq_abs(f64::INFINITY, f64::INFINITY, 1e-9));
        assert!(approx_eq_rel(f64::NEG_INFINITY, f64::NEG_INFINITY, 1e-9));
        assert!(!approx_eq_abs(f64::INFINITY, f64::NEG_INFINITY, 1e-9));
        assert!(!approx_eq_abs(f64::INFINITY, 1e300, 1e-9));
    }

    #[test]
    fn nan_is_never_equal() {
        assert!(!approx_eq_abs(f64::NAN, f64::NAN, 1.0));
        assert!(!approx_eq_abs(f64::NAN, 0.0, 1.0));
        assert!(!approx_eq_rel(f64::NAN, f64::NAN, 1.0));
    }

    #[test]
    fn relative_tolerance_scales() {
        assert!(approx_eq_rel(1e20, 1e20 + 1e5, 1e-9));
        assert!(!approx_eq_abs(1e20, 1e20 + 1e5, 1e-9));
        assert!(!approx_eq_rel(1e-20, 2e-20, 1e-9));
        assert!(approx_eq_abs(1e-20, 2e-20, 1e-9));
    }

    #[test]
    fn zero_tolerance_means_exact() {
        assert!(approx_eq_abs(2.0, 2.0, 0.0));
        assert!(!approx_eq_abs(2.0, 2.000_000_1, 0.0));
        assert!(approx_eq_rel(0.0, -0.0, 0.0));
    }
}
