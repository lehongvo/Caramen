// ==================================================================
// PRACTICE 0166: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0166_parse_a_key_value_pair.md
// Run the tests:          cargo test practice::p0166
// ==================================================================

//! # Problem 0166: Parse a Key=Value Pair
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split_once`, `trim`, returning `Option<(&str, &str)>` borrowed from the input
//! - **Source:** Original · [`str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once) · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! A configuration file contains lines such as `name = Ada`. Implement
//! `parse_pair(line: &str) -> Option<(&str, &str)>`:
//!
//! - split the line at the **first** `'='` (later `'='` characters belong to the value);
//! - trim whitespace around the key and around the value;
//! - return `Some((key, value))`, or `None` if the line has no `'='` or the key is empty after
//!   trimming. An empty value is allowed.
//!
//! Both returned strings must be slices of `line` (no allocation).
//!
//! ## Examples
//!
//! ```text
//! Input:  line = "name = Ada"
//! Output: Some(("name", "Ada"))
//!
//! Input:  line = "url=https://x.org/?a=b"
//! Output: Some(("url", "https://x.org/?a=b"))
//!
//! Input:  line = "flag ="
//! Output: Some(("flag", ""))
//!
//! Input:  line = " = orphan"
//! Output: None
//!
//! Input:  line = "no separator"
//! Output: None
//! ```
//!
//! ## Constraints
//!
//! - `0 <= line.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. `line.split_once('=')` returns `Some((before, after))` for the first `'='`, or `None`.
//! 2. `trim()` on each part returns sub-slices, still borrowing from `line`.

pub fn parse_pair(line: &str) -> Option<(&str, &str)> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_spaces_around() {
        assert_eq!(parse_pair("name = Ada"), Some(("name", "Ada")));
    }

    #[test]
    fn example_value_contains_equals() {
        assert_eq!(
            parse_pair("url=https://x.org/?a=b"),
            Some(("url", "https://x.org/?a=b"))
        );
    }

    #[test]
    fn example_empty_value() {
        assert_eq!(parse_pair("flag ="), Some(("flag", "")));
    }

    #[test]
    fn example_empty_key() {
        assert_eq!(parse_pair(" = orphan"), None);
    }

    #[test]
    fn example_no_separator() {
        assert_eq!(parse_pair("no separator"), None);
        assert_eq!(parse_pair(""), None);
    }

    #[test]
    fn inner_spaces_are_kept() {
        assert_eq!(
            parse_pair("  full name\t=  Ada  Lovelace "),
            Some(("full name", "Ada  Lovelace"))
        );
    }

    #[test]
    fn results_borrow_from_the_line() {
        let line = String::from("k=v");
        let (key, value) = parse_pair(&line).expect("valid pair");
        assert_eq!(key.as_ptr(), line.as_ptr());
        assert_eq!(value.as_ptr(), line[2..].as_ptr());
    }

    #[test]
    fn long_line() {
        let line = format!("{}={}", "k".repeat(5_000), "v".repeat(4_999));
        let (key, value) = parse_pair(&line).expect("valid pair");
        assert_eq!((key.len(), value.len()), (5_000, 4_999));
    }
}
