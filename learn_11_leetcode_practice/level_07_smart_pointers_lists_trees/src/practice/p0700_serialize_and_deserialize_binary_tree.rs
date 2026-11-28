// ==================================================================
// PRACTICE 0700: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0700_serialize_and_deserialize_binary_tree.md
// Run the tests:          cargo test practice::p0700
// ==================================================================

//! # Problem 0700: Serialize and Deserialize Binary Tree
//!
//! - **Difficulty:** Hard
//! - **Topics:** design struct, level-order encoding with `VecDeque`, parsing with `split` and `str::parse`, rebuilding `Rc<RefCell<TreeNode>>` links
//! - **Source:** [LeetCode 297. Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)
//!
//! ## Problem
//!
//! Design a `Codec` that turns a binary tree into a `String` and back:
//!
//! - `Codec::new() -> Self`;
//! - `serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String`;
//! - `deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>>`.
//!
//! The format is up to you; the only requirement is that `deserialize(serialize(tree))` rebuilds
//! a tree with exactly the same shape and values, for every tree (including the empty one and
//! trees with negative values). A good choice is LeetCode's own notation: the level-order values,
//! with `null` for missing children, for example `"[1,2,3,null,null,4,5]"`.
//!
//! ## Examples
//!
//! ```text
//! let codec = Codec::new();
//! let text = codec.serialize(parse_tree("[1,2,3,null,null,4,5]"));
//! codec.deserialize(text)   -> [1, 2, 3, null, null, 4, 5]
//!
//! codec.deserialize(codec.serialize(None)) -> []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^4`, `-1000 <= val <= 1000`
//!
//! ## Hints
//!
//! 1. Serialize with a breadth-first traversal over `Option` slots: a `Some` node writes its value
//!    and enqueues both children (even missing ones); a `None` slot writes `null`. Trailing `null`s
//!    can be dropped.
//! 2. Deserialize with the same queue discipline: the first token is the root; then each dequeued
//!    node takes the next two tokens as its left and right child.
//! 3. A level-order format is iterative in both directions, so deep trees cannot overflow the stack.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Codec {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl Codec {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        todo!("Write your solution here")
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree};

    fn round_trip(root: Option<Rc<RefCell<TreeNode>>>) {
        let codec = Codec::new();
        let text = codec.serialize(root.clone());
        assert_eq!(codec.deserialize(text), root);
    }

    #[test]
    fn example_tree() {
        round_trip(parse_tree("[1,2,3,null,null,4,5]"));
    }

    #[test]
    fn empty_and_single() {
        round_trip(None);
        round_trip(parse_tree("[-1000]"));
    }

    #[test]
    fn lopsided_and_negative_trees() {
        round_trip(parse_tree("[1,null,2,null,3,null,4]"));
        round_trip(parse_tree("[-1,-2,null,-3,null,-4]"));
        round_trip(parse_tree("[5,4,8,11,null,13,4,7,2,null,null,null,1]"));
    }

    #[test]
    fn large_tree() {
        let values: Vec<Option<i32>> = (0..10_000)
            .map(|i| {
                if i % 7 == 3 {
                    None
                } else {
                    Some(i % 2001 - 1000)
                }
            })
            .collect();
        round_trip(tree(&values));
    }

    #[test]
    fn serialization_is_stable() {
        // Serializing twice gives the same text, so the format is stable.
        let codec = Codec::new();
        let first = codec.serialize(parse_tree("[3,9,20,null,null,15,7]"));
        let second = codec.serialize(codec.deserialize(first.clone()));
        assert_eq!(first, second);
    }
}
