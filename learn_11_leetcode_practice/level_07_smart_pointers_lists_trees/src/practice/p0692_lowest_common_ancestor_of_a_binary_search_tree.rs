// ==================================================================
// PRACTICE 0692: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0692_lowest_common_ancestor_of_a_binary_search_tree.md
// Run the tests:          cargo test practice::p0692
// ==================================================================

//! # Problem 0692: Lowest Common Ancestor of a Binary Search Tree
//!
//! - **Difficulty:** Medium
//! - **Topics:** BST walk with an owned `Rc` cursor, returning a node handle, reading values from `Option<Rc<..>>` arguments
//! - **Source:** [LeetCode 235. Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)
//!
//! ## Problem
//!
//! The *lowest common ancestor* (LCA) of two nodes `p` and `q` is the deepest node that has both of
//! them as descendants, where a node counts as a descendant of itself. Given a binary search tree
//! and two of its nodes, return their LCA (a handle to the node in the tree).
//!
//! Implement
//! `Solution::lowest_common_ancestor(root, p, q) -> Option<Rc<RefCell<TreeNode>>>`, where all three
//! parameters are `Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [6, 2, 8, 0, 4, 7, 9, null, null, 3, 5], p = 2, q = 8
//! Output: 6
//!
//! Input:  root = [6, 2, 8, 0, 4, 7, 9, null, null, 3, 5], p = 2, q = 4
//! Output: 2              // a node is its own descendant
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 10^5`, `-10^9 <= val <= 10^9`, all values distinct.
//! - `p != q`, and both are nodes of the tree.
//!
//! ## Hints
//!
//! 1. Only the values of `p` and `q` matter: read them once with `p?.borrow().val`.
//! 2. Starting at the root: if both values are smaller than the node, the LCA is in the left
//!    subtree; if both are larger, it is in the right subtree; otherwise the paths split here, and
//!    this node is the LCA.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{find_node, parse_tree};

    const TREE: &str = "[6,2,8,0,4,7,9,null,null,3,5]";

    fn lca(text: &str, p: i32, q: i32) -> i32 {
        let root = parse_tree(text);
        let (p, q) = (find_node(&root, p), find_node(&root, q));
        let answer = Solution::lowest_common_ancestor(root.clone(), p, q).unwrap();
        let val = answer.borrow().val;
        assert!(Rc::ptr_eq(&answer, &find_node(&root, val).unwrap()));
        val
    }

    #[test]
    fn split_at_the_root() {
        assert_eq!(lca(TREE, 2, 8), 6);
    }

    #[test]
    fn ancestor_is_one_of_the_nodes() {
        assert_eq!(lca(TREE, 2, 4), 2);
        assert_eq!(lca(TREE, 4, 2), 2);
    }

    #[test]
    fn deeper_splits() {
        assert_eq!(lca(TREE, 3, 5), 4);
        assert_eq!(lca(TREE, 0, 5), 2);
        assert_eq!(lca(TREE, 7, 9), 8);
        assert_eq!(lca("[2,1]", 2, 1), 2);
    }
}
