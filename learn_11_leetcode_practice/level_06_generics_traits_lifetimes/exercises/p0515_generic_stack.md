# Problem 0515: A Generic Stack

- **Difficulty:** Easy
- **Topics:** generic data structure `Stack<T>`, `Option<&T>`, `Option<&mut T>`, `Default`
- **Source:** Original · [The Rust Book 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions)

## Problem

Build a last-in, first-out stack that can hold values of any type `T`. The struct keeps its
elements in a private field `items: Vec<T>` (the top of the stack is the end of the vector).

Implement in `impl<T> Stack<T>`:

- `new() -> Self`: an empty stack.
- `push(&mut self, item: T)`: puts `item` on top.
- `pop(&mut self) -> Option<T>`: removes and returns the top, or `None` when empty.
- `peek(&self) -> Option<&T>`: borrows the top without removing it.
- `peek_mut(&mut self) -> Option<&mut T>`: a mutable borrow of the top, to change it in place.
- `len(&self) -> usize` and `is_empty(&self) -> bool`.
- `into_vec(self) -> Vec<T>`: consumes the stack and returns its elements from **bottom** to top.

Also implement `Default` for `Stack<T>` (an empty stack), so `Stack::default()` works and
`#[derive(Default)]` structs can contain a stack.

## Examples

```text
let mut s = Stack::new();
s.push(1); s.push(2); s.push(3);
s.peek()          => Some(&3)
s.pop()           => Some(3)
*s.peek_mut().unwrap() *= 10;
s.into_vec()      => [1, 20]
```

## Constraints

- Up to `10^5` operations.

## Hints

1. Every method maps to one `Vec` method: `push`, `pop`, `last`, `last_mut`, `len`, `is_empty`.
2. `impl<T> Default for Stack<T>` needs no bound on `T`: an empty `Vec<T>` exists for every `T`.
   (`#[derive(Default)]` would add a `T: Default` bound, which is why we write it by hand.)

## Starter code

Write your code in [`src/practice/p0515_generic_stack.rs`](../src/practice/p0515_generic_stack.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0515
```

```rust
/// A last-in, first-out stack.
#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&mut self, item: T) {
        todo!("Write your solution here")
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!("Write your solution here")
    }

    pub fn peek(&self) -> Option<&T> {
        todo!("Write your solution here")
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    /// The elements from bottom to top.
    pub fn into_vec(self) -> Vec<T> {
        todo!("Write your solution here")
    }
}

impl<T> Default for Stack<T> {
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
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.peek(), Some(&3));
        assert_eq!(s.pop(), Some(3));
        *s.peek_mut().unwrap() *= 10;
        assert_eq!(s.len(), 2);
        assert_eq!(s.into_vec(), vec![1, 20]);
    }

    #[test]
    fn empty_stack() {
        let mut s: Stack<String> = Stack::default();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.peek(), None);
        assert_eq!(s.peek_mut(), None);
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn holds_non_copy_values() {
        let mut s = Stack::new();
        s.push(String::from("a"));
        s.push(String::from("b"));
        s.peek_mut().unwrap().push('!');
        assert_eq!(s.pop().as_deref(), Some("b!"));
        assert_eq!(s.peek().map(String::as_str), Some("a"));
    }

    #[test]
    fn default_inside_derived_struct() {
        #[derive(Default)]
        struct Undo {
            history: Stack<Vec<u8>>,
        }
        let mut u = Undo::default();
        u.history.push(vec![1]);
        assert_eq!(u.history.len(), 1);
    }

    #[test]
    fn many_operations() {
        let mut s = Stack::new();
        for i in 0..100_000 {
            s.push(i);
        }
        let mut expected = 99_999;
        while let Some(top) = s.pop() {
            assert_eq!(top, expected);
            expected -= 1;
            if top == 50_000 {
                break;
            }
        }
        assert_eq!(s.len(), 50_000);
        assert_eq!(s.peek(), Some(&49_999));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a thin wrapper around `Vec<T>` whose end is the top of the stack.

**Rust concepts:**
- The struct is generic, so `Stack<String>`, `Stack<(u8, char)>` and `Stack<Vec<i32>>` all reuse
  the same code; the compiler generates a specialised copy for each `T` (monomorphization).
- `peek` returns `Option<&T>` and `peek_mut` returns `Option<&mut T>`: callers borrow the top
  instead of copying it, so `T` needs no `Clone` bound.
- A manual `impl<T> Default` avoids the `T: Default` bound that `#[derive(Default)]` adds to
  generic types. Clippy's `new_without_default` lint also recommends providing `Default` next to
  a `new()` without arguments.

```rust
/// A last-in, first-out stack.
#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The elements from bottom to top.
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
```

Full tested solution: [`src/solutions/p0515_generic_stack.rs`](../src/solutions/p0515_generic_stack.rs)

</details>
