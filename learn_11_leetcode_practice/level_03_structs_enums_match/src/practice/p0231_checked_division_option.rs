// ==================================================================
// PRACTICE 0231: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0231_checked_division_option.md
// Run the tests:          cargo test practice::p0231
// ==================================================================

//! # Problem 0231: Checked Division with Option
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning `Option<T>`, `Some` / `None` as success / failure, overflow edge cases, early `return None`
//! - **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values) · [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
//!
//! ## Problem
//!
//! Some arithmetic has no sensible answer for certain inputs. Instead of panicking or returning a
//! magic number, return `Option`: `Some(answer)` when the answer exists and `None` otherwise.
//!
//! Implement, **without** calling the standard `checked_*` methods (write the checks yourself):
//!
//! - `safe_div(a: i32, b: i32) -> Option<i32>`: integer division truncating toward zero (like `/`).
//!   `None` when `b == 0`, and also for `i32::MIN / -1`, whose true result `2^31` does not fit in an
//!   `i32`;
//! - `safe_sqrt(x: f64) -> Option<f64>`: the square root, `None` for negative numbers and for `NaN`;
//! - `average(nums: &[i32]) -> Option<f64>`: the arithmetic mean, `None` for an empty slice (sum in
//!   `i64` so large inputs do not overflow);
//! - `percent(part: u32, whole: u32) -> Option<u32>`: `part` as a whole-number percentage of
//!   `whole`, rounded down. `None` if `whole == 0` or `part > whole`.
//!
//! ## Examples
//!
//! ```text
//! safe_div(7, 2)              => Some(3)
//! safe_div(-7, 2)             => Some(-3)
//! safe_div(1, 0)              => None
//! safe_div(i32::MIN, -1)      => None
//! safe_sqrt(9.0)              => Some(3.0)
//! safe_sqrt(-1.0)             => None
//! average(&[1, 2, 3, 4])      => Some(2.5)
//! average(&[])                => None
//! percent(1, 3)               => Some(33)
//! percent(5, 4)               => None
//! ```
//!
//! ## Constraints
//!
//! - `nums.len() <= 10^5`; `percent` inputs are any `u32` values.
//!
//! ## Hints
//!
//! 1. Guard clauses read well here: `if b == 0 { return None; }` at the top, then the normal case.
//! 2. `x >= 0.0` is `false` for `NaN`, so a single comparison rejects both negatives and `NaN`.
//! 3. In `percent`, `part * 100` can overflow a `u32`; compute in `u64`.

/// `a / b`, or `None` when the division is undefined or overflows.
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    todo!("Write your solution here")
}

/// The square root, or `None` for negative numbers and `NaN`.
pub fn safe_sqrt(x: f64) -> Option<f64> {
    todo!("Write your solution here")
}

/// The mean of `nums`, or `None` if it is empty.
pub fn average(nums: &[i32]) -> Option<f64> {
    todo!("Write your solution here")
}

/// `part` as a percentage of `whole`, rounded down.
pub fn percent(part: u32, whole: u32) -> Option<u32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn division() {
        assert_eq!(safe_div(7, 2), Some(3));
        assert_eq!(safe_div(-7, 2), Some(-3));
        assert_eq!(safe_div(0, 5), Some(0));
        assert_eq!(safe_div(1, 0), None);
        assert_eq!(safe_div(i32::MIN, -1), None);
        assert_eq!(safe_div(i32::MIN, 1), Some(i32::MIN));
        assert_eq!(safe_div(i32::MAX, -1), Some(-i32::MAX));
    }

    #[test]
    fn square_roots() {
        assert_eq!(safe_sqrt(9.0), Some(3.0));
        assert_eq!(safe_sqrt(0.0), Some(0.0));
        assert_eq!(safe_sqrt(-1.0), None);
        assert_eq!(safe_sqrt(f64::NAN), None);
        assert_eq!(safe_sqrt(f64::INFINITY), Some(f64::INFINITY));
    }

    #[test]
    fn averages() {
        assert_eq!(average(&[1, 2, 3, 4]), Some(2.5));
        assert_eq!(average(&[-5]), Some(-5.0));
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn average_does_not_overflow() {
        let big = vec![i32::MAX; 100_000];
        assert_eq!(average(&big), Some(i32::MAX as f64));
    }

    #[test]
    fn percentages() {
        assert_eq!(percent(1, 3), Some(33));
        assert_eq!(percent(3, 3), Some(100));
        assert_eq!(percent(0, 7), Some(0));
        assert_eq!(percent(5, 4), None);
        assert_eq!(percent(0, 0), None);
        assert_eq!(percent(u32::MAX - 1, u32::MAX), Some(99));
    }
}
