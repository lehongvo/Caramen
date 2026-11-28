# Problem 0321: How Many Numbers Are Smaller Than the Current Number

- **Difficulty:** Easy
- **Topics:** sorted copy with `clone` + `sort_unstable`, `partition_point`, `map` + `collect`
- **Source:** [LeetCode 1365. How Many Numbers Are Smaller Than the Current Number](https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/)

## Problem

Given a vector `nums`, build a vector `answer` of the same length where `answer[i]` is the
number of elements of `nums` that are **strictly smaller** than `nums[i]`.

Implement `Solution::smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  nums = [8, 1, 2, 2, 3]
Output: [4, 0, 1, 1, 3]
Explanation: four numbers are smaller than 8; none is smaller than 1; only 1 is smaller
             than 2 (twice); 1, 2, 2 are smaller than 3.

Input:  nums = [6, 5, 4, 8]
Output: [2, 1, 0, 3]

Input:  nums = [7, 7, 7, 7]
Output: [0, 0, 0, 0]
```

## Constraints

- `2 <= nums.len() <= 500`
- `0 <= nums[i] <= 100`

## Hints

1. In a **sorted** copy, the number of elements smaller than `x` is the index of the first
   element that is `>= x`.
2. `sorted.partition_point(|&y| y < x)` returns exactly that index.

## Starter code

Write your code in [`src/practice/p0321_how_many_numbers_are_smaller_than_the_current_number.rs`](../src/practice/p0321_how_many_numbers_are_smaller_than_the_current_number.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0321
```

```rust
pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
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
    fn example_one() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
    }

    #[test]
    fn example_three_all_equal() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }

    #[test]
    fn extremes() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![100, 0]),
            vec![1, 0]
        );
    }

    #[test]
    fn large_descending() {
        let nums: Vec<i32> = (0..500).rev().map(|i| i / 5).collect();
        let result = Solution::smaller_numbers_than_current(nums);
        assert_eq!(result[0], 495);
        assert_eq!(result[499], 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sort a copy of the input once. For every original value `x`, a binary search in the
sorted copy finds how many values are `< x`. The original vector keeps the output order.

**Complexity:** `O(n log n)` time, `O(n)` space for the sorted copy.

**Rust concepts:**
- `nums.clone()` is needed because we want both the original order and the sorted order.
- `partition_point` handles duplicates correctly: it finds the **first** element that is not
  smaller, whereas `binary_search` could land on any of several equal elements.
- A counting solution (`counts[v]` and prefix sums over `0..=100`) is `O(n + 100)`; both are fine
  for these limits.

```rust
pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        nums.iter()
            .map(|&x| sorted.partition_point(|&y| y < x) as i32)
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0321_how_many_numbers_are_smaller_than_the_current_number.rs`](../src/solutions/p0321_how_many_numbers_are_smaller_than_the_current_number.rs)

</details>
