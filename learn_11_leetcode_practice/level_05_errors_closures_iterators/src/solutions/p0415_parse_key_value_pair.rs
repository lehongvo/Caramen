//! # Problem 0415: Parse a key=value Pair
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split_once`, `Option::ok_or_else`, `trim`, `?` in a loop, `str::lines`
//! - **Source:** Original · [std `str::split_once`](https://doc.rust-lang.org/std/primitive.str.html#method.split_once)
//!
//! ## Problem
//!
//! Configuration files often contain lines like `retries = 3`. Write a small parser that turns
//! such lines into `(String, i32)` pairs and explains precisely what is wrong with a bad line.
//!
//! Implement `parse_pair(line: &str) -> Result<(String, i32), String>`. Check these rules **in
//! this order** and return the first error:
//!
//! 1. The line must contain a `'='`; otherwise ``"missing `=` in `LINE`"``. Only the first `'='`
//!    splits the line.
//! 2. The key (text before `'='`, trimmed) must not be empty; otherwise ``"empty key in `LINE`"``.
//! 3. Every character of the key must be an ASCII letter, an ASCII digit or `'_'`; otherwise
//!    ``"invalid key `KEY`"``.
//! 4. The value (text after `'='`, trimmed) must parse as an `i32`; otherwise
//!    ``"invalid value `VALUE` for key `KEY`"``.
//!
//! In the messages, `LINE` is the line exactly as given, `KEY` and `VALUE` are trimmed.
//!
//! Then implement `parse_pairs(text: &str) -> Result<Vec<(String, i32)>, String>`: parse every
//! line of `text` that is not blank (blank = empty after trimming), in order, and stop at the first
//! error.
//!
//! ## Examples
//!
//! ```text
//! parse_pair("retries = 3")     -> Ok(("retries", 3))
//! parse_pair("x=-1")            -> Ok(("x", -1))
//! parse_pair("retries 3")       -> Err("missing `=` in `retries 3`")
//! parse_pair(" = 3")            -> Err("empty key in ` = 3`")
//! parse_pair("max-size = 3")    -> Err("invalid key `max-size`")
//! parse_pair("a = 1 = 2")       -> Err("invalid value `1 = 2` for key `a`")
//! parse_pairs("a=1\n\n b = 2 ") -> Ok([("a", 1), ("b", 2)])
//! ```
//!
//! ## Constraints
//!
//! - Lines have at most `200` characters; `text` has at most `10^3` lines.
//!
//! ## Hints
//!
//! 1. `line.split_once('=')` returns `Option<(&str, &str)>`; `ok_or_else` turns it into a `Result`
//!    and `?` unpacks it.
//! 2. `key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')` checks rule 3.
//! 3. `text.lines()` iterates over the lines; inside a `for` loop, `parse_pair(line)?` stops at the
//!    first bad line.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each rule is one fallible step, and each step either produces the next piece of data
//! or returns early with a precise message. `parse_pairs` reuses `parse_pair` and lets `?`
//! propagate the first error.
//!
//! **Rust concepts:**
//! - `split_once` splits at the first occurrence only, so `"a = 1 = 2"` has the value `"1 = 2"`.
//! - `ok_or_else(|| format!(...))` creates the message only on failure; `?` then returns it.
//! - Plain `if` + `return Err(...)` is perfectly idiomatic for checks that are not `Option`s.
//! - `str::lines()` handles both `"\n"` and `"\r\n"` line endings.
// @solution-end

pub fn parse_pair(line: &str) -> Result<(String, i32), String> {
    // @solution-start
    let (key, value) = line
        .split_once('=')
        .ok_or_else(|| format!("missing `=` in `{line}`"))?;
    let key = key.trim();
    let value = value.trim();
    if key.is_empty() {
        return Err(format!("empty key in `{line}`"));
    }
    if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(format!("invalid key `{key}`"));
    }
    let number = value
        .parse::<i32>()
        .map_err(|_| format!("invalid value `{value}` for key `{key}`"))?;
    Ok((key.to_string(), number))
    // @solution-end
}

pub fn parse_pairs(text: &str) -> Result<Vec<(String, i32)>, String> {
    // @solution-start
    let mut pairs = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        pairs.push(parse_pair(line)?);
    }
    Ok(pairs)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(k: &str, v: i32) -> (String, i32) {
        (k.to_string(), v)
    }

    #[test]
    fn valid_pairs() {
        assert_eq!(parse_pair("retries = 3"), Ok(pair("retries", 3)));
        assert_eq!(parse_pair("x=-1"), Ok(pair("x", -1)));
        assert_eq!(parse_pair("  MAX_2 =  +7  "), Ok(pair("MAX_2", 7)));
    }

    #[test]
    fn structural_errors() {
        assert_eq!(
            parse_pair("retries 3"),
            Err("missing `=` in `retries 3`".to_string())
        );
        assert_eq!(parse_pair(" = 3"), Err("empty key in ` = 3`".to_string()));
        assert_eq!(
            parse_pair("max-size = 3"),
            Err("invalid key `max-size`".to_string())
        );
    }

    #[test]
    fn value_errors() {
        assert_eq!(
            parse_pair("a = 1 = 2"),
            Err("invalid value `1 = 2` for key `a`".to_string())
        );
        assert_eq!(
            parse_pair("a ="),
            Err("invalid value `` for key `a`".to_string())
        );
        assert_eq!(
            parse_pair("big = 3000000000"),
            Err("invalid value `3000000000` for key `big`".to_string())
        );
    }

    #[test]
    fn rules_are_checked_in_order() {
        // Empty key is reported before the invalid value.
        assert_eq!(parse_pair("=x"), Err("empty key in `=x`".to_string()));
        // Invalid key is reported before the invalid value.
        assert_eq!(parse_pair("a b = x"), Err("invalid key `a b`".to_string()));
    }

    #[test]
    fn parse_pairs_skips_blank_lines_and_stops_at_errors() {
        assert_eq!(
            parse_pairs("a=1\n\n b = 2 "),
            Ok(vec![pair("a", 1), pair("b", 2)])
        );
        assert_eq!(parse_pairs("   \n"), Ok(vec![]));
        assert_eq!(
            parse_pairs("a=1\nb:2\nc=x"),
            Err("missing `=` in `b:2`".to_string())
        );
    }
}
