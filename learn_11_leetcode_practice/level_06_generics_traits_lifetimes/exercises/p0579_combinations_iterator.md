# Problem 0579: A Generic Combinations Iterator

- **Difficulty:** Medium
- **Topics:** an iterator borrowing `&'a [T]` and yielding `Vec<&'a T>`, index-vector state machine, no `Clone` bound on `T`
- **Source:** Original · [The Rust Book 13.2](https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method)

## Problem

Generate all `k`-element combinations of a slice **lazily**, in lexicographic order of positions.
For `['a', 'b', 'c', 'd']` and `k = 2` the order is `ab, ac, ad, bc, bd, cd`.

Implement `Combinations<'a, T>` with:

- `Combinations::new(items: &'a [T], k: usize) -> Self`
- `impl<'a, T> Iterator for Combinations<'a, T>` with `type Item = Vec<&'a T>`: each combination is
  a vector of references into `items`, so `T` needs no `Clone` bound.

and the helper `combinations<T>(items: &[T], k: usize) -> Combinations<'_, T>`.

Special cases: `k == 0` yields exactly one empty combination; `k > items.len()` yields nothing.

The iterator stores `items`, the current index vector `indices: Vec<usize>` (strictly increasing)
and a `done: bool` flag.

## Examples

```text
combinations(&['a', 'b', 'c', 'd'], 2)  => [a, b], [a, c], [a, d], [b, c], [b, d], [c, d]
combinations(&[1, 2, 3], 3)             => [1, 2, 3]
combinations(&[1, 2, 3], 0)             => []          (one empty combination)
combinations(&[1, 2, 3], 4)             => (nothing)
combinations(&(0..20).collect::<Vec<_>>(), 10).count() => 184756
```

## Constraints

- `0 <= k`, `items.len() <= 30`

## Hints

1. Start with `indices = [0, 1, ..., k - 1]`.
2. To advance, find the rightmost position `i` with `indices[i] < n - k + i`, increment it, and reset
   every later position to "previous + 1". If there is no such `i`, you are done.

## Starter code

Write your code in [`src/practice/p0579_combinations_iterator.rs`](../src/practice/p0579_combinations_iterator.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0579
```

```rust
/// Lazily yields all `k`-combinations of a slice, as references.
#[derive(Debug, Clone)]
pub struct Combinations<'a, T> {
    items: &'a [T],
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T> Combinations<'a, T> {
    pub fn new(items: &'a [T], k: usize) -> Self {
        todo!("Write your solution here")
    }
}

impl<'a, T> Iterator for Combinations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        todo!("Write your solution here")
    }
}

/// All `k`-combinations of `items`.
pub fn combinations<T>(items: &[T], k: usize) -> Combinations<'_, T> {
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

    fn as_strings(items: &[char], k: usize) -> Vec<String> {
        combinations(items, k)
            .map(|c| c.into_iter().collect())
            .collect()
    }

    #[test]
    fn pairs_in_lexicographic_order() {
        assert_eq!(
            as_strings(&['a', 'b', 'c', 'd'], 2),
            vec!["ab", "ac", "ad", "bc", "bd", "cd"]
        );
    }

    #[test]
    fn edge_sizes() {
        assert_eq!(
            combinations(&[1, 2, 3], 3).collect::<Vec<_>>(),
            vec![vec![&1, &2, &3]]
        );
        assert_eq!(
            combinations(&[1, 2, 3], 0).collect::<Vec<_>>(),
            vec![Vec::<&i32>::new()]
        );
        assert_eq!(combinations(&[1, 2, 3], 4).count(), 0);
        assert_eq!(combinations::<u8>(&[], 0).count(), 1);
    }

    #[test]
    fn works_without_clone_and_outlives_iterator() {
        struct Card(u8); // not Clone
        let cards = [Card(1), Card(2), Card(3)];
        let all: Vec<Vec<&Card>> = Combinations::new(&cards, 2).collect();
        let sums: Vec<u8> = all
            .iter()
            .map(|c| c.iter().map(|card| card.0).sum())
            .collect();
        assert_eq!(sums, vec![3, 4, 5]);
    }

    #[test]
    fn counts_match_binomial_coefficients() {
        let items: Vec<u32> = (0..20).collect();
        assert_eq!(combinations(&items, 10).count(), 184_756);
        assert_eq!(combinations(&items, 1).count(), 20);
        assert_eq!(combinations(&items, 19).count(), 20);
        let last = combinations(&items, 3).last().unwrap();
        assert_eq!(last, vec![&17, &18, &19]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the index vector is the classic odometer for combinations. `next` emits the current
combination and then advances the indices; the rightmost index that can still move is incremented
and the tail is reset to consecutive values.

**Complexity:** `O(k)` per combination (building the output vector and advancing).

**Rust concepts:**
- Returning `Vec<&'a T>` instead of `Vec<T>` lets the iterator work for any `T`, including types that
  cannot be cloned; the references point into the caller's slice.
- The struct's lifetime `'a` flows into the associated type `Item`, so combinations can be stored
  after the iterator is gone.
- `rposition` finds the rightmost index satisfying a condition.

```rust
/// Lazily yields all `k`-combinations of a slice, as references.
#[derive(Debug, Clone)]
pub struct Combinations<'a, T> {
    items: &'a [T],
    indices: Vec<usize>,
    done: bool,
}

impl<'a, T> Combinations<'a, T> {
    pub fn new(items: &'a [T], k: usize) -> Self {
        Combinations {
            items,
            indices: (0..k).collect(),
            done: k > items.len(),
        }
    }
}

impl<'a, T> Iterator for Combinations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Vec<&'a T>> {
        if self.done {
            return None;
        }
        let current: Vec<&'a T> = self.indices.iter().map(|&i| &self.items[i]).collect();
        let n = self.items.len();
        let k = self.indices.len();
        match (0..k).rposition(|i| self.indices[i] < n - k + i) {
            Some(i) => {
                self.indices[i] += 1;
                for j in i + 1..k {
                    self.indices[j] = self.indices[j - 1] + 1;
                }
            }
            None => self.done = true,
        }
        Some(current)
    }
}

/// All `k`-combinations of `items`.
pub fn combinations<T>(items: &[T], k: usize) -> Combinations<'_, T> {
    Combinations::new(items, k)
}
```

Full tested solution: [`src/solutions/p0579_combinations_iterator.rs`](../src/solutions/p0579_combinations_iterator.rs)

</details>
