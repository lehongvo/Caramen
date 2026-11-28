# Problem 0013: Leap Year

- **Difficulty:** Easy
- **Topics:** `bool`, logical operators `&&`, `||`, `!`, `%`
- **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type)

## Problem

In the Gregorian calendar a year has 366 days (a **leap year**) when:

- it is divisible by 4, **and**
- it is **not** divisible by 100, **unless** it is also divisible by 400.

So 2024 is a leap year, 1900 is not (divisible by 100 but not by 400) and 2000 is (divisible by
400). Write `is_leap_year(year: u32) -> bool`.

## Examples

```text
is_leap_year(2024) -> true
is_leap_year(2023) -> false
is_leap_year(1900) -> false
is_leap_year(2000) -> true
```

## Constraints

- `1 <= year <= 10^9`

## Hints

1. Translate each rule into a boolean expression such as `year % 4 == 0`.
2. "not divisible by 100 unless divisible by 400" is the same as
   "not divisible by 100, **or** divisible by 400".
3. Try to write the whole function as a single expression, without `if`.

## Starter code

Write your code in [`src/practice/p0013_leap_year.rs`](../src/practice/p0013_leap_year.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0013
```

```rust
pub fn is_leap_year(year: u32) -> bool {
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
    fn divisible_by_four() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(4));
    }

    #[test]
    fn not_divisible_by_four() {
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(1));
    }

    #[test]
    fn centuries() {
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(1600));
    }

    #[test]
    fn four_hundred_year_cycle_has_97_leap_years() {
        let count = (1..=400).filter(|&y| is_leap_year(y)).count();
        assert_eq!(count, 97);
    }

    #[test]
    fn large_years() {
        assert!(is_leap_year(1_000_000_000));
        assert!(!is_leap_year(999_999_900));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a year is a leap year when `year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)`.

**Rust concepts:**
- Comparison operators (`==`, `!=`, `<`, ...) produce a `bool`, and `&&` / `||` combine
  booleans. Both are **short-circuiting**: in `a && b`, `b` is not evaluated when `a` is false.
- `&&` binds tighter than `||`, so the parentheses around the `||` part are required here.
- A function that answers a yes/no question can simply return the boolean expression; writing
  `if cond { true } else { false }` is redundant (Clippy warns about it).

```rust
pub fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
```

Full tested solution: [`src/solutions/p0013_leap_year.rs`](../src/solutions/p0013_leap_year.rs)

</details>
