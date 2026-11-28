# Problem 0192: Next Permutation

- **Difficulty:** Medium
- **Topics:** in-place `swap` + `reverse` of a sub-slice, searching from the back with `rev().find`
- **Source:** [LeetCode 31. Next Permutation](https://leetcode.com/problems/next-permutation/)

## Problem

The permutations of a list of numbers can be sorted in lexicographic (dictionary) order. For
`[1, 2, 3]` that order is `[1,2,3]`, `[1,3,2]`, `[2,1,3]`, `[2,3,1]`, `[3,1,2]`, `[3,2,1]`.

Rearrange `nums` **in place** into the next permutation in that order. If `nums` is already the
last (largest) permutation, rearrange it into the first one (sorted in ascending order). Use only
`O(1)` extra memory.

Implement `Solution::next_permutation(nums: &mut Vec<i32>)`.

## Examples

```text
Input:  nums = [1, 2, 3]
Output: nums = [1, 3, 2]

Input:  nums = [3, 2, 1]
Output: nums = [1, 2, 3]

Input:  nums = [1, 1, 5]
Output: nums = [1, 5, 1]
```

## Constraints

- `1 <= nums.len() <= 100`
- `0 <= nums[i] <= 100`

## Hints

1. Find the longest non-increasing suffix. The element just before it, the *pivot* at index `i`,
   is the leftmost position that must change.
2. Swap the pivot with the **rightmost** element of the suffix that is greater than it.
3. The suffix is still non-increasing; reverse it to make it as small as possible.
   If there is no pivot, the whole vector is non-increasing: just reverse it.

## Starter code

Write your code in [`src/practice/p0192_next_permutation.rs`](../src/practice/p0192_next_permutation.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0192
```

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
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

    fn next(mut nums: Vec<i32>) -> Vec<i32> {
        Solution::next_permutation(&mut nums);
        nums
    }

    #[test]
    fn example_ascending() {
        assert_eq!(next(vec![1, 2, 3]), vec![1, 3, 2]);
    }

    #[test]
    fn example_last_wraps_around() {
        assert_eq!(next(vec![3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn example_duplicates() {
        assert_eq!(next(vec![1, 1, 5]), vec![1, 5, 1]);
    }

    #[test]
    fn single_and_equal_elements() {
        assert_eq!(next(vec![7]), vec![7]);
        assert_eq!(next(vec![2, 2, 2]), vec![2, 2, 2]);
    }

    #[test]
    fn walks_through_all_permutations_in_order() {
        let mut nums = vec![1, 2, 3, 4];
        let mut seen = vec![nums.clone()];
        for _ in 0..23 {
            Solution::next_permutation(&mut nums);
            seen.push(nums.clone());
        }
        let mut sorted = seen.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(seen, sorted); // strictly increasing, 24 distinct permutations
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4]);
    }

    #[test]
    fn longest_input() {
        let mut nums: Vec<i32> = (0..100).rev().collect();
        nums.swap(98, 99); // [99, 98, ..., 2, 0, 1]
        // The pivot is the 0 at index 98; it swaps with the 1, and the one-element suffix stays.
        let result = next(nums);
        assert_eq!(result[..3], [99, 98, 97]);
        assert_eq!(result[97..], [2, 1, 0]);
        // That is the last permutation, so one more step wraps around to ascending order.
        assert_eq!(next(result), (0..100).collect::<Vec<i32>>());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** scanning from the right, the first `i` with `nums[i] < nums[i + 1]` is the pivot.
Everything after it is in non-increasing order, so no rearrangement of the suffix alone can give
a larger permutation. Increase the pivot minimally by swapping it with the rightmost larger
element of the suffix, then sort the suffix ascending, which for a non-increasing sequence is a
simple reversal. Without a pivot the answer is the fully reversed (ascending) vector.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `(0..n - 1).rev().find(|&i| ...)` searches indices from the back; the closure only reads
  `nums`, and that shared borrow ends before `swap` needs a mutable one.
- `nums[start..].reverse()` reverses a sub-slice in place; with `start == 0` it is the whole vector.
- `expect` documents why the search cannot fail: `nums[i + 1] > nums[i]` itself qualifies.

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature takes `&mut Vec<i32>`; `&mut [i32]` would be enough here.
    #[allow(clippy::ptr_arg)]
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let pivot = (0..n.saturating_sub(1))
            .rev()
            .find(|&i| nums[i] < nums[i + 1]);
        let start = match pivot {
            Some(i) => {
                let j = (i + 1..n)
                    .rev()
                    .find(|&j| nums[j] > nums[i])
                    .expect("nums[i + 1] is greater than the pivot");
                nums.swap(i, j);
                i + 1
            }
            None => 0,
        };
        nums[start..].reverse();
    }
}
```

Full tested solution: [`src/solutions/p0192_next_permutation.rs`](../src/solutions/p0192_next_permutation.rs)

</details>
