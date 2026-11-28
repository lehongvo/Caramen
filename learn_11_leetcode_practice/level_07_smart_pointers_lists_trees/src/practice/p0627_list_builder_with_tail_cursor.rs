// ==================================================================
// PRACTICE 0627: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0627_list_builder_with_tail_cursor.md
// Run the tests:          cargo test practice::p0627
// ==================================================================

//! # Problem 0627: Build a List with a Tail Cursor
//!
//! - **Difficulty:** Easy
//! - **Topics:** LeetCode's `ListNode`, `&mut Option<Box<ListNode>>` cursors, `Option::insert`, `Option::as_deref`
//! - **Source:** Original · [`Option::insert`](https://doc.rust-lang.org/std/option/enum.Option.html#method.insert) · [The Rust Book 15.1](https://doc.rust-lang.org/book/ch15-01-box.html)
//!
//! ## Problem
//!
//! The rest of this level uses LeetCode's list node (defined in `crate::common`):
//!
//! ```text
//! pub struct ListNode {
//!     pub val: i32,
//!     pub next: Option<Box<ListNode>>,
//! }
//! ```
//!
//! A list is an `Option<Box<ListNode>>`: `None` is the empty list. Most list problems need two
//! skills: **reading** a list with a shared cursor, and **building** one front to back with a
//! mutable cursor that always points at the empty `next` slot at the end. Practise both:
//!
//! - `from_vec(values: Vec<i32>) -> Option<Box<ListNode>>`: build the list in order, front to
//!   back, in one pass (do not build it backwards);
//! - `push_back(head: &mut Option<Box<ListNode>>, val: i32)`: append one node at the end;
//! - `length(head: &Option<Box<ListNode>>) -> usize`;
//! - `get(head: &Option<Box<ListNode>>, index: usize) -> Option<i32>`: the value at `index`
//!   (`0` is the head), or `None` if the list is too short.
//!
//! ## Examples
//!
//! ```text
//! from_vec(vec![1, 2, 3])       -> 1 -> 2 -> 3
//! push_back(&mut head, 4)       -> 1 -> 2 -> 3 -> 4
//! length(&head)                 -> 4
//! get(&head, 0) -> Some(1), get(&head, 3) -> Some(4), get(&head, 4) -> None
//! from_vec(vec![])              -> None
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 5000`
//!
//! ## Hints
//!
//! 1. Building: `let mut head = None; let mut tail = &mut head;`. For each value,
//!    `let node = tail.insert(Box::new(ListNode::new(v)));` stores the node in the empty slot and
//!    returns `&mut Box<ListNode>`; then move the cursor: `tail = &mut node.next;`.
//! 2. Appending to an existing list: walk `while let Some(node) = cursor { cursor = &mut node.next; }`
//!    and write into the `None` you end on.
//! 3. Reading: `let mut current = head.as_deref();` is an `Option<&ListNode>`; step with
//!    `current = node.next.as_deref();`.

use crate::common::ListNode;

pub fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
    todo!("Write your solution here")
}

pub fn push_back(head: &mut Option<Box<ListNode>>, val: i32) {
    todo!("Write your solution here")
}

pub fn length(head: &Option<Box<ListNode>>) -> usize {
    todo!("Write your solution here")
}

pub fn get(head: &Option<Box<ListNode>>, index: usize) -> Option<i32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn from_vec_keeps_the_order() {
        let head = from_vec(vec![1, 2, 3]);
        assert_eq!(head, list(&[1, 2, 3]));
        assert_eq!(from_vec(vec![]), None);
    }

    #[test]
    fn push_back_appends() {
        let mut head = from_vec(vec![1, 2, 3]);
        push_back(&mut head, 4);
        assert_eq!(list_to_vec(&head), vec![1, 2, 3, 4]);
        let mut empty = None;
        push_back(&mut empty, 9);
        assert_eq!(list_to_vec(&empty), vec![9]);
    }

    #[test]
    fn length_and_get() {
        let head = from_vec(vec![5, 6, 7, 8]);
        assert_eq!(length(&head), 4);
        assert_eq!(get(&head, 0), Some(5));
        assert_eq!(get(&head, 3), Some(8));
        assert_eq!(get(&head, 4), None);
        assert_eq!(length(&None), 0);
        assert_eq!(get(&None, 0), None);
    }

    #[test]
    fn large_list() {
        let values: Vec<i32> = (0..5_000).collect();
        let mut head = from_vec(values.clone());
        push_back(&mut head, 5_000);
        assert_eq!(length(&head), 5_001);
        assert_eq!(get(&head, 4_321), Some(4_321));
        assert_eq!(get(&head, 5_000), Some(5_000));
    }
}
