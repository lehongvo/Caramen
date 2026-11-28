// ==================================================================
// PRACTICE 0361: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0361_implement_stack_using_queues.md
// Run the tests:          cargo test practice::p0361
// ==================================================================

//! # Problem 0361: Implement Stack using Queues
//!
//! - **Difficulty:** Easy
//! - **Topics:** `VecDeque` restricted to queue operations, rotating with `pop_front` + `push_back`, `front`
//! - **Source:** [LeetCode 225. Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues/)
//!
//! ## Problem
//!
//! Implement a last-in-first-out **stack** of integers using only **queue** operations: push to the
//! back, look at or remove from the front, size and emptiness. (A `VecDeque` can do more, but
//! restrict yourself to those operations.) Implement the `MyStack` struct:
//!
//! - `MyStack::new() -> Self`: an empty stack;
//! - `push(&mut self, x: i32)`: put `x` on top;
//! - `pop(&mut self) -> i32`: remove and return the top element;
//! - `top(&self) -> i32`: return the top element without removing it;
//! - `empty(&self) -> bool`: whether the stack is empty.
//!
//! `pop` and `top` are only called on a non-empty stack. The mutating methods take `&mut self`
//! instead of the template's `&self`, as LeetCode's comment allows.
//!
//! ## Examples
//!
//! ```text
//! let mut s = MyStack::new();
//! s.push(1);
//! s.push(2);
//! s.top()   => 2
//! s.pop()   => 2
//! s.empty() => false
//! s.pop()   => 1
//! s.empty() => true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= x <= 9`
//! - At most `100` calls; `pop` and `top` are always valid.
//!
//! ## Hints
//!
//! 1. Keep the queue ordered so that its **front** is the top of the stack.
//! 2. After pushing `x` at the back, move every older element from the front to the back. Now `x`
//!    is at the front.

use std::collections::VecDeque;

/// A stack built on a single queue.
pub struct MyStack {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyStack {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&mut self, x: i32) {
        todo!("Write your solution here")
    }

    pub fn pop(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn top(&self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn empty(&self) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut s = MyStack::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.top(), 2);
        assert_eq!(s.pop(), 2);
        assert!(!s.empty());
        assert_eq!(s.pop(), 1);
        assert!(s.empty());
    }

    #[test]
    fn new_stack_is_empty() {
        assert!(MyStack::new().empty());
    }

    #[test]
    fn interleaved_operations() {
        let mut s = MyStack::new();
        s.push(3);
        s.push(4);
        assert_eq!(s.pop(), 4);
        s.push(5);
        s.push(6);
        assert_eq!(s.top(), 6);
        assert_eq!(s.pop(), 6);
        assert_eq!(s.pop(), 5);
        assert_eq!(s.top(), 3);
    }

    #[test]
    fn fifty_pushes_then_pops_reverse_the_order() {
        let mut s = MyStack::new();
        for i in 0..50 {
            s.push(i % 9 + 1);
        }
        let popped: Vec<i32> = (0..50).map(|_| s.pop()).collect();
        let expected: Vec<i32> = (0..50).rev().map(|i| i % 9 + 1).collect();
        assert_eq!(popped, expected);
        assert!(s.empty());
    }
}
