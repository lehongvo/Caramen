# Problem 0517: Min Stack

- **Difficulty:** Medium
- **Topics:** design struct, storing derived data with each element, tuples in a `Vec`, `Default`
- **Source:** [LeetCode 155. Min Stack](https://leetcode.com/problems/min-stack/)

## Problem

Design a stack of `i32` that can also report its **minimum** element, with every operation in
constant time.

- `MinStack::new() -> Self`: an empty stack.
- `push(&mut self, val: i32)`: puts `val` on top.
- `pop(&mut self)`: removes the top element (returns nothing).
- `top(&self) -> i32`: the top element.
- `get_min(&self) -> i32`: the smallest element currently in the stack.

Also implement `Default`. This is a LeetCode *design* problem, so there is no `impl Solution`:
paste the struct and its `impl MinStack` block into LeetCode.

## Examples

```text
let mut s = MinStack::new();
s.push(-2); s.push(0); s.push(-3);
s.get_min() => -3
s.pop();
s.top()     => 0
s.get_min() => -2
```

## Constraints

- `-2^31 <= val <= 2^31 - 1`
- `pop`, `top` and `get_min` are only called on a non-empty stack.
- At most `3 * 10^4` calls in total.

## Hints

1. When an element is pushed, the minimum "from the bottom up to here" is fixed forever: it only
   depends on the elements below it.
2. Store that running minimum next to each element, for example as `Vec<(i32, i32)>`.

## Starter code

Write your code in [`src/practice/p0517_min_stack.rs`](../src/practice/p0517_min_stack.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0517
```

```rust
/// A stack that also reports its minimum in `O(1)`.
pub struct MinStack {}

impl MinStack {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&mut self, val: i32) {
        todo!("Write your solution here")
    }

    pub fn pop(&mut self) {
        todo!("Write your solution here")
    }

    pub fn top(&self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn get_min(&self) -> i32 {
        todo!("Write your solution here")
    }
}

impl Default for MinStack {
    fn default() -> Self {
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
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }

    #[test]
    fn duplicate_minimums() {
        let mut s = MinStack::default();
        s.push(1);
        s.push(1);
        s.push(2);
        s.pop();
        s.pop();
        assert_eq!(s.get_min(), 1);
        assert_eq!(s.top(), 1);
    }

    #[test]
    fn extreme_values() {
        let mut s = MinStack::new();
        s.push(i32::MAX);
        assert_eq!(s.get_min(), i32::MAX);
        s.push(i32::MIN);
        assert_eq!(s.get_min(), i32::MIN);
        s.pop();
        assert_eq!(s.get_min(), i32::MAX);
    }

    #[test]
    fn decreasing_then_popping() {
        let mut s = MinStack::new();
        for v in (0..30_000).rev() {
            s.push(v);
            assert_eq!(s.get_min(), v);
        }
        for v in 0..29_999 {
            assert_eq!(s.top(), v);
            s.pop();
            assert_eq!(s.get_min(), v + 1);
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** every entry stores `(value, min_of_this_and_everything_below)`. Pushing computes the new
running minimum from the previous top entry; popping simply removes the entry, which restores the
previous minimum automatically.

**Complexity:** `O(1)` per operation, `O(n)` space.

**Rust concepts:**
- A `Vec<(i32, i32)>` keeps the value and its derived data together, so they can never get out of
  sync.
- `self.entries.last().map_or(val, |&(_, m)| m.min(val))` handles the empty stack in one
  expression: `map_or` supplies the default when there is no previous entry.
- Comparing with `min` never overflows, even with `i32::MIN` and `i32::MAX` in the stack.

```rust
/// A stack that also reports its minimum in `O(1)`.
pub struct MinStack {
    entries: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            entries: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        let min = self.entries.last().map_or(val, |&(_, m)| m.min(val));
        self.entries.push((val, min));
    }

    pub fn pop(&mut self) {
        self.entries.pop();
    }

    pub fn top(&self) -> i32 {
        self.entries
            .last()
            .expect("top is only called on a non-empty stack")
            .0
    }

    pub fn get_min(&self) -> i32 {
        self.entries
            .last()
            .expect("get_min is only called on a non-empty stack")
            .1
    }
}

impl Default for MinStack {
    fn default() -> Self {
        Self::new()
    }
}
```

Full tested solution: [`src/solutions/p0517_min_stack.rs`](../src/solutions/p0517_min_stack.rs)

</details>
