# Problem 0384: Hand of Straights

- **Difficulty:** Medium
- **Topics:** `BTreeMap` counts, `first_key_value` for the smallest key, decrement-and-remove with `get_mut`, `while let`
- **Source:** [LeetCode 846. Hand of Straights](https://leetcode.com/problems/hand-of-straights/)

## Problem

Alice holds a hand of cards, `hand[i]` being the value of the `i`-th card. She wants to split
the whole hand into groups of exactly `group_size` cards, where each group consists of
`group_size` **consecutive** values (for example `3, 4, 5`). Every card must be used exactly
once. Return `true` if this is possible.

Implement `Solution::is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool`.

## Examples

```text
Input:  hand = [1, 2, 3, 6, 2, 3, 4, 7, 8], group_size = 3
Output: true      ([1, 2, 3], [2, 3, 4], [6, 7, 8])

Input:  hand = [1, 2, 3, 4, 5], group_size = 4
Output: false     (5 cards cannot form groups of 4)
```

## Constraints

- `1 <= hand.len() <= 10^4`
- `0 <= hand[i] <= 10^9`
- `1 <= group_size <= hand.len()`

## Hints

1. The smallest remaining card must start a group: nothing smaller is left to precede it.
2. Keep counts in a `BTreeMap`, so the smallest remaining value is `first_key_value()`.
3. Use up one card of each value `start, start + 1, ..., start + group_size - 1`; if one is
   missing, the answer is `false`.

## Starter code

Write your code in [`src/practice/p0384_hand_of_straights.rs`](../src/practice/p0384_hand_of_straights.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0384
```

```rust
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
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
        assert!(Solution::is_n_straight_hand(
            vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
            3
        ));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }

    #[test]
    fn group_size_one_and_gaps() {
        assert!(Solution::is_n_straight_hand(vec![5, 5, 9], 1));
        // [1, 2] works, but 4 would need a 5.
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 4, 6], 2));
        assert!(Solution::is_n_straight_hand(vec![4, 1, 5, 2], 2));
    }

    #[test]
    fn duplicates_need_matching_runs() {
        assert!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3], 3));
        assert!(!Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 4], 3));
    }

    #[test]
    fn large_hand() {
        // 2_000 groups of 5 consecutive values near 10^9, shuffled.
        let mut hand: Vec<i32> = (0..10_000)
            .map(|i| 999_990_000 + (i % 5) + 5 * (i / 5))
            .collect();
        hand.reverse();
        hand.rotate_left(4_321);
        assert!(Solution::is_n_straight_hand(hand.clone(), 5));
        hand[0] += 1;
        assert!(!Solution::is_n_straight_hand(hand, 5));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** greedy from the smallest card. While cards remain, take the smallest value `start`
and remove one card of each value in `start..start + group_size`, deleting a value from the map
when its count reaches zero. If any value of the run is absent, no valid split exists.

**Complexity:** `O(n log n)` time, `O(n)` space.

**Rust concepts:**
- `first_key_value()` returns `Option<(&K, &V)>` in `O(log n)`; copying the key out with
  `(&start, _)` ends the borrow before the map is modified.
- `match counts.get_mut(&v)` with `*c -= 1` and then `counts.remove(&v)` is accepted because
  the `&mut` reference `c` is not used after the removal (non-lexical lifetimes).
- A quick length check (`hand.len() % group_size != 0`) rejects impossible inputs early.
- `start + group_size` stays within `i32`: `10^9 + 10^4 < 2^31`.

```rust
use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        let mut counts: BTreeMap<i32, usize> = BTreeMap::new();
        for card in hand {
            *counts.entry(card).or_insert(0) += 1;
        }
        while let Some((&start, _)) = counts.first_key_value() {
            for v in start..start + group_size {
                match counts.get_mut(&v) {
                    Some(c) => {
                        *c -= 1;
                        if *c == 0 {
                            counts.remove(&v);
                        }
                    }
                    None => return false,
                }
            }
        }
        true
    }
}
```

Full tested solution: [`src/solutions/p0384_hand_of_straights.rs`](../src/solutions/p0384_hand_of_straights.rs)

</details>
