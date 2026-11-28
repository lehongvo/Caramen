# Problem 0401: Safe Division with Option

- **Difficulty:** Easy
- **Topics:** `Option<T>`, `checked_div`, `unwrap_or`, `unwrap_or_default`, `match` on `Option`
- **Source:** Original · [std `Option`](https://doc.rust-lang.org/std/option/enum.Option.html) · [`i32::checked_div`](https://doc.rust-lang.org/std/primitive.i32.html#method.checked_div)

## Problem

The expression `a / b` panics when `b` is `0`, and `i32::MIN / -1` panics too, because the
true answer (`2147483648`) does not fit in an `i32`. Instead of crashing, a careful program
returns an `Option<i32>`: `Some(quotient)` when the division is possible and `None` otherwise.

Implement four small functions:

- `safe_div(a: i32, b: i32) -> Option<i32>`: `Some(a / b)` (integer division that truncates
  toward zero, like the `/` operator), or `None` when the division would panic.
- `div_or(a: i32, b: i32, fallback: i32) -> i32`: the quotient, or `fallback` when
  `safe_div` returns `None`.
- `div_or_zero(a: i32, b: i32) -> i32`: the quotient, or `0` (the default value of `i32`).
- `describe_div(a: i32, b: i32) -> String`: `"a / b = q"` when the division works, otherwise
  `"a / b is undefined"` (with the real numbers in place of `a`, `b` and `q`).

## Examples

```text
safe_div(7, 2)                -> Some(3)
safe_div(-7, 2)               -> Some(-3)     (truncates toward zero)
safe_div(7, 0)                -> None
safe_div(i32::MIN, -1)        -> None         (the result would overflow)
div_or(7, 0, -1)              -> -1
div_or_zero(9, 3)             -> 3
div_or_zero(9, 0)             -> 0
describe_div(10, 4)           -> "10 / 4 = 2"
describe_div(10, 0)           -> "10 / 0 is undefined"
```

## Constraints

- `a`, `b` and `fallback` are any `i32` values.
- None of the functions may panic.

## Hints

1. Integer types have `checked_*` methods that return `Option` instead of panicking.
2. `Option::unwrap_or(value)` returns the content of `Some` or `value` for `None`.
   `unwrap_or_default()` does the same with the type's `Default` value.
3. A `match` with the arms `Some(q) => ...` and `None => ...` handles both cases explicitly.

## Starter code

Write your code in [`src/practice/p0401_safe_division.rs`](../src/practice/p0401_safe_division.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0401
```

```rust
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    todo!("Write your solution here")
}

pub fn div_or(a: i32, b: i32, fallback: i32) -> i32 {
    todo!("Write your solution here")
}

pub fn div_or_zero(a: i32, b: i32) -> i32 {
    todo!("Write your solution here")
}

pub fn describe_div(a: i32, b: i32) -> String {
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
    fn safe_div_examples() {
        assert_eq!(safe_div(7, 2), Some(3));
        assert_eq!(safe_div(-7, 2), Some(-3));
        assert_eq!(safe_div(0, 5), Some(0));
    }

    #[test]
    fn safe_div_returns_none_instead_of_panicking() {
        assert_eq!(safe_div(7, 0), None);
        assert_eq!(safe_div(i32::MIN, -1), None);
        assert_eq!(safe_div(i32::MIN, 1), Some(i32::MIN));
    }

    #[test]
    fn div_or_uses_the_fallback_only_for_none() {
        assert_eq!(div_or(7, 0, -1), -1);
        assert_eq!(div_or(8, 2, -1), 4);
        assert_eq!(div_or(i32::MIN, -1, 42), 42);
    }

    #[test]
    fn div_or_zero_uses_the_default_value() {
        assert_eq!(div_or_zero(9, 3), 3);
        assert_eq!(div_or_zero(9, 0), 0);
        assert_eq!(div_or_zero(-9, 4), -2);
    }

    #[test]
    fn describe_div_both_cases() {
        assert_eq!(describe_div(10, 4), "10 / 4 = 2");
        assert_eq!(describe_div(10, 0), "10 / 0 is undefined");
        assert_eq!(describe_div(-3, -3), "-3 / -3 = 1");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `i32::checked_div` already does exactly what `safe_div` needs: it returns `None` for
a zero divisor and for the single overflowing case `i32::MIN / -1`. The other functions only
decide what to do with the `Option`: `unwrap_or` and `unwrap_or_default` pick a replacement
value, and `describe_div` uses `match` to build a different message for each variant.

**Rust concepts:**
- `Option<T>` is an enum with two variants, `Some(T)` and `None`; the compiler forces you to
  handle `None` before you can use the value.
- `unwrap_or(x)` evaluates `x` even when the option is `Some`; that is fine for a plain number.
- `unwrap_or_default()` needs `T: Default`; for every integer type the default is `0`.
- Reusing `safe_div` inside the other functions keeps the "can this panic?" logic in one place.

```rust
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    a.checked_div(b)
}

pub fn div_or(a: i32, b: i32, fallback: i32) -> i32 {
    safe_div(a, b).unwrap_or(fallback)
}

pub fn div_or_zero(a: i32, b: i32) -> i32 {
    safe_div(a, b).unwrap_or_default()
}

pub fn describe_div(a: i32, b: i32) -> String {
    match safe_div(a, b) {
        Some(q) => format!("{a} / {b} = {q}"),
        None => format!("{a} / {b} is undefined"),
    }
}
```

Full tested solution: [`src/solutions/p0401_safe_division.rs`](../src/solutions/p0401_safe_division.rs)

</details>
