# Problem 0715: Fruit Into Baskets

- **Difficulty:** Medium
- **Topics:** sliding window with at most two distinct values, count table `Vec<usize>`
- **Source:** [LeetCode 904. Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets/)

## Problem

A row of trees is described by `fruits`, where `fruits[i]` is the type of fruit tree `i`
produces. You have **two** baskets, and each basket may hold only one type of fruit (but any
amount of it). You choose a starting tree and walk to the right, picking exactly one fruit from
every tree, and you must stop as soon as a tree's fruit fits in neither basket.

Return the maximum number of fruits you can pick. In other words: the length of the longest
contiguous subarray that contains at most two distinct values.

Implement `Solution::total_fruit(fruits: Vec<i32>) -> i32`.

## Examples

```text
Input:  fruits = [1, 2, 1]
Output: 3

Input:  fruits = [0, 1, 2, 2]
Output: 3             (pick [1, 2, 2])

Input:  fruits = [1, 2, 3, 2, 2]
Output: 4             (pick [2, 3, 2, 2])
```

## Constraints

- `1 <= fruits.len() <= 10^5`
- `0 <= fruits[i] < fruits.len()`

## Hints

1. Keep a window and the number of fruits of each type inside it.
2. When a third type enters, shrink the window from the left until one type disappears.

## Starter code

Write your code in [`src/practice/p0715_fruit_into_baskets.rs`](../src/practice/p0715_fruit_into_baskets.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0715
```

```rust
pub struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
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
    fn example_two_kinds_only() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    }

    #[test]
    fn example_skip_first() {
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    }

    #[test]
    fn example_middle_window() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    }

    #[test]
    fn single_tree() {
        assert_eq!(Solution::total_fruit(vec![0]), 1);
    }

    #[test]
    fn longer_mixed_row() {
        assert_eq!(
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
            5
        );
    }

    #[test]
    fn long_alternating_row() {
        let fruits: Vec<i32> = (0..100_000).map(|i| i % 2).collect();
        assert_eq!(Solution::total_fruit(fruits), 100_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sliding window with at most two kinds: keep `counts[t]` (how many fruits of type
`t` are in the window) and `kinds` (how many types have a non-zero count). Add `fruits[right]`;
if its count goes from 0 to 1, a new kind entered. While there are more than two kinds, remove
`fruits[left]` and move `left`; when a count drops to 0, a kind left. The valid window length is
a candidate.

**Complexity:** `O(n)` time, `O(n)` space for the count table.

**Rust concepts:**
- The fruit types are guaranteed to be smaller than `fruits.len()`, so a `Vec<usize>` indexed
  by type replaces a `HashMap<i32, usize>`: no hashing and no entries to remove.
- Pick the smallest data structure the constraints allow; a `HashMap` is the general fallback
  when keys are unbounded.

```rust
pub struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut counts = vec![0usize; fruits.len()];
        let (mut left, mut kinds, mut best) = (0, 0, 0);
        for (right, &fruit) in fruits.iter().enumerate() {
            let count = &mut counts[fruit as usize];
            *count += 1;
            if *count == 1 {
                kinds += 1;
            }
            while kinds > 2 {
                let out = &mut counts[fruits[left] as usize];
                *out -= 1;
                if *out == 0 {
                    kinds -= 1;
                }
                left += 1;
            }
            best = best.max(right + 1 - left);
        }
        best as i32
    }
}
```

Full tested solution: [`src/solutions/p0715_fruit_into_baskets.rs`](../src/solutions/p0715_fruit_into_baskets.rs)

</details>
