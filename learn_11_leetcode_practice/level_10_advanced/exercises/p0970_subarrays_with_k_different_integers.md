# Problem 0970: Subarrays with K Different Integers

- **Difficulty:** Hard
- **Topics:** "exactly k = at most k - at most (k - 1)", sliding window with a count array, counting subarrays by their right end
- **Source:** [LeetCode 992. Subarrays with K Different Integers](https://leetcode.com/problems/subarrays-with-k-different-integers/)

## Problem

Given an integer array `nums` and an integer `k`, return the number of **good** subarrays: non-empty
contiguous subarrays that contain **exactly** `k` different integers.

Implement `Solution::subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32`.

## Examples

```text
Input:  nums = [1, 2, 1, 2, 3], k = 2
Output: 7     ([1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2])

Input:  nums = [1, 2, 1, 3, 4], k = 3
Output: 3     ([1,2,1,3], [2,1,3], [1,3,4])
```

## Constraints

- `1 <= nums.len() <= 2 * 10^4`
- `1 <= nums[i], k <= nums.len()`

## Hints

1. "Exactly `k`" is awkward for a sliding window, but "at most `k`" is easy: for each right end,
   the valid left ends form a suffix `[left, right]`, contributing `right - left + 1` subarrays.
2. `exactly(k) = at_most(k) - at_most(k - 1)`.
3. Values are at most `n`, so a `Vec<usize>` of counts replaces a hash map.

## Starter code

Write your code in [`src/practice/p0970_subarrays_with_k_different_integers.rs`](../src/practice/p0970_subarrays_with_k_different_integers.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0970
```

```rust
pub struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
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
    use std::collections::HashSet;

    fn naive(nums: &[i32], k: usize) -> i32 {
        let mut total = 0;
        for i in 0..nums.len() {
            let mut seen = HashSet::new();
            for &x in &nums[i..] {
                seen.insert(x);
                if seen.len() == k {
                    total += 1;
                }
            }
        }
        total
    }

    #[test]
    fn examples() {
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2),
            7
        );
        assert_eq!(
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3),
            3
        );
    }

    #[test]
    fn edge_cases() {
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1], 1), 1);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 1, 1], 1), 6);
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2], 3), 0);
    }

    #[test]
    fn matches_naive() {
        let nums: Vec<i32> = (0..300).map(|i: i32| (i * 7_919) % 6 + 1).collect();
        for k in 1..=6 {
            assert_eq!(
                Solution::subarrays_with_k_distinct(nums.clone(), k as i32),
                naive(&nums, k),
                "k = {k}"
            );
        }
    }

    #[test]
    fn all_distinct_large() {
        let n = 20_000;
        let nums: Vec<i32> = (1..=n).collect();
        // Exactly k distinct means exactly length k: n - k + 1 subarrays.
        assert_eq!(Solution::subarrays_with_k_distinct(nums.clone(), 1), n);
        assert_eq!(Solution::subarrays_with_k_distinct(nums, 500), n - 499);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `at_most(k)` slides a window, shrinking from the left whenever it holds more than `k`
distinct values; every right end then adds the number of valid left ends. Subtracting
`at_most(k - 1)` leaves the subarrays with exactly `k` distinct values.

**Complexity:** `O(n)` time, `O(n)` space.

**Rust concepts:**
- A nested helper function keeps the two window passes identical.
- Counting in `usize` and converting once at the end keeps index arithmetic in one type; the
  answer is at most `n (n + 1) / 2 = 2 * 10^8`, which fits in `i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        fn at_most(nums: &[i32], k: usize) -> usize {
            let mut counts = vec![0_usize; nums.len() + 1];
            let mut distinct = 0;
            let mut left = 0;
            let mut total = 0;
            for (right, &value) in nums.iter().enumerate() {
                let v = value as usize;
                if counts[v] == 0 {
                    distinct += 1;
                }
                counts[v] += 1;
                while distinct > k {
                    let out = nums[left] as usize;
                    counts[out] -= 1;
                    if counts[out] == 0 {
                        distinct -= 1;
                    }
                    left += 1;
                }
                total += right + 1 - left;
            }
            total
        }
        let k = k as usize;
        (at_most(&nums, k) - at_most(&nums, k - 1)) as i32
    }
}
```

Full tested solution: [`src/solutions/p0970_subarrays_with_k_different_integers.rs`](../src/solutions/p0970_subarrays_with_k_different_integers.rs)

</details>
