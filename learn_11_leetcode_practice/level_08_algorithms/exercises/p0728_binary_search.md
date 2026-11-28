# Problem 0728: Binary Search

- **Difficulty:** Easy
- **Topics:** binary search, half-open bounds `lo..hi`, `match` on `Ordering`, overflow-free midpoint
- **Source:** [LeetCode 704. Binary Search](https://leetcode.com/problems/binary-search/)

## Problem

Given a vector `nums` sorted in ascending order with **distinct** values, and an integer
`target`, return the index of `target` in `nums`, or `-1` if it is not there. Your algorithm
must run in `O(log n)` time, so write the search yourself (do not scan the vector).

Implement `Solution::search(nums: Vec<i32>, target: i32) -> i32`.

## Examples

```text
Input:  nums = [-1, 0, 3, 5, 9, 12], target = 9
Output: 4

Input:  nums = [-1, 0, 3, 5, 9, 12], target = 2
Output: -1
```

## Constraints

- `1 <= nums.len() <= 10^4`
- `-10^4 < nums[i], target < 10^4`
- All values of `nums` are distinct and sorted in ascending order.

## Hints

1. Keep a range of indices `lo..hi` that still may contain the target, and look at its middle.
2. With a half-open range, the loop runs while `lo < hi`; decide carefully whether the new
   bound is `mid` or `mid + 1`.

## Starter code

Write your code in [`src/practice/p0728_binary_search.rs`](../src/practice/p0728_binary_search.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0728
```

```rust
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
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

    fn sample() -> Vec<i32> {
        vec![-1, 0, 3, 5, 9, 12]
    }

    #[test]
    fn example_found() {
        assert_eq!(Solution::search(sample(), 9), 4);
    }

    #[test]
    fn example_missing() {
        assert_eq!(Solution::search(sample(), 2), -1);
    }

    #[test]
    fn first_and_last_positions() {
        assert_eq!(Solution::search(sample(), -1), 0);
        assert_eq!(Solution::search(sample(), 12), 5);
    }

    #[test]
    fn outside_the_range() {
        assert_eq!(Solution::search(sample(), -5), -1);
        assert_eq!(Solution::search(sample(), 100), -1);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::search(vec![5], 5), 0);
        assert_eq!(Solution::search(vec![5], -5), -1);
    }

    #[test]
    fn many_values_of_a_long_array() {
        let nums: Vec<i32> = (0..10_000).map(|i| i * 2 - 9_999).collect();
        for (i, &x) in nums.iter().enumerate().step_by(97) {
            assert_eq!(Solution::search(nums.clone(), x), i as i32);
            assert_eq!(Solution::search(nums.clone(), x + 1), -1);
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** binary search: the half-open range `lo..hi` contains every index where the target
could still be. Compare the middle element: if it is smaller than the target, the target can
only be to the right (`lo = mid + 1`); if it is larger, only to the left (`hi = mid`, which
excludes `mid` because the range is half-open). Each step halves the range, and an empty range
(`lo == hi`) means the target is absent.

Half-open bounds fit Rust well: `hi` starts at `nums.len()`, and no step ever computes
`mid - 1`, which could underflow a `usize` when `mid == 0`.

**Complexity:** `O(log n)` time, `O(1)` space.

**Rust concepts:**
- `lo + (hi - lo) / 2` never overflows, unlike `(lo + hi) / 2` for huge indices.
- `match nums[mid].cmp(&target)` with the three
  [`Ordering`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html) variants is exhaustive.
- In real code, [`slice::binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)
  does this for you and returns `Result<usize, usize>`.

```rust
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid,
                Ordering::Equal => return mid as i32,
            }
        }
        -1
    }
}
```

Full tested solution: [`src/solutions/p0728_binary_search.rs`](../src/solutions/p0728_binary_search.rs)

</details>
