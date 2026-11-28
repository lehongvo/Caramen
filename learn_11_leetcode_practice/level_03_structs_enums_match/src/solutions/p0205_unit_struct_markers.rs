//! # Problem 0205: Unit Structs as Zero-Sized Tools
//!
//! - **Difficulty:** Easy
//! - **Topics:** unit-like structs, methods on a struct without fields, `std::mem::size_of`
//! - **Source:** Original · [The Rust Book 5.1](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) · [`size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html)
//!
//! ## Problem
//!
//! A *unit-like struct* has no fields at all: `pub struct Shout;`. Its name is both the type and the
//! only value of that type, and it occupies **zero bytes**. Unit structs are still useful: you can
//! give them methods, and later (level 6) implement traits for them.
//!
//! This problem uses three unit structs and one ordinary struct:
//!
//! ```text
//! pub struct Point { pub x: i32, pub y: i32 }
//! pub struct Origin;    // the point (0, 0)
//! pub struct Shout;     // a text formatter
//! pub struct Whisper;   // another text formatter
//! ```
//!
//! Implement these methods:
//!
//! - `Origin::as_point(&self) -> Point` returns `Point { x: 0, y: 0 }`.
//! - `Origin::manhattan_to(&self, p: Point) -> i32` returns `|p.x| + |p.y|`.
//! - `Shout::apply(&self, text: &str) -> String` returns `text` in uppercase followed by `"!"`.
//! - `Whisper::apply(&self, text: &str) -> String` returns `text` in lowercase followed by `"..."`.
//!
//! A method on a unit struct is called on the value itself: `Shout.apply("hi")`.
//!
//! ## Examples
//!
//! ```text
//! Origin.as_point()                       => Point { x: 0, y: 0 }
//! Origin.manhattan_to(Point { x: 3, y: -4 }) => 7
//! Shout.apply("hello")                    => "HELLO!"
//! Whisper.apply("Quiet Please")           => "quiet please..."
//! ```
//!
//! ## Constraints
//!
//! - `-10^6 <= x, y <= 10^6`; `text` is ASCII or any UTF-8 (use the full Unicode case mapping).
//!
//! ## Hints
//!
//! 1. `i32::abs` gives the absolute value: `p.x.abs()`.
//! 2. `str::to_uppercase` and `str::to_lowercase` return a new `String`; `format!` or `push_str`
//!    can then append the suffix.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each method only uses its arguments; the `&self` of a unit struct carries no data.
//! The two formatters change case and append a suffix.
//!
//! **Rust concepts:**
//! - `pub struct Origin;` declares a type with exactly one value, also written `Origin`.
//! - `size_of::<Origin>() == 0`: a unit struct needs no memory, so it is free to pass around.
//! - Methods on unit structs group related behaviour under a name. `Shout` and `Whisper` both have an
//!   `apply` method; there is no conflict because each lives in its own `impl` block.
//! - `#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]` works on unit structs too.
// @solution-end

/// A point on a grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// The origin `(0, 0)` as a zero-sized value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Origin;

/// Formats text loudly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Shout;

/// Formats text quietly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Whisper;

impl Origin {
    /// The origin as an ordinary point.
    pub fn as_point(&self) -> Point {
        // @solution-start
        Point { x: 0, y: 0 }
        // @solution-end
    }

    /// Manhattan distance from the origin to `p`.
    pub fn manhattan_to(&self, p: Point) -> i32 {
        // @solution-start
        p.x.abs() + p.y.abs()
        // @solution-end
    }
}

impl Shout {
    /// Uppercase plus `"!"`.
    pub fn apply(&self, text: &str) -> String {
        // @solution-start
        format!("{}!", text.to_uppercase())
        // @solution-end
    }
}

impl Whisper {
    /// Lowercase plus `"..."`.
    pub fn apply(&self, text: &str) -> String {
        // @solution-start
        format!("{}...", text.to_lowercase())
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn example_origin_as_point() {
        assert_eq!(Origin.as_point(), Point { x: 0, y: 0 });
    }

    #[test]
    fn example_manhattan_distance() {
        assert_eq!(Origin.manhattan_to(Point { x: 3, y: -4 }), 7);
    }

    #[test]
    fn example_shout_and_whisper() {
        assert_eq!(Shout.apply("hello"), "HELLO!");
        assert_eq!(Whisper.apply("Quiet Please"), "quiet please...");
    }

    #[test]
    fn distance_to_origin_itself_is_zero() {
        let origin = Origin;
        assert_eq!(origin.manhattan_to(origin.as_point()), 0);
    }

    #[test]
    fn empty_and_unicode_text() {
        assert_eq!(Shout.apply(""), "!");
        assert_eq!(Whisper.apply("ÄPFEL"), "äpfel...");
    }

    #[test]
    fn unit_structs_take_no_space_and_still_work() {
        assert_eq!(size_of::<Origin>(), 0);
        assert_eq!(size_of::<Shout>(), 0);
        let tools = [Shout, Shout];
        let out: Vec<String> = tools.iter().map(|t| t.apply("go")).collect();
        assert_eq!(out, ["GO!", "GO!"]);
    }

    #[test]
    fn large_coordinates() {
        assert_eq!(
            Origin.manhattan_to(Point {
                x: -1_000_000,
                y: 1_000_000
            }),
            2_000_000
        );
    }
}
