// ==================================================================
// PRACTICE 0340: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0340_inventory_with_get_mut.md
// Run the tests:          cargo test practice::p0340
// ==================================================================

//! # Problem 0340: An Inventory with get and get_mut
//!
//! - **Difficulty:** Easy
//! - **Topics:** `HashMap::insert` returns the old value, `get`, `get_mut`, `remove`, `contains_key`, `#[derive(Default)]`
//! - **Source:** Original · [The Rust Book 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) · [`HashMap::get_mut`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut)
//!
//! ## Problem
//!
//! A shop keeps its stock in a struct `Inventory` that maps item names to quantities. Implement:
//!
//! - `Inventory::new() -> Self`: an empty inventory (also available as `Inventory::default()`);
//! - `set(&mut self, item: &str, qty: u32) -> Option<u32>`: store exactly `qty` for `item` and
//!   return the previous quantity, if the item was known;
//! - `quantity(&self, item: &str) -> u32`: the stored quantity, `0` for unknown items;
//! - `take(&mut self, item: &str, qty: u32) -> bool`: if at least `qty` units are in stock,
//!   remove them and return `true`; otherwise change nothing and return `false`. When an item's
//!   quantity drops to `0`, the item is removed from the inventory completely;
//! - `discontinue(&mut self, item: &str) -> Option<u32>`: remove the item and return its quantity;
//! - `item_count(&self) -> usize`: the number of different items stored;
//! - `has(&self, item: &str) -> bool`: whether the item is stored.
//!
//! ## Examples
//!
//! ```text
//! let mut inv = Inventory::new();
//! inv.set("apple", 5)     => None
//! inv.set("apple", 7)     => Some(5)
//! inv.take("apple", 3)    => true     (4 left)
//! inv.take("apple", 9)    => false    (still 4)
//! inv.take("pear", 1)     => false    (unknown item)
//! inv.take("apple", 4)    => true     (0 left: "apple" is removed)
//! inv.has("apple")        => false
//! inv.set("kiwi", 2); inv.discontinue("kiwi") => Some(2)
//! inv.item_count()        => 0
//! ```
//!
//! ## Constraints
//!
//! - At most `10^5` operations.
//!
//! ## Hints
//!
//! 1. `HashMap::insert` returns `Option<V>`: the value that was replaced.
//! 2. `get_mut(key)` returns `Option<&mut V>`, which lets you change a value in place.
//! 3. Finish using the `&mut` reference before calling `remove` (the borrow checker insists).

use std::collections::HashMap;

/// Item names mapped to quantities in stock.
#[derive(Debug, Default)]
pub struct Inventory {}

impl Inventory {
    /// Creates an empty inventory.
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    /// Stores `qty` for `item` and returns the previous quantity.
    pub fn set(&mut self, item: &str, qty: u32) -> Option<u32> {
        todo!("Write your solution here")
    }

    /// Returns the quantity of `item` (0 if unknown).
    pub fn quantity(&self, item: &str) -> u32 {
        todo!("Write your solution here")
    }

    /// Removes `qty` units if enough are in stock.
    pub fn take(&mut self, item: &str, qty: u32) -> bool {
        todo!("Write your solution here")
    }

    /// Removes the item and returns its quantity.
    pub fn discontinue(&mut self, item: &str) -> Option<u32> {
        todo!("Write your solution here")
    }

    /// Returns the number of different items.
    pub fn item_count(&self) -> usize {
        todo!("Write your solution here")
    }

    /// Returns whether `item` is stored.
    pub fn has(&self, item: &str) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut inv = Inventory::new();
        assert_eq!(inv.set("apple", 5), None);
        assert_eq!(inv.set("apple", 7), Some(5));
        assert!(inv.take("apple", 3));
        assert_eq!(inv.quantity("apple"), 4);
        assert!(!inv.take("apple", 9));
        assert_eq!(inv.quantity("apple"), 4);
        assert!(!inv.take("pear", 1));
        assert!(inv.take("apple", 4));
        assert!(!inv.has("apple"));
        inv.set("kiwi", 2);
        assert_eq!(inv.discontinue("kiwi"), Some(2));
        assert_eq!(inv.item_count(), 0);
    }

    #[test]
    fn unknown_items() {
        let mut inv = Inventory::default();
        assert_eq!(inv.quantity("ghost"), 0);
        assert_eq!(inv.discontinue("ghost"), None);
        assert!(!inv.has("ghost"));
        assert!(!inv.take("ghost", 0));
        assert_eq!(inv.item_count(), 0);
    }

    #[test]
    fn set_to_zero_keeps_the_item() {
        let mut inv = Inventory::new();
        inv.set("bolt", 0);
        assert!(inv.has("bolt"));
        assert_eq!(inv.item_count(), 1);
        assert!(inv.take("bolt", 0));
        assert!(!inv.has("bolt"));
    }

    #[test]
    fn many_items() {
        let mut inv = Inventory::new();
        for i in 0..10_000u32 {
            inv.set(&format!("item{i}"), i % 10);
        }
        assert_eq!(inv.item_count(), 10_000);
        let taken = (0..10_000u32)
            .filter(|i| inv.take(&format!("item{i}"), 5))
            .count();
        assert_eq!(taken, 5_000);
        // Items with exactly 5 units dropped to zero and were removed.
        assert_eq!(inv.item_count(), 9_000);
        assert_eq!(inv.quantity("item9"), 4);
    }
}
