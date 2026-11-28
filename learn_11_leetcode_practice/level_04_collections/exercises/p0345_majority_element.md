# Problem 0345: Majority Element

- **Difficulty:** Easy
- **Topics:** count map, `into_iter().max_by_key`, Boyer-Moore voting (follow-up)
- **Source:** [LeetCode 169. Majority Element](https://leetcode.com/problems/majority-element/)

## Problem

The **majority element** of a vector of length `n` is the value that appears **more than**
`n / 2` times (rounded down). Given `nums`, return its majority element; one always exists.

LeetCode's follow-up asks for `O(n)` time and `O(1)` extra space.

Implement `Solution::majority_element(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [3, 2, 3]
Output: 3

Input:  nums = [2, 2, 1, 1, 1, 2, 2]
Output: 2
```

## Constraints

- `1 <= nums.len() <= 5 * 10^4`
- `-10^9 <= nums[i] <= 10^9`
- A majority element exists.

## Hints

1. Count every value; the majority element has the largest count.
2. `max_by_key(|&(_, count)| count)` finds the entry with the largest count.

## Starter code

Write your code in [`src/practice/p0345_majority_element.rs`](../src/practice/p0345_majority_element.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0345
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
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
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn single_element_and_negatives() {
        assert_eq!(Solution::majority_element(vec![-7]), -7);
        assert_eq!(
            Solution::majority_element(vec![-1_000_000_000, 5, -1_000_000_000]),
            -1_000_000_000
        );
    }

    #[test]
    fn large_barely_majority() {
        // 25_001 copies of 42 and 24_999 distinct other values.
        let mut nums: Vec<i32> = (0..24_999).collect();
        nums.extend(std::iter::repeat_n(42_000, 25_001));
        nums.rotate_left(12_345);
        assert_eq!(Solution::majority_element(nums), 42_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count the values in a `HashMap<i32, usize>` and return the key with the largest count.
Because the majority element occurs more than `n / 2` times, its count is strictly the largest,
so ties cannot happen.

**Complexity:** `O(n)` average time, `O(n)` space.

Follow-up, Boyer-Moore voting in `O(1)` space: keep a `candidate` and a `count`; for each value,
if `count == 0` take it as the new candidate; then add `1` if it equals the candidate, else
subtract `1`. Every non-majority value can cancel at most one majority vote, so the majority
value survives as the final candidate.

**Rust concepts:**
- `into_iter()` on a `HashMap` yields owned `(key, value)` pairs.
- `max_by_key` returns an `Option` (the map could be empty); `map(|(value, _)| value)` keeps the
  key and `unwrap_or_default()` avoids a panic.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for x in nums {
            *counts.entry(x).or_insert(0) += 1;
        }
        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(value, _)| value)
            .unwrap_or_default()
    }
}
```

Full tested solution: [`src/solutions/p0345_majority_element.rs`](../src/solutions/p0345_majority_element.rs)

</details>
