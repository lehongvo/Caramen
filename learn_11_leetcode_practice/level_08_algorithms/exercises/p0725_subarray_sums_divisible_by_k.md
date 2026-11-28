# Problem 0725: Subarray Sums Divisible by K

- **Difficulty:** Medium
- **Topics:** prefix remainders, counting equal pairs, `rem_euclid` for negative numbers
- **Source:** [LeetCode 974. Subarray Sums Divisible by K](https://leetcode.com/problems/subarray-sums-divisible-by-k/)

## Problem

Given a vector `nums` (values may be negative) and an integer `k`, return the number of
non-empty **contiguous** subarrays whose sum is divisible by `k`.

Implement `Solution::subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32`.

## Examples

```text
Input:  nums = [4, 5, 0, -2, -3, 1], k = 5
Output: 7
  ([4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3])

Input:  nums = [5], k = 9
Output: 0
```

## Constraints

- `1 <= nums.len() <= 3 * 10^4`
- `-10^4 <= nums[i] <= 10^4`
- `2 <= k <= 10^4`

## Hints

1. A subarray is divisible by `k` exactly when the two prefix sums around it have the same
   remainder modulo `k`.
2. Count how many earlier prefixes had each remainder.
3. In Rust, `-3 % 5` is `-3`, not `2`. Which method gives the mathematical remainder?

## Starter code

Write your code in [`src/practice/p0725_subarray_sums_divisible_by_k.rs`](../src/practice/p0725_subarray_sums_divisible_by_k.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0725
```

```rust
pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
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
    fn example_seven_subarrays() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
    }

    #[test]
    fn example_none() {
        assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
    }

    #[test]
    fn negative_values_need_euclidean_remainder() {
        // [2] and [-1, 2, 9] are even.
        assert_eq!(Solution::subarrays_div_by_k(vec![-1, 2, 9], 2), 2);
        assert_eq!(Solution::subarrays_div_by_k(vec![-5, 5], 5), 3);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(Solution::subarrays_div_by_k(vec![0, 0, 0], 3), 6);
    }

    #[test]
    fn every_subarray_counts() {
        // 30_000 * 30_001 / 2 subarrays, all divisible by 5.
        assert_eq!(
            Solution::subarrays_div_by_k(vec![5; 30_000], 5),
            450_015_000
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** prefix remainders + counting: walk through the array keeping `r`, the remainder of
the running sum modulo `k`, and `counts[r]`, how many earlier prefixes (including the empty
prefix, remainder `0`) had that remainder. Every earlier prefix with the same remainder starts
a subarray ending here whose sum is divisible by `k`, so add `counts[r]` to the answer, then
record the current prefix with `counts[r] += 1`.

The remainder must be normalised to `0..k`: Rust's `%` keeps the sign of the dividend, so the
prefix sums `-2` and `3` would get the different remainders `-2` and `3` modulo 5, although
they differ by `5`. `rem_euclid` always returns a value in `0..k`.

**Complexity:** `O(n + k)` time, `O(k)` space.

**Rust concepts:**
- [`i32::rem_euclid`](https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid) vs
  `%`: `(-3).rem_euclid(5) == 2` while `-3 % 5 == -3`.
- Because the remainders live in `0..k`, a `Vec` of length `k` replaces a `HashMap`.

```rust
pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = vec![0; k as usize];
        counts[0] = 1; // the empty prefix
        let (mut remainder, mut total) = (0, 0);
        for &x in &nums {
            remainder = (remainder + x).rem_euclid(k);
            let seen = &mut counts[remainder as usize];
            total += *seen;
            *seen += 1;
        }
        total
    }
}
```

Full tested solution: [`src/solutions/p0725_subarray_sums_divisible_by_k.rs`](../src/solutions/p0725_subarray_sums_divisible_by_k.rs)

</details>
