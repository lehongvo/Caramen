// ==================================================================
// PRACTICE 0626: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0626_doubly_linked_list_with_weak.md
// Run the tests:          cargo test practice::p0626
// ==================================================================

//! # Problem 0626: Doubly Linked List with Rc and Weak
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Rc<RefCell<Node>>` forward links, `Weak` backward links, `Rc::try_unwrap`, `RefCell::into_inner`, iterative `Drop`
//! - **Source:** Original · [The Rust Book 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html) · [`Rc::try_unwrap`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.try_unwrap)
//!
//! ## Problem
//!
//! In a doubly linked list every node points to the next node **and** to the previous one. With
//! `Rc` in both directions, neighbours would form reference cycles. Use strong links forward
//! (`next: Option<Rc<RefCell<Node>>>`) and weak links backward (`prev: Option<Weak<RefCell<Node>>>`);
//! the list itself keeps a strong `head` and a strong `tail`.
//!
//! Implement a double-ended queue of `i32`:
//!
//! - `Deque::new() -> Deque`;
//! - `push_front(&mut self, value: i32)` and `push_back(&mut self, value: i32)`;
//! - `pop_front(&mut self) -> Option<i32>` and `pop_back(&mut self) -> Option<i32>`;
//! - `peek_front(&self) -> Option<i32>` and `peek_back(&self) -> Option<i32>`;
//! - `len(&self) -> usize`, `is_empty(&self) -> bool`;
//! - `to_vec(&self) -> Vec<i32>`: the values from front to back;
//! - `impl Drop for Deque` that frees long lists with a loop (the forward links form a chain).
//!
//! The node type and the fields of `Deque` are up to you.
//!
//! ## Examples
//!
//! ```text
//! let mut d = Deque::new();
//! d.push_back(2); d.push_back(3); d.push_front(1);
//! d.to_vec()     -> [1, 2, 3]
//! d.pop_back()   -> Some(3)
//! d.pop_front()  -> Some(1)
//! d.peek_front() -> Some(2), d.peek_back() -> Some(2)
//! d.len()        -> 1
//! ```
//!
//! ## Constraints
//!
//! - Up to `10^5` values.
//!
//! ## Hints
//!
//! 1. `push_back`: create the node; if there is a tail, set `tail.next = Some(Rc::clone(&node))` and
//!    `node.prev = Some(Rc::downgrade(&tail))`; then make `node` the tail (and the head if the list
//!    was empty).
//! 2. `pop_front`: take the head; its `next` becomes the new head (clear that node's `prev`), or the
//!    list becomes empty (take the tail too). Then the old head is owned only by you:
//!    `Rc::try_unwrap(old).ok()` gives the `RefCell<Node>`, and `.into_inner()` gives the `Node`.
//! 3. `pop_back`: the tail's `prev` is a `Weak`: `upgrade()` it to reach the new tail, and clear
//!    that node's `next` so the old tail is uniquely owned.
//! 4. `drop` can simply call `pop_front` until it returns `None`.

#[derive(Default)]
pub struct Deque {}

impl Deque {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push_front(&mut self, value: i32) {
        todo!("Write your solution here")
    }

    pub fn push_back(&mut self, value: i32) {
        todo!("Write your solution here")
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        todo!("Write your solution here")
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        todo!("Write your solution here")
    }

    pub fn peek_front(&self) -> Option<i32> {
        todo!("Write your solution here")
    }

    pub fn peek_back(&self) -> Option<i32> {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn to_vec(&self) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

impl Drop for Deque {
    fn drop(&mut self) {
        // TODO: free the nodes with a loop.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut d = Deque::new();
        d.push_back(2);
        d.push_back(3);
        d.push_front(1);
        assert_eq!(d.to_vec(), vec![1, 2, 3]);
        assert_eq!(d.pop_back(), Some(3));
        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.peek_front(), Some(2));
        assert_eq!(d.peek_back(), Some(2));
        assert_eq!(d.len(), 1);
    }

    #[test]
    fn empty_deque() {
        let mut d = Deque::new();
        assert!(d.is_empty());
        assert_eq!(d.pop_front(), None);
        assert_eq!(d.pop_back(), None);
        assert_eq!(d.peek_front(), None);
        assert!(d.to_vec().is_empty());
        d.push_front(5);
        assert_eq!(d.pop_back(), Some(5));
        assert!(d.is_empty());
        d.push_back(6);
        assert_eq!(d.pop_front(), Some(6));
        assert_eq!(d.peek_back(), None);
    }

    #[test]
    fn mixed_operations_match_vecdeque() {
        use std::collections::VecDeque;
        let mut d = Deque::new();
        let mut expected = VecDeque::new();
        for i in 0..1_000 {
            match i % 5 {
                0 | 1 => {
                    d.push_back(i);
                    expected.push_back(i);
                }
                2 => {
                    d.push_front(i);
                    expected.push_front(i);
                }
                3 => assert_eq!(d.pop_front(), expected.pop_front()),
                _ => assert_eq!(d.pop_back(), expected.pop_back()),
            }
            assert_eq!(d.len(), expected.len());
        }
        assert_eq!(d.to_vec(), expected.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn long_deque_drops_without_overflow() {
        let mut d = Deque::new();
        for i in 0..100_000 {
            d.push_back(i);
        }
        assert_eq!(d.peek_back(), Some(99_999));
        assert_eq!(d.pop_back(), Some(99_999));
        drop(d);
    }
}
