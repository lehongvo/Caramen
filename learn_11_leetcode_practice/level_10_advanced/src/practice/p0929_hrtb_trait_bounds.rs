// ==================================================================
// PRACTICE 0929: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0929_hrtb_trait_bounds.md
// Run the tests:          cargo test practice::p0929
// ==================================================================

//! # Problem 0929: Higher-Ranked Bounds on Your Own Trait
//!
//! - **Difficulty:** Hard
//! - **Topics:** traits with a lifetime parameter, `E: for<'a> Extract<'a>`, blanket impls for closures, closure signature inference
//! - **Source:** Original · [Reference: Higher-ranked trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds) · [The Rustonomicon: HRTB](https://doc.rust-lang.org/nomicon/hrtb.html)
//!
//! ## Problem
//!
//! An extractor pulls a piece out of a text record **without copying**, so its output borrows the
//! record. The trait therefore carries the record's lifetime:
//!
//! ```text
//! pub trait Extract<'a> {
//!     fn extract(&self, record: &'a str) -> Option<&'a str>;
//! }
//! ```
//!
//! A function that applies an extractor to records **it creates itself** (or to many records with
//! unrelated lifetimes) needs an extractor that works for *every* lifetime: `E: for<'a> Extract<'a>`.
//!
//! Implement:
//!
//! 1. `Field(n)`: the `n`-th (0-based) comma-separated field of the record, trimmed; `None` if there
//!    are not enough fields.
//! 2. `Key(name)`: in a record of `key=value` pairs separated by `;`, the trimmed value of the first
//!    pair whose trimmed key is `name`; `None` if absent. Pairs without `=` are skipped.
//! 3. A blanket impl: every `F: Fn(&'a str) -> Option<&'a str>` is an `Extract<'a>`.
//! 4. `extract_all<E>(records: &[String], extractor: &E) -> Vec<Option<String>>` where
//!    `E: for<'a> Extract<'a>`: one owned result per record.
//! 5. `extract_from_lines<E>(text: &str, extractor: &E) -> Vec<String>` (same bound): for every line
//!    of `text`, build an **uppercased copy** of the line (a local `String`), apply the extractor to
//!    the copy, and collect the successful results.
//!
//! A helper `extractor_fn(f)` is given: it returns `f` unchanged, but its `Fn(&str) -> Option<&str>`
//! bound gives a closure literal the higher-ranked signature it needs (a closure passed directly to
//! `extract_all` would only see the bound `E: for<'a> Extract<'a>`, which says nothing about closure
//! signatures).
//!
//! ## Examples
//!
//! ```text
//! Field(1).extract("ada, lovelace , 1815")            => Some("lovelace")
//! Key("lang").extract("name = Rust; lang= en ")       => Some("en")
//! extract_all(&records, &Field(2))                     => [Some("1815"), None]
//! extract_all(&records, &extractor_fn(|r| r.split(',').next()))
//! extract_from_lines("id=1;x=y\nid=22", &Key("ID"))    => ["1", "22"]   (lines are uppercased first)
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `impl<'a> Extract<'a> for Field { ... }` works for every `'a`, so `Field: for<'a> Extract<'a>`.
//! 2. Blanket impl: `impl<'a, F> Extract<'a> for F where F: Fn(&'a str) -> Option<&'a str>`. It does
//!    not conflict with `Field` and `Key` because the `Fn` traits are `#[fundamental]`: the compiler
//!    knows your structs do not implement them.
//! 3. In `extract_from_lines`, convert the extracted `&str` to a `String` before the local copy is
//!    dropped.

pub trait Extract<'a> {
    fn extract(&self, record: &'a str) -> Option<&'a str>;
}

/// The `n`-th (0-based) comma-separated field, trimmed.
pub struct Field(pub usize);

/// The value of `key=value` pairs separated by `;`.
pub struct Key(pub &'static str);

impl<'a> Extract<'a> for Field {
    fn extract(&self, record: &'a str) -> Option<&'a str> {
        todo!("Write your solution here")
    }
}

impl<'a> Extract<'a> for Key {
    fn extract(&self, record: &'a str) -> Option<&'a str> {
        todo!("Write your solution here")
    }
}

impl<'a, F> Extract<'a> for F
where
    F: Fn(&'a str) -> Option<&'a str>,
{
    fn extract(&self, record: &'a str) -> Option<&'a str> {
        todo!("Write your solution here")
    }
}

/// Gives a closure literal the signature `for<'a> Fn(&'a str) -> Option<&'a str>`.
pub fn extractor_fn<F>(f: F) -> F
where
    F: Fn(&str) -> Option<&str>,
{
    f
}

pub fn extract_all<E>(records: &[String], extractor: &E) -> Vec<Option<String>>
where
    E: for<'a> Extract<'a>,
{
    todo!("Write your solution here")
}

pub fn extract_from_lines<E>(text: &str, extractor: &E) -> Vec<String>
where
    E: for<'a> Extract<'a>,
{
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn records() -> Vec<String> {
        vec![
            "ada, lovelace , 1815".to_string(),
            "alan,turing".to_string(),
        ]
    }

    #[test]
    fn field_extractor() {
        assert_eq!(Field(1).extract("ada, lovelace , 1815"), Some("lovelace"));
        assert_eq!(Field(0).extract(""), Some(""));
        assert_eq!(Field(3).extract("a,b"), None);
        assert_eq!(
            extract_all(&records(), &Field(2)),
            vec![Some("1815".to_string()), None]
        );
    }

    #[test]
    fn key_extractor() {
        assert_eq!(Key("lang").extract("name = Rust; lang= en "), Some("en"));
        assert_eq!(Key("name").extract("junk;name=a=b"), Some("a=b"));
        assert_eq!(Key("missing").extract("a=1"), None);
        assert_eq!(Key("a").extract("a=1;a=2"), Some("1"));
    }

    #[test]
    fn closures_through_the_blanket_impl() {
        let first = extractor_fn(|r| r.split(',').next());
        assert_eq!(
            extract_all(&records(), &first),
            vec![Some("ada".to_string()), Some("alan".to_string())]
        );
        let last_char = extractor_fn(|r| r.get(r.len().checked_sub(1)?..));
        assert_eq!(
            extract_all(&records(), &last_char),
            vec![Some("5".to_string()), Some("g".to_string())]
        );
    }

    #[test]
    fn extracting_from_local_copies() {
        assert_eq!(
            extract_from_lines("id=1;x=y\nid=22\nnothing", &Key("ID")),
            vec!["1", "22"]
        );
        assert_eq!(extract_from_lines("a,b\nc", &Field(1)), vec!["B"]);
        assert!(extract_from_lines("", &Field(0)).is_empty());
    }
}
