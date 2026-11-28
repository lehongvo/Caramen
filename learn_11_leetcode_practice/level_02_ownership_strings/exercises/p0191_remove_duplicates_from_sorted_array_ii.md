# Problem 0191: Remove Duplicates from Sorted Array II

- **Difficulty:** Medium
- **Topics:** in-place compaction with a write index, comparing with `nums[write - 2]`
- **Source:** [LeetCode 80. Remove Duplicates from Sorted Array II](https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/)

## Problem

`nums` is sorted in non-decreasing order. Remove extra duplicates **in place** so that every
distinct value appears **at most twice**, keeping the sorted order, and return `k`, the number of
elements kept. After the call, `nums[..k]` must hold the kept elements; what is stored after
position `k` does not matter. Use `O(1)` extra memory.

Implement `Solution::remove_duplicates(nums: &mut Vec<i32>) -> i32`.

## Examples

```text
Input:  nums = [1, 1, 1, 2, 2, 3]
Output: 5, nums starts with [1, 1, 2, 2, 3]

Input:  nums = [0, 0, 1, 1, 1, 1, 2, 3, 3]
Output: 7, nums starts with [0, 0, 1, 1, 2, 3, 3]
```

## Constraints

- `1 <= nums.len() <= 3 * 10^4`
- `-10^4 <= nums[i] <= 10^4`
- `nums` is sorted in non-decreasing order.

## Hints

1. The first two elements can always be kept.
2. A later element may be kept only if it differs from the element **two places before** the write
   position, `nums[write - 2]` (because of the sorting, that is the only way a third copy could
   appear).

## Starter code

Write your code in [`src/practice/p0191_remove_duplicates_from_sorted_array_ii.rs`](../src/practice/p0191_remove_duplicates_from_sorted_array_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0191
```

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
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

    fn run(mut nums: Vec<i32>) -> Vec<i32> {
        let k = Solution::remove_duplicates(&mut nums) as usize;
        nums.truncate(k);
        nums
    }

    #[test]
    fn example_short() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        assert_eq!(&nums[..5], &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn example_long() {
        assert_eq!(
            run(vec![0, 0, 1, 1, 1, 1, 2, 3, 3]),
            vec![0, 0, 1, 1, 2, 3, 3]
        );
    }

    #[test]
    fn tiny_inputs() {
        assert_eq!(run(vec![5]), vec![5]);
        assert_eq!(run(vec![5, 5]), vec![5, 5]);
        assert_eq!(run(vec![5, 5, 5]), vec![5, 5]);
    }

    #[test]
    fn all_distinct() {
        assert_eq!(run(vec![-3, -1, 0, 4]), vec![-3, -1, 0, 4]);
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..30_000).map(|i| i / 10).collect();
        let expected: Vec<i32> = (0..3_000).flat_map(|v| [v, v]).collect();
        assert_eq!(run(nums), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** generalize the "at most once" version: keep `nums[read]` when fewer than two elements
have been written, or when it is different from `nums[write - 2]`. Since the kept prefix is
sorted, equality with the element two places back means two copies are already kept.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `write < 2 || nums[read] != nums[write - 2]`: the short-circuiting `||` makes sure `write - 2`
  is only computed when it cannot underflow.
- The same pattern with `write - 1` solves problem 0125, and with `write - k` keeps at most `k`
  copies.
- The signature is LeetCode's; Clippy's `ptr_arg` is allowed on it.

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write = 0;
        for read in 0..nums.len() {
            if write < 2 || nums[read] != nums[write - 2] {
                nums[write] = nums[read];
                write += 1;
            }
        }
        write as i32
    }
}
```

Full tested solution: [`src/solutions/p0191_remove_duplicates_from_sorted_array_ii.rs`](../src/solutions/p0191_remove_duplicates_from_sorted_array_ii.rs)

</details>
