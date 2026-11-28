# Problem 0462: Extremes with min_by_key and max_by

- **Difficulty:** Easy
- **Topics:** `min_by_key`, `max_by_key`, `max_by` with `f64::total_cmp`, `cmp::Reverse`, `next_back` vs `last`, `nth`, tie-breaking rules
- **Source:** Original · [std `Iterator::max_by`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by) · [std `Iterator::min_by_key`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.min_by_key)

## Problem

A shop has a list of products. Answer some "extreme" questions with iterator consumers. Pay
attention to ties: `min_by_key` / `min_by` return the **first** minimum, `max_by_key` / `max_by`
return the **last** maximum.

Implement (all return `None` for an empty list):

- `cheapest(products: &[Product]) -> Option<&Product>`: lowest `price_cents`; ties -> the first.
- `best_rated(products: &[Product]) -> Option<&Product>`: highest `rating` (an `f64`); ties ->
  the **last** one.
- `longest_name(products: &[Product]) -> Option<&str>`: the name with the most characters; ties
  -> the **first** one.
- `last_name(products: &[Product]) -> Option<&str>`: the name of the last product.
- `nth_cheapest(products: &[Product], n: usize) -> Option<&Product>`: the product at position
  `n` (0-based) when sorted by price (stable, so equal prices keep list order); `None` if
  `n >= len`.
- `price_range(products: &[Product]) -> Option<(u32, u32)>`: `(lowest price, highest price)`.

## Examples

```text
products = [("tea", 300, 4.5), ("cake", 450, 4.8), ("coffee", 300, 4.8), ("water", 100, 3.0)]
cheapest       -> water
best_rated     -> coffee        (4.8 ties with cake; the last maximum wins)
longest_name   -> "coffee"
last_name      -> "water"
nth_cheapest 1 -> tea           (sorted: water 100, tea 300, coffee 300, cake 450)
price_range    -> (100, 450)
```

## Constraints

- At most `10^4` products; ratings are never NaN.

## Hints

1. `f64` does not implement `Ord`, so `max_by_key(|p| p.rating)` does not compile; use
   `max_by(|a, b| a.rating.total_cmp(&b.rating))`.
2. For "first of the longest", minimise `Reverse(length)`: `min_by_key(|p| Reverse(...))`.
3. `nth(n)` skips `n` items and returns the next one; `next_back()` takes an item from the end.

## Starter code

Write your code in [`src/practice/p0462_extremes_min_max_by.rs`](../src/practice/p0462_extremes_min_max_by.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0462
```

```rust
use std::cmp::Reverse;

#[derive(Debug, PartialEq)]
pub struct Product {
    pub name: String,
    pub price_cents: u32,
    pub rating: f64,
}

pub fn cheapest(products: &[Product]) -> Option<&Product> {
    todo!("Write your solution here")
}

pub fn best_rated(products: &[Product]) -> Option<&Product> {
    todo!("Write your solution here")
}

pub fn longest_name(products: &[Product]) -> Option<&str> {
    todo!("Write your solution here")
}

pub fn last_name(products: &[Product]) -> Option<&str> {
    todo!("Write your solution here")
}

pub fn nth_cheapest(products: &[Product], n: usize) -> Option<&Product> {
    todo!("Write your solution here")
}

pub fn price_range(products: &[Product]) -> Option<(u32, u32)> {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn product(name: &str, price_cents: u32, rating: f64) -> Product {
        Product {
            name: name.to_string(),
            price_cents,
            rating,
        }
    }

    fn shop() -> Vec<Product> {
        vec![
            product("tea", 300, 4.5),
            product("cake", 450, 4.8),
            product("coffee", 300, 4.8),
            product("water", 100, 3.0),
        ]
    }

    fn name(p: Option<&Product>) -> Option<&str> {
        p.map(|p| p.name.as_str())
    }

    #[test]
    fn cheapest_and_best_rated() {
        let items = shop();
        assert_eq!(name(cheapest(&items)), Some("water"));
        assert_eq!(name(best_rated(&items)), Some("coffee"));
        let ties = vec![product("a", 5, 1.0), product("b", 5, 1.0)];
        assert_eq!(name(cheapest(&ties)), Some("a"));
        assert_eq!(name(best_rated(&ties)), Some("b"));
    }

    #[test]
    fn names() {
        let items = shop();
        assert_eq!(longest_name(&items), Some("coffee"));
        assert_eq!(last_name(&items), Some("water"));
        let ties = vec![
            product("ab", 1, 0.0),
            product("cd", 1, 0.0),
            product("e", 1, 0.0),
        ];
        assert_eq!(longest_name(&ties), Some("ab"));
    }

    #[test]
    fn nth_by_price() {
        let items = shop();
        assert_eq!(name(nth_cheapest(&items, 0)), Some("water"));
        assert_eq!(name(nth_cheapest(&items, 1)), Some("tea"));
        assert_eq!(name(nth_cheapest(&items, 2)), Some("coffee"));
        assert_eq!(name(nth_cheapest(&items, 3)), Some("cake"));
        assert_eq!(nth_cheapest(&items, 4), None);
    }

    #[test]
    fn ranges() {
        assert_eq!(price_range(&shop()), Some((100, 450)));
        assert_eq!(price_range(&[product("x", 7, 1.0)]), Some((7, 7)));
    }

    #[test]
    fn empty_shop() {
        assert_eq!(cheapest(&[]), None);
        assert_eq!(best_rated(&[]), None);
        assert_eq!(longest_name(&[]), None);
        assert_eq!(last_name(&[]), None);
        assert_eq!(nth_cheapest(&[], 0), None);
        assert_eq!(price_range(&[]), None);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every question is one consumer. The only subtle part is ties, and the rule is easy to
remember: minimum functions keep the first candidate, maximum functions keep the last. To get
"first maximum" use a minimum on a reversed key.

**Rust concepts:**
- `min_by_key` / `max_by_key` need an `Ord` key; `min_by` / `max_by` take a comparison closure.
- `f64::total_cmp` defines a total order on floats (NaN included), suitable for `max_by`.
- `std::cmp::Reverse` flips the order of a key.
- `Iterator::last()` walks the whole iterator to find the final item. A slice iterator (and
  `map` over it) is a `DoubleEndedIterator`, so `next_back()` gets the last item in `O(1)`;
  Clippy's `double_ended_iterator_last` lint suggests exactly this. (`products.last()` on the
  slice itself works too.) `nth(n)` consumes the first `n + 1` items.
- `min().zip(max())` combines two `Option`s into an `Option` of a pair.

```rust
use std::cmp::Reverse;

#[derive(Debug, PartialEq)]
pub struct Product {
    pub name: String,
    pub price_cents: u32,
    pub rating: f64,
}

pub fn cheapest(products: &[Product]) -> Option<&Product> {
    products.iter().min_by_key(|p| p.price_cents)
}

pub fn best_rated(products: &[Product]) -> Option<&Product> {
    products.iter().max_by(|a, b| a.rating.total_cmp(&b.rating))
}

pub fn longest_name(products: &[Product]) -> Option<&str> {
    products
        .iter()
        .min_by_key(|p| Reverse(p.name.chars().count()))
        .map(|p| p.name.as_str())
}

pub fn last_name(products: &[Product]) -> Option<&str> {
    products.iter().map(|p| p.name.as_str()).next_back()
}

pub fn nth_cheapest(products: &[Product], n: usize) -> Option<&Product> {
    let mut by_price: Vec<&Product> = products.iter().collect();
    by_price.sort_by_key(|p| p.price_cents);
    by_price.into_iter().nth(n)
}

pub fn price_range(products: &[Product]) -> Option<(u32, u32)> {
    let prices = || products.iter().map(|p| p.price_cents);
    prices().min().zip(prices().max())
}
```

Full tested solution: [`src/solutions/p0462_extremes_min_max_by.rs`](../src/solutions/p0462_extremes_min_max_by.rs)

</details>
