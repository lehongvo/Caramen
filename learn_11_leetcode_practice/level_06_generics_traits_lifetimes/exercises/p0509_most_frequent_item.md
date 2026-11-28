# Problem 0509: Most Frequent Item

- **Difficulty:** Medium
- **Topics:** multiple trait bounds, `where` clauses, `T: Hash + Eq + Ord + Clone`, `HashMap<&T, usize>`
- **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax)

## Problem

Write two generic counting functions.

- `most_frequent<T>(items: &[T]) -> Option<(T, usize)>` returns the element that appears most
  often together with its count. When several elements share the highest count, return the
  **smallest** of them (by `Ord`). Return `None` for an empty slice.
  Bounds: `T: Hash + Eq + Ord + Clone`.
- `top_n<T>(items: &[T], n: usize) -> Vec<(T, usize)>` returns the `n` most frequent elements
  (fewer if there are fewer distinct elements), sorted by count **descending**, then by value
  **ascending**. Same bounds.

Write the bounds in a `where` clause.

## Examples

```text
most_frequent(&[3, 1, 3, 2, 1, 3])          => Some((3, 3))
most_frequent(&["b", "a", "b", "a"])        => Some(("a", 2))    tie: "a" < "b"
most_frequent::<u8>(&[])                    => None
top_n(&['x', 'y', 'x', 'z', 'y', 'x'], 2)   => [('x', 3), ('y', 2)]
```

## Constraints

- `0 <= items.len() <= 10^5`
- `0 <= n <= 10^5`

## Hints

1. Count with a `HashMap<&T, usize>`: the keys borrow from `items`, so nothing is cloned while
   counting. `Hash + Eq` are exactly what a `HashMap` key needs.
2. Clone a value only when you put it in the result.
3. Sorting by `(Reverse(count), value)` gives "count descending, then value ascending".

## Starter code

Write your code in [`src/practice/p0509_most_frequent_item.rs`](../src/practice/p0509_most_frequent_item.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0509
```

```rust
use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::Hash;

/// The most frequent element and its count; ties go to the smallest element.
pub fn most_frequent<T>(items: &[T]) -> Option<(T, usize)>
where
    T: Hash + Eq + Ord + Clone,
{
    todo!("Write your solution here")
}

/// The `n` most frequent elements, by count descending then value ascending.
pub fn top_n<T>(items: &[T], n: usize) -> Vec<(T, usize)>
where
    T: Hash + Eq + Ord + Clone,
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
    fn example_most_frequent() {
        assert_eq!(most_frequent(&[3, 1, 3, 2, 1, 3]), Some((3, 3)));
        assert_eq!(most_frequent(&["b", "a", "b", "a"]), Some(("a", 2)));
        assert_eq!(most_frequent::<u8>(&[]), None);
    }

    #[test]
    fn example_top_n() {
        assert_eq!(
            top_n(&['x', 'y', 'x', 'z', 'y', 'x'], 2),
            vec![('x', 3), ('y', 2)]
        );
        assert_eq!(
            top_n(&['x', 'y', 'x', 'z', 'y', 'x'], 10),
            vec![('x', 3), ('y', 2), ('z', 1)]
        );
        assert_eq!(top_n(&[1, 2, 3], 0), Vec::<(i32, usize)>::new());
    }

    #[test]
    fn works_with_owned_strings() {
        let words: Vec<String> = "to be or not to be".split(' ').map(String::from).collect();
        assert_eq!(most_frequent(&words), Some(("be".to_string(), 2)));
        assert_eq!(
            top_n(&words, 3),
            vec![
                ("be".to_string(), 2),
                ("to".to_string(), 2),
                ("not".to_string(), 1)
            ]
        );
    }

    #[test]
    fn large_input() {
        let items: Vec<u32> = (0..100_000).map(|i| i % 97).collect();
        // 100_000 = 97 * 1030 + 90, so values 0..90 appear 1031 times and 90..97 appear 1030 times.
        assert_eq!(most_frequent(&items), Some((0, 1031)));
        let top = top_n(&items, 91);
        assert_eq!(top[89], (89, 1031));
        assert_eq!(top[90], (90, 1030));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count occurrences with a map whose keys are references into the slice. For
`most_frequent`, scan the counts keeping the best `(count, value)`; for `top_n`, collect the
counts into a vector, sort it by `(Reverse(count), value)` and keep the first `n`.

**Rust concepts:**
- Each bound has a reason: `Hash + Eq` for the map keys, `Ord` for the tie-break, `Clone` to
  return owned values. A `where` clause keeps this list readable.
- `&T` implements `Hash`, `Eq` and `Ord` whenever `T` does, so a `HashMap<&T, usize>` works
  without cloning keys.
- `std::cmp::Reverse` flips the order of whatever it wraps, which turns an ascending sort key into
  a descending one.

```rust
use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::Hash;

fn count<T: Hash + Eq>(items: &[T]) -> HashMap<&T, usize> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

/// The most frequent element and its count; ties go to the smallest element.
pub fn most_frequent<T>(items: &[T]) -> Option<(T, usize)>
where
    T: Hash + Eq + Ord + Clone,
{
    count(items)
        .into_iter()
        .min_by_key(|&(value, n)| (Reverse(n), value))
        .map(|(value, n)| (value.clone(), n))
}

/// The `n` most frequent elements, by count descending then value ascending.
pub fn top_n<T>(items: &[T], n: usize) -> Vec<(T, usize)>
where
    T: Hash + Eq + Ord + Clone,
{
    let mut counted: Vec<(&T, usize)> = count(items).into_iter().collect();
    counted.sort_by_key(|&(value, c)| (Reverse(c), value));
    counted
        .into_iter()
        .take(n)
        .map(|(value, c)| (value.clone(), c))
        .collect()
}
```

Full tested solution: [`src/solutions/p0509_most_frequent_item.rs`](../src/solutions/p0509_most_frequent_item.rs)

</details>
