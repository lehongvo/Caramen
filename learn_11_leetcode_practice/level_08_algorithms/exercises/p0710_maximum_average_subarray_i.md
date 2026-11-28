# Problem 0710: Maximum Average Subarray I

- **Difficulty:** Easy
- **Topics:** fixed-size sliding window, `zip`, `f64` conversion
- **Source:** [LeetCode 643. Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/)

## Problem

You are given a vector `nums` of `n` integers and an integer `k`. Among all contiguous
subarrays of length **exactly** `k`, find the one with the largest average and return that
average as an `f64`. Answers within `10^-5` of the exact value are accepted.

Implement `Solution::find_max_average(nums: Vec<i32>, k: i32) -> f64`.

## Examples

```text
Input:  nums = [1, 12, -5, -6, 50, 3], k = 4
Output: 12.75         ((12 - 5 - 6 + 50) / 4)

Input:  nums = [5], k = 1
Output: 5.0
```

## Constraints

- `1 <= k <= n <= 10^5`
- `-10^4 <= nums[i] <= 10^4`

## Hints

1. All windows have the same length `k`, so the largest average is the largest **sum**.
2. When the window moves one step right, one value enters and one value leaves. Update the sum
   instead of recomputing it.

## Starter code

Write your code in [`src/practice/p0710_maximum_average_subarray_i.rs`](../src/practice/p0710_maximum_average_subarray_i.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0710
```

```rust
pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
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

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
    }

    #[test]
    fn example_window_of_four() {
        assert_close(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75,
        );
    }

    #[test]
    fn example_single_value() {
        assert_close(Solution::find_max_average(vec![5], 1), 5.0);
    }

    #[test]
    fn all_negative() {
        assert_close(Solution::find_max_average(vec![-1, -2, -3], 2), -1.5);
    }

    #[test]
    fn window_is_whole_array() {
        assert_close(Solution::find_max_average(vec![0, 4, 0, 3, 2], 5), 1.8);
    }

    #[test]
    fn best_window_at_the_end() {
        assert_close(Solution::find_max_average(vec![-5, -5, 1, 9, 9], 2), 9.0);
    }

    #[test]
    fn largest_input() {
        assert_close(
            Solution::find_max_average(vec![10_000; 100_000], 100_000),
            10_000.0,
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** fixed-size sliding window: compute the sum of the first `k` values. Then slide:
the value `nums[i]` enters while `nums[i - k]` leaves, so the new sum is
`sum + nums[i] - nums[i - k]`. Keep the largest sum and divide by `k` once at the end (dividing
only once also avoids accumulating floating-point error).

**Complexity:** `O(n)` time, `O(1)` space.

**Rust concepts:**
- `nums[k..].iter().zip(&nums)` pairs each entering value with the value leaving the window,
  with no index arithmetic at all: the second iterator simply lags `k` positions behind.
- The largest sum is `10^4 * 10^5 = 10^9`, which fits in `i32`. The final `as f64` conversion is
  exact for every `i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum: i32 = nums[..k].iter().sum();
        let mut best = sum;
        for (entering, leaving) in nums[k..].iter().zip(&nums) {
            sum += entering - leaving;
            best = best.max(sum);
        }
        f64::from(best) / k as f64
    }
}
```

Full tested solution: [`src/solutions/p0710_maximum_average_subarray_i.rs`](../src/solutions/p0710_maximum_average_subarray_i.rs)

</details>
