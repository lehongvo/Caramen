// ==================================================================
// PRACTICE 0551: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0551_add_different_units.md
// Run the tests:          cargo test practice::p0551
// ==================================================================

//! # Problem 0551: Adding Different Units
//!
//! - **Difficulty:** Easy
//! - **Topics:** default generic type parameters (`Add<Rhs = Self>`), `impl Add<Meters> for Millimeters`, choosing `Output`, `Mul<u32>`
//! - **Source:** Original · [The Rust Book 20.2](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#default-generic-type-parameters-and-operator-overloading)
//!
//! ## Problem
//!
//! The `Add` trait is declared as `trait Add<Rhs = Self> { type Output; fn add(self, rhs: Rhs) ->
//! Self::Output; }`. The **default type parameter** `Rhs = Self` is why `impl Add for X` means "`X + X`",
//! but you can also choose another right-hand side.
//!
//! Two length newtypes are given: `Millimeters(pub u32)` and `Meters(pub u32)`. Implement:
//!
//! - `Millimeters + Millimeters -> Millimeters` (`impl Add for Millimeters`, using the default `Rhs`).
//! - `Millimeters + Meters -> Millimeters` (`impl Add<Meters> for Millimeters`).
//! - `Meters + Millimeters -> Millimeters` (`impl Add<Millimeters> for Meters`): mixing units always
//!   gives the finer unit, so nothing is lost.
//! - `Meters * u32 -> Meters` (`impl Mul<u32> for Meters`).
//! - `From<Meters> for Millimeters`.
//! - `total(mm: &[Millimeters], m: &[Meters]) -> Millimeters`: the sum of everything, using the
//!   operators above.
//!
//! ## Examples
//!
//! ```text
//! Millimeters(300) + Millimeters(25)   => Millimeters(325)
//! Millimeters(500) + Meters(2)         => Millimeters(2500)
//! Meters(1) + Millimeters(1)           => Millimeters(1001)
//! Meters(3) * 4                        => Meters(12)
//! total(&[Millimeters(1)], &[Meters(1), Meters(2)]) => Millimeters(3001)
//! ```
//!
//! ## Constraints
//!
//! - All results fit in `u32`.
//!
//! ## Hints
//!
//! 1. The impl for a different right-hand side names it: `impl Add<Meters> for Millimeters { type
//!    Output = Millimeters; fn add(self, other: Meters) -> Millimeters { ... } }`.
//! 2. Implement `From<Meters> for Millimeters` first and reuse it in the mixed additions.

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Millimeters(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Meters(pub u32);

impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self {
        todo!("Write your solution here")
    }
}

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        todo!("Write your solution here")
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        todo!("Write your solution here")
    }
}

impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        todo!("Write your solution here")
    }
}

impl Mul<u32> for Meters {
    type Output = Meters;

    fn mul(self, k: u32) -> Meters {
        todo!("Write your solution here")
    }
}

/// The total length of all parts, in millimeters.
pub fn total(mm: &[Millimeters], m: &[Meters]) -> Millimeters {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_unit_addition() {
        assert_eq!(Millimeters(300) + Millimeters(25), Millimeters(325));
        assert_eq!(Millimeters(0) + Millimeters(0), Millimeters(0));
    }

    #[test]
    fn mixed_unit_addition() {
        assert_eq!(Millimeters(500) + Meters(2), Millimeters(2500));
        assert_eq!(Meters(1) + Millimeters(1), Millimeters(1001));
        assert_eq!(Millimeters::from(Meters(7)), Millimeters(7000));
    }

    #[test]
    fn scaling_and_totals() {
        assert_eq!(Meters(3) * 4, Meters(12));
        assert_eq!(
            total(&[Millimeters(1)], &[Meters(1), Meters(2)]),
            Millimeters(3001)
        );
        assert_eq!(total(&[], &[]), Millimeters(0));
        let mm: Vec<Millimeters> = (1..=1000).map(Millimeters).collect();
        let m: Vec<Meters> = vec![Meters(1); 100];
        assert_eq!(total(&mm, &m), Millimeters(500_500 + 100_000));
    }
}
