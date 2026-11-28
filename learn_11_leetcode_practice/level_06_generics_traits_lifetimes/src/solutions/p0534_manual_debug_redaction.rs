//! # Problem 0534: A Manual `Debug` That Redacts Secrets
//!
//! - **Difficulty:** Easy
//! - **Topics:** implementing `fmt::Debug` by hand, `debug_tuple`, `debug_map`, generic impls without bounds, `{:?}` vs `{:#?}`
//! - **Source:** Original · [std::fmt::Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
//!
//! ## Problem
//!
//! `#[derive(Debug)]` prints every field, which is a problem when a field is a password that ends up
//! in a log. Write `Debug` by hand for three types:
//!
//! 1. `Secret<T>(pub T)`: always prints `Secret(<redacted>)`, whatever `T` is. The impl must **not**
//!    require `T: Debug`, so even a `Secret` of a type without `Debug` can be printed.
//! 2. `ApiKey(pub String)`: prints like a tuple struct whose only field is a **string** in which every
//!    character except the last 4 is replaced by `*`: `ApiKey("********1234")`. A key of 4 characters
//!    or fewer is fully masked. Build it with `f.debug_tuple(..)`, so `{:#?}` pretty-prints it.
//! 3. `Inventory { items: Vec<(String, u32)> }`: prints as a map, `{"apple": 3, "pear": 0}`, in the
//!    stored order. Build it with `f.debug_map()`.
//!
//! `Credentials { user: String, password: Secret<String>, key: ApiKey }` simply uses
//! `#[derive(Debug)]`: it works because all its fields implement `Debug`.
//!
//! ## Examples
//!
//! ```text
//! format!("{:?}", Secret("hunter2"))                     => "Secret(<redacted>)"
//! format!("{:?}", ApiKey("sk-live-1234".to_string()))    => "ApiKey(\"********1234\")"
//! format!("{:?}", inventory)                             => "{\"apple\": 3, \"pear\": 0}"
//! format!("{:?}", credentials)
//!   => "Credentials { user: \"ada\", password: Secret(<redacted>), key: ApiKey(\"****abcd\") }"
//! ```
//!
//! ## Constraints
//!
//! - Keys may contain any Unicode characters; count characters, not bytes.
//!
//! ## Hints
//!
//! 1. `impl<T> fmt::Debug for Secret<T>` has no bound, so you cannot (and must not) print `self.0`.
//! 2. `f.debug_tuple("ApiKey").field(&masked).finish()` prints `masked` with its own `Debug`, which adds
//!    the quotes.
//! 3. `f.debug_map().entries(iter_of_pairs).finish()`; the pairs can be `(&String, &u32)`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `Secret` writes a fixed string. `ApiKey` builds the masked string first and hands it to a
//! tuple builder. `Inventory` feeds `(name, count)` pairs to a map builder.
//!
//! **Rust concepts:**
//! - The `debug_*` builders (`debug_struct`, `debug_tuple`, `debug_list`, `debug_map`) handle commas,
//!   braces and the pretty `{:#?}` layout for you, so manual impls stay consistent with derived ones.
//! - `#[derive(Debug)]` on a generic type adds a `T: Debug` bound; writing the impl by hand lets
//!   `Secret<T>` be `Debug` for every `T`.
//! - Derived `Debug` for `Credentials` calls the manual impls of its fields, so redaction composes.
// @solution-end

use std::fmt;

/// A value that must never be printed.
pub struct Secret<T>(pub T);

/// An API key shown with all but its last 4 characters masked.
pub struct ApiKey(pub String);

/// Item names and counts, printed as a map.
pub struct Inventory {
    pub items: Vec<(String, u32)>,
}

#[derive(Debug)]
pub struct Credentials {
    pub user: String,
    pub password: Secret<String>,
    pub key: ApiKey,
}

impl<T> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // @solution-start
        f.write_str("Secret(<redacted>)")
        // @solution-end
    }
}

impl fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // @solution-start
        let len = self.0.chars().count();
        let hidden = len.saturating_sub(4);
        let masked: String = self
            .0
            .chars()
            .enumerate()
            .map(|(i, c)| if i < hidden || len <= 4 { '*' } else { c })
            .collect();
        f.debug_tuple("ApiKey").field(&masked).finish()
        // @solution-end
    }
}

impl fmt::Debug for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // @solution-start
        f.debug_map()
            .entries(self.items.iter().map(|(name, count)| (name, count)))
            .finish()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secrets_are_redacted_for_any_type() {
        assert_eq!(format!("{:?}", Secret("hunter2")), "Secret(<redacted>)");
        struct NoDebug;
        assert_eq!(format!("{:?}", Secret(NoDebug)), "Secret(<redacted>)");
        assert_eq!(format!("{:#?}", Secret(42)), "Secret(<redacted>)");
    }

    #[test]
    fn api_keys_show_last_four() {
        assert_eq!(
            format!("{:?}", ApiKey("sk-live-1234".to_string())),
            "ApiKey(\"********1234\")"
        );
        assert_eq!(
            format!("{:?}", ApiKey("abcd".to_string())),
            "ApiKey(\"****\")"
        );
        assert_eq!(format!("{:?}", ApiKey(String::new())), "ApiKey(\"\")");
        assert_eq!(
            format!("{:?}", ApiKey("\u{e9}\u{e9}\u{e9}\u{e9}\u{e9}".to_string())),
            "ApiKey(\"*\u{e9}\u{e9}\u{e9}\u{e9}\")"
        );
    }

    #[test]
    fn inventory_prints_as_a_map() {
        let inv = Inventory {
            items: vec![("apple".to_string(), 3), ("pear".to_string(), 0)],
        };
        assert_eq!(format!("{inv:?}"), "{\"apple\": 3, \"pear\": 0}");
        assert_eq!(format!("{:?}", Inventory { items: vec![] }), "{}");
    }

    #[test]
    fn derived_debug_uses_the_manual_impls() {
        let c = Credentials {
            user: "ada".to_string(),
            password: Secret("correct horse".to_string()),
            key: ApiKey("xyz-abcd".to_string()),
        };
        assert_eq!(
            format!("{c:?}"),
            "Credentials { user: \"ada\", password: Secret(<redacted>), key: ApiKey(\"****abcd\") }"
        );
        let pretty = format!("{c:#?}");
        assert!(pretty.contains("    password: Secret(<redacted>),\n"));
        assert!(pretty.contains("    key: ApiKey(\n        \"****abcd\",\n    ),\n"));
        assert!(!pretty.contains("correct horse"));
    }
}
