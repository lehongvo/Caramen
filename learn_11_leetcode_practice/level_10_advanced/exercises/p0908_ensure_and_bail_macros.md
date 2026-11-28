# Problem 0908: Early Returns: `ensure!` and `bail!`

- **Difficulty:** Medium
- **Topics:** macros that `return`, forwarding format arguments with `$($arg:tt)+`, `stringify!`, `From`
- **Source:** Original · [Reference: Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html) · [std: `stringify!`](https://doc.rust-lang.org/std/macro.stringify.html)

## Problem

Validation code is full of `if !condition { return Err(...); }`. Popular error crates shorten it
with two macros; write them yourself for the error type

```text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub String);
```

- `bail!("format string", args...)` returns early from the **enclosing function** with
  `Err(ValidationError(format!(...)))`. Convert through `From::from` so the enclosing function may
  return any `Result<_, E>` where `E: From<ValidationError>`.
- `ensure!(condition, "format string", args...)` bails with that message when `condition` is
  false and does nothing otherwise.
- `ensure!(condition)` (no message) bails with `"condition failed: <the condition's source text>"`,
  e.g. `"condition failed: name.is_ascii()"`.

Then write two validators with your macros:

- `parse_age(text: &str) -> Result<u8, ValidationError>`: trims the text, then
  `"age is empty"` if it is empty, `"age {text:?} is not a number"` if it does not parse as `u8`
  (use the trimmed text), `"age {age} is out of range"` if it is greater than `150`.
- `validate_username(name: &str) -> Result<(), ValidationError>`, in this order:
  `ensure!(name.is_ascii())`, then at least 3 characters (`"username {name:?} is too short"`),
  then only ASCII letters, digits or `_` (`"username {name:?} contains {c:?}"` for the first bad
  character `c`).

## Examples

```text
parse_age(" 42 ")        => Ok(42)
parse_age("  ")          => Err(ValidationError("age is empty"))
parse_age("abc")         => Err(ValidationError("age \"abc\" is not a number"))
parse_age("200")         => Err(ValidationError("age 200 is out of range"))
validate_username("zoë")  => Err(ValidationError("condition failed: name.is_ascii()"))
validate_username("ab")   => Err(ValidationError("username \"ab\" is too short"))
validate_username("a-b-c") => Err(ValidationError("username \"a-b-c\" contains '-'"))
```

## Constraints

- No special constraints.

## Hints

1. A macro expansion is pasted into the caller, so a `return` inside it returns from the
   **caller's** function.
2. `($($arg:tt)+) => { ... format!($($arg)+) ... }` forwards everything, including inline
   arguments like `{name:?}`.
3. Put the more specific `ensure!` arm (with a message) first, and build the default message with
   `stringify!($cond)`.

## Starter code

Write your code in [`src/practice/p0908_ensure_and_bail_macros.rs`](../src/practice/p0908_ensure_and_bail_macros.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0908
```

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub String);

macro_rules! bail {
    ($($t:tt)*) => {
        todo!()
    };
}

macro_rules! ensure {
    ($($t:tt)*) => {
        todo!()
    };
}

pub fn parse_age(text: &str) -> Result<u8, ValidationError> {
    todo!("Write your solution here")
}

pub fn validate_username(name: &str) -> Result<(), ValidationError> {
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

    fn err(message: &str) -> ValidationError {
        ValidationError(message.to_string())
    }

    #[test]
    fn parse_age_examples() {
        assert_eq!(parse_age(" 42 "), Ok(42));
        assert_eq!(parse_age("150"), Ok(150));
        assert_eq!(parse_age("  "), Err(err("age is empty")));
        assert_eq!(parse_age("abc"), Err(err("age \"abc\" is not a number")));
        assert_eq!(parse_age("200"), Err(err("age 200 is out of range")));
        assert_eq!(parse_age("300"), Err(err("age \"300\" is not a number")));
    }

    #[test]
    fn validate_username_examples() {
        assert_eq!(validate_username("ada_99"), Ok(()));
        assert_eq!(
            validate_username("zoë"),
            Err(err("condition failed: name.is_ascii()"))
        );
        assert_eq!(
            validate_username("ab"),
            Err(err("username \"ab\" is too short"))
        );
        assert_eq!(
            validate_username("a-b-c"),
            Err(err("username \"a-b-c\" contains '-'"))
        );
    }

    /// The macros work in any function whose error type can be built `From<ValidationError>`.
    #[derive(Debug, PartialEq)]
    struct AppError(String);

    impl From<ValidationError> for AppError {
        fn from(e: ValidationError) -> Self {
            AppError(format!("invalid input: {}", e.0))
        }
    }

    fn checked_div(a: i32, b: i32) -> Result<i32, AppError> {
        ensure!(b != 0, "cannot divide {a} by zero");
        ensure!(a % b == 0);
        Ok(a / b)
    }

    #[test]
    fn errors_convert_with_from() {
        assert_eq!(checked_div(12, 4), Ok(3));
        assert_eq!(
            checked_div(1, 0),
            Err(AppError(
                "invalid input: cannot divide 1 by zero".to_string()
            ))
        );
        assert_eq!(
            checked_div(7, 2),
            Err(AppError(
                "invalid input: condition failed: a % b == 0".to_string()
            ))
        );
    }

    fn first_negative(values: &[i32]) -> Result<usize, ValidationError> {
        for (i, &v) in values.iter().enumerate() {
            if v < 0 {
                bail!("value #{} is negative ({})", i, v);
            }
        }
        Ok(values.len())
    }

    #[test]
    fn bail_returns_from_the_enclosing_function() {
        assert_eq!(first_negative(&[1, 2, 3]), Ok(3));
        assert_eq!(
            first_negative(&[4, -1, -2]),
            Err(err("value #1 is negative (-1)"))
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `bail!` expands to `return Err(From::from(ValidationError(format!(...))))`.
`ensure!` expands to `if !(cond) { bail!(...); }`; its message-less arm calls itself with the
message `"condition failed: {}"` and `stringify!($cond)` as the argument.

**Rust concepts:**
- Macros can contain control flow (`return`, `?`, `break`) because they are expanded in place,
  which is something a function can never do to its caller.
- `$($arg:tt)+` captures the format string and its arguments as raw tokens so they can be handed
  to `format!` unchanged.
- `stringify!` turns tokens into a string literal at compile time; for a captured `$cond:expr` it
  prints the expression's source.
- Wrapping the condition in parentheses, `!($cond)`, keeps operator precedence right for
  conditions like `a || b`.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub String);

macro_rules! bail {
    ($($arg:tt)+) => {
        return ::std::result::Result::Err(::std::convert::From::from(ValidationError(format!($($arg)+))))
    };
}

macro_rules! ensure {
    ($cond:expr, $($arg:tt)+) => {
        if !($cond) {
            bail!($($arg)+);
        }
    };
    ($cond:expr $(,)?) => {
        ensure!($cond, "condition failed: {}", stringify!($cond))
    };
}

pub fn parse_age(text: &str) -> Result<u8, ValidationError> {
    let text = text.trim();
    ensure!(!text.is_empty(), "age is empty");
    let age: u8 = match text.parse() {
        Ok(age) => age,
        Err(_) => bail!("age {text:?} is not a number"),
    };
    ensure!(age <= 150, "age {age} is out of range");
    Ok(age)
}

pub fn validate_username(name: &str) -> Result<(), ValidationError> {
    ensure!(name.is_ascii());
    ensure!(name.len() >= 3, "username {name:?} is too short");
    if let Some(c) = name
        .chars()
        .find(|c| !(c.is_ascii_alphanumeric() || *c == '_'))
    {
        bail!("username {name:?} contains {c:?}");
    }
    Ok(())
}
```

Full tested solution: [`src/solutions/p0908_ensure_and_bail_macros.rs`](../src/solutions/p0908_ensure_and_bail_macros.rs)

</details>
