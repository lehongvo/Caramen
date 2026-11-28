// ==================================================================
// PRACTICE 0542: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0542_from_into_temperature.md
// Run the tests:          cargo test practice::p0542
// ==================================================================

//! # Problem 0542: Temperatures with `From` and `Into`
//!
//! - **Difficulty:** Easy
//! - **Topics:** newtypes, implementing `From`, getting `Into` for free, `impl Into<T>` parameters, generic `T: Into<U> + Copy`
//! - **Source:** Original · [std::convert::From](https://doc.rust-lang.org/std/convert/trait.From.html)
//!
//! ## Problem
//!
//! Three newtypes wrap an `f64`: `Celsius(pub f64)`, `Fahrenheit(pub f64)` and `Kelvin(pub f64)`.
//! Implement the conversions
//!
//! - `From<Celsius> for Fahrenheit`: `F = C * 9 / 5 + 32`
//! - `From<Fahrenheit> for Celsius`: `C = (F - 32) * 5 / 9`
//! - `From<Celsius> for Kelvin`: `K = C + 273.15`
//! - `From<Kelvin> for Celsius`: `C = K - 273.15`
//!
//! and then two functions that accept **any** temperature that converts into `Celsius`:
//!
//! - `is_freezing(t: impl Into<Celsius>) -> bool`: `true` when the temperature is at or below `0 °C`.
//! - `average_celsius<T: Into<Celsius> + Copy>(temps: &[T]) -> Option<Celsius>`: the average in Celsius,
//!   `None` for an empty slice.
//!
//! Do **not** implement `Into` yourself: it comes for free from `From`.
//!
//! ## Examples
//!
//! ```text
//! Fahrenheit::from(Celsius(100.0))          => Fahrenheit(212.0)
//! let c: Celsius = Fahrenheit(-40.0).into(); => Celsius(-40.0)
//! Kelvin::from(Celsius(0.0))                => Kelvin(273.15)
//! is_freezing(Fahrenheit(30.0))             => true
//! is_freezing(Kelvin(300.0))                => false
//! average_celsius(&[Fahrenheit(32.0), Fahrenheit(212.0)]) => Some(Celsius(50.0))
//! ```
//!
//! ## Constraints
//!
//! - Values are finite; compare results with a small tolerance (`1e-9`).
//!
//! ## Hints
//!
//! 1. `impl From<Celsius> for Fahrenheit { fn from(c: Celsius) -> Self { ... } }`.
//! 2. `Celsius` converts into itself through the standard blanket impl `impl<T> From<T> for T`, so
//!    `is_freezing(Celsius(-1.0))` works too.
//! 3. Kelvin to Fahrenheit has no direct impl: go through Celsius, `Fahrenheit::from(Celsius::from(k))`.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Celsius(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fahrenheit(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kelvin(pub f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        todo!("Write your solution here")
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        todo!("Write your solution here")
    }
}

impl From<Celsius> for Kelvin {
    fn from(c: Celsius) -> Self {
        todo!("Write your solution here")
    }
}

impl From<Kelvin> for Celsius {
    fn from(k: Kelvin) -> Self {
        todo!("Write your solution here")
    }
}

/// `true` at or below 0 degrees Celsius.
pub fn is_freezing(t: impl Into<Celsius>) -> bool {
    todo!("Write your solution here")
}

/// The average temperature in Celsius.
pub fn average_celsius<T: Into<Celsius> + Copy>(temps: &[T]) -> Option<Celsius> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn celsius_fahrenheit_round_trip() {
        assert_eq!(Fahrenheit::from(Celsius(100.0)), Fahrenheit(212.0));
        let c: Celsius = Fahrenheit(-40.0).into();
        assert_eq!(c, Celsius(-40.0));
        let back: Celsius = Fahrenheit::from(Celsius(37.0)).into();
        assert!(close(back.0, 37.0));
    }

    #[test]
    fn kelvin_conversions() {
        assert_eq!(Kelvin::from(Celsius(0.0)), Kelvin(273.15));
        assert!(close(Celsius::from(Kelvin(373.15)).0, 100.0));
        let f = Fahrenheit::from(Celsius::from(Kelvin(0.0)));
        assert!(close(f.0, -459.67));
    }

    #[test]
    fn freezing_accepts_any_unit() {
        assert!(is_freezing(Celsius(0.0)));
        assert!(is_freezing(Fahrenheit(30.0)));
        assert!(!is_freezing(Fahrenheit(33.0)));
        assert!(!is_freezing(Kelvin(300.0)));
        assert!(is_freezing(Kelvin(273.0)));
    }

    #[test]
    fn averages() {
        let avg = average_celsius(&[Fahrenheit(32.0), Fahrenheit(212.0)]).unwrap();
        assert!(close(avg.0, 50.0));
        let avg = average_celsius(&[Kelvin(273.15), Kelvin(283.15)]).unwrap();
        assert!(close(avg.0, 5.0));
        assert_eq!(average_celsius::<Celsius>(&[]), None);
        assert_eq!(average_celsius(&[Celsius(-3.0)]), Some(Celsius(-3.0)));
    }
}
