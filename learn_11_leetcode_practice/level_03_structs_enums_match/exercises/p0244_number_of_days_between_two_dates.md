# Problem 0244: Number of Days Between Two Dates

- **Difficulty:** Easy
- **Topics:** a `Date` struct with a "days since epoch" method, parsing with `?`, `abs_diff`, `match` with a guard
- **Source:** [LeetCode 1360. Number of Days Between Two Dates](https://leetcode.com/problems/number-of-days-between-two-dates/)

## Problem

Given two valid dates `date1` and `date2` as `"YYYY-MM-DD"` strings, return the number of days
between them: the absolute difference, so the order of the arguments does not matter. Use the
Gregorian leap-year rule (divisible by 4, except centuries not divisible by 400).

Implement `Solution::days_between_dates(date1: String, date2: String) -> i32`.

For this level, write a `Date` struct with a method `days_since_1971(&self) -> u32` (the number
of days from January 1st, 1971 to the date), and compute the answer from two such numbers.

## Examples

```text
Input:  date1 = "2019-06-29", date2 = "2019-06-30"
Output: 1

Input:  date1 = "2020-01-15", date2 = "2019-12-31"
Output: 15

Input:  date1 = "1971-01-01", date2 = "2100-12-31"
Output: 47481
```

## Constraints

- Both dates are valid and lie between the years `1971` and `2100` (inclusive).

## Hints

1. Count whole years from 1971 up to (not including) the date's year, then whole months of the
   date's year, then the day.
2. Converting both dates to a single number turns "difference of dates" into a subtraction.

## Starter code

Write your code in [`src/practice/p0244_number_of_days_between_two_dates.rs`](../src/practice/p0244_number_of_days_between_two_dates.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0244
```

```rust
pub struct Solution;

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn days(a: &str, b: &str) -> i32 {
        Solution::days_between_dates(a.to_string(), b.to_string())
    }

    #[test]
    fn example_consecutive_days() {
        assert_eq!(days("2019-06-29", "2019-06-30"), 1);
    }

    #[test]
    fn example_across_new_year_reversed() {
        assert_eq!(days("2020-01-15", "2019-12-31"), 15);
    }

    #[test]
    fn whole_range() {
        assert_eq!(days("1971-01-01", "2100-12-31"), 47_481);
    }

    #[test]
    fn same_date_and_leap_days() {
        assert_eq!(days("2000-02-28", "2000-02-28"), 0);
        assert_eq!(days("2000-02-28", "2000-03-01"), 2);
        assert_eq!(days("2100-02-28", "2100-03-01"), 1);
    }

    #[test]
    fn full_years() {
        assert_eq!(days("2019-01-01", "2020-01-01"), 365);
        assert_eq!(days("2020-01-01", "2021-01-01"), 366);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** map each date to its day number since 1971-01-01 (that date is day `0`), then return
the absolute difference of the two numbers.

**Complexity:** `O(Y)` time per date, where `Y <= 130` is the number of years since 1971; `O(1)` space.

**Rust concepts:**
- A struct method that reduces a value to a canonical number (`days_since_1971`) makes
  comparisons and differences trivial.
- `u32::abs_diff(a, b)` returns `|a - b|` without the risk of an unsigned underflow.
- `days_in_month` uses a match guard: `2 if is_leap(year) => 29` must come before `2 => 28`.
- `Option` plus `?` in `Date::parse` keeps the parser short; the result is unwrapped with
  `map_or` in `days_between_dates`, since LeetCode guarantees valid input.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

impl Date {
    fn parse(s: &str) -> Option<Date> {
        let mut parts = s.split('-');
        let year = parts.next()?.parse().ok()?;
        let month = parts.next()?.parse().ok()?;
        let day = parts.next()?.parse().ok()?;
        Some(Date { year, month, day })
    }

    fn days_since_1971(&self) -> u32 {
        let years: u32 = (1971..self.year)
            .map(|y| if is_leap(y) { 366 } else { 365 })
            .sum();
        let months: u32 = (1..self.month).map(|m| days_in_month(self.year, m)).sum();
        years + months + self.day - 1
    }
}

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        match (Date::parse(&date1), Date::parse(&date2)) {
            // At most about 47_500 days, which fits in an `i32`.
            (Some(a), Some(b)) => a.days_since_1971().abs_diff(b.days_since_1971()) as i32,
            _ => 0,
        }
    }
}
```

Full tested solution: [`src/solutions/p0244_number_of_days_between_two_dates.rs`](../src/solutions/p0244_number_of_days_between_two_dates.rs)

</details>
