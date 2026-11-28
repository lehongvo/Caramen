# Problem 0510: Group Items by a Key

- **Difficulty:** Medium
- **Topics:** several type parameters, closure bounds `F: Fn(&T) -> K`, `K: Ord`, `BTreeMap`
- **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax)

## Problem

Write two generic grouping functions with **three** type parameters: the element type `T`, the
key type `K` and the key-extracting closure type `F`.

- `group_by<T, K, F>(items: Vec<T>, key: F) -> BTreeMap<K, Vec<T>>` moves every element into the
  group of its key. Inside each group, elements keep their original relative order.
- `count_by<T, K, F>(items: &[T], key: F) -> BTreeMap<K, usize>` only counts how many elements
  fall into each key. It borrows the elements.

In both functions the bounds are `K: Ord` and `F: Fn(&T) -> K`; `T` needs no bound at all.
A `BTreeMap` keeps the keys sorted, which makes the result deterministic.

## Examples

```text
group_by(vec!["apple", "bob", "avocado", "cat", "blue"], |w| w.chars().next().unwrap())
    => {'a': ["apple", "avocado"], 'b': ["bob", "blue"], 'c': ["cat"]}
group_by(vec![1, 2, 3, 4, 5, 6], |n| n % 3)
    => {0: [3, 6], 1: [1, 4], 2: [2, 5]}
count_by(&["hi", "hello", "yo"], |w| w.len())
    => {2: 2, 5: 1}
```

## Constraints

- `0 <= items.len() <= 10^5`

## Hints

1. A closure's type has no name, so you accept it through a type parameter `F` with an
   `Fn(&T) -> K` bound.
2. `map.entry(k).or_default().push(item)` creates an empty `Vec` the first time a key is seen.

## Starter code

Write your code in [`src/practice/p0510_group_by_key.rs`](../src/practice/p0510_group_by_key.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0510
```

```rust
use std::collections::BTreeMap;

/// Moves every element into the group of its key, keeping the original order inside each group.
pub fn group_by<T, K, F>(items: Vec<T>, key: F) -> BTreeMap<K, Vec<T>>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    todo!("Write your solution here")
}

/// Counts how many elements fall into each key.
pub fn count_by<T, K, F>(items: &[T], key: F) -> BTreeMap<K, usize>
where
    K: Ord,
    F: Fn(&T) -> K,
{
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

    #[test]
    fn groups_words_by_first_letter() {
        let groups = group_by(vec!["apple", "bob", "avocado", "cat", "blue"], |w| {
            w.chars().next().unwrap()
        });
        let expected = BTreeMap::from([
            ('a', vec!["apple", "avocado"]),
            ('b', vec!["bob", "blue"]),
            ('c', vec!["cat"]),
        ]);
        assert_eq!(groups, expected);
    }

    #[test]
    fn groups_numbers_by_remainder() {
        let groups = group_by(vec![1, 2, 3, 4, 5, 6], |n| n % 3);
        assert_eq!(
            groups,
            BTreeMap::from([(0, vec![3, 6]), (1, vec![1, 4]), (2, vec![2, 5])])
        );
    }

    #[test]
    fn counts_by_length() {
        assert_eq!(
            count_by(&["hi", "hello", "yo"], |w| w.len()),
            BTreeMap::from([(2, 2), (5, 1)])
        );
        assert!(count_by(&[] as &[i32], |x| *x).is_empty());
    }

    #[test]
    fn moves_non_clone_values() {
        #[derive(Debug, PartialEq)]
        struct Ticket {
            id: u32,
            team: &'static str,
        }
        let tickets = vec![
            Ticket { id: 1, team: "web" },
            Ticket { id: 2, team: "db" },
            Ticket { id: 3, team: "web" },
        ];
        let groups = group_by(tickets, |t| t.team);
        let web_ids: Vec<u32> = groups["web"].iter().map(|t| t.id).collect();
        assert_eq!(web_ids, vec![1, 3]);
        assert_eq!(groups["db"], vec![Ticket { id: 2, team: "db" }]);
    }

    #[test]
    fn empty_and_large_inputs() {
        assert!(group_by(Vec::<u8>::new(), |x| *x).is_empty());
        let data: Vec<u32> = (0..100_000).collect();
        let counts = count_by(&data, |x| x % 10);
        assert_eq!(counts.len(), 10);
        assert!(counts.values().all(|&c| c == 10_000));
        let groups = group_by(data, |x| x / 50_000);
        assert_eq!(groups[&1][0], 50_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** iterate once, compute each element's key with the closure, and push the element into the
vector stored under that key. For counting, increment a counter instead.

**Rust concepts:**
- `F: Fn(&T) -> K` is a trait bound like any other: every closure implements one of the `Fn*`
  traits, and generic code is monomorphized for the concrete closure type (no boxing).
- `group_by` takes `Vec<T>` by value so it can **move** the elements into the groups without
  `Clone`. `count_by` only needs to look, so it borrows `&[T]`.
- `BTreeMap::entry(..).or_default()` needs `V: Default`, which `Vec<T>` and `usize` both satisfy.

```rust
use std::collections::BTreeMap;

/// Moves every element into the group of its key, keeping the original order inside each group.
pub fn group_by<T, K, F>(items: Vec<T>, key: F) -> BTreeMap<K, Vec<T>>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut groups: BTreeMap<K, Vec<T>> = BTreeMap::new();
    for item in items {
        groups.entry(key(&item)).or_default().push(item);
    }
    groups
}

/// Counts how many elements fall into each key.
pub fn count_by<T, K, F>(items: &[T], key: F) -> BTreeMap<K, usize>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut counts = BTreeMap::new();
    for item in items {
        *counts.entry(key(item)).or_default() += 1;
    }
    counts
}
```

Full tested solution: [`src/solutions/p0510_group_by_key.rs`](../src/solutions/p0510_group_by_key.rs)

</details>
