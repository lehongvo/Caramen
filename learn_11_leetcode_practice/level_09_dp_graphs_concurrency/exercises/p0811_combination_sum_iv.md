# Problem 0811: Combination Sum IV

- **Difficulty:** Medium
- **Topics:** counting ordered sequences with DP, loop order vs problem 0810, `wrapping_add`
- **Source:** [LeetCode 377. Combination Sum IV](https://leetcode.com/problems/combination-sum-iv/)

## Problem

Given an array of **distinct** positive integers `nums` and a positive `target`, return the
number of **ordered** sequences of elements of `nums` (each element may be used any number of
times) whose sum is exactly `target`. Unlike problem 0810, order matters here: `1 + 2` and
`2 + 1` are two different sequences.

Implement `Solution::combination_sum4(nums: Vec<i32>, target: i32) -> i32`.

## Examples

```text
Input:  nums = [1, 2, 3], target = 4
Output: 7
Explanation: (1,1,1,1) (1,1,2) (1,2,1) (2,1,1) (2,2) (1,3) (3,1).

Input:  nums = [9], target = 3
Output: 0
```

## Constraints

- `1 <= nums.len() <= 200`
- `1 <= nums[i] <= 1000`, all values distinct
- `1 <= target <= 1000`
- The answer fits in an `i32` (intermediate counts may not).

## Hints

1. Let `ways[t]` be the number of sequences that sum to `t`. The **last** element of such a
   sequence is some `x` in `nums` with `x <= t`.
2. That gives `ways[t] = sum of ways[t - x]`. Compare with problem 0810: here the target is the
   outer loop and the numbers the inner loop.
3. Counts for small targets can be astronomically large even when the final answer is small.

## Starter code

Write your code in [`src/practice/p0811_combination_sum_iv.rs`](../src/practice/p0811_combination_sum_iv.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0811
```

```rust
pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
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
    fn example_target_four() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }

    #[test]
    fn example_unreachable() {
        assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
    }

    #[test]
    fn ones_and_twos_give_fibonacci() {
        assert_eq!(Solution::combination_sum4(vec![1, 2], 10), 89);
        assert_eq!(Solution::combination_sum4(vec![1], 1), 1);
    }

    #[test]
    fn large_answer() {
        assert_eq!(Solution::combination_sum4(vec![2, 1, 3], 35), 1_132_436_852);
    }

    #[test]
    fn intermediate_overflow_is_harmless() {
        let nums: Vec<i32> = (10..=1000).step_by(10).collect();
        assert_eq!(Solution::combination_sum4(nums, 999), 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `ways[0] = 1` (the empty sequence). For `t` in `1..=target`, add `ways[t - x]` for
every `x <= t`. Putting the target in the outer loop means each position `t` sees every
possible last element, so different orders are counted separately.

**Complexity:** `O(target * nums.len())` time, `O(target)` space.

**Rust concepts:**
- `u64::wrapping_add` keeps the arithmetic modulo `2^64` instead of panicking on overflow in
  debug builds. The true answer is below `2^31`, so the value modulo `2^64` is exact.
  For example `nums = [10, 20, ..., 1000]` with `target = 999` has huge counts for multiples of
  10, but `0` ways for the odd target.
- `filter` plus `fold` over a borrowed slice keeps the inner loop allocation-free.

```rust
pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut ways = vec![0_u64; target + 1];
        ways[0] = 1;
        for t in 1..=target {
            ways[t] = nums
                .iter()
                .map(|&x| x as usize)
                .filter(|&x| x <= t)
                .fold(0_u64, |acc, x| acc.wrapping_add(ways[t - x]));
        }
        ways[target] as i32
    }
}
```

Full tested solution: [`src/solutions/p0811_combination_sum_iv.rs`](../src/solutions/p0811_combination_sum_iv.rs)

</details>
