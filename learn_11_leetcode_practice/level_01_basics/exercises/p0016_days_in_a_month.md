# Problem 0016: Days in a Month

- **Difficulty:** Easy
- **Topics:** `match` with `|` patterns, `if` inside a `match` arm, `Option`
- **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#multiple-patterns)

## Problem

Write `days_in_month(year: u32, month: u32) -> Option<u32>` that returns the number of days in
the given month of the given year (Gregorian calendar), or `None` when `month` is not between
1 and 12.

- January, March, May, July, August, October and December have 31 days.
- April, June, September and November have 30 days.
- February has 29 days in a leap year and 28 otherwise. A leap year is divisible by 4 and not by
  100, unless it is divisible by 400 (see problem 0013).

## Examples

```text
days_in_month(2023, 1)  -> Some(31)
days_in_month(2023, 4)  -> Some(30)
days_in_month(2024, 2)  -> Some(29)
days_in_month(1900, 2)  -> Some(28)
days_in_month(2023, 13) -> None
```

## Constraints

- `1 <= year <= 10^9`
- `0 <= month <= u32::MAX`

## Hints

1. One `match` arm can list several values separated by `|`: `4 | 6 | 9 | 11 => ...`.
2. Arms are expressions too, so the February arm can contain an `if`.

## Starter code

Write your code in [`src/practice/p0016_days_in_a_month.rs`](../src/practice/p0016_days_in_a_month.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0016
```

```rust
pub fn days_in_month(year: u32, month: u32) -> Option<u32> {
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
    fn long_and_short_months() {
        assert_eq!(days_in_month(2023, 1), Some(31));
        assert_eq!(days_in_month(2023, 4), Some(30));
        assert_eq!(days_in_month(2023, 12), Some(31));
        assert_eq!(days_in_month(2023, 11), Some(30));
    }

    #[test]
    fn february() {
        assert_eq!(days_in_month(2024, 2), Some(29));
        assert_eq!(days_in_month(2023, 2), Some(28));
        assert_eq!(days_in_month(1900, 2), Some(28));
        assert_eq!(days_in_month(2000, 2), Some(29));
    }

    #[test]
    fn invalid_months() {
        assert_eq!(days_in_month(2023, 0), None);
        assert_eq!(days_in_month(2023, 13), None);
        assert_eq!(days_in_month(2023, u32::MAX), None);
    }

    #[test]
    fn whole_year_totals() {
        let total = |year| {
            (1..=12)
                .map(|m| days_in_month(year, m).unwrap_or(0))
                .sum::<u32>()
        };
        assert_eq!(total(2023), 365);
        assert_eq!(total(2024), 366);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** match on the month. Group the 31-day and 30-day months with `|`, compute February
from the leap-year rule, and return `None` for everything else.

**Rust concepts:**
- `p1 | p2 | p3` is an "or pattern": the arm runs when any of them matches.
- The value of a `match` arm can be any expression, including `if ... { 29 } else { 28 }`.
- A private helper function (`is_leap`) keeps the `match` readable. Functions can be defined in
  any order in a file; Rust does not need forward declarations.

```rust
fn is_leap(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub fn days_in_month(year: u32, month: u32) -> Option<u32> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap(year) { 29 } else { 28 }),
        _ => None,
    }
}
```

Full tested solution: [`src/solutions/p0016_days_in_a_month.rs`](../src/solutions/p0016_days_in_a_month.rs)

</details>
