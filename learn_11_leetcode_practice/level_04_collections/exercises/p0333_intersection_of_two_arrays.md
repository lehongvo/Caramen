# Problem 0333: Intersection of Two Arrays

- **Difficulty:** Easy
- **Topics:** `HashSet::intersection`, `copied`, results in unspecified order
- **Source:** [LeetCode 349. Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/)

## Problem

Given two vectors `nums1` and `nums2`, return the values that appear in **both** of them. Each
value appears only **once** in the result, even if it is repeated in the inputs. The result may
be in **any order**.

Implement `Solution::intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  nums1 = [1, 2, 2, 1], nums2 = [2, 2]
Output: [2]

Input:  nums1 = [4, 9, 5], nums2 = [9, 4, 9, 8, 4]
Output: [9, 4]          ([4, 9] is also accepted)
```

## Constraints

- `1 <= nums1.len(), nums2.len() <= 1000`
- `0 <= nums1[i], nums2[i] <= 1000`

## Hints

1. Turn both vectors into `HashSet`s; that removes the repetitions.
2. `a.intersection(&b)` iterates over the values present in both sets.

## Starter code

Write your code in [`src/practice/p0333_intersection_of_two_arrays.rs`](../src/practice/p0333_intersection_of_two_arrays.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0333
```

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
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

    fn sorted(mut v: Vec<i32>) -> Vec<i32> {
        v.sort_unstable();
        v
    }

    #[test]
    fn example_one() {
        assert_eq!(
            sorted(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2])),
            vec![2]
        );
    }

    #[test]
    fn example_two_any_order() {
        assert_eq!(
            sorted(Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4])),
            vec![4, 9]
        );
    }

    #[test]
    fn nothing_in_common() {
        assert_eq!(
            Solution::intersection(vec![1, 3, 5], vec![0, 2, 4]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn large_inputs() {
        let nums1: Vec<i32> = (0..1000).map(|i| i % 500).collect();
        let nums2: Vec<i32> = (250..1000).collect();
        assert_eq!(
            sorted(Solution::intersection(nums1, nums2)),
            (250..500).collect::<Vec<i32>>()
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** convert both inputs to sets and take their intersection. The sets remove duplicates,
and the intersection keeps exactly the common values.

**Complexity:** `O(n + m)` average time, `O(n + m)` space.

**Rust concepts:**
- `into_iter().collect::<HashSet<i32>>()` consumes a vector into a set.
- `intersection` returns a lazy iterator of **references** (`&i32`) into the first set;
  `copied()` turns them into `i32` values before collecting into a `Vec`.
- The order of the result depends on the hash set's internal layout and is unspecified, so the
  tests sort the output before comparing it.

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let a: HashSet<i32> = nums1.into_iter().collect();
        let b: HashSet<i32> = nums2.into_iter().collect();
        a.intersection(&b).copied().collect()
    }
}
```

Full tested solution: [`src/solutions/p0333_intersection_of_two_arrays.rs`](../src/solutions/p0333_intersection_of_two_arrays.rs)

</details>
