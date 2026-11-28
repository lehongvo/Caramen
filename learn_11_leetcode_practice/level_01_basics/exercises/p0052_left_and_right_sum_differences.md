# Problem 0052: Left and Right Sum Differences

- **Difficulty:** Easy
- **Topics:** prefix sums, total minus prefix, `abs`, building a `Vec`
- **Source:** [LeetCode 2574. Left and Right Sum Differences](https://leetcode.com/problems/left-and-right-sum-differences/)

## Problem

Given a vector `nums` of length `n`, compute for every index `i`:

- `left[i]`: the sum of the elements **before** index `i` (`0` for `i = 0`);
- `right[i]`: the sum of the elements **after** index `i` (`0` for `i = n - 1`).

Return the vector `answer` where `answer[i] = |left[i] - right[i]|`.

Implement `Solution::left_right_difference(nums: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  nums = [10, 4, 8, 3]
Output: [15, 1, 11, 22]
Explanation: left = [0, 10, 14, 22], right = [15, 11, 3, 0].

Input:  nums = [1]
Output: [0]
```

## Constraints

- `1 <= nums.len() <= 1000`
- `1 <= nums[i] <= 10^5`

## Hints

1. Recomputing both sums for every index is `O(n^2)`. Can you get `right[i]` from the total?
2. `right[i] = total - left[i] - nums[i]`.

## Starter code

Write your code in [`src/practice/p0052_left_and_right_sum_differences.rs`](../src/practice/p0052_left_and_right_sum_differences.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0052
```

```rust
pub struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
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
    fn example_four_elements() {
        assert_eq!(
            Solution::left_right_difference(vec![10, 4, 8, 3]),
            vec![15, 1, 11, 22]
        );
    }

    #[test]
    fn example_single_element() {
        assert_eq!(Solution::left_right_difference(vec![1]), vec![0]);
    }

    #[test]
    fn symmetric_input() {
        assert_eq!(
            Solution::left_right_difference(vec![1, 2, 3, 2, 1]),
            vec![8, 5, 0, 5, 8]
        );
    }

    #[test]
    fn largest_values() {
        let answer = Solution::left_right_difference(vec![100_000; 1000]);
        assert_eq!(answer[0], 99_900_000);
        assert_eq!(answer[999], 99_900_000);
        // Index 500 has 500 elements on the left and 499 on the right.
        assert_eq!(answer[500], 100_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** compute `total` once. Walk left to right keeping `left` (the sum of the elements
already passed). For the current element `x`, `right = total - left - x`; push `|left - right|`
and then add `x` to `left`. All sums are at most `10^8`, so `i32` is enough.

**Complexity:** `O(n)` time, `O(n)` space for the output.

**Rust concepts:**
- `i32::abs` returns the absolute value (it would overflow only for `i32::MIN`, which cannot
  occur here). `abs_diff` is the overflow-proof alternative that returns a `u32`.
- The order of statements inside the loop matters: use `left` **before** adding the current element.

```rust
pub struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut total = 0;
        for &x in &nums {
            total += x;
        }
        let mut left = 0;
        let mut answer = Vec::with_capacity(nums.len());
        for &x in &nums {
            let right = total - left - x;
            answer.push((left - right).abs());
            left += x;
        }
        answer
    }
}
```

Full tested solution: [`src/solutions/p0052_left_and_right_sum_differences.rs`](../src/solutions/p0052_left_and_right_sum_differences.rs)

</details>
