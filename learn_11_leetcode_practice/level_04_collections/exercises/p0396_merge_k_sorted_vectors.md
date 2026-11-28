# Problem 0396: Merge K Sorted Vectors

- **Difficulty:** Medium
- **Topics:** k-way merge with `BinaryHeap<Reverse<(i32, usize, usize)>>`, `get` for safe look-ahead, laziness
- **Source:** Original · [`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) · [`Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html)

## Problem

You receive `k` vectors, each sorted in non-decreasing order. Implement:

1. `merge_sorted(lists: &[Vec<i32>]) -> Vec<i32>`: all elements of all lists in one
   non-decreasing vector (duplicates kept);
2. `kth_smallest(lists: &[Vec<i32>], k: usize) -> Option<i32>`: the `k`-th smallest element
   overall (`k = 1` is the smallest), counting duplicates, or `None` if there are fewer than `k`
   elements or `k == 0`. It should stop as soon as it knows the answer instead of merging
   everything.

Do not simply concatenate and sort: use the fact that every list is already sorted.

## Examples

```text
lists = [[1, 4, 5], [1, 3, 4], [2, 6]]
merge_sorted(lists)    => [1, 1, 2, 3, 4, 4, 5, 6]
kth_smallest(lists, 4) => Some(3)
kth_smallest(lists, 9) => None

merge_sorted([[], [7]]) => [7]
```

## Constraints

- `0 <= k <= 10^4` lists, at most `10^5` elements in total.

## Hints

1. The next element of the merged output is the smallest among the **current fronts** of the
   lists.
2. Keep one entry per list in a min-heap: `Reverse((value, list_index, position))`.
3. After popping the entry of list `i` at position `p`, push the entry for position `p + 1` of the
   same list, if it exists.

## Starter code

Write your code in [`src/practice/p0396_merge_k_sorted_vectors.rs`](../src/practice/p0396_merge_k_sorted_vectors.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0396
```

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Merges sorted lists into one sorted vector.
pub fn merge_sorted(lists: &[Vec<i32>]) -> Vec<i32> {
    todo!("Write your solution here")
}

/// The `k`-th smallest element across all lists (1-based).
pub fn kth_smallest(lists: &[Vec<i32>], k: usize) -> Option<i32> {
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

    fn example() -> Vec<Vec<i32>> {
        vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]]
    }

    #[test]
    fn merge_example() {
        assert_eq!(merge_sorted(&example()), vec![1, 1, 2, 3, 4, 4, 5, 6]);
        assert_eq!(merge_sorted(&[vec![], vec![7]]), vec![7]);
        assert_eq!(merge_sorted(&[]), Vec::<i32>::new());
    }

    #[test]
    fn kth_example() {
        assert_eq!(kth_smallest(&example(), 4), Some(3));
        assert_eq!(kth_smallest(&example(), 1), Some(1));
        assert_eq!(kth_smallest(&example(), 8), Some(6));
        assert_eq!(kth_smallest(&example(), 9), None);
        assert_eq!(kth_smallest(&example(), 0), None);
    }

    #[test]
    fn negative_values_and_duplicates() {
        let lists = vec![vec![-5, -5, 0], vec![-5], vec![-10, 10]];
        assert_eq!(merge_sorted(&lists), vec![-10, -5, -5, -5, 0, 10]);
        assert_eq!(kth_smallest(&lists, 4), Some(-5));
    }

    #[test]
    fn many_lists() {
        // List r holds r, r + 1000, r + 2000, ...: merging gives 0..100_000 in order.
        let lists: Vec<Vec<i32>> = (0..1000)
            .map(|r| (0..100).map(|j| r + 1000 * j).collect())
            .collect();
        assert_eq!(merge_sorted(&lists), (0..100_000).collect::<Vec<i32>>());
        assert_eq!(kth_smallest(&lists, 12_345), Some(12_344));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the classic k-way merge. Seed a min-heap with the first element of every non-empty
list. Each pop yields the next smallest element overall; the popped list then contributes its
next element. `kth_smallest` runs the same loop but stops after `k` pops.

**Complexity:** `O(N log k)` time for `N` elements in `k` lists (`O(k + K log k)` for the `K`-th
smallest), `O(k)` heap space.

**Rust concepts:**
- The heap stores small `Copy` tuples, not the values' lists; `(value, list, position)` compares
  by value first, and the indices make every entry unique.
- `lists[i].get(p + 1)` returns `None` at the end of a list, so there is no bounds check to
  forget.
- A private helper `seed` builds the initial heap for both functions; `filter_map` with
  `first()` skips empty lists.
- Sorting the concatenation would be `O(N log N)`; the merge exploits the existing order.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// A min-heap holding the first element of every non-empty list.
fn seed(lists: &[Vec<i32>]) -> BinaryHeap<Reverse<(i32, usize, usize)>> {
    lists
        .iter()
        .enumerate()
        .filter_map(|(i, list)| list.first().map(|&v| Reverse((v, i, 0))))
        .collect()
}

/// Merges sorted lists into one sorted vector.
pub fn merge_sorted(lists: &[Vec<i32>]) -> Vec<i32> {
    let mut heap = seed(lists);
    let mut merged = Vec::with_capacity(lists.iter().map(Vec::len).sum());
    while let Some(Reverse((value, i, p))) = heap.pop() {
        merged.push(value);
        if let Some(&next) = lists[i].get(p + 1) {
            heap.push(Reverse((next, i, p + 1)));
        }
    }
    merged
}

/// The `k`-th smallest element across all lists (1-based).
pub fn kth_smallest(lists: &[Vec<i32>], k: usize) -> Option<i32> {
    let mut heap = seed(lists);
    let mut popped = 0;
    while let Some(Reverse((value, i, p))) = heap.pop() {
        popped += 1;
        if popped == k {
            return Some(value);
        }
        if let Some(&next) = lists[i].get(p + 1) {
            heap.push(Reverse((next, i, p + 1)));
        }
    }
    None
}
```

Full tested solution: [`src/solutions/p0396_merge_k_sorted_vectors.rs`](../src/solutions/p0396_merge_k_sorted_vectors.rs)

</details>
