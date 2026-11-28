// ==================================================================
// PRACTICE 0410: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0410_transpose_optional_number.md
// Run the tests:          cargo test practice::p0410
// ==================================================================

//! # Problem 0410: Parse an Optional Number with transpose
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Option::transpose`, `Option<Result<T, E>>` vs `Result<Option<T>, E>`, `ParseIntError`, `?`
//! - **Source:** Original · [std `Option::transpose`](https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose)
//!
//! ## Problem
//!
//! Some inputs are both **optional** and **fallible**: a setting may be absent (that is fine),
//! but if it is present it must be a valid number. Mapping a parser over an `Option<&str>` gives
//! an `Option<Result<i32, ParseIntError>>`, which is awkward: you cannot use `?` on it directly.
//! `transpose` swaps the two layers into a `Result<Option<i32>, ParseIntError>`:
//!
//! ```text
//! None          -> Ok(None)
//! Some(Ok(v))   -> Ok(Some(v))
//! Some(Err(e))  -> Err(e)
//! ```
//!
//! Implement:
//!
//! - `parse_optional(input: Option<&str>) -> Result<Option<i32>, ParseIntError>`.
//! - `sum_optional(a: Option<&str>, b: Option<&str>) -> Result<i32, ParseIntError>`: treat a
//!   missing number as `0`, but return the parse error of an invalid one (check `a` first).
//! - `parse_port(raw: Option<&str>) -> Result<u16, String>`: a missing port means `8080`; a present
//!   port must parse as a `u16`, otherwise return ``"invalid port `RAW`: REASON"`` where `REASON` is
//!   the `Display` text of the `ParseIntError`.
//!
//! ## Examples
//!
//! ```text
//! parse_optional(None)          -> Ok(None)
//! parse_optional(Some("42"))    -> Ok(Some(42))
//! parse_optional(Some("4x"))    -> Err(ParseIntError { kind: InvalidDigit })
//! sum_optional(Some("5"), None) -> Ok(5)
//! parse_port(None)              -> Ok(8080)
//! parse_port(Some("443"))       -> Ok(443)
//! parse_port(Some("70000"))     -> Err("invalid port `70000`: number too large to fit in target type")
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `input.map(|s| s.parse::<i32>())` has type `Option<Result<i32, ParseIntError>>`.
//! 2. After `.transpose()` you can apply `?` and then `unwrap_or(0)`.
//! 3. In `parse_port`, call `map_err` inside the `map` closure, where you still have `s`.

use std::num::ParseIntError;

pub fn parse_optional(input: Option<&str>) -> Result<Option<i32>, ParseIntError> {
    todo!("Write your solution here")
}

pub fn sum_optional(a: Option<&str>, b: Option<&str>) -> Result<i32, ParseIntError> {
    todo!("Write your solution here")
}

pub fn parse_port(raw: Option<&str>) -> Result<u16, String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn parse_optional_three_cases() {
        assert_eq!(parse_optional(None), Ok(None));
        assert_eq!(parse_optional(Some("42")), Ok(Some(42)));
        assert_eq!(parse_optional(Some("-7")), Ok(Some(-7)));
        let err = parse_optional(Some("4x")).unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::InvalidDigit);
    }

    #[test]
    fn sum_treats_missing_as_zero() {
        assert_eq!(sum_optional(Some("5"), None), Ok(5));
        assert_eq!(sum_optional(None, None), Ok(0));
        assert_eq!(sum_optional(Some("5"), Some("-8")), Ok(-3));
    }

    #[test]
    fn sum_reports_the_first_invalid_number() {
        let err = sum_optional(Some(""), Some("x")).unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::Empty);
        let err = sum_optional(Some("1"), Some("x")).unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::InvalidDigit);
    }

    #[test]
    fn parse_port_default_and_valid() {
        assert_eq!(parse_port(None), Ok(8080));
        assert_eq!(parse_port(Some("443")), Ok(443));
        assert_eq!(parse_port(Some("0")), Ok(0));
    }

    #[test]
    fn parse_port_errors_explain_the_problem() {
        assert_eq!(
            parse_port(Some("70000")),
            Err("invalid port `70000`: number too large to fit in target type".to_string())
        );
        assert_eq!(
            parse_port(Some("http")),
            Err("invalid port `http`: invalid digit found in string".to_string())
        );
    }
}
