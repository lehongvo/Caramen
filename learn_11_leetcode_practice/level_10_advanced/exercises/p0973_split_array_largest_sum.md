# Problem 0973: Split Array Largest Sum

- **Difficulty:** Hard
- **Topics:** binary search on the answer, a greedy feasibility check, monotone predicates, `i64` sums
- **Source:** [LeetCode 410. Split Array Largest Sum](https://leetcode.com/problems/split-array-largest-sum/)

## Problem

Split the array `nums` into exactly `k` non-empty **contiguous** parts so that the largest part
sum is as small as possible. Return that minimal largest sum.

Implement `Solution::split_array(nums: Vec<i32>, k: i32) -> i32`.

## Examples

```text
Input:  nums = [7, 2, 5, 10, 8], k = 2
Output: 18          ([7, 2, 5] and [10, 8])

Input:  nums = [1, 2, 3, 4, 5], k = 2
Output: 9           ([1, 2, 3] and [4, 5])
```

## Constraints

- `1 <= nums.len() <= 1000`, `0 <= nums[i] <= 10^6`
- `1 <= k <= min(50, nums.len())`

## Hints

1. For a limit `L`, a greedy scan tells how many parts are needed if no part may exceed `L`:
   start a new part whenever adding the next number would exceed `L`.
2. "At most `k` parts are needed" is false for small `L` and true for large `L`: binary search the
   smallest `L` for which it is true.
3. The answer lies between `max(nums)` and `sum(nums)`. Using fewer than `k` parts is fine:
   splitting a part further never increases the largest sum.

## Starter code

Write your code in [`src/practice/p0973_split_array_largest_sum.rs`](../src/practice/p0973_split_array_largest_sum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0973
```

```rust
pub struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
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

    fn naive(nums: &[i32], k: usize) -> i32 {
        // dp[p][i]: best largest sum splitting nums[..i] into p parts.
        let n = nums.len();
        let mut prefix = vec![0_i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + i64::from(nums[i]);
        }
        let mut dp = vec![vec![i64::MAX; n + 1]; k + 1];
        dp[0][0] = 0;
        for p in 1..=k {
            for i in 1..=n {
                for j in 0..i {
                    if dp[p - 1][j] != i64::MAX {
                        let candidate = dp[p - 1][j].max(prefix[i] - prefix[j]);
                        dp[p][i] = dp[p][i].min(candidate);
                    }
                }
            }
        }
        dp[k][n] as i32
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(Solution::split_array(vec![1, 4, 4], 3), 4);
    }

    #[test]
    fn one_part_and_zeros() {
        assert_eq!(Solution::split_array(vec![3, 1, 4], 1), 8);
        assert_eq!(Solution::split_array(vec![0, 0, 0], 2), 0);
        assert_eq!(Solution::split_array(vec![0, 5, 0], 3), 5);
    }

    #[test]
    fn matches_dp() {
        let nums: Vec<i32> = (0..40).map(|i: i32| (i * 7_919) % 97).collect();
        for k in 1..=8 {
            assert_eq!(
                Solution::split_array(nums.clone(), k as i32),
                naive(&nums, k),
                "k = {k}"
            );
        }
    }

    #[test]
    fn large_values() {
        let nums = vec![1_000_000; 1_000];
        assert_eq!(Solution::split_array(nums.clone(), 1), 1_000_000_000);
        assert_eq!(Solution::split_array(nums.clone(), 50), 20_000_000);
        assert_eq!(Solution::split_array(nums, 3), 334_000_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** binary search over `L` in `[max, sum]` with the predicate "the greedy needs at most `k`
parts". The greedy is optimal for a fixed `L` (each part is made as long as possible), and the
predicate is monotone in `L`, so the smallest feasible `L` is the answer.

**Complexity:** `O(n log S)` time with `S = sum(nums)`, `O(1)` space.

**Rust concepts:**
- A closure `parts_needed(limit)` captures `nums` by reference and is called inside the search.
- The usual "lower bound" loop `while lo < hi { mid; if ok { hi = mid } else { lo = mid + 1 } }`.
- Sums of up to `10^9` fit in `i32`, but `i64` makes the arithmetic obviously safe.

```rust
pub struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let parts_needed = |limit: i64| {
            let mut parts = 1;
            let mut current = 0_i64;
            for &x in &nums {
                let x = i64::from(x);
                if current + x > limit {
                    parts += 1;
                    current = 0;
                }
                current += x;
            }
            parts
        };
        let mut lo = nums.iter().copied().map(i64::from).max().unwrap_or(0);
        let mut hi: i64 = nums.iter().copied().map(i64::from).sum();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if parts_needed(mid) <= k {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
    }
}
```

Full tested solution: [`src/solutions/p0973_split_array_largest_sum.rs`](../src/solutions/p0973_split_array_largest_sum.rs)

</details>
