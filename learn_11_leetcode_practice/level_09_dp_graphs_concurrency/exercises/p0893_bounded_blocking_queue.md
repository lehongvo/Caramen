# Problem 0893: Bounded Blocking Queue

- **Difficulty:** Medium
- **Topics:** a generic producer-consumer queue from `Mutex<VecDeque<T>>` and **two**
  `Condvar`s (`not_empty`, `not_full`), blocking `push`/`pop`, a non-blocking `try_pop`
- **Source:** Original · [`Condvar`](https://doc.rust-lang.org/std/sync/struct.Condvar.html)

## Problem

Build the data structure behind `sync_channel` yourself: a FIFO queue with a fixed capacity that
many threads can use through `&self`.

```text
pub struct BoundedQueue<T> { /* your fields */ }
impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self;      // capacity >= 1
    pub fn push(&self, item: T);              // blocks while the queue is full
    pub fn pop(&self) -> T;                   // blocks while the queue is empty
    pub fn try_pop(&self) -> Option<T>;       // never blocks
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
    pub fn peak(&self) -> usize;              // the largest len() ever reached
}
```

Items come out in the order they went in. `peak` lets the tests verify that the queue never held
more than `capacity` items, even with many producers and consumers.

## Examples

```text
let q = BoundedQueue::new(2);
q.push('a'); q.push('b');        // a third push would block until someone pops
q.len()     -> 2
q.pop()     -> 'a'
q.try_pop() -> Some('b')
q.try_pop() -> None
q.peak()    -> 2
```

## Constraints

- `1 <= capacity <= 1000`; up to 8 producers and 8 consumers.

## Hints

1. One mutex protects the deque (and the peak). `push` waits on `not_full` while
   `len == capacity`; `pop` waits on `not_empty` while the deque is empty.
2. After pushing, `not_empty.notify_one()`; after popping, `not_full.notify_one()`. Each event
   frees exactly one slot or one item, so waking one waiter is enough.
3. Always wait with a predicate (`wait_while`): another thread may have taken the item first.

## Starter code

Write your code in [`src/practice/p0893_bounded_blocking_queue.rs`](../src/practice/p0893_bounded_blocking_queue.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0893
```

```rust
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

/// A fixed-capacity FIFO queue whose `push` and `pop` block when needed.
pub struct BoundedQueue<T> {
    _todo: std::marker::PhantomData<T>,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&self, item: T) {
        todo!("Write your solution here")
    }

    pub fn pop(&self) -> T {
        todo!("Write your solution here")
    }

    pub fn try_pop(&self) -> Option<T> {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn peak(&self) -> usize {
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
    use std::thread;

    #[test]
    fn example_fifo() {
        let q = BoundedQueue::new(2);
        q.push('a');
        q.push('b');
        assert_eq!(q.len(), 2);
        assert_eq!(q.pop(), 'a');
        assert_eq!(q.try_pop(), Some('b'));
        assert_eq!(q.try_pop(), None);
        assert!(q.is_empty());
        assert_eq!(q.peak(), 2);
    }

    #[test]
    fn push_blocks_until_there_is_room() {
        let q = BoundedQueue::new(1);
        q.push(1);
        thread::scope(|s| {
            s.spawn(|| q.push(2)); // blocks until the main thread pops 1
            assert_eq!(q.pop(), 1);
            assert_eq!(q.pop(), 2); // blocks until the push above has happened
        });
        assert_eq!(q.peak(), 1);
    }

    #[test]
    fn pop_blocks_until_an_item_arrives() {
        let q = BoundedQueue::new(3);
        thread::scope(|s| {
            let consumer = s.spawn(|| q.pop() + q.pop());
            q.push(20);
            q.push(22);
            assert_eq!(consumer.join().unwrap(), 42);
        });
    }

    #[test]
    fn many_producers_and_consumers() {
        let q = BoundedQueue::new(4);
        let mut received: Vec<u32> = thread::scope(|s| {
            for p in 0..8 {
                let q = &q;
                s.spawn(move || {
                    for i in 0..1000 {
                        q.push(p * 1000 + i);
                    }
                });
            }
            let consumers: Vec<_> = (0..8)
                .map(|_| s.spawn(|| (0..1000).map(|_| q.pop()).collect::<Vec<u32>>()))
                .collect();
            consumers
                .into_iter()
                .flat_map(|c| c.join().unwrap())
                .collect()
        });
        received.sort_unstable();
        assert_eq!(received, (0..8000).collect::<Vec<u32>>());
        assert!(q.peak() <= 4 && q.peak() >= 1);
        assert!(q.is_empty());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `Mutex<State>` with a `VecDeque<T>` and the peak, plus two condition variables.
`push` waits until there is room, pushes, updates the peak and signals `not_empty`. `pop` waits
until an item exists, pops the front and signals `not_full`. `try_pop` pops if possible and also
signals `not_full` when it succeeds.

**Rust concepts:**
- Two condition variables on one mutex let producers and consumers wait for *different*
  conditions without waking each other needlessly.
- `BoundedQueue<T>` is `Sync` when `T: Send` (inherited from `Mutex<VecDeque<T>>`), so it can be
  shared by reference across scoped threads without any `unsafe`.
- `pop` can safely `expect` a value: `wait_while` returned only after seeing a non-empty deque,
  and the lock has been held since.

```rust
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

/// A fixed-capacity FIFO queue whose `push` and `pop` block when needed.
pub struct BoundedQueue<T> {
    state: Mutex<State<T>>,
    not_empty: Condvar,
    not_full: Condvar,
    capacity: usize,
}

struct State<T> {
    items: VecDeque<T>,
    peak: usize,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity >= 1, "the capacity must be at least 1");
        Self {
            state: Mutex::new(State {
                items: VecDeque::with_capacity(capacity),
                peak: 0,
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
            capacity,
        }
    }

    fn lock(&self) -> MutexGuard<'_, State<T>> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn push(&self, item: T) {
        let guard = self.lock();
        let mut state = self
            .not_full
            .wait_while(guard, |s| s.items.len() == self.capacity)
            .unwrap_or_else(PoisonError::into_inner);
        state.items.push_back(item);
        state.peak = state.peak.max(state.items.len());
        self.not_empty.notify_one();
    }

    pub fn pop(&self) -> T {
        let guard = self.lock();
        let mut state = self
            .not_empty
            .wait_while(guard, |s| s.items.is_empty())
            .unwrap_or_else(PoisonError::into_inner);
        let item = state
            .items
            .pop_front()
            .expect("wait_while saw a non-empty queue");
        self.not_full.notify_one();
        item
    }

    pub fn try_pop(&self) -> Option<T> {
        let item = self.lock().items.pop_front();
        if item.is_some() {
            self.not_full.notify_one();
        }
        item
    }

    pub fn len(&self) -> usize {
        self.lock().items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.lock().items.is_empty()
    }

    pub fn peak(&self) -> usize {
        self.lock().peak
    }
}
```

Full tested solution: [`src/solutions/p0893_bounded_blocking_queue.rs`](../src/solutions/p0893_bounded_blocking_queue.rs)

</details>
