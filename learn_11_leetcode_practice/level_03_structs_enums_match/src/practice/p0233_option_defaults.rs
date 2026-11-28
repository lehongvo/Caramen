// ==================================================================
// PRACTICE 0233: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0233_option_defaults.md
// Run the tests:          cargo test practice::p0233
// ==================================================================

//! # Problem 0233: Defaults for Missing Values
//!
//! - **Difficulty:** Easy
//! - **Topics:** `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`, `map_or`, `is_some_and`, `Option` fields in a struct
//! - **Source:** Original · [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
//!
//! ## Problem
//!
//! A server reads its settings from a file where every setting is optional. The parsed file is a
//! `RawConfig` whose fields are all `Option`s; the server needs a `Config` where every field has a
//! value.
//!
//! ```text
//! RawConfig { host: Option<String>, port: Option<u16>, retries: Option<u32>,
//!             timeout_ms: Option<u64>, verbose: Option<bool> }
//! Config    { host: String, port: u16, retries: u32, timeout_ms: u64, verbose: bool }
//! ```
//!
//! Implement `resolve(raw: RawConfig) -> Config` with these defaults:
//!
//! - `host`: `"localhost"`;
//! - `port`: `8080`;
//! - `retries`: `3`;
//! - `timeout_ms`: `1000 * (retries + 1)`, computed from the **resolved** `retries` (so it is only
//!   computed when needed);
//! - `verbose`: `false` (the type's default value).
//!
//! Also implement two queries on a `&RawConfig`, without cloning anything:
//!
//! - `is_privileged(raw: &RawConfig) -> bool`: `true` if a port is given and it is below `1024`;
//! - `host_length(raw: &RawConfig) -> usize`: the length of the given host, or `0` if none.
//!
//! ## Examples
//!
//! ```text
//! resolve(RawConfig { all None })
//!   => Config { host: "localhost", port: 8080, retries: 3, timeout_ms: 4000, verbose: false }
//! resolve(RawConfig { host: Some("example.org"), retries: Some(0), verbose: Some(true), .. None })
//!   => Config { host: "example.org", port: 8080, retries: 0, timeout_ms: 1000, verbose: true }
//! is_privileged(&RawConfig { port: Some(443), .. })  => true
//! is_privileged(&RawConfig { port: None, .. })       => false
//! host_length(&RawConfig { host: Some("db"), .. })   => 2
//! ```
//!
//! ## Constraints
//!
//! - `retries <= 10^6`.
//!
//! ## Hints
//!
//! 1. `unwrap_or(value)` evaluates `value` eagerly; `unwrap_or_else(|| ...)` only calls the closure
//!    when the option is `None`.
//! 2. `unwrap_or_default()` uses `Default::default()`: `false` for `bool`, `0` for numbers, `""` for `String`.
//! 3. `is_some_and(|p| ...)` and `map_or(default, |h| ...)` read a value without taking it out, but
//!    they consume the `Option`: call them on `raw.port` (a `Copy` type) directly, and on
//!    `raw.host.as_ref()` for the `String`.

/// Settings as read from the file: everything is optional.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RawConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub retries: Option<u32>,
    pub timeout_ms: Option<u64>,
    pub verbose: Option<bool>,
}

/// Fully resolved settings.
#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub retries: u32,
    pub timeout_ms: u64,
    pub verbose: bool,
}

/// Fills every missing setting with its default.
pub fn resolve(raw: RawConfig) -> Config {
    todo!("Write your solution here")
}

/// Whether a port below 1024 is configured.
pub fn is_privileged(raw: &RawConfig) -> bool {
    todo!("Write your solution here")
}

/// The length of the configured host, or 0.
pub fn host_length(raw: &RawConfig) -> usize {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_defaults() {
        assert_eq!(
            resolve(RawConfig::default()),
            Config {
                host: "localhost".to_string(),
                port: 8080,
                retries: 3,
                timeout_ms: 4000,
                verbose: false,
            }
        );
    }

    #[test]
    fn timeout_follows_resolved_retries() {
        let raw = RawConfig {
            host: Some("example.org".to_string()),
            retries: Some(0),
            verbose: Some(true),
            ..RawConfig::default()
        };
        assert_eq!(
            resolve(raw),
            Config {
                host: "example.org".to_string(),
                port: 8080,
                retries: 0,
                timeout_ms: 1000,
                verbose: true,
            }
        );
    }

    #[test]
    fn explicit_values_win() {
        let raw = RawConfig {
            host: Some("db".to_string()),
            port: Some(5432),
            retries: Some(1_000_000),
            timeout_ms: Some(7),
            verbose: Some(false),
        };
        let cfg = resolve(raw);
        assert_eq!(cfg.port, 5432);
        assert_eq!(cfg.retries, 1_000_000);
        assert_eq!(cfg.timeout_ms, 7);
    }

    #[test]
    fn huge_retries_timeout() {
        let raw = RawConfig {
            retries: Some(1_000_000),
            ..RawConfig::default()
        };
        assert_eq!(resolve(raw).timeout_ms, 1_000_001_000);
    }

    #[test]
    fn privileged_ports() {
        let mut raw = RawConfig {
            port: Some(443),
            ..RawConfig::default()
        };
        assert!(is_privileged(&raw));
        raw.port = Some(1024);
        assert!(!is_privileged(&raw));
        raw.port = None;
        assert!(!is_privileged(&raw));
    }

    #[test]
    fn host_lengths_without_moving() {
        let raw = RawConfig {
            host: Some("db".to_string()),
            ..RawConfig::default()
        };
        assert_eq!(host_length(&raw), 2);
        // `raw` was only borrowed, so it can still be used.
        assert_eq!(raw.host.as_deref(), Some("db"));
        assert_eq!(host_length(&RawConfig::default()), 0);
    }
}
