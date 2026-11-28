# Problem 0487: Maximum Subarray

- **Difficulty:** Medium
- **Topics:** Kadane's algorithm as a `scan` (best sum ending here) followed by `max`
- **Source:** [LeetCode 53. Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)

## Problem

Given an integer array `nums`, find the **non-empty** contiguous subarray with the largest sum
and return that sum.

Implement `Solution::max_sub_array(nums: Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
Output: 6
Explanation: the subarray [4, -1, 2, 1] has the largest sum.

Input:  nums = [1]
Output: 1

Input:  nums = [5, 4, -1, 7, 8]
Output: 23
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `-10^4 <= nums[i] <= 10^4`

## Hints

1. Let `best_here` be the largest sum of a subarray that **ends** at the current position. It is
   either the current number alone, or the current number appended to the best subarray ending
   at the previous position: `best_here = max(x, best_here + x)`.
2. The answer is the maximum `best_here` over all positions. `scan` produces the `best_here`
   values, and `max` picks the largest.

## Starter code

Write your code in [`src/practice/p0487_maximum_subarray.rs`](../src/practice/p0487_maximum_subarray.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0487
```

```rust
pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
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
    fn example_mixed() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn example_single() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn example_almost_all_positive() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn all_negative_picks_the_largest_element() {
        assert_eq!(Solution::max_sub_array(vec![-3, -1, -2]), -1);
        assert_eq!(Solution::max_sub_array(vec![-10_000]), -10_000);
    }

    #[test]
    fn large_input() {
        let nums = vec![10_000; 100_000];
        assert_eq!(Solution::max_sub_array(nums), 1_000_000_000);
        let alternating: Vec<i32> = (0..100_000)
            .map(|i| if i % 2 == 0 { 3 } else { -2 })
            .collect();
        // 50 000 threes and the 49 999 minus-twos between them: 150 000 - 99 998.
        assert_eq!(Solution::max_sub_array(alternating), 50_002);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** Kadane's algorithm. Walking left to right, a running sum that has become negative can
only hurt the next subarrays, so the best sum ending at `x` restarts at `x` in that case. Emitting
every "best sum ending here" with `scan` and taking the `max` gives the answer.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `scan(0, |best_here, &x| ...)` keeps `best_here` as state; starting from `0` makes the first
  step `max(x, 0 + x) = x`, as required.
- `max()` returns an `Option` because an iterator might be empty; `nums` never is, and
  `unwrap_or(i32::MIN)` keeps the code panic-free.
- Sums stay within `10^5 * 10^4 = 10^9`, which fits in an `i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .scan(0, |best_here, &x| {
                *best_here = x.max(*best_here + x);
                Some(*best_here)
            })
            .max()
            .unwrap_or(i32::MIN)
    }
}
```

Full tested solution: [`src/solutions/p0487_maximum_subarray.rs`](../src/solutions/p0487_maximum_subarray.rs)

</details>
