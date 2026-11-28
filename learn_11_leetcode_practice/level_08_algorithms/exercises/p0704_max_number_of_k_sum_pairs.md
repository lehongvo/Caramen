# Problem 0704: Max Number of K-Sum Pairs

- **Difficulty:** Medium
- **Topics:** sorting + two pointers, `match` on `Ordering`, `i64` sums
- **Source:** [LeetCode 1679. Max Number of K-Sum Pairs](https://leetcode.com/problems/max-number-of-k-sum-pairs/)

## Problem

You are given a vector `nums` and an integer `k`. In one operation you choose two elements at
different positions whose sum is exactly `k` and remove both of them from the vector.

Return the maximum number of operations you can perform.

Implement `Solution::max_operations(nums: Vec<i32>, k: i32) -> i32`.

## Examples

```text
Input:  nums = [1, 2, 3, 4], k = 5
Output: 2             (remove 1 and 4, then 2 and 3)

Input:  nums = [3, 1, 3, 4, 3], k = 6
Output: 1             (remove two of the 3s; the third 3 has no partner left)
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `1 <= nums[i] <= 10^9`
- `1 <= k <= 10^9`

## Hints

1. Removing a pair never makes another pair impossible, so you can be greedy.
2. After sorting, the smallest and the largest values are at the two ends. Compare their sum
   with `k`.

## Starter code

Write your code in [`src/practice/p0704_max_number_of_k_sum_pairs.rs`](../src/practice/p0704_max_number_of_k_sum_pairs.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0704
```

```rust
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
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
    fn example_two_pairs() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    }

    #[test]
    fn example_one_pair() {
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::max_operations(vec![1], 2), 0);
    }

    #[test]
    fn all_equal_halves() {
        assert_eq!(Solution::max_operations(vec![2, 2, 2, 2, 2], 4), 2);
    }

    #[test]
    fn large_values_no_pair() {
        assert_eq!(
            Solution::max_operations(vec![1_000_000_000, 1_000_000_000], 1_000_000_000),
            0
        );
    }

    #[test]
    fn long_input_all_pairs() {
        // 1..=100_000 with k = 100_001: (1, 100000), (2, 99999), ... gives 50_000 pairs.
        let nums: Vec<i32> = (1..=100_000).rev().collect();
        assert_eq!(Solution::max_operations(nums, 100_001), 50_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sorting + two pointers: sort, then look at the two ends. If `nums[lo] + nums[hi]`
is smaller than `k`, `nums[lo]` cannot pair with anything (its largest partner is too small), so
skip it. If the sum is larger, `nums[hi]` cannot pair with anything, so skip it. If the sum is
exactly `k`, remove both and count one operation. This is the same reasoning as Two Sum II,
repeated until the pointers meet.

**Complexity:** `O(n log n)` time for the sort, `O(1)` extra space. (A `HashMap` of counts gives
`O(n)` time at the cost of `O(n)` memory.)

**Rust concepts:**
- The sum of two values up to `10^9` is up to `2 * 10^9`, which still fits in `i32` (max about
  `2.147 * 10^9`), but `i64::from` makes the code safe even if the limits change.
- `i64::from(x)` is a lossless conversion; prefer it over `as` when widening.

```rust
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let (mut lo, mut hi) = (0, nums.len());
        let mut operations = 0;
        while lo + 1 < hi {
            let sum = i64::from(nums[lo]) + i64::from(nums[hi - 1]);
            match sum.cmp(&i64::from(k)) {
                Ordering::Less => lo += 1,
                Ordering::Greater => hi -= 1,
                Ordering::Equal => {
                    operations += 1;
                    lo += 1;
                    hi -= 1;
                }
            }
        }
        operations
    }
}
```

Full tested solution: [`src/solutions/p0704_max_number_of_k_sum_pairs.rs`](../src/solutions/p0704_max_number_of_k_sum_pairs.rs)

</details>
