// ==================================================================
// PRACTICE 0423: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0423_temperature_error_enum.md
// Run the tests:          cargo test practice::p0423
// ==================================================================

//! # Problem 0423: A Custom Error Enum with Display
//!
//! - **Difficulty:** Easy
//! - **Topics:** error enums, `impl fmt::Display`, `impl std::error::Error`, `Box<dyn Error>`, `f64` parsing
//! - **Source:** Original · [std `Error`](https://doc.rust-lang.org/std/error/trait.Error.html) · [Rust By Example: Defining an error type](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html)
//!
//! ## Problem
//!
//! `String` errors are easy to create but hard to inspect: a caller cannot `match` on a message.
//! A dedicated error **enum** gives each failure its own variant (callers can match on it), and
//! implementing `Display` gives each variant a human-readable message. Implementing the
//! `std::error::Error` trait (which requires `Debug + Display`) lets the error be used everywhere
//! the standard library expects an error, for example inside a `Box<dyn Error>`.
//!
//! Parse temperatures such as `"21.5C"`, `"-40F"` or `"300K"` into `Celsius`:
//!
//! - trim the input; an empty input is `TemperatureError::Empty`;
//! - the **last character** is the unit. If it is an ASCII digit or `'.'`, the unit is missing:
//!   `MissingUnit`. If it is any other character except `C`, `F` or `K`: `UnknownUnit(c)`;
//! - the rest (trimmed) must parse as a **finite** `f64`; otherwise `NotANumber(rest)`;
//! - convert to Celsius: `F` -> `(f - 32) * 5 / 9`, `K` -> `k - 273.15`;
//! - a result below `-273.15` is `BelowAbsoluteZero(celsius)`.
//!
//! Implement `parse_temperature(s: &str) -> Result<Celsius, TemperatureError>` and `Display` for
//! `TemperatureError` with these messages:
//!
//! ```text
//! Empty                  -> "empty input"
//! MissingUnit            -> "missing unit (expected C, F or K)"
//! UnknownUnit('X')       -> "unknown unit `X`"
//! NotANumber("abc")      -> "`abc` is not a number"
//! BelowAbsoluteZero(c)   -> "C_WITH_2_DECIMALS C is below absolute zero"   e.g. "-300.00 C is below absolute zero"
//! ```
//!
//! ## Examples
//!
//! ```text
//! parse_temperature("21.5C")  -> Ok(Celsius(21.5))
//! parse_temperature("-40F")   -> Ok(Celsius(-40.0))
//! parse_temperature(" 300K ") -> Ok(Celsius(26.85))   (approximately)
//! parse_temperature("21")     -> Err(MissingUnit)
//! parse_temperature("21X")    -> Err(UnknownUnit('X'))
//! parse_temperature("hotC")   -> Err(NotANumber("hot"))
//! parse_temperature("-300C")  -> Err(BelowAbsoluteZero(-300.0))
//! ```
//!
//! ## Constraints
//!
//! - No special constraints. Units are case-sensitive (`"21c"` is an unknown unit).
//!
//! ## Hints
//!
//! 1. `s.chars().last()` returns the last character; `s.char_indices().last()` also gives its byte
//!    position, so you can slice off the number part safely.
//! 2. `impl fmt::Display for TemperatureError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { match self { ... } } }`
//!    and inside each arm `write!(f, "...")`.
//! 3. `impl std::error::Error for TemperatureError {}` is enough: every method has a default.

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Celsius(pub f64);

#[derive(Debug, PartialEq)]
pub enum TemperatureError {
    Empty,
    MissingUnit,
    UnknownUnit(char),
    NotANumber(String),
    BelowAbsoluteZero(f64),
}

impl fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for TemperatureError {}

pub fn parse_temperature(s: &str) -> Result<Celsius, TemperatureError> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(result: Result<Celsius, TemperatureError>, expected: f64) -> bool {
        matches!(result, Ok(Celsius(c)) if (c - expected).abs() < 1e-9)
    }

    #[test]
    fn valid_temperatures() {
        assert!(close(parse_temperature("21.5C"), 21.5));
        assert!(close(parse_temperature("-40F"), -40.0));
        assert!(close(parse_temperature(" 300K "), 26.85));
        assert!(close(parse_temperature("212 F"), 100.0));
        assert!(close(parse_temperature("0K"), -273.15));
    }

    #[test]
    fn unit_errors() {
        assert_eq!(parse_temperature(""), Err(TemperatureError::Empty));
        assert_eq!(parse_temperature("   "), Err(TemperatureError::Empty));
        assert_eq!(parse_temperature("21"), Err(TemperatureError::MissingUnit));
        assert_eq!(parse_temperature("21."), Err(TemperatureError::MissingUnit));
        assert_eq!(
            parse_temperature("21X"),
            Err(TemperatureError::UnknownUnit('X'))
        );
        assert_eq!(
            parse_temperature("21c"),
            Err(TemperatureError::UnknownUnit('c'))
        );
    }

    #[test]
    fn number_and_range_errors() {
        assert_eq!(
            parse_temperature("hotC"),
            Err(TemperatureError::NotANumber("hot".to_string()))
        );
        assert_eq!(
            parse_temperature("C"),
            Err(TemperatureError::NotANumber(String::new()))
        );
        assert_eq!(
            parse_temperature("infC"),
            Err(TemperatureError::NotANumber("inf".to_string()))
        );
        assert!(matches!(
            parse_temperature("-300C"),
            Err(TemperatureError::BelowAbsoluteZero(c)) if c == -300.0
        ));
        assert!(matches!(
            parse_temperature("-1K"),
            Err(TemperatureError::BelowAbsoluteZero(_))
        ));
    }

    #[test]
    fn display_messages() {
        assert_eq!(TemperatureError::Empty.to_string(), "empty input");
        assert_eq!(
            TemperatureError::MissingUnit.to_string(),
            "missing unit (expected C, F or K)"
        );
        assert_eq!(
            TemperatureError::UnknownUnit('X').to_string(),
            "unknown unit `X`"
        );
        assert_eq!(
            TemperatureError::NotANumber("abc".to_string()).to_string(),
            "`abc` is not a number"
        );
        assert_eq!(
            TemperatureError::BelowAbsoluteZero(-300.0).to_string(),
            "-300.00 C is below absolute zero"
        );
    }

    #[test]
    fn works_as_a_boxed_std_error() {
        let err: Box<dyn Error> = Box::new(parse_temperature("12Q").unwrap_err());
        assert_eq!(err.to_string(), "unknown unit `Q`");
        assert!(err.source().is_none());
    }
}
