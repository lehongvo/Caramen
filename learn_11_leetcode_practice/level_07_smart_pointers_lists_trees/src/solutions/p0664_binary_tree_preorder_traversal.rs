//! # Problem 0664: Binary Tree Preorder Traversal
//!
//! - **Difficulty:** Easy
//! - **Topics:** explicit stack of `Rc<RefCell<TreeNode>>`, `Rc::clone`, iterative traversal
//! - **Source:** [LeetCode 144. Binary Tree Preorder Traversal](https://leetcode.com/problems/binary-tree-preorder-traversal/)
//!
//! ## Problem
//!
//! Return the values of a binary tree in **pre-order**: a node first, then its whole left subtree,
//! then its whole right subtree. Solve it **without recursion**, using an explicit stack.
//!
//! Implement `Solution::preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [1, null, 2, 3]
//! Output: [1, 2, 3]
//!
//! Input:  root = [1, 2, 3, 4, 5, null, 8, null, null, 6, 7, 9]
//! Output: [1, 2, 4, 5, 6, 7, 3, 8, 9]
//!
//! Input:  root = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 100`, `-100 <= val <= 100`
//!
//! ## Hints
//!
//! 1. Start with the root on a `Vec` used as a stack. Pop a node, record its value, then push its
//!    children.
//! 2. A stack is last in, first out: push the **right** child before the left one, so the left
//!    subtree is visited first.
//! 3. Pushing `node.left.clone()` clones an `Option<Rc<..>>`, which only bumps a counter.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** simulate the recursion with a stack of node handles. Each pop visits one node and
//! schedules its children, right first.
//!
//! **Complexity:** `O(n)` time, `O(h)` extra space for the stack (at most `O(n)`).
//!
//! **Rust concepts:**
//! - The stack owns `Rc` handles, so nodes stay alive and reachable while the tree is borrowed.
//! - `stack.extend([right, left].into_iter().flatten())` pushes only the children that exist.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // @solution-start
        let mut values = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        while let Some(node) = stack.pop() {
            let node = node.borrow();
            values.push(node.val);
            stack.extend(
                [node.right.clone(), node.left.clone()]
                    .into_iter()
                    .flatten(),
            );
        }
        values
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree};

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::preorder_traversal(parse_tree("[1,null,2,3]")),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn example_two() {
        let root = parse_tree("[1,2,3,4,5,null,8,null,null,6,7,9]");
        assert_eq!(
            Solution::preorder_traversal(root),
            vec![1, 2, 4, 5, 6, 7, 3, 8, 9]
        );
    }

    #[test]
    fn empty_and_single() {
        assert!(Solution::preorder_traversal(None).is_empty());
        assert_eq!(Solution::preorder_traversal(parse_tree("[1]")), vec![1]);
    }

    #[test]
    fn complete_tree() {
        let values: Vec<Option<i32>> = (1..=7).map(Some).collect();
        assert_eq!(
            Solution::preorder_traversal(tree(&values)),
            vec![1, 2, 4, 5, 3, 6, 7]
        );
    }
}
