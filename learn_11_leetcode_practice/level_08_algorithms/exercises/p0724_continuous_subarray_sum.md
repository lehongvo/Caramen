# Problem 0724: Continuous Subarray Sum

- **Difficulty:** Medium
- **Topics:** prefix remainders, `HashMap` of first positions, `i64` to avoid overflow
- **Source:** [LeetCode 523. Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum/)

## Problem

Given a vector `nums` of non-negative integers and a positive integer `k`, return `true` if
`nums` has a **good** subarray, and `false` otherwise. A good subarray:

- is contiguous and has **at least two** elements, and
- has a sum that is a multiple of `k` (`0` counts as a multiple of every `k`).

Implement `Solution::check_subarray_sum(nums: Vec<i32>, k: i32) -> bool`.

## Examples

```text
Input:  nums = [23, 2, 4, 6, 7], k = 6
Output: true          ([2, 4] sums to 6)

Input:  nums = [23, 2, 6, 4, 7], k = 6
Output: true          (the whole array sums to 42 = 7 * 6)

Input:  nums = [23, 2, 6, 4, 7], k = 13
Output: false
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `0 <= nums[i] <= 10^9`
- `0 <= sum(nums) <= 2^31 - 1`
- `1 <= k <= 2^31 - 1`

## Hints

1. `sum(nums[a..b]) = prefix(b) - prefix(a)`. When is that difference a multiple of `k`?
2. Two prefixes with the same remainder modulo `k` enclose a multiple of `k`. The subarray must
   also be at least two long, so remember the **first** position of each remainder.

## Starter code

Write your code in [`src/practice/p0724_continuous_subarray_sum.rs`](../src/practice/p0724_continuous_subarray_sum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0724
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
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
    fn example_short_subarray() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    }

    #[test]
    fn example_whole_array() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    }

    #[test]
    fn example_no_good_subarray() {
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }

    #[test]
    fn single_element_is_too_short() {
        assert!(!Solution::check_subarray_sum(vec![0], 1));
        assert!(!Solution::check_subarray_sum(vec![6], 6));
    }

    #[test]
    fn zeros_sum_to_a_multiple() {
        assert!(Solution::check_subarray_sum(vec![0, 0], 1));
        assert!(Solution::check_subarray_sum(vec![5, 0, 0, 0], 3));
    }

    #[test]
    fn remainder_repeats_but_too_close() {
        // Prefix remainders mod 2: 0, 1, 1. The repeat is only one element apart.
        assert!(!Solution::check_subarray_sum(vec![1, 0], 2));
    }

    #[test]
    fn huge_values_and_k() {
        let nums = vec![1_000_000_000, 1_000_000_000];
        assert!(Solution::check_subarray_sum(nums.clone(), 2_000_000_000));
        assert!(!Solution::check_subarray_sum(nums, i32::MAX));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** prefix remainders: `prefix(b) - prefix(a)` is a multiple of `k` exactly when
`prefix(a) % k == prefix(b) % k`. Walk through the array keeping the running remainder and a
map from each remainder to the **first** prefix length where it appeared (remainder `0` at
length `0`). When a remainder shows up again at prefix length `p`, the subarray between the
first occurrence and `p` has a sum divisible by `k`; it is good when it is at least 2 long.
Keeping the earliest position gives the longest candidate, so no good subarray is missed.

**Complexity:** `O(n)` time, `O(min(n, k))` space for the map.

**Rust concepts:**
- The remainder plus the next value can exceed `i32::MAX` (`k` itself can be close to it), so
  the arithmetic is done in `i64`.
- `*map.entry(key).or_insert(p)` reads the first position or records the current one.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let k = i64::from(k);
        let mut first: HashMap<i64, usize> = HashMap::from([(0, 0)]);
        let mut remainder = 0i64;
        for (i, &x) in nums.iter().enumerate() {
            remainder = (remainder + i64::from(x)) % k;
            let p = i + 1; // prefix length
            let start = *first.entry(remainder).or_insert(p);
            if p - start >= 2 {
                return true;
            }
        }
        false
    }
}
```

Full tested solution: [`src/solutions/p0724_continuous_subarray_sum.rs`](../src/solutions/p0724_continuous_subarray_sum.rs)

</details>
