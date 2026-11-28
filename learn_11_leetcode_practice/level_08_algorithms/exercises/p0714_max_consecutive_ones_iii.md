# Problem 0714: Max Consecutive Ones III

- **Difficulty:** Medium
- **Topics:** sliding window with a budget, counting inside the window
- **Source:** [LeetCode 1004. Max Consecutive Ones III](https://leetcode.com/problems/max-consecutive-ones-iii/)

## Problem

You are given a binary vector `nums` (every value is `0` or `1`) and an integer `k`. You may
flip **at most** `k` zeros into ones. Return the length of the longest block of consecutive
ones you can obtain.

Implement `Solution::longest_ones(nums: Vec<i32>, k: i32) -> i32`.

## Examples

```text
Input:  nums = [1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], k = 2
Output: 6             (flip the zeros at indices 5 and 10: [1, 1, 1, 1, 1, 1] at 5..=10)

Input:  nums = [0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], k = 3
Output: 10            (indices 2..=11 after flipping the zeros at 4, 5 and 9)
```

## Constraints

- `1 <= nums.len() <= 10^5`
- `nums[i]` is `0` or `1`.
- `0 <= k <= nums.len()`

## Hints

1. Rephrase the question: find the longest window that contains at most `k` zeros.
2. Grow the window on the right and shrink it on the left whenever it holds too many zeros.

## Starter code

Write your code in [`src/practice/p0714_max_consecutive_ones_iii.rs`](../src/practice/p0714_max_consecutive_ones_iii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0714
```

```rust
pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
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
    fn example_budget_two() {
        assert_eq!(
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2),
            6
        );
    }

    #[test]
    fn example_budget_three() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        assert_eq!(Solution::longest_ones(nums, 3), 10);
    }

    #[test]
    fn no_flips_allowed() {
        assert_eq!(Solution::longest_ones(vec![1, 0, 1, 1], 0), 2);
        assert_eq!(Solution::longest_ones(vec![0, 0, 0], 0), 0);
    }

    #[test]
    fn enough_flips_for_everything() {
        assert_eq!(Solution::longest_ones(vec![0, 0, 0], 3), 3);
    }

    #[test]
    fn long_alternating_input() {
        // 0, 1, 0, 1, ...: with k = 1000 the best window holds 1000 zeros and 1001 ones.
        let nums: Vec<i32> = (0..100_000).map(|i| i % 2).collect();
        assert_eq!(Solution::longest_ones(nums, 1000), 2001);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sliding window with a budget: a window can be turned into all ones exactly when it
contains at most `k` zeros. Extend the window to the right and count its zeros; while the count
exceeds `k`, move `left` forward (decrementing the count when a zero leaves). After that, the
window is valid and its length is a candidate.

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `usize::from(x == 0)` turns a comparison into `0` or `1`, so counting needs no `if`.
- `k` is converted once to `usize` so it can be compared with the `usize` zero count.

```rust
pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let (mut left, mut zeros, mut best) = (0, 0, 0);
        for (right, &x) in nums.iter().enumerate() {
            zeros += usize::from(x == 0);
            while zeros > k {
                zeros -= usize::from(nums[left] == 0);
                left += 1;
            }
            best = best.max(right + 1 - left);
        }
        best as i32
    }
}
```

Full tested solution: [`src/solutions/p0714_max_consecutive_ones_iii.rs`](../src/solutions/p0714_max_consecutive_ones_iii.rs)

</details>
