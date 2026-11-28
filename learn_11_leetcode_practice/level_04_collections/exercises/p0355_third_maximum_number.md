# Problem 0355: Third Maximum Number

- **Difficulty:** Easy
- **Topics:** `BTreeSet` as a small sorted window, `pop_first`, `first`, `last`
- **Source:** [LeetCode 414. Third Maximum Number](https://leetcode.com/problems/third-maximum-number/)

## Problem

Given a vector `nums`, return its **third largest distinct** value. If there are fewer than
three distinct values, return the largest value instead.

LeetCode's follow-up asks for an `O(n)` solution.

Implement `Solution::third_max(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [3, 2, 1]
Output: 1

Input:  nums = [1, 2]
Output: 2          (only two distinct values: return the maximum)

Input:  nums = [2, 2, 3, 1]
Output: 1          (distinct values 3, 2, 1; the two 2s count once)
```

## Constraints

- `1 <= nums.len() <= 10^4`
- `-2^31 <= nums[i] <= 2^31 - 1`

## Hints

1. A `BTreeSet` keeps distinct values in sorted order.
2. You only care about the three largest values. After each insert, if the set holds four
   values, drop the smallest one with `pop_first()`.

## Starter code

Write your code in [`src/practice/p0355_third_maximum_number.rs`](../src/practice/p0355_third_maximum_number.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0355
```

```rust
use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
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
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    }

    #[test]
    fn example_two_fewer_than_three() {
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
    }

    #[test]
    fn example_three_duplicates() {
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }

    #[test]
    fn extreme_values() {
        assert_eq!(Solution::third_max(vec![1, 2, i32::MIN]), i32::MIN);
        assert_eq!(Solution::third_max(vec![i32::MAX, i32::MAX]), i32::MAX);
        assert_eq!(
            Solution::third_max(vec![i32::MIN, i32::MIN, i32::MIN]),
            i32::MIN
        );
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..10_000).map(|i| (i * 37) % 10_000).collect();
        assert_eq!(Solution::third_max(nums), 9_997);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** maintain a `BTreeSet` with the (at most) three largest distinct values seen so far.
Insert each number; when the set grows to four elements, remove its smallest. At the end, if the
set has three values the smallest of them (`first`) is the third maximum, otherwise the answer
is the largest (`last`).

**Complexity:** `O(n)` time (every set operation works on at most 4 elements), `O(1)` space.

**Rust concepts:**
- `BTreeSet::insert` ignores duplicates, which takes care of "distinct".
- `pop_first()` removes and returns the smallest element; `first()` and `last()` peek at the
  smallest and largest. All of them return `Option`.
- Using a set avoids the classic bug of initialising "max" variables with `i32::MIN`, which is a
  legal input value here.

```rust
use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut top: BTreeSet<i32> = BTreeSet::new();
        for x in nums {
            top.insert(x);
            if top.len() > 3 {
                top.pop_first();
            }
        }
        let answer = if top.len() == 3 {
            top.first()
        } else {
            top.last()
        };
        // `nums` is non-empty, so the set is too.
        answer.copied().unwrap_or_default()
    }
}
```

Full tested solution: [`src/solutions/p0355_third_maximum_number.rs`](../src/solutions/p0355_third_maximum_number.rs)

</details>
