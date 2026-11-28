# Problem 0516: Implement Queue using Stacks

- **Difficulty:** Easy
- **Topics:** design struct, two stacks, amortized analysis, `&self` vs `&mut self`, `Default`
- **Source:** [LeetCode 232. Implement Queue using Stacks](https://leetcode.com/problems/implement-queue-using-stacks/)

## Problem

Build a first-in, first-out queue of `i32` using **only stack operations**: push to the top,
look at or pop the top, ask for the size, ask whether it is empty. In Rust, use `Vec<i32>` as
the stack and only call `push`, `pop`, `last`, `len` and `is_empty` on it (no indexing, no
`first`, no `remove(0)`, no `VecDeque`).

Implement `MyQueue`:

- `MyQueue::new() -> Self`: an empty queue.
- `push(&mut self, x: i32)`: adds `x` at the back.
- `pop(&mut self) -> i32`: removes and returns the front element.
- `peek(&self) -> i32`: returns the front element without removing it. Note that it takes
  `&self`, so it cannot move elements between stacks.
- `empty(&self) -> bool`: whether the queue is empty.

Also implement `Default` (an empty queue). This is a LeetCode *design* problem, so there is no
`impl Solution`: paste the struct and its `impl MyQueue` block into LeetCode (the extra
`impl Default` is accepted too).

## Examples

```text
let mut q = MyQueue::new();
q.push(1);
q.push(2);
q.peek()  => 1
q.pop()   => 1
q.empty() => false
```

## Constraints

- `1 <= x <= 9`
- At most `100` calls to `push`, `pop`, `peek` and `empty`.
- `pop` and `peek` are only called on a non-empty queue.
- Follow-up: make every operation amortized `O(1)`.

## Hints

1. Push onto an `input` stack. To pop, take from an `output` stack; when `output` is empty, pour
   all of `input` into it, which reverses the order.
2. `peek` cannot pour (it only has `&self`). Remember the front of `input` in an extra field: it
   is the element pushed while `input` was empty.

## Starter code

Write your code in [`src/practice/p0516_implement_queue_using_stacks.rs`](../src/practice/p0516_implement_queue_using_stacks.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0516
```

```rust
/// A FIFO queue built from two LIFO stacks.
pub struct MyQueue {}

impl MyQueue {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&mut self, x: i32) {
        todo!("Write your solution here")
    }

    pub fn pop(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn peek(&self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn empty(&self) -> bool {
        todo!("Write your solution here")
    }
}

impl Default for MyQueue {
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
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
    }

    #[test]
    fn interleaved_operations_keep_fifo_order() {
        let mut q = MyQueue::default();
        assert!(q.empty());
        q.push(3);
        q.push(4);
        assert_eq!(q.pop(), 3);
        q.push(5); // goes to `input` while `output` still holds 4
        assert_eq!(q.peek(), 4);
        assert_eq!(q.pop(), 4);
        assert_eq!(q.peek(), 5);
        assert_eq!(q.pop(), 5);
        assert!(q.empty());
    }

    #[test]
    fn peek_after_refill() {
        let mut q = MyQueue::new();
        for x in 1..=3 {
            q.push(x);
        }
        assert_eq!(q.pop(), 1);
        q.push(9);
        q.push(8);
        assert_eq!(q.peek(), 2);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.pop(), 3);
        assert_eq!(q.peek(), 9);
    }

    #[test]
    fn matches_a_reference_queue() {
        use std::collections::VecDeque;
        let mut q = MyQueue::new();
        let mut reference = VecDeque::new();
        for step in 0..10_000 {
            if step % 3 == 2 && !reference.is_empty() {
                assert_eq!(q.peek(), *reference.front().unwrap());
                assert_eq!(q.pop(), reference.pop_front().unwrap());
            } else {
                let x = step % 9 + 1;
                q.push(x);
                reference.push_back(x);
            }
            assert_eq!(q.empty(), reference.is_empty());
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** two stacks. `input` receives pushes; `output` holds older elements in reversed order, so
its top is the queue front. `pop` refills `output` only when it is empty. For `peek(&self)`, the
front is `output`'s top if `output` is non-empty, otherwise it is the bottom of `input`, which we
remember in `input_front` whenever we push onto an empty `input`.

**Complexity:** amortized `O(1)` per operation (each element moves from `input` to `output` once);
`O(n)` space.

**Rust concepts:**
- The receiver says what a method may do: `&mut self` methods can change the stacks, `&self`
  methods cannot. Designing the state so that `peek` works through `&self` avoids interior
  mutability (`RefCell`).
- `while let Some(x) = self.input.pop()` drains one stack into the other.
- The constraints guarantee `pop`/`peek` are called on a non-empty queue, so `expect` documents an
  invariant rather than handling a user error.

```rust
/// A FIFO queue built from two LIFO stacks.
pub struct MyQueue {
    input: Vec<i32>,
    output: Vec<i32>,
    input_front: i32,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            input: Vec::new(),
            output: Vec::new(),
            input_front: 0,
        }
    }

    pub fn push(&mut self, x: i32) {
        if self.input.is_empty() {
            self.input_front = x;
        }
        self.input.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.output.is_empty() {
            while let Some(x) = self.input.pop() {
                self.output.push(x);
            }
        }
        self.output
            .pop()
            .expect("pop is only called on a non-empty queue")
    }

    pub fn peek(&self) -> i32 {
        match self.output.last() {
            Some(&x) => x,
            None => self.input_front,
        }
    }

    pub fn empty(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }
}

impl Default for MyQueue {
    fn default() -> Self {
        Self::new()
    }
}
```

Full tested solution: [`src/solutions/p0516_implement_queue_using_stacks.rs`](../src/solutions/p0516_implement_queue_using_stacks.rs)

</details>
