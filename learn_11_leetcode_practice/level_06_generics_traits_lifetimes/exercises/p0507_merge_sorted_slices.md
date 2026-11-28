# Problem 0507: Merge Two Sorted Slices

- **Difficulty:** Easy
- **Topics:** `T: Ord + Clone`, `Ord::cmp`, `std::cmp::Ordering`, `T: PartialOrd`
- **Source:** Original · [std::cmp::Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)

## Problem

Implement two generic functions:

- `is_non_decreasing<T: PartialOrd>(items: &[T]) -> bool` returns `true` when every element is
  `<=` the next one. Empty and one-element slices are sorted. Do not use `slice::is_sorted`.
- `merge_sorted<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T>` takes two slices that are each sorted
  in non-decreasing order and returns one sorted vector with all elements of both. The merge must
  be **stable**: when an element of `a` and an element of `b` compare equal, the one from `a` comes
  first. Do not concatenate and sort; walk both slices once.

## Examples

```text
is_non_decreasing(&[1, 2, 2, 5])           => true
is_non_decreasing(&["b", "a"])             => false
merge_sorted(&[1, 4, 9], &[2, 3, 10, 11])  => [1, 2, 3, 4, 9, 10, 11]
merge_sorted(&["ant", "cat"], &["bee"])    => ["ant", "bee", "cat"]
merge_sorted::<i32>(&[], &[])              => []
```

## Constraints

- `0 <= a.len(), b.len() <= 10^5`
- `a` and `b` are sorted in non-decreasing order.

## Hints

1. Keep one index per slice and compare `a[i].cmp(&b[j])`: it returns `Ordering::Less`,
   `Ordering::Equal` or `Ordering::Greater`.
2. For stability, take from `a` on `Equal`.
3. When one slice runs out, copy the rest of the other with `extend_from_slice`.

## Starter code

Write your code in [`src/practice/p0507_merge_sorted_slices.rs`](../src/practice/p0507_merge_sorted_slices.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0507
```

```rust
use std::cmp::Ordering;

/// `true` when every element is `<=` the next one.
pub fn is_non_decreasing<T: PartialOrd>(items: &[T]) -> bool {
    todo!("Write your solution here")
}

/// Stable merge of two sorted slices.
pub fn merge_sorted<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
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
    fn checks_sortedness() {
        assert!(is_non_decreasing(&[1, 2, 2, 5]));
        assert!(!is_non_decreasing(&["b", "a"]));
        assert!(is_non_decreasing::<u8>(&[]));
        assert!(is_non_decreasing(&[0.5, 0.5, 1.0]));
    }

    #[test]
    fn merges_numbers_and_strings() {
        assert_eq!(
            merge_sorted(&[1, 4, 9], &[2, 3, 10, 11]),
            vec![1, 2, 3, 4, 9, 10, 11]
        );
        assert_eq!(
            merge_sorted(&["ant", "cat"], &["bee"]),
            vec!["ant", "bee", "cat"]
        );
    }

    #[test]
    fn empty_inputs() {
        assert_eq!(merge_sorted::<i32>(&[], &[]), Vec::<i32>::new());
        assert_eq!(merge_sorted(&[1, 2], &[]), vec![1, 2]);
        assert_eq!(merge_sorted(&[], &[3]), vec![3]);
    }

    #[test]
    fn merge_is_stable() {
        // Compare only by the first field so equal keys can be told apart by the second one.
        #[derive(Clone, Debug)]
        struct Item(u8, char);
        impl PartialEq for Item {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl Eq for Item {}
        impl PartialOrd for Item {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Item {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        let a = [Item(1, 'a'), Item(2, 'a')];
        let b = [Item(1, 'b'), Item(2, 'b')];
        let merged = merge_sorted(&a, &b);
        let tags: Vec<char> = merged.iter().map(|item| item.1).collect();
        assert_eq!(tags, vec!['a', 'b', 'a', 'b']);
    }

    #[test]
    fn large_merge() {
        let evens: Vec<u32> = (0..100_000).map(|x| x * 2).collect();
        let odds: Vec<u32> = (0..100_000).map(|x| x * 2 + 1).collect();
        let merged = merge_sorted(&evens, &odds);
        assert_eq!(merged.len(), 200_000);
        assert!(is_non_decreasing(&merged));
        assert_eq!(merged[123_456], 123_456);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** classic two-pointer merge. While both slices have elements left, push the smaller head
(preferring `a` on ties); afterwards append whatever remains.

**Rust concepts:**
- `Ord::cmp` returns an `Ordering` enum, which reads well in a `match`. `Ord` is a total order, so
  every pair of values is comparable (unlike `f64`, which is only `PartialOrd`).
- `is_non_decreasing` only needs `PartialOrd` because it uses `<=`; using the weakest bound that
  works makes a generic function usable with more types (it also accepts `f64`).
- `extend_from_slice` needs `T: Clone`, which is already part of the bound.

```rust
use std::cmp::Ordering;

/// `true` when every element is `<=` the next one.
pub fn is_non_decreasing<T: PartialOrd>(items: &[T]) -> bool {
    items.windows(2).all(|w| w[0] <= w[1])
}

/// Stable merge of two sorted slices.
pub fn merge_sorted<T: Ord + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            Ordering::Less | Ordering::Equal => {
                out.push(a[i].clone());
                i += 1;
            }
            Ordering::Greater => {
                out.push(b[j].clone());
                j += 1;
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}
```

Full tested solution: [`src/solutions/p0507_merge_sorted_slices.rs`](../src/solutions/p0507_merge_sorted_slices.rs)

</details>
