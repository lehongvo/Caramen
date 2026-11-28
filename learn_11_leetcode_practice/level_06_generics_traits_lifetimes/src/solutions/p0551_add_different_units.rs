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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** convert meters to millimeters (`* 1000`) whenever units are mixed, then add the raw
//! numbers. `total` folds both slices with the operators.
//!
//! **Rust concepts:**
//! - Default type parameters (`Rhs = Self`) let common cases stay short while still allowing
//!   `impl Add<Other> for T` for the unusual ones.
//! - The same trait can be implemented several times for one type with **different type parameters**
//!   (`Add<Millimeters>` and `Add<Meters>` for `Millimeters`): each is a distinct trait.
//! - `Output` is chosen per impl, so `Meters + Millimeters` can return `Millimeters`.
// @solution-end

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Millimeters(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Meters(pub u32);

impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self {
        // @solution-start
        Millimeters(m.0 * 1000)
        // @solution-end
    }
}

impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        // @solution-start
        Millimeters(self.0 + other.0)
        // @solution-end
    }
}

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        // @solution-start
        self + Millimeters::from(other)
        // @solution-end
    }
}

impl Add<Millimeters> for Meters {
    type Output = Millimeters;

    fn add(self, other: Millimeters) -> Millimeters {
        // @solution-start
        Millimeters::from(self) + other
        // @solution-end
    }
}

impl Mul<u32> for Meters {
    type Output = Meters;

    fn mul(self, k: u32) -> Meters {
        // @solution-start
        Meters(self.0 * k)
        // @solution-end
    }
}

/// The total length of all parts, in millimeters.
pub fn total(mm: &[Millimeters], m: &[Meters]) -> Millimeters {
    // @solution-start
    let fine = mm.iter().fold(Millimeters(0), |acc, &x| acc + x);
    m.iter().fold(fine, |acc, &x| acc + x)
    // @solution-end
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
