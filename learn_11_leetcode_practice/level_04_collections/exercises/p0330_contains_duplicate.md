# Problem 0330: Contains Duplicate

- **Difficulty:** Easy
- **Topics:** `HashSet::with_capacity`, `insert` returns `bool`, `Iterator::any`
- **Source:** [LeetCode 217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)

## Problem

Given a vector of integers `nums`, return `true` if some value appears **at least twice**, and
`false` if all values are distinct.

Implement `Solution::contains_duplicate(nums: Vec<i32>) -> bool`.

## Examples

```text
Input:  nums = [1, 2, 3, 1]
Output: true

Input:  nums = [1, 2, 3, 4]
Output: false

Input:  nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2]
Output: true
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `-10^9 <= nums[i] <= 10^9`

## Hints

1. Remember the values seen so far in a `HashSet`.
2. `insert` returns `false` if the value was already present: that is a duplicate, so you can
   stop immediately.

## Starter code

Write your code in [`src/practice/p0330_contains_duplicate.rs`](../src/practice/p0330_contains_duplicate.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0330
```

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
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
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_three() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }

    #[test]
    fn single_element_and_extremes() {
        assert!(!Solution::contains_duplicate(vec![0]));
        assert!(!Solution::contains_duplicate(vec![
            -1_000_000_000,
            1_000_000_000
        ]));
        assert!(Solution::contains_duplicate(vec![
            -1_000_000_000,
            -1_000_000_000
        ]));
    }

    #[test]
    fn large_inputs() {
        let distinct: Vec<i32> = (0..100_000).map(|i| i * 10_000 - 500_000_000).collect();
        assert!(!Solution::contains_duplicate(distinct.clone()));
        let mut with_dup = distinct;
        with_dup[99_999] = with_dup[0];
        assert!(Solution::contains_duplicate(with_dup));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** scan the numbers and insert each into a set. The first failed insert proves a
duplicate; if every insert succeeds, all values are distinct.

**Complexity:** `O(n)` average time, `O(n)` space.

**Rust concepts:**
- `HashSet::with_capacity(n)` reserves room for `n` elements, avoiding re-hashing while it grows.
- `any(|x| !seen.insert(x))` short-circuits on the first duplicate. The closure mutates `seen`,
  which is allowed because `any` takes an `FnMut`.
- Alternatives: sort and check `windows(2)` (`O(n log n)`, no extra memory), or compare
  `nums.len()` with the size of `nums.iter().collect::<HashSet<_>>()` (no early exit).

```rust
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());
        nums.into_iter().any(|x| !seen.insert(x))
    }
}
```

Full tested solution: [`src/solutions/p0330_contains_duplicate.rs`](../src/solutions/p0330_contains_duplicate.rs)

</details>
