# Problem 0227: Weekdays with Explicit Discriminants

- **Difficulty:** Easy
- **Topics:** C-like enums with explicit discriminants, `as` casts from an enum, number to `Option<Enum>`, `rem_euclid`
- **Source:** Original · [The Rust Reference: Enumerations](https://doc.rust-lang.org/reference/items/enumerations.html)

## Problem

Use ISO numbering for the days of the week: `Monday = 1`, `Tuesday = 2`, ..., `Sunday = 7`. The
enum `Weekday` is declared with these **explicit discriminants**, so `Weekday::Friday as u8` is
`5`.

Implement:

- `number(self) -> u8`: the ISO number (`1..=7`);
- `Weekday::from_number(n: u8) -> Option<Weekday>`: the inverse, `None` outside `1..=7`;
- `is_weekend(self) -> bool`: `true` for Saturday and Sunday;
- `add_days(self, days: i64) -> Weekday`: the weekday `days` days later (earlier if negative);
- `days_until(self, other: Weekday) -> u8`: how many days forward from `self` until the next
  `other` (`0` if they are equal), always in `0..=6`.

## Examples

```text
Weekday::Friday.number()               => 5
Weekday::from_number(7)                => Some(Sunday)
Weekday::from_number(0)                => None
Weekday::Saturday.is_weekend()         => true
Weekday::Monday.add_days(10)           => Thursday
Weekday::Monday.add_days(-1)           => Sunday
Weekday::Friday.days_until(Weekday::Monday) => 3
```

## Constraints

- `|days| <= 10^12`.

## Hints

1. Casting an enum with `as` gives its discriminant. The other direction is not built in: write a
   `match` from `u8` to `Option<Weekday>`.
2. For `add_days`, work with the zero-based index `number - 1`, add, and use `rem_euclid(7)`.
3. `is_weekend` is a single `match` with `Weekday::Saturday | Weekday::Sunday`.

## Starter code

Write your code in [`src/practice/p0227_weekday_discriminants.rs`](../src/practice/p0227_weekday_discriminants.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0227
```

```rust
/// A day of the week with its ISO number as discriminant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

impl Weekday {
    /// The ISO number, `1..=7`.
    pub fn number(self) -> u8 {
        todo!("Write your solution here")
    }

    /// The weekday with ISO number `n`, or `None`.
    pub fn from_number(n: u8) -> Option<Weekday> {
        todo!("Write your solution here")
    }

    /// `true` for Saturday and Sunday.
    pub fn is_weekend(self) -> bool {
        todo!("Write your solution here")
    }

    /// The weekday `days` days later (earlier if negative).
    pub fn add_days(self, days: i64) -> Weekday {
        todo!("Write your solution here")
    }

    /// Days forward from `self` to the next `other`, in `0..=6`.
    pub fn days_until(self, other: Weekday) -> u8 {
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
    use super::Weekday::*;
    use super::*;

    #[test]
    fn numbers_round_trip() {
        assert_eq!(Friday.number(), 5);
        assert_eq!(Monday.number(), 1);
        for n in 1..=7 {
            let day = Weekday::from_number(n).expect("valid ISO number");
            assert_eq!(day.number(), n);
        }
    }

    #[test]
    fn invalid_numbers() {
        assert_eq!(Weekday::from_number(0), None);
        assert_eq!(Weekday::from_number(8), None);
        assert_eq!(Weekday::from_number(255), None);
        assert_eq!(Weekday::from_number(7), Some(Sunday));
    }

    #[test]
    fn weekend() {
        assert!(Saturday.is_weekend());
        assert!(Sunday.is_weekend());
        assert!(!Friday.is_weekend());
        assert!(!Monday.is_weekend());
    }

    #[test]
    fn adding_days() {
        assert_eq!(Monday.add_days(10), Thursday);
        assert_eq!(Monday.add_days(-1), Sunday);
        assert_eq!(Sunday.add_days(1), Monday);
        assert_eq!(Wednesday.add_days(0), Wednesday);
        assert_eq!(Wednesday.add_days(-15), Tuesday);
    }

    #[test]
    fn huge_offsets() {
        // 10^12 % 7 == 1, so 10^12 days later is one weekday later.
        assert_eq!(Monday.add_days(1_000_000_000_000), Tuesday);
        assert_eq!(Monday.add_days(-1_000_000_000_000), Sunday);
    }

    #[test]
    fn days_until_next() {
        assert_eq!(Friday.days_until(Monday), 3);
        assert_eq!(Monday.days_until(Friday), 4);
        assert_eq!(Sunday.days_until(Sunday), 0);
        assert_eq!(Sunday.days_until(Saturday), 6);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the discriminants give a cheap enum-to-number conversion. Every computation happens on
zero-based numbers in `0..7` with `rem_euclid`, and the result goes back through `from_number`.

**Rust concepts:**
- `enum Weekday { Monday = 1, ... }` sets explicit discriminants; `self as u8` reads them. This
  cast only works on enums whose variants carry no data.
- There is no automatic `u8 -> enum` conversion, because most numbers are not valid weekdays.
  Returning `Option<Weekday>` makes that explicit.
- `from_number` is only ever called with values in `1..=7` inside `add_days`, so the `expect` there
  documents an invariant instead of hiding a possible crash.
- `matches!(self, Weekday::Saturday | Weekday::Sunday)` would be a shorter `is_weekend`.

```rust
/// A day of the week with its ISO number as discriminant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

impl Weekday {
    /// The ISO number, `1..=7`.
    pub fn number(self) -> u8 {
        self as u8
    }

    /// The weekday with ISO number `n`, or `None`.
    pub fn from_number(n: u8) -> Option<Weekday> {
        use Weekday::*;
        match n {
            1 => Some(Monday),
            2 => Some(Tuesday),
            3 => Some(Wednesday),
            4 => Some(Thursday),
            5 => Some(Friday),
            6 => Some(Saturday),
            7 => Some(Sunday),
            _ => None,
        }
    }

    /// `true` for Saturday and Sunday.
    pub fn is_weekend(self) -> bool {
        match self {
            Weekday::Saturday | Weekday::Sunday => true,
            Weekday::Monday
            | Weekday::Tuesday
            | Weekday::Wednesday
            | Weekday::Thursday
            | Weekday::Friday => false,
        }
    }

    /// The weekday `days` days later (earlier if negative).
    pub fn add_days(self, days: i64) -> Weekday {
        let index = (i64::from(self.number()) - 1 + days).rem_euclid(7);
        // `index` is in 0..7, so `index + 1` is a valid ISO number.
        Weekday::from_number(index as u8 + 1).expect("index is in 0..7")
    }

    /// Days forward from `self` to the next `other`, in `0..=6`.
    pub fn days_until(self, other: Weekday) -> u8 {
        (other.number() + 7 - self.number()) % 7
    }
}
```

Full tested solution: [`src/solutions/p0227_weekday_discriminants.rs`](../src/solutions/p0227_weekday_discriminants.rs)

</details>
