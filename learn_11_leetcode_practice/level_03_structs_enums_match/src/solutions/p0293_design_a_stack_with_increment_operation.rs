//! # Problem 0293: Design a Stack With Increment Operation
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct with two parallel `Vec`s, lazy updates, `Option` from `pop`, keeping an invariant across methods
//! - **Source:** [LeetCode 1381. Design a Stack With Increment Operation](https://leetcode.com/problems/design-a-stack-with-increment-operation/)
//!
//! ## Problem
//!
//! Implement a stack of integers with a maximum size and a bulk-increment operation:
//!
//! - `CustomStack::new(max_size: i32) -> Self`: an empty stack that can hold at most `max_size`
//!   elements;
//! - `push(&mut self, x: i32)`: push `x` on top, unless the stack is already full (then do nothing);
//! - `pop(&mut self) -> i32`: remove and return the top element, or `-1` if the stack is empty;
//! - `increment(&mut self, k: i32, val: i32)`: add `val` to each of the **bottom** `k` elements
//!   (to all elements if there are fewer than `k`).
//!
//! Make every operation `O(1)`. This is a LeetCode *design* problem, so there is no
//! `impl Solution`.
//!
//! ## Examples
//!
//! ```text
//! let mut s = CustomStack::new(3);
//! s.push(1); s.push(2);
//! s.pop()             => 2
//! s.push(2); s.push(3); s.push(4);   // 4 is ignored: the stack is full ([1, 2, 3])
//! s.increment(5, 100);               // [101, 102, 103]
//! s.increment(2, 100);               // [201, 202, 103]
//! s.pop()             => 103
//! s.pop()             => 202
//! s.pop()             => 201
//! s.pop()             => -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= max_size, x, k <= 1000`, `0 <= val <= 100`; at most 1000 calls of each method.
//!
//! ## Hints
//!
//! 1. Instead of updating `k` elements, record "add `val` to everything at or below index `k - 1`"
//!    in a second vector `inc` at position `min(k, len) - 1`.
//! 2. When popping index `i`, the real value is `stack[i] + inc[i]`; pass `inc[i]` down to
//!    `inc[i - 1]` so the elements below still receive it.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** lazy propagation. `inc[i]` is an amount that must be added to element `i` **and to
//! every element below it**. `increment` touches one slot; `pop` applies the top slot to the
//! popped value and hands it down to the new top.
//!
//! **Complexity:** `O(1)` for every operation; `O(max_size)` space.
//!
//! **Rust concepts:**
//! - The two vectors always have the same length: `push` and `pop` update both. Keeping fields
//!   private makes that invariant impossible to break from outside.
//! - `self.values.pop()` returns `Option<i32>`; `let Some(v) = ... else { return -1; }` converts the
//!   empty case to LeetCode's sentinel.
//! - `if let Some(below) = self.inc.last_mut() { *below += extra; }` passes the pending increment down.
// @solution-end

/// A bounded stack with bulk increments of the bottom elements.
pub struct CustomStack {
    // @solution-start remove
    max_size: usize,
    values: Vec<i32>,
    inc: Vec<i32>,
    // @solution-end
}

impl CustomStack {
    /// An empty stack holding at most `max_size` elements.
    pub fn new(max_size: i32) -> Self {
        // @solution-start
        let max_size = max_size.max(0) as usize;
        Self {
            max_size,
            values: Vec::with_capacity(max_size),
            inc: Vec::with_capacity(max_size),
        }
        // @solution-end
    }

    /// Pushes `x` unless the stack is full.
    pub fn push(&mut self, x: i32) {
        // @solution-start
        if self.values.len() < self.max_size {
            self.values.push(x);
            self.inc.push(0);
        }
        // @solution-end
    }

    /// Pops the top element, or returns `-1` if empty.
    pub fn pop(&mut self) -> i32 {
        // @solution-start
        let (Some(v), Some(extra)) = (self.values.pop(), self.inc.pop()) else {
            return -1;
        };
        if let Some(below) = self.inc.last_mut() {
            *below += extra;
        }
        v + extra
        // @solution-end
    }

    /// Adds `val` to the bottom `k` elements.
    pub fn increment(&mut self, k: i32, val: i32) {
        // @solution-start
        let count = (k.max(0) as usize).min(self.values.len());
        if count > 0 {
            self.inc[count - 1] += val;
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut s = CustomStack::new(3);
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), 2);
        s.push(2);
        s.push(3);
        s.push(4);
        s.increment(5, 100);
        s.increment(2, 100);
        assert_eq!(s.pop(), 103);
        assert_eq!(s.pop(), 202);
        assert_eq!(s.pop(), 201);
        assert_eq!(s.pop(), -1);
    }

    #[test]
    fn increment_on_empty_stack_is_lost() {
        let mut s = CustomStack::new(2);
        s.increment(2, 50);
        s.push(1);
        assert_eq!(s.pop(), 1);
        assert_eq!(s.pop(), -1);
    }

    #[test]
    fn increments_survive_pushes_above() {
        let mut s = CustomStack::new(4);
        s.push(10);
        s.increment(1, 5);
        s.push(20);
        s.push(30);
        s.increment(2, 1);
        assert_eq!(s.pop(), 30);
        assert_eq!(s.pop(), 21);
        assert_eq!(s.pop(), 16);
    }

    #[test]
    fn many_operations() {
        let mut s = CustomStack::new(1000);
        for x in 1..=1000 {
            s.push(x);
        }
        for _ in 0..1000 {
            s.increment(500, 1);
        }
        assert_eq!(s.pop(), 1000);
        for _ in 0..499 {
            s.pop();
        }
        // The top is now element 500, which received all 1000 increments.
        assert_eq!(s.pop(), 1500);
    }
}
