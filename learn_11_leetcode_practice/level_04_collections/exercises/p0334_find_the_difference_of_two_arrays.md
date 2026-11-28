# Problem 0334: Find the Difference of Two Arrays

- **Difficulty:** Easy
- **Topics:** `HashSet::difference`, `vec![a, b]` of results, unspecified order
- **Source:** [LeetCode 2215. Find the Difference of Two Arrays](https://leetcode.com/problems/find-the-difference-of-two-arrays/)

## Problem

Given two vectors `nums1` and `nums2`, return a vector `answer` of two vectors:

- `answer[0]` holds the **distinct** values of `nums1` that do not appear in `nums2`;
- `answer[1]` holds the **distinct** values of `nums2` that do not appear in `nums1`.

The values inside `answer[0]` and inside `answer[1]` may be in any order.

Implement `Solution::find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  nums1 = [1, 2, 3], nums2 = [2, 4, 6]
Output: [[1, 3], [4, 6]]

Input:  nums1 = [1, 2, 3, 3], nums2 = [1, 1, 2, 2]
Output: [[3], []]
```

## Constraints

- `1 <= nums1.len(), nums2.len() <= 1000`
- `-1000 <= nums1[i], nums2[i] <= 1000`

## Hints

1. `a.difference(&b)` iterates over the elements of `a` that are not in `b`.
2. Note that `difference` is not symmetric: you need it in both directions.

## Starter code

Write your code in [`src/practice/p0334_find_the_difference_of_two_arrays.rs`](../src/practice/p0334_find_the_difference_of_two_arrays.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0334
```

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
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

    fn normalized(mut answer: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for part in answer.iter_mut() {
            part.sort_unstable();
        }
        answer
    }

    #[test]
    fn example_one() {
        assert_eq!(
            normalized(Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6])),
            vec![vec![1, 3], vec![4, 6]]
        );
    }

    #[test]
    fn example_two_duplicates() {
        assert_eq!(
            normalized(Solution::find_difference(
                vec![1, 2, 3, 3],
                vec![1, 1, 2, 2]
            )),
            vec![vec![3], vec![]]
        );
    }

    #[test]
    fn identical_inputs() {
        assert_eq!(
            Solution::find_difference(vec![5, -5], vec![-5, 5]),
            vec![Vec::<i32>::new(), Vec::new()]
        );
    }

    #[test]
    fn large_inputs() {
        let nums1: Vec<i32> = (-1000..0).collect();
        let nums2: Vec<i32> = (-500..500).collect();
        let answer = normalized(Solution::find_difference(nums1, nums2));
        assert_eq!(answer[0], (-1000..-500).collect::<Vec<i32>>());
        assert_eq!(answer[1], (0..500).collect::<Vec<i32>>());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** build one set per input (which also removes duplicates) and compute the set
difference in each direction.

**Complexity:** `O(n + m)` average time and space.

**Rust concepts:**
- `difference` borrows both sets and yields `&i32`; `copied()` converts to owned values.
- `a.difference(&b)` and `b.difference(&a)` are different sets, unlike `intersection`,
  `union` and `symmetric_difference`, which are symmetric.
- The order inside each inner vector is unspecified, so the tests sort them.

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let a: HashSet<i32> = nums1.into_iter().collect();
        let b: HashSet<i32> = nums2.into_iter().collect();
        vec![
            a.difference(&b).copied().collect(),
            b.difference(&a).copied().collect(),
        ]
    }
}
```

Full tested solution: [`src/solutions/p0334_find_the_difference_of_two_arrays.rs`](../src/solutions/p0334_find_the_difference_of_two_arrays.rs)

</details>
