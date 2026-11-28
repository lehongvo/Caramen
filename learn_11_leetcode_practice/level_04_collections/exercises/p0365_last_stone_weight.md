# Problem 0365: Last Stone Weight

- **Difficulty:** Easy
- **Topics:** `BinaryHeap::from(Vec)`, repeated `pop` of the two largest, `while let` with a tuple
- **Source:** [LeetCode 1046. Last Stone Weight](https://leetcode.com/problems/last-stone-weight/)

## Problem

You have a collection of stones with positive integer weights. On each turn, take the **two
heaviest** stones, with weights `x <= y`, and smash them together:

- if `x == y`, both stones are destroyed;
- otherwise the stone of weight `x` is destroyed and the other becomes a stone of weight `y - x`.

Repeat until at most one stone is left. Return the weight of the last stone, or `0` if none is
left.

Implement `Solution::last_stone_weight(stones: Vec<i32>) -> i32`.

## Examples

```text
Input:  stones = [2, 7, 4, 1, 8, 1]
Output: 1
Explanation: 8 and 7 -> 1, stones [2, 4, 1, 1, 1]; 4 and 2 -> 2, stones [2, 1, 1, 1];
             2 and 1 -> 1, stones [1, 1, 1]; 1 and 1 -> nothing, stones [1].

Input:  stones = [1]
Output: 1
```

## Constraints

- `1 <= stones.len() <= 30`
- `1 <= stones[i] <= 1000`

## Hints

1. "Repeatedly take the largest" is exactly what a max-heap is for.
2. Pop twice; if the weights differ, push the difference back.

## Starter code

Write your code in [`src/practice/p0365_last_stone_weight.rs`](../src/practice/p0365_last_stone_weight.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0365
```

```rust
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
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
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn example_single_stone() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn everything_cancels() {
        assert_eq!(Solution::last_stone_weight(vec![3, 3]), 0);
        assert_eq!(Solution::last_stone_weight(vec![2, 2, 4]), 0);
    }

    #[test]
    fn thirty_stones() {
        // Every round smashes the big stone against a stone of weight 1, so it shrinks by 1
        // per round: 1000 - 29 = 971.
        let mut stones = vec![1; 29];
        stones.push(1000);
        assert_eq!(Solution::last_stone_weight(stones), 971);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** put all stones in a `BinaryHeap`. While at least two stones remain, pop the two
heaviest and push back their difference if it is non-zero. The answer is the remaining stone, or
`0`.

**Complexity:** `O(n log n)` time, `O(n)` space.

**Rust concepts:**
- `BinaryHeap::from(stones)` turns the vector into a heap in `O(n)`, reusing its buffer.
- `match (heap.pop(), heap.pop())` pops twice per round and handles all three situations: two
  stones (smash them), exactly one stone (it was just popped, so return it), or none (return
  `0`). A tuple of `Option`s makes those cases explicit.
- Re-sorting the vector after every smash would be `O(n^2 log n)`; the heap keeps each step at
  `O(log n)`.

```rust
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);
        loop {
            match (heap.pop(), heap.pop()) {
                (Some(y), Some(x)) => {
                    if y != x {
                        heap.push(y - x);
                    }
                }
                (Some(last), None) => return last,
                _ => return 0,
            }
        }
    }
}
```

Full tested solution: [`src/solutions/p0365_last_stone_weight.rs`](../src/solutions/p0365_last_stone_weight.rs)

</details>
