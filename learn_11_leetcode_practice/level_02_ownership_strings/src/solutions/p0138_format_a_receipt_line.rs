//! # Problem 0138: Format a Receipt Line
//!
//! - **Difficulty:** Easy
//! - **Topics:** `format!`, width and alignment (`{:<10}`, `{:>3}`), zero padding (`{:02}`), `u64`
//! - **Source:** Original · [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! A shop prints one line per item on its receipts. Implement
//! `receipt_line(item: &str, quantity: u32, unit_price_cents: u32) -> String`, which returns the
//! line in this exact layout:
//!
//! ```text
//! <item, left-aligned, padded with spaces to 10 characters><quantity, right-aligned in 3 characters> x $<unit price> = $<total>
//! ```
//!
//! - The item name is padded on the right to a width of 10. A longer name is printed in full (never
//!   cut).
//! - The quantity is right-aligned in a field of width 3 (larger numbers are printed in full).
//! - Prices are written as dollars and cents: `$D.CC`, where `D` has no leading zeros (`0` for
//!   amounts below one dollar) and `CC` always has two digits.
//! - The total is `quantity * unit_price_cents` and can exceed `u32::MAX`.
//!
//! ## Examples
//!
//! ```text
//! Input:  item = "apple", quantity = 3, unit_price_cents = 50
//! Output: "apple       3 x $0.50 = $1.50"
//!
//! Input:  item = "watermelon", quantity = 12, unit_price_cents = 405
//! Output: "watermelon 12 x $4.05 = $48.60"
//!
//! Input:  item = "extraordinary", quantity = 1000, unit_price_cents = 7
//! Output: "extraordinary1000 x $0.07 = $70.00"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= item.len() <= 50`, `item` is ASCII
//! - `0 <= quantity, unit_price_cents <= u32::MAX`
//!
//! ## Hints
//!
//! 1. `format!("{item:<10}")` pads on the right; `format!("{quantity:>3}")` pads on the left.
//! 2. Split cents into dollars and cents with `/ 100` and `% 100`, and print the cents with `{:02}`.
//! 3. Convert to `u64` before multiplying so the total cannot overflow.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** compute the total in `u64`, then let a single `format!` call build the whole line using
//! width, alignment and zero-padding specifiers. A small helper formats a cent amount as `D.CC`.
//!
//! **Rust concepts:**
//! - `format!` returns a new `String`; `{name:<10}` left-aligns in 10 columns, `{:>3}` right-aligns,
//!   `{:02}` pads a number with zeros to 2 digits. Widths are minimums: longer values are not cut.
//! - Inline arguments (`{item}`) can only name variables; expressions like `cents / 100` go in the
//!   positional argument list.
//! - `u64::from(x)` widens a `u32` losslessly; `u32::MAX * u32::MAX` fits in a `u64`.
// @solution-end

// @solution-start remove
/// Formats an amount of cents as `D.CC`.
fn dollars(cents: u64) -> String {
    format!("{}.{:02}", cents / 100, cents % 100)
}
// @solution-end

pub fn receipt_line(item: &str, quantity: u32, unit_price_cents: u32) -> String {
    // @solution-start
    let total = u64::from(quantity) * u64::from(unit_price_cents);
    format!(
        "{item:<10}{quantity:>3} x ${} = ${}",
        dollars(u64::from(unit_price_cents)),
        dollars(total)
    )
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_apple() {
        assert_eq!(receipt_line("apple", 3, 50), "apple       3 x $0.50 = $1.50");
    }

    #[test]
    fn example_exact_width_name() {
        assert_eq!(receipt_line("watermelon", 12, 405), "watermelon 12 x $4.05 = $48.60");
    }

    #[test]
    fn example_long_name_and_quantity() {
        assert_eq!(
            receipt_line("extraordinary", 1000, 7),
            "extraordinary1000 x $0.07 = $70.00"
        );
    }

    #[test]
    fn zero_values_and_empty_name() {
        assert_eq!(receipt_line("", 0, 0), "            0 x $0.00 = $0.00");
    }

    #[test]
    fn total_does_not_overflow() {
        assert_eq!(
            receipt_line("gold", u32::MAX, u32::MAX),
            // (2^32 - 1)^2 = 18_446_744_065_119_617_025 cents.
            "gold      4294967295 x $42949672.95 = $184467440651196170.25"
        );
    }
}
