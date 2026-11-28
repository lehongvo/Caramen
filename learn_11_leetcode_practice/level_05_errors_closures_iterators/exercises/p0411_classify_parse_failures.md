# Problem 0411: Classify Parse Failures

- **Difficulty:** Easy
- **Topics:** `str::parse`, `ParseIntError`, `ParseIntError::kind`, `IntErrorKind`, `NonZeroU32`, `match` on `Result`
- **Source:** Original · [std `ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html) · [std `IntErrorKind`](https://doc.rust-lang.org/std/num/enum.IntErrorKind.html)

## Problem

`"123".parse::<i8>()` returns a `Result<i8, ParseIntError>`. When it fails, the error knows
**why**: `err.kind()` returns a `&IntErrorKind`, an enum with the variants `Empty`,
`InvalidDigit`, `PosOverflow`, `NegOverflow` and `Zero` (the last one only for the
`NonZero*` types, which reject `0`).

Implement:

- `describe_error(err: &ParseIntError) -> &'static str`, returning
  `"empty"`, `"invalid digit"`, `"too large"`, `"too small"` or `"zero"` for the five kinds, and
  `"unknown"` for any other kind (the enum is `#[non_exhaustive]`, so a catch-all arm is
  mandatory).
- `classify_i8(s: &str) -> &'static str`: `"ok"` if `s` parses as an `i8`, otherwise
  `describe_error` of the failure.
- `classify_nonzero(s: &str) -> &'static str`: the same for `NonZeroU32`.

The string is parsed exactly as given: **no trimming**. Note what the standard parser
accepts: an optional leading `+` (or `-` for signed types), then at least one ASCII digit.

## Examples

```text
classify_i8("42")       -> "ok"
classify_i8("+5")       -> "ok"
classify_i8("")         -> "empty"
classify_i8(" 5")       -> "invalid digit"   (spaces are not trimmed)
classify_i8("-")        -> "invalid digit"   (a sign alone is not a number)
classify_i8("128")      -> "too large"       (i8 goes from -128 to 127)
classify_i8("-129")     -> "too small"
classify_nonzero("0")   -> "zero"
classify_nonzero("-1")  -> "invalid digit"   (unsigned types reject '-')
```

## Constraints

- No special constraints.

## Hints

1. `use std::num::{IntErrorKind, NonZeroU32, ParseIntError};`
2. `match err.kind() { IntErrorKind::Empty => ..., ..., _ => "unknown" }`. Matching on a
   reference to an enum works with plain variant patterns.
3. For the classifiers, `match s.parse::<i8>() { Ok(_) => "ok", Err(e) => describe_error(&e) }`.

## Starter code

Write your code in [`src/practice/p0411_classify_parse_failures.rs`](../src/practice/p0411_classify_parse_failures.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0411
```

```rust
use std::num::{IntErrorKind, NonZeroU32, ParseIntError};

pub fn describe_error(err: &ParseIntError) -> &'static str {
    todo!("Write your solution here")
}

pub fn classify_i8(s: &str) -> &'static str {
    todo!("Write your solution here")
}

pub fn classify_nonzero(s: &str) -> &'static str {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_i8_values() {
        assert_eq!(classify_i8("42"), "ok");
        assert_eq!(classify_i8("+5"), "ok");
        assert_eq!(classify_i8("-128"), "ok");
        assert_eq!(classify_i8("127"), "ok");
        assert_eq!(classify_i8("-0"), "ok");
    }

    #[test]
    fn empty_and_invalid_digits() {
        assert_eq!(classify_i8(""), "empty");
        assert_eq!(classify_i8(" 5"), "invalid digit");
        assert_eq!(classify_i8("5 "), "invalid digit");
        assert_eq!(classify_i8("-"), "invalid digit");
        assert_eq!(classify_i8("1.5"), "invalid digit");
    }

    #[test]
    fn overflow_in_both_directions() {
        assert_eq!(classify_i8("128"), "too large");
        assert_eq!(classify_i8("-129"), "too small");
        assert_eq!(classify_i8("99999999999999999999"), "too large");
    }

    #[test]
    fn nonzero_rejects_zero_and_signs() {
        assert_eq!(classify_nonzero("0"), "zero");
        assert_eq!(classify_nonzero("7"), "ok");
        assert_eq!(classify_nonzero("-1"), "invalid digit");
        assert_eq!(classify_nonzero("4294967296"), "too large");
    }

    #[test]
    fn describe_error_directly() {
        let err = "x".parse::<u8>().unwrap_err();
        assert_eq!(describe_error(&err), "invalid digit");
        let err = "300".parse::<u8>().unwrap_err();
        assert_eq!(describe_error(&err), "too large");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** let the standard parser do the work and only translate its error kind into a word.
Both classifiers share `describe_error`, so the translation table is written once.

**Rust concepts:**
- `str::parse::<T>()` works for every type that implements `FromStr`; the turbofish `::<T>`
  picks the target type. `NonZeroU32` implements `FromStr` too and adds the `Zero` error.
- `ParseIntError::kind()` returns a reference, `&IntErrorKind`; match ergonomics let you write
  `IntErrorKind::Empty` instead of `&IntErrorKind::Empty`.
- `#[non_exhaustive]` enums can gain variants in future Rust versions, so code outside the
  standard library must always include a `_` arm.

```rust
use std::num::{IntErrorKind, NonZeroU32, ParseIntError};

pub fn describe_error(err: &ParseIntError) -> &'static str {
    match err.kind() {
        IntErrorKind::Empty => "empty",
        IntErrorKind::InvalidDigit => "invalid digit",
        IntErrorKind::PosOverflow => "too large",
        IntErrorKind::NegOverflow => "too small",
        IntErrorKind::Zero => "zero",
        _ => "unknown",
    }
}

pub fn classify_i8(s: &str) -> &'static str {
    match s.parse::<i8>() {
        Ok(_) => "ok",
        Err(e) => describe_error(&e),
    }
}

pub fn classify_nonzero(s: &str) -> &'static str {
    match s.parse::<NonZeroU32>() {
        Ok(_) => "ok",
        Err(e) => describe_error(&e),
    }
}
```

Full tested solution: [`src/solutions/p0411_classify_parse_failures.rs`](../src/solutions/p0411_classify_parse_failures.rs)

</details>
