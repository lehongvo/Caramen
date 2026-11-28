# Problem 0489: Rearrange Array Elements by Sign

- **Difficulty:** Medium
- **Topics:** stable `partition`, `zip` of the two halves, `flat_map` into arrays `[p, n]`
- **Source:** [LeetCode 2149. Rearrange Array Elements by Sign](https://leetcode.com/problems/rearrange-array-elements-by-sign/)

## Problem

`nums` has an even length and contains as many positive numbers as negative numbers (no zeros).
Rearrange it so that:

1. positive and negative numbers alternate, starting with a positive number;
2. the positive numbers keep their relative order from the input, and so do the negative ones.

Return the rearranged array.

Implement `Solution::rearrange_array(nums: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  nums = [3, 1, -2, -5, 2, -4]
Output: [3, -2, 1, -5, 2, -4]
Explanation: positives in order: 3, 1, 2; negatives in order: -2, -5, -4.

Input:  nums = [-1, 1]
Output: [1, -1]
```

## Constraints

- `2 <= nums.len() <= 2 * 10^5`, `nums.len()` is even
- `1 <= |nums[i]| <= 10^5`
- `nums` has the same number of positive and negative integers.

## Hints

1. `partition` keeps the input order inside each group.
2. `positives.into_iter().zip(negatives)` gives the pairs `(p, n)` in order; `flat_map(|(p, n)| [p, n])`
   lays each pair out as two consecutive elements.

## Starter code

Write your code in [`src/practice/p0489_rearrange_array_elements_by_sign.rs`](../src/practice/p0489_rearrange_array_elements_by_sign.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0489
```

```rust
pub struct Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
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
    fn example_six_numbers() {
        assert_eq!(
            Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
    }

    #[test]
    fn example_two_numbers() {
        assert_eq!(Solution::rearrange_array(vec![-1, 1]), vec![1, -1]);
    }

    #[test]
    fn all_negatives_first() {
        assert_eq!(
            Solution::rearrange_array(vec![-3, -2, -1, 1, 2, 3]),
            vec![1, -3, 2, -2, 3, -1]
        );
    }

    #[test]
    fn large_input_alternates() {
        let nums: Vec<i32> = (1..=100_000).chain((1..=100_000).map(|x| -x)).collect();
        let out = Solution::rearrange_array(nums);
        assert_eq!(out.len(), 200_000);
        assert!(out.iter().step_by(2).all(|&x| x > 0));
        assert!(out.iter().skip(1).step_by(2).all(|&x| x < 0));
        assert_eq!(&out[..4], &[1, -1, 2, -2]);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split the numbers by sign (stable), then weave the two lists together pair by pair.

**Complexity:** `O(n)` time and `O(n)` space.

**Rust concepts:**
- `partition` into `(Vec<i32>, Vec<i32>)` directly from an iterator of `&i32`: `Vec<i32>`
  implements `Extend<&i32>` for `Copy` elements.
- Arrays implement `IntoIterator` by value (since edition 2021), so `flat_map(|(p, n)| [p, n])`
  yields `p` then `n` without allocating.
- `zip` pairs the two groups; because they have equal lengths, nothing is dropped.

```rust
pub struct Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let (positives, negatives): (Vec<i32>, Vec<i32>) = nums.iter().partition(|&&x| x > 0);
        positives
            .into_iter()
            .zip(negatives)
            .flat_map(|(p, n)| [p, n])
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0489_rearrange_array_elements_by_sign.rs`](../src/solutions/p0489_rearrange_array_elements_by_sign.rs)

</details>
