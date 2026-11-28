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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** resolve the fields in an order where `retries` is known before `timeout_ms`, using the
//! most fitting `Option` method for each default.
//!
//! **Rust concepts:**
//! - `unwrap_or_else(|| String::from("localhost"))` avoids allocating the default string when a
//!   host is present.
//! - `unwrap_or_default()` expresses "the type's natural default" without repeating it.
//! - `timeout_ms.unwrap_or_else(|| 1000 * (u64::from(retries) + 1))` captures `retries` in the
//!   closure; the computation only runs when the timeout is missing.
//! - `Option<u16>` is `Copy`, so `raw.port.is_some_and(...)` copies the option and leaves `raw`
//!   untouched. `Option<String>` is not `Copy`: `raw.host.as_ref()` turns `&Option<String>` into
//!   `Option<&String>` first.
//! - The `Config` literal uses field init shorthand, since the local variables have the field names.
// @solution-end

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
    // @solution-start
    let host = raw.host.unwrap_or_else(|| String::from("localhost"));
    let port = raw.port.unwrap_or(8080);
    let retries = raw.retries.unwrap_or(3);
    let timeout_ms = raw
        .timeout_ms
        .unwrap_or_else(|| 1000 * (u64::from(retries) + 1));
    let verbose = raw.verbose.unwrap_or_default();
    Config {
        host,
        port,
        retries,
        timeout_ms,
        verbose,
    }
    // @solution-end
}

/// Whether a port below 1024 is configured.
pub fn is_privileged(raw: &RawConfig) -> bool {
    // @solution-start
    raw.port.is_some_and(|p| p < 1024)
    // @solution-end
}

/// The length of the configured host, or 0.
pub fn host_length(raw: &RawConfig) -> usize {
    // @solution-start
    raw.host.as_ref().map_or(0, |h| h.len())
    // @solution-end
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
