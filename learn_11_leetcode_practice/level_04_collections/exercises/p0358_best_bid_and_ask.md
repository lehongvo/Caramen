# Problem 0358: Best Bid and Ask

- **Difficulty:** Easy
- **Topics:** `BTreeMap::first_key_value`, `last_key_value`, `pop_first`, `pop_last`, `entry().or_insert`
- **Source:** Original · [`BTreeMap::first_key_value`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.first_key_value) · [`BTreeMap::pop_last`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.pop_last)

## Problem

A tiny stock-exchange **order book** keeps two sides, each mapping a price to the total quantity
offered at that price:

- **bids** (buyers): the *best* bid is the **highest** price;
- **asks** (sellers): the *best* ask is the **lowest** price.

Implement `OrderBook` with two `BTreeMap<u32, u32>`:

- `OrderBook::new() -> Self` (also `OrderBook::default()`): an empty book;
- `add_bid(&mut self, price: u32, qty: u32)` and `add_ask(&mut self, price: u32, qty: u32)`: add
  `qty` to that price level (creating it if needed);
- `best_bid(&self) -> Option<(u32, u32)>` and `best_ask(&self) -> Option<(u32, u32)>`: the best
  `(price, quantity)` of each side, `None` if that side is empty;
- `spread(&self) -> Option<i64>`: `best ask price - best bid price` (negative when the book is
  "crossed"), `None` if either side is empty;
- `take_best_bid(&mut self)` and `take_best_ask(&mut self)`, both `-> Option<(u32, u32)>`: remove
  the best level of that side and return it.

## Examples

```text
add_bid(99, 5); add_bid(101, 2); add_bid(99, 1)
add_ask(105, 4); add_ask(103, 7)
best_bid()      => Some((101, 2))
best_ask()      => Some((103, 7))
spread()        => Some(2)
take_best_bid() => Some((101, 2))
best_bid()      => Some((99, 6))     (5 + 1 at price 99)
spread()        => Some(4)
```

## Constraints

- At most `10^5` operations; the total quantity at one price fits in `u32`.

## Hints

1. A `BTreeMap` knows its smallest and largest keys: `first_key_value()` and `last_key_value()`
   run in `O(log n)`.
2. `pop_first()` and `pop_last()` remove and return those entries.
3. These methods return references (`Option<(&K, &V)>`) or owned pairs (`Option<(K, V)>`);
   convert as needed.

## Starter code

Write your code in [`src/practice/p0358_best_bid_and_ask.rs`](../src/practice/p0358_best_bid_and_ask.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0358
```

```rust
use std::collections::BTreeMap;

/// Price levels of buyers (bids) and sellers (asks).
#[derive(Debug, Default)]
pub struct OrderBook {}

impl OrderBook {
    /// Creates an empty order book.
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    /// Adds `qty` to the bid level at `price`.
    pub fn add_bid(&mut self, price: u32, qty: u32) {
        todo!("Write your solution here")
    }

    /// Adds `qty` to the ask level at `price`.
    pub fn add_ask(&mut self, price: u32, qty: u32) {
        todo!("Write your solution here")
    }

    /// The highest bid `(price, quantity)`.
    pub fn best_bid(&self) -> Option<(u32, u32)> {
        todo!("Write your solution here")
    }

    /// The lowest ask `(price, quantity)`.
    pub fn best_ask(&self) -> Option<(u32, u32)> {
        todo!("Write your solution here")
    }

    /// Best ask price minus best bid price.
    pub fn spread(&self) -> Option<i64> {
        todo!("Write your solution here")
    }

    /// Removes and returns the highest bid level.
    pub fn take_best_bid(&mut self) -> Option<(u32, u32)> {
        todo!("Write your solution here")
    }

    /// Removes and returns the lowest ask level.
    pub fn take_best_ask(&mut self) -> Option<(u32, u32)> {
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

    fn example() -> OrderBook {
        let mut book = OrderBook::new();
        book.add_bid(99, 5);
        book.add_bid(101, 2);
        book.add_bid(99, 1);
        book.add_ask(105, 4);
        book.add_ask(103, 7);
        book
    }

    #[test]
    fn example_sequence() {
        let mut book = example();
        assert_eq!(book.best_bid(), Some((101, 2)));
        assert_eq!(book.best_ask(), Some((103, 7)));
        assert_eq!(book.spread(), Some(2));
        assert_eq!(book.take_best_bid(), Some((101, 2)));
        assert_eq!(book.best_bid(), Some((99, 6)));
        assert_eq!(book.spread(), Some(4));
    }

    #[test]
    fn empty_sides() {
        let mut book = OrderBook::default();
        assert_eq!(book.best_bid(), None);
        assert_eq!(book.best_ask(), None);
        assert_eq!(book.spread(), None);
        assert_eq!(book.take_best_ask(), None);
        book.add_ask(10, 1);
        assert_eq!(book.spread(), None);
    }

    #[test]
    fn crossed_book_has_negative_spread() {
        let mut book = OrderBook::new();
        book.add_bid(12, 1);
        book.add_ask(10, 1);
        assert_eq!(book.spread(), Some(-2));
    }

    #[test]
    fn draining_asks_in_price_order() {
        let mut book = example();
        assert_eq!(book.take_best_ask(), Some((103, 7)));
        assert_eq!(book.take_best_ask(), Some((105, 4)));
        assert_eq!(book.take_best_ask(), None);
    }

    #[test]
    fn many_levels() {
        let mut book = OrderBook::new();
        for i in 0..50_000u32 {
            book.add_bid(1_000 + (i * 7) % 5_000, 1);
            book.add_ask(10_000 + (i * 11) % 5_000, 2);
        }
        assert_eq!(book.best_bid(), Some((5_999, 10)));
        assert_eq!(book.best_ask(), Some((10_000, 20)));
        assert_eq!(book.spread(), Some(4_001));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** both sides are sorted by price, so the best bid is the **last** entry of `bids` and the
best ask is the **first** entry of `asks`. Every query is one call to the matching `BTreeMap`
method.

**Rust concepts:**
- `first_key_value()` / `last_key_value()` return `Option<(&u32, &u32)>`;
  `.map(|(&p, &q)| (p, q))` copies the numbers out.
- `pop_first()` / `pop_last()` (stable since Rust 1.66) return owned `Option<(K, V)>`.
- `spread` combines two options: `let (Some(bid), Some(ask)) = (..., ...) else { return None; }`.
  Converting to `i64` before subtracting avoids `u32` underflow when the book is crossed.

```rust
use std::collections::BTreeMap;

/// Price levels of buyers (bids) and sellers (asks).
#[derive(Debug, Default)]
pub struct OrderBook {
    bids: BTreeMap<u32, u32>,
    asks: BTreeMap<u32, u32>,
}

impl OrderBook {
    /// Creates an empty order book.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds `qty` to the bid level at `price`.
    pub fn add_bid(&mut self, price: u32, qty: u32) {
        *self.bids.entry(price).or_insert(0) += qty;
    }

    /// Adds `qty` to the ask level at `price`.
    pub fn add_ask(&mut self, price: u32, qty: u32) {
        *self.asks.entry(price).or_insert(0) += qty;
    }

    /// The highest bid `(price, quantity)`.
    pub fn best_bid(&self) -> Option<(u32, u32)> {
        self.bids.last_key_value().map(|(&p, &q)| (p, q))
    }

    /// The lowest ask `(price, quantity)`.
    pub fn best_ask(&self) -> Option<(u32, u32)> {
        self.asks.first_key_value().map(|(&p, &q)| (p, q))
    }

    /// Best ask price minus best bid price.
    pub fn spread(&self) -> Option<i64> {
        let (Some((bid, _)), Some((ask, _))) = (self.best_bid(), self.best_ask()) else {
            return None;
        };
        Some(ask as i64 - bid as i64)
    }

    /// Removes and returns the highest bid level.
    pub fn take_best_bid(&mut self) -> Option<(u32, u32)> {
        self.bids.pop_last()
    }

    /// Removes and returns the lowest ask level.
    pub fn take_best_ask(&mut self) -> Option<(u32, u32)> {
        self.asks.pop_first()
    }
}
```

Full tested solution: [`src/solutions/p0358_best_bid_and_ask.rs`](../src/solutions/p0358_best_bid_and_ask.rs)

</details>
