# Problem 0085: Separate the Digits in an Array

- **Difficulty:** Easy
- **Topics:** collecting digits in reverse, `Vec::reverse`, slices of a `Vec`, `extend_from_slice`
- **Source:** [LeetCode 2553. Separate the Digits in an Array](https://leetcode.com/problems/separate-the-digits-in-an-array/)

## Problem

Given a vector of positive integers `nums`, return a vector containing all their decimal digits,
in order: the digits of `nums[0]` from left to right, then those of `nums[1]`, and so on.

Implement `Solution::separate_digits(nums: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  nums = [13, 25, 83, 77]
Output: [1, 3, 2, 5, 8, 3, 7, 7]

Input:  nums = [7, 1, 3, 9]
Output: [7, 1, 3, 9]
```

## Constraints

- `1 <= nums.len() <= 1000`
- `1 <= nums[i] <= 10^5`

## Hints

1. The digit loop produces digits from right to left; collect them and reverse them.
2. Reuse one small buffer for each number, then append it to the result.

## Starter code

Write your code in [`src/practice/p0085_separate_the_digits_in_an_array.rs`](../src/practice/p0085_separate_the_digits_in_an_array.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0085
```

```rust
pub struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
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
    fn example_two_digit_numbers() {
        assert_eq!(
            Solution::separate_digits(vec![13, 25, 83, 77]),
            vec![1, 3, 2, 5, 8, 3, 7, 7]
        );
    }

    #[test]
    fn example_single_digits() {
        assert_eq!(
            Solution::separate_digits(vec![7, 1, 3, 9]),
            vec![7, 1, 3, 9]
        );
    }

    #[test]
    fn zeros_inside_numbers() {
        assert_eq!(
            Solution::separate_digits(vec![100_000, 101]),
            vec![1, 0, 0, 0, 0, 0, 1, 0, 1]
        );
    }

    #[test]
    fn largest_input() {
        let result = Solution::separate_digits(vec![12_345; 1000]);
        assert_eq!(result.len(), 5000);
        assert_eq!(&result[..5], &[1, 2, 3, 4, 5]);
        assert_eq!(&result[4995..], &[1, 2, 3, 4, 5]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for each number, push its digits (right to left) into a temporary vector, reverse it,
and append it to the answer.

**Complexity:** `O(total number of digits)` time and space.

**Rust concepts:**
- `buffer.clear()` empties a vector but keeps its allocation, so it can be reused without
  allocating again.
- `buffer.reverse()` reverses in place; `result.extend_from_slice(&buffer)` copies the digits into
  the result.
- An alternative that avoids reversing: find the largest power of ten `<= x` and divide by
  decreasing powers.

```rust
pub struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut buffer = Vec::new();
        for &x in &nums {
            buffer.clear();
            let mut rest = x;
            while rest > 0 {
                buffer.push(rest % 10);
                rest /= 10;
            }
            buffer.reverse();
            result.extend_from_slice(&buffer);
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0085_separate_the_digits_in_an_array.rs`](../src/solutions/p0085_separate_the_digits_in_an_array.rs)

</details>
