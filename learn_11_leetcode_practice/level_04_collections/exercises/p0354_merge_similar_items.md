# Problem 0354: Merge Similar Items

- **Difficulty:** Easy
- **Topics:** accumulating into a `BTreeMap`, `chain`, ordered output without sorting
- **Source:** [LeetCode 2363. Merge Similar Items](https://leetcode.com/problems/merge-similar-items/)

## Problem

You are given two lists of items, `items1` and `items2`. Each item is a pair `[value, weight]`.
Within one list the values are distinct, but the same value may appear in both lists.

Return a list of pairs `[value, total_weight]`, one per distinct value appearing in either
list, where `total_weight` is the sum of that value's weights in both lists. The result must be
sorted by `value` in ascending order.

Implement `Solution::merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  items1 = [[1, 1], [4, 5], [3, 8]], items2 = [[3, 1], [1, 5]]
Output: [[1, 6], [3, 9], [4, 5]]

Input:  items1 = [[1, 1], [3, 2], [2, 3]], items2 = [[2, 1], [3, 2], [1, 3]]
Output: [[1, 4], [2, 4], [3, 4]]

Input:  items1 = [[1, 3], [2, 2]], items2 = [[7, 1], [2, 2], [1, 4]]
Output: [[1, 7], [2, 4], [7, 1]]
```

## Constraints

- `1 <= items1.len(), items2.len() <= 1000`
- `items[i].len() == 2`, `1 <= value, weight <= 1000`
- Values are unique within `items1` and within `items2`.

## Hints

1. A `BTreeMap<i32, i32>` from value to total weight both merges and sorts.
2. `items1.iter().chain(items2.iter())` walks both lists as one.

## Starter code

Write your code in [`src/practice/p0354_merge_similar_items.rs`](../src/practice/p0354_merge_similar_items.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0354
```

```rust
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
    fn example_one() {
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 1], vec![4, 5], vec![3, 8]],
                vec![vec![3, 1], vec![1, 5]]
            ),
            vec![vec![1, 6], vec![3, 9], vec![4, 5]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 1], vec![3, 2], vec![2, 3]],
                vec![vec![2, 1], vec![3, 2], vec![1, 3]]
            ),
            vec![vec![1, 4], vec![2, 4], vec![3, 4]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::merge_similar_items(
                vec![vec![1, 3], vec![2, 2]],
                vec![vec![7, 1], vec![2, 2], vec![1, 4]]
            ),
            vec![vec![1, 7], vec![2, 4], vec![7, 1]]
        );
    }

    #[test]
    fn disjoint_large_lists() {
        let items1: Vec<Vec<i32>> = (1..=1000)
            .rev()
            .filter(|v| v % 2 == 0)
            .map(|v| vec![v, 1000])
            .collect();
        let items2: Vec<Vec<i32>> = (1..=1000)
            .filter(|v| v % 2 == 1)
            .map(|v| vec![v, 1])
            .collect();
        let merged = Solution::merge_similar_items(items1, items2);
        assert_eq!(merged.len(), 1000);
        assert_eq!(merged[0], vec![1, 1]);
        assert_eq!(merged[999], vec![1000, 1000]);
        assert!(merged.windows(2).all(|w| w[0][0] < w[1][0]));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** add every `(value, weight)` from both lists into a `BTreeMap`, summing weights per
value. Iterating the map then yields the values in ascending order, so the output needs no
separate sort.

**Complexity:** `O((n + m) log(n + m))` time, `O(n + m)` space.

**Rust concepts:**
- `entry(value).or_insert(0)` works exactly as with a `HashMap`.
- `into_iter()` on a `BTreeMap` yields `(key, value)` pairs in key order.
- Each item is a `Vec<i32>` of length 2 (LeetCode's encoding of a pair); `item[0]` and
  `item[1]` are safe by the constraints.

```rust
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut totals: BTreeMap<i32, i32> = BTreeMap::new();
        for item in items1.iter().chain(items2.iter()) {
            // Every item has exactly two elements: [value, weight].
            *totals.entry(item[0]).or_insert(0) += item[1];
        }
        totals
            .into_iter()
            .map(|(value, weight)| vec![value, weight])
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0354_merge_similar_items.rs`](../src/solutions/p0354_merge_similar_items.rs)

</details>
