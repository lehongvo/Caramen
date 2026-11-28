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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for the signed version, define the limits of each smaller type, widened to `i64`, as
//! constants and use them in range patterns. The arms are tried from top to bottom, so the first
//! matching range is the smallest type. For the unsigned version, compare `x` with each `MAX` in
//! increasing order.
//!
//! **Rust concepts:**
//! - Every integer type has associated constants `MIN` and `MAX` (`i8::MIN == -128`, `u16::MAX == 65_535`).
//! - Range patterns must be built from literals or constant paths. Casting inside a `const`
//!   declaration is fine because it is evaluated at compile time; widening casts (`i8` to `i64`)
//!   never change the value.
//! - `u64::from(u8::MAX)` is the lossless way to widen a value in ordinary (non-pattern) code.
//! - `&'static str` is the type of a string literal: text that is stored in the program binary and
//!   lives for the whole run, so it can be returned freely.
//! - The final arm is `_` because `i64` values outside the `i32` range all need an `i64`.
// @solution-end

// @solution-start remove
const I8_MIN: i64 = i8::MIN as i64;
const I8_MAX: i64 = i8::MAX as i64;
const I16_MIN: i64 = i16::MIN as i64;
const I16_MAX: i64 = i16::MAX as i64;
const I32_MIN: i64 = i32::MIN as i64;
const I32_MAX: i64 = i32::MAX as i64;
// @solution-end

pub fn smallest_signed_type(x: i64) -> &'static str {
    // @solution-start
    match x {
        I8_MIN..=I8_MAX => "i8",
        I16_MIN..=I16_MAX => "i16",
        I32_MIN..=I32_MAX => "i32",
        _ => "i64",
    }
    // @solution-end
}

pub fn smallest_unsigned_type(x: u64) -> &'static str {
    // @solution-start
    if x <= u64::from(u8::MAX) {
        "u8"
    } else if x <= u64::from(u16::MAX) {
        "u16"
    } else if x <= u64::from(u32::MAX) {
        "u32"
    } else {
        "u64"
    }
    // @solution-end
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
