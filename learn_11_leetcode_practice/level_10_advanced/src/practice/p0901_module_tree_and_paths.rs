// ==================================================================
// PRACTICE 0901: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0901_module_tree_and_paths.md
// Run the tests:          cargo test practice::p0901
// ==================================================================

//! # Problem 0901: Modules, Paths and `pub(crate)`
//!
//! - **Difficulty:** Medium
//! - **Topics:** inline `mod` blocks, `self`/`super` paths, `pub(crate)`, `pub(super)`, private items
//! - **Source:** Original · [The Rust Book 7.2](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) · [Reference: Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
//!
//! ## Problem
//!
//! A small shop is organised as a tree of modules **inside this one file**:
//!
//! ```text
//! shop                      (pub)
//! ├── inventory             (pub)  Item, find (pub(super)), total_units (pub(crate))
//! ├── billing               (pub)  BillingError, line_total, invoice, discounted (private)
//! └── stock_report          (pub fn in shop)
//! ```
//!
//! The module tree and every signature (including its visibility) are given. Fill in the bodies,
//! using **relative paths** (`self::...`, `super::...`) so the code keeps working when the whole file
//! is moved somewhere else in the crate (it is: the same file is compiled as `solutions::p0901...`
//! and as `practice::p0901...`).
//!
//! - `inventory::find(items, name) -> Option<&Item>`: the first item whose `name` equals `name`.
//!   It is `pub(super)`: `shop` and its children (`billing`) may call it, code outside `shop` may not.
//! - `inventory::total_units(items) -> u32`: the sum of every item's `stock`. It is `pub(crate)`.
//! - `billing::line_total(items, name, quantity) -> Result<u64, BillingError>`: the price of buying
//!   `quantity` units of `name`, i.e. `price_cents * quantity`. Errors:
//!   `BillingError::UnknownItem(name)` if no item has that name, and
//!   `BillingError::OutOfStock { name, requested, available }` if `quantity > stock`.
//! - `billing::discounted(subtotal) -> u64` (private to `billing`): totals of at least `10_000` cents
//!   get 10% off (`subtotal - subtotal / 10`, integer division); smaller totals are unchanged.
//! - `billing::invoice(items, order) -> Result<u64, BillingError>`: `order` is a list of
//!   `(name, quantity)` lines. Every line is checked **independently** against the stock (a name may
//!   appear twice). Return the first error in order, or the discounted sum of all line totals.
//! - `shop::stock_report(items) -> String`: `"<number of items> items, <total units> units"`.
//!
//! ## Examples
//!
//! ```text
//! items = [Item::new("pen", 150, 10), Item::new("book", 2_500, 4)]
//!
//! billing::line_total(&items, "pen", 4)           => Ok(600)
//! billing::line_total(&items, "book", 5)          => Err(OutOfStock { name: "book", requested: 5, available: 4 })
//! billing::line_total(&items, "ink", 1)           => Err(UnknownItem("ink"))
//! billing::invoice(&items, &[("pen", 2), ("book", 4)])  => Ok(9_270)   (10_300 minus 10%)
//! billing::invoice(&items, &[("pen", 2)])         => Ok(300)           (no discount)
//! shop::stock_report(&items)                      => "2 items, 14 units"
//! ```
//!
//! ## Constraints
//!
//! - Prices and quantities are small enough that no product overflows `u64`.
//! - Do not change any visibility modifier: the point is to see which paths are allowed.
//!
//! ## Hints
//!
//! 1. Inside `billing`, the sibling module `inventory` is `super::inventory`.
//! 2. Inside `shop` itself, a child module is reachable as `self::inventory` (or just `inventory`).
//! 3. Summing an iterator of `Result<u64, E>` into a `Result<u64, E>` stops at the first error.

pub mod shop {
    use self::inventory::Item;

    pub mod inventory {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Item {
            pub name: String,
            pub price_cents: u64,
            pub stock: u32,
        }

        impl Item {
            pub fn new(name: &str, price_cents: u64, stock: u32) -> Self {
                Item {
                    name: name.to_string(),
                    price_cents,
                    stock,
                }
            }
        }

        /// Visible in `shop` and its descendants only.
        pub(super) fn find<'a>(items: &'a [Item], name: &str) -> Option<&'a Item> {
            todo!("Write your solution here")
        }

        /// Visible everywhere in this crate.
        pub(crate) fn total_units(items: &[Item]) -> u32 {
            todo!("Write your solution here")
        }
    }

    pub mod billing {
        use super::inventory::Item;

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum BillingError {
            UnknownItem(String),
            OutOfStock {
                name: String,
                requested: u32,
                available: u32,
            },
        }

        pub fn line_total(items: &[Item], name: &str, quantity: u32) -> Result<u64, BillingError> {
            todo!("Write your solution here")
        }

        pub fn invoice(items: &[Item], order: &[(&str, u32)]) -> Result<u64, BillingError> {
            todo!("Write your solution here")
        }

        /// Private to `billing`: 10% off totals of at least 10_000 cents.
        fn discounted(subtotal: u64) -> u64 {
            todo!("Write your solution here")
        }
    }

    pub fn stock_report(items: &[Item]) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::shop;
    use super::shop::billing::{self, BillingError};
    use super::shop::inventory::{self, Item};

    fn items() -> Vec<Item> {
        vec![Item::new("pen", 150, 10), Item::new("book", 2_500, 4)]
    }

    #[test]
    fn line_total_success_and_errors() {
        let items = items();
        assert_eq!(billing::line_total(&items, "pen", 4), Ok(600));
        assert_eq!(
            billing::line_total(&items, "book", 5),
            Err(BillingError::OutOfStock {
                name: "book".to_string(),
                requested: 5,
                available: 4
            })
        );
        assert_eq!(
            billing::line_total(&items, "ink", 1),
            Err(BillingError::UnknownItem("ink".to_string()))
        );
    }

    #[test]
    fn invoice_applies_the_discount_at_the_threshold() {
        let items = items();
        assert_eq!(
            billing::invoice(&items, &[("pen", 2), ("book", 4)]),
            Ok(9_270)
        );
        assert_eq!(billing::invoice(&items, &[("pen", 2)]), Ok(300));
        let exact = vec![Item::new("lamp", 10_000, 1), Item::new("bulb", 9_999, 1)];
        assert_eq!(billing::invoice(&exact, &[("lamp", 1)]), Ok(9_000));
        assert_eq!(billing::invoice(&exact, &[("bulb", 1)]), Ok(9_999));
    }

    #[test]
    fn invoice_returns_the_first_error() {
        let items = items();
        assert_eq!(
            billing::invoice(&items, &[("pen", 1), ("ink", 1), ("book", 9)]),
            Err(BillingError::UnknownItem("ink".to_string()))
        );
        assert_eq!(billing::invoice(&items, &[]), Ok(0));
    }

    #[test]
    fn lines_are_checked_independently() {
        let items = items();
        // 4 + 4 books: each line alone fits the stock of 4 (20_000 minus 10%).
        assert_eq!(
            billing::invoice(&items, &[("book", 4), ("book", 4)]),
            Ok(18_000)
        );
    }

    #[test]
    fn crate_visible_helpers_and_report() {
        let items = items();
        assert_eq!(inventory::total_units(&items), 14);
        assert_eq!(shop::stock_report(&items), "2 items, 14 units");
        assert_eq!(shop::stock_report(&[]), "0 items, 0 units");
    }
}
