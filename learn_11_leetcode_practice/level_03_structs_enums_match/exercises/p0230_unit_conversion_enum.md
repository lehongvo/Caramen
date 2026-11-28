# Problem 0230: Length Units with Enum Data

- **Difficulty:** Easy
- **Topics:** enum variants with different data shapes, converting between variants, `round`, `format!` precision
- **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)

## Problem

A length can be written in two systems, and each system naturally stores different data:

```text
enum Length {
    Metric(f64),                        // meters
    Imperial { feet: u32, inches: u32 }, // inches is always 0..=11
}
```

One inch is exactly `0.0254` meters and one foot is `12` inches. Implement:

- `to_meters(&self) -> f64`;
- `to_metric(&self) -> Length`: the same length as a `Metric` value;
- `to_imperial(&self) -> Length`: the same length as `Imperial`, **rounded to the nearest whole
  inch** (a half inch rounds up), with `inches` in `0..=11`;
- `plus(&self, other: &Length) -> Length`: the sum, expressed in the system of `self` (rounded to
  the nearest inch when `self` is imperial);
- `describe(&self) -> String`: `"<meters> m"` with exactly two decimals (e.g. `"1.80 m"`) for
  metric, and `<feet>' <inches>"` (e.g. `5' 11"`) for imperial.

## Examples

```text
Length::Imperial { feet: 1, inches: 0 }.to_meters()         => 0.3048
Length::Metric(1.8).to_imperial()                           => Imperial { feet: 5, inches: 11 }  (70.87 in)
Length::Metric(1.8).describe()                              => "1.80 m"
Length::Imperial { feet: 6, inches: 2 }.describe()          => "6' 2\""
Length::Imperial { feet: 5, inches: 10 }.plus(&Length::Metric(0.0508))
                                                            => Imperial { feet: 6, inches: 0 }
Length::Metric(1.0).plus(&Length::Imperial { feet: 0, inches: 10 })  => Metric(1.254)
```

## Constraints

- All lengths are non-negative and at most `10^5` meters. Tests compare meters with a tolerance
  of `1e-9`.

## Hints

1. Convert everything through meters: `to_metric` and `to_imperial` both start from `to_meters`.
2. `(meters / 0.0254).round()` gives the whole number of inches; then use `/ 12` and `% 12`.
3. `format!("{:.2} m", m)` prints two decimals.

## Starter code

Write your code in [`src/practice/p0230_unit_conversion_enum.rs`](../src/practice/p0230_unit_conversion_enum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0230
```

```rust
/// Meters in one inch.
pub const METERS_PER_INCH: f64 = 0.0254;

/// A length in metric (meters) or imperial (feet and inches) form.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Metric(f64),
    Imperial { feet: u32, inches: u32 },
}

impl Length {
    /// The length in meters.
    pub fn to_meters(&self) -> f64 {
        todo!("Write your solution here")
    }

    /// The same length as a `Metric` value.
    pub fn to_metric(&self) -> Length {
        todo!("Write your solution here")
    }

    /// The same length as an `Imperial` value, rounded to the nearest inch.
    pub fn to_imperial(&self) -> Length {
        todo!("Write your solution here")
    }

    /// `self + other`, in the system of `self`.
    pub fn plus(&self, other: &Length) -> Length {
        todo!("Write your solution here")
    }

    /// `"1.80 m"` or `5' 11"`.
    pub fn describe(&self) -> String {
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

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn to_meters_both_systems() {
        assert!(close(Length::Metric(2.5).to_meters(), 2.5));
        assert!(close(
            Length::Imperial { feet: 1, inches: 0 }.to_meters(),
            0.3048
        ));
        assert!(close(
            Length::Imperial { feet: 0, inches: 1 }.to_meters(),
            0.0254
        ));
    }

    #[test]
    fn conversions() {
        assert_eq!(
            Length::Metric(1.8).to_imperial(),
            Length::Imperial {
                feet: 5,
                inches: 11
            }
        );
        assert_eq!(
            Length::Metric(0.3048).to_imperial(),
            Length::Imperial { feet: 1, inches: 0 }
        );
        match (Length::Imperial { feet: 3, inches: 3 }).to_metric() {
            Length::Metric(m) => assert!(close(m, 0.9906)),
            other => panic!("expected metric, got {other:?}"),
        }
    }

    #[test]
    fn rounding_to_nearest_inch() {
        // 0.0127 m is exactly half an inch: rounds up.
        assert_eq!(
            Length::Metric(0.0127).to_imperial(),
            Length::Imperial { feet: 0, inches: 1 }
        );
        assert_eq!(
            Length::Metric(0.01).to_imperial(),
            Length::Imperial { feet: 0, inches: 0 }
        );
    }

    #[test]
    fn describing() {
        assert_eq!(Length::Metric(1.8).describe(), "1.80 m");
        assert_eq!(Length::Metric(0.0).describe(), "0.00 m");
        assert_eq!(Length::Imperial { feet: 6, inches: 2 }.describe(), "6' 2\"");
    }

    #[test]
    fn adding_keeps_the_system_of_self() {
        let a = Length::Imperial {
            feet: 5,
            inches: 10,
        };
        assert_eq!(
            a.plus(&Length::Metric(0.0508)),
            Length::Imperial { feet: 6, inches: 0 }
        );
        match Length::Metric(1.0).plus(&Length::Imperial {
            feet: 0,
            inches: 10,
        }) {
            Length::Metric(m) => assert!(close(m, 1.254)),
            other => panic!("expected metric, got {other:?}"),
        }
    }

    #[test]
    fn large_length() {
        let km100 = Length::Metric(100_000.0);
        // 100 km = 3_937_007.87 inches, rounds to 3_937_008 = 328_084 feet and 0 inches.
        assert_eq!(
            km100.to_imperial(),
            Length::Imperial {
                feet: 328_084,
                inches: 0
            }
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** meters are the common currency. `to_meters` is a two-arm `match`; every other method
converts to meters, computes, and converts back into the wanted variant.

**Rust concepts:**
- Variants of one enum may hold completely different data: a tuple variant with one `f64` and a
  struct-like variant with two `u32`s. A `match` gives access to each shape.
- `f64::round` rounds half away from zero, so `x.5` rounds up for non-negative values.
- Casting a non-negative, in-range `f64` with `as u32` truncates toward zero; after `round` the
  value is already whole.
- `plus` matches on `self` only to decide the output system; the arithmetic itself happens in
  meters.

```rust
/// Meters in one inch.
pub const METERS_PER_INCH: f64 = 0.0254;

/// A length in metric (meters) or imperial (feet and inches) form.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Metric(f64),
    Imperial { feet: u32, inches: u32 },
}

impl Length {
    /// The length in meters.
    pub fn to_meters(&self) -> f64 {
        match self {
            Length::Metric(m) => *m,
            Length::Imperial { feet, inches } => f64::from(feet * 12 + inches) * METERS_PER_INCH,
        }
    }

    /// The same length as a `Metric` value.
    pub fn to_metric(&self) -> Length {
        Length::Metric(self.to_meters())
    }

    /// The same length as an `Imperial` value, rounded to the nearest inch.
    pub fn to_imperial(&self) -> Length {
        // Lengths are non-negative and small, so the rounded value fits in a `u32`.
        let total_inches = (self.to_meters() / METERS_PER_INCH).round() as u32;
        Length::Imperial {
            feet: total_inches / 12,
            inches: total_inches % 12,
        }
    }

    /// `self + other`, in the system of `self`.
    pub fn plus(&self, other: &Length) -> Length {
        let sum = Length::Metric(self.to_meters() + other.to_meters());
        match self {
            Length::Metric(_) => sum,
            Length::Imperial { .. } => sum.to_imperial(),
        }
    }

    /// `"1.80 m"` or `5' 11"`.
    pub fn describe(&self) -> String {
        match self {
            Length::Metric(m) => format!("{m:.2} m"),
            Length::Imperial { feet, inches } => format!("{feet}' {inches}\""),
        }
    }
}
```

Full tested solution: [`src/solutions/p0230_unit_conversion_enum.rs`](../src/solutions/p0230_unit_conversion_enum.rs)

</details>
