//! # Problem 0461: Read a Header Block with skip_while and take_while
//!
//! - **Difficulty:** Easy
//! - **Topics:** `skip_while`, `take_while`, `map_while`, `skip`, `str::lines`, `split_once`
//! - **Source:** Original · [std `Iterator::skip_while`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.skip_while) · [std `Iterator::map_while`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map_while)
//!
//! ## Problem
//!
//! Emails and HTTP messages start with a block of `Name: value` header lines, then one blank line,
//! then the body:
//!
//! ```text
//!
//! From: ann@example.com
//! Subject: lunch?
//!
//! Are you free at noon?
//! See you.
//! ```
//!
//! A line is *blank* when it is empty after trimming. Implement:
//!
//! - `header_lines(text: &str) -> Vec<&str>`: skip the blank lines at the very beginning, then
//!   return the lines up to (not including) the next blank line.
//! - `body_lines(text: &str) -> Vec<&str>`: everything after the blank line that ends the header
//!   block, unchanged (it may contain blank lines). Empty if there is no such blank line.
//! - `parse_headers(text: &str) -> Vec<(String, String)>`: the header lines split at the first
//!   `':'` into a trimmed name and a trimmed value. Stop at the first header line without a `':'`
//!   (that line and everything after it are ignored).
//! - `leading_number(s: &str) -> Option<u32>`: the number formed by the ASCII digits at the
//!   beginning of `s` (`"42abc"` -> `42`); `None` if `s` does not start with a digit or the number
//!   does not fit in a `u32`.
//!
//! ## Examples
//!
//! ```text
//! (text above)
//! header_lines   -> ["From: ann@example.com", "Subject: lunch?"]
//! body_lines     -> ["Are you free at noon?", "See you."]
//! parse_headers  -> [("From", "ann@example.com"), ("Subject", "lunch?")]
//! parse_headers("A: 1\nbroken\nB: 2")  -> [("A", "1")]
//! leading_number("42abc")  -> Some(42)
//! leading_number("abc42")  -> None
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` lines.
//!
//! ## Hints
//!
//! 1. `skip_while(pred)` drops items while `pred` is true, then yields **everything** else;
//!    `take_while(pred)` yields items while `pred` is true and then stops for good.
//! 2. For the body: skip leading blanks, skip the header lines, then `skip(1)` for the separator.
//! 3. `map_while(f)` is like `map`, but stops at the first item for which `f` returns `None`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** describe the structure of the text as a sequence of "while" phases. The header is
//! "after the leading blanks, while not blank"; the body is "after the leading blanks, after the
//! non-blank lines, after one separator". `map_while` both parses and stops at the first bad header.
//!
//! **Rust concepts:**
//! - `skip_while` checks its predicate only until it first returns `false`; later items are never
//!   tested again. `take_while` stops at the first `false` and never yields that item.
//! - `map_while` (stable since Rust 1.57) combines `map` and `take_while` for `Option`-returning
//!   closures, which is perfect for "parse as long as it parses".
//! - All functions return slices of the original text (`&str`); nothing is copied except in
//!   `parse_headers`, which returns owned `String`s.
//! - `leading_number` collects the digit prefix into a `String` and lets `parse` handle overflow.
// @solution-end

// @solution-start remove
fn is_blank(line: &str) -> bool {
    line.trim().is_empty()
}
// @solution-end

pub fn header_lines(text: &str) -> Vec<&str> {
    // @solution-start
    text.lines()
        .skip_while(|l| is_blank(l))
        .take_while(|l| !is_blank(l))
        .collect()
    // @solution-end
}

pub fn body_lines(text: &str) -> Vec<&str> {
    // @solution-start
    text.lines()
        .skip_while(|l| is_blank(l))
        .skip_while(|l| !is_blank(l))
        .skip(1)
        .collect()
    // @solution-end
}

pub fn parse_headers(text: &str) -> Vec<(String, String)> {
    // @solution-start
    header_lines(text)
        .into_iter()
        .map_while(|line| line.split_once(':'))
        .map(|(name, value)| (name.trim().to_string(), value.trim().to_string()))
        .collect()
    // @solution-end
}

pub fn leading_number(s: &str) -> Option<u32> {
    // @solution-start
    s.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .ok()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE: &str =
        "\n  \nFrom: ann@example.com\nSubject: lunch?\n\nAre you free at noon?\nSee you.\n";

    fn pair(a: &str, b: &str) -> (String, String) {
        (a.to_string(), b.to_string())
    }

    #[test]
    fn header_block() {
        assert_eq!(
            header_lines(MESSAGE),
            vec!["From: ann@example.com", "Subject: lunch?"]
        );
        assert_eq!(header_lines("only: header"), vec!["only: header"]);
        assert_eq!(header_lines("\n\n"), Vec::<&str>::new());
    }

    #[test]
    fn body_block() {
        assert_eq!(
            body_lines(MESSAGE),
            vec!["Are you free at noon?", "See you."]
        );
        assert_eq!(body_lines("A: 1\n\nx\n\ny"), vec!["x", "", "y"]);
        assert_eq!(body_lines("A: 1"), Vec::<&str>::new());
    }

    #[test]
    fn parse_header_pairs() {
        assert_eq!(
            parse_headers(MESSAGE),
            vec![pair("From", "ann@example.com"), pair("Subject", "lunch?")]
        );
        assert_eq!(parse_headers("A: 1\nbroken\nB: 2"), vec![pair("A", "1")]);
        assert_eq!(parse_headers("Time: 12:30"), vec![pair("Time", "12:30")]);
    }

    #[test]
    fn leading_numbers() {
        assert_eq!(leading_number("42abc"), Some(42));
        assert_eq!(leading_number("abc42"), None);
        assert_eq!(leading_number("007"), Some(7));
        assert_eq!(leading_number(""), None);
        assert_eq!(leading_number("4294967296x"), None);
        assert_eq!(leading_number("12 34"), Some(12));
    }
}
