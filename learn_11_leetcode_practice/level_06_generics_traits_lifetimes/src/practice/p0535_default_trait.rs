// ==================================================================
// PRACTICE 0535: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0535_default_trait.md
// Run the tests:          cargo test practice::p0535
// ==================================================================

//! # Problem 0535: Sensible Defaults
//!
//! - **Difficulty:** Easy
//! - **Topics:** implementing `Default`, `#[derive(Default)]` with a `#[default]` enum variant, struct update syntax `..Default::default()`, `T: Default` bounds
//! - **Source:** Original · [std::default::Default](https://doc.rust-lang.org/std/default/trait.Default.html)
//!
//! ## Problem
//!
//! A server is configured with
//!
//! ```text
//! pub struct ServerConfig { host: String, port: u16, max_connections: u32, verbose: bool, log_level: LogLevel }
//! pub enum LogLevel { Error, Warn, Info, Debug }
//! ```
//!
//! (all fields `pub`). `LogLevel` derives `Default` with `Info` marked `#[default]`.
//!
//! 1. Implement `Default` for `ServerConfig` **by hand**: host `"127.0.0.1"`, port `8080`,
//!    `max_connections` `100`, `verbose` `false`, and the default log level. (Deriving it would give an
//!    empty host and port `0`, which are not sensible.)
//! 2. `dev_config() -> ServerConfig`: like the default but with `verbose: true` and
//!    `log_level: LogLevel::Debug`. Use struct update syntax (`..Default::default()`).
//! 3. `with_port(port: u16) -> ServerConfig`: the default config with another port.
//! 4. `parse_or_default<T: FromStr + Default>(text: &str) -> T`: parses `text` (after trimming
//!    whitespace) and falls back to `T::default()` when parsing fails.
//!
//! ## Examples
//!
//! ```text
//! ServerConfig::default().port            => 8080
//! LogLevel::default()                     => LogLevel::Info
//! dev_config().verbose                    => true
//! with_port(3000).host                    => "127.0.0.1"
//! parse_or_default::<u16>(" 42 ")         => 42
//! parse_or_default::<u16>("abc")          => 0
//! parse_or_default::<bool>("true")        => true
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `ServerConfig { verbose: true, ..Default::default() }` fills every other field from
//!    `ServerConfig::default()`.
//! 2. `text.trim().parse::<T>()` returns a `Result`; `Result::unwrap_or_default` needs `T: Default`.

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
    pub verbose: bool,
    pub log_level: LogLevel,
}

impl Default for ServerConfig {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

/// The default config with verbose output and debug logging.
pub fn dev_config() -> ServerConfig {
    todo!("Write your solution here")
}

/// The default config listening on `port`.
pub fn with_port(port: u16) -> ServerConfig {
    todo!("Write your solution here")
}

/// Parses the trimmed text, or returns `T::default()`.
pub fn parse_or_default<T: FromStr + Default>(text: &str) -> T {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let c = ServerConfig::default();
        assert_eq!(c.host, "127.0.0.1");
        assert_eq!(c.port, 8080);
        assert_eq!(c.max_connections, 100);
        assert!(!c.verbose);
        assert_eq!(c.log_level, LogLevel::Info);
    }

    #[test]
    fn dev_and_port_variants() {
        let dev = dev_config();
        assert!(dev.verbose);
        assert_eq!(dev.log_level, LogLevel::Debug);
        assert_eq!(dev.port, 8080);
        let p = with_port(3000);
        assert_eq!(p.port, 3000);
        assert_eq!(
            p,
            ServerConfig {
                port: 3000,
                ..ServerConfig::default()
            }
        );
    }

    #[test]
    fn generic_parse_with_fallback() {
        assert_eq!(parse_or_default::<u16>(" 42 "), 42);
        assert_eq!(parse_or_default::<u16>("abc"), 0);
        assert_eq!(parse_or_default::<u16>("70000"), 0);
        assert!(parse_or_default::<bool>("true"));
        assert_eq!(parse_or_default::<f64>("2.5"), 2.5);
        assert_eq!(parse_or_default::<String>("  kept  "), "kept");
    }

    #[test]
    fn default_through_generic_code() {
        fn make<T: Default>() -> T {
            T::default()
        }
        let c: ServerConfig = make();
        assert_eq!(c.port, 8080);
        assert_eq!(make::<LogLevel>(), LogLevel::Info);
    }
}
