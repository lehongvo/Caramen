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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a line-by-line state machine whose only state is "the current section". Each line is
//! classified (ignored, header, setting) and every failing rule returns an error tagged with the
//! line number. Settings are inserted through `entry` so a repeated section merges, and an occupied
//! key is reported as a duplicate.
//!
//! **Rust concepts:**
//! - Every error variant carries `line`, which makes the messages actionable.
//! - `Option::as_ref()` borrows the current section name without moving it out of the variable.
//! - `BTreeMap::entry(..).or_default()` and `contains_key` / `insert` build nested maps.
//! - `?` on `Option` inside `get` keeps lookups flat; the explicit `'a` says the returned `&str`
//!   borrows from `config`, not from the `section` or `key` arguments.
// @solution-end

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
        // @solution-start
        match self {
            IniError::UnclosedSection { line } => write!(f, "line {line}: unclosed section header"),
            IniError::EmptySectionName { line } => write!(f, "line {line}: empty section name"),
            IniError::KeyOutsideSection { line } => {
                write!(f, "line {line}: key outside of any section")
            }
            IniError::MissingEquals { line } => write!(f, "line {line}: expected `key = value`"),
            IniError::EmptyKey { line } => write!(f, "line {line}: empty key"),
            IniError::DuplicateKey { line, key } => write!(f, "line {line}: duplicate key `{key}`"),
        }
        // @solution-end
    }
}

impl Error for IniError {}

pub fn parse_ini(text: &str) -> Result<Config, IniError> {
    // @solution-start
    let mut config = Config::new();
    let mut section: Option<String> = None;
    for (i, raw) in text.lines().enumerate() {
        let line_no = i + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if let Some(header) = line.strip_prefix('[') {
            let name = header
                .strip_suffix(']')
                .ok_or(IniError::UnclosedSection { line: line_no })?
                .trim();
            if name.is_empty() {
                return Err(IniError::EmptySectionName { line: line_no });
            }
            config.entry(name.to_string()).or_default();
            section = Some(name.to_string());
            continue;
        }
        let current = section
            .as_ref()
            .ok_or(IniError::KeyOutsideSection { line: line_no })?;
        let (key, value) = line
            .split_once('=')
            .ok_or(IniError::MissingEquals { line: line_no })?;
        let key = key.trim();
        if key.is_empty() {
            return Err(IniError::EmptyKey { line: line_no });
        }
        let settings = config.entry(current.clone()).or_default();
        if settings.contains_key(key) {
            return Err(IniError::DuplicateKey {
                line: line_no,
                key: key.to_string(),
            });
        }
        settings.insert(key.to_string(), value.trim().to_string());
    }
    Ok(config)
    // @solution-end
}

pub fn get<'a>(config: &'a Config, section: &str, key: &str) -> Option<&'a str> {
    // @solution-start
    config.get(section)?.get(key).map(String::as_str)
    // @solution-end
}

pub fn get_or<'a>(config: &'a Config, section: &str, key: &str, default: &'a str) -> &'a str {
    // @solution-start
    get(config, section, key).unwrap_or(default)
    // @solution-end
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
