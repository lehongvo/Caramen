// ==================================================================
// PRACTICE 0391: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0391_design_circular_deque.md
// Run the tests:          cargo test practice::p0391
// ==================================================================

//! # Problem 0391: Design Circular Deque
//!
//! - **Difficulty:** Medium
//! - **Topics:** a capacity-limited `VecDeque`, `push_front`/`push_back`/`pop_front`/`pop_back`, `front`/`back`, design struct
//! - **Source:** [LeetCode 641. Design Circular Deque](https://leetcode.com/problems/design-circular-deque/)
//!
//! ## Problem
//!
//! Implement a double-ended queue with a fixed capacity `k`, as the `MyCircularDeque` struct:
//!
//! - `MyCircularDeque::new(k: i32) -> Self`: an empty deque holding at most `k` values;
//! - `insert_front(&mut self, value: i32) -> bool` / `insert_last(&mut self, value: i32) -> bool`:
//!   add at the front / back; return `false` (and change nothing) if the deque is full;
//! - `delete_front(&mut self) -> bool` / `delete_last(&mut self) -> bool`: remove from the front /
//!   back; return `false` if the deque is empty;
//! - `get_front(&self) -> i32` / `get_rear(&self) -> i32`: the front / back value, or `-1` if empty;
//! - `is_empty(&self) -> bool` and `is_full(&self) -> bool`.
//!
//! The mutating methods take `&mut self` instead of the template's `&self`, as LeetCode's comment
//! allows.
//!
//! ## Examples
//!
//! ```text
//! let mut d = MyCircularDeque::new(3);
//! d.insert_last(1)  => true
//! d.insert_last(2)  => true
//! d.insert_front(3) => true      (deque: 3 1 2)
//! d.insert_front(4) => false     (full)
//! d.get_rear()      => 2
//! d.is_full()       => true
//! d.delete_last()   => true      (deque: 3 1)
//! d.insert_front(4) => true      (deque: 4 3 1)
//! d.get_front()     => 4
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= 1000`, `0 <= value <= 1000`
//! - At most `2000` calls in total.
//!
//! ## Hints
//!
//! 1. `VecDeque` already is a circular buffer; you only need to enforce the capacity.
//! 2. Store `k` next to the deque and check `len() == k` before inserting.

use std::collections::VecDeque;

/// A deque with a fixed capacity.
pub struct MyCircularDeque {}

impl MyCircularDeque {
    pub fn new(k: i32) -> Self {
        todo!("Write your solution here")
    }

    pub fn insert_front(&mut self, value: i32) -> bool {
        todo!("Write your solution here")
    }

    pub fn insert_last(&mut self, value: i32) -> bool {
        todo!("Write your solution here")
    }

    pub fn delete_front(&mut self) -> bool {
        todo!("Write your solution here")
    }

    pub fn delete_last(&mut self) -> bool {
        todo!("Write your solution here")
    }

    pub fn get_front(&self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn get_rear(&self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn is_full(&self) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut d = MyCircularDeque::new(3);
        assert!(d.insert_last(1));
        assert!(d.insert_last(2));
        assert!(d.insert_front(3));
        assert!(!d.insert_front(4));
        assert_eq!(d.get_rear(), 2);
        assert!(d.is_full());
        assert!(d.delete_last());
        assert!(d.insert_front(4));
        assert_eq!(d.get_front(), 4);
    }

    #[test]
    fn empty_deque() {
        let mut d = MyCircularDeque::new(1);
        assert!(d.is_empty());
        assert!(!d.is_full());
        assert_eq!(d.get_front(), -1);
        assert_eq!(d.get_rear(), -1);
        assert!(!d.delete_front());
        assert!(!d.delete_last());
    }

    #[test]
    fn capacity_one() {
        let mut d = MyCircularDeque::new(1);
        assert!(d.insert_front(7));
        assert!(!d.insert_last(8));
        assert_eq!(d.get_front(), 7);
        assert_eq!(d.get_rear(), 7);
        assert!(d.delete_front());
        assert!(d.insert_last(8));
        assert_eq!(d.get_front(), 8);
    }

    #[test]
    fn wrap_around_many_times() {
        let mut d = MyCircularDeque::new(1000);
        for i in 0..1000 {
            assert!(d.insert_last(i));
        }
        assert!(d.is_full());
        for i in 0..1000 {
            // Rotate: take from the front, put at the back.
            assert_eq!(d.get_front(), i);
            assert!(d.delete_front());
            assert!(d.insert_last(i));
        }
        assert_eq!(d.get_front(), 0);
        assert_eq!(d.get_rear(), 999);
    }
}
