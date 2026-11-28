# Problem 0599: Units of Measure

- **Difficulty:** Hard
- **Topics:** `Length<U>` with `PhantomData<U>`, unit marker types with associated consts, operators that only accept the **same** unit, generic conversion `to::<V>()`, manual trait impls without bounds on `U`
- **Source:** Original · [Rust by Example: phantom type parameters](https://doc.rust-lang.org/rust-by-example/generics/phantom.html)

## Problem

Mixing meters and feet by accident is a classic bug. Put the unit **in the type**: `Length<U>`
stores an `f64` value and a zero-sized `PhantomData<U>`, so `Length<Meters>` and `Length<Feet>` are
different types and adding them is a compile error.

```text
pub trait Unit { const NAME: &'static str; const IN_METERS: f64; }
pub struct Meters;  // "m",  1.0
pub struct Feet;    // "ft", 0.3048
pub struct Miles;   // "mi", 1609.344
```

Implement for `Length<U: Unit>` (the struct has private fields `value: f64` and
`_unit: PhantomData<U>`):

- `new(value: f64) -> Self`, `value(self) -> f64`.
- `to<V: Unit>(self) -> Length<V>`: converts through meters.
- `Add` and `Sub` with `Output = Length<U>`, **only** for two lengths of the same unit `U`, and
  `Mul<f64>` (scaling).
- `Sum` for iterators of `Length<U>`.
- `Display`: `"{value:.2} {NAME}"`, e.g. `"3.28 ft"`.
- `Clone`, `Copy`, `PartialEq`, `PartialOrd` and `Debug` **by hand**, without requiring anything of
  `U` beyond `Unit` (the marker types implement nothing else). `Debug` prints like `Length(3.0 m)`.

And the free function `total_meters<A: Unit, B: Unit>(a: &[Length<A>], b: &[Length<B>]) ->
Length<Meters>` which adds everything after converting it.

## Examples

```text
let run = Length::<Miles>::new(1.0);
run.to::<Meters>().value()                        => 1609.344
(Length::<Feet>::new(3.0) + Length::new(2.0)).to_string() => "5.00 ft"
Length::<Meters>::new(1.0).to::<Feet>().to_string()       => "3.28 ft"
Length::<Meters>::new(2.0) * 1.5                  => Length(3.0 m)
// Length::<Meters>::new(1.0) + Length::<Feet>::new(1.0)   // does not compile
```

## Constraints

- Compare floating-point results with a tolerance (`1e-9`).

## Hints

1. `self.value * U::IN_METERS / V::IN_METERS` converts from `U` to `V`.
2. `impl<U: Unit> Add for Length<U>` has `Rhs = Self`, which is exactly "same unit only".
3. `#[derive(Clone, Copy, PartialEq)]` would add `U: Clone`, `U: Copy`, `U: PartialEq` bounds that the
   marker types do not satisfy; implement them manually.

## Starter code

Write your code in [`src/practice/p0599_units_of_measure_phantom.rs`](../src/practice/p0599_units_of_measure_phantom.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0599
```

```rust
use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

/// A unit of length.
pub trait Unit {
    const NAME: &'static str;
    const IN_METERS: f64;
}

pub struct Meters;
pub struct Feet;
pub struct Miles;

impl Unit for Meters {
    const NAME: &'static str = "m";
    const IN_METERS: f64 = 1.0;
}

impl Unit for Feet {
    const NAME: &'static str = "ft";
    const IN_METERS: f64 = 0.3048;
}

impl Unit for Miles {
    const NAME: &'static str = "mi";
    const IN_METERS: f64 = 1609.344;
}

/// A length measured in unit `U`.
pub struct Length<U: Unit> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U: Unit> Length<U> {
    pub fn new(value: f64) -> Self {
        todo!("Write your solution here")
    }

    pub fn value(self) -> f64 {
        todo!("Write your solution here")
    }

    pub fn to<V: Unit>(self) -> Length<V> {
        todo!("Write your solution here")
    }
}

impl<U: Unit> Clone for Length<U> {
    fn clone(&self) -> Self {
        todo!("Write your solution here")
    }
}

impl<U: Unit> Copy for Length<U> {}

impl<U: Unit> PartialEq for Length<U> {
    fn eq(&self, other: &Self) -> bool {
        todo!("Write your solution here")
    }
}

impl<U: Unit> PartialOrd for Length<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!("Write your solution here")
    }
}

impl<U: Unit> fmt::Debug for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl<U: Unit> fmt::Display for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl<U: Unit> Add for Length<U> {
    type Output = Length<U>;

    fn add(self, rhs: Length<U>) -> Length<U> {
        todo!("Write your solution here")
    }
}

impl<U: Unit> Sub for Length<U> {
    type Output = Length<U>;

    fn sub(self, rhs: Length<U>) -> Length<U> {
        todo!("Write your solution here")
    }
}

impl<U: Unit> Mul<f64> for Length<U> {
    type Output = Length<U>;

    fn mul(self, k: f64) -> Length<U> {
        todo!("Write your solution here")
    }
}

impl<U: Unit> Sum for Length<U> {
    fn sum<I: Iterator<Item = Length<U>>>(iter: I) -> Length<U> {
        todo!("Write your solution here")
    }
}

/// Everything converted to meters and added.
pub fn total_meters<A: Unit, B: Unit>(a: &[Length<A>], b: &[Length<B>]) -> Length<Meters> {
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

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn conversions() {
        let run = Length::<Miles>::new(1.0);
        assert!(close(run.to::<Meters>().value(), 1609.344));
        assert!(close(
            Length::<Feet>::new(5280.0).to::<Miles>().value(),
            1.0
        ));
        assert_eq!(
            Length::<Meters>::new(1.0).to::<Feet>().to_string(),
            "3.28 ft"
        );
        let round_trip = Length::<Feet>::new(12.5).to::<Miles>().to::<Feet>();
        assert!(close(round_trip.value(), 12.5));
    }

    #[test]
    fn same_unit_arithmetic() {
        let total = Length::<Feet>::new(3.0) + Length::new(2.0);
        assert_eq!(total.to_string(), "5.00 ft");
        assert_eq!(
            Length::<Meters>::new(5.0) - Length::new(1.5),
            Length::new(3.5)
        );
        assert_eq!(Length::<Meters>::new(2.0) * 1.5, Length::new(3.0));
        let sum: Length<Miles> = (1..=4).map(|i| Length::new(f64::from(i))).sum();
        assert_eq!(sum, Length::new(10.0));
    }

    #[test]
    fn manual_trait_impls() {
        let a = Length::<Meters>::new(3.0);
        let b = a; // Copy, although `Meters` itself is not Copy
        assert_eq!(a, b);
        assert!(Length::<Feet>::new(1.0) < Length::new(2.0));
        assert_eq!(format!("{a:?}"), "Length(3.0 m)");
        assert_eq!(
            std::mem::size_of::<Length<Miles>>(),
            std::mem::size_of::<f64>()
        );
    }

    #[test]
    fn totals_across_units() {
        let feet = [Length::<Feet>::new(10.0), Length::new(20.0)];
        let miles = [Length::<Miles>::new(0.5)];
        let total = total_meters(&feet, &miles);
        assert!(close(total.value(), 30.0 * 0.3048 + 804.672));
        assert_eq!(total_meters::<Meters, Feet>(&[], &[]).value(), 0.0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the marker type `U` only exists at compile time; at run time a `Length<U>` is a single
`f64`. All arithmetic is defined per unit, conversions are explicit, and the trait impls compare or
copy only the value.

**Rust concepts:**
- `PhantomData<U>` makes the struct generic over `U` without storing a `U`; it costs no space.
- Associated constants on the marker types (`U::NAME`, `U::IN_METERS`) attach data to types.
- Because operators are implemented only for `Length<U> op Length<U>`, unit mismatches are caught by
  the type checker. Converting with `to::<V>()` is the only way across units.
- Manual impls avoid the over-constrained bounds that `#[derive]` would add for `U`.

```rust
use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Sub};

/// A unit of length.
pub trait Unit {
    const NAME: &'static str;
    const IN_METERS: f64;
}

pub struct Meters;
pub struct Feet;
pub struct Miles;

impl Unit for Meters {
    const NAME: &'static str = "m";
    const IN_METERS: f64 = 1.0;
}

impl Unit for Feet {
    const NAME: &'static str = "ft";
    const IN_METERS: f64 = 0.3048;
}

impl Unit for Miles {
    const NAME: &'static str = "mi";
    const IN_METERS: f64 = 1609.344;
}

/// A length measured in unit `U`.
pub struct Length<U: Unit> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U: Unit> Length<U> {
    pub fn new(value: f64) -> Self {
        Length {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(self) -> f64 {
        self.value
    }

    pub fn to<V: Unit>(self) -> Length<V> {
        Length::new(self.value * U::IN_METERS / V::IN_METERS)
    }
}

impl<U: Unit> Clone for Length<U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<U: Unit> Copy for Length<U> {}

impl<U: Unit> PartialEq for Length<U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<U: Unit> PartialOrd for Length<U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<U: Unit> fmt::Debug for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Length({:?} {})", self.value, U::NAME)
    }
}

impl<U: Unit> fmt::Display for Length<U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {}", self.value, U::NAME)
    }
}

impl<U: Unit> Add for Length<U> {
    type Output = Length<U>;

    fn add(self, rhs: Length<U>) -> Length<U> {
        Length::new(self.value + rhs.value)
    }
}

impl<U: Unit> Sub for Length<U> {
    type Output = Length<U>;

    fn sub(self, rhs: Length<U>) -> Length<U> {
        Length::new(self.value - rhs.value)
    }
}

impl<U: Unit> Mul<f64> for Length<U> {
    type Output = Length<U>;

    fn mul(self, k: f64) -> Length<U> {
        Length::new(self.value * k)
    }
}

impl<U: Unit> Sum for Length<U> {
    fn sum<I: Iterator<Item = Length<U>>>(iter: I) -> Length<U> {
        iter.fold(Length::new(0.0), |acc, x| acc + x)
    }
}

/// Everything converted to meters and added.
pub fn total_meters<A: Unit, B: Unit>(a: &[Length<A>], b: &[Length<B>]) -> Length<Meters> {
    let from_a: Length<Meters> = a.iter().map(|x| x.to::<Meters>()).sum();
    let from_b: Length<Meters> = b.iter().map(|x| x.to::<Meters>()).sum();
    from_a + from_b
}
```

Full tested solution: [`src/solutions/p0599_units_of_measure_phantom.rs`](../src/solutions/p0599_units_of_measure_phantom.rs)

</details>
