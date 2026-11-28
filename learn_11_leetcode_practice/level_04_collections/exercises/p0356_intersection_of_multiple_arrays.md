# Problem 0356: Intersection of Multiple Arrays

- **Difficulty:** Easy
- **Topics:** `BTreeSet::retain`, `split_first`, sorted output from a set
- **Source:** [LeetCode 2248. Intersection of Multiple Arrays](https://leetcode.com/problems/intersection-of-multiple-arrays/)

## Problem

You are given a list `nums` of non-empty vectors; each inner vector contains **distinct**
positive integers. Return, in **ascending order**, the integers that appear in **every** inner
vector.

Implement `Solution::intersection(nums: Vec<Vec<i32>>) -> Vec<i32>`.

## Examples

```text
Input:  nums = [[3, 1, 2, 4, 5], [1, 2, 3, 4], [3, 4, 5, 6]]
Output: [3, 4]

Input:  nums = [[1, 2, 3], [4, 5, 6]]
Output: []
```

## Constraints

- `1 <= nums.len() <= 1000`
- `1 <= sum of nums[i].len() <= 1000`
- `1 <= nums[i][j] <= 1000`, values within one `nums[i]` are distinct

## Hints

1. Start with all values of the first vector in a `BTreeSet`.
2. For every other vector, keep only the values it contains: `set.retain(|x| ...)`.
3. A `BTreeSet` already iterates in ascending order.

## Starter code

Write your code in [`src/practice/p0356_intersection_of_multiple_arrays.rs`](../src/practice/p0356_intersection_of_multiple_arrays.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0356
```

```rust
use std::collections::{BTreeSet, HashSet};

pub struct Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
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
            Solution::intersection(vec![
                vec![3, 1, 2, 4, 5],
                vec![1, 2, 3, 4],
                vec![3, 4, 5, 6]
            ]),
            vec![3, 4]
        );
    }

    #[test]
    fn example_two_empty() {
        assert_eq!(
            Solution::intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn single_row_is_sorted() {
        assert_eq!(Solution::intersection(vec![vec![9, 2, 7]]), vec![2, 7, 9]);
    }

    #[test]
    fn many_rows() {
        // 100 rows of 10 distinct values (11..=810 are spread over the rows); every row also
        // contains 1000 and 7.
        let nums: Vec<Vec<i32>> = (0..100)
            .map(|r| {
                let mut row: Vec<i32> = (1..=8).map(|k| 10 + r * 8 + k).collect();
                row.push(1000);
                row.push(7);
                row
            })
            .collect();
        assert_eq!(Solution::intersection(nums), vec![7, 1000]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the answer is a subset of the first vector. Put that vector into a `BTreeSet`, then
shrink the set with `retain` for each remaining vector (testing membership through a `HashSet` of
that vector). Whatever survives is in every vector, and the `BTreeSet` hands it back sorted.

**Complexity:** `O(L log L)` time where `L` is the total number of elements, `O(L)` space.

**Rust concepts:**
- `split_first()` returns `Option<(&T, &[T])>`: the first element and the rest, without
  indexing.
- `BTreeSet::retain` (like `Vec::retain`) removes, in place, the elements for which the closure
  returns `false`.
- Building a `HashSet` per row makes each membership test `O(1)`; `row.contains(x)` on the `Vec`
  would also work for these small sizes.

```rust
use std::collections::{BTreeSet, HashSet};

pub struct Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let Some((first, rest)) = nums.split_first() else {
            return Vec::new();
        };
        let mut common: BTreeSet<i32> = first.iter().copied().collect();
        for row in rest {
            let present: HashSet<i32> = row.iter().copied().collect();
            common.retain(|x| present.contains(x));
        }
        common.into_iter().collect()
    }
}
```

Full tested solution: [`src/solutions/p0356_intersection_of_multiple_arrays.rs`](../src/solutions/p0356_intersection_of_multiple_arrays.rs)

</details>
