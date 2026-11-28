// ==================================================================
// PRACTICE 0434: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0434_parse_ini_config.md
// Run the tests:          cargo test practice::p0434
// ==================================================================

//! # Problem 0434: Parse an INI-style Config
//!
//! - **Difficulty:** Medium
//! - **Topics:** line-numbered error enum, `lines().enumerate()`, parser state, `BTreeMap::entry`, `?` on `Option` for lookups
//! - **Source:** Original · [std `BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) · [The Rust Book 9.2](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
//!
//! ## Problem
//!
//! INI files group `key = value` settings into `[sections]`:
//!
//! ```text
//! ; server settings
//! [server]
//! host = example.com
//! port = 8080
//!
//! [log]
//! level = debug
//! ```
//!
//! Implement `parse_ini(text: &str) -> Result<Config, IniError>` where
//! `Config = BTreeMap<String, BTreeMap<String, String>>` (section -> key -> value). Process the
//! lines in order (`N` is the 1-based line number) after trimming each one:
//!
//! - blank lines and lines starting with `';'` or `'#'` are ignored;
//! - a line starting with `'['` is a section header. It must end with `']'`
//!   (`UnclosedSection { line: N }`), and the name between the brackets, trimmed, must not be empty
//!   (`EmptySectionName { line: N }`). A section may appear several times; its keys are merged;
//! - every other line is a setting: it must come after a section header (`KeyOutsideSection`),
//!   contain a `'='` (`MissingEquals`), and have a non-empty trimmed key (`EmptyKey`). The value is
//!   the trimmed text after the first `'='` (it may be empty). A key that already exists in the same
//!   section is `DuplicateKey { line: N, key }`.
//!
//! Also implement:
//!
//! - `get<'a>(config: &'a Config, section: &str, key: &str) -> Option<&'a str>`.
//! - `get_or<'a>(config: &'a Config, section: &str, key: &str, default: &'a str) -> &'a str`.
//! - `Display` for `IniError`, always starting with `"line N: "` followed by
//!   `"unclosed section header"`, `"empty section name"`, `"key outside of any section"`,
//!   ``"expected `key = value`"``, `"empty key"` or ``"duplicate key `KEY`"``.
//!
//! ## Examples
//!
//! ```text
//! parse_ini(the text above)         -> Ok({"log": {"level": "debug"}, "server": {"host": "example.com", "port": "8080"}})
//! get(&config, "server", "port")    -> Some("8080")
//! get(&config, "db", "port")        -> None
//! get_or(&config, "log", "file", "stderr") -> "stderr"
//! parse_ini("x = 1")                -> Err(KeyOutsideSection { line: 1 })
//! parse_ini("[a]\nx = 1\n[b\n")     -> Err(UnclosedSection { line: 3 })
//! parse_ini("[a]\nx=1\nx=2")        -> Err(DuplicateKey { line: 3, key: "x" })
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` lines.
//!
//! ## Hints
//!
//! 1. Keep the current section name in an `Option<String>`; `as_ref().ok_or(...)?` gives you a
//!    reference to it or the `KeyOutsideSection` error.
//! 2. `config.entry(name).or_default()` returns the section's map, creating it if needed.
//! 3. `get` is two `?`s: `config.get(section)?.get(key).map(String::as_str)`.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

pub type Config = BTreeMap<String, BTreeMap<String, String>>;

#[derive(Debug, PartialEq)]
pub enum IniError {
    UnclosedSection { line: usize },
    EmptySectionName { line: usize },
    KeyOutsideSection { line: usize },
    MissingEquals { line: usize },
    EmptyKey { line: usize },
    DuplicateKey { line: usize, key: String },
}

impl fmt::Display for IniError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for IniError {}

pub fn parse_ini(text: &str) -> Result<Config, IniError> {
    todo!("Write your solution here")
}

pub fn get<'a>(config: &'a Config, section: &str, key: &str) -> Option<&'a str> {
    todo!("Write your solution here")
}

pub fn get_or<'a>(config: &'a Config, section: &str, key: &str, default: &'a str) -> &'a str {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str =
        "; server settings\n[server]\nhost = example.com\nport = 8080\n\n[log]\nlevel = debug\n";

    #[test]
    fn parse_sample() {
        let config = parse_ini(SAMPLE).unwrap();
        assert_eq!(config.len(), 2);
        assert_eq!(get(&config, "server", "host"), Some("example.com"));
        assert_eq!(get(&config, "server", "port"), Some("8080"));
        assert_eq!(get(&config, "log", "level"), Some("debug"));
        assert_eq!(get(&config, "db", "port"), None);
        assert_eq!(get(&config, "log", "port"), None);
        assert_eq!(get_or(&config, "log", "file", "stderr"), "stderr");
        assert_eq!(get_or(&config, "log", "level", "info"), "debug");
    }

    #[test]
    fn sections_merge_and_values_may_be_empty() {
        let config = parse_ini("[a]\nx = 1\n[b]\ny=\n# note\n[ a ]\nz = k=v\n").unwrap();
        assert_eq!(get(&config, "a", "x"), Some("1"));
        assert_eq!(get(&config, "a", "z"), Some("k=v"));
        assert_eq!(get(&config, "b", "y"), Some(""));
        assert_eq!(
            parse_ini("[empty]").unwrap().get("empty").map(|s| s.len()),
            Some(0)
        );
        assert_eq!(parse_ini("").unwrap().len(), 0);
    }

    #[test]
    fn header_errors() {
        assert_eq!(
            parse_ini("[a]\nx = 1\n[b\n"),
            Err(IniError::UnclosedSection { line: 3 })
        );
        assert_eq!(
            parse_ini("[  ]"),
            Err(IniError::EmptySectionName { line: 1 })
        );
    }

    #[test]
    fn setting_errors() {
        assert_eq!(
            parse_ini("x = 1"),
            Err(IniError::KeyOutsideSection { line: 1 })
        );
        assert_eq!(
            parse_ini("[a]\n\njust words"),
            Err(IniError::MissingEquals { line: 3 })
        );
        assert_eq!(parse_ini("[a]\n = 3"), Err(IniError::EmptyKey { line: 2 }));
        assert_eq!(
            parse_ini("[a]\nx=1\nx=2"),
            Err(IniError::DuplicateKey {
                line: 3,
                key: "x".to_string()
            })
        );
        assert_eq!(
            parse_ini("[a]\nx=1\n[b]\nx=2\n[a]\nx=3"),
            Err(IniError::DuplicateKey {
                line: 6,
                key: "x".to_string()
            })
        );
    }

    #[test]
    fn display_messages() {
        assert_eq!(
            IniError::UnclosedSection { line: 3 }.to_string(),
            "line 3: unclosed section header"
        );
        assert_eq!(
            IniError::EmptySectionName { line: 1 }.to_string(),
            "line 1: empty section name"
        );
        assert_eq!(
            IniError::KeyOutsideSection { line: 2 }.to_string(),
            "line 2: key outside of any section"
        );
        assert_eq!(
            IniError::MissingEquals { line: 4 }.to_string(),
            "line 4: expected `key = value`"
        );
        assert_eq!(
            IniError::EmptyKey { line: 5 }.to_string(),
            "line 5: empty key"
        );
        assert_eq!(
            IniError::DuplicateKey {
                line: 6,
                key: "x".to_string()
            }
            .to_string(),
            "line 6: duplicate key `x`"
        );
    }
}
