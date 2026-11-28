# Problem 0612: Persistent Stack

- **Difficulty:** Medium
- **Topics:** structural sharing with `Rc`, immutable data structures, `Rc::try_unwrap`, iterative `Drop`
- **Source:** Original · [The Rust Book 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html) · [`Rc::try_unwrap`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.try_unwrap)

## Problem

A *persistent* stack never changes: `push` and `pop` return a **new version** and leave the old
one intact. This is cheap because versions share their common nodes through `Rc`. Pushing onto
a version creates exactly one node; popping creates none.

The layout is given:

```text
pub struct PStack<T> {
    head: Option<Rc<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}
```

Implement:

- `PStack::new() -> PStack<T>`: the empty stack;
- `push(&self, value: T) -> PStack<T>`: a new version with `value` on top;
- `pop(&self) -> PStack<T>`: a new version without the top value (popping the empty stack gives
  the empty stack);
- `peek(&self) -> Option<&T>`, `len(&self) -> usize`, `is_empty(&self) -> bool`;
- `to_vec(&self) -> Vec<T>` (for `T: Clone`): the values from top to bottom;
- `impl Drop for PStack<T>`: release nodes with a loop. Walk down only while this version is
  the **last** owner of the node (use `Rc::try_unwrap`); stop at the first node another version
  still uses. Dropping a 100 000-element stack must not overflow the stack.

## Examples

```text
let s1 = PStack::new().push(1).push(2);   // [2, 1]
let s2 = s1.push(3);                      // [3, 2, 1]
let s3 = s1.push(4);                      // [4, 2, 1]
s1.to_vec() == [2, 1]                     // unchanged
s2.pop().to_vec() == [2, 1]               // shares its nodes with s1
s3.peek() == Some(&4), s3.len() == 3
```

## Constraints

- Up to `10^5` values in one version.

## Hints

1. `push`: the new head is `Rc::new(Node { value, next: self.head.clone() })`; cloning an
   `Option<Rc<..>>` only bumps a counter.
2. `pop`: the new head is `self.head.as_ref().and_then(|node| node.next.clone())`.
3. `Rc::try_unwrap(rc)` returns `Ok(node)` only when `rc` was the last strong handle; otherwise
   it gives the `Rc` back as `Err(rc)` (and dropping that just decrements the count).

## Starter code

Write your code in [`src/practice/p0612_persistent_stack.rs`](../src/practice/p0612_persistent_stack.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0612
```

```rust
use std::rc::Rc;

pub struct PStack<T> {
    head: Option<Rc<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

// Clippy would like a `Default` impl next to `new()`; it is not needed here.
#[allow(clippy::new_without_default)]
impl<T> PStack<T> {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&self, value: T) -> PStack<T> {
        todo!("Write your solution here")
    }

    pub fn pop(&self) -> PStack<T> {
        todo!("Write your solution here")
    }

    pub fn peek(&self) -> Option<&T> {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        todo!("Write your solution here")
    }
}

impl<T> Drop for PStack<T> {
    fn drop(&mut self) {
        // TODO: free the nodes this version owns alone, with a loop.
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
    use std::cell::Cell;

    struct Tracked(Rc<Cell<usize>>);

    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn old_versions_are_unchanged() {
        let s1 = PStack::new().push(1).push(2);
        let s2 = s1.push(3);
        let s3 = s1.push(4);
        assert_eq!(s1.to_vec(), vec![2, 1]);
        assert_eq!(s2.to_vec(), vec![3, 2, 1]);
        assert_eq!(s3.to_vec(), vec![4, 2, 1]);
        assert_eq!(s3.peek(), Some(&4));
        assert_eq!(s3.len(), 3);
    }

    #[test]
    fn pop_shares_nodes_with_the_older_version() {
        let s1 = PStack::new().push(1).push(2);
        let s2 = s1.push(3);
        let popped = s2.pop();
        assert_eq!(popped.to_vec(), vec![2, 1]);
        let (a, b) = (popped.head.as_ref().unwrap(), s1.head.as_ref().unwrap());
        assert!(Rc::ptr_eq(a, b));
    }

    #[test]
    fn empty_stack_behaviour() {
        let empty: PStack<i32> = PStack::new();
        assert!(empty.is_empty());
        assert_eq!(empty.peek(), None);
        assert!(empty.pop().is_empty());
        assert!(empty.to_vec().is_empty());
        assert_eq!(empty.push(5).pop().len(), 0);
    }

    #[test]
    fn values_live_until_the_last_version_is_dropped() {
        let drops = Rc::new(Cell::new(0));
        let s1 = PStack::new()
            .push(Tracked(Rc::clone(&drops)))
            .push(Tracked(Rc::clone(&drops)));
        let s2 = s1.push(Tracked(Rc::clone(&drops)));
        drop(s1);
        assert_eq!(drops.get(), 0);
        assert_eq!(s2.len(), 3);
        drop(s2);
        assert_eq!(drops.get(), 3);
    }

    #[test]
    fn dropping_a_long_stack_does_not_overflow() {
        let mut s = PStack::new();
        for i in 0..100_000 {
            s = s.push(i);
        }
        let branch = s.pop().pop().push(-1);
        assert_eq!(branch.len(), 99_999);
        assert_eq!(s.peek(), Some(&99_999));
        drop(s);
        assert_eq!(branch.peek(), Some(&-1));
        drop(branch);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each version is just a pointer to its top node plus a length. New versions point
into old chains, so memory is shared. The destructor takes the head and keeps unwrapping nodes
while it owns them alone; as soon as `try_unwrap` fails, the rest of the chain belongs to some
other version too, and dropping our handle only decrements its counter.

**Complexity:** `push`, `pop`, `peek`, `len` are `O(1)`; `to_vec` is `O(n)`; `drop` is `O(k)`
for the `k` nodes that only this version owned.

**Rust concepts:**
- `Rc` makes sharing safe without a garbage collector: a node is freed exactly when its last
  owner is gone.
- `Rc::try_unwrap` turns a uniquely owned `Rc<T>` back into a `T`, which lets the destructor
  take `next` out of the node.
- Methods take `&self` and return new values: there is no mutation, so no `RefCell` is needed.

```rust
use std::rc::Rc;

pub struct PStack<T> {
    head: Option<Rc<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

// Clippy would like a `Default` impl next to `new()`; it is not needed here.
#[allow(clippy::new_without_default)]
impl<T> PStack<T> {
    pub fn new() -> Self {
        PStack { head: None, len: 0 }
    }

    pub fn push(&self, value: T) -> PStack<T> {
        PStack {
            head: Some(Rc::new(Node {
                value,
                next: self.head.clone(),
            })),
            len: self.len + 1,
        }
    }

    pub fn pop(&self) -> PStack<T> {
        PStack {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            len: self.len.saturating_sub(1),
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut values = Vec::with_capacity(self.len);
        let mut current = self.head.as_deref();
        while let Some(node) = current {
            values.push(node.value.clone());
            current = node.next.as_deref();
        }
        values
    }
}

impl<T> Drop for PStack<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(rc) = link {
            match Rc::try_unwrap(rc) {
                Ok(mut node) => link = node.next.take(),
                Err(_shared) => break,
            }
        }
    }
}
```

Full tested solution: [`src/solutions/p0612_persistent_stack.rs`](../src/solutions/p0612_persistent_stack.rs)

</details>
