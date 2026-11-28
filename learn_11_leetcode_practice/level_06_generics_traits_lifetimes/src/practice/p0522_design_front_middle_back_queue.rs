// ==================================================================
// PRACTICE 0522: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0522_design_front_middle_back_queue.md
// Run the tests:          cargo test practice::p0522
// ==================================================================

//! # Problem 0522: Design Front Middle Back Queue
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, two balanced `VecDeque`s, private helper methods, invariants, `Default`
//! - **Source:** [LeetCode 1670. Design Front Middle Back Queue](https://leetcode.com/problems/design-front-middle-back-queue/)
//!
//! ## Problem
//!
//! Design a queue of `i32` that supports pushing and popping at the front, at the back **and in the
//! middle**.
//!
//! - `FrontMiddleBackQueue::new() -> Self`
//! - `push_front(&mut self, val: i32)`, `push_middle(&mut self, val: i32)`,
//!   `push_back(&mut self, val: i32)`
//! - `pop_front(&mut self) -> i32`, `pop_middle(&mut self) -> i32`, `pop_back(&mut self) -> i32`:
//!   remove and return the element, or return `-1` if the queue is empty.
//!
//! When there are two middle positions, the **front-most** one is used, both for pushing and for
//! popping. For example, `push_middle(6)` on `[1, 2, 3, 4, 5]` gives `[1, 2, 6, 3, 4, 5]`, and
//! `pop_middle()` on `[1, 2, 3, 4, 5, 6]` removes `3`.
//!
//! Also implement `Default`. This is a LeetCode *design* problem, so there is no `impl Solution`:
//! paste the struct and its `impl FrontMiddleBackQueue` block into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! let mut q = FrontMiddleBackQueue::new();
//! q.push_front(1);   // [1]
//! q.push_back(2);    // [1, 2]
//! q.push_middle(3);  // [1, 3, 2]
//! q.push_middle(4);  // [1, 4, 3, 2]
//! q.pop_front()  => 1    // [4, 3, 2]
//! q.pop_middle() => 3    // [4, 2]
//! q.pop_middle() => 4    // [2]
//! q.pop_back()   => 2    // []
//! q.pop_front()  => -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= val <= 10^9`
//! - At most `1000` calls in total.
//!
//! ## Hints
//!
//! 1. Split the queue into a `left` half and a `right` half, each a `VecDeque`.
//! 2. Keep `left.len() == right.len()` or `left.len() == right.len() + 1`. Then the front-most middle
//!    element is always `left.back()`.
//! 3. Write one private `rebalance` method and call it after every operation.

use std::collections::VecDeque;

/// A queue with O(1) access to the front, the middle and the back.
pub struct FrontMiddleBackQueue {}

impl FrontMiddleBackQueue {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push_front(&mut self, val: i32) {
        todo!("Write your solution here")
    }

    pub fn push_middle(&mut self, val: i32) {
        todo!("Write your solution here")
    }

    pub fn push_back(&mut self, val: i32) {
        todo!("Write your solution here")
    }

    pub fn pop_front(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn pop_middle(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn pop_back(&mut self) -> i32 {
        todo!("Write your solution here")
    }
}

impl Default for FrontMiddleBackQueue {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut q = FrontMiddleBackQueue::new();
        q.push_front(1);
        q.push_back(2);
        q.push_middle(3);
        q.push_middle(4);
        assert_eq!(q.pop_front(), 1);
        assert_eq!(q.pop_middle(), 3);
        assert_eq!(q.pop_middle(), 4);
        assert_eq!(q.pop_back(), 2);
        assert_eq!(q.pop_front(), -1);
    }

    #[test]
    fn middle_rules_from_the_statement() {
        let mut q = FrontMiddleBackQueue::default();
        for x in 1..=5 {
            q.push_back(x);
        }
        q.push_middle(6); // [1, 2, 6, 3, 4, 5]
        assert_eq!(q.pop_middle(), 6); // front-most middle of 6 elements is index 2
        assert_eq!(q.pop_middle(), 3); // [1, 2, 3, 4, 5] -> index 2 is 3
        assert_eq!(q.pop_front(), 1);
        assert_eq!(q.pop_back(), 5);
    }

    #[test]
    fn empty_pops() {
        let mut q = FrontMiddleBackQueue::new();
        assert_eq!(q.pop_front(), -1);
        assert_eq!(q.pop_middle(), -1);
        assert_eq!(q.pop_back(), -1);
        q.push_middle(9);
        assert_eq!(q.pop_back(), 9);
        assert_eq!(q.pop_middle(), -1);
    }

    #[test]
    fn matches_a_vec_model() {
        let mut q = FrontMiddleBackQueue::new();
        let mut model: Vec<i32> = Vec::new();
        for step in 0..1000_i32 {
            let val = step + 1;
            match step % 7 {
                0 | 4 => {
                    q.push_front(val);
                    model.insert(0, val);
                }
                1 | 5 => {
                    q.push_middle(val);
                    model.insert(model.len() / 2, val);
                }
                2 => {
                    q.push_back(val);
                    model.push(val);
                }
                3 => {
                    let expected = if model.is_empty() {
                        -1
                    } else {
                        model.remove((model.len() - 1) / 2)
                    };
                    assert_eq!(q.pop_middle(), expected);
                }
                _ => {
                    let expected = if model.is_empty() {
                        -1
                    } else {
                        model.remove(0)
                    };
                    assert_eq!(q.pop_front(), expected);
                }
            }
        }
        while let Some(expected) = model.pop() {
            assert_eq!(q.pop_back(), expected);
        }
        assert_eq!(q.pop_back(), -1);
    }
}
