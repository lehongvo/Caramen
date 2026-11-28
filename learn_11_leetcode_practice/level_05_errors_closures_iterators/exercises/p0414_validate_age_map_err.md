# Problem 0414: Validate an Age with map_err and and_then

- **Difficulty:** Easy
- **Topics:** `Result::map_err`, `Result::and_then`, `Result::map`, `Result::or_else`, `String` errors
- **Source:** Original · [Rust By Example: `map` for `Result`](https://doc.rust-lang.org/rust-by-example/error/result/result_map.html)

## Problem

`Result` has the same family of combinators as `Option`. They let you build a validation
pipeline without a single `match`:

- `map_err(f)` changes the **error** value (for example into a friendly message);
- `and_then(f)` runs another step that can fail;
- `map(f)` transforms the success value;
- `or_else(f)` gets a chance to recover from an error.

Implement:

- `parse_age(s: &str) -> Result<u8, String>`: trim `s` and parse it as a `u8`. A parse failure
  becomes ``"`TRIMMED` is not a valid age: REASON"`` (`REASON` = the `ParseIntError`'s
  `Display` text). A parsed age above `150` becomes `"AGE is not a realistic age"`.
- `age_group(s: &str) -> Result<&'static str, String>`: `"child"` (0-12), `"teen"` (13-17),
  `"adult"` (18-64) or `"senior"` (65 and up), or the error of `parse_age`.
- `can_vote(s: &str) -> Result<bool, String>`: whether the age is at least 18.
- `parse_age_or_newborn(s: &str) -> Result<u8, String>`: like `parse_age`, but if it fails and
  the trimmed input is exactly `"newborn"`, the age is `0`. Other errors stay unchanged.

## Examples

```text
parse_age(" 42 ")               -> Ok(42)
parse_age("abc")                -> Err("`abc` is not a valid age: invalid digit found in string")
parse_age("200")                -> Err("200 is not a realistic age")
parse_age("300")                -> Err("`300` is not a valid age: number too large to fit in target type")
age_group("15")                 -> Ok("teen")
can_vote("18")                  -> Ok(true)
parse_age_or_newborn("newborn") -> Ok(0)
```

## Constraints

- No special constraints.

## Hints

1. Start with `let s = s.trim();` and then `s.parse::<u8>().map_err(|e| format!(...))`.
2. Chain `.and_then(|age| if age <= 150 { Ok(age) } else { Err(...) })`.
3. `age_group` and `can_vote` are one `map` call on top of `parse_age`.

## Starter code

Write your code in [`src/practice/p0414_validate_age_map_err.rs`](../src/practice/p0414_validate_age_map_err.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0414
```

```rust
pub fn parse_age(s: &str) -> Result<u8, String> {
    todo!("Write your solution here")
}

pub fn age_group(s: &str) -> Result<&'static str, String> {
    todo!("Write your solution here")
}

pub fn can_vote(s: &str) -> Result<bool, String> {
    todo!("Write your solution here")
}

pub fn parse_age_or_newborn(s: &str) -> Result<u8, String> {
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
    fn parse_valid_ages() {
        assert_eq!(parse_age(" 42 "), Ok(42));
        assert_eq!(parse_age("0"), Ok(0));
        assert_eq!(parse_age("150"), Ok(150));
    }

    #[test]
    fn parse_errors_have_messages() {
        assert_eq!(
            parse_age("abc"),
            Err("`abc` is not a valid age: invalid digit found in string".to_string())
        );
        assert_eq!(
            parse_age("200"),
            Err("200 is not a realistic age".to_string())
        );
        assert_eq!(
            parse_age("300"),
            Err("`300` is not a valid age: number too large to fit in target type".to_string())
        );
        assert_eq!(
            parse_age("   "),
            Err("`` is not a valid age: cannot parse integer from empty string".to_string())
        );
    }

    #[test]
    fn age_groups() {
        assert_eq!(age_group("0"), Ok("child"));
        assert_eq!(age_group("12"), Ok("child"));
        assert_eq!(age_group("15"), Ok("teen"));
        assert_eq!(age_group("18"), Ok("adult"));
        assert_eq!(age_group("64"), Ok("adult"));
        assert_eq!(age_group("65"), Ok("senior"));
        assert_eq!(
            age_group("151"),
            Err("151 is not a realistic age".to_string())
        );
    }

    #[test]
    fn voting_age() {
        assert_eq!(can_vote("18"), Ok(true));
        assert_eq!(can_vote("17"), Ok(false));
        assert!(can_vote("-1").is_err());
    }

    #[test]
    fn newborn_is_the_only_recovered_error() {
        assert_eq!(parse_age_or_newborn("newborn"), Ok(0));
        assert_eq!(parse_age_or_newborn(" newborn "), Ok(0));
        assert_eq!(parse_age_or_newborn("7"), Ok(7));
        assert_eq!(
            parse_age_or_newborn("baby"),
            Err("`baby` is not a valid age: invalid digit found in string".to_string())
        );
        assert_eq!(
            parse_age_or_newborn("180"),
            Err("180 is not a realistic age".to_string())
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `parse_age` is a two-step pipeline (parse, then range check). Everything else reuses
it: `map` for the derived answers and `or_else` for the single recoverable error.

**Rust concepts:**
- `map_err` keeps the success value and converts the error; closures can capture `s` to put the
  offending input into the message.
- `and_then` is for steps that return a `Result` themselves; `map` is for steps that cannot fail.
- A `match` on a `u8` with range patterns (`0..=12`, `13..=17`, ...) is exhaustive once the last
  arm is `_`.
- `or_else(|err| ...)` receives the error and returns a new `Result`: `Ok(0)` to recover, or
  `Err(err)` to keep failing with the original message.

```rust
pub fn parse_age(s: &str) -> Result<u8, String> {
    let s = s.trim();
    s.parse::<u8>()
        .map_err(|e| format!("`{s}` is not a valid age: {e}"))
        .and_then(|age| {
            if age <= 150 {
                Ok(age)
            } else {
                Err(format!("{age} is not a realistic age"))
            }
        })
}

pub fn age_group(s: &str) -> Result<&'static str, String> {
    parse_age(s).map(|age| match age {
        0..=12 => "child",
        13..=17 => "teen",
        18..=64 => "adult",
        _ => "senior",
    })
}

pub fn can_vote(s: &str) -> Result<bool, String> {
    parse_age(s).map(|age| age >= 18)
}

pub fn parse_age_or_newborn(s: &str) -> Result<u8, String> {
    parse_age(s).or_else(|err| {
        if s.trim() == "newborn" {
            Ok(0)
        } else {
            Err(err)
        }
    })
}
```

Full tested solution: [`src/solutions/p0414_validate_age_map_err.rs`](../src/solutions/p0414_validate_age_map_err.rs)

</details>
