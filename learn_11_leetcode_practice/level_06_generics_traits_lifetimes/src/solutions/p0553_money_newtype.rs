//! # Problem 0553: A Money Newtype
//!
//! - **Difficulty:** Medium
//! - **Topics:** newtype pattern with a private field, arithmetic operator traits, `Mul<i64>`, `Sum`, `Display` with signs and padding
//! - **Source:** Original · [The Rust Book 20.3](https://doc.rust-lang.org/book/ch20-03-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction)
//!
//! ## Problem
//!
//! Floating-point dollars lose cents (`0.1 + 0.2 != 0.3`). Store money as an integer number of cents
//! inside a newtype `Money` whose field is **private**, and give it exactly the operations that make
//! sense for money:
//!
//! - `Money::from_cents(cents: i64) -> Money` and `cents(self) -> i64`.
//! - `Money::dollars(d: i64) -> Money` (whole dollars).
//! - `Add`, `Sub` (both `Money` with `Money`), `Neg`, `AddAssign`, `SubAssign`.
//! - `Mul<i64> for Money`: a price times a quantity. (There is deliberately **no** `Money * Money`.)
//! - `Sum` for iterators of `Money` and of `&Money`.
//! - `Display`: `"$12.34"`, `"$0.05"`, `"-$7.50"` (the minus sign goes before the dollar sign).
//! - `split(self, parts: u32) -> Vec<Money>`: divide into `parts` amounts that differ by at most one
//!   cent and add up exactly to `self`; the larger amounts come first. `$10.00` split 3 ways is
//!   `[$3.34, $3.33, $3.33]`. `parts` is at least 1. (Only non-negative amounts are split in the tests.)
//!
//! `Money` derives `Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default`.
//!
//! ## Examples
//!
//! ```text
//! Money::from_cents(1234).to_string()                 => "$12.34"
//! (Money::dollars(5) - Money::from_cents(750)).to_string() => "-$2.50"
//! Money::from_cents(199) * 3                          => Money::from_cents(597)
//! [Money::dollars(1), Money::from_cents(5)].iter().sum::<Money>() => Money::from_cents(105)
//! Money::dollars(10).split(3)                         => [$3.34, $3.33, $3.33]
//! ```
//!
//! ## Constraints
//!
//! - Amounts fit in `i64` cents.
//!
//! ## Hints
//!
//! 1. For `Display`, write the sign first, then work with `cents.unsigned_abs()`: dollars are
//!    `abs / 100` and cents are `abs % 100`, printed with `{:02}`.
//! 2. For `split`, every part gets `total / parts` cents and the first `total % parts` parts get one
//!    more.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** all arithmetic happens on the inner `i64`; the newtype only exposes operations that are
//! meaningful for money. `Display` handles the sign separately so `-$0.50` prints correctly even when
//! the dollar part is zero.
//!
//! **Rust concepts:**
//! - The newtype pattern creates a distinct type over `i64` with its own API: callers cannot
//!   accidentally multiply two amounts or mix cents with other integers.
//! - A private field plus constructors (`from_cents`, `dollars`) keeps the representation an
//!   implementation detail you could change later.
//! - `impl Mul<i64> for Money` without `impl Mul<Money> for Money` encodes a unit rule in the type
//!   system: `price * price` simply does not compile.
//! - `unsigned_abs()` avoids overflow for `i64::MIN`, where `abs()` would panic.
// @solution-end

use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

/// An amount of money stored as whole cents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Money {
    // @solution-start remove
    cents: i64,
    // @solution-end
}

impl Money {
    pub fn from_cents(cents: i64) -> Money {
        // @solution-start
        Money { cents }
        // @solution-end
    }

    pub fn dollars(d: i64) -> Money {
        // @solution-start
        Money { cents: d * 100 }
        // @solution-end
    }

    pub fn cents(self) -> i64 {
        // @solution-start
        self.cents
        // @solution-end
    }

    /// Splits into `parts` amounts that differ by at most one cent, larger ones first.
    pub fn split(self, parts: u32) -> Vec<Money> {
        // @solution-start
        let n = i64::from(parts);
        let base = self.cents / n;
        let extra = self.cents % n;
        (0..n)
            .map(|i| Money::from_cents(base + i64::from(i < extra)))
            .collect()
        // @solution-end
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Money) -> Money {
        // @solution-start
        Money::from_cents(self.cents + rhs.cents)
        // @solution-end
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, rhs: Money) -> Money {
        // @solution-start
        Money::from_cents(self.cents - rhs.cents)
        // @solution-end
    }
}

impl Neg for Money {
    type Output = Money;

    fn neg(self) -> Money {
        // @solution-start
        Money::from_cents(-self.cents)
        // @solution-end
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Money) {
        // @solution-start
        *self = *self + rhs;
        // @solution-end
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Money) {
        // @solution-start
        *self = *self - rhs;
        // @solution-end
    }
}

impl Mul<i64> for Money {
    type Output = Money;

    fn mul(self, quantity: i64) -> Money {
        // @solution-start
        Money::from_cents(self.cents * quantity)
        // @solution-end
    }
}

impl Sum for Money {
    fn sum<I: Iterator<Item = Money>>(iter: I) -> Money {
        // @solution-start
        iter.fold(Money::default(), |acc, m| acc + m)
        // @solution-end
    }
}

impl<'a> Sum<&'a Money> for Money {
    fn sum<I: Iterator<Item = &'a Money>>(iter: I) -> Money {
        // @solution-start
        iter.copied().sum()
        // @solution-end
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // @solution-start
        let sign = if self.cents < 0 { "-" } else { "" };
        let abs = self.cents.unsigned_abs();
        write!(f, "{sign}${}.{:02}", abs / 100, abs % 100)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats() {
        assert_eq!(Money::from_cents(1234).to_string(), "$12.34");
        assert_eq!(Money::from_cents(5).to_string(), "$0.05");
        assert_eq!(Money::from_cents(-750).to_string(), "-$7.50");
        assert_eq!(Money::from_cents(-50).to_string(), "-$0.50");
        assert_eq!(Money::default().to_string(), "$0.00");
        assert_eq!(
            Money::from_cents(i64::MIN).to_string(),
            "-$92233720368547758.08"
        );
    }

    #[test]
    fn arithmetic() {
        let price = Money::from_cents(199);
        assert_eq!(price * 3, Money::from_cents(597));
        assert_eq!(
            (Money::dollars(5) - Money::from_cents(750)).to_string(),
            "-$2.50"
        );
        assert_eq!(-Money::dollars(2), Money::from_cents(-200));
        let mut wallet = Money::dollars(20);
        wallet -= price * 2;
        wallet += Money::from_cents(2);
        assert_eq!(wallet.cents(), 1604);
        assert!(Money::dollars(1) > Money::from_cents(99));
    }

    #[test]
    fn sums() {
        let items = [Money::dollars(1), Money::from_cents(5)];
        assert_eq!(items.iter().sum::<Money>(), Money::from_cents(105));
        assert_eq!(items.into_iter().sum::<Money>(), Money::from_cents(105));
        assert_eq!(Vec::<Money>::new().iter().sum::<Money>(), Money::default());
        // 0.1 + 0.2 as floats is not 0.3, but as cents it is exact.
        assert_eq!(
            Money::from_cents(10) + Money::from_cents(20),
            Money::from_cents(30)
        );
    }

    #[test]
    fn splitting() {
        let parts = Money::dollars(10).split(3);
        let shown: Vec<String> = parts.iter().map(Money::to_string).collect();
        assert_eq!(shown, vec!["$3.34", "$3.33", "$3.33"]);
        assert_eq!(parts.iter().sum::<Money>(), Money::dollars(10));
        assert_eq!(Money::from_cents(7).split(1), vec![Money::from_cents(7)]);
        let many = Money::from_cents(1_000_003).split(1000);
        assert_eq!(many.len(), 1000);
        assert_eq!(many[0], Money::from_cents(1001));
        assert_eq!(many[2], Money::from_cents(1001));
        assert_eq!(many[3], Money::from_cents(1000));
        assert_eq!(many.iter().sum::<Money>(), Money::from_cents(1_000_003));
    }
}
