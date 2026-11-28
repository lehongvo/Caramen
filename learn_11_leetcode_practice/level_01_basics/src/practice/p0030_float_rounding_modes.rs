// ==================================================================
// PRACTICE 0030: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0030_float_rounding_modes.md
// Run the tests:          cargo test practice::p0030
// ==================================================================

//! # Problem 0030: Float Rounding Modes
//!
//! - **Difficulty:** Easy
//! - **Topics:** `f64::floor`, `ceil`, `round`, `trunc`, float-to-integer `as` casts (saturating, NaN), tuples
//! - **Source:** Original · [`f64` methods](https://doc.rust-lang.org/std/primitive.f64.html#method.round)
//!
//! ## Problem
//!
//! Converting a float to an integer requires choosing a rounding rule. Write
//! `rounding_modes(x: f64) -> (i64, i64, i64, i64)` that returns `(floor, ceil, round, trunc)` of
//! `x`, each converted to `i64`:
//!
//! - **floor**: the largest integer `<= x`;
//! - **ceil**: the smallest integer `>= x`;
//! - **round**: the nearest integer, with halfway cases rounded **away from zero** (`2.5 -> 3`, `-2.5 -> -3`);
//! - **trunc**: drop the fractional part (round toward zero).
//!
//! Values that do not fit in an `i64` must become `i64::MAX` or `i64::MIN` (whichever is closer),
//! and NaN ("not a number") must become `0`. Infinity counts as "too large".
//!
//! ## Examples
//!
//! ```text
//! rounding_modes(2.5)    -> (2, 3, 3, 2)
//! rounding_modes(-2.5)   -> (-3, -2, -3, -2)
//! rounding_modes(7.0)    -> (7, 7, 7, 7)
//! rounding_modes(-0.4)   -> (-1, 0, 0, 0)
//! rounding_modes(1e300)  -> (i64::MAX, i64::MAX, i64::MAX, i64::MAX)
//! rounding_modes(f64::NAN) -> (0, 0, 0, 0)
//! ```
//!
//! ## Constraints
//!
//! - Any `f64` value, including infinities and NaN.
//!
//! ## Hints
//!
//! 1. `f64` has methods named exactly `floor`, `ceil`, `round` and `trunc`; they return an `f64`.
//! 2. Casting a float with `as i64` already saturates out-of-range values and maps NaN to 0.

pub fn rounding_modes(x: f64) -> (i64, i64, i64, i64) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halfway_cases() {
        assert_eq!(rounding_modes(2.5), (2, 3, 3, 2));
        assert_eq!(rounding_modes(-2.5), (-3, -2, -3, -2));
        assert_eq!(rounding_modes(0.5), (0, 1, 1, 0));
    }

    #[test]
    fn whole_numbers_are_unchanged() {
        assert_eq!(rounding_modes(7.0), (7, 7, 7, 7));
        assert_eq!(rounding_modes(-7.0), (-7, -7, -7, -7));
        assert_eq!(rounding_modes(0.0), (0, 0, 0, 0));
    }

    #[test]
    fn small_fractions() {
        assert_eq!(rounding_modes(-0.4), (-1, 0, 0, 0));
        assert_eq!(rounding_modes(1.2), (1, 2, 1, 1));
        assert_eq!(rounding_modes(-1.7), (-2, -1, -2, -1));
    }

    #[test]
    fn out_of_range_values_saturate() {
        let max = (i64::MAX, i64::MAX, i64::MAX, i64::MAX);
        let min = (i64::MIN, i64::MIN, i64::MIN, i64::MIN);
        assert_eq!(rounding_modes(1e300), max);
        assert_eq!(rounding_modes(f64::INFINITY), max);
        assert_eq!(rounding_modes(-1e300), min);
        assert_eq!(rounding_modes(f64::NEG_INFINITY), min);
    }

    #[test]
    fn nan_becomes_zero() {
        assert_eq!(rounding_modes(f64::NAN), (0, 0, 0, 0));
    }
}
