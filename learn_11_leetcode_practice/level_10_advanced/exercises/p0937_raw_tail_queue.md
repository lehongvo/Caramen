# Problem 0937: A Queue with a Raw Tail Pointer

- **Difficulty:** Hard
- **Topics:** `Box::into_raw` / `Box::from_raw`, `*mut Node<T>`, `ptr::null_mut`, ownership through raw pointers, `PhantomData<Box<Node<T>>>`, iterative `Drop`
- **Source:** Original · [std: `Box::into_raw`](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.into_raw) · [The Rustonomicon: Ownership Based Resource Management](https://doc.rust-lang.org/nomicon/obrm.html)

## Problem

A FIFO queue as a singly linked list needs to push at the **tail** and pop at the **head** in
`O(1)`. With `Box` links only, the tail pointer would be a second owner of the last node, which
safe Rust forbids. Use raw pointers for every link instead and manage ownership yourself:

```text
struct Node<T> { value: T, next: *mut Node<T> }
pub struct Queue<T> { head: *mut Node<T>, tail: *mut Node<T>, len: usize, _owns: PhantomData<Box<Node<T>>> }
```

Each node is created with `Box::into_raw(Box::new(...))` and freed with exactly one
`Box::from_raw`. An empty queue has `head` and `tail` both null.

Implement `new()` (and `Default`), `push_back(&mut self, value: T)`,
`pop_front(&mut self) -> Option<T>`, `peek_front(&self) -> Option<&T>`,
`peek_back(&self) -> Option<&T>`, `peek_front_mut(&mut self) -> Option<&mut T>`, `len()`,
`is_empty()`, `iter(&self) -> impl Iterator<Item = &T>` (front to back), and `Drop` (free every
node with a loop, so a long queue cannot overflow the stack).

## Examples

```text
let mut q = Queue::new();
q.push_back(1); q.push_back(2); q.push_back(3);
q.pop_front()   => Some(1)
q.peek_front()  => Some(&2),  q.peek_back() => Some(&3)
q.iter().copied().collect::<Vec<_>>() => [2, 3]
q.pop_front(); q.pop_front(); q.pop_front() => None, and both pointers are null again
```

## Constraints

- Up to `10^6` elements.

## Hints

1. `push_back`: allocate the node, then link it after `tail` (or make it the `head` if the queue
   is empty), then move `tail` to it.
2. `pop_front`: turn `head` back into a `Box` with `Box::from_raw`, move `head` to its `next`, and
   reset `tail` to null when the queue becomes empty (otherwise it would dangle).
3. `iter` can walk the pointers with `std::iter::successors` or a small loop in `from_fn`.

## Starter code

Write your code in [`src/practice/p0937_raw_tail_queue.rs`](../src/practice/p0937_raw_tail_queue.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0937
```

```rust
use std::marker::PhantomData;
use std::ptr;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

pub struct Queue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
    _owns: PhantomData<Box<Node<T>>>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push_back(&mut self, value: T) {
        todo!("Write your solution here")
    }

    pub fn pop_front(&mut self) -> Option<T> {
        todo!("Write your solution here")
    }

    pub fn peek_front(&self) -> Option<&T> {
        todo!("Write your solution here")
    }

    pub fn peek_back(&self) -> Option<&T> {
        todo!("Write your solution here")
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        std::iter::from_fn(|| todo!())
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        // TODO: free every node with a loop (never panic inside `drop`).
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
    use std::rc::Rc;

    #[test]
    fn fifo_order() {
        let mut q = Queue::new();
        q.push_back(1);
        q.push_back(2);
        q.push_back(3);
        assert_eq!(q.pop_front(), Some(1));
        assert_eq!((q.peek_front(), q.peek_back()), (Some(&2), Some(&3)));
        assert_eq!(q.iter().copied().collect::<Vec<_>>(), vec![2, 3]);
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn emptying_resets_the_tail() {
        let mut q = Queue::default();
        q.push_back("a");
        assert_eq!(q.pop_front(), Some("a"));
        assert_eq!((q.pop_front(), q.peek_back()), (None, None));
        assert!(q.is_empty());
        // After emptying, pushing must not write through a dangling tail.
        q.push_back("b");
        q.push_back("c");
        assert_eq!(q.iter().collect::<Vec<_>>(), vec![&"b", &"c"]);
        assert_eq!(q.peek_back(), Some(&"c"));
    }

    #[test]
    fn mutate_the_front() {
        let mut q = Queue::new();
        q.push_back(String::from("x"));
        q.push_back(String::from("y"));
        if let Some(front) = q.peek_front_mut() {
            front.push('!');
        }
        assert_eq!(q.pop_front().as_deref(), Some("x!"));
        assert_eq!(q.peek_front().map(String::as_str), Some("y"));
    }

    #[test]
    fn drop_frees_every_node() {
        let token = Rc::new(());
        {
            let mut q = Queue::new();
            for _ in 0..10 {
                q.push_back(Rc::clone(&token));
            }
            drop(q.pop_front());
            assert_eq!(Rc::strong_count(&token), 10);
        }
        assert_eq!(Rc::strong_count(&token), 1);
    }

    #[test]
    fn matches_a_vecdeque_model() {
        use std::collections::VecDeque;
        let mut q = Queue::new();
        let mut model = VecDeque::new();
        for i in 0..3_000_u32 {
            if i % 3 == 2 {
                assert_eq!(q.pop_front(), model.pop_front());
            } else {
                q.push_back(i);
                model.push_back(i);
            }
            assert_eq!(q.len(), model.len());
            assert_eq!(q.peek_back(), model.back());
        }
        assert!(q.iter().eq(model.iter()));
    }

    #[test]
    fn long_queue_drops_without_stack_overflow() {
        let mut long = Queue::new();
        for i in 0..1_000_000_u32 {
            long.push_back(i);
        }
        assert_eq!(long.len(), 1_000_000);
        assert_eq!(long.peek_back(), Some(&999_999));
        drop(long);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the queue owns every node through the raw `head` chain; `tail` is a non-owning alias of
the last node, used only to append. Pushing writes `(*tail).next`; popping reclaims the head box,
which frees the node when it goes out of scope after its value is moved out. `Drop` pops until
empty.

**Complexity:** `push_back`, `pop_front` and the peeks are `O(1)`; `iter` and `drop` are `O(n)`.

**Rust concepts:**
- `Box::into_raw` gives up the box's ownership without freeing; `Box::from_raw` takes it back.
  Calling `from_raw` twice on the same pointer is a double free.
- Raw pointers are not `Send`/`Sync` and carry no ownership, so the struct adds
  `PhantomData<Box<Node<T>>>` to tell the drop checker that the queue owns `T` values.
- The returned references (`&T`, `&mut T`) borrow `self`, so the borrow checker still prevents
  popping a node while a reference into it is alive.

```rust
use std::marker::PhantomData;
use std::ptr;

struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

pub struct Queue<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
    _owns: PhantomData<Box<Node<T>>>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
            _owns: PhantomData,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let node = Box::into_raw(Box::new(Node {
            value,
            next: ptr::null_mut(),
        }));
        if self.tail.is_null() {
            self.head = node;
        } else {
            // SAFETY: a non-null `tail` points to the last node, which the queue owns and keeps alive.
            unsafe { (*self.tail).next = node };
        }
        self.tail = node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }
        // SAFETY: a non-null `head` came from `Box::into_raw` and is reclaimed exactly once, here.
        let node = unsafe { Box::from_raw(self.head) };
        self.head = node.next;
        if self.head.is_null() {
            self.tail = ptr::null_mut();
        }
        self.len -= 1;
        Some(node.value)
    }

    pub fn peek_front(&self) -> Option<&T> {
        // SAFETY: `head` is null or points to a live node owned by `self`, borrowed for `&self`.
        unsafe { self.head.as_ref().map(|node| &node.value) }
    }

    pub fn peek_back(&self) -> Option<&T> {
        // SAFETY: `tail` is null or points to the live last node.
        unsafe { self.tail.as_ref().map(|node| &node.value) }
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        // SAFETY: as in `peek_front`, and `&mut self` makes the access unique.
        unsafe { self.head.as_mut().map(|node| &mut node.value) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut current = self.head;
        std::iter::from_fn(move || {
            // SAFETY: every pointer in the chain is null or a live node owned by `self`, which stays
            // borrowed while the iterator exists.
            let node = unsafe { current.as_ref()? };
            current = node.next;
            Some(&node.value)
        })
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
```

Full tested solution: [`src/solutions/p0937_raw_tail_queue.rs`](../src/solutions/p0937_raw_tail_queue.rs)

</details>
