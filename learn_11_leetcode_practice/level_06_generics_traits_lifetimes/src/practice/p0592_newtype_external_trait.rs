// ==================================================================
// PRACTICE 0592: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0592_newtype_external_trait.md
// Run the tests:          cargo test practice::p0592
// ==================================================================

//! # Problem 0592: Newtype for an External Trait
//!
//! - **Difficulty:** Easy
//! - **Topics:** the orphan rule, wrapping a foreign type (`Vec<String>`, `Vec<u8>`) in a local newtype to implement foreign traits (`Display`, `FromStr`), `Deref` to keep the inner API
//! - **Source:** Original · [The Rust Book 20.3](https://doc.rust-lang.org/book/ch20-03-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction)
//!
//! ## Problem
//!
//! The **orphan rule** forbids `impl fmt::Display for Vec<String>`: both the trait and the type are
//! defined outside your crate. The workaround is a **newtype**: a local tuple struct wrapping the
//! foreign type.
//!
//! 1. `Wrapper(pub Vec<String>)`:
//!    - `Display` as `"[a, b, c]"` (`"[]"` when empty).
//!    - `Deref<Target = Vec<String>>`, so `wrapper.len()`, `wrapper.iter()`, `wrapper[0]` keep working.
//!    - `From<Vec<String>>`.
//! 2. `Hex(pub Vec<u8>)`:
//!    - `Display` as lowercase hex pairs separated by spaces: `[222, 173, 190, 239]` is
//!      `"de ad be ef"`.
//!    - `FromStr` with `type Err = String`: parses hex text, ignoring spaces, such as `"deadbeef"` or
//!      `"de ad be ef"`. An odd number of hex digits or a non-hex character is an error (any message).
//!
//! ## Examples
//!
//! ```text
//! Wrapper(vec!["hello".into(), "world".into()]).to_string()   => "[hello, world]"
//! Wrapper::from(vec!["x".to_string()]).len()                  => 1        (through Deref)
//! Hex(vec![222, 173, 190, 239]).to_string()                   => "de ad be ef"
//! "DE ad BEEF".parse::<Hex>()                                 => Ok(Hex([222, 173, 190, 239]))
//! "abc".parse::<Hex>()                                        => Err(..)   (odd number of digits)
//! ```
//!
//! ## Constraints
//!
//! - Hex digits may be upper or lower case.
//!
//! ## Hints
//!
//! 1. In `Display`, write `self.0.join(", ")` between brackets.
//! 2. For parsing, collect the non-space characters, check that they are all hex digits
//!    (`char::is_ascii_hexdigit`) and that their count is even, then convert each pair with
//!    `u8::from_str_radix(pair, 16)`. (Check the digits yourself: `from_str_radix` accepts a leading
//!    `+`, so `"+1"` would otherwise parse.)

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

/// A local wrapper so that `Display` can be implemented for a list of strings.
#[derive(Debug, Clone, PartialEq)]
pub struct Wrapper(pub Vec<String>);

/// Bytes displayed and parsed as hexadecimal.
#[derive(Debug, Clone, PartialEq)]
pub struct Hex(pub Vec<u8>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        todo!("Write your solution here")
    }
}

impl From<Vec<String>> for Wrapper {
    fn from(v: Vec<String>) -> Self {
        todo!("Write your solution here")
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl FromStr for Hex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapper_display_and_deref() {
        let w = Wrapper(vec!["hello".into(), "world".into()]);
        assert_eq!(w.to_string(), "[hello, world]");
        assert_eq!(w.len(), 2);
        assert_eq!(w[1], "world");
        assert_eq!(w.iter().map(String::len).sum::<usize>(), 10);
        assert_eq!(Wrapper(vec![]).to_string(), "[]");
        assert_eq!(Wrapper::from(vec!["x".to_string()]).len(), 1);
    }

    #[test]
    fn hex_display() {
        assert_eq!(Hex(vec![222, 173, 190, 239]).to_string(), "de ad be ef");
        assert_eq!(Hex(vec![0, 15, 255]).to_string(), "00 0f ff");
        assert_eq!(Hex(vec![]).to_string(), "");
    }

    #[test]
    fn hex_parsing() {
        assert_eq!(
            "DE ad BEEF".parse::<Hex>(),
            Ok(Hex(vec![222, 173, 190, 239]))
        );
        assert_eq!("".parse::<Hex>(), Ok(Hex(vec![])));
        assert!("abc".parse::<Hex>().is_err());
        assert!("zz".parse::<Hex>().is_err());
        assert!("+1".parse::<Hex>().is_err());
        let bytes: Vec<u8> = (0..=255).collect();
        let text = Hex(bytes.clone()).to_string();
        assert_eq!(text.parse::<Hex>(), Ok(Hex(bytes)));
    }
}
