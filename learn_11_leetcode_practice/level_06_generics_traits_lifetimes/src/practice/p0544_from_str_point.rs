// ==================================================================
// PRACTICE 0544: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0544_from_str_point.md
// Run the tests:          cargo test practice::p0544
// ==================================================================

//! # Problem 0544: Parsing a Point with `FromStr`
//!
//! - **Difficulty:** Easy
//! - **Topics:** implementing `FromStr` with a custom error enum, `str::parse::<T>()`, `strip_prefix`/`strip_suffix`, `Display` round trip
//! - **Source:** Original · [std::str::FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)
//!
//! ## Problem
//!
//! Implement `FromStr` for `Point { x: i64, y: i64 }` so that `"(3, -4)".parse::<Point>()` works.
//!
//! The accepted format, after trimming surrounding whitespace, is `(` + `x` + `,` + `y` + `)`, where
//! whitespace around each number is allowed. Errors are reported with
//! `ParsePointError`:
//!
//! - `MissingParens`: the trimmed text does not start with `(` and end with `)`.
//! - `WrongArity(n)`: the inside does not split into exactly 2 comma-separated parts (`n` is the
//!   number of parts found; an empty inside counts as 1 part).
//! - `BadNumber(text)`: a part is not a valid `i64`; `text` is the trimmed part.
//!
//! Also implement `Display` as `"(x, y)"`, so that formatting and parsing round-trip, and write
//! `parse_path(text: &str) -> Result<Vec<Point>, ParsePointError>` which parses points separated by
//! `;` (empty pieces, e.g. after a trailing `;`, are skipped).
//!
//! ## Examples
//!
//! ```text
//! "(3, -4)".parse::<Point>()       => Ok(Point { x: 3, y: -4 })
//! "  ( 10 ,20 ) ".parse::<Point>() => Ok(Point { x: 10, y: 20 })
//! "3, 4".parse::<Point>()          => Err(MissingParens)
//! "(1, 2, 3)".parse::<Point>()     => Err(WrongArity(3))
//! "(1, two)".parse::<Point>()      => Err(BadNumber("two"))
//! Point { x: 1, y: 2 }.to_string() => "(1, 2)"
//! parse_path("(0,0); (1, 2);")     => Ok([Point { x: 0, y: 0 }, Point { x: 1, y: 2 }])
//! ```
//!
//! ## Constraints
//!
//! - Coordinates fit in `i64`.
//!
//! ## Hints
//!
//! 1. `impl FromStr for Point { type Err = ParsePointError; fn from_str(s: &str) -> Result<Self,
//!    Self::Err> { ... } }`. Once it exists, `s.parse::<Point>()` calls it.
//! 2. `s.trim().strip_prefix('(').and_then(|r| r.strip_suffix(')'))` returns the inside, or `None`.
//! 3. Convert a `ParseIntError` into your own error with `map_err`.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsePointError {
    MissingParens,
    WrongArity(usize),
    BadNumber(String),
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

/// Parses `;`-separated points, skipping empty pieces.
pub fn parse_path(text: &str) -> Result<Vec<Point>, ParsePointError> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_points() {
        assert_eq!("(3, -4)".parse::<Point>(), Ok(Point { x: 3, y: -4 }));
        assert_eq!("  ( 10 ,20 ) ".parse::<Point>(), Ok(Point { x: 10, y: 20 }));
        let p: Point = "(0,0)".parse().unwrap();
        assert_eq!(p, Point { x: 0, y: 0 });
    }

    #[test]
    fn reports_errors() {
        assert_eq!("3, 4".parse::<Point>(), Err(ParsePointError::MissingParens));
        assert_eq!(
            "(3, 4".parse::<Point>(),
            Err(ParsePointError::MissingParens)
        );
        assert_eq!(
            "(1, 2, 3)".parse::<Point>(),
            Err(ParsePointError::WrongArity(3))
        );
        assert_eq!("()".parse::<Point>(), Err(ParsePointError::WrongArity(1)));
        assert_eq!(
            "(1, two)".parse::<Point>(),
            Err(ParsePointError::BadNumber("two".to_string()))
        );
        assert_eq!(
            "(, 5)".parse::<Point>(),
            Err(ParsePointError::BadNumber(String::new()))
        );
    }

    #[test]
    fn display_round_trip() {
        assert_eq!(Point { x: 1, y: 2 }.to_string(), "(1, 2)");
        for p in [
            Point { x: -7, y: 0 },
            Point {
                x: i64::MAX,
                y: i64::MIN,
            },
        ] {
            assert_eq!(p.to_string().parse::<Point>(), Ok(p));
        }
    }

    #[test]
    fn paths() {
        assert_eq!(
            parse_path("(0,0); (1, 2);"),
            Ok(vec![Point { x: 0, y: 0 }, Point { x: 1, y: 2 }])
        );
        assert_eq!(parse_path(""), Ok(vec![]));
        assert_eq!(
            parse_path("(1,1);oops"),
            Err(ParsePointError::MissingParens)
        );
        let long: String = (0..1000).map(|i| format!("({i}, {});", -i)).collect();
        let points = parse_path(&long).unwrap();
        assert_eq!(points.len(), 1000);
        assert_eq!(points[999], Point { x: 999, y: -999 });
    }
}
