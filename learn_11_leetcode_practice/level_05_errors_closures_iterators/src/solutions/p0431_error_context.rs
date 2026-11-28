//! # Problem 0431: Add Context to Errors
//!
//! - **Difficulty:** Medium
//! - **Topics:** wrapper error with `Box<dyn Error>` source, generic `E: Error + 'static`, `map_err` with a capturing closure, lazy messages with `FnOnce`
//! - **Source:** Original · [Rust By Example: Wrapping errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/wrap_error.html) · [std `Error::source`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)
//!
//! ## Problem
//!
//! "invalid digit found in string" is a correct message, but it does not say **what** was being
//! parsed. Good programs add *context* while an error travels up: "invalid height `12px`" caused by
//! "invalid digit found in string". The context becomes the error's message, and the original
//! error stays available through `source()`.
//!
//! `ContextError` stores a context message and the original error as a `Box<dyn Error>`.
//! Implement:
//!
//! - `Display` for `ContextError`: just the context. `Error::source`: the boxed error.
//! - `with_context<T, E: Error + 'static>(result: Result<T, E>, context: &str) -> Result<T, ContextError>`:
//!   keep `Ok` values; wrap an `Err` into a `ContextError`.
//! - `with_context_lazy<T, E, F>(result: Result<T, E>, make: F) -> Result<T, ContextError>` where
//!   `F: FnOnce() -> String`: the same, but the message is built by calling `make` **only on error**.
//! - `parse_size(width: &str, height: &str) -> Result<(u32, u32), ContextError>`: parse both
//!   (trimmed) as `u32`. A bad width gets the context `"invalid width"`; a bad height gets
//!   ``"invalid height `HEIGHT`"`` (the untrimmed input), built lazily.
//! - `full_message(err: &ContextError) -> String`: `"CONTEXT: SOURCE MESSAGE"`.
//!
//! ## Examples
//!
//! ```text
//! parse_size("800", " 600 ")       -> Ok((800, 600))
//! parse_size("wide", "600")        -> Err: "invalid width"         source: "invalid digit found in string"
//! parse_size("800", "12px")        -> Err: "invalid height `12px`" source: "invalid digit found in string"
//! full_message(&that_last_error)   -> "invalid height `12px`: invalid digit found in string"
//! with_context(Ok::<i32, ParseIntError>(5), "never used") -> Ok(5)
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `result.map_err(|e| ContextError { context: context.to_string(), source: Box::new(e) })`.
//! 2. The `'static` bound on `E` is needed to put the error into a `Box<dyn Error>` (which means
//!    `Box<dyn Error + 'static>`).
//! 3. `fn source(&self) -> Option<&(dyn Error + 'static)> { Some(self.source.as_ref()) }`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `map_err` is the natural place to add context: its closure receives the original error
//! and can capture the message from the surrounding scope. Boxing the original error keeps it intact
//! for `source()` and lets one wrapper type hold any kind of cause.
//!
//! **Rust concepts:**
//! - Generic functions over the error type (`E: Error + 'static`) work for every error.
//! - `FnOnce() -> String` is the most permissive closure bound for something that is called at
//!   most once; `with_context_lazy` calls it only inside `map_err`, so successful results never pay
//!   for `format!`.
//! - `Box<dyn Error>` implements `Display`, so `err.source` can be formatted directly.
//! - `downcast_ref::<ParseIntError>()` on the source recovers the original typed error.
// @solution-end

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ContextError {
    pub context: String,
    pub source: Box<dyn Error>,
}

impl fmt::Display for ContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // @solution-start
        write!(f, "{}", self.context)
        // @solution-end
    }
}

impl Error for ContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // @solution-start
        Some(self.source.as_ref())
        // @solution-end
    }
}

pub fn with_context<T, E: Error + 'static>(
    result: Result<T, E>,
    context: &str,
) -> Result<T, ContextError> {
    // @solution-start
    result.map_err(|e| ContextError {
        context: context.to_string(),
        source: Box::new(e),
    })
    // @solution-end
}

pub fn with_context_lazy<T, E, F>(result: Result<T, E>, make: F) -> Result<T, ContextError>
where
    E: Error + 'static,
    F: FnOnce() -> String,
{
    // @solution-start
    result.map_err(|e| ContextError {
        context: make(),
        source: Box::new(e),
    })
    // @solution-end
}

pub fn parse_size(width: &str, height: &str) -> Result<(u32, u32), ContextError> {
    // @solution-start
    let w = with_context(width.trim().parse::<u32>(), "invalid width")?;
    let h = with_context_lazy(height.trim().parse::<u32>(), || {
        format!("invalid height `{height}`")
    })?;
    Ok((w, h))
    // @solution-end
}

pub fn full_message(err: &ContextError) -> String {
    // @solution-start
    format!("{}: {}", err, err.source)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::ParseIntError;

    #[test]
    fn ok_values_pass_through() {
        assert_eq!(
            with_context(Ok::<i32, ParseIntError>(5), "never used").unwrap(),
            5
        );
        let mut called = false;
        let r = with_context_lazy(Ok::<i32, ParseIntError>(7), || {
            called = true;
            "never used".to_string()
        });
        assert_eq!(r.unwrap(), 7);
        assert!(!called, "the message must only be built on error");
    }

    #[test]
    fn errors_get_context_and_keep_their_source() {
        let err = with_context("x".parse::<u8>(), "reading the level").unwrap_err();
        assert_eq!(err.to_string(), "reading the level");
        let source = err.source().expect("the original error is kept");
        assert_eq!(source.to_string(), "invalid digit found in string");
        assert!(source.downcast_ref::<ParseIntError>().is_some());
    }

    #[test]
    fn parse_size_valid() {
        assert_eq!(parse_size("800", " 600 ").unwrap(), (800, 600));
        assert_eq!(parse_size("0", "0").unwrap(), (0, 0));
    }

    #[test]
    fn parse_size_errors() {
        let err = parse_size("wide", "600").unwrap_err();
        assert_eq!(err.to_string(), "invalid width");
        assert_eq!(
            full_message(&err),
            "invalid width: invalid digit found in string"
        );

        let err = parse_size("800", "12px").unwrap_err();
        assert_eq!(err.to_string(), "invalid height `12px`");
        assert_eq!(
            full_message(&err),
            "invalid height `12px`: invalid digit found in string"
        );

        let err = parse_size("-1", "x").unwrap_err();
        assert_eq!(err.to_string(), "invalid width");
    }

    #[test]
    fn lazy_message_is_built_on_error() {
        let mut calls = 0;
        let err = with_context_lazy("".parse::<i64>(), || {
            calls += 1;
            format!("attempt {calls}")
        })
        .unwrap_err();
        assert_eq!(calls, 1);
        assert_eq!(
            full_message(&err),
            "attempt 1: cannot parse integer from empty string"
        );
    }
}
