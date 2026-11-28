// ==================================================================
// PRACTICE 0609: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0609_iterative_drop_for_long_list.md
// Run the tests:          cargo test practice::p0609
// ==================================================================

//! # Problem 0609: Iterative Drop for a Long List
//!
//! - **Difficulty:** Medium
//! - **Topics:** custom `Drop`, `Option<Box<Node<T>>>`, `Option::take`, `Option::map`, stack overflow in recursive destructors
//! - **Source:** Original · [The Rust Book 15.3](https://doc.rust-lang.org/book/ch15-03-drop.html) · [`Option::take`](https://doc.rust-lang.org/std/option/enum.Option.html#method.take)
//!
//! ## Problem
//!
//! A singly linked stack is a chain of boxes: `head -> node -> node -> ... -> None`. When the
//! stack is dropped, the compiler-generated destructor drops `head`, which drops its node, which
//! drops *its* `next`, and so on: one nested call per node. With a few hundred thousand nodes the
//! thread runs out of stack and the program aborts ("has overflowed its stack").
//!
//! The layout is given:
//!
//! ```text
//! pub struct Stack<T> {
//!     head: Option<Box<Node<T>>>,
//!     len: usize,
//! }
//!
//! struct Node<T> {
//!     value: T,
//!     next: Option<Box<Node<T>>>,
//! }
//! ```
//!
//! Implement `new`, `push(&mut self, value: T)`, `pop(&mut self) -> Option<T>`,
//! `peek(&self) -> Option<&T>`, `len` and `is_empty` (last in, first out), and an
//! `impl Drop for Stack<T>` that frees the nodes **with a loop**, so dropping a stack of `200 000`
//! items works. Every item must be dropped exactly once.
//!
//! Note: while your `drop` is still empty, the long-stack test crashes the whole test binary with
//! a stack overflow. That crash is the bug this problem fixes.
//!
//! ## Examples
//!
//! ```text
//! let mut s = Stack::new();
//! s.push(1); s.push(2); s.push(3);
//! s.peek()  -> Some(&3)
//! s.pop()   -> Some(3)
//! s.len()   -> 2
//!
//! let mut big = Stack::new();
//! for i in 0..200_000 { big.push(i); }
//! drop(big);                // must not overflow the stack
//! ```
//!
//! ## Constraints
//!
//! - Up to `200 000` items; the tests run on a thread with the default 2 MiB stack.
//!
//! ## Hints
//!
//! 1. `push`: `let old = self.head.take();` then store `Some(Box::new(Node { value, next: old }))`.
//! 2. `pop`: `self.head.take().map(|node| { ... })` moves the first node out; put `node.next` back
//!    into `self.head` and return `node.value`.
//! 3. In `drop`, take the head, then repeatedly take each node's `next` **before** the node itself
//!    is dropped. A node whose `next` is already `None` is cheap to drop: there is nothing left to
//!    recurse into.

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Clippy would like a `Default` impl next to `new()`; a stack is not the point of this problem.
#[allow(clippy::new_without_default)]
impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&mut self, value: T) {
        todo!("Write your solution here")
    }

    pub fn pop(&mut self) -> Option<T> {
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
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        // TODO: free the nodes with a loop instead of the default recursive drop.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    struct Tracked(Rc<Cell<usize>>);

    impl Drop for Tracked {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[test]
    fn push_pop_peek_are_lifo() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.peek(), Some(&3));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.len(), 2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert_eq!(s.peek(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn every_item_is_dropped_exactly_once() {
        let drops = Rc::new(Cell::new(0));
        let mut s = Stack::new();
        for _ in 0..5 {
            s.push(Tracked(Rc::clone(&drops)));
        }
        drop(s.pop());
        drop(s.pop());
        assert_eq!(drops.get(), 2);
        drop(s);
        assert_eq!(drops.get(), 5);
    }

    #[test]
    fn dropping_a_long_stack_does_not_overflow() {
        let mut s = Stack::new();
        for i in 0..200_000 {
            s.push(i);
        }
        assert_eq!(s.len(), 200_000);
        assert_eq!(s.peek(), Some(&199_999));
        drop(s);
    }

    #[test]
    fn long_stack_of_owned_strings() {
        let mut s = Stack::new();
        for i in 0..200_000 {
            s.push(i.to_string());
        }
        assert_eq!(s.pop().as_deref(), Some("199999"));
        assert_eq!(s.len(), 199_999);
    }
}
