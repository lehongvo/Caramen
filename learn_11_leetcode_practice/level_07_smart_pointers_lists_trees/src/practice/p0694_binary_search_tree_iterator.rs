// ==================================================================
// PRACTICE 0694: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0694_binary_search_tree_iterator.md
// Run the tests:          cargo test practice::p0694
// ==================================================================

//! # Problem 0694: Binary Search Tree Iterator
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct holding a `Vec<Rc<RefCell<TreeNode>>>`, lazy in-order traversal, `#[allow(clippy::should_implement_trait)]`
//! - **Source:** [LeetCode 173. Binary Search Tree Iterator](https://leetcode.com/problems/binary-search-tree-iterator/)
//!
//! ## Problem
//!
//! Design an iterator over the values of a binary search tree in **increasing** order:
//!
//! - `BSTIterator::new(root: Option<Rc<RefCell<TreeNode>>>) -> Self`;
//! - `next(&mut self) -> i32`: return the next smallest value. It is only called when a value
//!   remains;
//! - `has_next(&self) -> bool`: whether another value remains.
//!
//! Both operations should take `O(1)` time on average and the iterator should use only `O(h)`
//! memory, where `h` is the height of the tree. Copying all values into a `Vec` up front would
//! use `O(n)`. (LeetCode's template declares `next(&self)` with a hint to switch to `&mut self`.)
//!
//! ## Examples
//!
//! ```text
//! let mut it = BSTIterator::new(parse_tree("[7,3,15,null,null,9,20]"));
//! it.next()     -> 3
//! it.next()     -> 7
//! it.has_next() -> true
//! it.next()     -> 9
//! it.next()     -> 15
//! it.next()     -> 20
//! it.has_next() -> false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^5`, `0 <= val <= 10^6`; at most `10^5` calls.
//!
//! ## Hints
//!
//! 1. Store a stack of nodes: the left spine from the root down to the smallest value.
//! 2. `next` pops the top node (the smallest remaining value), then pushes the left spine of its
//!    **right** child.
//! 3. Each node is pushed and popped exactly once, so `n` calls to `next` cost `O(n)` in total.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct BSTIterator {}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        todo!("Write your solution here")
    }

    // LeetCode's interface names this method `next`; Clippy would prefer an `Iterator` impl.
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
    use crate::common::{parse_tree, tree};

    #[test]
    fn example_sequence() {
        let mut it = BSTIterator::new(parse_tree("[7,3,15,null,null,9,20]"));
        assert_eq!(it.next(), 3);
        assert_eq!(it.next(), 7);
        assert!(it.has_next());
        assert_eq!(it.next(), 9);
        assert!(it.has_next());
        assert_eq!(it.next(), 15);
        assert!(it.has_next());
        assert_eq!(it.next(), 20);
        assert!(!it.has_next());
    }

    #[test]
    fn single_node() {
        let mut it = BSTIterator::new(parse_tree("[1]"));
        assert!(it.has_next());
        assert_eq!(it.next(), 1);
        assert!(!it.has_next());
    }

    #[test]
    fn full_iteration_is_sorted() {
        let root = parse_tree("[8,4,12,2,6,10,14,1,3,5,7,9,11,13,15]");
        let mut it = BSTIterator::new(root);
        let mut values = Vec::new();
        while it.has_next() {
            values.push(it.next());
        }
        assert_eq!(values, (1..=15).collect::<Vec<_>>());
    }

    #[test]
    fn right_chain() {
        let values: Vec<Option<i32>> = vec![Some(1), None, Some(2), None, Some(3)];
        let mut it = BSTIterator::new(tree(&values));
        assert_eq!((it.next(), it.next(), it.next()), (1, 2, 3));
        assert!(!it.has_next());
    }
}
