# Problem 0413: Add Two Number Strings with `?`

- **Difficulty:** Easy
- **Topics:** the `?` operator on `Result`, `Result::map`, `ParseIntError`, `i64::from`
- **Source:** Original · [The Rust Book 9.2: A shortcut for propagating errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)

## Problem

When a function calls something that can fail and cannot fix the failure itself, it should
*propagate* the error to its caller. Writing `match` for every call is noisy; the `?` operator
does it for you: `expr?` evaluates to the `Ok` value, or returns the `Err` from the function.

Implement, all returning `Result<i64, ParseIntError>`:

- `add_strs(a: &str, b: &str)`: parse both strings (after trimming surrounding whitespace) as
  `i32` and return their sum as an `i64`. If `a` is invalid return its error; otherwise if `b`
  is invalid return its error.
- `double_str(s: &str)`: the trimmed `i32` value of `s` times two, as an `i64`.
- `sum_csv(line: &str)`: split `line` on `','`, parse every field (trimmed) as an `i32`, and
  return the total as an `i64`. The first invalid field stops the function with its error. An
  empty line has one empty field, so it is an error too.

## Examples

```text
add_strs("40", " 2 ")             -> Ok(42)
add_strs("2147483647", "1")       -> Ok(2147483648)     (the sum is computed in i64)
add_strs("4o", "2")               -> Err(InvalidDigit)  (from "4o")
double_str("-21")                 -> Ok(-42)
sum_csv("1, 2, 3")                -> Ok(6)
sum_csv("1,,3")                   -> Err(Empty)
```

## Constraints

- Valid numbers fit in an `i32`; `sum_csv` lines have at most `10^4` fields.

## Hints

1. `let x = a.trim().parse::<i32>()?;` either gives you the number or leaves the function.
2. `Result::map(|n| ...)` transforms the success value and leaves an error untouched.
3. A `for` loop can contain `?` too: the first failing field returns immediately.

## Starter code

Write your code in [`src/practice/p0413_add_number_strings.rs`](../src/practice/p0413_add_number_strings.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0413
```

```rust
use std::num::ParseIntError;

pub fn add_strs(a: &str, b: &str) -> Result<i64, ParseIntError> {
    todo!("Write your solution here")
}

pub fn double_str(s: &str) -> Result<i64, ParseIntError> {
    todo!("Write your solution here")
}

pub fn sum_csv(line: &str) -> Result<i64, ParseIntError> {
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
    fn add_valid_strings() {
        assert_eq!(add_strs("40", " 2 "), Ok(42));
        assert_eq!(add_strs("-5", "-6"), Ok(-11));
        assert_eq!(add_strs("2147483647", "1"), Ok(2_147_483_648));
    }

    #[test]
    fn add_reports_the_first_bad_argument() {
        assert_eq!(
            add_strs("4o", "2").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit
        );
        assert_eq!(add_strs("4", "").unwrap_err().kind(), &IntErrorKind::Empty);
        assert_eq!(add_strs("", "x").unwrap_err().kind(), &IntErrorKind::Empty);
    }

    #[test]
    fn double_maps_the_success_value() {
        assert_eq!(double_str("-21"), Ok(-42));
        assert_eq!(double_str(" 2147483647 "), Ok(4_294_967_294));
        assert_eq!(
            double_str("ten").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit
        );
    }

    #[test]
    fn sum_csv_valid_lines() {
        assert_eq!(sum_csv("1, 2, 3"), Ok(6));
        assert_eq!(sum_csv("7"), Ok(7));
        assert_eq!(sum_csv("2147483647,2147483647"), Ok(4_294_967_294));
    }

    #[test]
    fn sum_csv_stops_at_the_first_bad_field() {
        assert_eq!(sum_csv("1,,3").unwrap_err().kind(), &IntErrorKind::Empty);
        assert_eq!(sum_csv("").unwrap_err().kind(), &IntErrorKind::Empty);
        assert_eq!(
            sum_csv("1,99999999999,x").unwrap_err().kind(),
            &IntErrorKind::PosOverflow
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every function follows the happy path and marks each fallible step with `?`.
`double_str` shows the alternative style: instead of `?` it transforms the `Result` with `map`.

**Rust concepts:**
- `?` works in any function whose return type is a `Result` with a compatible error type; here
  both are `ParseIntError`, so the error passes through unchanged.
- `i64::from(n)` is a lossless conversion from `i32`; doing the arithmetic in `i64` avoids
  overflow when adding two large `i32` values.
- `Result::map` is the `Result` cousin of `Option::map`: it only touches the `Ok` value.

```rust
use std::num::ParseIntError;

pub fn add_strs(a: &str, b: &str) -> Result<i64, ParseIntError> {
    let a = a.trim().parse::<i32>()?;
    let b = b.trim().parse::<i32>()?;
    Ok(i64::from(a) + i64::from(b))
}

pub fn double_str(s: &str) -> Result<i64, ParseIntError> {
    s.trim().parse::<i32>().map(|n| i64::from(n) * 2)
}

pub fn sum_csv(line: &str) -> Result<i64, ParseIntError> {
    let mut total = 0i64;
    for field in line.split(',') {
        total += i64::from(field.trim().parse::<i32>()?);
    }
    Ok(total)
}
```

Full tested solution: [`src/solutions/p0413_add_number_strings.rs`](../src/solutions/p0413_add_number_strings.rs)

</details>
