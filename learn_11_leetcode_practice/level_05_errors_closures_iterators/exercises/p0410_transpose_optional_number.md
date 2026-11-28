# Problem 0410: Parse an Optional Number with transpose

- **Difficulty:** Easy
- **Topics:** `Option::transpose`, `Option<Result<T, E>>` vs `Result<Option<T>, E>`, `ParseIntError`, `?`
- **Source:** Original · [std `Option::transpose`](https://doc.rust-lang.org/std/option/enum.Option.html#method.transpose)

## Problem

Some inputs are both **optional** and **fallible**: a setting may be absent (that is fine),
but if it is present it must be a valid number. Mapping a parser over an `Option<&str>` gives
an `Option<Result<i32, ParseIntError>>`, which is awkward: you cannot use `?` on it directly.
`transpose` swaps the two layers into a `Result<Option<i32>, ParseIntError>`:

```text
None          -> Ok(None)
Some(Ok(v))   -> Ok(Some(v))
Some(Err(e))  -> Err(e)
```

Implement:

- `parse_optional(input: Option<&str>) -> Result<Option<i32>, ParseIntError>`.
- `sum_optional(a: Option<&str>, b: Option<&str>) -> Result<i32, ParseIntError>`: treat a
  missing number as `0`, but return the parse error of an invalid one (check `a` first).
- `parse_port(raw: Option<&str>) -> Result<u16, String>`: a missing port means `8080`; a present
  port must parse as a `u16`, otherwise return ``"invalid port `RAW`: REASON"`` where `REASON` is
  the `Display` text of the `ParseIntError`.

## Examples

```text
parse_optional(None)          -> Ok(None)
parse_optional(Some("42"))    -> Ok(Some(42))
parse_optional(Some("4x"))    -> Err(ParseIntError { kind: InvalidDigit })
sum_optional(Some("5"), None) -> Ok(5)
parse_port(None)              -> Ok(8080)
parse_port(Some("443"))       -> Ok(443)
parse_port(Some("70000"))     -> Err("invalid port `70000`: number too large to fit in target type")
```

## Constraints

- No special constraints.

## Hints

1. `input.map(|s| s.parse::<i32>())` has type `Option<Result<i32, ParseIntError>>`.
2. After `.transpose()` you can apply `?` and then `unwrap_or(0)`.
3. In `parse_port`, call `map_err` inside the `map` closure, where you still have `s`.

## Starter code

Write your code in [`src/practice/p0410_transpose_optional_number.rs`](../src/practice/p0410_transpose_optional_number.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0410
```

```rust
use std::num::ParseIntError;

pub fn parse_optional(input: Option<&str>) -> Result<Option<i32>, ParseIntError> {
    todo!("Write your solution here")
}

pub fn sum_optional(a: Option<&str>, b: Option<&str>) -> Result<i32, ParseIntError> {
    todo!("Write your solution here")
}

pub fn parse_port(raw: Option<&str>) -> Result<u16, String> {
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
    use std::num::IntErrorKind;

    #[test]
    fn parse_optional_three_cases() {
        assert_eq!(parse_optional(None), Ok(None));
        assert_eq!(parse_optional(Some("42")), Ok(Some(42)));
        assert_eq!(parse_optional(Some("-7")), Ok(Some(-7)));
        let err = parse_optional(Some("4x")).unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::InvalidDigit);
    }

    #[test]
    fn sum_treats_missing_as_zero() {
        assert_eq!(sum_optional(Some("5"), None), Ok(5));
        assert_eq!(sum_optional(None, None), Ok(0));
        assert_eq!(sum_optional(Some("5"), Some("-8")), Ok(-3));
    }

    #[test]
    fn sum_reports_the_first_invalid_number() {
        let err = sum_optional(Some(""), Some("x")).unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::Empty);
        let err = sum_optional(Some("1"), Some("x")).unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::InvalidDigit);
    }

    #[test]
    fn parse_port_default_and_valid() {
        assert_eq!(parse_port(None), Ok(8080));
        assert_eq!(parse_port(Some("443")), Ok(443));
        assert_eq!(parse_port(Some("0")), Ok(0));
    }

    #[test]
    fn parse_port_errors_explain_the_problem() {
        assert_eq!(
            parse_port(Some("70000")),
            Err("invalid port `70000`: number too large to fit in target type".to_string())
        );
        assert_eq!(
            parse_port(Some("http")),
            Err("invalid port `http`: invalid digit found in string".to_string())
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** map the parser over the option, then `transpose` so that the error moves to the
outside where `?` can handle it. What is left is an `Option` that `unwrap_or` fills in.

**Rust concepts:**
- `transpose` exists on both types: `Option<Result<T, E>>::transpose` and
  `Result<Option<T>, E>::transpose` are inverses of each other.
- `?` on a `Result` returns the error early; `ParseIntError` passes through unchanged because
  the function returns the same error type.
- The `Display` text of a `ParseIntError` (`"invalid digit found in string"`,
  `"number too large to fit in target type"`, ...) is a ready-made human explanation.

```rust
use std::num::ParseIntError;

pub fn parse_optional(input: Option<&str>) -> Result<Option<i32>, ParseIntError> {
    input.map(|s| s.parse::<i32>()).transpose()
}

pub fn sum_optional(a: Option<&str>, b: Option<&str>) -> Result<i32, ParseIntError> {
    let a = parse_optional(a)?.unwrap_or(0);
    let b = parse_optional(b)?.unwrap_or(0);
    Ok(a + b)
}

pub fn parse_port(raw: Option<&str>) -> Result<u16, String> {
    let port = raw
        .map(|s| {
            s.parse::<u16>()
                .map_err(|e| format!("invalid port `{s}`: {e}"))
        })
        .transpose()?;
    Ok(port.unwrap_or(8080))
}
```

Full tested solution: [`src/solutions/p0410_transpose_optional_number.rs`](../src/solutions/p0410_transpose_optional_number.rs)

</details>
