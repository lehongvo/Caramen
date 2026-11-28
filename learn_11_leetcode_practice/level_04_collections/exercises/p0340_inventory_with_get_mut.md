# Problem 0340: An Inventory with get and get_mut

- **Difficulty:** Easy
- **Topics:** `HashMap::insert` returns the old value, `get`, `get_mut`, `remove`, `contains_key`, `#[derive(Default)]`
- **Source:** Original · [The Rust Book 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) · [`HashMap::get_mut`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut)

## Problem

A shop keeps its stock in a struct `Inventory` that maps item names to quantities. Implement:

- `Inventory::new() -> Self`: an empty inventory (also available as `Inventory::default()`);
- `set(&mut self, item: &str, qty: u32) -> Option<u32>`: store exactly `qty` for `item` and
  return the previous quantity, if the item was known;
- `quantity(&self, item: &str) -> u32`: the stored quantity, `0` for unknown items;
- `take(&mut self, item: &str, qty: u32) -> bool`: if at least `qty` units are in stock,
  remove them and return `true`; otherwise change nothing and return `false`. When an item's
  quantity drops to `0`, the item is removed from the inventory completely;
- `discontinue(&mut self, item: &str) -> Option<u32>`: remove the item and return its quantity;
- `item_count(&self) -> usize`: the number of different items stored;
- `has(&self, item: &str) -> bool`: whether the item is stored.

## Examples

```text
let mut inv = Inventory::new();
inv.set("apple", 5)     => None
inv.set("apple", 7)     => Some(5)
inv.take("apple", 3)    => true     (4 left)
inv.take("apple", 9)    => false    (still 4)
inv.take("pear", 1)     => false    (unknown item)
inv.take("apple", 4)    => true     (0 left: "apple" is removed)
inv.has("apple")        => false
inv.set("kiwi", 2); inv.discontinue("kiwi") => Some(2)
inv.item_count()        => 0
```

## Constraints

- At most `10^5` operations.

## Hints

1. `HashMap::insert` returns `Option<V>`: the value that was replaced.
2. `get_mut(key)` returns `Option<&mut V>`, which lets you change a value in place.
3. Finish using the `&mut` reference before calling `remove` (the borrow checker insists).

## Starter code

Write your code in [`src/practice/p0340_inventory_with_get_mut.rs`](../src/practice/p0340_inventory_with_get_mut.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0340
```

```rust
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
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
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
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a thin wrapper around `HashMap<String, u32>` where each method maps onto one or two
map operations. `take` is the interesting one: it uses `get_mut` to check and decrement the
quantity in place, remembers whether the quantity reached zero, and only then removes the item.

**Rust concepts:**
- `insert(k, v)` overwrites and returns the old value: exactly what `set` needs.
- `get_mut` avoids a second lookup for "read, then write". The `&mut u32` it returns borrows the
  map, so `self.items.remove(item)` is only allowed after the last use of that reference
  (non-lexical lifetimes make this work inside the same function).
- `let Some(stock) = ... else { return false; };` handles the unknown item.
- `#[derive(Default)]` creates an empty map, so `new` can simply return `Self::default()`;
  providing `Default` also keeps Clippy's `new_without_default` lint quiet.

```rust
use std::collections::HashMap;

/// Item names mapped to quantities in stock.
#[derive(Debug, Default)]
pub struct Inventory {
    items: HashMap<String, u32>,
}

impl Inventory {
    /// Creates an empty inventory.
    pub fn new() -> Self {
        Self::default()
    }

    /// Stores `qty` for `item` and returns the previous quantity.
    pub fn set(&mut self, item: &str, qty: u32) -> Option<u32> {
        self.items.insert(item.to_string(), qty)
    }

    /// Returns the quantity of `item` (0 if unknown).
    pub fn quantity(&self, item: &str) -> u32 {
        self.items.get(item).copied().unwrap_or(0)
    }

    /// Removes `qty` units if enough are in stock.
    pub fn take(&mut self, item: &str, qty: u32) -> bool {
        let Some(stock) = self.items.get_mut(item) else {
            return false;
        };
        if *stock < qty {
            return false;
        }
        *stock -= qty;
        let empty = *stock == 0;
        if empty {
            self.items.remove(item);
        }
        true
    }

    /// Removes the item and returns its quantity.
    pub fn discontinue(&mut self, item: &str) -> Option<u32> {
        self.items.remove(item)
    }

    /// Returns the number of different items.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Returns whether `item` is stored.
    pub fn has(&self, item: &str) -> bool {
        self.items.contains_key(item)
    }
}
```

Full tested solution: [`src/solutions/p0340_inventory_with_get_mut.rs`](../src/solutions/p0340_inventory_with_get_mut.rs)

</details>
