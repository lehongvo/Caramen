# Problem 0128: Apply Operations to an Array

- **Difficulty:** Easy
- **Topics:** owned `mut` vector, sequential in-place updates, moving zeros with a write index
- **Source:** [LeetCode 2460. Apply Operations to an Array](https://leetcode.com/problems/apply-operations-to-an-array/)

## Problem

You are given a vector `nums` of `n` non-negative integers. Perform these two steps and return
the resulting vector:

1. For `i = 0, 1, ..., n - 2`, **in this order**: if `nums[i] == nums[i + 1]`, double `nums[i]`
   and set `nums[i + 1]` to `0`. Each operation sees the changes made by the previous ones.
2. Move all zeros to the end, keeping the order of the non-zero values.

Implement `Solution::apply_operations(nums: Vec<i32>) -> Vec<i32>`, reusing the owned vector.

## Examples

```text
Input:  nums = [1, 2, 2, 1, 1, 0]
Output: [1, 4, 2, 0, 0, 0]
Explanation: i = 1 gives [1, 4, 0, 1, 1, 0]; i = 3 gives [1, 4, 0, 2, 0, 0];
             i = 4 compares 0 with 0 (doubling 0 changes nothing). Then zeros move to the end.

Input:  nums = [0, 1]
Output: [1, 0]
```

## Constraints

- `2 <= nums.len() <= 2000`
- `0 <= nums[i] <= 1000`

## Hints

1. Step 1 is a plain index loop over `0..n - 1` that reads and writes `nums[i]` and `nums[i + 1]`.
2. Step 2 is "Move Zeroes": a write index plus `swap`.

## Starter code

Write your code in [`src/practice/p0128_apply_operations_to_an_array.rs`](../src/practice/p0128_apply_operations_to_an_array.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0128
```

```rust
pub struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
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
    fn example_mixed() {
        assert_eq!(
            Solution::apply_operations(vec![1, 2, 2, 1, 1, 0]),
            vec![1, 4, 2, 0, 0, 0]
        );
    }

    #[test]
    fn example_two_elements() {
        assert_eq!(Solution::apply_operations(vec![0, 1]), vec![1, 0]);
    }

    #[test]
    fn chain_of_equal_values_is_processed_in_order() {
        // [2,2,2,2] -> i=0: [4,0,2,2] -> i=2: [4,0,4,0]
        assert_eq!(
            Solution::apply_operations(vec![2, 2, 2, 2]),
            vec![4, 4, 0, 0]
        );
        // [3,3,3] -> i=0: [6,0,3] -> nothing else
        assert_eq!(Solution::apply_operations(vec![3, 3, 3]), vec![6, 3, 0]);
    }

    #[test]
    fn no_equal_neighbours() {
        assert_eq!(Solution::apply_operations(vec![5, 0, 7]), vec![5, 7, 0]);
    }

    #[test]
    fn largest_input() {
        let result = Solution::apply_operations(vec![1000; 2000]);
        assert!(result[..1000].iter().all(|&x| x == 2000));
        assert!(result[1000..].iter().all(|&x| x == 0));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** simulate step 1 directly; because it runs left to right on the same vector, later
comparisons automatically see the zeros written by earlier ones. Then compact the non-zero values
to the front with a write index, swapping zeros behind them.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `mut nums: Vec<i32>` lets us modify the owned input and return it.
- `nums.len() - 1` is safe because the constraints guarantee at least two elements; for possibly
  empty input you would write `nums.len().saturating_sub(1)`.
- Index-based loops are natural here: each step needs two positions of the same vector.

```rust
pub struct Solution;

impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        let mut write = 0;
        for read in 0..nums.len() {
            if nums[read] != 0 {
                nums.swap(write, read);
                write += 1;
            }
        }
        nums
    }
}
```

Full tested solution: [`src/solutions/p0128_apply_operations_to_an_array.rs`](../src/solutions/p0128_apply_operations_to_an_array.rs)

</details>
