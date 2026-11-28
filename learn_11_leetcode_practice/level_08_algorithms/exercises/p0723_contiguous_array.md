# Problem 0723: Contiguous Array

- **Difficulty:** Medium
- **Topics:** prefix balance, first occurrence in a `HashMap`, `entry().or_insert`
- **Source:** [LeetCode 525. Contiguous Array](https://leetcode.com/problems/contiguous-array/)

## Problem

Given a binary vector `nums` (values `0` and `1`), return the length of the longest contiguous
subarray that contains the **same number** of `0`s and `1`s. Return `0` if there is none.

Implement `Solution::find_max_length(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [0, 1]
Output: 2

Input:  nums = [0, 1, 0]
Output: 2             ([0, 1] or [1, 0])

Input:  nums = [0, 1, 1, 1, 1, 1, 0, 0, 0]
Output: 6             ([1, 1, 1, 0, 0, 0])
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `nums[i]` is `0` or `1`.

## Hints

1. Count a `1` as `+1` and a `0` as `-1`. A subarray is balanced exactly when its total is `0`.
2. A subarray's total is the difference of two prefix totals. When are two prefix totals equal?
3. For the longest subarray, remember the **first** place each prefix total appears.

## Starter code

Write your code in [`src/practice/p0723_contiguous_array.rs`](../src/practice/p0723_contiguous_array.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0723
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
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
    fn example_pair() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }

    #[test]
    fn example_long_block() {
        assert_eq!(
            Solution::find_max_length(vec![0, 1, 1, 1, 1, 1, 0, 0, 0]),
            6
        );
    }

    #[test]
    fn no_balanced_subarray() {
        assert_eq!(Solution::find_max_length(vec![1, 1, 1]), 0);
    }

    #[test]
    fn balanced_block_in_the_middle() {
        assert_eq!(Solution::find_max_length(vec![0, 0, 1, 0, 0, 0, 1, 1]), 6);
    }

    #[test]
    fn long_alternating_input() {
        let nums: Vec<i32> = (0..100_000).map(|i| i % 2).collect();
        assert_eq!(Solution::find_max_length(nums), 100_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** prefix balance + first occurrence: let `balance(p)` be the `+1/-1` total of the
first `p` elements. The subarray `nums[a..b]` is balanced exactly when
`balance(b) == balance(a)`. So for every prefix length `b`, the longest balanced subarray ending
there starts at the **smallest** `a` with the same balance. Store the first prefix length of
each balance (starting with balance `0` at length `0`, the empty prefix) and compare.

**Complexity:** `O(n)` time, `O(n)` space for the map.

**Rust concepts:**
- `*first.entry(balance).or_insert(p)` returns the stored first length, or inserts `p` and
  returns it (giving a length of `0`): one lookup does both jobs.
- `HashMap::from([(0, 0)])` builds a map from an array of pairs.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut first: HashMap<i32, usize> = HashMap::from([(0, 0)]);
        let (mut balance, mut best) = (0, 0);
        for (i, &x) in nums.iter().enumerate() {
            balance += if x == 1 { 1 } else { -1 };
            let p = i + 1; // prefix length
            let start = *first.entry(balance).or_insert(p);
            best = best.max(p - start);
        }
        best as i32
    }
}
```

Full tested solution: [`src/solutions/p0723_contiguous_array.rs`](../src/solutions/p0723_contiguous_array.rs)

</details>
