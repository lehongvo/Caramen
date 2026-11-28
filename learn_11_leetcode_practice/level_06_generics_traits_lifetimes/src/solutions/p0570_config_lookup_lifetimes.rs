//! # Problem 0570: Config Lookups That Outlive the Borrow
//!
//! - **Difficulty:** Medium
//! - **Topics:** a struct holding `&'a str` pairs, methods returning `&'a str` instead of `&self`-bound references, lifetime bounds `'a: 'b`
//! - **Source:** Original · [The Rust Reference: lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds)
//!
//! ## Problem
//!
//! Parse a small configuration file **without copying any text**. `Config<'a>` keeps a list of
//! `(key, value)` slices into the source text in a field `entries: Vec<(&'a str, &'a str)>`.
//!
//! Implement in `impl<'a> Config<'a>`:
//!
//! - `parse(text: &'a str) -> Result<Config<'a>, String>`: one entry per line in the form
//!   `key = value` (both trimmed). Empty lines and lines starting with `#` (after trimming) are skipped.
//!   A line without `=` fails with `"line {n}: missing '='"` where `n` is the 1-based line number.
//! - `get(&self, key: &str) -> Option<&'a str>`: the value of the **last** entry with that key. The
//!   result has lifetime `'a`, so it stays valid after the `Config` is dropped, as long as the text is
//!   alive.
//! - `get_or<'b>(&self, key: &str, default: &'b str) -> &'b str where 'a: 'b`: the value or `default`.
//!   The result may come from the text (`'a`) or from `default` (`'b`); the bound `'a: 'b` ("`'a`
//!   outlives `'b`") makes a `&'a str` usable as a `&'b str`.
//! - `keys(&self) -> Vec<&'a str>`: the distinct keys in order of first appearance.
//!
//! ## Examples
//!
//! ```text
//! text = "# server\nhost = example.org\nport = 8080\n\nport = 9090\n"
//! let c = Config::parse(&text)?;
//! c.get("host")              => Some("example.org")
//! c.get("port")              => Some("9090")        (last one wins)
//! c.get_or("user", "admin")  => "admin"
//! c.keys()                   => ["host", "port"]
//! Config::parse("a = 1\noops")  => Err("line 2: missing '='")
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `text.lines().enumerate()` gives the line numbers; `line.split_once('=')` the two halves.
//! 2. If `get` were declared as `fn get(&self, key: &str) -> Option<&str>`, elision (rule 3) would tie
//!    the result to `&self`, and the value could not outlive the `Config`. Write `&'a str` instead.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** store only slices of the input. Every lookup scans the entries (from the end for `get`),
//! and returns a slice with the text's lifetime `'a`.
//!
//! **Rust concepts:**
//! - A method on `Config<'a>` can return data with the struct's lifetime parameter `'a`, which may be
//!   longer than the borrow of `self`. This is the main reason to name the lifetime in the return type.
//! - `'a: 'b` is a *lifetime bound*: it lets the compiler shorten a `&'a str` to `&'b str`, so both
//!   branches of `get_or` have the same type.
//! - Parsing into borrowed slices makes the parser allocation-free except for the `Vec` of pairs.
// @solution-end

/// `key = value` pairs borrowed from a configuration text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config<'a> {
    entries: Vec<(&'a str, &'a str)>,
}

impl<'a> Config<'a> {
    pub fn parse(text: &'a str) -> Result<Config<'a>, String> {
        // @solution-start
        let mut entries = Vec::new();
        for (i, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("line {}: missing '='", i + 1))?;
            entries.push((key.trim(), value.trim()));
        }
        Ok(Config { entries })
        // @solution-end
    }

    pub fn get(&self, key: &str) -> Option<&'a str> {
        // @solution-start
        self.entries
            .iter()
            .rev()
            .find(|(k, _)| *k == key)
            .map(|&(_, v)| v)
        // @solution-end
    }

    pub fn get_or<'b>(&self, key: &str, default: &'b str) -> &'b str
    where
        'a: 'b,
    {
        // @solution-start
        self.get(key).unwrap_or(default)
        // @solution-end
    }

    pub fn keys(&self) -> Vec<&'a str> {
        // @solution-start
        let mut keys: Vec<&'a str> = Vec::new();
        for &(k, _) in &self.entries {
            if !keys.contains(&k) {
                keys.push(k);
            }
        }
        keys
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "# server\nhost = example.org\nport = 8080\n\nport = 9090\n";

    #[test]
    fn parse_and_get() {
        let c = Config::parse(TEXT).unwrap();
        assert_eq!(c.get("host"), Some("example.org"));
        assert_eq!(c.get("port"), Some("9090"));
        assert_eq!(c.get("user"), None);
        assert_eq!(c.keys(), vec!["host", "port"]);
    }

    #[test]
    fn values_outlive_the_config() {
        let text = String::from("name = Ada\nlang = Rust");
        let lang;
        {
            let c = Config::parse(&text).unwrap();
            lang = c.get("lang");
        } // `c` is dropped, `lang` still borrows from `text`
        assert_eq!(lang, Some("Rust"));
    }

    #[test]
    fn defaults_with_a_shorter_lifetime() {
        let c = Config::parse(TEXT).unwrap();
        let fallback = String::from("admin");
        assert_eq!(c.get_or("user", &fallback), "admin");
        assert_eq!(c.get_or("host", &fallback), "example.org");
    }

    #[test]
    fn errors_and_edge_cases() {
        assert_eq!(
            Config::parse("a = 1\noops"),
            Err("line 2: missing '='".to_string())
        );
        assert_eq!(Config::parse("").unwrap().keys(), Vec::<&str>::new());
        let c = Config::parse("  # comment\n empty =\n x=y=z ").unwrap();
        assert_eq!(c.get("empty"), Some(""));
        assert_eq!(c.get("x"), Some("y=z"));
    }
}
