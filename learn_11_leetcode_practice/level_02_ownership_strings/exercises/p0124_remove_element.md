# Problem 0124: Remove Element

- **Difficulty:** Easy
- **Topics:** in-place compaction through `&mut Vec<i32>`, read/write indices, returning a length
- **Source:** [LeetCode 27. Remove Element](https://leetcode.com/problems/remove-element/)

## Problem

Given a vector `nums` and a value `val`, remove every occurrence of `val` **in place** and return
`k`, the number of elements that are not equal to `val`.

After the call, the first `k` elements of `nums` must be exactly the elements different from
`val` (in any order). What is stored at positions `k` and later does not matter.

Implement `Solution::remove_element(nums: &mut Vec<i32>, val: i32) -> i32`.

## Examples

```text
Input:  nums = [3, 2, 2, 3], val = 3
Output: 2, nums starts with [2, 2]

Input:  nums = [0, 1, 2, 2, 3, 0, 4, 2], val = 2
Output: 5, nums starts with [0, 1, 3, 0, 4] (any order of these five values is accepted)
```

## Constraints

- `0 <= nums.len() <= 100`
- `0 <= nums[i] <= 50`
- `0 <= val <= 100`

## Hints

1. Keep a `write` index. Every element that should stay is copied to `nums[write]`.
2. `i32` is `Copy`, so `nums[write] = nums[read]` simply copies the value.

## Starter code

Write your code in [`src/practice/p0124_remove_element.rs`](../src/practice/p0124_remove_element.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0124
```

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
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

    /// Runs the function and returns `k` and the sorted first `k` elements.
    fn run(mut nums: Vec<i32>, val: i32) -> (i32, Vec<i32>) {
        let k = Solution::remove_element(&mut nums, val);
        let mut kept = nums[..k as usize].to_vec();
        kept.sort();
        (k, kept)
    }

    #[test]
    fn example_remove_threes() {
        assert_eq!(run(vec![3, 2, 2, 3], 3), (2, vec![2, 2]));
    }

    #[test]
    fn example_remove_twos() {
        assert_eq!(run(vec![0, 1, 2, 2, 3, 0, 4, 2], 2), (5, vec![0, 0, 1, 3, 4]));
    }

    #[test]
    fn empty_input() {
        assert_eq!(run(vec![], 1), (0, vec![]));
    }

    #[test]
    fn value_not_present_and_all_removed() {
        assert_eq!(run(vec![1, 2, 3], 9), (3, vec![1, 2, 3]));
        assert_eq!(run(vec![7, 7, 7], 7), (0, vec![]));
    }

    #[test]
    fn largest_input() {
        let nums: Vec<i32> = (0..100).map(|i| i % 5).collect();
        let (k, kept) = run(nums, 0);
        assert_eq!(k, 80);
        assert!(kept.iter().all(|&x| x != 0));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** scan with `read`; each kept value is copied to position `write`, which then advances.
At the end `write` is the number of kept values, and they occupy `nums[..write]` in their
original order.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- Copying within one vector by index is fine for `Copy` types; for `String` elements you would
  use `swap` instead, because a plain assignment would try to move out of the vector.
- Idiomatic Rust for "remove all matching elements" is `nums.retain(|&x| x != val)`, which also
  shrinks the vector; the manual version shows what `retain` does internally.
- `k as i32` converts the `usize` count to LeetCode's return type (safe: `k <= 100`).

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut write = 0;
        for read in 0..nums.len() {
            if nums[read] != val {
                nums[write] = nums[read];
                write += 1;
            }
        }
        write as i32
    }
}
```

Full tested solution: [`src/solutions/p0124_remove_element.rs`](../src/solutions/p0124_remove_element.rs)

</details>
