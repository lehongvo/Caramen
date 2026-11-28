# Problem 0318: Relative Sort Array

- **Difficulty:** Easy
- **Topics:** rank table in a `Vec<Option<usize>>`, `sort_unstable_by_key` with a tuple key
- **Source:** [LeetCode 1122. Relative Sort Array](https://leetcode.com/problems/relative-sort-array/)

## Problem

You are given two vectors `arr1` and `arr2`. The values of `arr2` are distinct, and each of
them also appears in `arr1` (possibly several times). Sort `arr1` so that:

- values that appear in `arr2` come first, in the **same relative order as in `arr2`**
  (all copies of `arr2[0]`, then all copies of `arr2[1]`, ...);
- values that do not appear in `arr2` come after them, in **ascending** order.

Return the sorted vector.

Implement `Solution::relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  arr1 = [2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], arr2 = [2, 1, 4, 3, 9, 6]
Output: [2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]

Input:  arr1 = [28, 6, 22, 8, 44, 17], arr2 = [22, 28, 8, 6]
Output: [22, 28, 8, 6, 17, 44]
```

## Constraints

- `1 <= arr1.len(), arr2.len() <= 1000`
- `0 <= arr1[i], arr2[i] <= 1000`
- The values of `arr2` are distinct and all appear in `arr1`.

## Hints

1. Build a table `rank[value] = position of value in arr2` (or "no position").
2. Sort by the key `(rank, value)`, where values without a rank get a rank larger than every
   real one, e.g. `arr2.len()`.

## Starter code

Write your code in [`src/practice/p0318_relative_sort_array.rs`](../src/practice/p0318_relative_sort_array.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0318
```

```rust
pub struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
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
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
            vec![22, 28, 8, 6, 17, 44]
        );
    }

    #[test]
    fn every_value_ranked() {
        assert_eq!(
            Solution::relative_sort_array(vec![3, 1, 2, 3, 1], vec![3, 2, 1]),
            vec![3, 3, 2, 1, 1]
        );
    }

    #[test]
    fn extreme_values_unranked() {
        assert_eq!(
            Solution::relative_sort_array(vec![1000, 0, 5, 1000, 0], vec![5]),
            vec![5, 0, 0, 1000, 1000]
        );
    }

    #[test]
    fn large_input() {
        let arr1: Vec<i32> = (0..1000).rev().collect();
        let arr2: Vec<i32> = vec![500, 10];
        let result = Solution::relative_sort_array(arr1, arr2);
        assert_eq!(&result[..3], &[500, 10, 0]);
        assert_eq!(result[999], 999);
        assert!(result[2..].windows(2).all(|w| w[0] < w[1]));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** precompute the position of every value of `arr2` in a table indexed by value. Then
one sort with the key `(position or arr2.len(), value)` does everything: the first part groups
the `arr2` values in `arr2` order and pushes the others to the end; the second part sorts the
others ascending (for `arr2` values it never matters, since equal positions mean equal values).

**Complexity:** `O(n log n + m)` time, `O(1001)` extra space for the table.

**Rust concepts:**
- `vec![None; 1001]` creates a `Vec<Option<usize>>`; `Option<usize>` is `Clone`, so `vec!`
  can repeat it.
- `unwrap_or(arr2.len())` turns "no rank" into "after every ranked value".
- `enumerate()` gives `(index, &value)` pairs to fill the table.

```rust
pub struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut rank: Vec<Option<usize>> = vec![None; 1001];
        for (i, &v) in arr2.iter().enumerate() {
            rank[v as usize] = Some(i);
        }
        let unranked = arr2.len();
        arr1.sort_unstable_by_key(|&v| (rank[v as usize].unwrap_or(unranked), v));
        arr1
    }
}
```

Full tested solution: [`src/solutions/p0318_relative_sort_array.rs`](../src/solutions/p0318_relative_sort_array.rs)

</details>
