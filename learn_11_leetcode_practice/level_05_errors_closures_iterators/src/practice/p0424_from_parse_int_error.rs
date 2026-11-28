// ==================================================================
// PRACTICE 0424: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0424_from_parse_int_error.md
// Run the tests:          cargo test practice::p0424
// ==================================================================

//! # Problem 0424: Automatic Error Conversion with From
//!
//! - **Difficulty:** Easy
//! - **Topics:** `impl From<ParseIntError>`, `?` calls `From::from`, struct-like enum variants, `Error::source`
//! - **Source:** Original · [The Rust Book 9.2: the `?` operator and `From`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator) · [Rust By Example: Wrapping errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html)
//!
//! ## Problem
//!
//! When `?` meets an `Err(e)`, it does not return `e` unchanged: it returns `From::from(e)`. So if
//! your error type implements `From<ParseIntError>`, you can apply `?` to a `parse` call inside a
//! function that returns *your* error type, and the conversion happens automatically.
//!
//! A server reads its configuration from `(key, value)` pairs. Implement:
//!
//! - `impl From<ParseIntError> for ConfigError`, producing `ConfigError::BadNumber(err)`.
//! - `Display` for `ConfigError`:
//!   - `Missing(key)` -> ``"missing key `KEY`"``
//!   - `BadNumber(e)` -> `"invalid number: E"` (the `ParseIntError` message)
//!   - `OutOfRange { key, value }` -> ``"`KEY` = VALUE is out of range"``
//! - `Error::source` for `ConfigError`: `Some(the ParseIntError)` for `BadNumber`, `None` otherwise.
//! - `read_number(pairs: &[(&str, &str)], key: &str) -> Result<i64, ConfigError>`: the value of the
//!   first pair with that key, trimmed and parsed as an `i64`. Use `?` on the parse.
//! - `load(pairs: &[(&str, &str)]) -> Result<ServerConfig, ConfigError>`: read `port` (must be in
//!   `1..=65535`) and then `workers` (must be in `1..=64`); a value outside its range gives
//!   `OutOfRange` with that key and value.
//!
//! ## Examples
//!
//! ```text
//! pairs = [("port", "8080"), ("workers", " 8 ")]
//! load(pairs)                             -> Ok(ServerConfig { port: 8080, workers: 8 })
//! load([("workers", "8")])                -> Err(Missing("port"))
//! load([("port", "80x"), ...])            -> Err(BadNumber(..))   displayed as "invalid number: invalid digit found in string"
//! load([("port", "0"), ...])              -> Err(OutOfRange { key: "port", value: 0 })
//! OutOfRange { key: "workers", value: 99 }.to_string() -> "`workers` = 99 is out of range"
//! ```
//!
//! ## Constraints
//!
//! - At most `100` pairs.
//!
//! ## Hints
//!
//! 1. `impl From<ParseIntError> for ConfigError { fn from(err: ParseIntError) -> Self { ... } }`
//! 2. With that impl, `Ok(value.trim().parse::<i64>()?)` compiles in `read_number`.
//! 3. `fn source(&self) -> Option<&(dyn Error + 'static)>`: return `Some(e)` for the `BadNumber(e)` arm.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    Missing(String),
    BadNumber(ParseIntError),
    OutOfRange { key: String, value: i64 },
}

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub port: u16,
    pub workers: u8,
}

impl From<ParseIntError> for ConfigError {
    fn from(err: ParseIntError) -> Self {
        todo!("Write your solution here")
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        todo!("Write your solution here")
    }
}

pub fn read_number(pairs: &[(&str, &str)], key: &str) -> Result<i64, ConfigError> {
    todo!("Write your solution here")
}

pub fn load(pairs: &[(&str, &str)]) -> Result<ServerConfig, ConfigError> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_wraps_the_parse_error() {
        let parse_err = "x".parse::<i64>().unwrap_err();
        let err: ConfigError = parse_err.clone().into();
        assert_eq!(err, ConfigError::BadNumber(parse_err));
    }

    #[test]
    fn read_number_uses_question_mark() {
        let pairs = [("port", " 8080 "), ("port", "1"), ("bad", "1e3")];
        assert_eq!(read_number(&pairs, "port"), Ok(8080));
        assert_eq!(
            read_number(&pairs, "host"),
            Err(ConfigError::Missing("host".to_string()))
        );
        assert!(matches!(
            read_number(&pairs, "bad"),
            Err(ConfigError::BadNumber(_))
        ));
    }

    #[test]
    fn load_valid_config() {
        assert_eq!(
            load(&[("port", "8080"), ("workers", " 8 ")]),
            Ok(ServerConfig {
                port: 8080,
                workers: 8
            })
        );
        assert_eq!(
            load(&[("workers", "64"), ("port", "65535")]),
            Ok(ServerConfig {
                port: 65_535,
                workers: 64
            })
        );
    }

    #[test]
    fn load_errors() {
        assert_eq!(
            load(&[("workers", "8")]),
            Err(ConfigError::Missing("port".to_string()))
        );
        assert_eq!(
            load(&[("port", "0"), ("workers", "8")]),
            Err(ConfigError::OutOfRange {
                key: "port".to_string(),
                value: 0
            })
        );
        assert_eq!(
            load(&[("port", "80"), ("workers", "99")]),
            Err(ConfigError::OutOfRange {
                key: "workers".to_string(),
                value: 99
            })
        );
        let err = load(&[("port", "80x"), ("workers", "8")]).unwrap_err();
        assert_eq!(
            err.to_string(),
            "invalid number: invalid digit found in string"
        );
    }

    #[test]
    fn display_and_source() {
        assert_eq!(
            ConfigError::Missing("port".to_string()).to_string(),
            "missing key `port`"
        );
        let out = ConfigError::OutOfRange {
            key: "workers".to_string(),
            value: 99,
        };
        assert_eq!(out.to_string(), "`workers` = 99 is out of range");
        assert!(out.source().is_none());
        let bad = ConfigError::from("".parse::<i64>().unwrap_err());
        let inner = bad.source().map(|e| e.to_string());
        assert_eq!(
            inner.as_deref(),
            Some("cannot parse integer from empty string")
        );
    }
}
