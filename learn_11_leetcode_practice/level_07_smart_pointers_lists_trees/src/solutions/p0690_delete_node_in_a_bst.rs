//! # Problem 0690: Delete Node in a BST
//!
//! - **Difficulty:** Medium
//! - **Topics:** recursive rebuild with `take()`, matching on a pair of children, in-order successor
//! - **Source:** [LeetCode 450. Delete Node in a BST](https://leetcode.com/problems/delete-node-in-a-bst/)
//!
//! ## Problem
//!
//! Remove the node whose value is `key` from a binary search tree (if there is one) and return the
//! root of the resulting tree, which must still be a valid BST. Several results are valid; any of
//! them is accepted.
//!
//! Implement
//! `Solution::delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [5, 3, 6, 2, 4, null, 7], key = 3
//! Output: [5, 4, 6, 2, null, null, 7]     // [5, 2, 6, null, 4, null, 7] is also valid
//!
//! Input:  root = [5, 3, 6, 2, 4, null, 7], key = 0
//! Output: [5, 3, 6, 2, 4, null, 7]        // key not found
//!
//! Input:  root = [], key = 0
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^4`, `-10^5 <= val, key <= 10^5`, all values distinct; `root` is a valid BST.
//!
//! ## Hints
//!
//! 1. Search recursively: delete from the left subtree when `key` is smaller, from the right when it
//!    is larger, and store the returned subtree back into the node.
//! 2. When you find the node: with no child, the subtree becomes `None`; with one child, it becomes
//!    that child.
//! 3. With two children: copy the **smallest value of the right subtree** (the in-order successor)
//!    into the node, then delete that value from the right subtree.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** recursive deletion. Children are taken out of the node (`take()`), processed, and put
//! back. The two-children case keeps the node, replaces its value by the successor's, and removes
//! the successor, which has no left child, from the right subtree.
//!
//! **Complexity:** `O(h)` time, `O(h)` stack space.
//!
//! **Rust concepts:**
//! - `match (node.left.take(), node.right.take())` moves both children into the pattern, so every
//!   arm decides what the node's subtrees become.
//! - `(Some(child), None) | (None, Some(child)) => Some(child)` handles both one-child cases in a
//!   single arm.
//! - The `borrow_mut()` guard is released at the end of its block, before the `Rc` is returned.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // @solution-start remove
    fn min_value(node: &Rc<RefCell<TreeNode>>) -> i32 {
        let mut current = Rc::clone(node);
        loop {
            let left = current.borrow().left.clone();
            match left {
                Some(next) => current = next,
                None => return current.borrow().val,
            }
        }
    }
    // @solution-end

    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // @solution-start
        let rc = root?;
        {
            let mut node = rc.borrow_mut();
            match key.cmp(&node.val) {
                Ordering::Less => node.left = Self::delete_node(node.left.take(), key),
                Ordering::Greater => node.right = Self::delete_node(node.right.take(), key),
                Ordering::Equal => match (node.left.take(), node.right.take()) {
                    (None, None) => return None,
                    (Some(child), None) | (None, Some(child)) => return Some(child),
                    (Some(left), Some(right)) => {
                        let successor = Self::min_value(&right);
                        node.val = successor;
                        node.left = Some(left);
                        node.right = Self::delete_node(Some(right), successor);
                    }
                },
            }
        }
        Some(rc)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree_to_vec};

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            inorder(&node.left, out);
            out.push(node.val);
            inorder(&node.right, out);
        }
    }

    /// Any valid answer is accepted: the in-order traversal must be the old values minus `key`.
    fn assert_valid_delete(text: &str, key: i32) {
        let expected: Vec<i32> = {
            let mut values = Vec::new();
            inorder(&parse_tree(text), &mut values);
            values.into_iter().filter(|&v| v != key).collect()
        };
        let result = Solution::delete_node(parse_tree(text), key);
        let mut values = Vec::new();
        inorder(&result, &mut values);
        assert_eq!(values, expected);
    }

    #[test]
    fn delete_node_with_two_children() {
        assert_valid_delete("[5,3,6,2,4,null,7]", 3);
    }

    #[test]
    fn key_not_found_and_empty_tree() {
        let result = Solution::delete_node(parse_tree("[5,3,6,2,4,null,7]"), 0);
        assert_eq!(
            tree_to_vec(&result),
            tree_to_vec(&parse_tree("[5,3,6,2,4,null,7]"))
        );
        assert_eq!(Solution::delete_node(None, 0), None);
    }

    #[test]
    fn delete_leaf_root_and_one_child_nodes() {
        assert_valid_delete("[5,3,6,2,4,null,7]", 7);
        assert_valid_delete("[5,3,6,2,4,null,7]", 6);
        assert_valid_delete("[5,3,6,2,4,null,7]", 5);
        assert_eq!(Solution::delete_node(parse_tree("[1]"), 1), None);
    }

    #[test]
    fn delete_every_value_one_by_one() {
        let text = "[50,30,70,20,40,60,80,10,25,35,45,55,65,75,85]";
        let mut root = parse_tree(text);
        let mut remaining: Vec<i32> = {
            let mut values = Vec::new();
            inorder(&root, &mut values);
            values
        };
        for key in [50, 30, 85, 10, 60, 45, 70, 25, 20, 35, 40, 55, 65, 75, 80] {
            root = Solution::delete_node(root, key);
            remaining.retain(|&v| v != key);
            let mut values = Vec::new();
            inorder(&root, &mut values);
            assert_eq!(values, remaining, "after deleting {key}");
        }
        assert_eq!(root, None);
    }
}
