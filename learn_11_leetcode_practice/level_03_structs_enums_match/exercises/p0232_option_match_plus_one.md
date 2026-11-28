# Problem 0232: Matching on Option

- **Difficulty:** Easy
- **Topics:** `match` on `Option<T>`, `Some(x)` / `None` patterns, matching a tuple of two `Option`s, `Option<&str>`
- **Source:** Original · [The Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont)

## Problem

Practise writing `match` expressions on `Option` by hand. Use `match` in every function (no
`map`, `and_then`, `unwrap_or`... yet; the next problems cover those).

- `plus_one(x: Option<i32>) -> Option<i32>`: `None` stays `None`; `Some(n)` becomes `Some(n + 1)`,
  except that `Some(i32::MAX)` becomes `None` because the result would overflow;
- `greet(name: Option<&str>) -> String`: `"Hello, <name>!"` for a name, `"Hello, stranger!"` for
  `None`, and also `"Hello, stranger!"` for `Some("")` (an empty name);
- `add_both(a: Option<i32>, b: Option<i32>) -> Option<i32>`: the sum if **both** are present,
  otherwise `None`;
- `sum_present(values: &[Option<i32>]) -> (usize, i64)`: how many values are `Some`, and the sum
  of those values (as `i64`).

## Examples

```text
plus_one(Some(5))                         => Some(6)
plus_one(None)                            => None
plus_one(Some(i32::MAX))                  => None
greet(Some("Ana"))                        => "Hello, Ana!"
greet(None)                               => "Hello, stranger!"
add_both(Some(2), Some(3))                => Some(5)
add_both(Some(2), None)                   => None
sum_present(&[Some(1), None, Some(-4)])   => (2, -3)
```

## Constraints

- `values.len() <= 10^5`.

## Hints

1. A `match` on `Option` needs both a `Some(..)` arm and a `None` arm; leaving one out is a
   compile error ("non-exhaustive patterns").
2. Patterns can contain literals: `Some(i32::MAX) => None` must come before `Some(n) => ...`.
3. Match on a tuple to handle two options at once: `match (a, b) { (Some(x), Some(y)) => ..., _ => ... }`.

## Starter code

Write your code in [`src/practice/p0232_option_match_plus_one.rs`](../src/practice/p0232_option_match_plus_one.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0232
```

```rust
/// `n + 1` inside the option, or `None` on overflow.
pub fn plus_one(x: Option<i32>) -> Option<i32> {
    todo!("Write your solution here")
}

/// A greeting for an optional name.
pub fn greet(name: Option<&str>) -> String {
    todo!("Write your solution here")
}

/// `a + b` if both are present.
pub fn add_both(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!("Write your solution here")
}

/// The number of `Some` values and their sum.
pub fn sum_present(values: &[Option<i32>]) -> (usize, i64) {
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
    fn plus_one_cases() {
        assert_eq!(plus_one(Some(5)), Some(6));
        assert_eq!(plus_one(Some(-1)), Some(0));
        assert_eq!(plus_one(None), None);
        assert_eq!(plus_one(Some(i32::MAX)), None);
    }

    #[test]
    fn greetings() {
        assert_eq!(greet(Some("Ana")), "Hello, Ana!");
        assert_eq!(greet(None), "Hello, stranger!");
        assert_eq!(greet(Some("")), "Hello, stranger!");
        assert_eq!(greet(Some(" ")), "Hello,  !");
    }

    #[test]
    fn adding_two_options() {
        assert_eq!(add_both(Some(2), Some(3)), Some(5));
        assert_eq!(add_both(Some(2), None), None);
        assert_eq!(add_both(None, Some(3)), None);
        assert_eq!(add_both(None, None), None);
    }

    #[test]
    fn summing_present_values() {
        assert_eq!(sum_present(&[Some(1), None, Some(-4)]), (2, -3));
        assert_eq!(sum_present(&[]), (0, 0));
        assert_eq!(sum_present(&[None, None]), (0, 0));
    }

    #[test]
    fn summing_many_values_uses_i64() {
        let values: Vec<Option<i32>> = (0..100_000)
            .map(|i| if i % 2 == 0 { Some(i32::MAX) } else { None })
            .collect();
        assert_eq!(sum_present(&values), (50_000, 50_000 * i64::from(i32::MAX)));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each function is a direct translation of its rules into `match` arms, most specific
arm first.

**Rust concepts:**
- `match` is exhaustive: the compiler checks that `Some` and `None` are both handled, which is
  how Rust avoids "null pointer" bugs.
- Constants such as `i32::MAX` and string literals such as `""` can be used inside patterns.
- `(Some(x), Some(y))` matches a tuple of two options in one go; `_` then covers the three
  other combinations.
- `for v in values` over `&[Option<i32>]` yields `&Option<i32>`; `match v { Some(x) => ... }`
  binds `x: &i32` thanks to default binding modes.

```rust
/// `n + 1` inside the option, or `None` on overflow.
pub fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i32::MAX) => None,
        Some(n) => Some(n + 1),
        None => None,
    }
}

/// A greeting for an optional name.
pub fn greet(name: Option<&str>) -> String {
    match name {
        None | Some("") => String::from("Hello, stranger!"),
        Some(n) => format!("Hello, {n}!"),
    }
}

/// `a + b` if both are present.
pub fn add_both(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        _ => None,
    }
}

/// The number of `Some` values and their sum.
pub fn sum_present(values: &[Option<i32>]) -> (usize, i64) {
    let mut count = 0;
    let mut sum = 0i64;
    for v in values {
        let (seen, value) = match v {
            Some(x) => (1, i64::from(*x)),
            None => (0, 0),
        };
        count += seen;
        sum += value;
    }
    (count, sum)
}
```

Full tested solution: [`src/solutions/p0232_option_match_plus_one.rs`](../src/solutions/p0232_option_match_plus_one.rs)

</details>
