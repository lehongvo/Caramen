# Problem 0040: Running Sum of 1d Array

- **Difficulty:** Easy
- **Topics:** indexing a `Vec`, updating elements in place, shadowing a parameter with `let mut`
- **Source:** [LeetCode 1480. Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)

## Problem

Given a vector `nums`, return a vector of the same length whose element `i` is the sum of
`nums[0] + nums[1] + ... + nums[i]` (the **running sum**, also called prefix sum).

Implement `Solution::running_sum(nums: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  nums = [1, 2, 3, 4]
Output: [1, 3, 6, 10]
Explanation: [1, 1+2, 1+2+3, 1+2+3+4].

Input:  nums = [1, 1, 1, 1, 1]
Output: [1, 2, 3, 4, 5]

Input:  nums = [3, 1, 2, 10, 1]
Output: [3, 4, 6, 16, 17]
```

## Constraints

- `1 <= nums.len() <= 1000`
- `-10^6 <= nums[i] <= 10^6`

## Hints

1. The running sum at `i` is the running sum at `i - 1` plus `nums[i]`.
2. You own `nums` (it was passed by value), so you may modify it and return it. Parameters are
   immutable by default; `let mut nums = nums;` makes a mutable binding.

## Starter code

Write your code in [`src/practice/p0040_running_sum_of_1d_array.rs`](../src/practice/p0040_running_sum_of_1d_array.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0040
```

```rust
pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
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
    fn example_increasing() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn example_all_ones() {
        assert_eq!(
            Solution::running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn example_mixed() {
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            vec![3, 4, 6, 16, 17]
        );
    }

    #[test]
    fn single_element_and_negatives() {
        assert_eq!(Solution::running_sum(vec![-5]), vec![-5]);
        assert_eq!(Solution::running_sum(vec![5, -5, 5, -5]), vec![5, 0, 5, 0]);
    }

    #[test]
    fn largest_values() {
        let result = Solution::running_sum(vec![1_000_000; 1000]);
        assert_eq!(result.len(), 1000);
        assert_eq!(result[999], 1_000_000_000);
        assert_eq!(result[499], 500_000_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** walk from index 1 to the end and add the previous (already accumulated) element to
the current one: `nums[i] += nums[i - 1]`. The vector becomes its own running sum, with no extra
allocation. The largest absolute value is `1000 * 10^6 = 10^9`, which fits in an `i32`.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `let mut nums = nums;` **shadows** the immutable parameter with a mutable binding that owns
  the same vector; nothing is copied. Writing `mut nums: Vec<i32>` in the signature is equivalent.
- Indexing `nums[i]` on a `Vec` is bounds-checked: an out-of-range index panics instead of
  reading random memory. The range `1..nums.len()` keeps every index valid.
- `nums[i] += nums[i - 1]` reads one element and writes another; both are `i32` (`Copy`), so the
  borrow checker is happy.

```rust
pub struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}
```

Full tested solution: [`src/solutions/p0040_running_sum_of_1d_array.rs`](../src/solutions/p0040_running_sum_of_1d_array.rs)

</details>
