//! # Problem 0419: Panic or Return an Error?
//!
//! - **Difficulty:** Easy
//! - **Topics:** `panic!`, `Result` for expected failures, validated newtype, `strip_suffix`, `#[should_panic(expected = ...)]`
//! - **Source:** Original · [The Rust Book 9.3: To `panic!` or Not to `panic!`](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)
//!
//! ## Problem
//!
//! Rust separates two kinds of failure:
//!
//! - **Bugs** (a broken promise between parts of *your* program): the code cannot continue
//!   meaningfully, so it calls `panic!` with a clear message.
//! - **Expected problems** (bad user input, a missing file...): the caller should decide what to
//!   do, so the function returns a `Result`.
//!
//! As in the Book's `Guess` example, build a `Percentage` type that can only hold values
//! `0..=100`:
//!
//! - `Percentage::new(value: u8) -> Percentage` is for values your own code computed. Passing a
//!   value above `100` is a bug: panic with the exact message
//!   `"percentage must be between 0 and 100, got VALUE"`.
//! - `Percentage::try_new(value: i64) -> Result<Percentage, String>` is for untrusted numbers:
//!   it returns the error `"VALUE is outside 0..=100"` instead of panicking.
//! - `Percentage::value(&self) -> u8` returns the stored value.
//! - `parse_percentage(input: &str) -> Result<Percentage, String>` reads what a user typed: trim
//!   the input, remove **one** optional trailing `'%'`, trim again, and parse an `i64`. A parse
//!   failure gives ``"`INPUT` is not a number"`` (`INPUT` = the trimmed input); a number out of range
//!   gives the `try_new` error.
//!
//! ## Examples
//!
//! ```text
//! Percentage::new(42).value()      -> 42
//! Percentage::new(150)             -> panics: "percentage must be between 0 and 100, got 150"
//! Percentage::try_new(-5)          -> Err("-5 is outside 0..=100")
//! parse_percentage(" 75% ")        -> Ok(Percentage 75)
//! parse_percentage("12 %")         -> Ok(Percentage 12)
//! parse_percentage("abc")          -> Err("`abc` is not a number")
//! parse_percentage("101%")         -> Err("101 is outside 0..=100")
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `panic!("... got {value}")` accepts a format string, like `format!`.
//! 2. `try_new` can check the range first and then build the value directly.
//! 3. `s.strip_suffix('%').unwrap_or(s)` removes one trailing `%` if there is one.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** keep the field private so that every `Percentage` is created through `new` or
//! `try_new`, which both check the range. After that, the rest of the program can trust any
//! `Percentage` it receives. `new` panics because a wrong value there is a programming error;
//! `try_new` and `parse_percentage` return `Result` because wrong input is normal.
//!
//! **Rust concepts:**
//! - A private field plus checking constructors is the standard "validated newtype" pattern.
//! - `panic!` unwinds the current thread; tests check the message with
//!   `#[should_panic(expected = "...")]`.
//! - `(0..=100).contains(&value)` is a readable range check.
//! - `strip_suffix` returns `Option<&str>`; `unwrap_or(s)` keeps the input when there is no `%`.
// @solution-end

#[derive(Debug, PartialEq)]
pub struct Percentage {
    // @solution-start remove
    value: u8,
    // @solution-end
}

impl Percentage {
    pub fn new(value: u8) -> Percentage {
        // @solution-start
        if value > 100 {
            panic!("percentage must be between 0 and 100, got {value}");
        }
        Percentage { value }
        // @solution-end
    }

    pub fn try_new(value: i64) -> Result<Percentage, String> {
        // @solution-start
        if (0..=100).contains(&value) {
            // The range check above guarantees that the value fits in a u8.
            Ok(Percentage { value: value as u8 })
        } else {
            Err(format!("{value} is outside 0..=100"))
        }
        // @solution-end
    }

    pub fn value(&self) -> u8 {
        // @solution-start
        self.value
        // @solution-end
    }
}

pub fn parse_percentage(input: &str) -> Result<Percentage, String> {
    // @solution-start
    let input = input.trim();
    let number = input.strip_suffix('%').unwrap_or(input).trim();
    let value = number
        .parse::<i64>()
        .map_err(|_| format!("`{input}` is not a number"))?;
    Percentage::try_new(value)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_the_whole_range() {
        assert_eq!(Percentage::new(0).value(), 0);
        assert_eq!(Percentage::new(42).value(), 42);
        assert_eq!(Percentage::new(100).value(), 100);
    }

    #[test]
    #[should_panic(expected = "percentage must be between 0 and 100, got 150")]
    fn new_panics_on_a_bug() {
        Percentage::new(150);
    }

    #[test]
    fn try_new_returns_errors() {
        assert_eq!(
            Percentage::try_new(-5),
            Err("-5 is outside 0..=100".to_string())
        );
        assert_eq!(
            Percentage::try_new(101),
            Err("101 is outside 0..=100".to_string())
        );
        assert_eq!(Percentage::try_new(100).map(|p| p.value()), Ok(100));
        assert_eq!(
            Percentage::try_new(1_000).map(|p| p.value()),
            Err("1000 is outside 0..=100".to_string())
        );
    }

    #[test]
    fn parse_user_input() {
        assert_eq!(parse_percentage(" 75% ").map(|p| p.value()), Ok(75));
        assert_eq!(parse_percentage("12 %").map(|p| p.value()), Ok(12));
        assert_eq!(parse_percentage("0").map(|p| p.value()), Ok(0));
    }

    #[test]
    fn parse_user_input_errors() {
        assert_eq!(
            parse_percentage("abc").map(|p| p.value()),
            Err("`abc` is not a number".to_string())
        );
        assert_eq!(
            parse_percentage("101%").map(|p| p.value()),
            Err("101 is outside 0..=100".to_string())
        );
        assert_eq!(
            parse_percentage("5%%").map(|p| p.value()),
            Err("`5%%` is not a number".to_string())
        );
        assert_eq!(
            parse_percentage("  ").map(|p| p.value()),
            Err("`` is not a number".to_string())
        );
    }
}
