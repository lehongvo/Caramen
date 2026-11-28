# Problem 0210: Temperature with Multiple impl Blocks

- **Difficulty:** Easy
- **Topics:** several `impl` blocks for one type, associated constants, private fields, `Self`
- **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

## Problem

A type may have any number of `impl` blocks. Splitting them by purpose (constructors, conversions,
comparisons) keeps long types readable. An `impl` block may also hold *associated constants*,
written `Self::NAME` inside the block and `Temperature::NAME` outside.

`Temperature` stores one temperature in a **private** field of your choice (Celsius is a good
choice). Temperatures below absolute zero (`-273.15 °C`) do not exist: every constructor clamps
such values up to absolute zero.

Implement, in three separate `impl` blocks:

1. Constructors: `from_celsius(c)`, `from_fahrenheit(f)`, `from_kelvin(k)`, all `f64 -> Self`.
2. Conversions: `celsius(&self)`, `fahrenheit(&self)`, `kelvin(&self)`, all returning `f64`.
3. Comparisons: `is_freezing(&self) -> bool` (at or below `0 °C`) and
   `warmer_than(&self, other: &Temperature) -> bool` (strictly warmer).

Formulas: `F = C * 9 / 5 + 32` and `K = C + 273.15`. The constant
`Temperature::ABSOLUTE_ZERO_C = -273.15` is already given.

## Examples

```text
Temperature::from_celsius(100.0).fahrenheit()   => 212.0
Temperature::from_fahrenheit(32.0).celsius()    => 0.0
Temperature::from_kelvin(0.0).celsius()         => -273.15
Temperature::from_celsius(-500.0).kelvin()      => 0.0      (clamped)
Temperature::from_celsius(0.0).is_freezing()    => true
from_celsius(20.0).warmer_than(&from_fahrenheit(60.0)) => true   (60 °F is about 15.6 °C)
```

## Constraints

- Inputs are finite `f64` values with absolute value below `10^6`. Results are compared with a
  tolerance of `1e-9`.

## Hints

1. Write `from_fahrenheit` and `from_kelvin` in terms of `from_celsius`, so the clamping lives in
   one place.
2. `f64::max(c, Self::ABSOLUTE_ZERO_C)` clamps from below.

## Starter code

Write your code in [`src/practice/p0210_multiple_impl_blocks_temperature.rs`](../src/practice/p0210_multiple_impl_blocks_temperature.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0210
```

```rust
/// A temperature that is never below absolute zero.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature {}

// Constants and constructors.
impl Temperature {
    /// Absolute zero in degrees Celsius.
    pub const ABSOLUTE_ZERO_C: f64 = -273.15;

    /// From degrees Celsius (clamped at absolute zero).
    pub fn from_celsius(c: f64) -> Self {
        todo!("Write your solution here")
    }

    /// From degrees Fahrenheit (clamped at absolute zero).
    pub fn from_fahrenheit(f: f64) -> Self {
        todo!("Write your solution here")
    }

    /// From kelvins (clamped at absolute zero).
    pub fn from_kelvin(k: f64) -> Self {
        todo!("Write your solution here")
    }
}

// Conversions.
impl Temperature {
    /// In degrees Celsius.
    pub fn celsius(&self) -> f64 {
        todo!("Write your solution here")
    }

    /// In degrees Fahrenheit.
    pub fn fahrenheit(&self) -> f64 {
        todo!("Write your solution here")
    }

    /// In kelvins.
    pub fn kelvin(&self) -> f64 {
        todo!("Write your solution here")
    }
}

// Comparisons.
impl Temperature {
    /// At or below the freezing point of water.
    pub fn is_freezing(&self) -> bool {
        todo!("Write your solution here")
    }

    /// Strictly warmer than `other`.
    pub fn warmer_than(&self, other: &Temperature) -> bool {
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
    fn example_conversions() {
        assert!(close(Temperature::from_celsius(100.0).fahrenheit(), 212.0));
        assert!(close(Temperature::from_fahrenheit(32.0).celsius(), 0.0));
        assert!(close(Temperature::from_kelvin(0.0).celsius(), -273.15));
    }

    #[test]
    fn example_clamping() {
        assert!(close(Temperature::from_celsius(-500.0).kelvin(), 0.0));
        assert!(close(
            Temperature::from_fahrenheit(-1000.0).celsius(),
            Temperature::ABSOLUTE_ZERO_C
        ));
        assert!(close(Temperature::from_kelvin(-5.0).kelvin(), 0.0));
    }

    #[test]
    fn example_comparisons() {
        assert!(Temperature::from_celsius(0.0).is_freezing());
        assert!(!Temperature::from_celsius(0.5).is_freezing());
        let room = Temperature::from_celsius(20.0);
        let cool = Temperature::from_fahrenheit(60.0);
        assert!(room.warmer_than(&cool));
        assert!(!cool.warmer_than(&room));
    }

    #[test]
    fn equal_temperatures_are_not_warmer() {
        let a = Temperature::from_celsius(37.0);
        let b = Temperature::from_kelvin(310.15);
        assert!(close(a.celsius(), b.celsius()));
        assert!(!a.warmer_than(&Temperature::from_celsius(37.0)));
    }

    #[test]
    fn minus_forty_is_the_same_in_both_scales() {
        let t = Temperature::from_fahrenheit(-40.0);
        assert!(close(t.celsius(), -40.0));
        assert!(close(t.fahrenheit(), -40.0));
        assert!(t.is_freezing());
    }

    #[test]
    fn round_trip_through_kelvin() {
        for c in [-273.15, -10.0, 0.0, 36.6, 1_000.0] {
            let t = Temperature::from_kelvin(Temperature::from_celsius(c).kelvin());
            assert!(close(t.celsius(), c));
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** store Celsius internally. Every constructor converts to Celsius and goes through
`from_celsius`, which clamps. Conversions apply the formulas to the stored value.

**Rust concepts:**
- Multiple `impl Temperature` blocks are merged by the compiler; methods in one block can call
  methods from another.
- `pub const ABSOLUTE_ZERO_C: f64 = -273.15;` is an associated constant: it belongs to the type,
  not to a value, and is read as `Self::ABSOLUTE_ZERO_C`.
- A private field forces all construction through the constructors, so the "not below absolute
  zero" rule cannot be bypassed from outside the module.
- `f64::max` ignores a `NaN` argument, so it is a safe clamp for finite inputs.

```rust
/// A temperature that is never below absolute zero.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature {
    celsius: f64,
}

// Constants and constructors.
impl Temperature {
    /// Absolute zero in degrees Celsius.
    pub const ABSOLUTE_ZERO_C: f64 = -273.15;

    /// From degrees Celsius (clamped at absolute zero).
    pub fn from_celsius(c: f64) -> Self {
        Self {
            celsius: c.max(Self::ABSOLUTE_ZERO_C),
        }
    }

    /// From degrees Fahrenheit (clamped at absolute zero).
    pub fn from_fahrenheit(f: f64) -> Self {
        Self::from_celsius((f - 32.0) * 5.0 / 9.0)
    }

    /// From kelvins (clamped at absolute zero).
    pub fn from_kelvin(k: f64) -> Self {
        Self::from_celsius(k + Self::ABSOLUTE_ZERO_C)
    }
}

// Conversions.
impl Temperature {
    /// In degrees Celsius.
    pub fn celsius(&self) -> f64 {
        self.celsius
    }

    /// In degrees Fahrenheit.
    pub fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    /// In kelvins.
    pub fn kelvin(&self) -> f64 {
        self.celsius - Self::ABSOLUTE_ZERO_C
    }
}

// Comparisons.
impl Temperature {
    /// At or below the freezing point of water.
    pub fn is_freezing(&self) -> bool {
        self.celsius() <= 0.0
    }

    /// Strictly warmer than `other`.
    pub fn warmer_than(&self, other: &Temperature) -> bool {
        self.celsius() > other.celsius()
    }
}
```

Full tested solution: [`src/solutions/p0210_multiple_impl_blocks_temperature.rs`](../src/solutions/p0210_multiple_impl_blocks_temperature.rs)

</details>
