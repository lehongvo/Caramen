// ==================================================================
// PRACTICE 0426: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0426_implement_from_str_point.md
// Run the tests:          cargo test practice::p0426
// ==================================================================

//! # Problem 0426: Implement FromStr for a Point
//!
//! - **Difficulty:** Easy
//! - **Topics:** `std::str::FromStr`, associated type `Err`, `str::parse::<Point>()`, `From`, `Display` round trip, `windows`
//! - **Source:** Original · [std `FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) · [Rust By Example: From and Into strings](https://doc.rust-lang.org/rust-by-example/conversion/string.html)
//!
//! ## Problem
//!
//! `"42".parse::<i32>()` works because `i32` implements the `FromStr` trait. Implement `FromStr`
//! for your own type and `.parse::<YourType>()` works too.
//!
//! Points are written `(x, y)`, for example `(3, -4)`. Implement:
//!
//! - `impl FromStr for Point` with `type Err = ParsePointError`. Trim the input; it must start with
//!   `'('` and end with `')'` (otherwise `MissingParens`); the inside must contain a `','`
//!   (otherwise `MissingComma`); both coordinates (trimmed) must parse as `i32` (otherwise
//!   `BadCoordinate(the ParseIntError)`, `x` checked first).
//! - `impl From<ParseIntError> for ParsePointError` so that `?` converts parse errors.
//! - `Display` for `Point`, writing `(x, y)` with one space after the comma, so that
//!   `p.to_string().parse::<Point>() == Ok(p)`.
//! - `Display` for `ParsePointError`: `"expected parentheses around the point"`,
//!   `"expected a comma between the coordinates"`, `"bad coordinate: E"` (the `ParseIntError` message).
//! - `parse_path(s: &str) -> Result<Vec<Point>, ParsePointError>`: points separated by `';'`. A
//!   blank string is an empty path.
//! - `path_length(s: &str) -> Result<i64, ParsePointError>`: the sum of the Manhattan distances
//!   between consecutive points of the path (`0` for fewer than two points).
//!
//! ## Examples
//!
//! ```text
//! "(3, -4)".parse::<Point>()     -> Ok(Point { x: 3, y: -4 })
//! " ( 3 ,4 ) ".parse::<Point>()  -> Ok(Point { x: 3, y: 4 })
//! "3, 4".parse::<Point>()        -> Err(MissingParens)
//! "(3 4)".parse::<Point>()       -> Err(MissingComma)
//! "(3, four)".parse::<Point>()   -> Err(BadCoordinate(InvalidDigit))
//! Point { x: 1, y: 2 }.to_string() -> "(1, 2)"
//! parse_path("(0,0); (1,2)")     -> Ok([Point { x: 0, y: 0 }, Point { x: 1, y: 2 }])
//! path_length("(0,0);(1,2);(1,-1)") -> Ok(6)          (3 + 3)
//! ```
//!
//! ## Constraints
//!
//! - Paths have at most `10^4` points.
//!
//! ## Hints
//!
//! 1. `fn from_str(s: &str) -> Result<Self, Self::Err>` is the one method of `FromStr`.
//! 2. `strip_prefix('(')` and `strip_suffix(')')` chain with `and_then`.
//! 3. In `parse_path`, `s.split(';').map(str::parse).collect()` works once the target type is known.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub enum ParsePointError {
    MissingParens,
    MissingComma,
    BadCoordinate(ParseIntError),
}

impl From<ParseIntError> for ParsePointError {
    fn from(err: ParseIntError) -> Self {
        todo!("Write your solution here")
    }
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for ParsePointError {}

pub fn parse_path(s: &str) -> Result<Vec<Point>, ParsePointError> {
    todo!("Write your solution here")
}

pub fn path_length(s: &str) -> Result<i64, ParsePointError> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    fn p(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    #[test]
    fn parse_valid_points() {
        assert_eq!("(3, -4)".parse::<Point>(), Ok(p(3, -4)));
        assert_eq!(" ( 3 ,4 ) ".parse::<Point>(), Ok(p(3, 4)));
        assert_eq!("(0,0)".parse::<Point>(), Ok(p(0, 0)));
    }

    #[test]
    fn parse_errors() {
        assert_eq!("3, 4".parse::<Point>(), Err(ParsePointError::MissingParens));
        assert_eq!(
            "(3, 4".parse::<Point>(),
            Err(ParsePointError::MissingParens)
        );
        assert_eq!("(3 4)".parse::<Point>(), Err(ParsePointError::MissingComma));
        match "(3, four)".parse::<Point>() {
            Err(ParsePointError::BadCoordinate(e)) => {
                assert_eq!(e.kind(), &IntErrorKind::InvalidDigit)
            }
            other => panic!("unexpected result {other:?}"),
        }
        match "(, 4)".parse::<Point>() {
            Err(ParsePointError::BadCoordinate(e)) => assert_eq!(e.kind(), &IntErrorKind::Empty),
            other => panic!("unexpected result {other:?}"),
        }
    }

    #[test]
    fn display_round_trip() {
        assert_eq!(p(1, 2).to_string(), "(1, 2)");
        for point in [p(0, 0), p(-7, 9), p(i32::MAX, i32::MIN)] {
            assert_eq!(point.to_string().parse::<Point>(), Ok(point));
        }
    }

    #[test]
    fn error_messages() {
        assert_eq!(
            ParsePointError::MissingParens.to_string(),
            "expected parentheses around the point"
        );
        assert_eq!(
            ParsePointError::MissingComma.to_string(),
            "expected a comma between the coordinates"
        );
        let err = "(1, x)".parse::<Point>().unwrap_err();
        assert_eq!(
            err.to_string(),
            "bad coordinate: invalid digit found in string"
        );
    }

    #[test]
    fn paths() {
        assert_eq!(parse_path("(0,0); (1,2)"), Ok(vec![p(0, 0), p(1, 2)]));
        assert_eq!(parse_path("  "), Ok(vec![]));
        assert_eq!(parse_path("(0,0);"), Err(ParsePointError::MissingParens));
        assert_eq!(path_length("(0,0);(1,2);(1,-1)"), Ok(6));
        assert_eq!(path_length("(5,5)"), Ok(0));
        assert_eq!(
            path_length("(-2147483648,0);(2147483647,0)"),
            Ok(4_294_967_295)
        );
    }
}
