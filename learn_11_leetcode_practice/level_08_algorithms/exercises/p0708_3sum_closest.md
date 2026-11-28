# Problem 0708: 3Sum Closest

- **Difficulty:** Medium
- **Topics:** sorting + two pointers, tracking the best candidate, `abs`, early return
- **Source:** [LeetCode 16. 3Sum Closest](https://leetcode.com/problems/3sum-closest/)

## Problem

Given a vector `nums` and an integer `target`, choose three values at three different positions
whose sum is as close as possible to `target` (smallest absolute difference). Return that
**sum** (not the difference).

The inputs are chosen so that exactly one closest sum exists.

Implement `Solution::three_sum_closest(nums: Vec<i32>, target: i32) -> i32`.

## Examples

```text
Input:  nums = [-1, 2, 1, -4], target = 1
Output: 2             (-1 + 2 + 1 = 2 is at distance 1 from the target)

Input:  nums = [0, 0, 0], target = 1
Output: 0
```

## Constraints

- `3 <= nums.len() <= 500`
- `-1000 <= nums[i] <= 1000`
- `-10^4 <= target <= 10^4`

## Hints

1. This is 3Sum with "equal to zero" replaced by "as close as possible". The same sorted
   two-pointer scan applies.
2. When the sum is too small, moving the left pointer is the only move that can bring it closer.

## Starter code

Write your code in [`src/practice/p0708_3sum_closest.rs`](../src/practice/p0708_3sum_closest.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0708
```

```rust
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
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
    fn example_mixed_signs() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn example_three_zeros() {
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn target_far_below() {
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    }

    #[test]
    fn exact_match_exists() {
        // Sorted: -5 -5 -4 0 0 3 3 4 5 and -5 + 0 + 3 = -2.
        assert_eq!(
            Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
            -2
        );
    }

    #[test]
    fn target_far_above() {
        let nums: Vec<i32> = (-250..250).collect();
        // The largest possible sum is 249 + 248 + 247.
        assert_eq!(Solution::three_sum_closest(nums, 10_000), 744);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sorting + two pointers: sort, fix the first index `i`, and scan `lo`/`hi` over the
rest. Each sum is a candidate: keep it if it is closer to `target` than the best so far. Then
move like Two Sum II: a sum below the target can only get closer by increasing (move `lo`
right), a sum above it only by decreasing (move `hi` left). A sum equal to the target is the
best possible answer, so return it immediately.

**Complexity:** `O(n^2)` time, `O(1)` extra space.

**Rust concepts:**
- `(sum - target).abs()` measures the distance; all values are small so `i32` cannot overflow.
- `return` inside a nested loop leaves the whole function, which is the cleanest early exit.

```rust
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut best = nums[0] + nums[1] + nums[2]; // len >= 3 by the constraints
        for (i, &first) in nums.iter().enumerate().take(n - 2) {
            let (mut lo, mut hi) = (i + 1, n - 1);
            while lo < hi {
                let sum = first + nums[lo] + nums[hi];
                if (sum - target).abs() < (best - target).abs() {
                    best = sum;
                }
                match sum.cmp(&target) {
                    Ordering::Less => lo += 1,
                    Ordering::Greater => hi -= 1,
                    Ordering::Equal => return sum,
                }
            }
        }
        best
    }
}
```

Full tested solution: [`src/solutions/p0708_3sum_closest.rs`](../src/solutions/p0708_3sum_closest.rs)

</details>
