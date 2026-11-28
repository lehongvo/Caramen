# Problem 0368: Kth Largest Element in a Stream

- **Difficulty:** Easy
- **Topics:** min-heap with `BinaryHeap<Reverse<i32>>`, keeping only `k` elements, `peek`, design struct
- **Source:** [LeetCode 703. Kth Largest Element in a Stream](https://leetcode.com/problems/kth-largest-element-in-a-stream/)

## Problem

A school wants to know, after every new test score, the `k`-th highest score so far (counting
duplicates: in `[5, 5, 3]` the 2nd largest is `5`). Implement the `KthLargest` struct:

- `KthLargest::new(k: i32, nums: Vec<i32>) -> Self` starts with the scores `nums`;
- `add(&mut self, val: i32) -> i32` adds the score `val` and returns the `k`-th largest score
  among all scores so far.

Whenever `add` is called there are at least `k` scores (including the new one). The method takes
`&mut self` instead of the template's `&self`, as LeetCode's comment allows.

## Examples

```text
let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
kth.add(3)  => 4    (scores 2 3 4 5 8: the 3rd largest is 4)
kth.add(5)  => 5    (2 3 4 5 5 8)
kth.add(10) => 5    (2 3 4 5 5 8 10)
kth.add(9)  => 8    (2 3 4 5 5 8 9 10)
kth.add(4)  => 8
```

## Constraints

- `0 <= nums.len() <= 10^4`, `1 <= k <= nums.len() + 1`
- `-10^4 <= nums[i], val <= 10^4`
- At most `10^4` calls to `add`.

## Hints

1. Scores smaller than the current `k`-th largest can never become the `k`-th largest again.
2. Keep only the `k` largest scores. The answer is the **smallest** of them, so you need a
   min-heap: `BinaryHeap<Reverse<i32>>`.

## Starter code

Write your code in [`src/practice/p0368_kth_largest_element_in_a_stream.rs`](../src/practice/p0368_kth_largest_element_in_a_stream.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0368
```

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Tracks the `k`-th largest score of a stream.
pub struct KthLargest {}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        todo!("Write your solution here")
    }

    pub fn add(&mut self, val: i32) -> i32 {
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
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }

    #[test]
    fn starting_empty() {
        let mut kth = KthLargest::new(1, vec![]);
        assert_eq!(kth.add(-3), -3);
        assert_eq!(kth.add(-2), -2);
        assert_eq!(kth.add(-4), -2);
        assert_eq!(kth.add(0), 0);
    }

    #[test]
    fn duplicates_count() {
        let mut kth = KthLargest::new(2, vec![5]);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(3), 5);
        assert_eq!(kth.add(7), 5);
        assert_eq!(kth.add(7), 7);
    }

    #[test]
    fn long_stream() {
        let nums: Vec<i32> = (-10_000..0).collect();
        let mut kth = KthLargest::new(100, nums);
        assert_eq!(kth.add(-10_000), -100);
        let mut last = 0;
        for v in 0..10_000 {
            last = kth.add(v);
        }
        assert_eq!(last, 9_900);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** store the `k` largest scores in a min-heap. Adding a score pushes it and, if the heap
now holds `k + 1` scores, pops the smallest. The top of the heap is then the `k`-th largest.

**Complexity:** `O(n log k)` for `new`, `O(log k)` per `add`; `O(k)` space.

**Rust concepts:**
- `BinaryHeap` is a max-heap; wrapping values in `std::cmp::Reverse` turns it into a min-heap,
  because `Reverse(a) > Reverse(b)` exactly when `a < b`.
- `peek()` returns `Option<&Reverse<i32>>`; the pattern `Some(&Reverse(x))` unwraps both layers.
- A private method `push` is reused by `new` (for the initial scores) and `add`.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Tracks the `k`-th largest score of a stream.
pub struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut this = Self {
            k,
            heap: BinaryHeap::with_capacity(k + 1),
        };
        for x in nums {
            this.push(x);
        }
        this
    }

    fn push(&mut self, val: i32) {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.push(val);
        match self.heap.peek() {
            Some(&Reverse(x)) => x,
            // Unreachable: the constraints guarantee at least k scores.
            None => i32::MIN,
        }
    }
}
```

Full tested solution: [`src/solutions/p0368_kth_largest_element_in_a_stream.rs`](../src/solutions/p0368_kth_largest_element_in_a_stream.rs)

</details>
