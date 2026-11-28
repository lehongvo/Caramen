# Problem 0520: A Generic Ring Buffer

- **Difficulty:** Medium
- **Topics:** generic struct `RingBuffer<T>`, `Vec<Option<T>>`, `Option::take`/`replace`, method-level `where T: Clone`
- **Source:** Original · [Option::take](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)

## Problem

A ring buffer is a fixed-capacity queue that **overwrites its oldest element** when it is full.
Implement `RingBuffer<T>` for any element type. It stores its elements in a private field
`slots: Vec<Option<T>>` (one slot per unit of capacity), plus the index `head` of the oldest
element and the number of stored elements `len`. Do not use `VecDeque`.

- `with_capacity(capacity: usize) -> Self`: an empty buffer. Panics with the message
  `"capacity must be positive"` when `capacity == 0`.
- `capacity(&self) -> usize`, `len(&self) -> usize`, `is_empty(&self) -> bool`,
  `is_full(&self) -> bool`.
- `push(&mut self, item: T) -> Option<T>`: appends `item` as the newest element. If the buffer was
  full, the oldest element is removed to make room and returned; otherwise returns `None`.
- `pop(&mut self) -> Option<T>`: removes and returns the **oldest** element.
- `peek(&self) -> Option<&T>`: the oldest element.
- `get(&self, i: usize) -> Option<&T>`: the `i`-th oldest element (`0` is the oldest), or `None`
  if `i >= len`.
- `to_vec(&self) -> Vec<T>` returns the elements from oldest to newest. This is the only method
  that needs `T: Clone`: put the bound on the **method** with `where T: Clone`, not on the impl
  block, so a `RingBuffer` of non-cloneable values still has every other method.

## Examples

```text
let mut rb = RingBuffer::with_capacity(3);
rb.push(1); rb.push(2); rb.push(3)  => None, None, None
rb.push(4)                          => Some(1)     (overwrites the oldest)
rb.to_vec()                         => [2, 3, 4]
rb.pop()                            => Some(2)
rb.get(1)                           => Some(&4)
```

## Constraints

- `0 <= capacity <= 10^5` (zero panics)
- Up to `10^5` operations.

## Hints

1. The slot of the `i`-th oldest element is `(head + i) % capacity`.
2. `self.slots[k].take()` moves the value out of a slot and leaves `None` behind;
   `self.slots[k].replace(item)` puts `item` in and gives back what was there.

## Starter code

Write your code in [`src/practice/p0520_generic_ring_buffer.rs`](../src/practice/p0520_generic_ring_buffer.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0520
```

```rust
/// A fixed-capacity FIFO buffer that overwrites its oldest element when full.
#[derive(Debug)]
pub struct RingBuffer<T> {
    slots: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        todo!("Write your solution here")
    }

    pub fn capacity(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn is_full(&self) -> bool {
        todo!("Write your solution here")
    }

    /// Appends `item`; returns the overwritten oldest element when the buffer was full.
    pub fn push(&mut self, item: T) -> Option<T> {
        todo!("Write your solution here")
    }

    /// Removes and returns the oldest element.
    pub fn pop(&mut self) -> Option<T> {
        todo!("Write your solution here")
    }

    pub fn peek(&self) -> Option<&T> {
        todo!("Write your solution here")
    }

    /// The `i`-th oldest element.
    pub fn get(&self, i: usize) -> Option<&T> {
        todo!("Write your solution here")
    }

    /// The elements from oldest to newest.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
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
        let mut rb = RingBuffer::with_capacity(3);
        assert_eq!(rb.push(1), None);
        assert_eq!(rb.push(2), None);
        assert_eq!(rb.push(3), None);
        assert!(rb.is_full());
        assert_eq!(rb.push(4), Some(1));
        assert_eq!(rb.to_vec(), vec![2, 3, 4]);
        assert_eq!(rb.pop(), Some(2));
        assert_eq!(rb.get(1), Some(&4));
        assert_eq!(rb.get(2), None);
        assert_eq!(rb.len(), 2);
        assert_eq!(rb.capacity(), 3);
    }

    #[test]
    fn empty_buffer() {
        let mut rb: RingBuffer<String> = RingBuffer::with_capacity(2);
        assert!(rb.is_empty());
        assert_eq!(rb.pop(), None);
        assert_eq!(rb.peek(), None);
        assert!(rb.to_vec().is_empty());
    }

    #[test]
    #[should_panic(expected = "capacity must be positive")]
    fn zero_capacity_panics() {
        let _ = RingBuffer::<i32>::with_capacity(0);
    }

    #[test]
    fn non_clone_values_still_work() {
        struct Token(u32); // neither Clone nor Debug
        let mut rb = RingBuffer::with_capacity(1);
        assert!(rb.push(Token(1)).is_none());
        let evicted = rb.push(Token(2)).map(|t| t.0);
        assert_eq!(evicted, Some(1));
        assert_eq!(rb.peek().map(|t| t.0), Some(2));
    }

    #[test]
    fn wraps_around_many_times() {
        let mut rb = RingBuffer::with_capacity(7);
        for i in 0..100_000_u32 {
            assert_eq!(rb.push(i), i.checked_sub(7));
        }
        assert_eq!(rb.to_vec(), (99_993..100_000).collect::<Vec<u32>>());
        // Rotate: move the oldest element to the newest position, three times.
        for _ in 0..3 {
            let oldest = rb.pop().unwrap();
            assert_eq!(rb.push(oldest), None);
        }
        assert_eq!(
            rb.to_vec(),
            vec![99_996, 99_997, 99_998, 99_999, 99_993, 99_994, 99_995]
        );
        assert_eq!(rb.len(), 7);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `slots` is a circle. The newest element goes to slot `(head + len) % capacity`. When the
buffer is full that slot is exactly `head`, so we replace the oldest value and advance `head`.
Popping takes the value at `head` and advances it.

**Rust concepts:**
- `Vec<Option<T>>` lets a slot be empty without requiring `T: Default`; `(0..capacity).map(|_|
  None).collect()` builds it (`vec![None; n]` would require `T: Clone`).
- `Option::take` and `Option::replace` move values in and out of a slot through `&mut`, without
  cloning.
- A `where T: Clone` clause on one method restricts only that method. Bounds on the impl block
  would restrict all of them.

```rust
/// A fixed-capacity FIFO buffer that overwrites its oldest element when full.
#[derive(Debug)]
pub struct RingBuffer<T> {
    slots: Vec<Option<T>>,
    head: usize,
    len: usize,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        RingBuffer {
            slots: (0..capacity).map(|_| None).collect(),
            head: 0,
            len: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Appends `item`; returns the overwritten oldest element when the buffer was full.
    pub fn push(&mut self, item: T) -> Option<T> {
        let cap = self.capacity();
        if self.is_full() {
            let old = self.slots[self.head].replace(item);
            self.head = (self.head + 1) % cap;
            old
        } else {
            let tail = (self.head + self.len) % cap;
            self.slots[tail] = Some(item);
            self.len += 1;
            None
        }
    }

    /// Removes and returns the oldest element.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let item = self.slots[self.head].take();
        self.head = (self.head + 1) % self.capacity();
        self.len -= 1;
        item
    }

    pub fn peek(&self) -> Option<&T> {
        self.get(0)
    }

    /// The `i`-th oldest element.
    pub fn get(&self, i: usize) -> Option<&T> {
        if i >= self.len {
            return None;
        }
        self.slots[(self.head + i) % self.capacity()].as_ref()
    }

    /// The elements from oldest to newest.
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        (0..self.len).filter_map(|i| self.get(i).cloned()).collect()
    }
}
```

Full tested solution: [`src/solutions/p0520_generic_ring_buffer.rs`](../src/solutions/p0520_generic_ring_buffer.rs)

</details>
