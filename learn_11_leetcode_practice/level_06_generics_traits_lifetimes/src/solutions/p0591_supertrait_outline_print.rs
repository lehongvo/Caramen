//! # Problem 0591: Supertraits: Outline Printing
//!
//! - **Difficulty:** Easy
//! - **Topics:** supertraits (`trait OutlinePrint: fmt::Display`), default methods that use the supertrait, implementing a trait with an empty impl block
//! - **Source:** Original · [The Rust Book 20.2](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait)
//!
//! ## Problem
//!
//! `trait OutlinePrint: fmt::Display` means "only types that implement `Display` may implement
//! `OutlinePrint`", and in return its default methods may call `self.to_string()`.
//!
//! Implement the trait with one default method `outline(&self) -> String` that draws the `Display`
//! text in a frame of asterisks (lines joined with `'\n'`, no trailing newline). For a text of width
//! `w` characters the frame is `w + 4` characters wide:
//!
//! ```text
//! **********
//! *        *
//! * (1, 3) *
//! *        *
//! **********
//! ```
//!
//! Then:
//!
//! - `Point { x: i32, y: i32 }`: implement `Display` as `"(x, y)"` and `OutlinePrint` with an **empty**
//!   impl block (it uses the default).
//! - `Title(pub String)`: implement `Display` (the string itself) and `OutlinePrint` (empty impl).
//! - `outline_all<T: OutlinePrint>(items: &[T]) -> String`: the outlines separated by a blank line
//!   (`"\n\n"`).
//!
//! ## Examples
//!
//! ```text
//! Point { x: 1, y: 3 }.outline()      => the frame above
//! Title("Hi".into()).outline()        => "******\n*    *\n* Hi *\n*    *\n******"
//! ```
//!
//! ## Constraints
//!
//! - The text is a single line; width is measured in `char`s.
//!
//! ## Hints
//!
//! 1. `let text = self.to_string();` works inside the default method thanks to the supertrait bound.
//! 2. `"*".repeat(n)` and `format!("* {text} *")` build the lines.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the default method formats the value once with `to_string` (available because `Display`
//! is a supertrait) and builds the five lines around it.
//!
//! **Rust concepts:**
//! - A supertrait is a requirement on implementors *and* a capability for the trait's own methods:
//!   without `: fmt::Display`, `self.to_string()` would not compile inside `outline`.
//! - `impl OutlinePrint for Point {}` with no items accepts every default; trying the same for a type
//!   without `Display` fails with "`Point` doesn't implement `std::fmt::Display`".
//! - Generic code bounded by `T: OutlinePrint` may also use `Display` on `T`, because the supertrait
//!   is implied.
// @solution-end

use std::fmt;

/// Draws the `Display` form of a value inside a frame of asterisks.
pub trait OutlinePrint: fmt::Display {
    fn outline(&self) -> String {
        // @solution-start
        let text = self.to_string();
        let width = text.chars().count();
        let border = "*".repeat(width + 4);
        let blank = format!("*{}*", " ".repeat(width + 2));
        [
            border.clone(),
            blank.clone(),
            format!("* {text} *"),
            blank,
            border,
        ]
        .join("\n")
        // @solution-end
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Title(pub String);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // @solution-start
        write!(f, "({}, {})", self.x, self.y)
        // @solution-end
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // @solution-start
        f.write_str(&self.0)
        // @solution-end
    }
}

impl OutlinePrint for Point {}

impl OutlinePrint for Title {}

/// All outlines, separated by a blank line.
pub fn outline_all<T: OutlinePrint>(items: &[T]) -> String {
    // @solution-start
    items
        .iter()
        .map(|item| item.outline())
        .collect::<Vec<_>>()
        .join("\n\n")
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_outline() {
        let expected = "**********\n*        *\n* (1, 3) *\n*        *\n**********";
        assert_eq!(Point { x: 1, y: 3 }.outline(), expected);
        assert_eq!(Point { x: 1, y: 3 }.to_string(), "(1, 3)");
    }

    #[test]
    fn title_outline() {
        assert_eq!(
            Title("Hi".into()).outline(),
            "******\n*    *\n* Hi *\n*    *\n******"
        );
        assert_eq!(
            Title(String::new()).outline(),
            "****\n*  *\n*  *\n*  *\n****"
        );
        let accented = Title("caf\u{e9}".into()).outline();
        assert_eq!(accented.lines().next().unwrap().chars().count(), 8);
    }

    #[test]
    fn outlining_several_items() {
        let points = [Point { x: 0, y: 0 }, Point { x: -5, y: 10 }];
        let all = outline_all(&points);
        assert_eq!(all.matches("* (0, 0) *").count(), 1);
        assert!(all.contains("**********\n\n************"));
        assert_eq!(outline_all::<Title>(&[]), "");
    }
}
