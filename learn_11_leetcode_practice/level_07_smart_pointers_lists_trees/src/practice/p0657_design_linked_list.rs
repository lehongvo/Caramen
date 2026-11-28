// ==================================================================
// PRACTICE 0657: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0657_design_linked_list.md
// Run the tests:          cargo test practice::p0657
// ==================================================================

//! # Problem 0657: Design Linked List
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct over `Option<Box<Node>>`, a helper returning `&mut Option<Box<Node>>`, `usize::try_from`
//! - **Source:** [LeetCode 707. Design Linked List](https://leetcode.com/problems/design-linked-list/)
//!
//! ## Problem
//!
//! Implement a singly linked list of integers with 0-based positions:
//!
//! - `MyLinkedList::new() -> Self`: an empty list;
//! - `get(&self, index: i32) -> i32`: the value at `index`, or `-1` if the index is invalid;
//! - `add_at_head(&mut self, val: i32)`: insert before the first element;
//! - `add_at_tail(&mut self, val: i32)`: append after the last element;
//! - `add_at_index(&mut self, index: i32, val: i32)`: insert so that the new value ends up at
//!   `index`. If `index` equals the length, append; if it is larger, do nothing;
//! - `delete_at_index(&mut self, index: i32)`: remove the element at `index` if it exists.
//!
//! LeetCode's template declares every method with `&self` and a comment telling you to switch to
//! `&mut self` where you need it; the mutating methods here use `&mut self`. Define the node type
//! yourself (LeetCode's `ListNode` is not available in this problem).
//!
//! ## Examples
//!
//! ```text
//! let mut list = MyLinkedList::new();
//! list.add_at_head(1);        // [1]
//! list.add_at_tail(3);        // [1, 3]
//! list.add_at_index(1, 2);    // [1, 2, 3]
//! list.get(1)        -> 2
//! list.delete_at_index(1);    // [1, 3]
//! list.get(1)        -> 3
//! list.get(5)        -> -1
//! ```
//!
//! ## Constraints
//!
//! - `0 <= index, val <= 1000`; at most `2000` calls.
//!
//! ## Hints
//!
//! 1. Store `head: Option<Box<Node>>` and `len: usize`.
//! 2. Write one helper `fn link_at(&mut self, index: usize) -> Option<&mut Option<Box<Node>>>`
//!    that returns the link (the `head` field or some node's `next` field) at position `index`.
//!    Inserting at `index` writes a new node into that link; deleting replaces the link with the
//!    removed node's `next`.
//! 3. Inside the helper, `link = &mut link.as_mut()?.next;` walks one step.

pub struct MyLinkedList {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyLinkedList {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn get(&self, index: i32) -> i32 {
        todo!("Write your solution here")
    }

    pub fn add_at_head(&mut self, val: i32) {
        todo!("Write your solution here")
    }

    pub fn add_at_tail(&mut self, val: i32) {
        todo!("Write your solution here")
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        todo!("Write your solution here")
    }

    pub fn delete_at_index(&mut self, index: i32) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contents(list: &MyLinkedList, len: i32) -> Vec<i32> {
        (0..len).map(|i| list.get(i)).collect()
    }

    #[test]
    fn example_sequence() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);
        assert_eq!(list.get(1), 2);
        list.delete_at_index(1);
        assert_eq!(list.get(1), 3);
        assert_eq!(list.get(5), -1);
    }

    #[test]
    fn invalid_indices_are_ignored() {
        let mut list = MyLinkedList::new();
        assert_eq!(list.get(0), -1);
        list.add_at_index(1, 10);
        list.delete_at_index(0);
        assert_eq!(list.get(0), -1);
        list.add_at_index(0, 10);
        list.add_at_index(1, 20);
        list.delete_at_index(2);
        assert_eq!(contents(&list, 2), vec![10, 20]);
        assert_eq!(list.get(-1), -1);
    }

    #[test]
    fn head_tail_and_middle_operations() {
        let mut list = MyLinkedList::new();
        for v in [3, 2, 1] {
            list.add_at_head(v);
        }
        for v in [4, 5] {
            list.add_at_tail(v);
        }
        list.add_at_index(5, 6);
        list.add_at_index(0, 0);
        assert_eq!(contents(&list, 7), vec![0, 1, 2, 3, 4, 5, 6]);
        list.delete_at_index(0);
        list.delete_at_index(5);
        list.delete_at_index(2);
        assert_eq!(contents(&list, 4), vec![1, 2, 4, 5]);
        assert_eq!(list.get(4), -1);
    }

    #[test]
    fn matches_a_vec_model() {
        let mut list = MyLinkedList::new();
        let mut model: Vec<i32> = Vec::new();
        for step in 0..2_000 {
            let index = step * 31 % 50;
            match step % 4 {
                0 => {
                    list.add_at_head(step);
                    model.insert(0, step);
                }
                1 => {
                    list.add_at_tail(step);
                    model.push(step);
                }
                2 => {
                    list.add_at_index(index, step);
                    if index as usize <= model.len() {
                        model.insert(index as usize, step);
                    }
                }
                _ => {
                    list.delete_at_index(index);
                    if (index as usize) < model.len() {
                        model.remove(index as usize);
                    }
                }
            }
        }
        assert_eq!(contents(&list, model.len() as i32 + 1), {
            let mut expected = model.clone();
            expected.push(-1);
            expected
        });
    }
}
