# Problem 0123: Move Zeroes

- **Difficulty:** Easy
- **Topics:** in-place modification through `&mut Vec<i32>`, read/write indices, `slice::swap`
- **Source:** [LeetCode 283. Move Zeroes](https://leetcode.com/problems/move-zeroes/)

## Problem

Given a vector `nums`, move every `0` to the end while keeping the **relative order** of the
non-zero elements. Work in place through the mutable reference: do not make a copy of the vector.

Implement `Solution::move_zeroes(nums: &mut Vec<i32>)`.

## Examples

```text
Input:  nums = [0, 1, 0, 3, 12]
Output: nums = [1, 3, 12, 0, 0]

Input:  nums = [0]
Output: nums = [0]
```

## Constraints

- `1 <= nums.len() <= 10^4`
- `-2^31 <= nums[i] <= 2^31 - 1`

## Hints

1. Use a `write` index: the slot where the next non-zero value belongs.
2. Scan with a `read` index; when `nums[read] != 0`, swap it into `nums[write]` and advance
   `write`. Zeros end up behind automatically.

## Starter code

Write your code in [`src/practice/p0123_move_zeroes.rs`](../src/practice/p0123_move_zeroes.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0123
```

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough.
    #[allow(clippy::ptr_arg)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
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

    fn moved(mut nums: Vec<i32>) -> Vec<i32> {
        Solution::move_zeroes(&mut nums);
        nums
    }

    #[test]
    fn example_mixed() {
        assert_eq!(moved(vec![0, 1, 0, 3, 12]), vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn example_single_zero() {
        assert_eq!(moved(vec![0]), vec![0]);
    }

    #[test]
    fn no_zeros_keeps_everything() {
        assert_eq!(moved(vec![4, -2, 7]), vec![4, -2, 7]);
    }

    #[test]
    fn zeros_already_at_the_end_and_extremes() {
        assert_eq!(
            moved(vec![i32::MIN, 0, i32::MAX, 0]),
            vec![i32::MIN, i32::MAX, 0, 0]
        );
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..10_000)
            .map(|i| if i % 2 == 0 { 0 } else { i })
            .collect();
        let result = moved(nums);
        let expected: Vec<i32> = (0..10_000)
            .filter(|i| i % 2 == 1)
            .chain(std::iter::repeat_n(0, 5_000))
            .collect();
        assert_eq!(result, expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** everything before `write` is the non-zero prefix in original order. When `read` finds a
non-zero value, swapping it with `nums[write]` appends it to that prefix and moves a zero (or the
value itself, when `read == write`) further right.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- Mutating through `&mut Vec<i32>`: the caller's vector changes, nothing is returned.
- Two indices into the same vector are just `usize`s, so there is no borrow conflict; the borrow
  checker would object only if we held two `&mut` references to elements at once.
- The signature is fixed by LeetCode; Clippy's `ptr_arg` (prefer `&mut [i32]`) is allowed there.

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough.
    #[allow(clippy::ptr_arg)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut write = 0;
        for read in 0..nums.len() {
            if nums[read] != 0 {
                nums.swap(write, read);
                write += 1;
            }
        }
    }
}
```

Full tested solution: [`src/solutions/p0123_move_zeroes.rs`](../src/solutions/p0123_move_zeroes.rs)

</details>
