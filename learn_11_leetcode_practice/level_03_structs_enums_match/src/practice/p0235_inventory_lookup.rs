// ==================================================================
// PRACTICE 0235: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0235_inventory_lookup.md
// Run the tests:          cargo test practice::p0235
// ==================================================================

//! # Problem 0235: Inventory Lookup
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning `Option<&T>` and `Option<&mut T>`, `iter().find`, `iter_mut().find`, `min_by_key`, private `Vec` field
//! - **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) · [`Iterator::find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
//!
//! ## Problem
//!
//! A shop keeps its stock in an `Inventory` (a private list of `Item`s). An `Item` has public fields
//! `sku: String` (a unique code), `name: String`, `quantity: u32` and `price_cents: u32`.
//!
//! Implement on `Inventory`:
//!
//! - `new() -> Inventory`: an empty inventory;
//! - `add(&mut self, item: Item)`: if an item with the same `sku` exists, add the new quantity to it
//!   (keep its name and price); otherwise store the new item;
//! - `get(&self, sku: &str) -> Option<&Item>`;
//! - `get_mut(&mut self, sku: &str) -> Option<&mut Item>`;
//! - `restock(&mut self, sku: &str, amount: u32) -> bool`: add `amount` to the quantity; `false` if
//!   the sku is unknown;
//! - `sell(&mut self, sku: &str, amount: u32) -> Option<u32>`: if the item exists and has at least
//!   `amount` units, remove them and return the total price `amount * price_cents`; otherwise change
//!   nothing and return `None`;
//! - `cheapest_in_stock(&self) -> Option<&Item>`: the item with the lowest price among those with a
//!   quantity above zero (the first one added on ties).
//!
//! ## Examples
//!
//! ```text
//! inv.add(Item { sku: "A1", name: "pen", quantity: 10, price_cents: 150 })
//! inv.add(Item { sku: "B2", name: "pad", quantity: 0,  price_cents: 90 })
//! inv.add(Item { sku: "A1", name: "?",   quantity: 5,  price_cents: 1 })   // merges into A1
//! inv.get("A1").map(|i| i.quantity)     => Some(15)
//! inv.sell("A1", 4)                     => Some(600)
//! inv.sell("A1", 100)                   => None       (not enough stock; nothing changes)
//! inv.cheapest_in_stock().map(|i| &i.sku) => Some("A1")  (B2 is cheaper but out of stock)
//! inv.restock("ZZ", 1)                  => false
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` items; quantities and totals fit in a `u32`.
//!
//! ## Hints
//!
//! 1. `self.items.iter().find(|i| i.sku == sku)` gives `Option<&Item>`; `iter_mut()` gives `Option<&mut Item>`.
//! 2. Implement `restock` and `sell` on top of `get_mut`.
//! 3. `filter` then `min_by_key(|i| i.price_cents)` returns the first minimum.

/// One product line.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub sku: String,
    pub name: String,
    pub quantity: u32,
    pub price_cents: u32,
}

/// The shop's stock.
#[derive(Debug, Default)]
pub struct Inventory {}

impl Inventory {
    /// An empty inventory.
    pub fn new() -> Inventory {
        todo!("Write your solution here")
    }

    /// Adds an item, merging quantities for an existing sku.
    pub fn add(&mut self, item: Item) {
        todo!("Write your solution here")
    }

    /// The item with this sku.
    pub fn get(&self, sku: &str) -> Option<&Item> {
        todo!("Write your solution here")
    }

    /// The item with this sku, mutably.
    pub fn get_mut(&mut self, sku: &str) -> Option<&mut Item> {
        todo!("Write your solution here")
    }

    /// Adds `amount` units; `false` for an unknown sku.
    pub fn restock(&mut self, sku: &str, amount: u32) -> bool {
        todo!("Write your solution here")
    }

    /// Sells `amount` units and returns the total price, or `None` without changes.
    pub fn sell(&mut self, sku: &str, amount: u32) -> Option<u32> {
        todo!("Write your solution here")
    }

    /// The cheapest item that is in stock.
    pub fn cheapest_in_stock(&self) -> Option<&Item> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(sku: &str, name: &str, quantity: u32, price_cents: u32) -> Item {
        Item {
            sku: sku.to_string(),
            name: name.to_string(),
            quantity,
            price_cents,
        }
    }

    fn sample() -> Inventory {
        let mut inv = Inventory::new();
        inv.add(item("A1", "pen", 10, 150));
        inv.add(item("B2", "pad", 0, 90));
        inv.add(item("A1", "?", 5, 1));
        inv
    }

    #[test]
    fn add_merges_by_sku() {
        let inv = sample();
        let a1 = inv.get("A1").expect("A1 exists");
        assert_eq!(a1.quantity, 15);
        assert_eq!(a1.name, "pen");
        assert_eq!(a1.price_cents, 150);
        assert!(inv.get("C3").is_none());
    }

    #[test]
    fn selling() {
        let mut inv = sample();
        assert_eq!(inv.sell("A1", 4), Some(600));
        assert_eq!(inv.sell("A1", 100), None);
        assert_eq!(inv.get("A1").map(|i| i.quantity), Some(11));
        assert_eq!(inv.sell("nope", 1), None);
        assert_eq!(inv.sell("A1", 11), Some(1650));
        assert_eq!(inv.get("A1").map(|i| i.quantity), Some(0));
    }

    #[test]
    fn restock_and_get_mut() {
        let mut inv = sample();
        assert!(inv.restock("B2", 3));
        assert!(!inv.restock("ZZ", 1));
        if let Some(b2) = inv.get_mut("B2") {
            b2.price_cents = 80;
        }
        assert_eq!(inv.get("B2"), Some(&item("B2", "pad", 3, 80)));
    }

    #[test]
    fn cheapest_skips_out_of_stock() {
        let mut inv = sample();
        assert_eq!(inv.cheapest_in_stock().map(|i| i.sku.as_str()), Some("A1"));
        inv.restock("B2", 1);
        assert_eq!(inv.cheapest_in_stock().map(|i| i.sku.as_str()), Some("B2"));
        assert!(Inventory::new().cheapest_in_stock().is_none());
    }

    #[test]
    fn cheapest_tie_keeps_first_added() {
        let mut inv = Inventory::new();
        for i in 0..10_000 {
            inv.add(item(&format!("S{i}"), "x", 1, 500 - (i % 100)));
        }
        // Price 401 first appears at i = 99.
        assert_eq!(inv.cheapest_in_stock().map(|i| i.sku.as_str()), Some("S99"));
    }
}
