//! # Problem 0543: Validated Values with `TryFrom`
//!
//! - **Difficulty:** Easy
//! - **Topics:** `TryFrom` with an associated `Error` type, `try_into()`, private fields to protect invariants, collecting `Result`s
//! - **Source:** Original · [std::convert::TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
//!
//! ## Problem
//!
//! Some conversions can fail. `TryFrom<T>` is the fallible version of `From<T>`; it has an associated
//! type `Error` that describes the failure.
//!
//! 1. `Percentage` wraps a **private** `u8` that is always in `0..=100`; `value(self) -> u8` reads it.
//!    Implement `TryFrom<i32> for Percentage` with `type Error = PercentageError`, where
//!    `PercentageError` is `Negative(i32)` or `TooLarge(i32)` (carrying the rejected input).
//! 2. `Weekday` is `Mon, Tue, Wed, Thu, Fri, Sat, Sun`. Implement `TryFrom<u8> for Weekday` with
//!    `type Error = String`: `1` is `Mon`, ..., `7` is `Sun`; any other number fails with the message
//!    `"{n} is not a weekday number (1-7)"`. Also implement `From<Weekday> for u8` (the reverse,
//!    which cannot fail).
//! 3. `parse_percentages(values: &[i32]) -> Result<Vec<Percentage>, PercentageError>` converts every
//!    value and stops at the **first** error.
//!
//! Because `Percentage`'s field is private, `TryFrom` is the only way to build one outside this module,
//! so an invalid percentage cannot exist.
//!
//! ## Examples
//!
//! ```text
//! Percentage::try_from(42).map(Percentage::value)    => Ok(42)
//! Percentage::try_from(-5)                           => Err(Negative(-5))
//! let p: Result<Percentage, _> = 250.try_into();     => Err(TooLarge(250))
//! Weekday::try_from(6)                               => Ok(Sat)
//! Weekday::try_from(0)                               => Err("0 is not a weekday number (1-7)")
//! u8::from(Weekday::Sun)                             => 7
//! parse_percentages(&[10, 120, -1])                  => Err(TooLarge(120))
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `impl TryFrom<i32> for Percentage { type Error = PercentageError; fn try_from(v: i32) ->
//!    Result<Self, Self::Error> { ... } }`.
//! 2. Implementing `TryFrom` also gives `TryInto` for free, just like `From` gives `Into`.
//! 3. An iterator of `Result<T, E>` can be collected into `Result<Vec<T>, E>`; collection stops at the
//!    first `Err`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** validate the input with a `match` on ranges and only build the value when it is valid.
//! For weekdays, a lookup table maps `1..=7` to the variants; the reverse conversion is the enum's
//! position plus one.
//!
//! **Rust concepts:**
//! - The associated type `Error` lets each `TryFrom` impl choose its own error type: a dedicated enum
//!   for `Percentage`, a `String` for `Weekday`.
//! - `x.try_into()` needs the target type from context (`let p: Result<Percentage, _> = ...`), exactly
//!   like `into()`.
//! - Private fields plus a validating constructor are the standard way to encode invariants in types
//!   ("parse, don't validate").
// @solution-end

/// A whole percentage in `0..=100`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percentage(u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PercentageError {
    Negative(i32),
    TooLarge(i32),
}

impl Percentage {
    pub fn value(self) -> u8 {
        // @solution-start
        self.0
        // @solution-end
    }
}

impl TryFrom<i32> for Percentage {
    type Error = PercentageError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        // @solution-start
        match value {
            v if v < 0 => Err(PercentageError::Negative(v)),
            v @ 0..=100 => Ok(Percentage(v as u8)),
            v => Err(PercentageError::TooLarge(v)),
        }
        // @solution-end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

// @solution-start remove
const WEEK: [Weekday; 7] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];
// @solution-end

impl TryFrom<u8> for Weekday {
    type Error = String;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        // @solution-start
        match n {
            1..=7 => Ok(WEEK[usize::from(n) - 1]),
            _ => Err(format!("{n} is not a weekday number (1-7)")),
        }
        // @solution-end
    }
}

impl From<Weekday> for u8 {
    fn from(day: Weekday) -> u8 {
        // @solution-start
        day as u8 + 1
        // @solution-end
    }
}

/// Converts every value, stopping at the first error.
pub fn parse_percentages(values: &[i32]) -> Result<Vec<Percentage>, PercentageError> {
    // @solution-start
    values.iter().map(|&v| Percentage::try_from(v)).collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentages() {
        assert_eq!(Percentage::try_from(42).map(Percentage::value), Ok(42));
        assert_eq!(Percentage::try_from(0).map(Percentage::value), Ok(0));
        assert_eq!(Percentage::try_from(100).map(Percentage::value), Ok(100));
        assert_eq!(Percentage::try_from(-5), Err(PercentageError::Negative(-5)));
        let p: Result<Percentage, _> = 250.try_into();
        assert_eq!(p, Err(PercentageError::TooLarge(250)));
    }

    #[test]
    fn weekdays() {
        assert_eq!(Weekday::try_from(1), Ok(Weekday::Mon));
        assert_eq!(Weekday::try_from(6), Ok(Weekday::Sat));
        assert_eq!(
            Weekday::try_from(0),
            Err("0 is not a weekday number (1-7)".to_string())
        );
        assert_eq!(
            Weekday::try_from(8),
            Err("8 is not a weekday number (1-7)".to_string())
        );
        assert_eq!(u8::from(Weekday::Sun), 7);
        let n: u8 = Weekday::Wed.into();
        assert_eq!(n, 3);
    }

    #[test]
    fn weekday_round_trip() {
        for n in 1..=7_u8 {
            let day: Weekday = n.try_into().unwrap();
            assert_eq!(u8::from(day), n);
        }
    }

    #[test]
    fn collecting_results() {
        assert_eq!(
            parse_percentages(&[10, 120, -1]),
            Err(PercentageError::TooLarge(120))
        );
        assert_eq!(
            parse_percentages(&[-1, 120]),
            Err(PercentageError::Negative(-1))
        );
        let ok = parse_percentages(&[0, 50, 100]).unwrap();
        assert_eq!(
            ok.iter().map(|p| p.value()).collect::<Vec<_>>(),
            vec![0, 50, 100]
        );
        assert_eq!(parse_percentages(&[]), Ok(vec![]));
    }
}
