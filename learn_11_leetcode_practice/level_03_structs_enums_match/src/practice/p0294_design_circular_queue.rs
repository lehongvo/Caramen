// ==================================================================
// PRACTICE 0294: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0294_design_circular_queue.md
// Run the tests:          cargo test practice::p0294
// ==================================================================

//! # Problem 0294: Design Circular Queue
//!
//! - **Difficulty:** Medium
//! - **Topics:** ring-buffer struct, index arithmetic with `%`, methods with `&self` and `&mut self`, sentinel values
//! - **Source:** [LeetCode 622. Design Circular Queue](https://leetcode.com/problems/design-circular-queue/)
//!
//! ## Problem
//!
//! Implement a first-in first-out queue with a fixed capacity `k`, stored in a fixed-size buffer
//! whose end wraps around to its beginning (a *ring buffer*), so freed slots at the front can be
//! reused. Do not use a built-in queue type.
//!
//! - `MyCircularQueue::new(k: i32) -> Self`;
//! - `en_queue(&mut self, value: i32) -> bool`: add at the rear; `false` if full;
//! - `de_queue(&mut self) -> bool`: remove from the front; `false` if empty;
//! - `front(&self) -> i32` and `rear(&self) -> i32`: the first / last element, or `-1` if empty;
//! - `is_empty(&self) -> bool` and `is_full(&self) -> bool`.
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`.
//!
//! ## Examples
//!
//! ```text
//! let mut q = MyCircularQueue::new(3);
//! q.en_queue(1) => true
//! q.en_queue(2) => true
//! q.en_queue(3) => true
//! q.en_queue(4) => false   (full)
//! q.rear()      => 3
//! q.is_full()   => true
//! q.de_queue()  => true
//! q.en_queue(4) => true    (reuses the slot freed at the front)
//! q.rear()      => 4
//! q.front()     => 2
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= 1000`, `0 <= value <= 1000`; at most 3000 calls.
//!
//! ## Hints
//!
//! 1. Store `buf: Vec<i32>` of length `k`, the index `head` of the front element, and `len`.
//! 2. The rear element is at `(head + len - 1) % k`; a new element goes to `(head + len) % k`.

/// A fixed-capacity FIFO queue in a ring buffer.
pub struct MyCircularQueue {}

impl MyCircularQueue {
    /// An empty queue with capacity `k`.
    pub fn new(k: i32) -> Self {
        todo!("Write your solution here")
    }

    /// Adds `value` at the rear; `false` if full.
    pub fn en_queue(&mut self, value: i32) -> bool {
        todo!("Write your solution here")
    }

    /// Removes the front element; `false` if empty.
    pub fn de_queue(&mut self) -> bool {
        todo!("Write your solution here")
    }

    /// The front element, or `-1`.
    pub fn front(&self) -> i32 {
        todo!("Write your solution here")
    }

    /// The rear element, or `-1`.
    pub fn rear(&self) -> i32 {
        todo!("Write your solution here")
    }

    /// Whether the queue holds no element.
    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    /// Whether the queue is at capacity.
    pub fn is_full(&self) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut q = MyCircularQueue::new(3);
        assert!(q.en_queue(1));
        assert!(q.en_queue(2));
        assert!(q.en_queue(3));
        assert!(!q.en_queue(4));
        assert_eq!(q.rear(), 3);
        assert!(q.is_full());
        assert!(q.de_queue());
        assert!(q.en_queue(4));
        assert_eq!(q.rear(), 4);
        assert_eq!(q.front(), 2);
    }

    #[test]
    fn empty_queue_sentinels() {
        let mut q = MyCircularQueue::new(2);
        assert!(q.is_empty());
        assert!(!q.is_full());
        assert_eq!(q.front(), -1);
        assert_eq!(q.rear(), -1);
        assert!(!q.de_queue());
    }

    #[test]
    fn capacity_one() {
        let mut q = MyCircularQueue::new(1);
        assert!(q.en_queue(7));
        assert!(!q.en_queue(8));
        assert_eq!((q.front(), q.rear()), (7, 7));
        assert!(q.de_queue());
        assert!(q.is_empty());
        assert!(q.en_queue(9));
        assert_eq!(q.front(), 9);
    }

    #[test]
    fn wraps_around_many_times() {
        let mut q = MyCircularQueue::new(5);
        for i in 0..1000 {
            assert!(q.en_queue(i));
            if i >= 4 {
                assert_eq!(q.front(), i - 4);
                assert!(q.de_queue());
            }
        }
        assert_eq!(q.rear(), 999);
        assert_eq!(q.front(), 996);
    }
}
