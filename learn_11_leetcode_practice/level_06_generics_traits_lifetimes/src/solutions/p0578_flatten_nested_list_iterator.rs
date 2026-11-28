//! # Problem 0578: Flatten Nested List Iterator
//!
//! - **Difficulty:** Medium
//! - **Topics:** recursive enum `NestedInteger`, a stack of `std::vec::IntoIter`, lazy iteration with a peeked value, `has_next(&self)`
//! - **Source:** [LeetCode 341. Flatten Nested List Iterator](https://leetcode.com/problems/flatten-nested-list-iterator/)
//!
//! ## Problem
//!
//! A nested list holds integers and other nested lists, e.g. `[[1, 1], 2, [1, 1]]`. LeetCode models it as
//!
//! ```text
//! pub enum NestedInteger { Int(i32), List(Vec<NestedInteger>) }
//! ```
//!
//! Implement an iterator over all the integers, in order, **lazily** (do not flatten everything up
//! front):
//!
//! - `NestedIterator::new(nested_list: Vec<NestedInteger>) -> Self`
//! - `next(&mut self) -> i32`: the next integer (only called when `has_next()` is `true`).
//! - `has_next(&self) -> bool`: whether integers remain. It takes `&self`, so it cannot advance the
//!   iterator itself: keep the next integer ready (peeked) after every operation.
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`: paste the struct and its
//! `impl NestedIterator` block into LeetCode (LeetCode already defines `NestedInteger`).
//!
//! ## Examples
//!
//! ```text
//! [[1, 1], 2, [1, 1]]    => 1, 1, 2, 1, 1
//! [1, [4, [6]]]          => 1, 4, 6
//! [[], [[]]]             => (nothing: has_next() is false at once)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nested_list.len() <= 500`
//! - Integers are in `[-10^6, 10^6]`.
//!
//! ## Hints
//!
//! 1. Keep a stack of `std::vec::IntoIter<NestedInteger>`: the top is the list currently being read.
//! 2. "Advance" means: pull from the top iterator; an `Int` becomes the peeked value, a `List` is pushed
//!    as a new iterator, an exhausted iterator is popped.
//! 3. Call that advance step in `new` and after each `next`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a stack of owning iterators walks the tree depth-first. A private `advance` method moves
//! forward until it finds an integer (stored in `peeked`) or the stack is empty. `has_next` just checks
//! `peeked`, and `next` returns it and advances again.
//!
//! **Complexity:** amortized `O(1)` per integer (each list is pushed and popped once); `O(d)` extra space
//! for nesting depth `d`.
//!
//! **Rust concepts:**
//! - `Vec<NestedInteger>::into_iter()` gives an owning iterator, so lists are moved, never cloned.
//! - `self.stack.last_mut()` borrows the top iterator mutably; the `match` on `top.next()` ends that
//!   borrow before `self.stack.push(..)` or `pop()`.
//! - LeetCode's method is named `next` but is not `Iterator::next` (it returns `i32`, not
//!   `Option<i32>`), so Clippy's `should_implement_trait` lint is allowed on it.
// @solution-end

/// LeetCode's nested list type.
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/// Iterates lazily over the integers of a nested list.
pub struct NestedIterator {
    // @solution-start remove
    stack: Vec<std::vec::IntoIter<NestedInteger>>,
    peeked: Option<i32>,
    // @solution-end
}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        // @solution-start
        let mut it = NestedIterator {
            stack: vec![nested_list.into_iter()],
            peeked: None,
        };
        it.advance();
        it
        // @solution-end
    }

    // @solution-start remove
    /// Moves to the next integer and stores it in `peeked` (or `None` at the end).
    fn advance(&mut self) {
        self.peeked = None;
        while let Some(top) = self.stack.last_mut() {
            match top.next() {
                Some(NestedInteger::Int(x)) => {
                    self.peeked = Some(x);
                    return;
                }
                Some(NestedInteger::List(list)) => self.stack.push(list.into_iter()),
                None => {
                    self.stack.pop();
                }
            }
        }
    }
    // @solution-end

    // LeetCode's API: `next` returns `i32`, so it cannot be `Iterator::next`.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        // @solution-start
        let value = self
            .peeked
            .expect("next is only called when has_next() is true");
        self.advance();
        value
        // @solution-end
    }

    pub fn has_next(&self) -> bool {
        // @solution-start
        self.peeked.is_some()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use NestedInteger::{Int, List};

    fn flatten(list: Vec<NestedInteger>) -> Vec<i32> {
        let mut it = NestedIterator::new(list);
        let mut out = Vec::new();
        while it.has_next() {
            out.push(it.next());
        }
        out
    }

    #[test]
    fn example_pairs_around_two() {
        let list = vec![
            List(vec![Int(1), Int(1)]),
            Int(2),
            List(vec![Int(1), Int(1)]),
        ];
        assert_eq!(flatten(list), vec![1, 1, 2, 1, 1]);
    }

    #[test]
    fn example_deep_nesting() {
        let list = vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])];
        assert_eq!(flatten(list), vec![1, 4, 6]);
    }

    #[test]
    fn empty_lists() {
        let list = vec![List(vec![]), List(vec![List(vec![])])];
        let it = NestedIterator::new(list);
        assert!(!it.has_next());
        assert_eq!(flatten(vec![List(vec![]), Int(-7), List(vec![])]), vec![-7]);
    }

    #[test]
    fn very_deep_and_wide() {
        let mut deep = Int(42);
        for _ in 0..500 {
            deep = List(vec![deep]);
        }
        assert_eq!(flatten(vec![deep]), vec![42]);
        let wide: Vec<NestedInteger> = (0..500)
            .map(|i| {
                if i % 2 == 0 {
                    Int(i)
                } else {
                    List(vec![Int(i), List(vec![])])
                }
            })
            .collect();
        assert_eq!(flatten(wide), (0..500).collect::<Vec<i32>>());
    }
}
