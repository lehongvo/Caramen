// ==================================================================
// PRACTICE 0023: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0023_smallest_integer_type.md
// Run the tests:          cargo test practice::p0023
// ==================================================================

//! # Problem 0023: Smallest Integer Type
//!
//! - **Difficulty:** Easy
//! - **Topics:** integer types and their ranges, `i8::MIN`/`i8::MAX`, `const` in range patterns, `&'static str`
//! - **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)
//!
//! ## Problem
//!
//! Rust has signed integer types `i8`, `i16`, `i32`, `i64` and unsigned types `u8`, `u16`, `u32`,
//! `u64`. A type with `n` bits can store:
//!
//! - signed: `-(2^(n-1))` to `2^(n-1) - 1` (for `i8`: `-128..=127`);
//! - unsigned: `0` to `2^n - 1` (for `u8`: `0..=255`).
//!
//! Write two functions that name the **smallest** type able to store a value:
//!
//! 1. `smallest_signed_type(x: i64) -> &'static str` returns one of `"i8"`, `"i16"`, `"i32"`, `"i64"`.
//! 2. `smallest_unsigned_type(x: u64) -> &'static str` returns one of `"u8"`, `"u16"`, `"u32"`, `"u64"`.
//!
//! Do not hard-code the limits as numbers: use the associated constants such as `i16::MAX`.
//!
//! ## Examples
//!
//! ```text
//! smallest_signed_type(100)     -> "i8"
//! smallest_signed_type(-129)    -> "i16"     i8 stops at -128
//! smallest_signed_type(40_000)  -> "i32"     i16 stops at 32_767
//! smallest_signed_type(i64::MIN) -> "i64"
//!
//! smallest_unsigned_type(255)   -> "u8"
//! smallest_unsigned_type(256)   -> "u16"
//! smallest_unsigned_type(70_000) -> "u32"
//! ```
//!
//! ## Constraints
//!
//! - Any `i64` value for the first function and any `u64` value for the second.
//!
//! ## Hints
//!
//! 1. A `match` arm can use a range pattern whose ends are **constants**: `LOW..=HIGH => ...`.
//! 2. Patterns cannot contain arbitrary expressions such as `i8::MIN as i64`, but a `const` can:
//!    `const I8_MIN: i64 = i8::MIN as i64;`.
//! 3. For the unsigned version an `if` / `else if` chain of `x <= ...` comparisons is enough,
//!    because every range starts at 0.

pub fn smallest_signed_type(x: i64) -> &'static str {
    todo!("Write your solution here")
}

pub fn smallest_unsigned_type(x: u64) -> &'static str {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_examples() {
        assert_eq!(smallest_signed_type(100), "i8");
        assert_eq!(smallest_signed_type(-129), "i16");
        assert_eq!(smallest_signed_type(40_000), "i32");
        assert_eq!(smallest_signed_type(i64::MIN), "i64");
    }

    #[test]
    fn signed_boundaries() {
        assert_eq!(smallest_signed_type(-128), "i8");
        assert_eq!(smallest_signed_type(127), "i8");
        assert_eq!(smallest_signed_type(128), "i16");
        assert_eq!(smallest_signed_type(32_767), "i16");
        assert_eq!(smallest_signed_type(-32_768), "i16");
        assert_eq!(smallest_signed_type(32_768), "i32");
        assert_eq!(smallest_signed_type(2_147_483_647), "i32");
        assert_eq!(smallest_signed_type(-2_147_483_648), "i32");
        assert_eq!(smallest_signed_type(2_147_483_648), "i64");
        assert_eq!(smallest_signed_type(i64::MAX), "i64");
    }

    #[test]
    fn unsigned_examples() {
        assert_eq!(smallest_unsigned_type(255), "u8");
        assert_eq!(smallest_unsigned_type(256), "u16");
        assert_eq!(smallest_unsigned_type(70_000), "u32");
    }

    #[test]
    fn unsigned_boundaries() {
        assert_eq!(smallest_unsigned_type(0), "u8");
        assert_eq!(smallest_unsigned_type(65_535), "u16");
        assert_eq!(smallest_unsigned_type(65_536), "u32");
        assert_eq!(smallest_unsigned_type(4_294_967_295), "u32");
        assert_eq!(smallest_unsigned_type(4_294_967_296), "u64");
        assert_eq!(smallest_unsigned_type(u64::MAX), "u64");
    }
}
