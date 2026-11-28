// ==================================================================
// PRACTICE 0578: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0578_flatten_nested_list_iterator.md
// Run the tests:          cargo test practice::p0578
// ==================================================================

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

/// LeetCode's nested list type.
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/// Iterates lazily over the integers of a nested list.
pub struct NestedIterator {}

impl NestedIterator {
    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        todo!("Write your solution here")
    }

    // LeetCode's API: `next` returns `i32`, so it cannot be `Iterator::next`.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn has_next(&self) -> bool {
        todo!("Write your solution here")
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
