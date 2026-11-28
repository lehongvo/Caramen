# Problem 0303: Create Target Array in the Given Order

- **Difficulty:** Easy
- **Topics:** `Vec::insert`, `zip`, `as usize` casts
- **Source:** [LeetCode 1389. Create Target Array in the Given Order](https://leetcode.com/problems/create-target-array-in-the-given-order/)

## Problem

You receive two vectors of the same length, `nums` and `index`. Start with an empty vector
`target` and, for `i = 0, 1, 2, ...` in order, insert the value `nums[i]` at position
`index[i]` of `target`. Inserting at a position moves the element already there (and every
element after it) one place to the right. Return `target` after all insertions.

Every insertion is guaranteed to be valid: at step `i`, `index[i]` is at most the current
length of `target`.

Implement `Solution::create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32>`.

## Examples

```text
Input:  nums = [0, 1, 2, 3, 4], index = [0, 1, 2, 2, 1]
Output: [0, 4, 1, 3, 2]
Steps:  [0] -> [0, 1] -> [0, 1, 2] -> [0, 1, 3, 2] -> [0, 4, 1, 3, 2]

Input:  nums = [1, 2, 3, 4, 0], index = [0, 1, 2, 3, 0]
Output: [0, 1, 2, 3, 4]

Input:  nums = [1], index = [0]
Output: [1]
```

## Constraints

- `1 <= nums.len() == index.len() <= 100`
- `0 <= nums[i] <= 100`
- `0 <= index[i] <= i`

## Hints

1. `Vec::insert(position, value)` does exactly what one step describes.
2. Walk both vectors together with `nums.into_iter().zip(index)`.

## Starter code

Write your code in [`src/practice/p0303_create_target_array_in_the_given_order.rs`](../src/practice/p0303_create_target_array_in_the_given_order.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0303
```

```rust
pub struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
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
            Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::create_target_array(vec![1], vec![0]), vec![1]);
    }

    #[test]
    fn always_insert_at_front_reverses() {
        let nums: Vec<i32> = (0..100).collect();
        let expected: Vec<i32> = (0..100).rev().collect();
        assert_eq!(Solution::create_target_array(nums, vec![0; 100]), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** simulate the description literally. `Vec::insert` shifts the tail of the vector one
place to the right and writes the new value into the gap.

**Complexity:** `O(n^2)` time in the worst case (each insert may shift up to `n` elements),
`O(n)` space for the result.

**Rust concepts:**
- `Vec::with_capacity(n)` allocates once, because we know the final length.
- `insert(i, x)` panics if `i > len`; the constraints (`index[i] <= i`) guarantee it never
  happens here.
- LeetCode passes indices as `i32`, but `Vec` positions are `usize`, so cast with `as usize`
  (safe because the values are non-negative).

```rust
pub struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut target = Vec::with_capacity(nums.len());
        for (value, position) in nums.into_iter().zip(index) {
            // The constraints guarantee 0 <= position <= target.len().
            target.insert(position as usize, value);
        }
        target
    }
}
```

Full tested solution: [`src/solutions/p0303_create_target_array_in_the_given_order.rs`](../src/solutions/p0303_create_target_array_in_the_given_order.rs)

</details>
