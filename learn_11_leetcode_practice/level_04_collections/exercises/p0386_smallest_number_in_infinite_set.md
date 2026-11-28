# Problem 0386: Smallest Number in Infinite Set

- **Difficulty:** Medium
- **Topics:** `BTreeSet::pop_first`, representing an infinite set with a counter plus a small set, design struct
- **Source:** [LeetCode 2336. Smallest Number in Infinite Set](https://leetcode.com/problems/smallest-number-in-infinite-set/)

## Problem

A set initially contains **every** positive integer `1, 2, 3, ...`. Implement the
`SmallestInfiniteSet` struct:

- `SmallestInfiniteSet::new() -> Self`: the full set of positive integers;
- `pop_smallest(&mut self) -> i32`: remove the smallest number in the set and return it;
- `add_back(&mut self, num: i32)`: put `num` back into the set, if it is not already there.

The mutating methods take `&mut self` instead of the template's `&self`, as LeetCode's comment
allows.

## Examples

```text
let mut s = SmallestInfiniteSet::new();
s.add_back(2);          (2 is already in the set: nothing changes)
s.pop_smallest() => 1
s.pop_smallest() => 2
s.pop_smallest() => 3
s.add_back(1);          (1 is back)
s.pop_smallest() => 1
s.pop_smallest() => 4
s.pop_smallest() => 5
```

## Constraints

- `1 <= num <= 1000`
- At most `1000` calls in total.

## Hints

1. You cannot store infinitely many numbers, but you can store "every number `>= next` is
   present" as a single integer `next`.
2. Numbers smaller than `next` that were added back go into a `BTreeSet`, whose smallest element
   is always popped first.

## Starter code

Write your code in [`src/practice/p0386_smallest_number_in_infinite_set.rs`](../src/practice/p0386_smallest_number_in_infinite_set.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0386
```

```rust
use std::collections::BTreeSet;

/// All positive integers, with removals and re-insertions.
pub struct SmallestInfiniteSet {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl SmallestInfiniteSet {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn pop_smallest(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn add_back(&mut self, num: i32) {
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
    fn example_sequence() {
        let mut s = SmallestInfiniteSet::new();
        s.add_back(2);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 2);
        assert_eq!(s.pop_smallest(), 3);
        s.add_back(1);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 4);
        assert_eq!(s.pop_smallest(), 5);
    }

    #[test]
    fn repeated_add_back_counts_once() {
        let mut s = SmallestInfiniteSet::new();
        assert_eq!(s.pop_smallest(), 1);
        s.add_back(1);
        s.add_back(1);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 2);
    }

    #[test]
    fn added_back_numbers_come_out_sorted() {
        let mut s = SmallestInfiniteSet::new();
        for _ in 0..10 {
            s.pop_smallest();
        }
        s.add_back(7);
        s.add_back(3);
        s.add_back(9);
        let popped: Vec<i32> = (0..4).map(|_| s.pop_smallest()).collect();
        assert_eq!(popped, vec![3, 7, 9, 11]);
    }

    #[test]
    fn many_calls() {
        let mut s = SmallestInfiniteSet::new();
        for expected in 1..=500 {
            assert_eq!(s.pop_smallest(), expected);
        }
        for num in (1..=500).rev().step_by(2) {
            s.add_back(num);
        }
        // 500, 498, ..., 2 were added back: they come out in increasing order.
        assert_eq!(s.pop_smallest(), 2);
        assert_eq!(s.pop_smallest(), 4);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split the set into two parts: the infinite tail `next, next + 1, ...` (one counter)
and a `BTreeSet` of numbers below `next` that were added back. The smallest number is the
smallest of the `BTreeSet` if it is non-empty (all of its numbers are `< next`), otherwise
`next`. `add_back` only has to remember numbers `< next`; larger ones are already in the tail.

**Complexity:** `O(log m)` per call for `m` added-back numbers, `O(m)` space.

**Rust concepts:**
- `pop_first()` returns `Option<T>`: `if let Some(x) = ... { x } else { ... }` chooses between
  the two parts.
- `BTreeSet::insert` ignores numbers that are already present, which covers repeated
  `add_back` calls.
- A `BinaryHeap<Reverse<i32>>` plus a `HashSet` for membership would also work; the `BTreeSet`
  does both jobs at once.

```rust
use std::collections::BTreeSet;

/// All positive integers, with removals and re-insertions.
pub struct SmallestInfiniteSet {
    next: i32,
    added_back: BTreeSet<i32>,
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl SmallestInfiniteSet {
    pub fn new() -> Self {
        Self {
            next: 1,
            added_back: BTreeSet::new(),
        }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        if let Some(x) = self.added_back.pop_first() {
            x
        } else {
            self.next += 1;
            self.next - 1
        }
    }

    pub fn add_back(&mut self, num: i32) {
        if num < self.next {
            self.added_back.insert(num);
        }
    }
}
```

Full tested solution: [`src/solutions/p0386_smallest_number_in_infinite_set.rs`](../src/solutions/p0386_smallest_number_in_infinite_set.rs)

</details>
