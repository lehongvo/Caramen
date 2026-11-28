//! # Problem 0284: Money with Currencies
//!
//! - **Difficulty:** Medium
//! - **Topics:** a struct with an enum field, `Option` for currency mismatches, `checked_add`/`checked_sub`, `?` chains, associated functions
//! - **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) · [`i64::checked_add`](https://doc.rust-lang.org/std/primitive.i64.html#method.checked_add)
//!
//! ## Problem
//!
//! Money must never be added across currencies by accident. Model an amount as
//! `Money { cents: i64, currency: Currency }` with `enum Currency { Eur, Usd, Jpy }`. Japanese yen
//! have no cents, so for `Jpy` the `cents` field simply counts whole yen.
//!
//! Implement:
//!
//! - `Money::new(cents: i64, currency: Currency) -> Money`;
//! - `plus(self, other: Money) -> Option<Money>` and `minus(self, other: Money) -> Option<Money>`:
//!   `None` if the currencies differ or the result overflows an `i64`;
//! - `split(self, parts: u32) -> Option<Vec<Money>>`: divide into `parts` amounts that differ by at
//!   most one cent and add up **exactly** to the original; the larger shares come first. `None` for
//!   `parts == 0` or a negative amount;
//! - `display(&self) -> String`: `"12.34 EUR"`, `"-0.05 USD"`, `"1500 JPY"` (two decimals for EUR
//!   and USD, none for JPY);
//! - the free function `total(items: &[Money]) -> Option<Money>`: the sum of a non-empty list in a
//!   single currency, `None` for an empty list or mixed currencies.
//!
//! ## Examples
//!
//! ```text
//! Money::new(1050, Eur).plus(Money::new(275, Eur))  => Some(13.25 EUR)
//! Money::new(1050, Eur).plus(Money::new(275, Usd))  => None
//! Money::new(100, Usd).minus(Money::new(105, Usd))  => Some(-0.05 USD)
//! Money::new(1000, Eur).split(3)                    => Some([3.34 EUR, 3.33 EUR, 3.33 EUR])
//! Money::new(1500, Jpy).display()                   => "1500 JPY"
//! total(&[1.00 USD, 2.50 USD])                      => Some(3.50 USD)
//! ```
//!
//! ## Constraints
//!
//! - Amounts fit in an `i64`; `parts <= 10^4`.
//!
//! ## Hints
//!
//! 1. Guard the currency first: `if self.currency != other.currency { return None; }`.
//! 2. `self.cents.checked_add(other.cents)?` stops on overflow.
//! 3. For `split`, give `cents / parts` to everybody and one extra cent to the first `cents % parts` shares.
//! 4. For negative EUR/USD amounts, format the sign separately and use `abs` on the parts.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every operation first checks the currency, then does integer arithmetic on cents
//! with overflow checks. Working in integer cents avoids floating-point rounding entirely.
//!
//! **Rust concepts:**
//! - Combining a struct (the amount) with an enum (the unit) makes "mixing currencies" a value
//!   the code must handle: `Option<Money>` forces callers to think about it.
//! - `checked_add` / `checked_sub` return `None` on overflow, so `?` handles both failure reasons
//!   the same way.
//! - `total` uses `split_first` and a `for` loop with `acc = acc.plus(m)?`.
//! - `Currency::code` and `Currency::has_cents` keep the per-currency facts in exhaustive `match`es.
// @solution-end

/// A currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    Eur,
    Usd,
    Jpy,
}

/// An amount of money in the smallest unit of its currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub cents: i64,
    pub currency: Currency,
}

// @solution-start remove
impl Currency {
    fn code(self) -> &'static str {
        match self {
            Currency::Eur => "EUR",
            Currency::Usd => "USD",
            Currency::Jpy => "JPY",
        }
    }

    fn has_cents(self) -> bool {
        match self {
            Currency::Eur | Currency::Usd => true,
            Currency::Jpy => false,
        }
    }
}
// @solution-end

impl Money {
    /// An amount in the smallest unit of `currency`.
    pub fn new(cents: i64, currency: Currency) -> Money {
        // @solution-start
        Money { cents, currency }
        // @solution-end
    }

    /// The sum, or `None` for different currencies or overflow.
    pub fn plus(self, other: Money) -> Option<Money> {
        // @solution-start
        if self.currency != other.currency {
            return None;
        }
        Some(Money::new(
            self.cents.checked_add(other.cents)?,
            self.currency,
        ))
        // @solution-end
    }

    /// The difference, or `None` for different currencies or overflow.
    pub fn minus(self, other: Money) -> Option<Money> {
        // @solution-start
        if self.currency != other.currency {
            return None;
        }
        Some(Money::new(
            self.cents.checked_sub(other.cents)?,
            self.currency,
        ))
        // @solution-end
    }

    /// Splits into `parts` near-equal shares that add up exactly.
    pub fn split(self, parts: u32) -> Option<Vec<Money>> {
        // @solution-start
        if parts == 0 || self.cents < 0 {
            return None;
        }
        let n = i64::from(parts);
        let (base, extra) = (self.cents / n, self.cents % n);
        Some(
            (0..n)
                .map(|i| Money::new(base + i64::from(i < extra), self.currency))
                .collect(),
        )
        // @solution-end
    }

    /// `"12.34 EUR"` or `"1500 JPY"`.
    pub fn display(&self) -> String {
        // @solution-start
        let code = self.currency.code();
        if !self.currency.has_cents() {
            return format!("{} {code}", self.cents);
        }
        let sign = if self.cents < 0 { "-" } else { "" };
        let abs = self.cents.unsigned_abs();
        format!("{sign}{}.{:02} {code}", abs / 100, abs % 100)
        // @solution-end
    }
}

/// The sum of a non-empty, single-currency list.
pub fn total(items: &[Money]) -> Option<Money> {
    // @solution-start
    let (first, rest) = items.split_first()?;
    let mut acc = *first;
    for &m in rest {
        acc = acc.plus(m)?;
    }
    Some(acc)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::Currency::*;
    use super::*;

    fn m(cents: i64, c: Currency) -> Money {
        Money::new(cents, c)
    }

    #[test]
    fn adding_and_subtracting() {
        assert_eq!(m(1050, Eur).plus(m(275, Eur)), Some(m(1325, Eur)));
        assert_eq!(m(1050, Eur).plus(m(275, Usd)), None);
        assert_eq!(m(100, Usd).minus(m(105, Usd)), Some(m(-5, Usd)));
        assert_eq!(m(1, Jpy).minus(m(1, Eur)), None);
    }

    #[test]
    fn overflow_is_none() {
        assert_eq!(m(i64::MAX, Eur).plus(m(1, Eur)), None);
        assert_eq!(m(i64::MIN, Eur).minus(m(1, Eur)), None);
        assert_eq!(
            m(i64::MAX, Eur).minus(m(1, Eur)),
            Some(m(i64::MAX - 1, Eur))
        );
    }

    #[test]
    fn splitting() {
        assert_eq!(
            m(1000, Eur).split(3),
            Some(vec![m(334, Eur), m(333, Eur), m(333, Eur)])
        );
        assert_eq!(m(5, Usd).split(5), Some(vec![m(1, Usd); 5]));
        assert_eq!(
            m(2, Usd).split(4),
            Some(vec![m(1, Usd), m(1, Usd), m(0, Usd), m(0, Usd)])
        );
        assert_eq!(m(10, Usd).split(0), None);
        assert_eq!(m(-5, Usd).split(2), None);
    }

    #[test]
    fn split_adds_up_for_many_parts() {
        let shares = m(1_000_003, Jpy).split(10_000).expect("parts > 0");
        assert_eq!(shares.len(), 10_000);
        assert_eq!(total(&shares), Some(m(1_000_003, Jpy)));
    }

    #[test]
    fn displays() {
        assert_eq!(m(1234, Eur).display(), "12.34 EUR");
        assert_eq!(m(-5, Usd).display(), "-0.05 USD");
        assert_eq!(m(1500, Jpy).display(), "1500 JPY");
        assert_eq!(m(-1500, Jpy).display(), "-1500 JPY");
        assert_eq!(m(0, Eur).display(), "0.00 EUR");
        assert_eq!(m(i64::MIN, Usd).display(), "-92233720368547758.08 USD");
    }

    #[test]
    fn totals() {
        assert_eq!(total(&[m(100, Usd), m(250, Usd)]), Some(m(350, Usd)));
        assert_eq!(total(&[m(100, Usd), m(250, Eur)]), None);
        assert_eq!(total(&[]), None);
        assert_eq!(total(&[m(7, Jpy)]), Some(m(7, Jpy)));
    }
}
